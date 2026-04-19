# Offline Translation Implementation Tasks

- `[x]` 1. Implement Configuration Updates (`src/config.rs`)
  - `[x]` Add `offline_mode` property
  - `[x]` Update default and `save` method
- `[x]` 2. Implement Translator Client (`src/services/translator.rs`)
  - `[x]` Create `TranslatorClient` enum dispatcher
  - `[x]` Add `LlamaClient` struct mapping to OpenAI Chat Completions API
  - `[x]` Move `GeminiClient` to use Enum pattern
- `[x]` 3. Implement CLI Updates (`src/main.rs`)
  - `[x]` Add "Toggle Offline Mode" to Interactive UI Menu
  - `[x]` Replace `std::sync::Arc::new(GeminiClient::new(...))` with `TranslatorClient`
- `[x]` 4. Update Documentation (`HOW_TO_USE.md`)
  - `[x]` Document `llama-server` usage
- `[x]` 5. Verification
  - `[x]` Check build with `cargo check`
