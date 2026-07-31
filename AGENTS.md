You are an expert [0.7 Dioxus](https://dioxuslabs.com/learn/0.7) assistant. Dioxus 0.7 changes every api in dioxus. Only use this up to date documentation. `cx`, `Scope`, and `use_state` are gone

Provide concise code examples with detailed descriptions

# Dioxus Dependency

You can add Dioxus to your `Cargo.toml` like this:

```toml
[dependencies]
dioxus = { version = "0.7.1" }

[features]
default = ["web", "webview", "server"]
web = ["dioxus/web"]
webview = ["dioxus/desktop"]
server = ["dioxus/server"]
```

# Launching your application

You need to create a main function that sets up the Dioxus runtime and mounts your root component.

```rust
use dioxus::prelude::*;

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! { "Hello, Dioxus!" }
}
```

Then serve with `dx serve`:

```sh
curl -sSL http://dioxus.dev/install.sh | sh
dx serve
```

# UI with RSX

```rust
rsx! {
	div {
		class: "container", // Attribute
		color: "red", // Inline styles
		width: if condition { "100%" }, // Conditional attributes
		"Hello, Dioxus!"
	}
	// Prefer loops over iterators
	for i in 0..5 {
		div { "{i}" } // use elements or components directly in loops
	}
	if condition {
		div { "Condition is true!" } // use elements or components directly in conditionals
	}

	{children} // Expressions are wrapped in brace
	{(0..5).map(|i| rsx! { span { "Item {i}" } })} // Iterators must be wrapped in braces
}
```

# Assets

The asset macro can be used to link to local files to use in your project. All links start with `/` and are relative to the root of your project.

```rust
rsx! {
	img {
		src: asset!("/assets/image.png"),
		alt: "An image",
	}
}
```

## Styles

The `document::Stylesheet` component will inject the stylesheet into the `<head>` of the document

```rust
rsx! {
	document::Stylesheet {
		href: asset!("/assets/styles.css"),
	}
}
```

# Components

Components are the building blocks of apps

* Component are functions annotated with the `#[component]` macro.
* The function name must start with a capital letter or contain an underscore.
* A component re-renders only under two conditions:
	1.  Its props change (as determined by `PartialEq`).
	2.  An internal reactive state it depends on is updated.

```rust
#[component]
fn Input(mut value: Signal<String>) -> Element {
	rsx! {
		input {
            value,
			oninput: move |e| {
				*value.write() = e.value();
			},
			onkeydown: move |e| {
				if e.key() == Key::Enter {
					value.write().clear();
				}
			},
		}
	}
}
```

Each component accepts function arguments (props)

* Props must be owned values, not references. Use `String` and `Vec<T>` instead of `&str` or `&[T]`.
* Props must implement `PartialEq` and `Clone`.
* To make props reactive and copy, you can wrap the type in `ReadOnlySignal`. Any reactive state like memos and resources that read `ReadOnlySignal` props will automatically re-run when the prop changes.

# State

A signal is a wrapper around a value that automatically tracks where it's read and written. Changing a signal's value causes code that relies on the signal to rerun.

## Local State

The `use_signal` hook creates state that is local to a single component. You can call the signal like a function (e.g. `my_signal()`) to clone the value, or use `.read()` to get a reference. `.write()` gets a mutable reference to the value.

Use `use_memo` to create a memoized value that recalculates when its dependencies change. Memos are useful for expensive calculations that you don't want to repeat unnecessarily.

```rust
#[component]
fn Counter() -> Element {
	let mut count = use_signal(|| 0);
	let mut doubled = use_memo(move || count() * 2); // doubled will re-run when count changes because it reads the signal

	rsx! {
		h1 { "Count: {count}" } // Counter will re-render when count changes because it reads the signal
		h2 { "Doubled: {doubled}" }
		button {
			onclick: move |_| *count.write() += 1, // Writing to the signal rerenders Counter
			"Increment"
		}
		button {
			onclick: move |_| count.with_mut(|count| *count += 1), // use with_mut to mutate the signal
			"Increment with with_mut"
		}
	}
}
```

## Context API

The Context API allows you to share state down the component tree. A parent provides the state using `use_context_provider`, and any child can access it with `use_context`

```rust
#[component]
fn App() -> Element {
	let mut theme = use_signal(|| "light".to_string());
	use_context_provider(|| theme); // Provide a type to children
	rsx! { Child {} }
}

#[component]
fn Child() -> Element {
	let theme = use_context::<Signal<String>>(); // Consume the same type
	rsx! {
		div {
			"Current theme: {theme}"
		}
	}
}
```

# Async

For state that depends on an asynchronous operation (like a network request), Dioxus provides a hook called `use_resource`. This hook manages the lifecycle of the async task and provides the result to your component.

* The `use_resource` hook takes an `async` closure. It re-runs this closure whenever any signals it depends on (reads) are updated
* The `Resource` object returned can be in several states when read:
1. `None` if the resource is still loading
2. `Some(value)` if the resource has successfully loaded

```rust
let mut dog = use_resource(move || async move {
	// api request
});

match dog() {
	Some(dog_info) => rsx! { Dog { dog_info } },
	None => rsx! { "Loading..." },
}
```

# Routing

All possible routes are defined in a single Rust `enum` that derives `Routable`. Each variant represents a route and is annotated with `#[route("/path")]`. Dynamic Segments can capture parts of the URL path as parameters by using `:name` in the route string. These become fields in the enum variant.

The `Router<Route> {}` component is the entry point that manages rendering the correct component for the current URL.

You can use the `#[layout(NavBar)]` to create a layout shared between pages and place an `Outlet<Route> {}` inside your layout component. The child routes will be rendered in the outlet.

```rust
#[derive(Routable, Clone, PartialEq)]
enum Route {
	#[layout(NavBar)] // This will use NavBar as the layout for all routes
		#[route("/")]
		Home {},
		#[route("/blog/:id")] // Dynamic segment
		BlogPost { id: i32 },
}

#[component]
fn NavBar() -> Element {
	rsx! {
		a { href: "/", "Home" }
		Outlet<Route> {} // Renders Home or BlogPost
	}
}

#[component]
fn App() -> Element {
	rsx! { Router::<Route> {} }
}
```

```toml
dioxus = { version = "0.7.1", features = ["router"] }
```

# Fullstack

Fullstack enables server rendering and ipc calls. It uses Cargo features (`server` and a client feature like `web`) to split the code into a server and client binaries.

```toml
dioxus = { version = "0.7.1", features = ["fullstack"] }
```

## Server Functions

Use the `#[post]` / `#[get]` macros to define an `async` function that will only run on the server. On the server, this macro generates an API endpoint. On the client, it generates a function that makes an HTTP request to that endpoint.

```rust
#[post("/api/double/:path/&query")]
async fn double_server(number: i32, path: String, query: i32) -> Result<i32, ServerFnError> {
	tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	Ok(number * 2)
}
```

## Hydration

Hydration is the process of making a server-rendered HTML page interactive on the client. The server sends the initial HTML, and then the client-side runs, attaches event listeners, and takes control of future rendering.

### Errors
The initial UI rendered by the component on the client must be identical to the UI rendered on the server.

* Use the `use_server_future` hook instead of `use_resource`. It runs the future on the server, serializes the result, and sends it to the client, ensuring the client has the data immediately for its first render.
* Any code that relies on browser-specific APIs (like accessing `localStorage`) must be run *after* hydration. Place this code inside a `use_effect` hook.

---

# TTS Reader Application

## Overview

A minimalistic macOS desktop application that reads text aloud from the system clipboard. Built with Dioxus 0.7, it provides a tiny floating player with playback controls and a global keyboard shortcut for hands-free operation.

## Project Structure

```
dioxus/
├── Cargo.toml                    # Workspace root
├── AGENTS.md                     # This file
├── .env                          # Voice config (KOKORO_VOICE)
├── docs/superpowers/specs/       # Design specs
└── packages/
    ├── kokoro/                   # Local patched kokoro-en crate
    ├── ui/                       # Shared UI components
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs            # Component exports
    │       ├── icons.rs          # SVG icons (Play, Pause, Stop, FastBackward, FastForward)
    │       ├── voices.rs         # Voice list constant + label helper
    │       └── player_bar.rs     # Player toolbar + voice selector
    └── desktop/                  # Desktop application
        ├── Cargo.toml
        ├── assets/
        │   └── main.css          # Shadcn-inspired light theme
        └── src/
            ├── main.rs           # App entry point + state management
            ├── tts_engine/       # Directory module
            │   ├── mod.rs        # Re-exports TtsEngine
            │   ├── backend.rs    # TtsEngine public API + Backend enum
            │   ├── kokoro.rs     # Kokoro ONNX backend (primary)
            │   └── say.rs        # macOS `say` command (fallback)
            ├── clipboard_monitor.rs  # Clipboard polling via `pbpaste`
            └── text_selector/        # Directory module
                ├── mod.rs            # TextSelector trait + get_text_for_playback()
                ├── macos.rs          # Clipboard simulation (enigo + arboard)
                ├── linux.rs          # Stub (TODO: AT-SPI2)
                └── windows.rs        # Stub (TODO: UI Automation)
```

## Workspace Configuration

The workspace contains two packages:
- **ui** - Shared UI components (icons, player bar)
- **desktop** - Desktop application entry point

```toml
# Root Cargo.toml
[workspace]
members = ["packages/ui", "packages/desktop"]

[workspace.dependencies]
dioxus = { version = "0.7.1" }
dioxus-desktop = { version = "0.7.1" }
ui = { path = "packages/ui" }
```

## Features

### 1. Clipboard Monitoring
- Polls system clipboard every 500ms using `pbpaste` command
- Detects when new text is copied
- Updates the UI signal for playback

### 2. Text-to-Speech
- **Primary**: Kokoro ONNX neural TTS model (82M params) for natural-sounding speech
- **Fallback**: macOS `say` command when Kokoro unavailable
- Configurable speech rate (0.5x to 2.0x)
- Supports pause/resume functionality

### 3. Voice Selection
- 28 English voices (American/British, male/female)
- Runtime voice switching via dropdown selector
- Labels show "Name (Nationality + Gender)" format
- Default voice configurable via `.env` file

### 4. Playback Controls
- **Play/Pause** - Toggle speech playback
- **Stop** - Stop playback completely
- **Speed Down** (<<) - Decrease speech rate
- **Speed Up** (>>) - Increase speech rate
- **Speed Label** - Display current speed (e.g., "1.00x")

### 5. Global Keyboard Shortcut
- **Cmd+Shift+R** - Toggle play/pause from anywhere
- Works even when app is not focused
- Requires accessibility permissions on macOS

### 6. Self-Contained Binary
- ONNX model (310MB) embedded via `include_bytes!`
- 52 voice files embedded for all supported languages
- No external files needed at runtime
- Falls back to temp directory extraction

### 7. Selected Text Reading
- Simulates Cmd+C via `enigo` (CoreGraphics event posting)
- Reads selected text from system pasteboard via `arboard`
- Restores original clipboard content after reading
- Falls back to clipboard monitor text if simulation fails
- Works with any app that responds to Cmd+C (browsers, PDF readers, IDEs)

### 8. System Tray Icon
- Tray icon always visible in menu bar (macOS) / system tray (Windows/Linux)
- Window hides to tray on close (app keeps running in background)
- Left-click tray icon shows and focuses the window
- Right-click context menu with "Show" and "Quit" options
- "Quit" terminates the app completely
- Uses existing `logo.png` as tray icon (embedded via `icon_from_memory` + `include_bytes!`)
- No new dependencies (tray-icon, muda are transitive deps of dioxus-desktop)

**Important**: Uses `use_muda_event_handler` (not `use_tray_menu_event_handler`) due to
a dioxus-desktop 0.7.x bug — see "Known Bugs / Workarounds" below.

## UI Layout

```
[<<] [▶/⏸] [⏹] [>>] 1.00x [📌]
[▼ Heart (American Female)          ]
```

- Minimalistic 290x80px window
- Light theme with shadcn-inspired styling
- Non-resizable, always on top (optional)
- No borders or decorations

## Key Components

### PlayerBar (`packages/ui/src/player_bar.rs`)

```rust
#[component]
pub fn PlayerBar(
    is_playing: Signal<bool>,
    speed: Signal<f32>,
    voice: Signal<String>,
    is_always_on_top: Signal<bool>,
    on_play: EventHandler<()>,
    on_stop: EventHandler<()>,
    on_voice_change: EventHandler<String>,
    on_always_on_top: EventHandler<AlwaysOnTopEvent>,
    on_play_pause_hover: EventHandler<bool>,
) -> Element
```

Renders the playback controls with speed adjustment buttons and voice selector dropdown.

### TtsEngine (`packages/desktop/src/tts_engine.rs`)

```rust
pub struct TtsEngine {
    backend: Backend,
}

impl TtsEngine {
    pub fn new() -> Self;           // Tries Kokoro, falls back to say
    pub fn speak(&mut self, text: &str, rate: f32);
    pub fn stop(&mut self);
    pub fn is_speaking(&mut self) -> bool;
    pub fn set_voice(&mut self, voice: &str);
}
```

Enum strategy: `Backend::Kokoro` | `Backend::Say` with silent fallback. Voice changes take effect on next `speak()` call.

### ClipboardMonitor (`packages/desktop/src/clipboard_monitor.rs`)

```rust
pub fn use_clipboard_monitor() -> Signal<String>
```

Hook that polls the clipboard and returns a signal with the current clipboard text.

### TextSelector (`packages/desktop/src/text_selector/`)

```rust
pub trait TextSelector: Send {
    fn get_selected_text(&mut self) -> Option<String>;
    fn name(&self) -> &str;
}

pub fn create_text_selector() -> Box<dyn TextSelector>;
pub fn get_text_for_playback(selector: &mut dyn TextSelector, clipboard_text: &str) -> String;
```

Platform-specific implementations:
- **macOS**: Clipboard simulation via `enigo` + `arboard`
- **Linux/Windows**: Stubs returning `None` (TODO)

The `get_text_for_playback()` function tries selected text first, falls back to clipboard monitor text.

## State Management

All state lives in `main.rs`:

```rust
let mut is_playing = use_signal(|| false);  // Playback state
let speed = use_signal(|| 1.0);             // Speech rate
let mut voice = use_signal(|| std::env::var("KOKORO_VOICE").unwrap_or_else(|_| "af_heart".to_string()));
let mut tts = use_signal(|| None::<TtsEngine>);  // TTS engine
let clipboard_text = use_clipboard_monitor();   // Clipboard text
```

## Dependencies

```toml
# Desktop Cargo.toml
[dependencies]
dioxus = { version = "0.7.1", features = ["desktop"] }
dioxus-desktop = { version = "0.7.1" }
tokio = { version = "1", features = ["time"] }
ui = { path = "../ui" }
kokoro-en = { version = "0.1.4", default-features = false }
rodio = "0.20"
ort = { version = "2.0", features = ["coreml"] }
ort-sys = { version = "2.0", features = ["lax-feature-matching"] }
dotenvy = "5"

# macOS-specific dependencies
[target.'cfg(target_os = "macos")'.dependencies]
arboard = "3.6"      # Clipboard access via NSPasteboard
enigo = "0.6"        # Cmd+C simulation via CoreGraphics
```

**Transitive crates used directly** (via `dioxus_desktop::tao` and `dioxus_desktop::trayicon`):
- **tao** — Cross-platform window creation and event loop (re-exported by dioxus-desktop)
- **muda** — Menu and tray icon system (re-exported as `dioxus_desktop::trayicon::menu`)
- **tray-icon** — System tray icon (re-exported as `dioxus_desktop::trayicon`)

## Building & Running

```sh
# Development (with hot-reload)
cd packages/desktop
dx serve

# Production build (no server)
dx build --release

# Run the built app
./target/release/desktop
```

## Design Decisions

### Why Kokoro TTS?
- Open-weight neural model (82M params) for natural-sounding speech
- 28 English voices with different accents and genders
- ONNX runtime with CoreML acceleration on macOS
- Embedded in binary for self-contained distribution
- Falls back to `say` command on failure

### Why clipboard polling?
- Simplicity - no need for accessibility APIs
- Works with any text selection
- Less invasive than accessibility permissions (initially)

### Why a tiny window?
- Minimalist design - only shows what's needed
- Doesn't obscure the text being read
- Fast to appear/disappear

### Why clipboard simulation (not Accessibility API)?
- Accessibility API (`axuielement`) requires per-process trust via TCC
- Ad-hoc signed apps have unreliable `is_process_trusted()` results
- Different code-signing identities between dev and compiled builds
- Clipboard simulation works with any app that responds to Cmd+C
- Simpler implementation (55 lines vs 250+ lines)
- No complex accessibility tree traversal needed

## Kokoro TTS Backend

The Kokoro backend uses the `kokoro-en` crate with ONNX runtime for neural text-to-speech synthesis.

### Architecture
- Model loaded via `ort` with CoreML execution provider
- 52 voice files embedded via `include_bytes!`
- Background thread synthesis to avoid runtime nesting
- `Arc<Mutex<KokoroTts>>` for thread-safe access
- Rodio for audio output with lazy initialization

### Configuration
Single `.env` file at workspace root:
```env
KOKORO_VOICE=af_heart                    # Default voice
# KOKORO_MODEL_DIR=/path/to/models       # Optional: override model path
# KOKORO_VOICE_DIR=/path/to/voices       # Optional: override voice dir
```

### Voice List
28 English voices available:
- **American Female**: Heart, Alloy, Aoede, Bella, Jessica, Kore, Nicole, Nova, River, Sarah, Sky
- **American Male**: Adam, Echo, Eric, Fenrir, Liam, Michael, Onyx, Puck, Santa
- **British Female**: Alice, Emma, Isabella, Lily
- **British Male**: Daniel, Fable, George, Lewis

## Future Improvements

### Short-term
- [ ] Add keyboard shortcuts for speed control
- [x] Add system tray icon
- [ ] Add text file import
- [x] Add voice selection

### Medium-term
- [ ] Replace `say` command with NSSpeechSynthesizer FFI
- [ ] Add word highlighting in source app
- [ ] Add sentence/paragraph navigation
- [ ] Add dark mode toggle

### Long-term
- [ ] Cross-platform support (Linux, Windows)
- [ ] Clipboard history
- [ ] Text-to-audio file export
- [ ] Browser extension for web text reading

## Troubleshooting

### Global shortcut not working
- Grant accessibility permissions in System Settings → Privacy & Security → Accessibility
- Ensure the app is running (not just compiled)

### Selected text not reading
- Grant accessibility permissions: System Settings → Privacy & Security → Accessibility
- Ensure the app you're reading from supports Cmd+C
- Check if enigo initializes (look for "[TTS Reader] Failed to initialize input simulator" in logs)
- Some apps (terminal, password managers) don't respond to simulated Cmd+C

### No speech output
- Check if `say` command works in Terminal: `say "Hello World"`
- Verify audio output is working

### Clipboard not detected
- Ensure `pbpaste` works in Terminal: `echo "test" | pbpaste`
- Check if another app is holding the clipboard

## Development Tips

1. **Test TTS independently**: Run `say -r 200 "Hello World"` in Terminal
2. **Test clipboard**: Run `pbpaste` to see current clipboard content
3. **Check Dioxus logs**: Use `dx serve --verbose` for detailed logging
4. **Modify window size**: Update `LogicalSize::new(290.0, 80.0)` in `main.rs`
5. **Run tests**: `cargo test --package tts-reader` (mock-based, no display server needed)
6. **Check clippy**: `cargo clippy --package tts-reader`

## Architecture Notes

- **No server** - This is a pure desktop app, no backend
- **No routing** - Single component app, no navigation needed
- **Minimal dependencies** - Only uses Dioxus desktop and tokio
- **Process-based TTS** - Uses system commands, not FFI
- **Signal-based state** - All state managed via Dioxus signals
- **Clipboard simulation** - Uses enigo + arboard for text selection
- **No Accessibility API** - Avoids TCC trust issues with ad-hoc signing

## Known Bugs / Workarounds

### dioxus-desktop 0.7.x: `use_tray_menu_event_handler` never fires

**Affects**: dioxus-desktop 0.7.x (tested on 0.7.9, 0.7.10)
**Root cause**: `tray_icon::menu::MenuEvent` is a re-export of `muda::MenuEvent`.
Both share the same global `MENU_EVENT_HANDLER: OnceCell`. In `App::new()`:
1. `set_menubar_receiver()` calls `muda::MenuEvent::set_event_handler()` → succeeds (claims the OnceCell)
2. `set_tray_icon_receiver()` calls `muda::MenuEvent::set_event_handler()` again → silently fails (`OnceCell` already initialized, `let _ = ...`)

**Result**: ALL menu events (including tray menu) arrive as `UserWindowEvent::MudaMenuEvent`, not `TrayMenuEvent`. The `use_tray_menu_event_handler` hook only listens for `TrayMenuEvent` → never fires.

**Workaround**: Use `use_muda_event_handler` instead. Both receive `&MenuEvent` with the same `.id.0` field, so handler code is identical. This is what we use in `main.rs` for tray menu events.

**Tracking**: Will be fixed in dioxus-desktop when the internal `OnceCell` is replaced or a single handler dispatches both variants. If you upgrade dioxus-desktop and `use_tray_menu_event_handler` works, you can switch back.

## Testing

### Unit Tests
```bash
cargo test --package tts-reader
```

### Test Coverage
- **text_selector_tests**: 7 mock-based tests for fallback logic
- **clipboard_tests**: 3 tests for clipboard command execution
- **tray_icon_tests**: 10 tests for constants, menu items, and OnceCell bug documentation
- **tts_engine_tests**: (placeholder for future tests)

### Why mock tests?
- `get_selected_text()` requires display server + accessibility permissions
- CoreGraphics events (enigo) SIGTRAP in test harnesses
- Mock tests cover fallback logic without system dependencies
