#!/bin/bash

# Termux Setup Script for Rust Speech-to-Text System
# Run this script on your Termux device after cloning the repo

set -e

echo "=== Termux STT Setup ==="
echo ""

# 1. Install dependencies
echo "[1/5] Installing dependencies..."
pkg update -y && pkg upgrade -y
pkg install termux-api rust wget unzip -y
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
        LIB_DIR="aarch64"
        ;;
    armv7l|armv8l|arm)
        LIB_DIR="armv7"
        ;;
    x86_64)
        LIB_DIR="x86_64"
        ;;
    x86|i686)
        LIB_DIR="x86"
        ;;
    *)
        echo "ERROR: Unsupported architecture: $ARCH"
        exit 1
        ;;
esac
echo "Using library: libs/$LIB_DIR/libvosk.so"
echo ""

# 4. Copy libvosk.so to project root
echo "[3/5] Copying libvosk.so..."
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
echo "Script directory: $SCRIPT_DIR"
echo "Looking for library at: $SCRIPT_DIR/libs/$LIB_DIR/libvosk.so"

# Check what's in the libs directory
if [ -d "$SCRIPT_DIR/libs" ]; then
    echo "Available library directories:"
    ls -la "$SCRIPT_DIR/libs/"
else
    echo "ERROR: libs/ directory not found in $SCRIPT_DIR"
    exit 1
fi

if [ -f "$SCRIPT_DIR/libs/$LIB_DIR/libvosk.so" ]; then
    cp "$SCRIPT_DIR/libs/$LIB_DIR/libvosk.so" .
    echo "libvosk.so copied successfully."
else
    echo "ERROR: libs/$LIB_DIR/libvosk.so not found!"
    echo ""
    echo "Troubleshooting:"
    echo "1. Make sure you cloned the entire repository (including libs/ directory)"
    echo "2. Check if the library file exists: ls -la libs/$LIB_DIR/"
    echo "3. If missing, you may need to download libvosk.so from the Vosk releases page"
    echo ""
    echo "Available architectures in libs/:"
    ls libs/ 2>/dev/null || echo "  (libs/ directory not found)"
    exit 1
fi
echo ""

# 5. Download Vosk model if not present
echo "[4/5] Checking Vosk model..."
MODEL_DIR="vosk-model-small-en-us-0.15"
if [ ! -d "$MODEL_DIR" ]; then
    echo "Downloading Vosk model (this may take a while)..."
    wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
    unzip vosk-model-small-en-us-0.15.zip
    rm vosk-model-small-en-us-0.15.zip
    echo "Model downloaded and extracted."
else
    echo "Model already exists. Skipping download."
fi
echo ""

# 6. Set LD_LIBRARY_PATH and build
echo "[5/5] Building application..."
export LD_LIBRARY_PATH="$PROJECT_DIR"
cargo build --release

echo ""
echo "=== Setup Complete! ==="
echo ""
echo "To run the application:"
echo "  cd $PROJECT_DIR"
echo "  export LD_LIBRARY_PATH=\$(pwd)"
echo "  ./target/release/termux_stt"
echo ""
echo "Or add this to your ~/.bashrc to set LD_LIBRARY_PATH automatically:"
echo "  export LD_LIBRARY_PATH=$PROJECT_DIR:\$LD_LIBRARY_PATH"
echo ""
