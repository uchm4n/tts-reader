//! Kokoro TTS backend using kokoro-en crate.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use kokoro_en::{KokoroTts, Voice};
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};

// Embedded model and voice files for self-contained builds.
// Users can override via KOKORO_MODEL_DIR / KOKORO_VOICE_DIR env vars.
const MODEL_BYTES: &[u8] = include_bytes!("../../../../packages/kokoro/models/model.onnx");
const VOICES: &[(&str, &[u8])] = &[
    ("af_heart.bin", include_bytes!("../../../../packages/kokoro/voices/af_heart.bin")),
    ("am_adam.bin", include_bytes!("../../../../packages/kokoro/voices/am_adam.bin")),
];

struct AudioState {
    _stream: OutputStream,
    sink: Sink,
}

pub struct KokoroBackend {
    tts: Arc<Mutex<KokoroTts>>,
    voice: String,
    /// Heap-allocated audio state, managed via raw pointer.
    /// AudioState contains rodio::OutputStream which is !Send on macOS,
    /// but TtsEngine is only accessed from the main thread, so this is safe.
    audio: *mut Option<AudioState>,
}

// Safety: KokoroBackend is only ever accessed from the main thread.
// The raw pointer to AudioState is only created, read, and dropped on the main thread.
unsafe impl Send for KokoroBackend {}

/// Write embedded files to a temp directory and return (model_path, voice_dir).
fn write_embedded_to_temp() -> Result<(PathBuf, String), String> {
    let temp = std::env::temp_dir().join("tts-reader");
    std::fs::create_dir_all(&temp)
        .map_err(|e| format!("Failed to create temp dir: {e}"))?;

    std::fs::write(temp.join("model.onnx"), MODEL_BYTES)
        .map_err(|e| format!("Failed to write model.onnx: {e}"))?;

    for (name, bytes) in VOICES {
        std::fs::write(temp.join(name), bytes)
            .map_err(|e| format!("Failed to write {name}: {e}"))?;
    }

    let voice_dir = temp.to_string_lossy().into_owned();
    let model_path = temp.join("model.onnx");
    Ok((model_path, voice_dir))
}

impl KokoroBackend {
    /// Create the backend, loading the ONNX model.
    /// This is the heavy part (ONNX + CoreML init) — safe to call from a background thread.
    /// OutputStream is NOT created here (it requires the main thread on macOS).
    pub fn new() -> Result<Self, String> {
        // Load .env from workspace root (single source of truth).
        // CARGO_MANIFEST_DIR is compile-time and always points to packages/desktop/.
        let env_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.env");
        dotenvy::from_path(&env_path).ok();

        let voice = std::env::var("KOKORO_VOICE").unwrap_or_else(|_| "af_heart".to_string());

        // Determine model and voice paths:
        // - If KOKORO_MODEL_DIR is set, use custom paths from .env / environment
        // - Otherwise, write embedded files to temp dir
        let (model_path, voice_dir) = if std::env::var("KOKORO_MODEL_DIR").is_ok() {
            let model_dir = std::env::var("KOKORO_MODEL_DIR").unwrap();
            let voice_dir = std::env::var("KOKORO_VOICE_DIR")
                .unwrap_or_else(|_| model_dir.clone());
            let model_name = std::env::var("KOKORO_MODEL")
                .unwrap_or_else(|_| "model.onnx".to_string());
            let path = PathBuf::from(&model_dir).join(&model_name);
            if !path.exists() {
                return Err(format!("Model file not found: {}", path.display()));
            }
            (path, voice_dir)
        } else {
            write_embedded_to_temp()?
        };

        // Temporary runtime for ONNX model loading (runs on spawn_blocking thread).
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| format!("Failed to create tokio runtime: {e}"))?;

        let tts = rt.block_on(async {
            KokoroTts::new(model_path.to_str().unwrap(), &voice_dir)
                .await
                .map_err(|e| format!("KokoroTts init failed: {e}"))
        })?;
        drop(rt);

        let audio = Box::into_raw(Box::new(None::<AudioState>));

        Ok(Self {
            tts: Arc::new(Mutex::new(tts)),
            voice,
            audio,
        })
    }

    /// Lazily initialize rodio audio output (must be called on the main thread).
    fn ensure_audio(&mut self) -> Result<(), String> {
        // Safety: self.audio is a valid Box pointer, only accessed from main thread.
        let slot = unsafe { &mut *self.audio };
        if slot.is_some() {
            return Ok(());
        }
        let (stream, handle) = OutputStream::try_default()
            .map_err(|e| format!("Audio output failed: {e}"))?;
        let sink = Sink::try_new(&handle)
            .map_err(|e| format!("Sink creation failed: {e}"))?;
        *slot = Some(AudioState {
            _stream: stream,
            sink,
        });
        Ok(())
    }

    pub fn speak(&mut self, text: &str, rate: f32) {
        self.stop();

        if text.is_empty() {
            return;
        }

        if let Err(e) = self.ensure_audio() {
            eprintln!("[TTS] Audio init failed: {e}");
            return;
        }

        let tts = self.tts.clone();
        let voice = self.voice.clone();
        let text = text.to_string();

        // Spawn synthesis on a background thread with its own tokio runtime.
        // This avoids "Cannot start a runtime from within a runtime" panics
        // when called from within Dioxus's async context.
        let handle = std::thread::spawn(move || -> Result<Vec<f32>, String> {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| format!("Failed to create runtime: {e}"))?;
            rt.block_on(async {
                let tts = tts.lock().map_err(|e| format!("Lock failed: {e}"))?;
                tts.synth(&text, Voice::new(&voice).with_speed(rate))
                    .await
                    .map(|(samples, _)| samples)
                    .map_err(|e| format!("Synth failed: {e}"))
            })
        });

        let samples = handle
            .join()
            .unwrap_or_else(|_| Err("Synthesis thread panicked".into()))
            .ok();

        if let Some(samples) = samples {
            // Safety: self.audio is a valid Box pointer, only accessed from main thread.
            let audio = unsafe { &*self.audio };
            if let Some(ref state) = audio {
                let source = SamplesBuffer::new(1, 24000, samples);
                state.sink.append(source);
            }
        } else {
            eprintln!("[TTS] Kokoro synthesis failed");
        }
    }

    pub fn stop(&mut self) {
        // Safety: self.audio is a valid Box pointer, only accessed from main thread.
        let audio = unsafe { &*self.audio };
        if let Some(ref state) = audio {
            state.sink.stop();
        }
    }

    pub fn is_speaking(&self) -> bool {
        // Safety: self.audio is a valid Box pointer, only accessed from main thread.
        let audio = unsafe { &*self.audio };
        audio
            .as_ref()
            .map(|state| !state.sink.empty())
            .unwrap_or(false)
    }
}

impl Drop for KokoroBackend {
    fn drop(&mut self) {
        // Safety: self.audio is a valid Box pointer, created in new().
        let _ = unsafe { Box::from_raw(self.audio) };
    }
}
