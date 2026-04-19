#!/bin/bash

# Termux Setup Script for Professional STT-Translation-TTS Pipeline (Rust 2024)
# Run this script on your Termux device after cloning the repo

set -e

echo "=== 🎤 Professional Termux STT Pipeline Setup ==="
echo "=== (Senior Rust Architect Standards) ==="
echo ""

# 1. Install dependencies
echo "[1/8] Fixing potential package conflicts and installing dependencies..."
pkg uninstall openssl-tool -y || true
apt --fix-broken install -y || true
pkg update -y && pkg upgrade -y
# Added llama-cpp and ffmpeg for offline translation and video support
pkg install termux-api rust wget unzip make libllvm openssl ca-certificates llama-cpp ffmpeg -y
echo "Dependencies installed."

echo ""

# 2. Navigate to project directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]:-$0}")" && pwd)"
cd "$SCRIPT_DIR"
PROJECT_DIR=$(pwd)
echo "Project directory: $PROJECT_DIR"
echo ""

# 3. Detect architecture
ARCH=$(uname -m)
echo "[2/8] Detecting architecture: $ARCH"
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
echo "Target architecture: $LIB_ARCH"
echo ""

# 4. Verify pre-compiled libraries
echo "[3/8] Verifying libvosk.so..."
if [ -f "libs/$LIB_ARCH/libvosk.so" ]; then
    echo "Found libvosk.so for $LIB_ARCH."
else
    echo "ERROR: libs/$LIB_ARCH/libvosk.so not found!"
    echo "Please ensure you have the full repository with pre-compiled libraries."
    exit 1
fi
echo ""

# 5. Setup Language Models Directory
echo "[4/8] Setting up Vosk Language Models..."
mkdir -p models

# --- Priority: User-provided bulk models bundle ---
BUNDLE_LINK="https://file-share.nannaungoo.workers.dev/file/65edf787-0d1b-4f1a-8f2e-1f7dd1321259?download=true"
BUNDLE_FILE="models/vosk-models-bundle.zip"

# Check if any model folder already exists to avoid redundant downloads
shopt -s nullglob
EXISTING_MODELS=(models/*/)

if [ ${#EXISTING_MODELS[@]} -eq 0 ]; then
    echo "Attempting to download bulk models bundle from private link..."
    # Use -q for quiet but check status
    if wget -c "$BUNDLE_LINK" -O "$BUNDLE_FILE"; then
        # Safety check: if file is too small, it's likely an HTML error page
        FILE_SIZE=$(wc -c <"$BUNDLE_FILE")
        if [ "$FILE_SIZE" -lt 100000 ]; then
            echo "⚠️ Downloaded file is too small ($FILE_SIZE bytes). It is likely an HTML page, not a ZIP."
            echo "Please ensure you are using a DIRECT download link."
            rm "$BUNDLE_FILE"
        else
            echo "Bundle downloaded successfully. Extracting..."
            if unzip -o "$BUNDLE_FILE" -d models/; then
                echo "✅ Bulk models successfully installed."
                rm "$BUNDLE_FILE"
            else
                echo "⚠️ Extraction failed (Invalid ZIP). Deleting failed download..."
                rm "$BUNDLE_FILE"
            fi
        fi
    else
        echo "⚠️ Bulk download failed (Server unreachable)."
    fi
fi

# --- Standard Logic: Process existing zip files if any ---
ZIP_FILES=(models/*.zip)
if [ ${#ZIP_FILES[@]} -gt 0 ]; then
    echo "Found language model zip files. Extracting..."
    for f in "${ZIP_FILES[@]}"; do
        echo "Extracting $f..."
        if unzip -o "$f" -d models/; then
             # Suggest removing zip after success to save space
             echo "Unzipped $f."
        fi
    done
fi

# --- Fallback: Download default English model if still no folders exist ---
EXISTING_MODELS_AFTER=(models/*/)
if [ ${#EXISTING_MODELS_AFTER[@]} -eq 0 ]; then
    MODEL_DIR="models/vosk-model-small-en-us-0.15"
    echo "No models found after bundle attempt. Downloading default English model..."
    wget -c https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
    unzip vosk-model-small-en-us-0.15.zip -d models/
    rm vosk-model-small-en-us-0.15.zip
    echo "Default model ready."
fi
echo ""

# 6. Setup Offline Translation System (llama.cpp)
echo "[5/8] Checking Offline Translation Engine (llama-server)..."
if command -v llama-server &> /dev/null; then
    echo "llama-server found in system path."
else
    echo "llama-server not found. Please ensure 'pkg install llama-cpp' was successful."
fi
echo ""

# 7. Check Offline AI Model
echo "[6/8] Checking Offline AI Model (Qwen 2.5 1.5B)..."
MODEL_FILE="qwen2.5-1.5b-instruct-q4_k_m.gguf"
if [ ! -f "$MODEL_FILE" ]; then
    echo "The offline AI model ($MODEL_FILE) is not present."
    read -r -p "Do you want to download it now? [y/N] " response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])$ ]]; then
        wget -c "https://huggingface.co/Qwen/Qwen2.5-1.5B-Instruct-GGUF/resolve/main/qwen2.5-1.5b-instruct-q4_k_m.gguf" -O "$MODEL_FILE"
    fi
fi
echo ""

# 8. Setup Environment and Folders
echo "[7/8] Setting up environment and folders..."
mkdir -p input output
[ -f .env.example ] && [ ! -f .env ] && cp .env.example .env
echo ""

# 9. Build application
echo "[8/8] Building application with Makefile..."
make build

echo ""
echo "=== ✅ Setup Complete! ==="
echo "To run the application: make run"
echo ""
