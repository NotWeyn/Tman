# Contributing to Tman

Thank you for your interest in contributing to Tman! This guide covers everything you need to know — from setting up the development environment to understanding the codebase internals and submitting your first pull request.

---

## Table of Contents

- [Development Environment Setup](#-development-environment-setup)
- [Project Architecture](#-project-architecture)
- [Core Design Principles](#-core-design-principles)
- [Codebase Walkthrough](#-codebase-walkthrough)
- [Adding a New Feature](#-adding-a-new-feature)
- [Adding a Translation Provider](#-adding-a-translation-provider)
- [Adding a New UI Language](#-adding-a-new-ui-language)
- [Frontend Guidelines](#-frontend-guidelines)
- [Rust Backend Guidelines](#-rust-backend-guidelines)
- [Commit & PR Workflow](#-commit--pr-workflow)
- [Reporting Bugs](#-reporting-bugs)
- [Suggesting Features](#-suggesting-features)
- [Good First Issues](#-good-first-issues)

---

## 🚀 Development Environment Setup

### Prerequisites

| Tool | Minimum Version | Why |
|---|---|---|
| **Rust** | 1.95+ | Compiles Tauri backend + OCR engine |
| **Node.js** | 22+ | Builds Svelte frontend |
| **npm** | (bundled) | Frontend package manager |
| **grim** | any | Wayland screenshot capture |
| **slurp** | any | Wayland region selection |
| **Wayland compositor** | — | Hyprland, Sway, or any wlroots-based WM |

### System Libraries

Tauri requires several system libraries. Install for your distro:

**Arch Linux:**
```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips grim slurp
```

**Ubuntu/Debian:**
```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev grim slurp
```

### Fork, Clone & Run

```bash
# 1. Fork on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/Tman.git
cd Tman

# 2. Install frontend dependencies
npm ci

# 3. Start development mode (hot-reload)
npm run tauri dev

# Or use the convenience script (auto-installs deps, sets Wayland flags):
./dev.sh
```

Development mode starts:
- **Vite dev server** on port `1420` — Svelte hot-reload
- **Tauri Rust backend** — recompiles on file changes in `src-tauri/`
- **Axum WebSocket server** — automatically starts on port `4000`

### Verifying Your Setup

After `npm run tauri dev` completes, you should see:

```
[startup] UI ready in Xms (DB/server loading in background)
[startup] DB ready in Xms
[startup] Server ready on port 4000 in Xms
```

If the app window appears and you can select a screen region with `slurp`, you're ready to contribute.

---

## 🏗️ Project Architecture

```
Tman/
├── src/                          # Svelte frontend
│   ├── routes/+page.svelte       # Main UI (single page, tab-based)
│   ├── lib/
│   │   ├── i18n.js               # Reactive i18n store (Svelte derived store)
│   │   ├── languages.ts          # 65+ languages with flag emoji + RTL support
│   │   └── LanguagePicker.svelte # Searchable language dropdown
│   ├── i18n/                     # Locale JSON files (en, tr, de, es, ru, ja, zh)
│   ├── app.css                   # Global design system (CSS custom properties)
│   └── app.html                  # HTML shell
│
├── src-tauri/                    # Rust backend
│   └── src/
│       ├── lib.rs                # Entry point, AppState, all Tauri commands
│       ├── capture.rs            # grim/slurp invocation + image preprocessing
│       ├── ocr.rs                # oar-ocr engine (lazy-init, memory-resident)
│       ├── translate.rs          # Translation provider adapters
│       ├── db.rs                 # SQLite schema, CRUD, translation cache
│       ├── server.rs             # Axum HTTP server + WebSocket handler
│       ├── broadcaster.rs        # tokio::broadcast event bus
│       ├── config.rs             # JSON config load/save + keyring secrets
│       └── utils.rs              # Image encoding helpers
│
├── web-ui/
│   └── index.html                # Glassmorphic mobile translation viewer
│
├── dev.sh                        # Dev launch script
├── start.sh                      # Production build + launch script
└── config.json                   # Default config template
```

### Data Flow

```
User Action
    │
    ▼
+page.svelte ──invoke()──► lib.rs (Tauri IPC)
    │                           │
    │                           ├─► capture.rs → grim/slurp (Wayland)
    │                           │       │
    │                           │       ▼
    │                           ├─► ocr.rs → oar-ocr (ONNX, in-memory)
    │                           │       │
    │                           │       ▼
    │                           ├─► db.rs → SQLite (cache check)
    │                           │       │
    │                           │       ▼
    │                           ├─► translate.rs → API call (if cache miss)
    │                           │       │
    │                           │       ▼
    │                           └─► broadcaster.rs → tokio::broadcast
    │                                   │
    ├◄── app.emit("translation-update") │
    │    (Tauri event, no images)       │
    │                                   ▼
    │                           server.rs → WebSocket → web-ui/index.html
    ▼                                                     (Mobile clients)
UI updates
```

### Key State Management

State lives in `AppState` (shared `Arc<AppState>` across all Tauri commands):

```rust
pub struct AppState {
    pub db: Mutex<Option<SqlitePool>>,        // Lazy-init DB (None until background task completes)
    pub broadcaster: Broadcaster,              // tokio::broadcast channel (capacity: 100)
    pub config: Mutex<AppConfig>,              // Runtime config (auto-saved to JSON on change)
    pub actual_port: Mutex<u16>,               // Bound server port (may differ from config)
    pub server_shutdown_tx: Mutex<Option<...>>, // oneshot for graceful server shutdown
    pub last_text: Mutex<Option<String>>,       // Deduplication for "degisim" (change detection) mode
    pub last_broadcast: Mutex<Option<String>>,  // WebSocket broadcast deduplication
    pub http_client: reqwest::Client,           // Shared HTTP client (single instance)
}
```

---

## 🧭 Core Design Principles

These are non-negotiable. PRs that violate these will be rejected.

### 1. No Python. No Sidecars. No External Processes (for OCR).

Tman previously used Python-based OCR sidecars (PaddleOCR, EasyOCR, RapidOCR). These caused:
- 2+ second delays per capture
- Uncontrolled process spawning
- Python runtime dependency hell

**All OCR MUST use the native Rust `oar-ocr` engine.** It runs in-memory, stays resident, and processes captures in ~50 ms.

### 2. Wayland First

Screen capture uses `grim` and `slurp`. Do **NOT** add X11-specific code (like `xdotool`, `xwininfo`, `XFixes`) unless:
- It is cleanly separated behind a feature flag or runtime check.
- It does not break Wayland functionality.
- You have a clear use case.

### 3. Never Block the Main Thread

The backend uses `tokio`. During startup:
- Config loading is sync (~1 ms, acceptable).
- **Everything else** (DB init, server startup, OCR model download) must happen in `tokio::spawn` background tasks.

Do NOT use `tauri::async_runtime::block_on()` or `std::thread::sleep()` on the setup thread.

### 4. Glassmorphic Design Language

The UI uses a consistent dark-mode, glassmorphic aesthetic:
- Dark backgrounds (`#1a1a1e`, `#131316`)
- Indigo accent (`#6366f1`)
- Subtle borders (`rgba(255, 255, 255, 0.08)`)
- Smooth transitions (`cubic-bezier(0.4, 0, 0.2, 1)`)
- Inter font family

New UI elements **must match** this design system. Reference `src/app.css` for the design tokens.

### 5. Every User-Facing String Must Be Localized

The app supports 7 languages. Any text visible to the user must go through the i18n system — never hardcode strings in Svelte templates.

---

## 📖 Codebase Walkthrough

### `capture.rs` — Screen Capture

Two public functions:

| Function | Purpose |
|---|---|
| `pick_region()` | Runs `slurp` to let user select a screen region. Returns geometry string like `"596,809 525x181"`. |
| `capture_region(cfg)` | Runs `grim -g <region> -` to capture PNG to stdout, then applies preprocessing pipeline. |

**Preprocessing pipeline** (configurable via `AppConfig`):
1. **Lanczos3 upscale** — if `pre_scale > 1.0`
2. **Grayscale** — if `pre_grayscale`
3. **Contrast adjustment** — `"hafif"` (+15) or `"guclu"` (+40)

Returns `(original_image, processed_image, region_string)`.

### `ocr.rs` — Native OCR Engine

- Uses a `static Mutex<Option<OAROCR>>` to keep the engine resident in memory.
- First call initializes the engine (downloads ~50 MB of ONNX models to `~/.oar` if not present).
- Subsequent calls reuse the same engine instance — ~50 ms per invocation.
- Saves a temp PNG to `/tmp/tman_oar_capture.png` because `oar-ocr` reads from disk.

Post-OCR processing:
- **Line merging** — replaces `\n` with spaces, removes hyphenated breaks.
- **Minimum character filter** — rejects results below `ocr_min_chars`.
- **Language detection** — `whichlang::detect_language()` on extracted text.

### `translate.rs` — Translation Providers

Four adapters, all with the same signature:

```rust
async fn translate_xxx(text, source_lang, cfg, client) -> Result<String, String>
```

| Provider | Function | API Key Source | Notes |
|---|---|---|---|
| Google Translate | `translate_google` | None required | Uses free `translate.googleapis.com` endpoint |
| OpenAI-compatible | `translate_openai` | Keyring: `openai_key` | Configurable endpoint + model |
| DeepL | `translate_deepl` | Keyring: `deepl_key` | Auto-detects free vs. pro by key suffix `:fx` |
| LibreTranslate | `translate_libre` | None required | Self-hosted, configurable URL |

`translate_text()` dispatches to the correct adapter based on `cfg.trans_provider`.

### `db.rs` — SQLite Layer

- **Schema:** Single `history` table with `id, original_text, translated_text, source_lang, target_lang, timestamp`.
- **Cache:** `get_cached_translation()` looks up identical `(original_text, target_lang)` pairs to avoid redundant API calls.
- **Pruning:** After each insert, rows exceeding `history_max_records` are deleted (oldest first).
- **Pool:** `max_connections(2)` — kept small for a local desktop app.

### `server.rs` — Axum Web Server

- Serves static files from `web-ui/` directory (glassmorphic mobile viewer).
- WebSocket endpoint at `/ws` — broadcasts `TranslationEvent` JSON to all connected clients.
- Graceful shutdown via `tokio::sync::oneshot`.
- CORS enabled (any origin) for mobile browser access.

### `broadcaster.rs` — Event Bus

Simple `tokio::broadcast::channel(100)`:
- `send(TranslationEvent)` — called after each successful translation.
- `subscribe()` — called by WebSocket handler for each new client.
- Bounded at 100 events — old events are dropped if a slow consumer falls behind.

### `config.rs` — Configuration & Secrets

- **Config path:** `~/.local/share/tman/config.json` (default). Overridable via `config_pointer.txt`.
- **Load/Save:** `serde_json` serialization with `#[serde(default)]` for forward-compatibility.
- **Secrets:** `keyring-rs` crate wraps the OS keyring (GNOME Keyring, KDE Wallet). Never stored in JSON.

### `lib.rs` — Application Entry & Tauri Commands

All Tauri IPC commands are defined here. Key patterns:
- `get_db(&state)` — helper that returns the lazy-initialized DB pool or an error message.
- Auto-save config — frontend reactive statement (`$:`) triggers `saveConfig()` on any state change.
- Broadcast deduplication — `last_broadcast` Mutex skips identical consecutive WebSocket messages.
- Tauri events — `app.emit("translation-update", ...)` sends lightweight events (no base64 images) to the frontend.

### Frontend: `+page.svelte`

Single-page app with tab navigation. Key patterns:

| Pattern | How It Works |
|---|---|
| **Tab system** | `activeTab` string switches between `{#if}` blocks |
| **Auto-save** | Reactive `$:` block watches all config variables, calls `saveConfig()` on any change |
| **i18n** | `$t('key.subkey')` derived store resolves dot-separated keys from locale JSON |
| **Tauri IPC** | `invoke('command_name', { args })` calls Rust `#[tauri::command]` functions |
| **Server toggle** | `handleServerToggle()` calls `toggle_server` command + refreshes QR code |
| **Overlay system** | `showOverlay(msg, title, type)` renders modal with error/success/info styling |

### Frontend: Design System (`app.css`)

All colors, spacing, and transitions are CSS custom properties:

```css
--bg-main: #1a1a1e;           /* Main background */
--bg-sidebar: #131316;        /* Sidebar background */
--bg-surface: #232328;        /* Card/input backgrounds */
--accent-color: #6366f1;      /* Primary indigo */
--danger-color: #ef4444;      /* Error/destructive red */
--success-color: #10b981;     /* Confirmation green */
--border-color: rgba(255, 255, 255, 0.08);  /* Subtle borders */
--radius-md: 8px;             /* Default border radius */
--transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
```

Use these tokens — do NOT hardcode colors or transitions.

---

## 🔧 Adding a New Feature

### Step-by-step checklist

1. **Rust side** (if backend logic needed):
   - Add your logic in the appropriate module (or create a new one in `src-tauri/src/`).
   - If it's a new module, add `pub mod your_module;` to `lib.rs`.
   - Add config fields to `AppConfig` in `config.rs` with `#[serde(default)]` + `Default` impl.
   - Expose via `#[tauri::command]` in `lib.rs`.
   - Register in `tauri::generate_handler![]`.
   - Run `cargo check` — **zero warnings required**.
   - Run `cargo fmt` and `cargo clippy`.

2. **Frontend side** (if UI needed):
   - Add your UI to the appropriate tab in `+page.svelte`, or create a new tab entry in the `tabs` array.
   - Use existing CSS classes from `app.css` (`.btn`, `.form-input`, `.section`, `.row`, etc.).
   - Use the i18n system for all text: `{$t('your_section.your_key')}`.
   - If adding config state, add it to the reactive `$:` auto-save block.

3. **Localization** (required for any user-facing text):
   - Add keys to **all 7** locale files in `src/i18n/`.
   - Follow existing key hierarchy (e.g., `"capture.your_key"`, `"translate.your_key"`).

4. **Testing**:
   - Test the full capture → OCR → translate pipeline.
   - Verify mobile Web UI still receives WebSocket events.
   - Switch languages in the UI to verify i18n completeness.

---

## 🌐 Adding a Translation Provider

To add a new translation provider (e.g., `MyTranslate`):

### 1. Backend (`src-tauri/src/translate.rs`)

Add a new adapter function:

```rust
async fn translate_mytranslate(
    text: &str,
    source_lang: &str,
    cfg: &AppConfig,
    client: &reqwest::Client,
) -> Result<String, String> {
    let api_key = get_secret("mytranslate_key");
    if api_key.is_empty() {
        return Err("MyTranslate API key not found in keyring".to_string());
    }

    // Make API request using the shared `client`
    let res = client.post("https://api.mytranslate.com/v1/translate")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "text": text,
            "source": source_lang,
            "target": cfg.trans_target_lang,
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        return Err(format!("MyTranslate API error: {}", res.status()));
    }

    let res_json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;

    res_json.get("translated_text")
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .ok_or("Invalid MyTranslate response".to_string())
}
```

Add the match arm to `translate_text()`:

```rust
match cfg.trans_provider.as_str() {
    "openai" => translate_openai(text, source_lang, cfg, client).await,
    "deepl" => translate_deepl(text, source_lang, cfg, client).await,
    "libre" => translate_libre(text, source_lang, cfg, client).await,
    "mytranslate" => translate_mytranslate(text, source_lang, cfg, client).await,  // ← NEW
    _ => translate_google(text, source_lang, cfg, client).await,
}
```

### 2. Frontend (`+page.svelte`)

Add the option to the provider dropdown:

```svelte
<select bind:value={activeProvider} class="form-select w-auto">
  <option value="openai">OpenAI-compatible</option>
  <option value="google">Google Translate</option>
  <option value="deepl">DeepL</option>
  <option value="libre">LibreTranslate (Local)</option>
  <option value="mytranslate">MyTranslate</option>   <!-- NEW -->
</select>
```

Add the provider-specific settings panel:

```svelte
{#if activeProvider === 'mytranslate'}
<section class="section">
  <h3 class="section-title">{$t('provider.mytranslate_title')}</h3>
  <div class="row full-col">
    <label>{$t('provider.mytranslate_key')}</label>
    <input type="password" bind:value={mytranslateKey} placeholder="Key..." class="form-input" />
  </div>
</section>
{/if}
```

### 3. i18n (all 7 locale files)

Add to `src/i18n/en.json` (and all other locale files):

```json
"provider": {
  ...
  "mytranslate_title": "MYTRANSLATE SETTINGS",
  "mytranslate_key": "API key"
}
```

---

## 🌍 Adding a New UI Language

To add a new interface language (e.g., French):

### 1. Create the locale file

Copy the English locale as a template:

```bash
cp src/i18n/en.json src/i18n/fr.json
```

Translate **every key** in `fr.json`. The key structure must remain identical — only values change.

### 2. Register in the i18n store

Edit `src/lib/i18n.js`:

```javascript
import fr from '../i18n/fr.json';  // ← ADD import

const translations = { tr, en, de, es, ru, ja, zh, fr };  // ← ADD to map
```

### 3. Add to the language dropdown in `+page.svelte`

Find the UI language dropdown in the "App" tab and add the option. The display name should be in the target language:

```svelte
<option value="fr">Français</option>
```

### 4. Verify

- Switch to the new language in the app.
- Navigate every tab — look for missing translations (they show as raw keys like `"capture.title"`).
- Check that the language persists after restart (`app_lang` is saved to config).

---

## 🎨 Frontend Guidelines

### Component Patterns

- **Single-page architecture** — `+page.svelte` contains the entire UI. Keep it tab-based.
- **No component library** — We use vanilla CSS with custom properties. No Tailwind, no component frameworks.
- **Reactive auto-save** — Any new config variable must be added to the reactive `$:` auto-save block and the `saveConfig()` function.
- **Inline SVG icons** — Icons are defined as SVG strings in the `icons` object at the top of `+page.svelte`. Use [Lucide Icons](https://lucide.dev/) style.

### CSS Rules

1. **Use design tokens** — Colors, radii, shadows, and transitions from `:root` in `app.css`.
2. **Use existing classes** — `.section`, `.section-title`, `.row`, `.row-info`, `.form-input`, `.form-select`, `.btn`, `.toggle-wrapper`, etc.
3. **No !important** — If you need to override a style, your selector specificity is wrong.
4. **Dark mode only** — The app is dark mode exclusively. No light mode support needed.
5. **Match the glassmorphic aesthetic** — Subtle borders, soft shadows, smooth transitions.

### i18n Rules

- Import: `import { t } from '$lib/i18n';`
- Usage: `{$t('section.key')}`
- The `$t()` function resolves dot-separated keys. If a key is missing, it returns the raw key string as fallback.
- **NEVER** hardcode user-facing strings in templates.

---

## ⚙️ Rust Backend Guidelines

### Code Quality

```bash
# Must pass with zero warnings
cargo check

# Must pass formatting
cargo fmt --check

# Should pass linting (fix any reasonable warnings)
cargo clippy
```

### Patterns to Follow

| Pattern | Example | Why |
|---|---|---|
| Shared HTTP client | `state.http_client` | Reuse connection pool, avoid DNS lookups |
| Lazy DB init | `get_db(&state).await?` | DB loads in background task, not blocking setup |
| Config via AppConfig | `state.config.lock().await.clone()` | Clone early, release lock fast |
| Secrets via keyring | `config::get_secret("key_name")` | Never store secrets in JSON |
| Error as String | `-> Result<T, String>` | Tauri IPC requires String errors |
| Performance logging | `eprintln!("[perf] ...")` | Pipeline timing visible in dev console |

### Patterns to Avoid

| Anti-Pattern | Why |
|---|---|
| `block_on()` in setup | Freezes UI during startup |
| `std::thread::sleep()` | Blocks the async runtime |
| `unwrap()` on I/O operations | App crashes on transient failures |
| Long-held Mutex guards | Can deadlock concurrent Tauri commands |
| Unbounded channels/collections | Memory leaks under sustained load |
| Hardcoded file paths | Use `dirs::data_local_dir()` or `dirs::data_dir()` |

---

## 📝 Commit & PR Workflow

### Branch Naming

```
feature/your-feature-name     # New features
fix/description-of-fix        # Bug fixes
docs/what-you-documented      # Documentation changes
refactor/what-changed         # Refactoring (no behavior change)
i18n/language-name            # New language additions
```

### Commit Messages

Use clear, descriptive commit messages. We follow the conventional commits style:

```
feat: add MyTranslate provider support
fix: prevent empty OCR results from triggering translation
docs: update CONTRIBUTING with provider guide
refactor: extract capture preprocessing into separate functions
i18n: add French locale
chore: bump oar-ocr dependency
```

### Pre-PR Checklist

Before submitting a pull request, verify:

- [ ] `cargo check` passes with **zero warnings**
- [ ] `cargo fmt --check` passes (no formatting issues)
- [ ] `cargo clippy` passes without new warnings
- [ ] Frontend renders correctly (`npm run tauri dev`)
- [ ] If UI text was added, all 7 locale files are updated
- [ ] The capture → OCR → translate pipeline still works end-to-end
- [ ] Mobile Web UI still receives WebSocket events (if server-related changes)
- [ ] No hardcoded user-facing strings in templates
- [ ] No API keys or secrets committed to config files

### Submitting a PR

1. Fork the repo and create a branch from `main`.
2. Make your changes following the guidelines above.
3. Push to your fork.
4. Open a Pull Request against `main`.
5. In the PR description:
   - Explain **what** changed and **why**.
   - Link any related issues.
   - Include screenshots if UI was changed.

---

## 🐛 Reporting Bugs

When opening a bug report, please include:

| Field | Example |
|---|---|
| **OS** | Arch Linux 6.8.2 |
| **Window Manager** | Hyprland 0.40.0 |
| **Wayland compositor** | wlroots-based |
| **Tman version** | v0.0.1 (from App tab) |
| **Steps to reproduce** | 1. Select region  2. Click translate  3. Error appears |
| **Expected behavior** | Translation should appear |
| **Actual behavior** | Error overlay: "OAR-OCR init failed: ..." |
| **Logs** | Set `app_log_level` to `"debug"`, reproduce, paste stderr output |

### Getting Debug Logs

1. Open Settings → App tab → set Log Level to **Debug**.
2. Restart the app from terminal: `npm run tauri dev`
3. Reproduce the bug.
4. Copy the terminal output — it includes `[startup]`, `[cache]`, `[perf]` tagged lines.

---

## 💡 Suggesting Features

We welcome feature suggestions! When proposing a new feature:

1. **Explain the use case** — What problem does it solve? Who benefits?
2. **Respect the architecture** — Does it fit within Wayland-first, native Rust, sub-100ms goals?
3. **Consider i18n impact** — Will it add user-facing strings? To how many locale files?
4. **Scope it** — Can it be implemented incrementally?

Open an issue with the `enhancement` label.

---

## 🎯 Good First Issues

If you're new to the project, here are some areas where contributions are welcome:

- **New locale translations** — Copy `en.json`, translate to your language, register in `i18n.js`.
- **Additional translation providers** — Follow the provider guide above.
- **UI polish** — Improve animations, add hover states, refine spacing.
- **Documentation** — Improve README, add inline code comments, expand this guide.
- **Bug fixes** — Check the issue tracker for bugs tagged `good-first-issue`.

---

## ❓ Need Help?

If you have questions about the codebase, architecture, or how to implement something:

- Open a GitHub issue with the `question` label.
- Start a GitHub Discussion.
- Read the [README](README.md) for a complete architecture and configuration reference.

Thank you for helping make Tman better! 🚀
