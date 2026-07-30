//! macOS text selection via Accessibility API (AXUIElement).

use axuielement::ax_attribute::roles::{
    AX_GROUP_ROLE, AX_SCROLL_AREA_ROLE, AX_STATIC_TEXT_ROLE, AX_TEXT_AREA_ROLE,
};
use axuielement::ax_attribute::{AX_FOCUSED_UI_ELEMENT_ATTRIBUTE, AX_SELECTED_TEXT_ATTRIBUTE, AX_ROLE_ATTRIBUTE};
use axuielement::prelude::*;
use axuielement::process_trust::{is_process_trusted, is_process_trusted_with_prompt};

use super::TextSelector;

// AXWebArea role is not defined in axuielement crate
const AX_WEB_AREA_ROLE: &str = "AXWebArea";

pub struct MacOSOption;

impl MacOSOption {
    pub fn new() -> Self {
        // Always try the AX API — it handles its own permission errors gracefully.
        // Only prompt for permissions on first launch (not every time).
        if !is_process_trusted() {
            let _prompted = is_process_trusted_with_prompt();
            eprintln!(
                "[TTS Reader] Accessibility permissions not yet granted for this app.\n\
                 If a dialog appeared, please allow access. If not, go to:\n\
                 System Settings → Privacy & Security → Accessibility\n\
                 and add this application. Then restart the app."
            );
        }
        Self
    }
}

impl TextSelector for MacOSOption {
    fn get_selected_text(&self) -> Option<String> {
        // Don't check is_process_trusted() here — it's unreliable for ad-hoc signed apps.
        // Just try the AX API calls. They'll fail gracefully if not trusted.
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

        // Try to get focused element (may be None for Chrome with mouse selection)
        let focused_element = focused_app
            .element_attribute(AX_FOCUSED_UI_ELEMENT_ATTRIBUTE)
            .ok()
            .flatten();

        // Step 1: Check focused element first (fast path) — works for PDF readers, IDEs, address bar
        if let Some(ref el) = focused_element {
            if let Ok(Some(text)) = el.string_attribute(AX_SELECTED_TEXT_ATTRIBUTE) {
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }

        // Step 2: Search application -> windows -> AXWebArea (for browsers)
        if let Some(text) = search_web_area_for_selected_text(&focused_app) {
            return Some(text);
        }

        // Step 3: Search children by role priority from focused element
        if let Some(ref el) = focused_element {
            let roles = [
                AX_TEXT_AREA_ROLE,   // AXTextArea - text fields, form inputs
                AX_WEB_AREA_ROLE,    // AXWebArea - browser web content
                AX_GROUP_ROLE,       // AXGroup - general containers (PDF readers, etc.)
                AX_SCROLL_AREA_ROLE, // AXScrollArea - scrollable content
                AX_STATIC_TEXT_ROLE, // AXStaticText - individual text elements
            ];

            for role in &roles {
                if let Some(text) = search_children_by_role(el, role) {
                    return Some(text);
                }
            }
        }

        // Step 4: Activate app and retry — Chrome/browsers fallback
        // Chrome's accessibility tree doesn't expose AXSelectedText unless the app is frontmost
        activate_app(&focused_app);

        // Retry fast path with activated app
        if let Some(ref el) = focused_element {
            if let Ok(Some(text)) = el.string_attribute(AX_SELECTED_TEXT_ATTRIBUTE) {
                if !text.is_empty() {
                    return Some(text);
                }
            }
        }

        // Retry AXWebArea search with activated app
        if let Some(text) = search_web_area_for_selected_text(&focused_app) {
            return Some(text);
        }

        // Retry role-based search with activated app
        if let Some(ref el) = focused_element {
            let roles = [
                AX_TEXT_AREA_ROLE,
                AX_WEB_AREA_ROLE,
                AX_GROUP_ROLE,
                AX_SCROLL_AREA_ROLE,
                AX_STATIC_TEXT_ROLE,
            ];

            for role in &roles {
                if let Some(text) = search_children_by_role(el, role) {
                    return Some(text);
                }
            }
        }

        // Step 5: Clipboard fallback — last resort
        clipboard_copy_and_read()
    }

    fn name(&self) -> &str {
        "macOS Accessibility"
    }
}

/// Search application -> windows -> AXWebArea for selected text.
/// This handles Chrome and other browsers where AXFocusedUIElement may be None
/// or where the AXWebArea is an ancestor of the focused element.
fn search_web_area_for_selected_text(app: &AXUIElement) -> Option<String> {
    // Get windows from the application
    let windows = match app.children() {
        Ok(children) => children,
        Err(_) => return None,
    };

    for window in windows {
        // Check if this is a window
        if let Ok(Some(role)) = window.string_attribute(AX_ROLE_ATTRIBUTE) {
            if role != "AXWindow" {
                continue;
            }
        } else {
            continue;
        }

        // Search for AXWebArea in this window
        if let Some(text) = search_children_by_role(&window, AX_WEB_AREA_ROLE) {
            return Some(text);
        }
    }

    None
}

/// Search children recursively for a specific role with non-empty AXSelectedText.
fn search_children_by_role(element: &AXUIElement, target_role: &str) -> Option<String> {
    let children = match element.children() {
        Ok(children) => children,
        Err(_) => return None,
    };

    for child in children {
        // Check if this child has the target role
        if let Ok(Some(role)) = child.string_attribute(AX_ROLE_ATTRIBUTE) {
            if role == target_role {
                // Check for selected text
                if let Ok(Some(text)) = child.string_attribute(AX_SELECTED_TEXT_ATTRIBUTE) {
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }

        // Recurse into children
        if let Some(text) = search_children_by_role(&child, target_role) {
            return Some(text);
        }
    }

    None
}

/// Activate the focused app before querying its accessibility tree.
/// Mirrors the AppleScript: tell application "Google Chrome" to activate
fn activate_app(app: &AXUIElement) {
    if let Ok(pid) = app.pid() {
        let _ = std::process::Command::new("osascript")
            .args([
                "-e",
                &format!(
                    "tell application \"System Events\" to set frontmost of first process whose unix id is {} to true",
                    pid
                ),
            ])
            .output();
        // Brief delay to let activation complete
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}

/// Simulate Cmd+C and read clipboard, preserving original content.
fn clipboard_copy_and_read() -> Option<String> {
    use std::io::Write;

    // Save current clipboard
    let original = std::process::Command::new("pbpaste")
        .output()
        .ok()
        .map(|o| o.stdout)?;

    // Simulate Cmd+C
    std::process::Command::new("osascript")
        .args(["-e", "tell application \"System Events\" to keystroke \"c\" using command down"])
        .output()
        .ok()?;

    // Wait for clipboard to update
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Read clipboard
    let copied = std::process::Command::new("pbpaste")
        .output()
        .ok()
        .map(|o| o.stdout)?;

    // Restore original clipboard
    let mut pbcopy = std::process::Command::new("pbcopy")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .ok()?;

    if let Some(mut stdin) = pbcopy.stdin.take() {
        let _ = stdin.write_all(&original);
    }
    let _ = pbcopy.wait();

    // Return copied text if it's not empty
    let copied_str = String::from_utf8(copied).ok()?;
    if !copied_str.is_empty() {
        Some(copied_str)
    } else {
        None
    }
}
