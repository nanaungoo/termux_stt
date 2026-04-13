# How to Use: Termux Speech-to-Text System

This project provides a high-performance, offline speech-to-text system for Android (via Termux) using Rust and the Vosk toolkit.

## 🚀 Quick Start

1.  **Install Prerequisites in Termux**:
    ```bash
    pkg update && pkg upgrade
    pkg install termux-api rust wget unzip -y
    ```
    *Ensure you have the [Termux:API app](https://f-droid.org/packages/com.termux.api/) installed from F-Droid.*

2.  **Clone and Build**:
    ```bash
    git clone <your-repo-url>
    cd termux_stt
    cargo build --release
    ```

3.  **Run the Application**:
    ```bash
    ./target/release/termux_stt
    ```
    *The program will automatically ask to download the English Vosk model on its first run if it's missing.*

---

## 🛠️ Usage Modes

### 1. Real-time Transcription (Microphone)
Run the program with the `record` command (or no arguments):
```bash
./target/release/termux_stt record
```
*   **Feedback**: You will see **Partial Results** as you speak.
*   **Stopping**: Press **Ctrl+C** or stop the recording from your Android notification tray.

### 2. File Transcription (MP3/WAV)
To transcribe an existing audio file:
```bash
./target/release/termux_stt transcribe path/to/your/audio.mp3
```
*   **Options**: Use `-o` or `--output` to specify a custom output base name.
*   **Supported Formats**: `.mp3`, `.wav`, and other formats supported by `symphonia`.
*   **Output**: The program generates:
    *   `<output>.txt`: Plain-text transcript.
    *   `<output>.srt`: Subtitle file with timestamps.

---

## 📂 Project Structure & Libraries

*   **`libs/`**: Contains pre-compiled `libvosk.so` for different architectures (`aarch64`, `armv7`, `x86_64`).
*   **`build.rs`**: Automatically detects your device architecture and links the correct library during compilation.
*   **Vosk Model**: The application expects the model in `./vosk-model-small-en-us-0.15/`.

---

## 📱 Cross-Compilation (Building on PC for Android)

If you prefer building on a powerful PC for your phone:

1.  Add the Android target:
    ```bash
    rustup target add aarch64-linux-android
    ```
2.  Build using the target (requires Android NDK):
    ```bash
    cargo build --release --target aarch64-linux-android
    ```
3.  Copy the resulting binary from `target/aarch64-linux-android/release/termux_stt` to your phone.

---

## ❓ Troubleshooting

*   **`termux-api not found`**: Ensure you ran `pkg install termux-api` AND have the Termux:API helper app installed from F-Droid/Play Store.
*   **Permission Denied**: Grant Microphone permissions to the Termux:API app in your Android system settings.
*   **Library Not Found**: The `build.rs` handles linking, but if you move the binary, ensure `libvosk.so` for your architecture is in the same folder or in your `LD_LIBRARY_PATH`.
