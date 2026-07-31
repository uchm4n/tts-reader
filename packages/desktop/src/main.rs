//! TTS Reader - Minimal desktop application.

use std::time::Duration;

use dioxus::prelude::*;
use dioxus_desktop::{
    icon_from_memory, use_global_shortcut, use_muda_event_handler, use_window,
    use_wry_event_handler, Config, HotKeyState, LogicalSize, WindowBuilder, WindowCloseBehaviour,
};
use dioxus_desktop::trayicon::{init_tray_icon, menu::{Menu, MenuItem}};
#[cfg(target_os = "macos")]
use dioxus_desktop::tao::platform::macos::EventLoopWindowTargetExtMacOS;

mod clipboard_monitor;
mod text_selector;
mod tts_engine;

use clipboard_monitor::use_clipboard_monitor;
use text_selector::{create_text_selector, get_text_for_playback};
use tts_engine::TtsEngine;
use tts_reader::tray::{TRAY_SHOW_ID, TRAY_QUIT_ID};
use ui::{AlwaysOnTopEvent, PlayerBar};

const APP_NAME: &str = "TTS Reader";

fn main() {
    LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_close_behaviour(WindowCloseBehaviour::WindowHides)
                .with_tray_icon_show_window_on_click(true)
                .with_window(
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
    // Initialize tray icon (must be inside Dioxus runtime)
    let menu = Menu::new();
    let _show_item = MenuItem::with_id(TRAY_SHOW_ID, "Show", true, None);
    let _quit_item = MenuItem::with_id(TRAY_QUIT_ID, "Quit", true, None);
    menu.append(&_show_item).unwrap();
    menu.append(&_quit_item).unwrap();
    let icon = icon_from_memory(include_bytes!("../assets/logo.png")).ok();
    let _tray = init_tray_icon(menu, icon);

    // Signal to coordinate dock visibility restoration on macOS
    #[cfg(target_os = "macos")]
    let should_restore_dock = use_signal(|| false);

    // macOS: Hide from dock when window is closed (CloseRequested fires but window only hides)
    #[cfg(target_os = "macos")]
    use_wry_event_handler(move |event, target| {
        if let dioxus_desktop::tao::event::Event::WindowEvent {
            event: dioxus_desktop::tao::event::WindowEvent::CloseRequested,
            ..
        } = event
        {
            target.set_dock_visibility(false);
        }
    });

    // macOS: Restore dock visibility when "Show" is clicked from tray menu
    #[cfg(target_os = "macos")]
    use_wry_event_handler({
        let mut should_restore_dock = should_restore_dock;
        move |_event, target| {
            if *should_restore_dock.read() {
                target.set_dock_visibility(true);
                *should_restore_dock.write() = false;
            }
        }
    });

    let mut is_playing = use_signal(|| false);
    let mut is_paused = use_signal(|| false);
    let speed = use_signal(|| 1.0);
    let mut tts = use_signal(|| None::<TtsEngine>);
    let clipboard_text = use_clipboard_monitor();
    let window = use_window();

    // Handle tray menu events
    use_muda_event_handler({
        let window = window.clone();
        #[cfg(target_os = "macos")]
        let mut should_restore_dock = should_restore_dock;
        move |event| {
            if event.id.0 == TRAY_SHOW_ID {
                #[cfg(target_os = "macos")]
                {
                    *should_restore_dock.write() = true;
                }
                window.set_visible(true);
                window.set_focus();
            } else if event.id.0 == TRAY_QUIT_ID {
                std::process::exit(0);
            }
        }
    });

    let mut is_always_on_top = use_signal(|| false);
    let mut voice = use_signal(|| {
        // Load .env to get KOKORO_VOICE
        let env_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../.env");
        dotenvy::from_path(&env_path).ok();
        std::env::var("KOKORO_VOICE").unwrap_or_else(|_| "af_heart".to_string())
    });
    let mut selector = create_text_selector();

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
                let text = get_text_for_playback(selector.as_mut(), &clipboard_text());
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

    // Global shortcut: Cmd+Escape to stop playback
    use_global_shortcut("Cmd+Escape", move |state| {
        if let HotKeyState::Pressed = state {
            if let Some(ref mut engine) = *tts.write() {
                engine.stop();
            }
            *is_playing.write() = false;
            *is_paused.write() = false;
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

    let mut selector_play = create_text_selector();
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
            let text = get_text_for_playback(selector_play.as_mut(), &clipboard_text());
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
