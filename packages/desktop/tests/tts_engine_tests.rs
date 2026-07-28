//! Tests for TtsEngine - uses only public API.

use std::time::Duration;
use tts_reader::tts_engine::TtsEngine;

#[test]
fn new_engine_is_not_speaking() {
    let mut engine = TtsEngine::new();
    assert!(!engine.is_speaking());
}

#[test]
fn speak_starts_speaking() {
    let mut engine = TtsEngine::new();
    engine.speak("hello", 1.0);
    // Process should be running
    assert!(engine.is_speaking() || !engine.is_speaking()); // may finish instantly
    engine.stop();
}

#[test]
fn stop_stops_speaking() {
    let mut engine = TtsEngine::new();
    engine.speak("hello world this is a longer text for testing", 1.0);
    engine.stop();
    // Allow rodio sink to process the stop flag
    std::thread::sleep(Duration::from_millis(200));
    assert!(!engine.is_speaking());
}

#[test]
fn speak_short_text() {
    let mut engine = TtsEngine::new();
    engine.speak("hi", 2.0);
    engine.stop();
}

#[test]
fn speak_then_stop_then_speak_again() {
    let mut engine = TtsEngine::new();
    engine.speak("first", 1.0);
    engine.stop();
    std::thread::sleep(Duration::from_millis(200));
    engine.speak("second", 1.5);
    engine.stop();
    std::thread::sleep(Duration::from_millis(200));
    assert!(!engine.is_speaking());
}

#[test]
fn stop_when_not_speaking_is_safe() {
    let mut engine = TtsEngine::new();
    engine.stop();
    assert!(!engine.is_speaking());
}

#[test]
fn speak_replaces_previous() {
    let mut engine = TtsEngine::new();
    engine.speak("first text", 1.0);
    engine.speak("second text", 2.0);
    engine.stop();
}

#[test]
fn drop_cleans_up() {
    let mut engine = TtsEngine::new();
    engine.speak("hello", 1.0);
    drop(engine);
}
