# Technical Context

## Technologies Used
- **Frontend / UI:** Tauri v2, Svelte or SolidJS (for the desktop UI), Vanilla HTML/JS (for the Web UI).
- **Backend / Core:** Rust
- **Async Runtime:** `tokio`
- **Web/WebSocket Server:** `axum`
- **HTTP Client:** `reqwest`
- **Serialization:** `serde`, `serde_json`
- **Image Processing:** `image` crate
- **Language Detection:** `whichlang` crate (pure Rust, offline)
- **Database / Caching:** `sqlx` with SQLite
- **QR Code Generation:** `qrcode` crate
- **Path Management:** `dirs` crate
- **Screen Capture (Wayland):** `grim` and `slurp` (shell execution)

## OCR Options
- Rust-native: `leptess` (Tesseract) or `ocrs`
- Sidecars (Python): PaddleOCR, EasyOCR

## Translation Providers
- OpenAI-compatible API
- Google Translate API
- DeepL API

## Development Setup
Project Structure:
- `src-tauri/`: Rust backend, Tauri configuration, core logic modules (`capture.rs`, `ocr.rs`, `translate.rs`, `server.rs`, `broadcaster.rs`, `config.rs`).
- `src/`: Tauri frontend source (Svelte/SolidJS pages for Settings, Capture, History).
- `web-ui/`: Static files for the local web server.

## Technical Constraints
- Relies on `grim` and `slurp` being installed on the host system.
- Designed specifically for Wayland environments; fallback or alternative capture tools would be needed for X11 or other OS support.
