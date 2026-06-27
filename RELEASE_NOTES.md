# Release Notes - v0.3.0

## What's New

### 🏗️ Major Architectural Refactoring
- **Frontend Modularization**: Refactored the monolithic `+page.svelte` application shell into a clean, modular tab-based structure. All UI sections are now separated into independent components (`AppTab`, `CaptureTab`, `OcrTab`, `TranslateTab`, `OptionsTab`, `HistoryTab`, `ServerTab`), dramatically improving code maintainability.
- **Centralized State Management**: Implemented a unified Svelte store (`src/lib/stores.ts`) to handle all application state. This eliminates prop-drilling, ensures consistent reactivity, and guarantees synchronization between the UI and the Tauri backend.
- **Backend Pipeline Refactor**: Extracted the monolithic God-function `capture_and_translate` in `src-tauri/src/lib.rs` into discrete, async pipeline stages (`do_capture`, `do_ocr`, `do_translate`, `do_broadcast`). This improves the readability and testability of the core application logic.

### ✨ UI & UX Improvements
- **Custom Select Menus**: Upgraded all native `<select>` dropdowns across the application to a new, unified `CustomSelect` component. This brings visual consistency to the interface, matching the premium look of the Language Picker.
- **Smart Window Bounds**: Expandable menus (like the Language Picker and the new Custom Select) now intelligently calculate their position. If opening a dropdown would cause it to overflow outside the application window, it dynamically shifts to stay visible while remaining anchored to its parent button.
- **Accessibility**: Resolved several `a11y` warnings by ensuring all form elements have associated labels and descriptive text.

### 🐛 Bug Fixes & Code Quality
- Removed unsafe `.unwrap()` calls in `ocr.rs` and replaced them with robust Rust error handling.
- Fixed Rust compilation warnings regarding unused variables in the translation pipeline.
- Enforced strict Rust formatting (`cargo fmt`) across the entire Tauri backend.
