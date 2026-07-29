//! TTS Reader - Minimal desktop application.

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_desktop::{
    use_global_shortcut, use_window, Config, HotKeyState, LogicalSize, WindowBuilder,
};

mod clipboard_monitor;
mod tts_engine;
mod text_selector;

use clipboard_monitor::use_clipboard_monitor;
use tts_engine::TtsEngine;
use text_selector::create_text_selector;
use text_selector::is_text_selection_available;
use ui::{AlwaysOnTopEvent, PlayerBar};

const APP_NAME: &str = "TTS Reader";

/// Get text for playback: try selected text first, fall back to clipboard.
fn get_text_for_playback(
    selector: &dyn text_selector::TextSelector,
    clipboard_text: &str,
) -> String {
    selector
        .get_selected_text()
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| clipboard_text.to_string())
}

fn main() {
    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::new().with_window(
                WindowBuilder::new()
                    .with_title(APP_NAME)
                    .with_inner_size(LogicalSize::new(290.0, 80.0))
                    .with_resizable(false)
                    .with_always_on_top(false),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut is_playing = use_signal(|| false);
    let mut is_paused = use_signal(|| false);
    let speed = use_signal(|| 1.0);
    let mut tts = use_signal(|| None::<TtsEngine>);
    let clipboard_text = use_clipboard_monitor();
    let window = use_window();
    let mut is_always_on_top = use_signal(|| false);
    let mut voice = use_signal(|| {
        // Load .env to get KOKORO_VOICE
        let env_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.env");
        dotenvy::from_path(&env_path).ok();
        std::env::var("KOKORO_VOICE").unwrap_or_else(|_| "af_heart".to_string())
    });
    let selector = create_text_selector();

    // One-time warning if accessibility permissions are not granted
    use_effect(move || {
        if !is_text_selection_available() {
            eprintln!(
                "[TTS Reader] Accessibility permissions not granted.\n\
                 Selected text reading is disabled. The app will use clipboard text instead.\n\
                 To enable: System Settings → Privacy & Security → Accessibility → add this app.\n\
                 Then restart the app."
            );
        }
    });

    // Initialize TTS engine on a background thread to avoid blocking the webview
    spawn(async move {
        let engine = tokio::task::spawn_blocking(TtsEngine::new).await;
        match engine {
            Ok(engine) => {
                eprintln!("[TTS] Engine initialized");
                *tts.write() = Some(engine);
            }
            Err(e) => {
                eprintln!("[TTS] Engine init failed: {e}");
            }
        }
    });

    // Global shortcut: Cmd+Shift+R to toggle play/pause
    use_global_shortcut("Cmd+Shift+R", move |state| {
        if let HotKeyState::Pressed = state {
            if is_playing() {
                if let Some(ref mut engine) = *tts.write() {
                    if is_paused() {
                        engine.resume();
                        *is_paused.write() = false;
                    } else {
                        engine.pause();
                        *is_paused.write() = true;
                    }
                }
            } else {
                let text = get_text_for_playback(selector.as_ref(), &clipboard_text());
                if !text.is_empty() {
                    if let Some(ref mut engine) = *tts.write() {
                        engine.speak(&text, speed());
                        *is_playing.write() = true;
                        *is_paused.write() = false;
                    }
                }
            }
        }
    })
    .ok();

    // Poll is_speaking() and reset is_playing when speech finishes
    spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;
            if let Some(ref mut engine) = *tts.write() {
                // Poll audio chunks from the streaming channel and feed to rodio
                engine.poll_audio();

                if is_playing() && !is_paused() {
                    let speaking = engine.is_speaking();
                    if !speaking {
                        *is_playing.write() = false;
                        *is_paused.write() = false;
                    }
                }
            }
        }
    });

    let selector_play = create_text_selector();
    let handle_play = move |_| {
        if is_playing() {
            if let Some(ref mut engine) = *tts.write() {
                if is_paused() {
                    engine.resume();
                    *is_paused.write() = false;
                } else {
                    engine.pause();
                    *is_paused.write() = true;
                }
            }
        } else {
            let text = get_text_for_playback(selector_play.as_ref(), &clipboard_text());
            if !text.is_empty() {
                if let Some(ref mut engine) = *tts.write() {
                    engine.speak(&text, speed());
                    *is_playing.write() = true;
                    *is_paused.write() = false;
                }
            }
        }
    };

    let handle_stop = move |_| {
        if let Some(ref mut engine) = *tts.write() {
            engine.stop();
        }
        *is_playing.write() = false;
        *is_paused.write() = false;
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

    let handle_always_on_top = move |event: AlwaysOnTopEvent| match event {
        AlwaysOnTopEvent::HoverEnter => {
            let state = if !is_always_on_top() { "On" } else { "Off" };
            window.set_title(&format!("Always On Top: {state}"));
        }
        AlwaysOnTopEvent::HoverLeave => window.set_title(APP_NAME),
        AlwaysOnTopEvent::Toggle => {
            let new_state = !is_always_on_top();
            window.set_always_on_top(new_state);
            *is_always_on_top.write() = new_state;
        }
    };

    let handle_voice_change = {
        let mut tts = tts.clone();
        move |new_voice: String| {
            *voice.write() = new_voice.clone();
            if let Some(ref mut engine) = *tts.write() {
                engine.set_voice(&new_voice);
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
                is_paused,
                speed,
                voice,
                is_always_on_top,
                on_play: handle_play,
                on_stop: handle_stop,
                on_voice_change: handle_voice_change,
                on_always_on_top: handle_always_on_top,
                on_play_pause_hover: handle_play_pause_hover,
            }
        }
    }
}
