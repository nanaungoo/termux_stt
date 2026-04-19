#!/bin/bash

# Termux Setup Script for Professional STT Pipeline (Rust 2024)
# (Clean Transcription Edition - No Translation/TTS)
# Run this script on your Termux device after cloning the repo

set -e

echo "=== 🎤 Professional Termux STT Pipeline Setup ==="
echo "=== (Senior Rust Architect Standards - Clean Edition) ==="
echo ""

# 1. Install dependencies
echo "[1/7] Fixing potential package conflicts and installing dependencies..."
pkg uninstall openssl-tool -y || true
apt --fix-broken install -y || true
pkg update -y && pkg upgrade -y
pkg install termux-api rust wget unzip make libllvm openssl ca-certificates ffmpeg clang binutils pkg-config -y

echo "✅ Dependencies installed."
echo ""

# 2. Storage Setup
echo "[2/7] Requesting storage access..."
termux-setup-storage || true
echo ""

# 3. Detect architecture
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
echo "[4/7] Verifying libvosk.so..."
if [ -f "libs/$LIB_ARCH/libvosk.so" ]; then
    echo "✅ Found libvosk.so for $LIB_ARCH."
else
    echo "❌ ERROR: libs/$LIB_ARCH/libvosk.so not found!"
    exit 1
fi
echo ""

# 5. Setup Language Models Directory
echo "[5/7] Setting up Vosk Language Models..."
mkdir -p models

shopt -s nullglob
EXISTING_FOLDERS=(models/*/)
EXISTING_ZIPS=(models/*.zip)

if [ ${#EXISTING_FOLDERS[@]} -gt 0 ]; then
    echo "✅ Existing model folder(s) detected. Skipping setup step."
elif [ ${#EXISTING_ZIPS[@]} -gt 0 ]; then
    echo "📦 Found zip files in models directory. Extracting instead of downloading..."
    for f in "${EXISTING_ZIPS[@]}"; do
        echo "Extracting $f..."
        unzip -o "$f" -d models/
    done
    echo "✅ Extraction complete. Models ready."
else
    # Prompt for custom link, local path or use default
    echo "No models found. You can:"
    echo "  - Paste a custom Vosk model download URL (.zip)"
    echo "  - Provide a local path to a .zip file on your phone (e.g., /sdcard/Download/vosk-model.zip)"
    echo "  - Leave blank and press Enter to use the default High-Performance Bundle."
    read -p "Input (URL or Path): " USER_INPUT

    if [ -z "$USER_INPUT" ]; then
        BUNDLE_LINK="https://file-share.nannaungoo.workers.dev/file/65edf787-0d1b-4f1a-8f2e-1f7dd1321259?download=true"
        BUNDLE_FILE="models/vosk-models-bundle.zip"
        echo "Attempting to download default bundle..."
        if wget -c "$BUNDLE_LINK" -O "$BUNDLE_FILE"; then
             FILE_SIZE=$(wc -c <"$BUNDLE_FILE")
             if [ "$FILE_SIZE" -gt 100000 ]; then
                 unzip -o "$BUNDLE_FILE" -d models/ && rm "$BUNDLE_FILE"
                 echo "✅ Model successfully installed from bundle."
             else
                 echo "⚠️ Bundle invalid. Reverting to small fallback..."
                 rm "$BUNDLE_FILE"
                 wget -c https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip -O models/en-us.zip
                 unzip models/en-us.zip -d models/ && rm models/en-us.zip
             fi
        fi
    elif [ -f "$USER_INPUT" ]; then
        echo "✅ Local file detected at: $USER_INPUT"
        echo "Extracting directly from storage..."
        unzip -o "$USER_INPUT" -d models/
        echo "✅ Model successfully installed from local path."
    else
        # Assume it is a URL
        BUNDLE_LINK="$USER_INPUT"
        BUNDLE_FILE="models/custom-vosk-model.zip"
        echo "Attempting to download from provided URL..."
        if wget -c "$BUNDLE_LINK" -O "$BUNDLE_FILE"; then
            FILE_SIZE=$(wc -c <"$BUNDLE_FILE")
            if [ "$FILE_SIZE" -gt 100000 ]; then
                unzip -o "$BUNDLE_FILE" -d models/ && rm "$BUNDLE_FILE"
                echo "✅ Model successfully installed from URL."
            else
                echo "❌ Invalid download file. Please check the URL."
                exit 1
            fi
        else
            echo "❌ Download failed. Link might be broken."
            exit 1
        fi
    fi
fi
echo ""

# 6. Setup Environment and Folders
echo "[6/7] Setting up environment and directories..."
mkdir -p data/input data/output
[ -f .env.example ] && [ ! -f .env ] && cp .env.example .env
echo "✅ Environment ready."
echo ""

# 7. Build application
echo "[7/7] Building application..."
make build

echo ""
echo "=== ✅ Setup Complete! ==="
echo "💡 RUN: To start the application, use 'make run'"
echo ""
