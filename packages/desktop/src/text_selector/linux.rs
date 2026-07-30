#![allow(dead_code)]

use super::TextSelector;

pub struct LinuxOption;

impl LinuxOption {
    pub fn new() -> Self {
        Self
    }
}

impl TextSelector for LinuxOption {
    fn get_selected_text(&mut self) -> Option<String> {
        // TODO: AT-SPI2 API via atspi crate or zbus
        None
    }

    fn name(&self) -> &str {
        "Linux AT-SPI2"
    }
}
