# Goal Description

Implement an offline translation integration using a local `llama.cpp` server (llama-server) inside `termux_stt`. This will allow you to bypass the cloud Gemini API completely and use a downloaded 4-bit Gemma model to translate purely natively on your ThinkPad or Termux device!

## User Review Required

> [!IMPORTANT]
> The offline mode logic will assume that you are running `llama-server` in the background on your device before starting the transcription process. It will communicate with `http://127.0.0.1:8080/v1`, which is the standard exposed endpoint. I will update `HOW_TO_USE.md` to reflect this.

## Proposed Changes

### 1. Configuration Changes
#### [MODIFY] [config.rs](file:///home/nanaungoo/project/termux_stt/src/config.rs)
- Expand `Config` by adding an `offline_mode: bool` with a default of `false`.
- Ensure string formatting accounts for this value during the `.save()` process so your switch works permanently.

### 2. Main Menu Updates
#### [MODIFY] [main.rs](file:///home/nanaungoo/project/termux_stt/src/main.rs)
- Add a new **Toggle Offline Translation (llama.cpp)** button directly into the main interactive menu to allow easy switching back and forth.
- Update the translator initialization inside the transcription processor to load `LlamaClient` when `config.offline_mode` is enabled.

### 3. Translator Integration
#### [MODIFY] [translator.rs](file:///home/nanaungoo/project/termux_stt/src/services/translator.rs)
- Introduce a new Enum `TranslatorClient` to cleanly dispatch translations between the existing `GeminiClient` and the new `LlamaClient`.
- Build the `LlamaClient` struct, configuring it purely to generate translations via the `reqwest` crate interacting with the standard local OpenAI-compatible Chat format `llama-server` expects.
- Ensure the prompt strictly enforces strict translation limits without conversational outputs (a known quirk of offline models).

### 4. Documentation
#### [MODIFY] [HOW_TO_USE.md](file:///home/nanaungoo/project/termux_stt/HOW_TO_USE.md)
- Write explicit instructions on how to start the background `llama-server` process using a local Gemma .gguf model, and how to enable the feature in your CLI.

## Open Questions

None yet! The design is modular, meaning it will seamlessly fit into the current ecosystem.

## Verification Plan

### Automated Tests
- Will execute `cargo check` to guarantee there are zero compilation or dependency errors.
### Manual Verification
- I will simulate starting the tool to ensure the Configuration Menu handles the `offline_mode` boolean properly.
- You will be asked to download `gemma-2-2b-it-Q4.gguf`, run the `llama-server`, and fire a test transcription to confirm functionality on your hardware.
