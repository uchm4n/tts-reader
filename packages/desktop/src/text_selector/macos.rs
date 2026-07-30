//! macOS text selection via Accessibility API (AXUIElement).

use axuielement::ax_attribute::{AX_FOCUSED_UI_ELEMENT_ATTRIBUTE, AX_SELECTED_TEXT_ATTRIBUTE};
use axuielement::prelude::*;

use super::TextSelector;

pub struct MacOSOption {
    enabled: bool,
}

impl MacOSOption {
    pub fn new() -> Self {
        let enabled = api_enabled();
        if !enabled {
            eprintln!(
                "[TTS Reader] Accessibility permissions not enabled.\n\
                 Go to System Settings → Privacy & Security → Accessibility\n\
                 and add this application. Then restart the app."
            );
        }
        Self { enabled }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl TextSelector for MacOSOption {
    fn get_selected_text(&self) -> Option<String> {
        if !self.enabled {
            eprintln!(
                "[TTS Reader] Accessibility permissions not granted. Falling back to clipboard."
            );
            return None;
        }

        let system = SystemWideElement::new()?;

        let focused_app = match system.focused_application() {
            Ok(Some(app)) => app,
            Ok(None) => {
                eprintln!("[TTS Reader] No focused application found");
                return None;
            }
            Err(e) => {
                eprintln!("[TTS Reader] Failed to get focused application: {:?}", e);
                return None;
            }
        };

        let focused_element = match focused_app.element_attribute(AX_FOCUSED_UI_ELEMENT_ATTRIBUTE) {
            Ok(Some(el)) => el,
            Ok(None) => {
                eprintln!("[TTS Reader] No focused UI element found");
                return None;
            }
            Err(e) => {
                eprintln!("[TTS Reader] Failed to get focused UI element: {:?}", e);
                return None;
            }
        };

        let selected_text = match focused_element.string_attribute(AX_SELECTED_TEXT_ATTRIBUTE) {
            Ok(Some(text)) if !text.is_empty() => text,
            Ok(Some(_)) => {
                eprintln!("[TTS Reader] Selected text is empty");
                return None;
            }
            Ok(None) => {
                eprintln!("[TTS Reader] AXSelectedText attribute not available on focused element");
                return None;
            }
            Err(e) => {
                eprintln!("[TTS Reader] Failed to read selected text: {:?}", e);
                return None;
            }
        };

        Some(selected_text)
    }

    fn name(&self) -> &str {
        "macOS Accessibility"
    }
}
