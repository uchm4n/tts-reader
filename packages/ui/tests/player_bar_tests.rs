//! Tests for PlayerBar component.

use dioxus::prelude::*;
use ui::PlayerBar;

fn render_test(f: fn() -> Element) -> String {
    let mut dom = VirtualDom::new(f);
    dom.rebuild_in_place();
    format!("{:?}", dom.render_immediate_to_vec())
}

#[test]
fn player_bar_renders() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_info_hover = move |_: bool| {};
        rsx! {
            PlayerBar {
                is_playing,
                speed,
                on_play,
                on_stop,
                on_info_hover,
            }
        }
    }
    let result = render_test(test_app);
    assert!(!result.is_empty());
}

#[test]
fn player_bar_at_min_speed() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let speed = use_signal(|| 0.5);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_info_hover = move |_: bool| {};
        rsx! {
            PlayerBar {
                is_playing,
                speed,
                on_play,
                on_stop,
                on_info_hover,
            }
        }
    }
    let result = render_test(test_app);
    assert!(!result.is_empty());
}

#[test]
fn player_bar_at_max_speed() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let speed = use_signal(|| 2.0);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_info_hover = move |_: bool| {};
        rsx! {
            PlayerBar {
                is_playing,
                speed,
                on_play,
                on_stop,
                on_info_hover,
            }
        }
    }
    let result = render_test(test_app);
    assert!(!result.is_empty());
}

#[test]
fn player_bar_when_playing() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| true);
        let speed = use_signal(|| 1.0);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_info_hover = move |_: bool| {};
        rsx! {
            PlayerBar {
                is_playing,
                speed,
                on_play,
                on_stop,
                on_info_hover,
            }
        }
    }
    let result = render_test(test_app);
    assert!(!result.is_empty());
}
