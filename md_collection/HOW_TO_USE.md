# How to Use: Professional STT Pipeline

This guide covers the usage of the **Transcription ➔ Translation ➔ Audio Generation** pipeline.

---

## 🎤 Usage Modes (အသုံးပြုပုံများ)

### 1. Interactive Mode (Menu-Driven)
The recommended way to use the application. Just run:
```bash
make run
```
**Features:**
- **Auto-Discovery:** Automatically lists audio files inside the `input/` folder.
- **Smart Flow:** Asks if you want to skip transcription if results already exist.
- **AI Chain:** Prompts you to translate the text and generate a voice file after transcription.

### 2. Standard Commands (Manual)
If you prefer direct commands:
```bash
# Transcribe a specific file
make run ARGS="transcribe test.mp3"

# Start real-time microphone transcription (Termux only)
make run ARGS="record"
```

---

## 📂 Folder Management (ဖိုင်နှင့် ဖိုဒါများ)

- **Input Folder (`input/`)**: Place your `.mp3` or `.wav` files here.
- **Output Folder (`output/`)**: All results will be saved here:
    - `<file>.txt`: Original English transcript.
    - `<file>.srt`: Subtitles with timestamps.
    - `<file>_my.txt`: Myanmar translation.
    - `<file>_my.wav`: Generated Myanmar voice output.

---

## 🌐 AI Configuration (AI ပြင်ဆင်ခြင်း)

To enable Translation and TTS, you must configure your Gemini API Key:
1.  Copy `.env.example` to `.env`.
2.  Add your key: `GEMINI_API_KEY=your_key_here`.
3.  The app will automatically detect the key and enable AI features.

---

## 📡 Offline Translation (Gemma / llama.cpp)

You can run entirely offline translations using a local model without the Gemini API.

1.  **Download the Engine**: Install `llama.cpp` on your system.
    ```bash
    git clone https://github.com/ggerganov/llama.cpp
    cd llama.cpp && make
    ```
2.  **Download a Model**: Get a 4-bit quantized model (like `gemma-2-2b-it-Q4_K_M.gguf`).
3.  **Start the Local Server**: Before running `termux_stt`, start the background AI server:
    ```bash
    ./llama-server -m /path/to/gemma.gguf -c 2048 --port 8080
    ```
4.  **Toggle Offline Mode**: Run `make run` in this project, and select the `Toggle Offline Mode (llama.cpp)` option in the menu. Translations will now securely process on your local device!

---

## 📱 Platform Specifics (ပလက်ဖောင်းများ)

### Termux (Android)
- Ensure **Termux:API** is installed and Microphone permission is granted.
- Real-time recording uses `termux-microphone-record`.

### Ubuntu (Linux)
- Use `make run` to ensure the correct Linux-native libraries are linked.
- Real-time recording is currently disabled on Linux (File transcription only).

---

## 🧪 Development
Run the unit tests to verify modular components:
```bash
make test
```
