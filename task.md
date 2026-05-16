Task: Wayland Screen Translator Implementation
[x] Phase 1: Project Initialization

[x] Scaffold Tauri + Svelte project.
[x] Setup global CSS for modern, vibrant multi-color theme.
[x] Add Rust backend dependencies (tokio, axum, serde, etc.).
[x] Update Memory Bank to reflect project init.
[x] Phase 2: Core Engine (Rust Backend)

[x] Implement capture.rs (grim + slurp integration, image preprocessing).
[x] Implement ocr.rs (Native Rust engine ocrs or leptess MVP).
[x] Implement translate.rs (OpenAI/Google API integration).
[x] Implement db.rs (SQLite history using sqlx).
[x] Phase 3: Event Bus & Server

[x] Implement broadcaster.rs (Tokio broadcast channel).
[x] Implement server.rs (Axum server for Web UI & WebSocket).
[x] Add QR code generation for local IP.
[x] Phase 4: Frontend (Svelte UI)

[x] Set up routing/pages layout.
[x] Build Main Menu (Purple Theme, Capture buttons).
[x] Build Settings Page (Blue Theme, API keys, Language).
[x] Build History/Info Page (Red Theme, DB view).
[x] Phase 5: Local Web UI & Startup System

[x] Create web-ui/index.html with WebSocket client.
[x] Ensure real-time text display when translation arrives.
[x] Integrate startup system in lib.rs (Init DB, Broadcaster, Axum Server).
[x] Phase 6: Future Enhancements (Post-MVP)

[x] Implement Python OCR Sidecars (PaddleOCR/EasyOCR -> RapidOCR).

Phase 7: Backend Integrations & Settings Wiring (Kritik Öncelik Sırasıyla)

- [x] 1. Config okuma/yazma sistemi (config.rs) - Temel altyapı
- [x] 2. Sunucu toggle'ı (Aç/kapa, Port, Local-only, Auto-start, QR Kod)
- [x] 3. Ekranı yakala butonu (grim + slurp tetikleme)
- [x] 4. OCR pipeline bağlantısı (Motor seçimi, Binary Path, GPU, Dil tespiti, Satır/Paragraf birleştirme, Min karakter eşiği)
- [x] 5. Çeviri provider bağlantısı (Aktif provider, API keys, Hedef/Kaynak dil, Önbellekleme, OpenAI/DeepL/Google/LibreTranslate ayarları)
- [x] 6. Görüntü Ön İşleme Ayarları (Gri tonlama, Binarization, Kontrast, Ölçekleme)
- [x] 7. Yakalama Modu Ayarları (Manuel/Interval/Değişim, Interval süresi, Eşik, Bölge hafızası)
- [x] 8. Geçmiş Ayarları (Kaydet toggle, Max kayıt, Temizle, Dışa aktar)
- [x] 9. Uygulama Ayarları (Sistem başlangıcında çalıştır, Sistem tepsisine küçült, Log seviyesi)