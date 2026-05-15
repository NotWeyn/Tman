# System Patterns

## System Architecture
The system consists of three main layers:

### 1. Core Engine — Gateway (Rust Backend)
This is where the business logic resides. It acts as the gateway between the frontend settings and the actual processing.
- **Capture Module:** Uses `slurp` for region selection and `grim` for capturing the screenshot via shell execution (`std::process::Command::new("grim")`).
- **OCR Pipeline:** Handles image pre-processing (grayscale, binarization, contrast enhancement) and sends the image to the selected OCR engine.
- **Translation Manager:** Sends API requests using `reqwest` to the configured translation provider.
- **Event Bus:** Uses `broadcaster.rs` to publish results simultaneously to the Tauri UI and the Local Web Server.

### 2. Local Web Server (Mobile Link)
- Runs an `axum` server inside a `tokio` thread on `0.0.0.0:<port>`.
- Serves a static HTML/JS page (`web-ui/index.html`) that connects via WebSocket to receive live translations.
- Generates a QR code using the `qrcode` crate, displaying the local IP in the Tauri UI for easy mobile connection.

### 3. Control UI (Tauri Frontend)
- A settings and control interface (no screen overlay).
- Manages capture modes, OCR selection, language settings, API keys, translation history, and the web server toggle.

## Key Technical Decisions
- **OCR Sidecar Architecture:** Support for Python-based OCR engines (PaddleOCR, EasyOCR) as Tauri sidecars, alongside native Rust options (`leptess`, `ocrs`).
- **Image Pre-processing:** Essential for improving OCR accuracy (by ~40%). Uses the `image` crate.
- **Text Post-processing:** Fixes broken lines and paragraph flow common in OCR outputs.
- **Caching:** Uses SQLite (`sqlx`) to cache translation history, avoiding redundant API calls for identical texts.
- **Configuration:** Uses the `dirs` crate for XDG-compliant config file paths (`~/.config/<app-name>/config.toml`).

## Component Relationships
- Frontend updates settings in Rust Backend via Tauri IPC.
- Rust Backend triggers `grim`/`slurp`, processes data, and pushes events via Event Bus.
- Event Bus pushes to Tauri UI and WebSocket (Web UI).
