<div align="center">
  <h1>Tman</h1>
  <p><strong>Lightning-fast, Wayland-native screen translation tool.</strong></p>
  <p>
    <a href="#-getting-started">Getting Started</a> •
    <a href="#-architecture">Architecture</a> •
    <a href="#-configuration-reference">Configuration</a> •
    <a href="#-mobile-link--web-ui">Mobile Link</a> •
    <a href="#-contributing">Contributing</a>
  </p>
</div>

---

Tman is a high-performance screen translation application purpose-built for Wayland compositors (Hyprland, Sway, etc.). It captures a screen region, extracts text via a native Rust OCR engine, translates it through your choice of provider, and streams the result to both the desktop UI and a mobile-friendly web interface — all in under 100 ms per cycle.

## ✨ Key Features

- **Wayland-native capture** — Uses `grim` + `slurp` for precise, compositor-integrated screen region selection.
- **Sub-100 ms native OCR** — Powered by [`oar-ocr`](https://github.com/greatv/oar-ocr) with PaddleOCR v5 ONNX models loaded once into memory. No Python. No sidecars.
- **Multiple translation providers** — Google Translate (free, no key), OpenAI-compatible APIs (GPT-4o-mini, etc.), DeepL, and LibreTranslate.
- **Mobile Link & Web UI** — Scan a QR code to view real-time translations on your phone or tablet via an embedded Axum WebSocket server and a glassmorphic web interface.
- **Smart caching** — SQLite-backed translation cache prevents redundant API calls; full offline history with export (JSON, CSV, TXT).
- **Multilingual interface** — UI fully localized in 7 languages: English, Turkish, German, Spanish, Russian, Japanese, and Simplified Chinese.
- **Secure API key storage** — Keys stored in OS keyring via `keyring-rs` (GNOME Keyring, KDE Wallet, etc.), never in plaintext config files.
- **Image preprocessing pipeline** — Configurable Lanczos3 upscaling, grayscale conversion, contrast adjustment, and binarization before OCR.
- **Automatic language detection** — Uses the `whichlang` crate to detect source language from extracted text.
- **Version check on startup** — Checks GitHub Releases for new versions and shows a non-intrusive notification.

---

## Table of Contents

- [Tech Stack](#-tech-stack)
- [Prerequisites](#-prerequisites)
- [Getting Started](#-getting-started)
- [Architecture](#-architecture)
- [Configuration Reference](#-configuration-reference)
- [Environment Variables & Secrets](#-environment-variables--secrets)
- [Available Scripts](#-available-scripts)
- [Mobile Link & Web UI](#-mobile-link--web-ui)
- [Translation Providers](#-translation-providers)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| **Desktop Shell** | [Tauri v2](https://v2.tauri.app/) |
| **Frontend** | [Svelte 5](https://svelte.dev/) + [SvelteKit](https://kit.svelte.dev/) (static adapter) |
| **Backend** | Rust (2021 edition) |
| **Async Runtime** | [Tokio](https://tokio.rs/) (full features) |
| **OCR Engine** | [`oar-ocr`](https://github.com/greatv/oar-ocr) — PaddleOCR v5, ONNX inference |
| **Web/WS Server** | [Axum](https://github.com/tokio-rs/axum) with WebSocket support |
| **Database** | [SQLx](https://github.com/launchbadge/sqlx) + SQLite |
| **HTTP Client** | [Reqwest](https://github.com/seanmonstar/reqwest) (shared singleton) |
| **Image Processing** | [`image`](https://github.com/image-rs/image) crate (Lanczos3, contrast, binarization) |
| **Language Detection** | [`whichlang`](https://crates.io/crates/whichlang) |
| **QR Code Generation** | [`qrcode`](https://crates.io/crates/qrcode) |
| **Secret Storage** | [`keyring`](https://crates.io/crates/keyring) (OS keyring integration) |
| **Screen Capture** | `grim` + `slurp` (Wayland utilities) |
| **Build System** | Vite 6 (frontend) + Cargo (backend) |

---

## 📋 Prerequisites

Before you begin, make sure you have the following installed:

| Dependency | Minimum Version | Purpose |
|---|---|---|
| **Rust** | 1.95+ | Compiles the Tauri backend and OCR engine |
| **Node.js** | 22+ | Builds the Svelte frontend |
| **npm** | (bundled with Node) | Package manager for frontend dependencies |
| **grim** | any | Wayland screenshot utility |
| **slurp** | any | Wayland region selection utility |
| **Wayland compositor** | — | Hyprland, Sway, or any wlroots-based WM |

### System libraries (may be needed)

Tauri requires several system libraries. On Arch Linux:

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips grim slurp
```

On Ubuntu/Debian:

```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev grim slurp
```

> **Note:** `grim` and `slurp` are only available on Wayland. This application does not support X11.

---

## 🚀 Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/NotWeyn/Tman.git
cd Tman
```

### 2. Install frontend dependencies

```bash
npm ci
```

### 3. Start in development mode

```bash
npm run tauri dev
```

This simultaneously starts the Vite dev server (port 1420) and the Tauri Rust backend with hot-reload.

**Alternative:** Use the convenience script:

```bash
./dev.sh
```

`dev.sh` automatically installs Node dependencies if `node_modules/` is missing, sets Wayland compatibility flags, and launches `npm run tauri dev`.

### 4. Build for production (release)

```bash
npm run tauri build
```

Or use the production launch script which builds if no binary exists:

```bash
./start.sh
```

`start.sh` features:
- **Skips rebuild** if `src-tauri/target/release/tman` already exists.
- **Zenity progress bar** (if `zenity` is installed) showing per-crate compilation progress.
- **Force rebuild** with `./start.sh -r` or `./start.sh --rebuild`.
- Automatically sets Wayland/WebKitGTK compatibility flags.

The compiled binary is at `src-tauri/target/release/tman`.

---

## 🏗️ Architecture

### High-Level Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                        │
│  ┌──────────────────┐          ┌──────────────────────────┐ │
│  │  Svelte Frontend │◄─IPC──►│      Rust Backend        │ │
│  │  (SvelteKit)     │         │                          │ │
│  │  • Main UI       │         │  capture.rs  → grim/slurp│ │
│  │  • Settings      │         │  ocr.rs      → oar-ocr   │ │
│  │  • History       │         │  translate.rs→ API calls  │ │
│  │  • i18n system   │         │  db.rs       → SQLite     │ │
│  │  • LanguagePicker│         │  server.rs   → Axum WS    │ │
│  └──────────────────┘         │  config.rs   → JSON+keyring│
│                                │  broadcaster.rs → events  │ │
│                                └──────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                                         │
                                    WebSocket
                                         │
                              ┌──────────┴──────────┐
                              │   Mobile Web UI     │
                              │   (Glassmorphic)    │
                              │   web-ui/index.html │
                              └─────────────────────┘
```

### Directory Structure

```
Tman/
├── src/                          # Svelte frontend source
│   ├── routes/
│   │   ├── +page.svelte          # Main application page (all UI)
│   │   ├── +layout.svelte        # Root layout
│   │   └── +layout.js            # SPA prerender config
│   ├── lib/
│   │   ├── i18n.js               # i18n store (reactive locale switching)
│   │   ├── languages.ts          # 65+ languages with flag emoji & RTL support
│   │   └── LanguagePicker.svelte # Searchable dropdown with country flags
│   ├── i18n/                     # Localization JSON files
│   │   ├── en.json               # English (default)
│   │   ├── tr.json               # Turkish
│   │   ├── de.json               # German
│   │   ├── es.json               # Spanish
│   │   ├── ru.json               # Russian
│   │   ├── ja.json               # Japanese
│   │   └── zh.json               # Simplified Chinese
│   ├── app.css                   # Global styles
│   └── app.html                  # HTML shell
│
├── src-tauri/                    # Rust backend (Tauri)
│   ├── src/
│   │   ├── lib.rs                # App entry, state, all Tauri commands
│   │   ├── main.rs               # Binary entry point
│   │   ├── capture.rs            # grim/slurp integration + image preprocessing
│   │   ├── ocr.rs                # oar-ocr engine (lazy-init, memory-resident)
│   │   ├── translate.rs          # Google, OpenAI, DeepL, LibreTranslate adapters
│   │   ├── db.rs                 # SQLite schema, history CRUD, translation cache
│   │   ├── server.rs             # Axum HTTP + WebSocket server
│   │   ├── broadcaster.rs        # tokio::broadcast event bus
│   │   ├── config.rs             # JSON config + keyring secret management
│   │   └── utils.rs              # Image encoding utilities
│   ├── Cargo.toml                # Rust dependencies
│   ├── tauri.conf.json           # Tauri app configuration
│   └── capabilities/             # Tauri v2 permission capabilities
│
├── web-ui/
│   └── index.html                # Glassmorphic remote translation viewer
│
├── dev.sh                        # Development launch script
├── start.sh                      # Production build + launch script
├── start_dev.sh                  # Alternative dev launch script
├── package.json                  # Node.js project config
├── svelte.config.js              # SvelteKit configuration
├── vite.config.js                # Vite build configuration
├── config.json                   # Default app configuration (template)
├── CONTRIBUTING.md               # Contribution guidelines
└── LICENSE                       # MIT License
```

### Request Lifecycle

1. **User action** → Clicks "Capture & Translate" or timer fires in auto-capture mode.
2. **Region selection** → `slurp` prompts the user to select a screen region (saved for repeat captures).
3. **Screen capture** → `grim -g <region> -` captures the region as PNG to stdout.
4. **Image preprocessing** → Rust `image` crate applies configurable pipeline: Lanczos3 upscaling → grayscale → contrast → binarization.
5. **OCR** → Preprocessed image passed to `oar-ocr` (PaddleOCR v5 ONNX models, resident in memory). Extracts text in ~50 ms.
6. **Language detection** → `whichlang` identifies source language from extracted text.
7. **Cache check** → SQLite lookup for identical `(original_text, target_lang)` pair. If hit, skip API call.
8. **Translation** → On cache miss, dispatches to selected provider (Google/OpenAI/DeepL/LibreTranslate).
9. **History save** → New translations inserted into SQLite; old records pruned to `history_max_records`.
10. **Broadcast** → `TranslationEvent` sent via `tokio::broadcast` to both:
    - Tauri frontend (via `app.emit("translation-update", ...)`) — without image payloads for performance.
    - Axum WebSocket server → all connected mobile clients.

### Performance Characteristics

| Stage | Typical Time |
|---|---|
| Screen capture (`grim`) | ~20 ms |
| Image preprocessing | ~5 ms |
| OCR (`oar-ocr`) | ~50 ms |
| Translation (cache hit) | ~0 ms |
| Translation (Google API) | ~200 ms |
| **Total pipeline (cache hit)** | **~75 ms** |
| **Total pipeline (API call)** | **~275 ms** |

Performance is logged per cycle: `[perf] capture=Xms ocr=Xms translate=Xms total=Xms`.

---

## ⚙️ Configuration Reference

Configuration is stored as JSON at `~/.local/share/tman/config.json` (Linux). The path can be overridden via a pointer file at `~/.local/share/tman/config_pointer.txt`.

### Full Configuration Schema

| Key | Type | Default | Description |
|---|---|---|---|
| **Server** | | | |
| `server_enabled` | `bool` | `true` | Enable the Mobile Link web server |
| `server_port` | `u16` | `4000` | Starting port (auto-increments if busy) |
| `server_local_only` | `bool` | `false` | Bind to `127.0.0.1` instead of `0.0.0.0` |
| `server_auto_start` | `bool` | `true` | Start server automatically on app launch |
| **Capture** | | | |
| `capture_mode` | `string` | `"manual"` | Capture mode: `"manual"` or `"degisim"` (auto-capture on text change) |
| `capture_interval_sec` | `u32` | `5` | Interval between captures in auto mode (seconds) |
| `capture_change_threshold` | `u32` | `5` | *(Legacy)* Pixel change threshold for auto-capture |
| `capture_last_region` | `string` | `""` | Last selected region (auto-saved, e.g. `"596,809 525x181"`) |
| **Preprocessing** | | | |
| `pre_grayscale` | `bool` | `false` | Convert capture to grayscale before OCR |
| `pre_binarize` | `bool` | `false` | Apply binary threshold (>128 → white, ≤128 → black) |
| `pre_contrast` | `string` | `"kapali"` | Contrast adjustment: `"kapali"` (off), `"hafif"` (+15), `"guclu"` (+40) |
| `pre_scale` | `f32` | `1.0` | Upscale factor (Lanczos3). Values > 1.0 enlarge the image. |
| **OCR** | | | |
| `ocr_source_lang` | `string` | `"eng"` | Manual source language code (used when auto-detect is off) |
| `ocr_auto_detect_lang` | `bool` | `true` | Automatically detect source language via `whichlang` |
| `ocr_merge_lines` | `bool` | `true` | Join multi-line OCR output into single line |
| `ocr_merge_paragraphs` | `bool` | `false` | Collapse double spaces after line merge |
| `ocr_min_chars` | `u32` | `1` | Minimum character count to accept OCR result |
| **Translation** | | | |
| `trans_provider` | `string` | `"google"` | Provider: `"google"`, `"openai"`, `"deepl"`, `"libre"` |
| `trans_target_lang` | `string` | `"tr"` | Target language code (ISO 639-1) |
| `trans_cache_enabled` | `bool` | `true` | Enable SQLite translation cache |
| `trans_openai_endpoint` | `string` | `"https://api.openai.com/v1/chat/completions"` | OpenAI-compatible API endpoint |
| `trans_openai_model` | `string` | `"gpt-4o-mini"` | Model name for OpenAI provider |
| `trans_libre_url` | `string` | `"http://localhost:5000/translate"` | LibreTranslate API endpoint |
| **History** | | | |
| `history_save` | `bool` | `true` | Save translations to SQLite history |
| `history_max_records` | `u32` | `1000` | Maximum history records (oldest pruned) |
| **Application** | | | |
| `app_log_level` | `string` | `"info"` | Log verbosity: `"bilgi"` (info), `"hata"` (error), `"hata_ayiklama"` (debug) |
| `app_lang` | `string` | `"en"` | UI language: `en`, `tr`, `de`, `es`, `ru`, `ja`, `zh` |

---

## 🔑 Environment Variables & Secrets

### API Keys (Stored in OS Keyring)

API keys are **never stored in config files**. They are managed via the OS keyring (`keyring-rs` crate) using the service name `tman`.

| Keyring Key | Provider | How to Set |
|---|---|---|
| `openai_key` | OpenAI / OpenAI-compatible | Settings panel in the app, or via Tauri command `set_secret` |
| `deepl_key` | DeepL | Settings panel in the app |

### Environment Variables

| Variable | Description | Set By |
|---|---|---|
| `WEBKIT_DISABLE_DMABUF_RENDERER` | Fixes WebKitGTK rendering issues on some Wayland compositors | `dev.sh` / `start.sh` (set to `1`) |
| `RUST_LOG` | Rust log level filter | Set automatically from `app_log_level` config at startup |
| `CARGO_PKG_VERSION` | Compile-time version string | Injected by Cargo during build |

---

## 📜 Available Scripts

### npm Scripts

| Command | Description |
|---|---|
| `npm run dev` | Start Vite dev server only (port 1420) |
| `npm run build` | Build Svelte frontend to `build/` |
| `npm run preview` | Preview the production frontend build |
| `npm run check` | Run `svelte-check` type validation |
| `npm run check:watch` | Run type checking in watch mode |
| `npm run tauri dev` | Start full Tauri dev environment (Vite + Rust backend, hot-reload) |
| `npm run tauri build` | Build production release (frontend + Rust, bundled) |

### Shell Scripts

| Script | Description |
|---|---|
| `./dev.sh` | One-command dev setup: installs deps if needed, sets Wayland flags, runs `npm run tauri dev` |
| `./start.sh` | Production launcher: builds release binary if missing, shows zenity progress, launches app |
| `./start.sh -r` | Force rebuild even if binary exists |
| `./start_dev.sh` | Alternative development launch script |

### Cargo Commands (from `src-tauri/`)

| Command | Description |
|---|---|
| `cargo check` | Fast compilation check (no binary output) |
| `cargo build` | Debug build |
| `cargo build --release` | Optimized release build |
| `cargo fmt` | Format Rust code |
| `cargo clippy` | Run Rust linter |

---

## 📱 Mobile Link & Web UI

Tman includes an embedded Axum web server that serves a glassmorphic remote translation viewer.

### How It Works

1. The server starts automatically on app launch (configurable via `server_auto_start`).
2. It binds to port `4000` by default (auto-increments up to +100 if busy).
3. A QR code is generated from the local network IP + port.
4. Scanning the QR code opens `web-ui/index.html` on your phone/tablet.
5. WebSocket connection at `/ws` streams `TranslationEvent` objects in real-time.
6. Each event includes: original text, translated text, source/target language, region coordinates, and optionally base64-encoded captured/processed images.

### Network Configuration

- **LAN access** (default): `server_local_only: false` → binds to `0.0.0.0`, accessible from any device on the same network.
- **Local only**: `server_local_only: true` → binds to `127.0.0.1`, accessible only from the host machine.

---

## 🌐 Translation Providers

### Google Translate (Default)

- **No API key required.** Uses the free `translate.googleapis.com` endpoint.
- Automatic source language detection.
- Rate limits may apply for heavy usage.

### OpenAI-Compatible

- Requires an API key (stored in OS keyring as `openai_key`).
- Configurable endpoint (`trans_openai_endpoint`) — works with any OpenAI-compatible API (OpenRouter, local LLMs, etc.).
- Configurable model (`trans_openai_model`, default: `gpt-4o-mini`).
- Uses a system prompt instructing the model to reply with only the translation.

### DeepL

- Requires an API key (stored in OS keyring as `deepl_key`).
- Automatically detects free vs. pro API by checking if key ends with `:fx`.

### LibreTranslate

- Self-hosted option. No API key needed by default.
- Configure the endpoint via `trans_libre_url` (default: `http://localhost:5000/translate`).

---

## 🔧 Troubleshooting

### `grim` or `slurp` not found

**Error:** `Failed to run slurp: No such file or directory`

**Solution:** Install Wayland capture utilities:
```bash
# Arch Linux
sudo pacman -S grim slurp

# Ubuntu/Debian (may need PPA or manual install)
sudo apt install grim slurp
```

### Black screen / WebKitGTK rendering issues

**Error:** App window is black or renders incorrectly.

**Solution:** The launch scripts set `WEBKIT_DISABLE_DMABUF_RENDERER=1` automatically. If running manually:
```bash
export WEBKIT_DISABLE_DMABUF_RENDERER=1
./src-tauri/target/release/tman
```

### OCR returns empty text

**Possible causes:**
1. Selected region is too small or contains no text.
2. Image preprocessing settings are too aggressive (try disabling binarization/contrast).
3. `ocr_min_chars` threshold is too high.

**Solution:** Check the preprocessed image in the Web UI, adjust `pre_scale`, `pre_grayscale`, and `pre_contrast` settings.

### OCR model download fails on first run

**Error:** `OAR-OCR init failed: ...`

**Cause:** `oar-ocr` auto-downloads PaddleOCR v5 ONNX models to `~/.oar` on first use. Network issues can cause this to fail.

**Solution:**
```bash
# Ensure ~/.oar directory exists and you have internet access
mkdir -p ~/.oar
# Re-run the app — models will download automatically
```

### Translation API errors

**Error:** `Google API error: 429` or `OpenAI API error: 401`

**Solutions:**
- **429 (rate limit):** Enable caching (`trans_cache_enabled: true`) to reduce API calls. Consider switching to a different provider.
- **401 (unauthorized):** Verify your API key is correctly set in the app settings. Keys are stored in the OS keyring — check with your system's keyring manager.

### Database locked / not ready

**Error:** `Veritabanı henüz hazır değil, lütfen birkaç saniye bekleyin.`

**Cause:** Database initializes asynchronously on startup. This error occurs if you trigger a capture before the DB is ready.

**Solution:** Wait 1-2 seconds after app launch for the background initialization to complete. Check logs for `[startup] DB ready in Xms`.

### Port already in use

**Cause:** Another process is using port 4000.

**Solution:** Tman auto-increments the port (up to 100 attempts). You can also change `server_port` in config. Check the app logs for the actual bound port: `[startup] Server ready on port XXXX`.

### Build failures (native dependencies)

**Error:** Compilation errors related to system libraries.

**Solution:** Install Tauri system dependencies for your distribution (see [Prerequisites](#-prerequisites)).

### Kitty terminal rendering conflict

**Cause:** Kitty terminal's `KITTY_WINDOW_ID` can interfere with WebKitGTK.

**Solution:** The launch scripts automatically unset this variable. If running manually:
```bash
unset KITTY_WINDOW_ID
```

---

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on:

- Development environment setup
- Architecture principles (no Python sidecars, Wayland-first, async-only)
- Coding guidelines (Rust: `clippy` + `fmt`, Frontend: modular Svelte, glassmorphic design language)
- How to add translations (add keys to all 7 JSON files in `src/i18n/`)
- Pull request workflow

---

## 📄 License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

Copyright © 2026 [NotWeyn](https://github.com/NotWeyn)
