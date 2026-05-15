# Screen Translator — Proje Spesifikasyonu

## Genel Bakış

Hyprland gibi Wayland tabanlı pencere yöneticilerinde overlay mekanizması güvenlik protokolleri nedeniyle X11'deki kadar esnek çalışmadığından, ekranda görünen metni çevirmek için uygun araç yok. Bu proje bu sorunu **Screen Capture → OCR → Çeviri → Yönlendirme** pipeline'ı ile çözer.

Kullanıcı uygulamayı açar, çevrilecek ekran bölgesini seçer ve çeviri sonucu:
- Tauri GUI'ye, **veya**
- Yerel ağda açılan bir web sitesine (telefon gibi ikinci bir ekrandan takip için)

anlık olarak iletilir.

---

## Teknoloji Yığını

| Katman | Teknoloji | Gerekçe |
|---|---|---|
| GUI / Ayarlar | Tauri v2 | Hafif WebView tabanlı ayar arayüzü, Rust gücünü kullanır |
| Backend dili | Rust | Hız ve düşük kaynak tüketimi |
| Async runtime | `tokio` | Asenkron işlemler |
| HTTP + WebSocket server | `axum` | Sunucu bu crate ile açılacak |
| Ekran yakalama | `grim` + `slurp` | Wayland/Hyprland standardı |
| OCR | Birden fazla seçenek (aşağıda) | Sidecar mimarisi |
| Çeviri | Birden fazla provider (aşağıda) | Provider abstraction katmanı |
| HTTP istekleri | `reqwest` | Çeviri API çağrıları |
| Seri hale getirme | `serde` / `serde_json` | Config ve IPC |
| Görüntü işleme | `image` crate | Screenshot ön işleme |
| Dil tespiti | `whichlang` | Pure Rust, offline |
| Çeviri geçmişi | `sqlx` (SQLite) | Yerel saklama ve önbellekleme |
| QR kod üretimi | `qrcode` crate | Web server URL'ini QR'a çevir |
| Config dosyası | `dirs` crate | XDG uyumlu yol yönetimi |
| Frontend framework | Svelte veya SolidJS | Tauri frontend |

---

## OCR Seçenekleri (Sidecar Mimarisi)

Kullanıcı aşağıdaki OCR motorlarından birini seçebilir:

- `leptess` veya `tesseract-rs` — Tesseract wrapper (Rust-native)
- `ocrs` — Rust tabanlı alternatif
- PaddleOCR — Sidecar olarak çalıştırılır
- EasyOCR — Sidecar olarak çalıştırılır

> **Not:** PaddleOCR ve EasyOCR Python tabanlıdır. Bunlar Tauri'nin **sidecar** mekanizmasıyla (ana binary'nin yanında çalışan yardımcı programlar olarak) entegre edilecek.

---

## Çeviri Provider Seçenekleri

- OpenAI-compatible API (özel endpoint desteği dahil)
- Google Translate API
- DeepL API
- (Genişletilebilir: yeni provider eklemek kolay olmalı)

---

## Mimari

Sistem üç ana katmandan oluşur:

### A. Core Engine — Gateway (Rust Backend)

Tüm iş mantığı burada çalışır. Frontend ve Backend birbirinden ayrıdır; frontend ayarları Gateway'e bildirir, Gateway sonuçları dağıtır.

**Modüller:**

- **Capture Module:** `slurp` ile kullanıcıdan koordinat alır, `grim` ile bölgeyi kırpar. Shell-out: `std::process::Command::new("grim")` — en temiz yol. Alternatif: `wayland-client` + `wlr-screencopy-unstable-v1` protokolü (daha karmaşık, gerekirse değerlendirilebilir).
- **OCR Pipeline:** Görüntüyü seçili OCR motoruna gönderir. Önce görüntü ön işleme yapılır (bkz. aşağı).
- **Translation Manager:** `reqwest` ile seçili çeviri API'sine istek atar.
- **Event Bus:** Sonucu hem Tauri frontend'ine hem de Local Web Server'a aynı anda yayınlar (`broadcaster.rs`).

### B. Local Web Server (Mobile Link)

- `axum` ile `tokio` thread içinde çalışır.
- Kullanıcı Tauri UI'daki butonu açtığında `0.0.0.0:<port>` üzerinden yayına başlar; buton kapalıyken server çalışmaz.
- Statik HTML/JS sayfası (`web-ui/index.html`) sunar; bu sayfa WebSocket'e bağlı kalır ve yeni çeviri geldiği an ekranda belirir.
- Server açıldığında `qrcode` crate ile yerel IP'yi QR koda çevirir; Tauri UI'da gösterilir (telefondan URL yazmadan bağlanmak için).

