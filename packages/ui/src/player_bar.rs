//! Minimal player bar with speed, playback controls, and voice selector.

#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::icons::{
    AlwaysOnTopIcon, FastBackwardIcon, FastForwardIcon, PauseIcon, PlayIcon, StopIcon,
};
use crate::voices::VOICES;
use crate::AlwaysOnTopEvent;

#[component]
pub fn PlayerBar(
    is_playing: Signal<bool>,
    is_paused: Signal<bool>,
    speed: Signal<f32>,
    voice: Signal<String>,
    is_always_on_top: Signal<bool>,
    on_play: EventHandler<()>,
    on_stop: EventHandler<()>,
    on_voice_change: EventHandler<String>,
    on_always_on_top: EventHandler<AlwaysOnTopEvent>,
    on_play_pause_hover: EventHandler<bool>,
) -> Element {
    let speeds = [0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0];
    let current_speed = speed();

    let prev_speed = speeds
        .iter()
        .rev()
        .find(|&&s| s < current_speed - 0.01)
        .copied();
    let next_speed = speeds.iter().find(|&&s| s > current_speed + 0.01).copied();

    rsx! {
        div {
            class: "player-bar",
            div {
                class: "controls-row",
                button {
                    class: "icon-btn",
                    disabled: prev_speed.is_none(),
                    onclick: move |_| {
                        if let Some(s) = prev_speed {
                            *speed.write() = s;
                        }
                    },
                    FastBackwardIcon {}
                }
                button {
                    class: "icon-btn play",
                    title: if is_playing() {
                        if is_paused() { "Resume" } else { "Pause" }
                    } else {
                        "Play"
                    },
                    onclick: move |_| on_play.call(()),
                    onmouseenter: move |_| on_play_pause_hover.call(true),
                    onmouseleave: move |_| on_play_pause_hover.call(false),
                    if is_playing() && !is_paused() {
                        PauseIcon {}
                    } else {
                        PlayIcon {}
                    }
                }
                button {
                    class: "icon-btn",
                    title: "Stop",
                    onclick: move |_| on_stop.call(()),
                    StopIcon {}
                }
                button {
                    class: "icon-btn",
                    disabled: next_speed.is_none(),
                    onclick: move |_| {
                        if let Some(s) = next_speed {
                            *speed.write() = s;
                        }
                    },
                    FastForwardIcon {}
                }
                span {
                    class: "speed-label",
                    "{current_speed:.2}x"
                }
                button {
                    class: if is_always_on_top() { "icon-btn always-on-top-btn active" } else { "icon-btn always-on-top-btn" },
                    title: if is_always_on_top() { "Always On Top: On" } else { "Always On Top: Off" },
                    onmouseenter: move |_| on_always_on_top.call(AlwaysOnTopEvent::HoverEnter),
                    onmouseleave: move |_| on_always_on_top.call(AlwaysOnTopEvent::HoverLeave),
                    onclick: move |_| on_always_on_top.call(AlwaysOnTopEvent::Toggle),
                    AlwaysOnTopIcon {}
                }
            }
            select {
                class: "voice-selector",
                value: "{voice()}",
                onchange: move |e| on_voice_change.call(e.value()),
                for (id, name, nationality, gender) in VOICES {
                    option {
                        value: "{id}",
                        selected: *id == voice(),
                        "{name} ({nationality} {gender})"
                    }
                }
            }
        }
    }
}
