# Walkthrough - Data Restructuring and Library Fix

I have successfully restructured the data directory and fixed the shared library error that was preventing the application from running on Ubuntu.

## Changes Made

### 1. Data Directory Update
I updated the default configuration in `src/config.rs` to reflect the new directory structure:
- **Input Directory**: Changed from `./input` to `./data/input`.
- **Output Directory**: Changed from `./output` to `./data/output`.

### 2. Library Fix (`liblog.so`)
To resolve the `error while loading shared libraries: liblog.so` on Ubuntu, I performed the following:
- Created a symbolic link in `libs/x86_64/liblog.so` pointing to the system's Android compatibility library (`/usr/lib/x86_64-linux-gnu/android/liblog.so.0`).
- Updated the `Makefile` to include the Android library path in `LD_LIBRARY_PATH` and switched from `PWD` to `CURDIR` for better path resolution.

### 3. Compilation Fix (`engine` module)
The application was failing to compile because the `engine` module had been moved to `src/services/engine`, but the rest of the codebase (including `src/lib.rs`) still expected it at `src/engine`. I moved the `engine` directory back to its correct location in `src/engine`.

## Verification Results

### `make run` Verification
Running `make run ARGS="--help"` now succeeds and displays the help menu:
```
Speech-to-Text tool for Termux and Ubuntu

Usage: termux_stt [OPTIONS] [COMMAND]

Commands:
  transcribe  Transcribe an audio file (mp3, wav)
  record      Real-time transcription from microphone (Termux only)
  help        Print this message or the help of the given subcommand(s)
...
```

### Path Verification
The application now correctly looks for files in the `data/input` directory.

> [!TIP]
> Since the Vosk model was not found in the root directory, the application will prompt you to download it on the first run. You can also manually move your model folder to the root of the project.
