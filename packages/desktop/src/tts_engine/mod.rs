//! TTS engine with Kokoro primary backend and macOS `say` fallback.

mod backend;
mod kokoro;
mod say;

pub use backend::TtsEngine;
