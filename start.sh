#!/bin/bash
# start.sh - Launch script for Wayland Screen Translator (Tman) - Production Release

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

BINARY="src-tauri/target/release/tman"

# --- Rebuild or Launch ---
if [ ! -f "$BINARY" ] || [ "$REBUILD" = true ]; then
    # --- Node dependencies setup ---
    if [ ! -d "node_modules" ]; then
        echo "📦 Node paketleri yükleniyor..."
        npm install >/dev/null 2>&1 || true
    fi

    # Auto-detect package count for progress estimate
    TOTAL=$(cd src-tauri && cargo metadata --format-version 1 2>/dev/null \
        | python3 -c "import sys,json; print(len(json.load(sys.stdin).get('packages',[])))" \
        2>/dev/null || echo "700")

    if command -v zenity &>/dev/null; then
        echo "🔨 Üretim sürümü derlemesi başlatılıyor (Optimizasyonlar aktif, birkaç dakika sürebilir)..."
        (npm run tauri build -- --no-bundle 2>&1 || true) | tee /dev/stderr | {
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
            --title="Tman - Üretim Sürümü Derleme" \
            --text="Üretim sürümü derleniyor..." \
            --percentage=0 \
            --auto-close \
            --width=500 \
            --no-cancel
    else
        echo "🔨 Üretim sürümü derlemesi başlatılıyor ($TOTAL paket, optimizasyonlar aktif)..."
        if ! npm run tauri build -- --no-bundle; then
            echo "❌ Hata: Derleme sırasında bir hata oluştu."
            exit 1
        fi
    fi
else
    echo "✨ Tman zaten derlenmiş, doğrudan başlatılıyor..."
    echo "💡 Not: Yeniden derlemeye zorlamak için komutu şöyle çalıştırabilirsiniz: ./start.sh -r"
fi

if [ ! -f "$BINARY" ]; then
    echo "❌ Hata: Derleme tamamlanamadı."
    exit 1
fi

echo "✨ Tman açılıyor..."
exec "$BINARY"
