pub mod clipboard_monitor;
pub mod text_selector;
pub mod tts_engine;

/// Tray icon menu item IDs.
pub mod tray {
    /// ID for the "Show" menu item.
    pub const TRAY_SHOW_ID: &str = "tray_show";
    /// ID for the "Quit" menu item.
    pub const TRAY_QUIT_ID: &str = "tray_quit";
}
