# TTS Reader

A minimalistic macOS desktop application that reads text aloud from the system clipboard.

![TTS Reader Screenshot](screenshot.png)

## Features

- **Clipboard Monitoring** - Automatically detects when you copy text
- **Text-to-Speech** - Reads clipboard text aloud using macOS `say` command
- **Global Shortcut** - Press `Cmd+Shift+R` to play/pause from anywhere
- **Speed Control** - Adjustable speech rate (0.5x to 2.0x)
- **Minimal UI** - Tiny floating player that doesn't get in the way

## Download

Download the latest release:

[TTS Reader.dmg](TtsReader.dmg)

Just open the `.dmg` file and drag the app to your Applications folder.

## How to Use

1. **Copy text** anywhere on your Mac (select text + `Cmd+C`)
2. **Click Play** or press `Cmd+Shift+R` to start listening
3. **Adjust speed** with the `<<` and `>>` buttons
4. **Click Stop** to stop playback

## Building from Source

### Prerequisites

- macOS 12.0 or later
- Rust (install via [rustup](https://rustup.rs/))
- Dioxus CLI

```bash
cargo install dioxus-cli
```

### Development

```bash
# Clone the repository
git clone https://github.com/yourusername/tts-reader.git
cd tts-reader

# Run in development mode
cargo run -p tts-reader
# Or
cd packages/desktop && cargo run
```

### Production Build

```bash
# Build release binary
cargo build --release

# Or create a .app bundle
dx bundle --platform macos --package-types app

# Or create a .dmg installer
dx bundle --platform macos --package-types dmg
```

## Tech Stack

- [Dioxus 0.7](https://dioxuslabs.com/) - Cross-platform UI framework
- [Tokio](https://tokio.rs/) - Async runtime
- macOS `say` command - Text-to-speech synthesis
- `pbpaste` - Clipboard monitoring

## License

MIT
