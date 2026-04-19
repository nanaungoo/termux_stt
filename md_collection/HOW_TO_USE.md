# How to Use: Professional STT Pipeline

This guide covers the usage of the **Offline Transcription** system.

---

## 🎤 Usage Modes (အသုံးပြုပုံများ)

### 1. Interactive Mode (Menu-Driven)
The recommended way to use the application. Just run:
```bash
make run
```
**Features:**
- **Auto-Discovery:** Automatically lists audio and video files inside the `data/input/` folder.
- **Model Selection:** Allows choosing between different downloaded Vosk models.
- **Video Processing:** Automatically extracts audio from video files before transcription.

### 2. Standard Commands (Manual)
If you prefer direct commands:
```bash
# Transcribe a specific file
make run ARGS="transcribe data/input/test.mp3"

# Start real-time microphone transcription (Termux only)
make run ARGS="record"
```

---

## 📂 Folder Management (ဖိုင်နှင့် ဖိုဒါများ)

- **Input Folder (`data/input/`)**: Place your audio (`.mp3`, `.wav`) or video (`.mp4`, `.mkv`, etc.) files here.
- **Output Folder (`data/output/`)**: All results will be saved here:
    - `<file>.txt`: Plain text transcript.
    - `<file>.srt`: Subtitles with timestamps.

---

## 📱 Platform Specifics (ပလက်ဖောင်းများ)

### Termux (Android)
- Ensure **Termux:API** is installed and Microphone permission is granted.
- Real-time recording uses `termux-microphone-record` via the API.
- Use `termux-setup-storage` to access external files.

### Ubuntu (Linux)
- Use `make run` to ensure the correct Linux-native libraries are linked via `LD_LIBRARY_PATH`.
- Real-time recording is optimized for Android; file transcription is the primary mode for Linux.

---

## 🧪 Development
Run the unit tests to verify modular components:
```bash
make test
```
