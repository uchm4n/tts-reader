//! Tests for text selector functionality.
//!
//! The `get_selected_text()` function requires a display server + accessibility
//! permissions (enigo sends CoreGraphics events). These can't be unit-tested
//! in CI or test harnesses. Instead, we test the fallback logic with a mock
//! selector and verify the factory works.

use tts_reader::text_selector::{create_text_selector, get_text_for_playback, TextSelector};

// --- Mock selector for testing get_text_for_playback logic ---

struct MockSelector {
    text: Option<String>,
}

impl TextSelector for MockSelector {
    fn get_selected_text(&mut self) -> Option<String> {
        self.text.take()
    }

    fn name(&self) -> &str {
        "mock"
    }
}

// --- Factory tests ---

#[test]
fn create_text_selector_returns_correct_name() {
    let selector = create_text_selector();
    assert_eq!(selector.name(), "macOS Clipboard Simulation");
}

// --- get_text_for_playback fallback logic tests ---

#[test]
fn playback_uses_selected_text_when_available() {
    let mut sel = MockSelector {
        text: Some("selected text".into()),
    };
    let result = get_text_for_playback(&mut sel, "clipboard text");
    assert_eq!(result, "selected text");
}

#[test]
fn playback_falls_back_to_clipboard_when_no_selection() {
    let mut sel = MockSelector { text: None };
    let result = get_text_for_playback(&mut sel, "clipboard text");
    assert_eq!(result, "clipboard text");
}

#[test]
fn playback_ignores_empty_selected_text() {
    let mut sel = MockSelector { text: Some("".into()) };
    let result = get_text_for_playback(&mut sel, "clipboard text");
    assert_eq!(result, "clipboard text");
}

#[test]
fn playback_returns_empty_when_both_empty() {
    let mut sel = MockSelector { text: Some("".into()) };
    let result = get_text_for_playback(&mut sel, "");
    assert_eq!(result, "");
}

#[test]
fn playback_uses_clipboard_after_selector_exhausted() {
    let mut sel = MockSelector {
        text: Some("first".into()),
    };
    // First call returns selected text
    assert_eq!(get_text_for_playback(&mut sel, "clip"), "first");
    // Second call falls back to clipboard (selector returns None now)
    assert_eq!(get_text_for_playback(&mut sel, "clip"), "clip");
}

#[test]
fn playback_trims_whitespace_does_not() {
    // Selected text with spaces should still be used (not trimmed)
    let mut sel = MockSelector {
        text: Some("  hello  ".into()),
    };
    let result = get_text_for_playback(&mut sel, "clipboard");
    assert_eq!(result, "  hello  ");
}
