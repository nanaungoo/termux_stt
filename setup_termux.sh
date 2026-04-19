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
# Removed llama-cpp as translation is no longer required
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

BUNDLE_LINK="https://file-share.nannaungoo.workers.dev/file/65edf787-0d1b-4f1a-8f2e-1f7dd1321259?download=true"
BUNDLE_FILE="models/vosk-models-bundle.zip"

shopt -s nullglob
EXISTING_MODELS=(models/*/)

if [ ${#EXISTING_MODELS[@]} -eq 0 ]; then
    echo "Attempting to download bulk models bundle..."
    if wget -c "$BUNDLE_LINK" -O "$BUNDLE_FILE"; then
        FILE_SIZE=$(wc -c <"$BUNDLE_FILE")
        if [ "$FILE_SIZE" -gt 100000 ]; then
            unzip -o "$BUNDLE_FILE" -d models/ && rm "$BUNDLE_FILE"
        else
            rm "$BUNDLE_FILE"
        fi
    fi
fi

EXISTING_MODELS_AFTER=(models/*/)
if [ ${#EXISTING_MODELS_AFTER[@]} -eq 0 ]; then
    echo "Downloading High-Quality English model (Large - 0.22)..."
    echo "This may take a while (~1.8GB)..."
    wget -c https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip -O models/en-us-large.zip
    unzip models/en-us-large.zip -d models/ && rm models/en-us-large.zip
    echo "✅ High-Quality model ready."
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
