# Progress

## What works
- Specifications are clearly defined.
- Memory bank is established.
- Tauri + Svelte project has been initialized.

## What's left to build
- **Project Scaffold:** Initialize Tauri v2 project.
- **Backend (Rust):**
  - Capture module (`grim`/`slurp`)
  - Image preprocessing
  - OCR abstraction & integration
  - Translation abstraction & API integration
  - SQLite Database integration for history
  - Axum Web server & WebSocket broadcaster
- **Frontend (Tauri UI):**
  - Settings UI (API keys, OCR selection, Capture mode)
  - History View
  - QR Code display
- **Web UI:**
  - Simple HTML/JS client to connect to WebSocket and display text.
- **Sidecars:**
  - Python scripts/binaries for PaddleOCR/EasyOCR (if chosen).

## Current Status
- Execution Phase. The implementation plan has been approved by the user. Initializing project structure and starting Phase 1.

## Known Issues
- None yet.
