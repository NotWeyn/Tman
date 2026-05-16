# Tman 🚀

Tman, modern ve hızlı bir ekran çeviri uygulamasıdır. Tauri ve Svelte kullanılarak geliştirilmiştir.

## Özellikler ✨

- **Ekran Yakalama**: `slurp` ve `grim` kullanarak hızlı alan seçimi ve yakalama.
- **OCR (Optik Karakter Tanıma)**: Ekrandaki metinleri yüksek doğrulukla tanıma.
- **Anlık Çeviri**: Yakalanan metni anında istediğiniz dile çevirme.
- **Geçmiş**: Daha önce yapılan çevirileri SQLite tabanlı yerel veritabanında saklama.
- **Uzaktan İzleme**: Axum tabanlı dahili web sunucusu ile çevirileri yerel ağdaki diğer cihazlardan takip etme.
- **Modern Arayüz**: Svelte ile geliştirilmiş, hızlı ve şık kullanıcı arayüzü.

## Teknoloji Yığını 🛠️

- **Backend**: Rust (Tauri, Axum, SQLite)
- **Frontend**: Svelte (Vite)
- **OCR**: Python (RapidOCR) / Rust (Native OCR)
- **Sistem Araçları**: Grim, Slurp

## Kurulum 📦

Projeyi yerelinizde çalıştırmak için:

1. Bağımlılıkları yükleyin:
   ```bash
   npm install
   ```

2. Projeyi başlatın:
   ```bash
   npm run tauri build
   ```

## Katkıda Bulunma 🤝

Katkılarınızı bekliyoruz! Lütfen [CONTRIBUTING.md](CONTRIBUTING.md) dosyasını inceleyin.

## Lisans 📄

Bu proje [MIT Lisansı](LICENSE) altında lisanslanmıştır.
