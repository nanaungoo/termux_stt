#!/bin/bash

# This script builds the Rust speech-to-text application for Termux.

# Ensure you are in the project directory
cd "$(dirname "$0")"

echo "Building the Rust project..."

# Set LD_LIBRARY_PATH to include the directory containing libvosk.so
# This assumes libvosk.so is in the project root directory
export LD_LIBRARY_PATH=$(pwd)

cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful! Executable is at target/release/termux_stt"
    echo "You can run it using: ./target/release/termux_stt"
else
    echo "Build failed. Please check the error messages above."
fi
