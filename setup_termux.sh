#!/bin/bash

# Termux Setup Script for Professional STT Pipeline (Rust 2024)
# (Senior Rust Architect Standards - Clean Edition)
# ဤ script သည် Termux ပေါ်တွင် STT ပရိုဂရမ်ကို အဆင်သင့်ဖြစ်စေရန် တည်ဆောက်ပေးမည် ဖြစ်ပါသည်။

set -e

echo "=== 🎤 Professional Termux STT Pipeline Setup ==="
echo "=== (Senior Rust Architect Standards - Clean Edition) ==="
echo "--- စတင်ပြင်ဆင်ခြင်း ---"
echo ""

# 1. Install dependencies
# Installing essential tools for audio processing and Rust development.
# Audio processing နှင့် Rust တည်ဆောက်မှုအတွက် လိုအပ်သော tools များကို တပ်ဆင်ခြင်း။
echo "[1/7] Installing dependencies (ffmpeg, rust, clang, etc.)..."
pkg uninstall openssl-tool -y || true
apt --fix-broken install -y || true
pkg update -y && pkg upgrade -y
pkg install termux-api rust wget unzip make libllvm openssl ca-certificates ffmpeg clang binutils pkg-config -y

# Verify ffmpeg installation
if command -v ffmpeg >/dev/null 2>&1; then
    echo "✅ ffmpeg detected: $(ffmpeg -version | head -n 1)"
else
    echo "❌ ERROR: ffmpeg installation failed."
    exit 1
fi
echo ""

# 2. Storage Setup
# Requesting access to shared storage to read/write audio files.
# ဖုန်း၏ storage ကို အသုံးပြုခွင့် တောင်းဆိုခြင်း။
echo "[2/7] Requesting storage access..."
termux-setup-storage || true
echo ""

# 3. Detect architecture
# Ensuring we use the correct native library for your CPU.
# သင်၏ CPU အမျိုးအစားအတွက် မှန်ကန်သော library ကို ရွေးချယ်ခြင်း။
ARCH=$(uname -m)
echo "[3/7] Detecting architecture: $ARCH"
case "$ARCH" in
    aarch64|arm64)
        LIB_ARCH="aarch64"
        ;;
    armv7l|armv8l|arm)
        LIB_ARCH="armv7"
        ;;
    x86_64)
        LIB_ARCH="x86_64"
        ;;
    *)
        LIB_ARCH="aarch64"
        ;;
esac
echo "Target architecture: $LIB_ARCH"
echo ""

# 4. Verify pre-compiled libraries
# Checking if libvosk.so exists for the detected architecture.
# Vosk library ရှိမရှိ စစ်ဆေးခြင်း။
echo "[4/7] Verifying libvosk.so..."
if [ -f "libs/$LIB_ARCH/libvosk.so" ]; then
    echo "✅ Found libvosk.so for $LIB_ARCH."
else
    echo "❌ ERROR: libs/$LIB_ARCH/libvosk.so not found!"
    exit 1
fi
echo ""

# 5. Setup Language Models Directory
# Handling Vosk language models. You can download or use local files.
# STT အတွက် Language Models များ ပြင်ဆင်ခြင်း။
echo "[5/7] Setting up Vosk Language Models..."
mkdir -p models

shopt -s nullglob
EXISTING_FOLDERS=(models/*/)
EXISTING_ZIPS=(models/*.zip)

if [ ${#EXISTING_FOLDERS[@]} -gt 0 ]; then
    echo "✅ Existing model folder(s) detected: ${EXISTING_FOLDERS[*]}"
elif [ ${#EXISTING_ZIPS[@]} -gt 0 ]; then
    echo "📦 Found zip files in models directory. Extracting..."
    for f in "${EXISTING_ZIPS[@]}"; do
        echo "Extracting $f..."
        unzip -o "$f" -d models/
    done
    echo "✅ Extraction complete."
else
    echo "No models found. Choose a download option:"
    echo "၁။ English (Small - Fast - 40MB)"
    echo "၂။ English (Large - Highly Accurate - 1.8GB)"
    echo "၃။ Custom URL (.zip)"
    echo "၄။ Local Path (e.g., /sdcard/Download/model.zip)"
    read -p "Select [1-4]: " choice
    
    case $choice in
        1)
            echo "Downloading Small Model..."
            wget -c https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip -O models/en-us-small.zip
            unzip models/en-us-small.zip -d models/ && rm models/en-us-small.zip
            ;;
        2)
            echo "Downloading Large Model (1.8GB)..."
            wget -c https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip -O models/en-us-large.zip
            unzip models/en-us-large.zip -d models/ && rm models/en-us-large.zip
            ;;
        3)
            read -p "Enter ZIP URL: " BUNDLE_LINK
            wget -c "$BUNDLE_LINK" -O models/custom-model.zip
            unzip -o models/custom-model.zip -d models/ && rm models/custom-model.zip
            ;;
        4)
            read -p "Enter local path: " USER_INPUT
            if [ -f "$USER_INPUT" ]; then
                unzip -o "$USER_INPUT" -d models/
            else
                echo "❌ File not found."
                exit 1
            fi
            ;;
        *)
            echo "Invalid option."
            exit 1
            ;;
    esac
    echo "✅ Model setup complete."
fi
echo ""

# 6. Setup Environment and Folders
# Creating input/output folders and default configuration.
# အဝင်နှင့် အထွက် ဖိုဒါများ ပြင်ဆင်ခြင်း။
echo "[6/7] Setting up environment and directories..."
mkdir -p data/input data/output
[ -f .env.example ] && [ ! -f .env ] && cp .env.example .env
echo "✅ Directories ready."
echo ""

# 7. Build application
# Compiling the Rust application.
# ပရိုဂရမ်ကို Rust ဖြင့် တည်ဆောက်ခြင်း။
echo "[7/7] Building application..."
make build

echo ""
echo "=== ✅ Setup Complete! (ပြင်ဆင်မှု ပြီးဆုံးပါပြီ) ==="
echo "💡 RUN: To start the application, use 'make run'"
echo "💡 ရှင်းလင်းချက်: Transcription ရလဒ်များကို data/output တွင် သိမ်းဆည်းသွားမည် ဖြစ်ပါသည်။"
echo ""
