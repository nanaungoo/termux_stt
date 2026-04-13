# How to Use: Termux Speech-to-Text System (Rust 2024)

This project provides a high-performance, offline speech-to-text system for Android (via Termux) and Linux (Ubuntu) using Rust 2024 and the Vosk toolkit.

## 🚀 Quick Start (Automated with Makefile)

1.  **Install Prerequisites**:
    - **Termux**: `pkg update && pkg install termux-api rust wget unzip make -y`
    - **Ubuntu**: `sudo apt update && sudo apt install build-essential rustc cargo wget unzip make -y`
    *Note: For Termux, ensure the [Termux:API app](https://f-droid.org/packages/com.termux.api/) is installed.*

2.  **Clone and Build**:
    ```bash
    git clone <your-repo-url>
    cd termux_stt
    make build
    ```

3.  **Run the Application**:
    ```bash
    make run
    ```
    *The program will automatically prompt to download the English Vosk model on its first run if it's missing.*

---

## 🛠️ Usage Modes

### 1. Real-time Transcription (Microphone)
Run the program with the `record` command (default mode):
```bash
./target/debug/termux_stt record
```
- **Platform Support**: Real-time recording is currently optimized for **Termux** using `termux-microphone-record`.
- **Feedback**: You will see **Partial Results** as you speak.
- **Stopping**: Press **Ctrl+C** to stop.

### 2. File Transcription (MP3/WAV)
To transcribe an existing audio file:
```bash
./target/debug/termux_stt transcribe path/to/audio.mp3
```
- **UX**: A visual **Progress Bar** will show the processing status.
- **Output**: Generates `<file>.txt` (plain text) and `<file>.srt` (subtitles with timestamps).
- **Options**: Use `-o <name>` to specify a custom output filename.

---

## 🧪 Testing & Development

We use a `Makefile` to handle the complexity of linking shared libraries (`libvosk.so`) across different architectures.

**Run All Tests**:
```bash
make test
```
*This command automatically sets the correct `LD_LIBRARY_PATH` for your system architecture (x86_64, aarch64, or armv7).*

**Clean Build Artifacts**:
```bash
make clean
```

---

## 📂 Project Architecture

- **`src/lib.rs`**: Core library entry point (No `unwrap()` allowed here).
- **`src/core.rs`**: STT engine logic and file processing with progress bars.
- **`src/cli.rs`**: Command-line argument definitions (Clap v4).
- **`src/error.rs`**: Rich diagnostic error types using `miette` and `thiserror`.
- **`libs/`**: Pre-compiled Vosk libraries for cross-platform support.

---

## 📱 Cross-Compilation (Building for Android)

If you are building on a Linux PC for a Termux target:
1.  Add the target: `rustup target add aarch64-linux-android`
2.  Build: `cargo build --release --target aarch64-linux-android`
3.  The binary will be in `target/aarch64-linux-android/release/termux_stt`.

---

## ❓ Troubleshooting

- **Library Not Found**: If running manually without `make`, ensure you set your library path:
  `export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)/libs/x86_64` (adjust for your arch).
- **Permission Denied (Termux)**: Ensure you have granted microphone permissions to the Termux:API app.
- **Invalid ELF Header**: Ensure your system's dynamic linker is compatible with the pre-compiled libraries in `libs/`.
