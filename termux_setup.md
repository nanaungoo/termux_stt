# Termux Setup Guide for Rust Speech-to-Text System

This guide provides step-by-step instructions to set up your Termux environment on Android, install the necessary tools, and prepare the Vosk speech recognition model for use with the Rust application.

## 1. Install Termux and Termux:API

If you haven't already, install Termux and Termux:API from F-Droid or the Google Play Store.

*   **Termux**: [F-Droid](https://f-droid.org/packages/com.termux/) | [Google Play](https://play.google.com/store/apps/details?id=com.termux)
*   **Termux:API**: [F-Droid](https://f-droid.org/packages/com.termux.api/) | [Google Play](https://play.google.com/store/apps/details?id=com.termux.api)

After installation, open Termux and run the following command to update packages and install the `termux-api` command-line utility:

```bash
pkg update && pkg upgrade -y
pkg install termux-api -y
```

Grant microphone permissions to Termux and Termux:API in your Android settings.

## 2. Install Rust and Cargo

Install the Rust toolchain using `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

During the installation, choose option `1` for the default installation. After installation, restart your Termux session or run `source $HOME/.cargo/env` to add Cargo to your PATH.

Verify the installation:

```bash
rustc --version
cargo --version
```

## 3. Download Vosk Speech Recognition Model

Download a Vosk model. For this project, we recommend a small English model. Navigate to your project directory and download the model:

```bash
cd ~/termux_stt
wget https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip
unzip vosk-model-small-en-us-0.15.zip
mv vosk-model-small-en-us-0.15 vosk-model-small-en-us-0.15
```

**Note**: The `mv` command above moves the unzipped directory to itself, which effectively renames it if it was unzipped into a directory with a different name. Ensure the model directory is named `vosk-model-small-en-us-0.15` in the root of your `termux_stt` project directory.

## 4. Download Vosk Native Libraries

Since Vosk is a C++ library with Rust bindings, you need the pre-compiled native libraries for your Android device's architecture. Termux runs on `aarch64` (ARM64) for most modern Android devices. Download the `vosk-android` package and extract the `libvosk.so` for `arm64-v8a`.

```bash
cd ~/termux_stt
wget https://sourceforge.net/projects/vosk-speech-recognition.mirror/files/v0.3.45/vosk-android-0.3.45.zip/download -O vosk-android-0.3.45.zip
unzip vosk-android-0.3.45.zip
mv arm64-v8a/libvosk.so .
rm -rf arm64-v8a armeabi-v7a x86 x86_64 vosk-android-0.3.45.zip
```

This will place `libvosk.so` directly in your `termux_stt` project directory, making it discoverable during the build process.

## 5. Build the Rust Application

Once all prerequisites are installed and the Vosk model and native library are in place, you can build the Rust application. Refer to `build_script.sh` for the build command.
