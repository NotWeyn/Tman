#!/bin/bash
# dev.sh - Development script with hot-reload for Screen Translator

set -e

# Ensure we are in the project directory regardless of where the script is called from
cd "$(dirname "$0")"

# --- Wayland / WebKitGTK Compatibility Fixes ---
# These are essential for the UI to render correctly on many Linux/Wayland setups
export WEBKIT_DISABLE_DMABUF_RENDERER=1
unset KITTY_WINDOW_ID 2>/dev/null || true

# --- Setup Python Sidecar (if needed) ---
# Ensure 'uv' is used for lightning-fast dependency management
if ! command -v uv &>/dev/null; then
    echo "⚠️ 'uv' is not installed. Python OCR engine may not work."
else
    if [ ! -d ".venv" ]; then
        echo "🐍 Creating Python virtual environment..."
        uv venv >/dev/null 2>&1 || true
    fi
    source .venv/bin/activate || true
    # Keep dependencies up to date silently
    uv pip install rapidocr-onnxruntime >/dev/null 2>&1 || true
fi

# --- Ensure Node dependencies are present ---
if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node dependencies..."
    npm install
fi

# --- Run Tauri in Dev Mode ---
# This is the feature you mentioned! 
# 'tauri dev' watches your files:
# 1. If you change Svelte/CSS/JS code -> Frontend hot-reloads instantly.
# 2. If you change Rust code (.rs) -> Backend automatically re-compiles and restarts.
echo "🚀 Starting development server with hot-reload..."
npm run tauri dev
