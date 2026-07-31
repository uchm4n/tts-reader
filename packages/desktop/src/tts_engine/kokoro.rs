//! Kokoro TTS backend using kokoro-en crate.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use futures::StreamExt;
use kokoro_en::{KokoroTts, Voice};
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};
use tokio::sync::mpsc;

// Embedded model and voice files for self-contained builds.
// Users can override via KOKORO_MODEL_DIR / KOKORO_VOICE_DIR env vars.
const MODEL_BYTES: &[u8] = include_bytes!("../../../../packages/kokoro/models/model.onnx");
const VOICES: &[(&str, &[u8])] = &[
    ("af_alloy.bin", include_bytes!("../../../../packages/kokoro/voices/af_alloy.bin")),
    ("af_aoede.bin", include_bytes!("../../../../packages/kokoro/voices/af_aoede.bin")),
    ("af_bella.bin", include_bytes!("../../../../packages/kokoro/voices/af_bella.bin")),
    ("af_heart.bin", include_bytes!("../../../../packages/kokoro/voices/af_heart.bin")),
    ("af_jessica.bin", include_bytes!("../../../../packages/kokoro/voices/af_jessica.bin")),
    ("af_kore.bin", include_bytes!("../../../../packages/kokoro/voices/af_kore.bin")),
    ("af_nicole.bin", include_bytes!("../../../../packages/kokoro/voices/af_nicole.bin")),
    ("af_nova.bin", include_bytes!("../../../../packages/kokoro/voices/af_nova.bin")),
    ("af_river.bin", include_bytes!("../../../../packages/kokoro/voices/af_river.bin")),
    ("af_sarah.bin", include_bytes!("../../../../packages/kokoro/voices/af_sarah.bin")),
    ("af_sky.bin", include_bytes!("../../../../packages/kokoro/voices/af_sky.bin")),
    ("am_adam.bin", include_bytes!("../../../../packages/kokoro/voices/am_adam.bin")),
    ("am_echo.bin", include_bytes!("../../../../packages/kokoro/voices/am_echo.bin")),
    ("am_eric.bin", include_bytes!("../../../../packages/kokoro/voices/am_eric.bin")),
    ("am_fenrir.bin", include_bytes!("../../../../packages/kokoro/voices/am_fenrir.bin")),
    ("am_liam.bin", include_bytes!("../../../../packages/kokoro/voices/am_liam.bin")),
    ("am_michael.bin", include_bytes!("../../../../packages/kokoro/voices/am_michael.bin")),
    ("am_onyx.bin", include_bytes!("../../../../packages/kokoro/voices/am_onyx.bin")),
    ("am_puck.bin", include_bytes!("../../../../packages/kokoro/voices/am_puck.bin")),
    ("am_santa.bin", include_bytes!("../../../../packages/kokoro/voices/am_santa.bin")),
    ("bf_alice.bin", include_bytes!("../../../../packages/kokoro/voices/bf_alice.bin")),
    ("bf_emma.bin", include_bytes!("../../../../packages/kokoro/voices/bf_emma.bin")),
    ("bf_isabella.bin", include_bytes!("../../../../packages/kokoro/voices/bf_isabella.bin")),
    ("bf_lily.bin", include_bytes!("../../../../packages/kokoro/voices/bf_lily.bin")),
    ("bm_daniel.bin", include_bytes!("../../../../packages/kokoro/voices/bm_daniel.bin")),
    ("bm_fable.bin", include_bytes!("../../../../packages/kokoro/voices/bm_fable.bin")),
    ("bm_george.bin", include_bytes!("../../../../packages/kokoro/voices/bm_george.bin")),
    ("bm_lewis.bin", include_bytes!("../../../../packages/kokoro/voices/bm_lewis.bin")),
    ("ef_dora.bin", include_bytes!("../../../../packages/kokoro/voices/ef_dora.bin")),
    ("em_alex.bin", include_bytes!("../../../../packages/kokoro/voices/em_alex.bin")),
    ("em_santa.bin", include_bytes!("../../../../packages/kokoro/voices/em_santa.bin")),
    ("ff_siwis.bin", include_bytes!("../../../../packages/kokoro/voices/ff_siwis.bin")),
    ("hf_alpha.bin", include_bytes!("../../../../packages/kokoro/voices/hf_alpha.bin")),
    ("hf_beta.bin", include_bytes!("../../../../packages/kokoro/voices/hf_beta.bin")),
    ("hm_omega.bin", include_bytes!("../../../../packages/kokoro/voices/hm_omega.bin")),
    ("hm_psi.bin", include_bytes!("../../../../packages/kokoro/voices/hm_psi.bin")),
    ("if_sara.bin", include_bytes!("../../../../packages/kokoro/voices/if_sara.bin")),
    ("im_nicola.bin", include_bytes!("../../../../packages/kokoro/voices/im_nicola.bin")),
    ("jf_alpha.bin", include_bytes!("../../../../packages/kokoro/voices/jf_alpha.bin")),
    ("jf_gongitsune.bin", include_bytes!("../../../../packages/kokoro/voices/jf_gongitsune.bin")),
    ("jf_nezumi.bin", include_bytes!("../../../../packages/kokoro/voices/jf_nezumi.bin")),
    ("jf_tebukuro.bin", include_bytes!("../../../../packages/kokoro/voices/jf_tebukuro.bin")),
    ("jm_kumo.bin", include_bytes!("../../../../packages/kokoro/voices/jm_kumo.bin")),
    ("pf_dora.bin", include_bytes!("../../../../packages/kokoro/voices/pf_dora.bin")),
    ("pm_alex.bin", include_bytes!("../../../../packages/kokoro/voices/pm_alex.bin")),
    ("pm_santa.bin", include_bytes!("../../../../packages/kokoro/voices/pm_santa.bin")),
    ("zf_xiaobei.bin", include_bytes!("../../../../packages/kokoro/voices/zf_xiaobei.bin")),
    ("zf_xiaoni.bin", include_bytes!("../../../../packages/kokoro/voices/zf_xiaoni.bin")),
    ("zf_xiaoxiao.bin", include_bytes!("../../../../packages/kokoro/voices/zf_xiaoxiao.bin")),
    ("zm_yunjian.bin", include_bytes!("../../../../packages/kokoro/voices/zm_yunjian.bin")),
    ("zm_yunxi.bin", include_bytes!("../../../../packages/kokoro/voices/zm_yunxi.bin")),
    ("zm_yunxia.bin", include_bytes!("../../../../packages/kokoro/voices/zm_yunxia.bin")),
    ("zm_yunyang.bin", include_bytes!("../../../../packages/kokoro/voices/zm_yunyang.bin")),
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
    /// Channel receiver for audio chunks from the streaming synthesis thread.
    /// Stored here to keep it alive; the spawned Dioxus task owns the receiver.
    _audio_rx: Option<mpsc::UnboundedReceiver<Vec<f32>>>,
    /// Flag to signal the streaming thread to stop synthesis.
    stop_flag: Arc<AtomicBool>,
    /// Flag indicating the streaming thread is still active (producing audio).
    is_active: Arc<AtomicBool>,
}

