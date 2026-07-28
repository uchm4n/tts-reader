#![allow(dead_code)]

use super::TextSelector;

pub struct WindowsOption;

impl WindowsOption {
    pub fn new() -> Self {
        Self
    }
}

impl TextSelector for WindowsOption {
    fn get_selected_text(&self) -> Option<String> {
        // TODO: Windows UI Automation API
        None
    }

    fn name(&self) -> &str {
        "Windows UI Automation"
    }
}
