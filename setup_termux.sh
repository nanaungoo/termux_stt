#!/bin/bash

# Termux Setup Script for Rust Speech-to-Text System (Rust 2024)
# Run this script on your Termux device after cloning the repo

set -e

echo "=== Termux STT Setup (Senior Rust Architect Standards) ==="
echo ""

# 1. Install dependencies
echo "[1/5] Installing dependencies (including make)..."
pkg update -y && pkg upgrade -y
pkg install termux-api rust wget unzip make -y
echo "Dependencies installed."
echo ""

# 2. Navigate to project directory
cd "$(dirname "$0")"
PROJECT_DIR=$(pwd)
echo "Project directory: $PROJECT_DIR"
echo ""

# 3. Detect architecture
ARCH=$(uname -m)
echo "[2/5] Detecting architecture: $ARCH"
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
        echo "ERROR: Unsupported architecture: $ARCH"
        exit 1
        ;;
esac
echo "Target architecture identified as: $LIB_ARCH"
echo ""

# 4. Verify library existence in libs/
echo "[3/5] Verifying libvosk.so..."
if [ -f "libs/$LIB_ARCH/libvosk.so" ]; then
    echo "Found libvosk.so for $LIB_ARCH."
else
    echo "ERROR: libs/$LIB_ARCH/libvosk.so not found!"
    echo "Please ensure you have the full repository with pre-compiled libraries."
    exit 1
fi
echo ""

# 5. Download Vosk model if not present
echo "[4/5] Checking Vosk model..."
MODEL_DIR="vosk-model-small-en-us-0.15"
if [ ! -d "$MODEL_DIR" ]; then
    echo "Downloading Vosk model (English small)..."
    wget -c https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
    unzip vosk-model-small-en-us-0.15.zip
    rm vosk-model-small-en-us-0.15.zip
    echo "Model downloaded and extracted."
else
    echo "Model already exists. Skipping download."
fi
echo ""

# 6. Build using Makefile
echo "[5/5] Building application with Makefile..."
make build

echo ""
echo "=== Setup Complete! ==="
echo ""
echo "To run the application:"
echo "  cd $PROJECT_DIR"
echo "  make run"
echo ""
echo "To run unit tests:"
echo "  make test"
echo ""
echo "Note: The Makefile automatically handles LD_LIBRARY_PATH for you."
echo ""
