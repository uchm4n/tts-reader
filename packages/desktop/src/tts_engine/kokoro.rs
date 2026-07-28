//! Kokoro TTS backend using kokoro-en crate.

use std::path::Path;
use std::sync::{Arc, Mutex};

use kokoro_en::{KokoroTts, Voice};
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};

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

impl KokoroBackend {
    /// Create the backend, loading the ONNX model.
    /// This is the heavy part (ONNX + CoreML init) — safe to call from a background thread.
    /// OutputStream is NOT created here (it requires the main thread on macOS).
    pub fn new() -> Result<Self, String> {
        dotenvy::dotenv().ok();

        let model_dir = std::env::var("KOKORO_MODEL_DIR")
            .map_err(|e| format!("KOKORO_MODEL_DIR not set: {e}"))?;
        let voice_dir = std::env::var("KOKORO_VOICE_DIR")
            .map_err(|e| format!("KOKORO_VOICE_DIR not set: {e}"))?;
        let model_name =
            std::env::var("KOKORO_MODEL").unwrap_or_else(|_| "model.onnx".to_string());
        let voice = std::env::var("KOKORO_VOICE")
            .unwrap_or_else(|_| "af_heart".to_string());

        let model_path = std::path::PathBuf::from(&model_dir).join(&model_name);
        if !model_path.exists() {
            return Err(format!("Model file not found: {}", model_path.display()));
        }
        if !Path::new(&voice_dir).exists() {
            return Err(format!("Voice directory not found: {voice_dir}"));
        }

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
