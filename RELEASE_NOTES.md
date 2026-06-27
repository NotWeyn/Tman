# Release Notes - v0.3.0

## 🚀 Enhancements & Features
- **Consistent UI Components**: Replaced all native `<select>` dropdown menus with a custom-designed, consistent `CustomSelect` component across the application (Language, Active Provider, Capture Mode, Scale, Contrast, Export Format, etc.).
- **Smart Dropdown Positioning**: Implemented dynamic bounds checking for custom dropdown menus (including `LanguagePicker`). Dropdowns will now automatically adjust their position to ensure they never overflow outside the window boundaries.

## 🛠 Refactoring & Technical Debt
- **Frontend Architecture**: Completely modularized the monolithic UI (`+page.svelte`) by extracting settings into separate, maintainable Tab components (e.g., `CaptureTab`, `TranslateTab`, `HistoryTab`, etc.).
- **State Management**: Centralized all UI state using Svelte writable stores in `stores.ts` for cleaner data flow and reactivity.
- **Backend Architecture**: Refactored the God-object `capture_and_translate` function in `src-tauri/src/lib.rs`. Extracted complex orchestration into focused helper methods (`do_capture`, `do_ocr`, `do_translate`, `do_broadcast`), greatly improving code readability and maintainability.
- **Code Cleanup**: Removed unnecessary `unwrap()` calls in OCR bindings and fixed various unused variable warnings to ensure robust compilation.

## 🐛 Bug Fixes
- Addressed accessibility (`a11y`) warnings in UI components.
- Fixed translation cache lookup issues.
