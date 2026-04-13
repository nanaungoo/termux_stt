# Termux Speech-to-Text System (Rust + Vosk)

This project provides an interactive speech-to-text system built with Rust, utilizing the Vosk speech recognition toolkit and Termux-API for audio capture. It is designed to run offline on Android devices via Termux.

## Features

*   **Offline Speech Recognition**: Uses Vosk, an open-source offline speech recognition library.
*   **Rust-based**: High-performance and memory-safe implementation.
*   **Termux Integration**: Leverages `termux-microphone-record` for direct audio input on Android.
*   **Interactive CLI**: Simple command-line interface for starting and stopping transcription.

## Prerequisites

To build and run this application on your Android device, you will need:

1.  **Termux**: A terminal emulator for Android. Install it from F-Droid or Google Play Store.
2.  **Termux:API**: An add-on for Termux to access device features like the microphone. Install it from F-Droid or Google Play Store.
3.  **Rust Toolchain**: The Rust programming language and Cargo build system.
4.  **Vosk Model**: A pre-trained Vosk speech recognition model (e.g., `vosk-model-small-en-us-0.15`).

## Setup and Installation

Follow the detailed instructions in `termux_setup.md` to prepare your Termux environment and then use the `build_script.sh` to compile and run the application.

## Usage

1.  Ensure the Vosk model is downloaded and placed in the project directory (the application will offer to download it automatically on the first run).
2.  Run the application:
    *   **Real-time transcription (Microphone)**:
        ```bash
        ./target/debug/termux_stt record
        ```
        (Or simply `./target/debug/termux_stt` as it defaults to record)
    *   **File transcription**:
        ```bash
        ./target/debug/termux_stt transcribe path/to/audio.mp3
        ```
3.  For more options, use the help command:
    ```bash
    ./target/debug/termux_stt --help
    ```

## Troubleshooting

*   **`termux-microphone-record: command not found`**: Ensure `termux-api` is installed and you have granted microphone permissions to Termux.
*   **`Failed to load Vosk model`**: Verify that the Vosk model file exists at the specified path and is not corrupted.
*   **`cannot find -lvosk`**: This indicates that the Vosk native library is not correctly linked. Ensure you have followed the `termux_setup.md` instructions for installing the Vosk libraries.

## Contributing

Feel free to open issues or submit pull requests on the GitHub repository.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
