//! Tests for clipboard functionality - tests the pbpaste command directly.

use std::process::Command;

#[test]
fn pbpaste_executes_successfully() {
    let output = Command::new("pbpaste")
        .output()
        .expect("Failed to execute pbpaste");
    assert!(output.status.success() || output.status.code() == Some(1));
}

#[test]
fn pbpaste_returns_valid_output() {
    let output = Command::new("pbpaste")
        .output()
        .expect("Failed to execute pbpaste");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(!text.contains('\0') || text.is_empty());
}

#[test]
fn pbpaste_multiple_calls_are_consistent() {
    let out1 = Command::new("pbpaste")
        .output()
        .expect("Failed to execute pbpaste");
    let out2 = Command::new("pbpaste")
        .output()
        .expect("Failed to execute pbpaste");
    assert_eq!(out1.stdout, out2.stdout);
}