### C. Control UI (Tauri Frontend)

**Sadece ayarlar arayüzü.** Overlay yapmaz.

İçerir:
- Ekran yakalama modu ve hız ayarları
- OCR motoru seçimi
- Dil ayarları (kaynak dil: Otomatik dahil)
- Çeviri provider seçimi ve API anahtarı yönetimi
- Çeviri geçmişi görünümü
- Local web server aç/kapa butonu
- Web server açıkken QR kodu gösterimi

---

## Proje Dosya Yapısı

```
proje/
├── src-tauri/               # Tauri + Rust gateway
│   ├── src/
│   │   ├── main.rs
│   │   ├── capture.rs       # grim/slurp entegrasyonu
│   │   ├── ocr.rs           # OCR motoru abstraction
│   │   ├── translate.rs     # Provider abstraction
│   │   ├── server.rs        # Axum web server
│   │   ├── broadcaster.rs   # WebSocket yönetimi
│   │   └── config.rs        # Ayar yönetimi
│   └── Cargo.toml
├── src/                     # Tauri frontend (Svelte / SolidJS)
│   ├── App.svelte
│   ├── pages/
│   │   ├── Settings.svelte
│   │   ├── Capture.svelte
│   │   └── History.svelte
│   └── lib/
└── web-ui/                  # Yerel ağ web sitesi (statik HTML)
    └── index.html           # WebSocket istemcisi
```

---

## Config Dosyası

Konum: `~/.config/<uygulama-adı>/config.toml`

`dirs` crate ile XDG uyumlu path yönetimi yapılacak.

---

## Yakalama Modları

1. **Manuel:** Butona bas, bir kere yakala.
2. **Interval:** Her X saniyede bir otomatik yakala.
3. **Değişim algılama:** Önceki frame ile karşılaştır; fark varsa çevir. Aynıysa API çağrısı yapma (gereksiz maliyeti önler).

---

## Görüntü Ön İşleme (OCR Kalitesi İyileştirme)

OCR'a göndermeden önce `image` crate ile:

- Gri tonlamaya çevir (grayscale)
- Binarization (siyah-beyaz) uygula
- Kontrast artır

Bu işlemler OCR doğruluğunu yaklaşık %40 artırır ve Tesseract gibi motorları bile çok daha etkili hale getirir.

---

## Text Post-processing (OCR Çıktı Düzeltme)

OCR motorları genellikle satır sonlarını yanlış keser ve paragraf akışını bozar. Gelişmiş bir metin birleştirme modülü gereklidir:

- Yanlış kesilen satırları birleştir.
- Dikey olarak akan paragrafları doğru şekilde tek bir metin bloğuna dönüştür.
- Birleştirme mantığı, satır yüksekliği ve boşluk analizi ile yönlendirilmeli.

---

## Otomatik Dil Tespiti

- `whichlang` crate (pure Rust, offline) ile OCR çıktısının dili otomatik algılanır.
- Tauri UI'da "Kaynak dil: Otomatik" seçeneği sunulur.

---

## Çeviri Geçmişi ve Önbellekleme

- `sqlx` crate ile SQLite veritabanında saklanır.
- Aynı metin daha önce çevrildiyse önbellekten döndürülür; API çağrısı yapılmaz.
- Tauri UI'da geçmiş sekmesinde listelenir.

---

## Ek Özellikler

### Hotkey Desteği
Hyprland `hyprland.conf` dosyasına bir binding eklenerek uygulama tetiklenebilir:
```
bind = $mainMod, S, exec, <uygulama-adı> --capture
```

### Pano (Clipboard) Senkronizasyonu
Çevrilen metni opsiyonel olarak panoya kopyalama özelliği.

### QR Kod ile Hızlı Bağlantı
Web server açıldığında yerel IP QR koda dönüştürülür ve Tauri UI'da gösterilir. Telefon veya tablet bu QR'ı okuyarak doğrudan bağlanabilir.

---

## Özet: Veri Akışı

```
Kullanıcı bölge seçer (slurp)
       ↓
Ekran görüntüsü alınır (grim)
       ↓
Görüntü ön işleme (image crate)
       ↓
OCR (seçili motor / sidecar)
       ↓
Text post-processing (satır birleştirme)
       ↓
Dil tespiti (whichlang)
       ↓
Çeviri (seçili provider / reqwest)
       ↓
Event Bus (broadcaster.rs)
      / \
     /   \
Tauri UI  WebSocket → Web UI (telefon)
```
