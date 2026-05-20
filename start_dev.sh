#!/bin/bash
# start_dev.sh - Launch script for Wayland Screen Translator (Tman) - Developer Debug Build

set -e

# Ensure we are in the project directory regardless of where the script is called from
cd "$(dirname "$0")"

# --- Wayland / WebKitGTK Compatibility Fixes ---
export WEBKIT_DISABLE_DMABUF_RENDERER=1
unset KITTY_WINDOW_ID 2>/dev/null || true

# --- Parameter Parsing ---
REBUILD=false
for arg in "$@"; do
    if [ "$arg" = "-r" ] || [ "$arg" = "--rebuild" ]; then
        REBUILD=true
    fi
done

BINARY="src-tauri/target/debug/tman"

# --- Rebuild or Launch ---
if [ ! -f "$BINARY" ] || [ "$REBUILD" = true ]; then
    # --- Node dependencies setup ---
    if [ ! -d "node_modules" ]; then
        echo "📦 Node paketleri yükleniyor..."
        npm install >/dev/null 2>&1 || true
    fi

    # Auto-detect total package count from cargo metadata
    TOTAL=$(cd src-tauri && cargo metadata --format-version 1 2>/dev/null \
        | python3 -c "import sys,json; print(len(json.load(sys.stdin).get('packages',[])))" \
        2>/dev/null || echo "700")

    if command -v zenity &>/dev/null; then
        echo "🔨 Hata ayıklama (debug) derlemesi başlatılıyor (Bu işlem birkaç dakika sürebilir)..."
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
            --title="Tman - Hata Ayıklama Derlemesi" \
            --text="Debug sürümü derleniyor..." \
            --percentage=0 \
            --auto-close \
            --width=500 \
            --no-cancel
    else
        echo "🔨 Hata ayıklama (debug) derlemesi başlatılıyor ($TOTAL paket)..."
        if ! npm run tauri build -- --no-bundle --debug; then
            echo "❌ Hata: Derleme sırasında bir hata oluştu."
            exit 1
        fi
    fi
else
    echo "✨ Tman zaten derlenmiş, hata ayıklama moduyla doğrudan başlatılıyor..."
    echo "💡 Not: Yeniden derlemeye zorlamak için komutu şöyle çalıştırabilirsiniz: ./start_dev.sh -r"
fi

# --- Launch the compiled binary directly (only port 4000, no vite dev server) ---
if [ ! -f "$BINARY" ]; then
    echo "❌ Hata: Derleme tamamlanamadı ve/veya iptal edildi."
    echo "Lütfen internet bağlantınızı kontrol edip ./start_dev.sh -r komutunu çalıştırın."
    exit 1
fi

echo "✨ Tman (Debug) açılıyor..."
exec "$BINARY"
