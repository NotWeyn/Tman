# Product Context

## Why this project exists
In Wayland-based window managers (such as Hyprland), the overlay mechanisms do not work as flexibly as they do in X11 due to strict security protocols. This makes existing on-screen translation tools incompatible or buggy. This project solves that problem by building a native, Wayland-compatible screen translator.

## Problems it solves
- Inability to use standard overlay-based screen translators on Wayland/Hyprland.
- High resource usage of existing tools (solved by using Rust for the backend).

## How it should work
1. The user opens the application and selects a region of the screen to translate.
2. The system captures the region, pre-processes the image, and extracts text using an OCR engine.
3. The extracted text is processed to fix formatting and then sent to a translation API.
4. The translated text is instantly broadcast to the main Tauri UI and/or a local web server interface.

## User Experience Goals
- Fast and lightweight execution.
- Easy access to translations either on the desktop (Tauri UI) or on a mobile device (by scanning a QR code to connect to the local web server).
- Support for different capture modes: Manual, Interval-based, and Change-detection (only translate if the screen region changes).
- Automatic language detection to minimize manual configuration.
