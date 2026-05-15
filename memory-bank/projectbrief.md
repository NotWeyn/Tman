# Project Brief

## Core Requirements & Goals
The project is a "Screen Translator" built primarily for Wayland-based window managers (like Hyprland) where traditional X11 overlay mechanisms are restricted due to security protocols.
The core goal is to provide a pipeline: **Screen Capture → OCR → Translation → Redirection**.

## Key Features
- Capture a specific region of the screen on Wayland.
- Perform OCR on the captured region to extract text.
- Translate the extracted text using various providers (OpenAI, Google Translate, DeepL).
- Display the translated text either in the Tauri GUI or broadcast it to a local web server (accessible via mobile/second screen).

## Scope
The application will not rely on traditional transparent overlays over the target window. Instead, it will use a separate GUI for settings and history, and direct translation output to the GUI or an external device (via QR code/local IP).
