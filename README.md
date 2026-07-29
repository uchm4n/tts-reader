# TTS Reader

A lightweight macOS player that speaks text aloud using Kokoro's 82M-param voice model.

![TTS Reader Screenshot](screenshot.png)

## Features

- **Kokoro TTS Engine** - Open-weight neural TTS model (82M params) for natural-sounding speech
- **28 English Voices** - American/British, male/female, selectable at runtime
- **Clipboard Monitoring** - Automatically detects when you copy text
- **Global Shortcut** - Press `Cmd+Shift+R` to play/pause from anywhere
- **Speed Control** - Adjustable speech rate (0.5x to 2.0x)
- **Voice Selection** - Choose from 28 voices in the dropdown below controls
- **Self-Contained** - Model and voices embedded in the binary, no external files needed
- **Minimal UI** - Tiny floating player that doesn't get in the way


**Note**: application only works for macOS, future support for Windows and Linux is planned.

## Download

Download the latest release:

[TTS Reader.dmg](https://limewire.com/d/9CiGK#Y496spzXJl)

Just open the `.dmg` file and drag the app to your Applications folder.

## How to Use

1. **Copy text** anywhere on your Mac (select text + `Cmd+C`)
2. **Click Play** or press `Cmd+Shift+R` to start listening
3. **Adjust speed** with the `<<` and `>>` buttons
4. **Select Voice** - Choose from 28 voices in the dropdown below the controls
5. **Click Stop** to stop playback

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
# Build release binary (includes embedded model, ~310MB)
cargo build --release

# Or create a .app bundle
dx bundle --platform macos --package-types app

# Or create a .dmg installer
dx bundle --platform macos --package-types dmg
```

## Tech Stack

- [Dioxus 0.7](https://dioxuslabs.com/) - Cross-platform UI framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Kokoro TTS](https://github.com/hexgrad/kokoro) - Open-weight neural TTS model
- [ort](https://github.com/pykeio/ort) - ONNX Runtime for model inference
- [rodio](https://github.com/RustAudio/rodio) - Audio playback
- macOS `say` command - Fallback TTS
- `pbpaste` - Clipboard monitoring

## Voice Configuration

The default voice can be configured via a `.env` file in the project root:

```env
KOKORO_VOICE=af_heart                    # Default voice (American Female)
# KOKORO_MODEL_DIR=/path/to/models       # Optional: override model path
# KOKORO_VOICE_DIR=/path/to/voices       # Optional: override voice dir
```

Available voices:
- **American Female**: Heart, Alloy, Aoede, Bella, Jessica, Kore, Nicole, Nova, River, Sarah, Sky
- **American Male**: Adam, Echo, Eric, Fenrir, Liam, Michael, Onyx, Puck, Santa
- **British Female**: Alice, Emma, Isabella, Lily
- **British Male**: Daniel, Fable, George, Lewis

## Voices/Samples


> Life is like a box of chocolates. You never know what you're gonna get.


| Name         | Nationality | Gender | Sample                                                                                                                                  |
| ------------ | ----------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| **af_heart** | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/S_9tkA75BT_QHKOzSX6S-.wav"></audio> |
| af_alloy     | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/wiZ3gvlL--p5pRItO4YRE.wav"></audio> |
| af_aoede     | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/Nv1xMwzjTdF9MR8v0oEEJ.wav"></audio> |
| af_bella     | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/sWN0rnKU6TlLsVdGqRktF.wav"></audio> |
| af_jessica   | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/2Oa4wITWAmiCXJ_Q97-7R.wav"></audio> |
| af_kore      | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/AOIgyspzZWDGpn7oQgwtu.wav"></audio> |
| af_nicole    | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/EY_V2OGr-hzmtTGrTCTyf.wav"></audio> |
| af_nova      | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/X-xdEkx3GPlQG5DK8Gsqd.wav"></audio> |
| af_river     | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/ZqaV2-xGUZdBQmZAF1Xqy.wav"></audio> |
| af_sarah     | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/xzoJBl1HCvkE8Fl8Xu2R4.wav"></audio> |
| af_sky       | American    | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/ubebYQoaseyQk-jDLeWX7.wav"></audio> |
| am_adam      | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/tvauhDVRGvGK98I-4wv3H.wav"></audio> |
| am_echo      | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/qy_KuUB0hXsu-u8XaJJ_Z.wav"></audio> |
| am_eric      | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/JhqPjbpMhraUv5nTSPpwD.wav"></audio> |
| am_fenrir    | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/c0R9caBdBiNjGUUalI_DQ.wav"></audio> |
| am_liam      | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/DFHvulaLeOjXIDKecvNG3.wav"></audio> |
| am_michael   | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/IPKhsnjq1tPh3JmHH8nEg.wav"></audio> |
| am_onyx      | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/ov0pFDfE8NNKZ80LqW6Di.wav"></audio> |
| am_puck      | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/MOC654sLMHWI64g8HWesV.wav"></audio> |
| am_santa     | American    | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/LzA6JmHBvQlhOviy8qVfJ.wav"></audio> |
| bf_alice    | British     | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/9mnYZ3JWq7f6U12plXilA.wav"></audio> |
| bf_emma     | British     | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/_fvGtKMttRI0cZVGqxMh8.wav"></audio> |
| bf_isabella | British     | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/VzlcJpqGEND_Q3duYnhiu.wav"></audio> |
| bf_lily     | British     | Female | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/qZCoartohiRlVamY8Xpok.wav"></audio> |
| bm_daniel   | British     | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/Eb0TLnLXHDRYOA3TJQKq3.wav"></audio> |
| bm_fable    | British     | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/NT9XkmvlezQ0FJ6Th5hoZ.wav"></audio> |
| bm_george   | British     | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/y6VJbCESszLZGupPoqNkF.wav"></audio> |
| bm_lewis    | British     | Male   | <audio controls src="https://cdn-uploads.huggingface.co/production/uploads/61b253b7ac5ecaae3d1efe0c/RlB5BRvLt-IFvTjzQNxCh.wav"></audio> |


## License

MIT
