//! macOS text selection via clipboard simulation (Cmd+C).
//!
//! Simulates Cmd+C to copy selected text, reads it from the pasteboard,
//! then restores the original clipboard content. Works with any app
//! that responds to Cmd+C — no Accessibility tree traversal needed.

use std::thread;
use std::time::Duration;

use arboard::Clipboard;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

use super::TextSelector;

pub struct MacOSOption {
    enigo: Option<Enigo>,
}

impl MacOSOption {
    pub fn new() -> Self {
        let enigo = Enigo::new(&Settings::default()).ok();
        if enigo.is_none() {
            eprintln!("[TTS Reader] Failed to initialize input simulator (enigo). Key simulation disabled.");
        }
        Self { enigo }
    }
}

impl TextSelector for MacOSOption {
    fn get_selected_text(&mut self) -> Option<String> {
        let enigo = self.enigo.as_mut()?;

        // 1. Save current clipboard content
        let mut clipboard = Clipboard::new().ok()?;
        let original = clipboard.get_text().ok().map(|s| s.to_string());

        // 2. Simulate Cmd+C via enigo (CoreGraphics event posting)
        enigo.key(Key::Meta, Press).ok()?;
        enigo.key(Key::Unicode('c'), Click).ok()?;
        enigo.key(Key::Meta, Release).ok()?;

        // 3. Wait for the pasteboard to update
        thread::sleep(Duration::from_millis(100));

        // 4. Read the copied text
        let copied = clipboard.get_text().ok().filter(|t| !t.is_empty())?;

        // 5. Restore the user's original clipboard content
        if let Some(orig) = original {
            let _ = clipboard.set_text(&orig);
        }

        Some(copied)
    }

    fn name(&self) -> &str {
        "macOS Clipboard Simulation"
    }
}
