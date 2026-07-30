//! Cross-platform text selection reader.
//!
//! Reads selected text from the currently focused application.
//! Falls back gracefully on unsupported platforms or missing permissions.

pub mod linux;
pub mod macos;
pub mod windows;

/// Reads selected text from the currently focused application.
pub trait TextSelector: Send {
    /// Attempt to read the currently selected text.
    /// Returns None if unavailable (permissions, no selection, unsupported app).
    fn get_selected_text(&self) -> Option<String>;

    /// Human-readable name of the selector backend.
    #[allow(unused)]
    fn name(&self) -> &str;
}

/// Fallback selector for unsupported platforms.
#[allow(unused)]
struct NullSelector;

impl TextSelector for NullSelector {
    fn get_selected_text(&self) -> Option<String> {
        None
    }

    fn name(&self) -> &str {
        "none"
    }
}

/// Create a platform-appropriate text selector.
pub fn create_text_selector() -> Box<dyn TextSelector> {
    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacOSOption::new())
    }

    #[cfg(target_os = "windows")]
    {
        Box::new(windows::WindowsOption::new())
    }

    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxOption::new())
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        Box::new(NullSelector)
    }
}


