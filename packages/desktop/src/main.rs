//! TTS Reader - Minimal desktop application.

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_desktop::{use_global_shortcut, use_window, HotKeyState, Config, WindowBuilder, LogicalSize};

mod clipboard_monitor;
mod tts_engine;

use clipboard_monitor::use_clipboard_monitor;
use tts_engine::TtsEngine;
use ui::{PlayerBar, AlwaysOnTopEvent};

const APP_NAME: &str = "TTS Reader";

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_title(APP_NAME)
                        .with_inner_size(LogicalSize::new(290.0, 48.0))
                        .with_resizable(false)
                        .with_always_on_top(false)
                )
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut is_playing = use_signal(|| false);
    let speed = use_signal(|| 1.0);
    let mut tts = use_signal(|| TtsEngine::new());
    let clipboard_text = use_clipboard_monitor();
    let window = use_window();
    let mut is_always_on_top = use_signal(|| false);

    // Global shortcut: Cmd+Shift+R to toggle play/pause
    use_global_shortcut("Cmd+Shift+R", move |state| {
        if let HotKeyState::Pressed = state {
            if is_playing() {
                tts.write().stop();
                *is_playing.write() = false;
            } else {
                let text = clipboard_text();
                if !text.is_empty() {
                    tts.write().speak(&text, speed());
                    *is_playing.write() = true;
                }
            }
        }
    }).ok();

    // Poll is_speaking() and reset is_playing when speech finishes
    spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            if is_playing() && !tts.write().is_speaking() {
                *is_playing.write() = false;
            }
        }
    });

    let handle_play = move |_| {
        if is_playing() {
            tts.write().stop();
            *is_playing.write() = false;
        } else {
            let text = clipboard_text();
            if !text.is_empty() {
                tts.write().speak(&text, speed());
                *is_playing.write() = true;
            }
        }
    };

    let handle_stop = move |_| {
        tts.write().stop();
        *is_playing.write() = false;
    };

    let handle_play_pause_hover = {
        let window = window.clone();
        move |hovering: bool| {
            if hovering {
                window.set_title("⌘+⇧+R: Play/Pause");
            } else {
                window.set_title(APP_NAME);
            }
        }
    };

    let handle_always_on_top = move |event: AlwaysOnTopEvent| {
        match event {
            AlwaysOnTopEvent::HoverEnter => {
                let state = if !is_always_on_top() { "On" } else { "Off" };
                window.set_title(&format!("Always On Top: {state}"));
            },
            AlwaysOnTopEvent::HoverLeave => window.set_title(APP_NAME),
            AlwaysOnTopEvent::Toggle => {
                let new_state = !is_always_on_top();
                window.set_always_on_top(new_state);
                *is_always_on_top.write() = new_state;
            }
        }
    };

    rsx! {
        document::Stylesheet {
            href: asset!("/assets/main.css"),
        }
        div {
            class: "app",
            PlayerBar {
                is_playing,
                speed,
                is_always_on_top,
                on_play: handle_play,
                on_stop: handle_stop,
                on_always_on_top: handle_always_on_top,
                on_play_pause_hover: handle_play_pause_hover,
            }
        }
    }
}
