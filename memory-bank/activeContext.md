# Active Context

## Current Work Focus
- MVP Core successfully built and integrated.
- Evaluating Phase 6 (Future Enhancements) involving Python OCR Sidecars (PaddleOCR/EasyOCR).

## Recent Changes
- Implemented Phase 5: Local Web UI & Startup System.
- Created `web-ui/index.html` with a glassmorphism theme and auto-reconnecting WebSocket client.
- Modified `lib.rs` to start the Axum Web Server and `tokio` broadcaster loop on application startup.
- Hooked the SQLite database initialization to application setup (`db::init_db`).
- Wrote Tauri commands `capture_and_translate`, `get_history`, and `save_settings` connecting the Rust backend to Svelte UI.
- Validated build success via `cargo check`.

## Next Steps
- Consider packaging the application or implementing Phase 6 features (Python-based OCR engine sidecars).
- Ensure Python venv and uv tools are managed if Sidecars are adopted.

## Active Decisions and Considerations
- **Frontend Framework**: Svelte has been selected based on the user's approval.
- **Initial OCR Engine**: We will start with a native Rust OCR engine (`ocrs` or `leptess`) for the MVP to minimize dependency issues. Python sidecars (PaddleOCR/EasyOCR) are postponed to later phases.
- **UI Styling**: The user requested a modern, vibrant, and dynamic UI. We will use distinct colors for different sections (e.g., Purple for Main Menu, Blue for Settings, Red for Info) to avoid a monotonous design.

## Important Patterns and Preferences
- Use shell-out for `grim` and `slurp`.
- Decouple the translation engine and OCR engine using trait abstractions (`ocr.rs`, `translate.rs`).
- Event-driven communication for translation results (`broadcaster.rs`).
- Emphasize rich aesthetics in UI design.
- **Added Caveman Skill**: Installed the `JuliusBrussee/caveman` skill for token-efficient communication.
