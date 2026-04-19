# 🎓 Master Learning Log: Professional STT Pipeline Evolution
# (သင်ယူမှုမှတ်တမ်း အပြည့်အစုံ)

This log tracks the complete transformation of a "vibe-coded" prototype into a professional-grade, multi-stage AI pipeline.

---

## 🏗️ Phase 1: Architectural Foundation (ဗိသုကာ အုတ်မြစ်ချခြင်း)

### [2026-04-13] ISA & Library-First Design
**User Request:** Initial setup and refactoring for multi-platform support (ThinkPad & Termux).
**Architect Response:** Established the "Constitutional Mandate" in `.gemini/GEMINI.md`. Optimized for `x86_64` (Linux) and `aarch64` (Android).
**Learning Moment:**
- **Hardware Metaphor:** x86 is a high-end kitchen; ARM (Termux) is an efficient food truck. Ingredients (libraries) must be prepared for both.
- **Library-First:** Moving logic to `src/lib.rs` makes it reusable; `main.rs` becomes a simple menu.
**Key Commands:**
- `cargo build --target aarch64-linux-android --release`: Baking the ARM cake in the x86 kitchen (Cross-compilation).
- `git add . && git commit -m "feat: align project standards"`: Snapshotted the clean state.

---

## 🛠️ Phase 2: Professional Standards & UX (စံနှုန်းမီ တည်ဆောက်ခြင်း)

### [2026-04-14] Senior Standards & Error Diagnostics
**User Request:** Clean up the "messy" code and add better user feedback.
**Architect Response:** Transitioned to Rust 2024. Replaced `unwrap()` with `miette` and `thiserror` for "Constitutional" stability. Added progress bars.
**Learning Moment:**
- **Diagnostics:** `miette` provides rich, actionable error reports instead of generic crashes.
- **UX:** `indicatif` progress bars transform "black box" operations into transparent experiences.
**Key Commands:**
- `make test`: Automates `LD_LIBRARY_PATH` and runs unit tests. (The "Smart Chef" approach).
- `ldd libs/x86_64/libvosk.so`: Checking the "Electricity Plugs" (Shared library dependencies).

---

## 🤖 Phase 3: AI Pipeline & Local Intelligence (AI နှင့် လုပ်ငန်းစဉ် ချိတ်ဆက်ခြင်း)

### [2026-04-17] Advanced Pipeline Integration
**User Request:** Combine transcription with translation and audio generation.
**Architect Response:** Built a 3-stage pipeline: **Transcribe** (Vosk) ➔ **Translate** (Gemini/Llama) ➔ **TTS** (Gemini Live/Google).
**Learning Moment:**
- **Concurrency:** Used `tokio::spawn` to translate 20 lines at once (Parallel Processing).
- **Llama Resilience:** Implemented a health-check wait loop in `main.rs` to ensure the local AI "brain" is fully loaded into RAM before use.
**Key Commands:**
- `llama-server -m <model> --port 8080 -c 2048`: Manually launching the offline engine.
- `pkg install llama-cpp`: Installing the offline translation provider on Termux.

---

## 🗄️ Phase 4: Persistence & Refinement (ဒေတာသိမ်းဆည်းမှုနှင့် ပြုပြင်မွမ်းမံခြင်း)

### [2026-04-19] MyMemory & Modular Refactor
**User Request:** Add a way for the AI to "remember" translations and organize the code better.
**Architect Response:** Integrated a local SQLite database (MyMemory) using `sqlx`. Performed a full **Modular Refactor** (`engine/`, `services/`, `ui/`).
**Learning Moment:**
- **Modularization:** Like building a wardrobe with specific drawers instead of a giant pile of clothes.
- **Persistence:** Text data is small; storing 100,000 sentences only takes ~20MB, saving on API costs and battery life.
**Key Commands:**
- `ffmpeg -i input.mp4 -vn -acodec libmp3lame output.mp3`: Extracting audio from video (Preprocessing).
- `git reset --hard origin/main`: The "Time Machine" to restore a clean state after history cleaning.

---

## 📜 Technical Knowledge Base (နည်းပညာဆိုင်ရာ ဗဟုသုတများ)

### 1. API Management (RPM/TPM/RPD)
- **Problem:** Going too fast causes Google to block the user.
- **Solution:** We added "API Tiers" in the settings menu. `governor` acts as a "Traffic Controller" to space out requests evenly.

### 2. Sourcing-Safe Scripts
- **Problem:** Running `. setup_termux.sh` caused it to `cd` into the wrong folder.
- **Solution:** Switched to `${BASH_SOURCE[0]}` to ensure the script always finds its own home, no matter how it is called.

### 3. Audio Engineering
- **Problem:** STT failed on stereo audio.
- **Solution:** Implemented a "Sum and Average" algorithm to merge stereo channels into mono before feeding the Vosk "Translator."

---

## 🏁 Final Conclusion
The project has evolved from a simple script into a **Resilient, Modular AI Pipeline**. It stands as a testament to professional Rust engineering: safe, fast, and user-centric.
