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
├── docs/superpowers/specs/       # Design specs
└── packages/
    ├── ui/                       # Shared UI components
    │   ├── Cargo.toml
    │   └── src/
    │       ├── lib.rs            # Component exports
    │       ├── icons.rs          # SVG icons (Play, Pause, Stop, FastBackward, FastForward)
    │       └── player_bar.rs     # Player toolbar component
    └── desktop/                  # Desktop application
        ├── Cargo.toml
        ├── assets/
        │   └── main.css          # Shadcn-inspired light theme
        └── src/
            ├── main.rs           # App entry point + state management
            ├── tts_engine.rs     # macOS `say` command wrapper
            └── clipboard_monitor.rs  # Clipboard polling via `pbpaste`
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
- Uses macOS `say` command for speech synthesis
- Configurable speech rate (0.5x to 2.0x)
- Supports pause/resume functionality

### 3. Playback Controls
- **Play/Pause** - Toggle speech playback
- **Stop** - Stop playback completely
- **Speed Down** (<<) - Decrease speech rate
- **Speed Up** (>>) - Increase speech rate
- **Speed Label** - Display current speed (e.g., "1.00x")

### 4. Global Keyboard Shortcut
- **Cmd+Shift+R** - Toggle play/pause from anywhere
- Works even when app is not focused
- Requires accessibility permissions on macOS

## UI Layout

```
[<<] [▶/⏸] [⏹] [>>] 1.00x
```

- Minimalistic 260x48px window
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
    on_play: EventHandler<()>,
    on_stop: EventHandler<()>,
) -> Element
```

Renders the playback controls with speed adjustment buttons.

### TtsEngine (`packages/desktop/src/tts_engine.rs`)

```rust
pub struct TtsEngine {
    process: Option<Child>,
}

impl TtsEngine {
    pub fn new() -> Self;
    pub fn speak(&mut self, text: &str, rate: f32);
    pub fn stop(&mut self);
    pub fn is_speaking(&mut self) -> bool;
}
```

Wraps the macOS `say` command for text-to-speech synthesis.

### ClipboardMonitor (`packages/desktop/src/clipboard_monitor.rs`)

```rust
pub fn use_clipboard_monitor() -> Signal<String>
```

Hook that polls the clipboard and returns a signal with the current clipboard text.

## State Management

All state lives in `main.rs`:

```rust
let mut is_playing = use_signal(|| false);  // Playback state
let speed = use_signal(|| 1.0);             // Speech rate
let mut tts = use_signal(|| TtsEngine::new());  // TTS engine
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
```

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

### Why macOS `say` command?
- No extra dependencies
- Works out of the box on macOS
- Simple to implement via `std::process::Command`
- Can be replaced with NSSpeechSynthesizer FFI later for more control

### Why clipboard polling?
- Simplicity - no need for accessibility APIs
- Works with any text selection
- Less invasive than accessibility permissions (initially)

### Why a tiny window?
- Minimalist design - only shows what's needed
- Doesn't obscure the text being read
- Fast to appear/disappear

## Future Improvements

### Short-term
- [ ] Add keyboard shortcuts for speed control
- [ ] Add system tray icon
- [ ] Add text file import
- [ ] Add voice selection

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

### No speech output
- Check if `say` command works in Terminal: `say "Hello World"`
- Verify audio output is working

### Clipboard not detected
- Ensure `pbpaste` works in Terminal: `echo "test" | pbpaste`
- Check if another app is锁定 the clipboard

## Development Tips

1. **Test TTS independently**: Run `say -r 200 "Hello World"` in Terminal
2. **Test clipboard**: Run `pbpaste` to see current clipboard content
3. **Check Dioxus logs**: Use `dx serve --verbose` for detailed logging
4. **Modify window size**: Update `LogicalSize::new(260.0, 48.0)` in `main.rs`

## Architecture Notes

- **No server** - This is a pure desktop app, no backend
- **No routing** - Single component app, no navigation needed
- **Minimal dependencies** - Only uses Dioxus desktop and tokio
- **Process-based TTS** - Uses system commands, not FFI
- **Signal-based state** - All state managed via Dioxus signals
