//! Tests for icon components.

use dioxus::prelude::*;
use ui::icons::*;

fn render_test(f: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(f);
    dom.rebuild_in_place();
    format!("{:?}", dom.render_immediate_to_vec())
}

#[test]
fn play_icon_renders() {
    fn test_app() -> Element { rsx! { PlayIcon {} } }
    assert!(!render_test(test_app).is_empty());
}

#[test]
fn pause_icon_renders() {
    fn test_app() -> Element { rsx! { PauseIcon {} } }
    assert!(!render_test(test_app).is_empty());
}

#[test]
fn stop_icon_renders() {
    fn test_app() -> Element { rsx! { StopIcon {} } }
    assert!(!render_test(test_app).is_empty());
}

#[test]
fn fast_backward_icon_renders() {
    fn test_app() -> Element { rsx! { FastBackwardIcon {} } }
    assert!(!render_test(test_app).is_empty());
}

#[test]
fn fast_forward_icon_renders() {
    fn test_app() -> Element { rsx! { FastForwardIcon {} } }
    assert!(!render_test(test_app).is_empty());
}

#[test]
fn always_on_top_icon_renders() {
    fn test_app() -> Element { rsx! { AlwaysOnTopIcon {} } }
    assert!(!render_test(test_app).is_empty());
}
