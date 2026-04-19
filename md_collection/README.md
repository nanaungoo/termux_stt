# Professional Termux STT Pipeline (Rust 2024)

A high-performance, offline Speech-to-Text (STT) system with integrated **Gemini AI Translation** and **Text-to-Speech (TTS)** capabilities. Designed for both **Android (Termux)** and **Ubuntu (Linux)**.

## ✨ Key Features
*   **Offline Transcription:** Powered by the Vosk engine for high-speed English recognition.
*   **AI Translation:** Integrated with Google Gemini API to translate transcribed text to Myanmar (or other languages).
*   **Voice-to-Voice:** Generates high-quality Myanmar voice output from translated text using Gemini Live or Google TTS.
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
- `src/services/`: External AI services (Gemini Translation & TTS).
- `src/ui/`: Interactive menus and CLI definitions.
- `input/` & `output/`: Organized file management.

## 🛠️ Requirements
- **Gemini API Key:** Required for Translation and TTS features (place in `.env`).
- **Termux:API:** Required for microphone access on Android.

## License
MIT License. See [LICENSE](LICENSE) for details.
