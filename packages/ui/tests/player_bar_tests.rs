//! Tests for PlayerBar component.

use dioxus::prelude::*;
use dioxus_core::Mutation;
use ui::{AlwaysOnTopEvent, PlayerBar};

fn render_test(f: fn() -> Element) -> Vec<Mutation> {
    let mut dom = VirtualDom::new(f);
    dom.rebuild_to_vec().edits
}

fn has_attribute(edits: &[Mutation], name: &str, value_contains: &str) -> bool {
    edits.iter().any(|m| matches!(m, Mutation::SetAttribute { name: n, value, .. } if *n == name && format!("{value:?}").contains(value_contains)))
}

fn has_text(edits: &[Mutation], value: &str) -> bool {
    edits
        .iter()
        .any(|m| matches!(m, Mutation::CreateTextNode { value: v, .. } if v == value))
}

#[test]
fn player_bar_renders() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(!edits.is_empty());
}

#[test]
fn player_bar_at_min_speed() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 0.5);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(!edits.is_empty());
}

#[test]
fn player_bar_at_max_speed() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 2.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(!edits.is_empty());
}

#[test]
fn player_bar_when_playing() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| true);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(!edits.is_empty());
}

#[test]
fn player_bar_always_on_top_active() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| true);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(has_attribute(&edits, "class", "always-on-top-btn active"));
}

#[test]
fn player_bar_always_on_top_inactive() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    let has_active = edits.iter().any(|m| {
        matches!(m, Mutation::SetAttribute { name: "class", value, .. }
            if format!("{value:?}").contains("active") && format!("{value:?}").contains("always-on-top"))
    });
    assert!(!has_active);
}

#[test]
fn always_on_top_event_variants_exist() {
    let hover_enter = AlwaysOnTopEvent::HoverEnter;
    let hover_leave = AlwaysOnTopEvent::HoverLeave;
    let toggle = AlwaysOnTopEvent::Toggle;

    assert_ne!(hover_enter, hover_leave);
    assert_ne!(hover_enter, toggle);
    assert_ne!(hover_leave, toggle);

    assert_eq!(hover_enter, AlwaysOnTopEvent::HoverEnter);
    assert_eq!(hover_leave, AlwaysOnTopEvent::HoverLeave);
    assert_eq!(toggle, AlwaysOnTopEvent::Toggle);
}

#[test]
fn always_on_top_event_debug_format() {
    assert_eq!(format!("{:?}", AlwaysOnTopEvent::HoverEnter), "HoverEnter");
    assert_eq!(format!("{:?}", AlwaysOnTopEvent::HoverLeave), "HoverLeave");
    assert_eq!(format!("{:?}", AlwaysOnTopEvent::Toggle), "Toggle");
}

#[test]
fn player_bar_speed_label_at_default() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(has_text(&edits, "1.00x"));
}

#[test]
fn player_bar_play_button_title() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| false);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(has_attribute(&edits, "title", "Play"));
}

#[test]
fn player_bar_pause_button_title_when_playing() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| true);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(has_attribute(&edits, "title", "Pause"));
}

// --- New is_paused tests ---

#[test]
fn player_bar_resume_button_title_when_paused() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| true);
        let is_paused = use_signal(|| true);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    assert!(has_attribute(&edits, "title", "Resume"));
}

#[test]
fn player_bar_shows_play_icon_when_paused() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| true);
        let is_paused = use_signal(|| true);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    // When paused, should show PlayIcon (not PauseIcon)
    // The title should be "Resume"
    assert!(has_attribute(&edits, "title", "Resume"));
}

#[test]
fn player_bar_shows_pause_icon_when_playing_not_paused() {
    fn test_app() -> Element {
        let is_playing = use_signal(|| true);
        let is_paused = use_signal(|| false);
        let speed = use_signal(|| 1.0);
        let voice = use_signal(|| "af_heart".to_string());
        let is_always_on_top = use_signal(|| false);
        let on_play = move |_| {};
        let on_stop = move |_| {};
        let on_always_on_top = move |_| {};
        rsx! {
            PlayerBar {
                is_playing,
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play,
                on_stop,
                on_voice_change: move |_| {},
                on_always_on_top,
                on_play_pause_hover: move |_| {},
            }
        }
    }
    let edits = render_test(test_app);
    // When playing and not paused, should show PauseIcon
    // The title should be "Pause"
    assert!(has_attribute(&edits, "title", "Pause"));
}
