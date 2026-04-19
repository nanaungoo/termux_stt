# Professional Termux STT Pipeline (Rust 2024)

A high-performance, offline Speech-to-Text (STT) system designed for both **Android (Termux)** and **Ubuntu (Linux)**.

## ✨ Key Features
*   **Offline Transcription:** Powered by the Vosk engine for high-speed English recognition.
*   **Video Support:** Automatic audio extraction from video files (mp4, mkv, avi, etc.) using ffmpeg.
*   **SRT Generation:** Automatically creates subtitle files (SRT) alongside plain text transcripts.
*   **Interactive UI:** Modern, menu-driven CLI using `dialoguer`—no need to remember complex commands.
*   **Modular Architecture:** Cleanly organized codebase for easy maintenance and scaling.

## 🚀 Quick Start
```bash
git clone <repo-url>
cd termux_stt
bash setup_termux.sh
```
*Note: The setup script installs all dependencies, downloads the model, and configures the environment.*

## 📖 Essential Links
- **[How to Use](./HOW_TO_USE.md)**: Detailed commands and usage modes.
- **[Setup Guide](./SETUP_GUIDE.md)**: Manual installation and platform-specific notes.
- **[Learning Log](./LEARNING_LOG.md)**: Technical insights and architecture decisions.

## 📂 Modular Structure
- `src/engine/`: Core STT and Audio processing logic.
- `src/ui/`: Interactive menus and CLI definitions.
- `data/input/` & `data/output/`: Organized file management.

## 🛠️ Requirements
- **Vosk Model:** Required for transcription (handled by setup script).
- **Termux:API:** Required for microphone access on Android.
- **FFmpeg:** Required for video processing.

## License
MIT License. See [LICENSE](LICENSE) for details.
