# Project Documentation: Professional Termux STT Pipeline

---

## 1. Project Overview (စီမံကိန်း အကျဉ်းချုပ်)

**English:**
This project is a high-performance, multi-stage audio processing pipeline. Its core purpose is to take raw audio/video input and produce translated text and synthesized speech. It is unique because it is designed to run efficiently on mobile hardware (Termux) and Linux servers (Ubuntu), offering both Online (Gemini API) and Offline (Local Llama) intelligence.

**Core Features:**
*   **Intelligent Input:** Automatically detects video files and extracts audio via FFmpeg.
*   **Segmented Transcription:** Uses Vosk to break audio into time-stamped text segments.
*   **Parallel Translation:** Splits text into batches and translates them simultaneously using asynchronous threads for maximum speed.
*   **Myanmar Localization:** Includes custom logic to fix Myanmar-specific punctuation and line breaks.
*   **Voice-to-Voice:** Converts translated text into high-quality Myanmar speech.

**Data Flow:**
1.  **Selection:** User picks a file (`.mp4`, `.mp3`, etc.) from the `input/` folder.
2.  **Conversion:** If it's a video, FFmpeg creates an MP3.
3.  **Probing:** The system "looks" inside the audio to find its sample rate and channels.
4.  **Transcription:** Vosk processes the audio data and produces `OwnedResult` blocks (text + timestamps).
5.  **Batching:** Results are grouped into batches of 10 or 20 lines.
6.  **Translation:** Each batch is sent to the AI (Gemini or Llama) in parallel.
7.  **Post-Processing:** Myanmar punctuation is applied.
8.  **Speech Synthesis:** Text is converted to WAV audio segments.
9.  **Combination:** All audio segments are merged into a final `_my.wav` file in the `output/` folder.

---

## 2. Getting Started Guide (စတင်ရန် လမ်းညွှန်)

**English:**
This project uses a `Makefile` to handle complex library paths and environment variables automatically.

**Standard Commands:**
*   **Build:** `make build` (Compiles the project).
*   **Run:** `make run` (Starts the interactive menu).
*   **Test:** `make test` (Runs unit tests with correct library links).
*   **Direct CLI:** `make run ARGS="transcribe test.mp3"` (Bypasses menu).

**Requirements & Setup:**
*   **External Tools:** `ffmpeg` (for video), `unzip`, `wget`, `make`.
*   **Libraries:** `libvosk.so` (included in `libs/`), `libllvm` (required on Termux).
*   **Environment Variables:** Create a `.env` file and add `GEMINI_API_KEY=your_key`.
*   **Vosk Model:** Downloaded automatically to `./vosk-model-small-en-us-0.15` on first run.
*   **Local AI (Optional):** Requires `llama-cpp` package on Termux and a `.gguf` model file.

---

## 3. Logic Preservation (Brain Dump - အရေးကြီးသော Logic များ)

### 3.1 The "Symphonia" Probing Logic (`src/engine/audio.rs` -> `transcribe_file`)
**Step-by-Step:**
1.  **Open & Metadata:** It opens the file and reads its size for the progress bar.
2.  **Hinting:** It tells the decoder what extension it *thinks* it is (mp3/wav) to speed up discovery.
3.  **The Probe:** It scans the first few bytes of the file to find the actual sample rate and channel count. **Why:** If we send 44.1kHz audio to a 16kHz recognizer, the transcription will be gibberish. This detection fixes that.
4.  **Stereo-to-Mono Merge:** If the file has 2 channels, it uses a "Sum and Average" loop: it takes two samples, adds them, and divides by two. **Why:** Vosk only understands mono.
5.  **Packet Loop:** It loops through audio packets, updates the progress bar, and feeds "waveform" bytes to Vosk until the file ends.

### 3.2 Parallel Batch Translation (`src/main.rs`)
**Step-by-Step:**
1.  **Batching:** It takes the long list of transcribed segments and groups them (e.g., lines 1-10, 11-20).
2.  **Arc & Mutex:** It wraps the results in an `Arc<Mutex<Vec>>`. **Why:** Multiple threads need to write to the same list safely without crashing.
3.  **Spawning:** It uses `tokio::spawn` to start many translation tasks at once.
4.  **ID Mapping:** It sends IDs (like `[5]`) to the AI. **Why:** Since threads finish at different times, we use the IDs to put the results back in the correct order.
5.  **Regex Parsing:** It uses a specific regex `(?m)^\[?(\d+)\]?[:\s.\-]*\s*(.*)$` to find the IDs and text in the AI's response.

### 3.3 Llama-Server Health Check (`src/main.rs`)
**Step-by-Step:**
1.  **Check Alive:** It first pings `http://127.0.0.1:8080/health`.
2.  **Auto-Spawn:** If it gets no answer, it runs `llama-server` as a background process.
3.  **The Wait Loop:** It loops 30 times, waiting 2 seconds each time.
4.  **Status Check:** It looks specifically for a `Success` status. **Why:** The server might be "up" but still loading a 2GB model into RAM. If we send text too early, it returns an error. This loop ensures the AI is actually "awake."

---

## 4. Standard Doc Comments (Standard Rust Doc Comments)

Copy and paste these above your items to generate professional documentation.

### 4.1 For the STT Engine (`src/engine/vosk.rs`)
```rust
/// Represents the core Vosk STT engine.
///
/// Holds the loaded language model and provides an interface to create
/// speech recognizers for transcription.
#[derive(Debug, Clone)]
pub struct SttEngine {
    /// The pre-loaded Vosk model.
    pub model: vosk::Model,
}

impl SttEngine {
    /// Initializes the engine by loading a model from the filesystem.
    ///
    /// # Arguments
    /// * `model_path` - Path to the directory containing the Vosk model files.
    ///
    /// # Returns
    /// Returns `Ok(Self)` if the model was loaded, or `SttError::ModelError` on failure.
    pub fn new(model_path: &str) -> Result<Self> { ... }
}
```

### 4.2 For Audio Transcription (`src/engine/audio.rs`)
```rust
/// Transcribes a local audio or video file into a list of results.
///
/// Automatically probes audio properties, converts stereo to mono,
/// and provides a visual progress bar during processing.
///
/// # Arguments
/// * `engine` - A reference to an initialized `SttEngine`.
/// * `path` - The filesystem path to the source audio/video file.
///
/// # Returns
/// Returns a vector of `OwnedResult` containing text and timings on success.
pub fn transcribe_file(engine: &SttEngine, path: &str) -> Result<Vec<OwnedResult>> { ... }
```

### 4.3 For the Translation Client (`src/services/translator.rs`)
```rust
/// A high-level client for batch translation using Google Gemini.
///
/// Manages networking, rate-limiting via the `governor` crate, 
/// and line-by-line result mapping.
pub struct GeminiClient {
    // ... internals ...
}

impl GeminiClient {
    /// Translates a batch of text segments with preserved line IDs.
    ///
    /// # Arguments
    /// * `lines` - A list of `(ID, Text)` tuples to translate.
    ///
    /// # Returns
    /// Returns a list of `(ID, TranslatedText)` on success.
    pub async fn translate_batch(&self, lines: &[(usize, String)]) -> Result<Vec<(usize, String)>> { ... }
}
```