// Safety: KokoroBackend is only ever accessed from the main thread.
// The raw pointer to AudioState is only created, read, and dropped on the main thread.
unsafe impl Send for KokoroBackend {}

/// Ensure model and voice files are extracted to ~/.tts-reader/.
/// Returns (model_path, voice_dir).
/// Uses .version file as atomicity marker — written last.
fn ensure_extracted() -> Result<(PathBuf, String), String> {
    let data_dir = dirs::home_dir()
        .ok_or("Could not determine home directory")?
        .join(".tts-reader");
    let model_path = data_dir.join("model.onnx");
    let version_file = data_dir.join(".version");
    let current_version = env!("CARGO_PKG_VERSION");

    // Atomic guard: extract only if version file missing or outdated
    let needs_extract = !version_file.exists()
        || std::fs::read_to_string(&version_file)
            .unwrap_or_default()
            .trim()
            != current_version;

    if needs_extract {
        eprintln!("[TTS] Extracting model files to {}", data_dir.display());
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data dir: {e}"))?;

        // Write model
        std::fs::write(&model_path, MODEL_BYTES)
            .map_err(|e| format!("Failed to write model.onnx: {e}"))?;

        // Verify model size
        let model_meta = std::fs::metadata(&model_path)
            .map_err(|e| format!("Failed to stat model.onnx: {e}"))?;
        if model_meta.len() != MODEL_BYTES.len() as u64 {
            return Err(format!(
                "Model file truncated: expected {} bytes, got {}",
                MODEL_BYTES.len(),
                model_meta.len()
            ));
        }

        // Write voices in parallel
        std::thread::scope(|s| {
            for (name, bytes) in VOICES {
                let dir = data_dir.clone();
                s.spawn(move || {
                    let path = dir.join(name);
                    if let Err(e) = std::fs::write(&path, bytes) {
                        eprintln!("[TTS] Failed to write {name}: {e}");
                    }
                });
            }
        });

        // Write version LAST — acts as atomicity marker
        std::fs::write(&version_file, current_version)
            .map_err(|e| format!("Failed to write version file: {e}"))?;

        eprintln!("[TTS] Extraction complete");
    }

    let voice_dir = data_dir.to_string_lossy().into_owned();
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
            let voice_dir = std::env::var("KOKORO_VOICE_DIR").unwrap_or_else(|_| model_dir.clone());
            let model_name = std::env::var("KOKORO_MODEL").unwrap_or_else(|_| "model.onnx".to_string());
            let path = PathBuf::from(&model_dir).join(&model_name);
            if !path.exists() {
                return Err(format!("Model file not found: {}", path.display()));
            }
            (path, voice_dir)
        } else {
            ensure_extracted()?
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
            _audio_rx: None,
            stop_flag: Arc::new(AtomicBool::new(false)),
            is_active: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Lazily initialize rodio audio output (must be called on the main thread).
    fn ensure_audio(&mut self) -> Result<(), String> {
        // Safety: self.audio is a valid Box pointer, only accessed from main thread.
        let slot = unsafe { &mut *self.audio };
        if slot.is_some() {
            return Ok(());
        }
        let (stream, handle) = OutputStream::try_default().map_err(|e| format!("Audio output failed: {e}"))?;
        let sink = Sink::try_new(&handle).map_err(|e| format!("Sink creation failed: {e}"))?;
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

        // Reset flags for new session.
        self.stop_flag.store(false, Ordering::Relaxed);
        self.is_active.store(true, Ordering::Relaxed);

        let stop_flag = self.stop_flag.clone();
        let is_active = self.is_active.clone();

        // Create channel for audio chunks.
        let (tx, rx) = mpsc::unbounded_channel::<Vec<f32>>();
        self._audio_rx = Some(rx);

        // Spawn streaming synthesis on a dedicated thread with its own tokio runtime.
        // This avoids "Cannot start a runtime from within a runtime" panics
        // when called from within Dioxus's async context.
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");

            rt.block_on(async {
                let (mut sink, mut stream) = {
                    let tts = match tts.lock() {
                        Ok(tts) => tts,
                        Err(e) => {
                            eprintln!("[TTS] Lock failed: {e}");
                            is_active.store(false, Ordering::Relaxed);
                            return;
                        }
                    };
                    tts.stream(Voice::new(&voice).with_speed(rate))
                };

                if let Err(e) = sink.synth(text).await {
                    eprintln!("[TTS] Synth request failed: {e}");
                    is_active.store(false, Ordering::Relaxed);
                    return;
                }
                drop(sink);

                while let Some((audio, _took)) = stream.next().await {
                    if stop_flag.load(Ordering::Relaxed) {
                        break;
                    }
                    if tx.send(audio).is_err() {
                        break;
                    }
                }

                is_active.store(false, Ordering::Relaxed);
            });
        });
    }

    /// Poll and consume audio chunks from the streaming channel.
    /// Call this periodically from the main thread to feed audio to rodio.
    pub fn poll_audio(&mut self) {
        if let Some(ref mut rx) = self._audio_rx {
            while let Ok(chunk) = rx.try_recv() {
                let audio = unsafe { &*self.audio };
                if let Some(ref state) = audio {
                    let source = SamplesBuffer::new(1, 24000, chunk);
                    state.sink.append(source);
                }
            }
        }
    }

    pub fn stop(&mut self) {
        // Signal the streaming thread to stop.
        self.stop_flag.store(true, Ordering::Relaxed);
        self.is_active.store(false, Ordering::Relaxed);

        // Safety: self.audio is a valid Box pointer, only accessed from main thread.
        let audio = unsafe { &*self.audio };
        if let Some(ref state) = audio {
            state.sink.stop();
        }

        // Clear the receiver.
        self._audio_rx = None;
    }

    pub fn pause(&mut self) {
        // Safety: self.audio is a valid Box pointer, only accessed from main thread.
        let audio = unsafe { &*self.audio };
        if let Some(ref state) = audio {
            state.sink.pause();
        }
    }

    pub fn resume(&mut self) {
        // Safety: self.audio is a valid Box pointer, only accessed from main thread.
        let audio = unsafe { &*self.audio };
        if let Some(ref state) = audio {
            state.sink.play();
        }
    }

    pub fn set_voice(&mut self, voice: &str) {
        self.voice = voice.to_string();
    }

    pub fn is_speaking(&self) -> bool {
        if self.is_active.load(Ordering::Relaxed) {
            return true;
        }
        // Check if rodio sink still has audio to play
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
