//! Tests for text selector functionality.
#![cfg(target_os = "macos")]

#[test]
fn text_selector_creation_works() {
    let selector = tts_reader::text_selector::create_text_selector();
    assert_eq!(selector.name(), "macOS Accessibility");
}


#[test]
fn get_selected_text_returns_option() {
    let selector = tts_reader::text_selector::create_text_selector();
    // This may return None if no text is selected or permissions aren't granted
    // We just verify it doesn't panic
    let _result = selector.get_selected_text();
}

#[test]
fn get_selected_text_with_no_selection() {
    let selector = tts_reader::text_selector::create_text_selector();
    // When no text is selected, should either return None or clipboard content
    let result = selector.get_selected_text();
    // We can't assert on the value since it depends on system state
    // Just verify it returns a valid Option
    match result {
        Some(text) => assert!(!text.is_empty()),
        None => {}
    }
}
