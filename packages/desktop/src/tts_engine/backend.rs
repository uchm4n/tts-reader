//! Backend selection and TtsEngine public API.

use super::kokoro::KokoroBackend;
use super::say::SayBackend;

enum Backend {
    Kokoro(KokoroBackend),
    Say(SayBackend),
}

pub struct TtsEngine {
    backend: Backend,
}

impl TtsEngine {
    pub fn new() -> Self {
        let backend = match KokoroBackend::new() {
            Ok(kokoro) => {
                eprintln!("[TTS] Using Kokoro backend");
                Backend::Kokoro(kokoro)
            }
            Err(e) => {
                eprintln!("[TTS] Kokoro unavailable: {e}. Falling back to say command.");
                Backend::Say(SayBackend::new())
            }
        };
        Self { backend }
    }

    pub fn speak(&mut self, text: &str, rate: f32) {
        match &mut self.backend {
            Backend::Kokoro(kokoro) => {
                kokoro.speak(text, rate);
            }
            Backend::Say(say) => {
                say.speak(text, rate);
            }
        }
    }

    pub fn stop(&mut self) {
        match &mut self.backend {
            Backend::Kokoro(kokoro) => kokoro.stop(),
            Backend::Say(say) => say.stop(),
        }
    }

    pub fn is_speaking(&mut self) -> bool {
        match &mut self.backend {
            Backend::Kokoro(kokoro) => kokoro.is_speaking(),
            Backend::Say(say) => say.is_speaking(),
        }
    }

    pub fn set_voice(&mut self, voice: &str) {
        match &mut self.backend {
            Backend::Kokoro(kokoro) => kokoro.set_voice(voice),
            Backend::Say(say) => say.set_voice(voice),
        }
    }
}

impl Default for TtsEngine {
    fn default() -> Self {
        Self::new()
    }
}
