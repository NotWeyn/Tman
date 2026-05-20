# Contributing to Tman

First off, thank you for considering contributing to Tman! It's people like you that make open-source tools great.

This document provides guidelines and a workflow for contributing to the project.

## 🚀 Getting Started

1. **Fork and Clone**: Fork the repository on GitHub and clone your fork locally.
2. **Setup Dependencies**:
   - Ensure you have **Rust 1.95+** and **Node.js 22+** installed.
   - You must be on a Linux environment (preferably a Wayland-based WM like Hyprland) with `grim` and `slurp` installed for capture testing.
   - Run `npm ci` in the root directory to install frontend dependencies.
3. **Run the App**:
   - Use `npm run tauri dev` or the provided `./dev.sh` to spin up the app in development mode.

## 🏗️ Architecture Overview

Tman is built on a specific, high-performance architecture. Before making changes, please understand the following core principles:
- **No Python Sidecars**: We have completely migrated away from Python-based OCR (like RapidOCR or EasyOCR). All OCR processes must use the native Rust `oar-ocr` engine to maintain sub-100ms response times.
- **Wayland First**: Our capture mechanism relies on `grim` and `slurp`. Avoid adding X11-specific overlay code unless it is cleanly separated and fallback-safe.
- **Tauri IPC**: Communication between the Svelte frontend and the Rust backend uses Tauri's IPC commands. 
- **Async Runtime**: The backend utilizes `tokio`. We take startup times seriously; avoid using blocking operations like `block_on` in the main thread during initialization.

## 📝 How to Contribute

### Reporting Bugs
If you find a bug, please open an issue and include:
- Your OS and Window Manager (e.g., Arch Linux, Hyprland).
- Steps to reproduce the bug.
- Expected vs. actual behavior.
- Any relevant logs (run the app with debug logs enabled).

### Suggesting Enhancements
We are open to new features! When suggesting a feature:
- Explain **why** this enhancement would be useful.
- Keep in mind the core goal of the project: a fast, Wayland-compatible screen translator.

### Submitting Pull Requests
1. Create a new branch from `main` (`git checkout -b feature/your-feature-name`).
2. Make your changes.
3. **Test your changes**:
   - Ensure `cargo check` passes with zero warnings.
   - Run `cargo fmt` to adhere to Rust formatting guidelines.
   - Ensure frontend code is formatted properly.
4. Commit your changes with clear, descriptive commit messages.
5. Push to your fork and submit a Pull Request.

## 🎨 Coding Guidelines
- **Rust**: Follow idiomatic Rust. We use `clippy` to maintain code quality. 
- **Frontend**: We use Svelte. Keep components modular. We use a glassmorphic design language—new UI elements should match this aesthetic.
- **Translations**: If you are adding a new feature that includes text, please add the corresponding keys to all language JSON files in `src/i18n/`.

## ❓ Need Help?
If you have questions about where to start or how a specific part of the codebase works, feel free to open an issue or start a discussion.

Thank you for helping make Tman better!
