<div align="center">
  <h1>Tman 🚀</h1>
  <p><strong>Lightning-fast, Wayland-native screen translation tool.</strong></p>
</div>

Tman (formerly Screen Translator) is a modern, high-performance screen translation application specifically designed for Wayland-based window managers (like Hyprland), bypassing the limitations of traditional X11 transparent overlays. Built with a native Rust core and a sleek Svelte frontend.

## ✨ Key Features

- **Wayland Native Capture**: Utilizes `grim` and `slurp` for seamless, precise screen region selection and capture on Wayland.
- **Blazing Fast Native OCR**: Powered by `oar-ocr` using in-process ONNX models. No Python dependencies or slow sidecars—achieving real-time text extraction in under 100ms.
- **Multiple Translation Providers**: Instantly translate extracted text using OpenAI, Google Translate, or DeepL.
- **Mobile Link & Web UI**: Scan a QR code to view real-time translations on your phone or tablet via an embedded Axum WebSocket server and a beautiful glassmorphic web UI.
- **Offline History & Caching**: SQLite-based caching prevents redundant API calls and keeps a history of your translations.
- **Multilingual Support**: Fully localized interface supporting English, Turkish, German, Spanish, Russian, Japanese, and Simplified Chinese.
- **Built-in Auto-Updater**: Zero-config auto-updates powered by Tauri and GitHub Releases.

## 🛠️ Architecture & Tech Stack

Tman completely abandons external Python processes in favor of a robust, fully native architecture:

- **Frontend**: Tauri v2, Svelte (Vite), Tailwind CSS (Glassmorphism design)
- **Backend Core**: Rust, Tokio (Async)
- **OCR Engine**: `oar-ocr` (In-memory PaddleOCR v5 ONNX models)
- **Web/WebSocket Server**: Axum
- **Database**: SQLx with SQLite
- **System Dependencies**: `grim`, `slurp` (Wayland screen capture)

## 📦 Installation & Setup

### Prerequisites
Make sure you have the following installed on your system:
- [Rust](https://www.rust-lang.org/tools/install) (1.95+)
- [Node.js](https://nodejs.org/) (22+)
- `grim` and `slurp` installed on your Wayland system.

### Running Locally

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/Tman.git
   cd Tman
   ```

2. **Install frontend dependencies:**
   ```bash
   npm ci
   ```

3. **Start in development mode:**
   ```bash
   npm run tauri dev
   ```
   *(Alternatively, you can use the localized `dev.sh` or `start_dev.sh` scripts provided in the project root.)*

4. **Build for release:**
   ```bash
   npm run tauri build
   ```

## 🤝 Contributing

Contributions are always welcome! Please see our [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to get started, our development workflow, and coding standards.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
