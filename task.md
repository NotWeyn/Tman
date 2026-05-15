Task: Wayland Screen Translator Implementation
[/] Phase 1: Project Initialization

[x] Scaffold Tauri + Svelte project.
[ ] Setup global CSS for modern, vibrant multi-color theme.
[ ] Add Rust backend dependencies (tokio, axum, serde, etc.).
[ ] Update Memory Bank to reflect project init.
[ ] Phase 2: Core Engine (Rust Backend)

[ ] Implement capture.rs (grim + slurp integration, image preprocessing).
[ ] Implement ocr.rs (Native Rust engine ocrs or leptess MVP).
[ ] Implement translate.rs (OpenAI/Google API integration).
[ ] Implement db.rs (SQLite history using sqlx).
[ ] Phase 3: Event Bus & Server

[ ] Implement broadcaster.rs (Tokio broadcast channel).
[ ] Implement server.rs (Axum server for Web UI & WebSocket).
[ ] Add QR code generation for local IP.
[ ] Phase 4: Frontend (Svelte UI)

[ ] Set up routing/pages layout.
[ ] Build Main Menu (Purple Theme, Capture buttons).
[ ] Build Settings Page (Blue Theme, API keys, Language).
[ ] Build History/Info Page (Red Theme, DB view).
[ ] Phase 5: Local Web UI

[ ] Create web-ui/index.html with WebSocket client.
[ ] Ensure real-time text display when translation arrives.
[ ] Phase 6: Future Enhancements (Post-MVP)

[ ] Implement Python OCR Sidecars (PaddleOCR/EasyOCR).