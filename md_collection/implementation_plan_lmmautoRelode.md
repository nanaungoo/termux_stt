# Automate Offline AI Engine Lifecycle

Currently, the user has to manually start and stop the `llama-server` process in a separate terminal. Our goal is to make the Rust application manage this automatically, ensuring an optimal user experience and responsible memory management on Termux (Android).

## Proposed Changes

### [MODIFY] `src/config.rs`
- Add `offline_model_path` string to the config struct. 
- Set default to `"./gemma-2b-it-Q4_K_M.gguf"`.
- This ensures users can change the model if they upgrade later.

### [MODIFY] `src/main.rs`
- **Pre-Translation Step:**
  - When the user chooses to translate with Offline Mode `ON`, the application will ping `127.0.0.1:8080/health` to see if the server is already running.
  - If not running, it will automatically spawn `./llama.cpp/llama-server` as a background child process using `std::process::Command`, passing in the `offline_model_path`.
  - It will display a spinner/loading message and poll the health endpoint until the server is fully loaded into memory (which can take a few seconds on mobile).
- **Post-Translation Step:**
  - After audio generation completes (or if the translation fails), the application will automatically send a `.kill()` signal to the local child process.
  - This prevents `llama-server` from permanently eating up ~1.6GB of the Android phone's RAM and draining the battery when the app is done.

## Open Questions

- By default, I will configure the system to **automatically kill** the AI server when the translation task finishes to free up your phone's memory. Does that sound good, or would you prefer it stays running permanently in the background for faster subsequent translations?
