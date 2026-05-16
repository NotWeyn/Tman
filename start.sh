#!/bin/bash
# start.sh - Launch script for Wayland Screen Translator

set -e

# Ensure we are in the project directory regardless of where the script is called from
cd "$(dirname "$0")"

# --- Wayland / WebKitGTK Compatibility Fixes ---
export WEBKIT_DISABLE_DMABUF_RENDERER=1
unset KITTY_WINDOW_ID 2>/dev/null || true

# --- Silent dependency setup ---
if ! command -v uv &>/dev/null; then
    echo "⚠️ 'uv' is not installed. Please install it."
    exit 1
fi
if [ ! -d ".venv" ]; then uv venv >/dev/null 2>&1 || true; fi
source .venv/bin/activate || true
uv pip install rapidocr-onnxruntime >/dev/null 2>&1 || true
if [ ! -d "node_modules" ]; then
    npm install >/dev/null 2>&1 || true
fi

# --- Tauri Build ---
# tauri build --no-bundle --debug:
#   1. Runs "bun run build" (frontend) automatically via beforeBuildCommand
#   2. Embeds the frontend INTO the binary (no vite dev server needed)
#   3. --no-bundle = skip deb/AppImage packaging
#   4. --debug = faster compilation (no optimizations)
BINARY="src-tauri/target/debug/tman"

if [ ! -f "$BINARY" ]; then
    # Auto-detect total package count from cargo metadata
    TOTAL=$(cd src-tauri && cargo metadata --format-version 1 2>/dev/null \
        | python3 -c "import sys,json; print(len(json.load(sys.stdin).get('packages',[])))" \
        2>/dev/null || echo "700")

    if command -v zenity &>/dev/null; then
        echo "🔨 İlk derleme başlatılıyor (Bu işlem birkaç dakika sürebilir)..."
        # tee /dev/stderr prints the compiler output to the terminal while also feeding zenity
        (npm run tauri build -- --no-bundle --debug 2>&1 || true) | tee /dev/stderr | {
            COUNT=0
            while IFS= read -r line; do
                case "$line" in
                    *Compiling*)
                        COUNT=$((COUNT + 1))
                        PCT=$((COUNT * 100 / TOTAL))
                        [ "$PCT" -gt 99 ] && PCT=99
                        CRATE=$(echo "$line" | sed 's/.*Compiling //' | cut -d' ' -f1)
                        echo "$PCT"
                        echo "# Derleniyor ($COUNT/$TOTAL): $CRATE"
                        ;;
                    *Finished*)
                        echo "100"
                        echo "# Derleme tamamlandı! ✨"
                        ;;
                esac
            done
        } | zenity --progress \
            --title="Tman - İlk Derleme" \
            --text="İlk açılış derlemesi hazırlanıyor..." \
            --percentage=0 \
            --auto-close \
            --width=500 \
            --no-cancel
    else
        echo "🔨 İlk derleme başlatılıyor ($TOTAL paket)..."
        if ! npm run tauri build -- --no-bundle --debug; then
            echo "❌ Hata: Derleme sırasında bir hata oluştu."
            exit 1
        fi
    fi
else
    # =====================================================
    # SUBSEQUENT LAUNCH: Rebuild silently unless error
    # =====================================================
    if ! npm run tauri build -- --no-bundle --debug >/dev/null 2>&1; then
        echo "⚠️  Hızlı derleme başarısız oldu! Terminal üzerinden yeniden derleniyor..."
        npm run tauri build -- --no-bundle --debug || exit 1
    fi
fi

# --- Launch the compiled binary directly (only port 4000, no vite dev server) ---
if [ ! -f "$BINARY" ]; then
    echo "❌ Hata: Derleme tamamlanamadı veya iptal edildi."
    echo "Lütfen internet bağlantınızı kontrol edip ./start.sh komutunu tekrar çalıştırın ve işlemin bitmesini bekleyin."
    exit 1
fi

echo "✨ Tman açılıyor..."
exec "$BINARY"
