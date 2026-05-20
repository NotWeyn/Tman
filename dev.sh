#!/bin/bash
# dev.sh - Development script with hot-reload for Screen Translator (Tman)

set -e

# Ensure we are in the project directory regardless of where the script is called from
cd "$(dirname "$0")"

# --- Wayland / WebKitGTK Compatibility Fixes ---
export WEBKIT_DISABLE_DMABUF_RENDERER=1
unset KITTY_WINDOW_ID 2>/dev/null || true

# --- Node dependencies setup ---
if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node dependencies..."
    npm install
fi

# --- Run Tauri in Dev Mode ---
echo "🚀 Starting development server (Hot-reload active)..."
npm run tauri dev
