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

# Attempt to find any existing model folders
shopt -s nullglob
EXISTING_MODELS=(models/*/)

if [ ${#EXISTING_MODELS[@]} -eq 0 ]; then
    echo "No models found. You can choose to download the fast small model or the accurate large model."
    echo "1) English (Small - Fast - 40MB)"
    echo "2) English (Large - Highly Accurate - 1.8GB)"
    read -p "Select a model to download [1/2]: " choice
    
    case $choice in
        2)
            echo "Downloading English Large Model (0.22)..."
            echo "This may take a long time depending on your connection (~1.8GB)..."
            wget -c https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip -O models/en-us-large.zip
            unzip models/en-us-large.zip -d models/ && rm models/en-us-large.zip
            ;;
        *)
            echo "Downloading English Small Model (0.15)..."
            wget -c https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip -O models/en-us-small.zip
            unzip models/en-us-small.zip -d models/ && rm models/en-us-small.zip
            ;;
    esac
    echo "✅ Model ready."
else
    echo "✅ Existing models detected in models/ directory."
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
