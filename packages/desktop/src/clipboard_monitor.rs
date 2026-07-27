//! Clipboard monitoring using pbpaste command.

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
    Command::new("pbpaste")
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        .unwrap_or_default()
}
