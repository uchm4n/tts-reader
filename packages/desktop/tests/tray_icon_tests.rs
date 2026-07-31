//! Tests for system tray icon functionality.
//!
//! The actual tray icon creation requires the Dioxus runtime and system APIs
//! (display server, accessibility permissions). These can't be unit-tested
//! in CI or test harnesses. Instead, we test the constants, menu item IDs,
//! and event handler matching logic.
//!
//! ## dioxus-desktop 0.7.x OnceCell Bug
//!
//! dioxus-desktop 0.7.x has a bug where `tray_icon::menu::MenuEvent` is a
//! re-export of `muda::MenuEvent`. Both share the same global
//! `MENU_EVENT_HANDLER: OnceCell`. In `App::new()`, `set_menubar_receiver()`
//! is called first and claims the OnceCell. When `set_tray_icon_receiver()`
//! later tries to call `muda::MenuEvent::set_event_handler()` again, the
//! OnceCell silently ignores it (`let _ = ...`).
//!
//! Result: ALL menu events (including tray menu) arrive as
//! `UserWindowEvent::MudaMenuEvent`, not `TrayMenuEvent`.
//!
//! Fix: Use `use_muda_event_handler` instead of `use_tray_menu_event_handler`.
//! Both receive `&MenuEvent` with the same `.id.0` field, so handler code
//! is identical.

use tts_reader::tray::{TRAY_SHOW_ID, TRAY_QUIT_ID};

// --- Constant tests ---

#[test]
fn tray_ids_are_non_empty() {
    assert!(!TRAY_SHOW_ID.is_empty());
    assert!(!TRAY_QUIT_ID.is_empty());
}

#[test]
fn tray_ids_are_unique() {
    assert_ne!(TRAY_SHOW_ID, TRAY_QUIT_ID);
}

#[test]
fn tray_ids_have_expected_values() {
    assert_eq!(TRAY_SHOW_ID, "tray_show");
    assert_eq!(TRAY_QUIT_ID, "tray_quit");
}

// --- Event ID matching logic tests ---

#[test]
fn show_event_matches_show_id() {
    let event_id = "tray_show";
    assert!(event_id == TRAY_SHOW_ID);
    assert!(event_id != TRAY_QUIT_ID);
}

#[test]
fn quit_event_matches_quit_id() {
    let event_id = "tray_quit";
    assert!(event_id == TRAY_QUIT_ID);
    assert!(event_id != TRAY_SHOW_ID);
}

#[test]
fn unknown_event_matches_neither() {
    let event_id = "unknown_id";
    assert!(event_id != TRAY_SHOW_ID);
    assert!(event_id != TRAY_QUIT_ID);
}

// --- Menu item creation tests ---

#[test]
fn menu_item_ids_match_constants() {
    // Verify that MenuItem::with_id creates items with the correct IDs
    let show_item = dioxus_desktop::trayicon::menu::MenuItem::with_id(
        TRAY_SHOW_ID, "Show", true, None,
    );
    let quit_item = dioxus_desktop::trayicon::menu::MenuItem::with_id(
        TRAY_QUIT_ID, "Quit", true, None,
    );

    assert_eq!(show_item.id().0, TRAY_SHOW_ID);
    assert_eq!(quit_item.id().0, TRAY_QUIT_ID);
}

#[test]
fn menu_item_ids_are_distinct() {
    let show_item = dioxus_desktop::trayicon::menu::MenuItem::with_id(
        TRAY_SHOW_ID, "Show", true, None,
    );
    let quit_item = dioxus_desktop::trayicon::menu::MenuItem::with_id(
        TRAY_QUIT_ID, "Quit", true, None,
    );

    assert_ne!(show_item.id(), quit_item.id());
}

// --- OnceCell bug documentation tests ---

/// Verify that `tray_icon::menu::MenuEvent` is re-exported from muda.
///
/// This documents the root cause of the dioxus-desktop 0.7.x OnceCell bug:
/// both types are identical, sharing the same global `MENU_EVENT_HANDLER`.
/// When `set_menubar_receiver()` claims it first, `set_tray_icon_receiver()`'s
/// call silently fails, so tray menu events arrive as `MudaMenuEvent`.
///
/// We verify this by checking the type name contains "muda" in its
/// module path. If this test fails, the dioxus-desktop bug may have been
/// fixed upstream and we can switch back to `use_tray_menu_event_handler`.
#[test]
fn tray_menu_event_is_reexported_from_muda() {
    let type_name = std::any::type_name::<dioxus_desktop::trayicon::menu::MenuEvent>();
    assert!(
        type_name.contains("muda"),
        "Expected tray_icon::menu::MenuEvent to be a re-export from muda, got: {type_name}"
    );
}

/// Verify the MenuId field type allows String comparison.
///
/// Our handler uses `event.id.0 == TRAY_QUIT_ID` where `.0` is a `String`
/// and `TRAY_QUIT_ID` is `&str`. This confirms `String == &str` comparison
/// works (Rust's `PartialEq<str>` impl for `String`).
#[test]
fn menu_event_id_allows_string_comparison() {
    let show_item = dioxus_desktop::trayicon::menu::MenuItem::with_id(
        TRAY_SHOW_ID, "Show", true, None,
    );
    let quit_item = dioxus_desktop::trayicon::menu::MenuItem::with_id(
        TRAY_QUIT_ID, "Quit", true, None,
    );

    // Verify .0 is String and compares correctly with &str constants
    let show_id: &String = &show_item.id().0;
    let quit_id: &String = &quit_item.id().0;

    assert_eq!(show_id, TRAY_SHOW_ID);
    assert_eq!(quit_id, TRAY_QUIT_ID);
    assert_ne!(show_id, TRAY_QUIT_ID);
    assert_ne!(quit_id, TRAY_SHOW_ID);
}
