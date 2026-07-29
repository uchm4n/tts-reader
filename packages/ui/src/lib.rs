//! This crate contains all shared UI for the TTS Reader.

pub mod icons;
pub mod player_bar;
pub mod voices;

pub use player_bar::PlayerBar;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AlwaysOnTopEvent {
    HoverEnter,
    HoverLeave,
    Toggle,
}
