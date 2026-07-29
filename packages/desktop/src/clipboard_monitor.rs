//! Clipboard monitoring using platform-specific commands.

use std::process::Command;
use std::time::Duration;

use dioxus::prelude::*;

pub fn use_clipboard_monitor() -> Signal<String> {
    let mut clipboard_text = use_signal(String::new);

    spawn(async move {
        let mut last_text = String::new();

        loop {
            let current = get_clipboard_text();
            if !current.is_empty() && current != last_text {
                last_text = current.clone();
                *clipboard_text.write() = current;
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    });

    clipboard_text
}

fn get_clipboard_text() -> String {
    #[cfg(target_os = "macos")]
    {
        Command::new("pbpaste")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .unwrap_or_default()
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xclip")
            .args(["-selection", "clipboard", "-o"])
            .output()
            .or_else(|_| {
                Command::new("xsel")
                    .args(["--clipboard", "--output"])
                    .output()
            })
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .unwrap_or_default()
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("powershell")
            .args(["-command", "Get-Clipboard"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
            .unwrap_or_default()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        String::new()
    }
}
