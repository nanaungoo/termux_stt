# Termux Speech-to-Text Setup Guide

A complete step-by-step guide to set up and run the Termux Speech-to-Text system on your Android device.

## 🚀 Quick Start (One-Command Setup)

If you're on your Termux device and have cloned this repository:

```bash
cd ~/termux_stt
chmod +x setup_termux.sh
./setup_termux.sh
```

This script will automatically:
- Install all required dependencies
- Detect your device architecture
- Copy the correct `libvosk.so` library
- Download the Vosk speech recognition model
- Build the application

After the script completes, run:
```bash
export LD_LIBRARY_PATH=$(pwd)
./target/release/termux_stt
```

---

## 📋 Manual Setup (Step-by-Step)

If you prefer to do things manually or need to troubleshoot, follow these steps:

### Step 1: Install Termux and Termux:API

Install both apps from **F-Droid** (recommended) or Google Play Store:
- **Termux**: [F-Droid](https://f-droid.org/packages/com.termux/) | [Google Play](https://play.google.com/store/apps/details?id=com.termux)
- **Termux:API**: [F-Droid](https://f-droid.org/packages/com.termux.api/) | [Google Play](https://play.google.com/store/apps/details?id=com.termux.api)

> ⚠️ **Important**: Both apps must be installed from the **same source** (both from F-Droid or both from Play Store). Mixing sources will cause compatibility issues.

Grant **microphone permissions** to Termux and Termux:API in your Android settings.

### Step 2: Install Dependencies

Open Termux and run:
```bash
pkg update && pkg upgrade -y
pkg install termux-api rust wget unzip git -y
```

Verify installation:
```bash
rustc --version
cargo --version
termux-microphone-record --help
```

### Step 3: Clone the Repository

```bash
cd ~
git clone <your-repo-url>
cd termux_stt
```

### Step 4: Download the Vosk Model

The application uses the `vosk-model-small-en-us-0.15` model:
```bash
wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
unzip vosk-model-small-en-us-0.15.zip
rm vosk-model-small-en-us-0.15.zip
```

> 💡 **Note**: You can also skip this step—the app will prompt you to download it automatically on first run.

### Step 5: Set Up the Native Library

The project includes pre-compiled `libvosk.so` for different architectures. Detect your architecture:
```bash
uname -m
```

Copy the appropriate library:
| Architecture | Command |
|-------------|---------|
| **aarch64** (most modern devices) | `cp libs/aarch64/libvosk.so .` |
| **armv7l** / **arm** | `cp libs/armv7/libvosk.so .` |
| **x86_64** | `cp libs/x86_64/libvosk.so .` |
| **x86** | `cp libs/x86/libvosk.so .` |

### Step 6: Build the Application

```bash
export LD_LIBRARY_PATH=$(pwd)
cargo build --release
```

### Step 7: Run the Application

```bash
export LD_LIBRARY_PATH=$(pwd)
./target/release/termux_stt
```

---

## 📖 Usage Modes

### Mode 1: Real-Time Microphone Transcription

Run without arguments:
```bash
./target/release/termux_stt
```

- Press **Enter** to start recording
- Speak clearly into the microphone
- Partial results will appear as you speak
- Press **Ctrl+C** to stop

### Mode 2: Audio File Transcription

Pass an audio file path as an argument:
```bash
./target/release/termux_stt path/to/audio.mp3
```

Supported formats: `.mp3`, `.wav`

Output files (auto-generated in the same directory as the audio file):
- `audio.txt` — Plain text transcript
- `audio.srt` — Subtitle file with word-level timestamps

---

## 🔧 Persistent Environment Setup

To avoid setting `LD_LIBRARY_PATH` every time, add it to your shell profile:

```bash
echo 'export LD_LIBRARY_PATH=~/termux_stt:$LD_LIBRARY_PATH' >> ~/.bashrc
source ~/.bashrc
```

---

## ❓ Troubleshooting

### `termux-api not found`
- Ensure you ran `pkg install termux-api`
- Ensure the **Termux:API app** is installed from F-Droid/Play Store
- Restart Termux completely

### `libvosk.so: cannot open shared object file`
- Verify `libvosk.so` is in the project root: `ls -l libvosk.so`
- Verify `LD_LIBRARY_PATH` is set: `echo $LD_LIBRARY_PATH`
- Ensure you copied the correct library for your architecture

### `Vosk model not found`
- Ensure the `vosk-model-small-en-us-0.15` directory exists in the project root
- Re-download the model if it's corrupted: `rm -rf vosk-model-small-en-us-0.15` and re-run the app

### Microphone not working
- Grant microphone permissions in **Settings > Apps > Termux > Permissions**
- Test with: `termux-microphone-record -d 5 test.wav`
- Ensure Termux and Termux:API are from the same source (both F-Droid or both Play Store)

---

## 📂 Project Structure

```
termux_stt/
├── src/
│   └── main.rs          # Main application code
├── libs/
│   ├── aarch64/         # ARM64 library
│   ├── armv7/           # ARM32 library
│   ├── x86_64/          # x86_64 library
│   └── x86/             # x86 library
├── vosk-model-small-en-us-0.15/  # Downloaded model (not in repo)
├── libvosk.so           # Symlinked/copied native library
├── setup_termux.sh      # Automated setup script
├── build.rs             # Cargo build script for linking
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

---

## 🔗 Additional Resources

- [Vosk Official](https://alphacephei.com/vosk/)
- [Symphonia Audio Library](https://github.com/pdeljanov/Symphonia)
- [Termux Wiki](https://wiki.termux.com/wiki/Main_Page)
