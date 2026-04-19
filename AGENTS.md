# Repository Guidelines

## Project Structure & Module Organization
`src/` contains the Rust crate. Use `src/main.rs` for the CLI entrypoint, `src/lib.rs` for exported APIs, `src/engine/` for audio/STT processing, `src/ui/` for command parsing and interactive prompts, `src/config.rs` for runtime configuration, and `src/services/` for external service integrations. Native Vosk libraries live in `libs/<arch>/`, downloaded models live in `models/`, and sample media lives in `data/input/`. Notes in `md_collection/` and `antigravity_note/` are supporting documentation, not application code.

## Build, Test, and Development Commands
Use `make` for the common workflow because it sets `LD_LIBRARY_PATH` for local Vosk libraries.

- `make build`: build the crate with `cargo build`.
- `make run ARGS="transcribe --file data/input/testaudio.mp3"`: run the CLI with explicit arguments.
- `make test`: run `cargo test -- --nocapture`.
- `make clean`: remove build artifacts.
- `bash setup_termux.sh`: install Termux-oriented dependencies and environment setup.

For local quality checks, run `cargo fmt` and `cargo clippy --all-targets --all-features` before opening a PR.

## Coding Style & Naming Conventions
Follow standard Rust formatting: 4-space indentation, `snake_case` for functions/modules/files, `PascalCase` for types and enums, and `SCREAMING_SNAKE_CASE` for constants. Keep modules focused: engine code should stay under `src/engine/`, UI logic under `src/ui/`, and shared errors in `src/error.rs`. Prefer `miette`/`thiserror`-based error propagation over `unwrap()`.

## Testing Guidelines
The main test entrypoint is `cargo test` through `make test`. Add unit tests next to the code they cover with `#[cfg(test)]` modules, and use descriptive names such as `transcribe_file_handles_mp3_input`. The repository also contains `test_select.rs` as an ad hoc binary-style check; keep new regression tests inside the crate unless a standalone repro is necessary.

## Commit & Pull Request Guidelines
Recent history uses short, imperative subjects with a scope prefix, for example `docs: add GEMINI.md section to learning log`. Keep commits focused and use the same `type: summary` pattern (`feat:`, `fix:`, `docs:`, `refactor:`). PRs should include the user-facing change, affected commands or platforms (Ubuntu, Termux, Android), linked issues if any, and screenshots or terminal snippets when CLI behavior changes.

## Configuration & Assets
Do not hardcode secrets. Copy `.env.example` to `.env` for API-backed features. Large models and native libraries are required runtime assets, so document any new model path, architecture-specific library, or external dependency in the PR.
