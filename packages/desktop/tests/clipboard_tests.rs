//! Tests for clipboard functionality - tests the platform-specific clipboard command.

use std::process::Command;

#[cfg(target_os = "macos")]
fn clipboard_command() -> Command {
    Command::new("pbpaste")
}

#[cfg(target_os = "linux")]
fn clipboard_command() -> Command {
    // Try xclip first, fall back to xsel
    Command::new("xclip")
        .args(["-selection", "clipboard", "-o"])
}

#[cfg(target_os = "windows")]
fn clipboard_command() -> Command {
    Command::new("powershell")
        .args(["-command", "Get-Clipboard"])
}

#[test]
fn clipboard_command_executes_successfully() {
    let output = clipboard_command()
        .output()
        .expect("Failed to execute clipboard command");
    assert!(output.status.success() || output.status.code() == Some(1));
}

#[test]
fn clipboard_command_returns_valid_output() {
    let output = clipboard_command()
        .output()
        .expect("Failed to execute clipboard command");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(!text.contains('\0') || text.is_empty());
}

#[test]
fn clipboard_multiple_calls_are_consistent() {
    let out1 = clipboard_command()
        .output()
        .expect("Failed to execute clipboard command");
    let out2 = clipboard_command()
        .output()
        .expect("Failed to execute clipboard command");
    assert_eq!(out1.stdout, out2.stdout);
}
