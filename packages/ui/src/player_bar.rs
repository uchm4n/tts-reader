//! Minimal player bar with speed and playback controls.

#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::icons::{FastBackwardIcon, FastForwardIcon, InfoIcon, PauseIcon, PlayIcon, StopIcon};

#[component]
pub fn PlayerBar(
    is_playing: Signal<bool>,
    speed: Signal<f32>,
    on_play: EventHandler<()>,
    on_stop: EventHandler<()>,
    on_info_hover: EventHandler<bool>,
) -> Element {
    let speeds = [0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0];
    let current_speed = speed();

    let prev_speed = speeds.iter().rev().find(|&&s| s < current_speed - 0.01).copied();
    let next_speed = speeds.iter().find(|&&s| s > current_speed + 0.01).copied();

    rsx! {
        div {
            class: "player-bar",
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
                title: if is_playing() { "Pause" } else { "Play" },
                onclick: move |_| on_play.call(()),
                if is_playing() {
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
                class: "icon-btn info-btn",
                onmouseenter: move |_| on_info_hover.call(true),
                onmouseleave: move |_| on_info_hover.call(false),
                InfoIcon {}
            }
        }
    }
}
