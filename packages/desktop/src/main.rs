//! TTS Reader - Minimal desktop application.

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_desktop::{use_global_shortcut, use_window, HotKeyState, Config, WindowBuilder, LogicalSize};

mod clipboard_monitor;
mod tts_engine;

use clipboard_monitor::use_clipboard_monitor;
use tts_engine::TtsEngine;
use ui::PlayerBar;

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_title("TTS Reader")
                        .with_inner_size(LogicalSize::new(290.0, 48.0))
                        .with_resizable(false)
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

    let handle_info_hover = move |hovering: bool| {
        if hovering {
            window.set_title("Cmd+Shift+R: Play/Pause");
        } else {
            window.set_title("TTS Reader");
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
                on_play: handle_play,
                on_stop: handle_stop,
                on_info_hover: handle_info_hover,
            }
        }
    }
}
