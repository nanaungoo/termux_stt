# Setup Guide: Professional STT Pipeline

This guide provides step-by-step instructions for installing the system on Termux and Ubuntu.

## 🚀 Recommended Setup (Termux & Ubuntu)

The easiest way to set up is using the automated script:

```bash
cd termux_stt
bash setup_termux.sh
```

**This script automates:**
1.  **Dependencies:** Installs Rust, Make, OpenSSL, and libllvm.
2.  **Architecture:** Detects your CPU and configures pre-compiled libraries.
3.  **Voice Model:** Downloads the English Vosk model automatically.
4.  **Environment:** Creates your `.env` file and necessary folders.

---

## 📋 Manual Installation (Advanced)

### Step 1: Install System Packages
- **Termux:** `pkg install rust make openssl ca-certificates libllvm termux-api`
- **Ubuntu:** `sudo apt install build-essential rustc cargo make openssl libssl-dev`

### Step 2: Configure Shared Libraries
The application requires `libvosk.so`.
- Architecture-specific libraries are located in `libs/`.
- **Note:** Always use the `Makefile` (`make run` or `make build`) as it handles the `LD_LIBRARY_PATH` for you.

### Step 3: Setup AI Credentials
1.  Obtain a **Gemini API Key** from [Google AI Studio](https://aistudio.google.com/).
2.  Create a file named `.env` in the project root.
3.  Add: `GEMINI_API_KEY=your_actual_key_here`

---

## 📂 Project Organization

- `src/engine/`: Core logic for Vosk and Audio processing.
- `src/services/`: Translation and Text-to-Speech (TTS) services.
- `src/ui/`: Interactive Command Line Interface.
- `libs/`: Pre-compiled binaries for x86_64, ARM64 (aarch64), and ARMv7.

---

## ❓ Common Issues

### 1. `liblog.so` not found (Ubuntu)
This occurs because standard Ubuntu doesn't have Android logging libraries. Use the provided Linux-native `libvosk.so` in `libs/x86_64`. `make run` handles this automatically.

### 2. No sound/Permission denied (Termux)
- Ensure the **Termux:API** app is installed from F-Droid.
- Go to Android Settings ➔ Apps ➔ Termux:API ➔ Permissions ➔ Allow Microphone.

### 3. Translation Error (404 Not Found)
- Ensure your API key is valid.
- Check that your region supports Gemini API.
