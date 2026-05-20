Görev Listesi — Screen Translator

Öncelik Sırası: 1 → 2 → 3 → 4
Her öncelik, bir önceki tamamlanmadan başlanmamalıdır.


ÖNCELİK 1 — Temizlik & Derleme Hızı
Hedef: Tüm Python/sidecar bağımlılıklarını kökten temizle, derleme süresini dramatik ölçüde azalt.
Görevler

[]Sidecar klasörünü tamamen sil.
[]PaddleOCR, EasyOCR ve RapidOCR'a ait tüm Python betiklerini, binary dosyalarını ve ilgili yapılandırmaları kaldır. sidecars/ klasörü projede kalmamalı.
[]Rust backend'den sidecar bağımlılıklarını temizle.
[]Cargo.toml'dan sidecar'larla ilgili tüm dependencies satırlarını kaldır. src-tauri/ içindeki sidecar'ı çağıran her Rust kodu (IPC komutları dahil) temizlenmeli.
[]src/ frontend klasöründen OCR ile ilgili her şeyi çıkar.
[]OCR engine seçimi, PaddleOCR/EasyOCR ayarları gibi UI bileşenleri src/ içinden kaldırılmalı. Yalnızca aktif olarak kullanılan (native Rust OCR: leptess veya ocrs) seçeneği kalmalı.
[]Cargo.toml ve tauri.conf.json'u gözden geçir.
[]Artık kullanılmayan her dependency, feature flag ve bundle ayarını kaldırarak cold build süresini minimize et.
[] [ÖNERİ] Derleme süresini ölç ve kaydet.
[]Temizlik öncesi ve sonrası cargo build sürelerini (time cargo build) karşılaştır; kazanımı progress.md'ye yaz.
[] [ÖNERİ] cargo check CI adımı ekle.
[]Temizlik sonrası projenin sıfır uyarıyla derlendiğini doğrula; bu durumu progress.md'de "Known Issues" altında belgele.


ÖNCELİK 2 — Başlangıç Gecikmesi (3–4 sn)
Hedef: Uygulama açılışındaki gecikmenin kaynağını tespit et ve ortadan kaldır veya minimize et.
Görevler

[] Gecikme kaynağını profille.
Aşağıdaki olası nedenler araştırılmalı:

Axum sunucusunun tokio::spawn içinde senkron başlatılması
SQLite init_db çağrısının ana thread'i bloklaması
grim/slurp binary'lerinin uygulama açılışında tetiklenmesi
QR kodu üretiminin startup'ta çalışması
Tauri'nin setup() hook'unun ağır iş yapması


[]Axum sunucusu ve DB init'i lazy/async yap.
[]Kritik olmayan başlangıç işlemlerini (sunucu başlatma, DB migration) tokio::spawn + async ile arka plana taşı; UI ilk frame'i göstermeden önce bunları beklemesin.
[] [ÖNERİ] Splash screen veya skeleton UI ekle.
[]Gecikme tamamen çözülemese bile, kullanıcıya anlık geri bildirim vermek için Tauri'nin splashscreen özelliği veya basit bir loading animasyonu eklenebilir.
[] [ÖNERİ] Startup süresi log'a yazılsın.
[]std::time::Instant ile uygulama açılış süresi ölçülüp ~/.local/share/<app>/startup.log veya console'a yazılsın. Regression tespiti için yararlı olur.


ÖNCELİK 3 — Genel Performans İyileştirmeleri
Hedef: Çeviri döngüsünü ve kaynak kullanımını optimize et.
Görevler

[]reqwest client'ını yeniden kullan.
Her çeviri isteğinde yeni bir reqwest::Client oluşturmak yerine uygulama boyunca tek bir instance paylaşılsın (Arc<Client>).
[]SQLite cache kontrolünü iyileştir.
Aynı OCR çıktısı + dil kombinasyonu için önce cache'e bak, API'ye istek atma. Cache hit oranı log'a yazılsın.
[] [ÖNERİ] Görüntü ön işleme pipeline'ını benchmark et.
image crate ile yapılan grayscale/binarization/contrast işlemlerinin süresi ölçülsün; gerekirse paralel işleme (rayon) değerlendirilsin.
[] [ÖNERİ] WebSocket broadcast'ini debounce et.
Interval modunda çok sık tetiklenme durumunda, aynı metin için tekrar broadcast yapılmasın; önceki sonuçla karşılaştır.
[] [ÖNERİ] Bellek kullanımını izle.
Uzun süreli çalışmada memory leak olup olmadığını kontrol et (özellikle broadcaster channel'ı ve SQLite connection pool).


ÖNCELİK 4 — Dil Seçim UI'ı & Çoklu Dil Desteği
Hedef: Kaynak ve hedef dil seçimini, bayrak desteğiyle zenginleştirilmiş, kullanıcı dostu bir arayüze taşı.
Görevler
4.1 — Dil Eşleme Dosyası

[]src/lib/languages.ts (veya languages.json) dosyası oluşturulacak.
Her giriş şu alanları içermeli:

json  { "code": "tr", "name": "Türkçe", "flag": "🇹🇷" }
  { "code": "en", "name": "English", "flag": "🇬🇧" }
  { "code": "ja", "name": "日本語", "flag": "🇯🇵" }
Bu dosya tek source-of-truth olacak; hem kaynak hem hedef dil bileşenleri buradan beslenecek.

[] [ÖNERİ] Dosyaya rtl: true alanı eklensin (Arapça, Farsça vb. için), gelecekte metin yönü desteği sağlanabilsin.

4.2 — Kaynak Dil Bileşeni

[]"Otomatik" modu: Mevcut whichlang tabanlı otomatik algılama korunacak. Herhangi ek bir kullanıcı girişi gerekmez.
[]"Özel (Custom)" modu: "Otomatik" seçildiğinde kaynak dil alanı pasif kalır. "Özel" seçildiğinde şu bileşen açılır:
Bir metin input'u (dil kodu girilir, örn. ja)
Input'un solunda algılanan koda ait bayrak emoji'si gösterilir
Input'un sağında bir genişletilebilir panel butonu bulunur (▼ ikonu)


Genişletilebilir panel:
languages.ts dosyasındaki tüm diller alfabetik sırayla listelenir
Her satırda: [bayrak] [dil adı] ([kod])]Bir dile tıklanınca input otomatik dolar, panel kapanır



4.3 — Hedef Dil Bileşeni

[]Kaynak dilin "Özel" moduna benzer şekilde çalışır, ancak "Otomatik" seçeneği yoktur — her zaman custom input modundadır.
Metin input'u (dil kodu)
Solunda bayrak emoji'si
Sağında genişletilebilir panel (aynı languages.ts listesi)


[] [ÖNERİ] Son kullanılan diller öne çıkarılsın.
[]Panelin en üstünde "Son Kullanılanlar" başlığı altında en fazla 3 dil gösterilsin (SQLite veya config.toml'a kaydedilir).

4.4 — Uygulama Çoklu Dil Desteği (i18n)

[]Svelte için svelte-i18n veya basit bir $t() store sistemi kurulacak.
[]Başlangıç için en az 2 dil dosyası: en.json ve tr.json.
[]Dil seçimi config.toml'a kaydedilecek ve uygulama açılışında yüklenecek.
[] [ÖNERİ] Dil dosyalarını src/i18n/ altında tutun, her biri { "key": "Türkçe karşılık" } formatında olsun. Yeni dil eklemek tek bir JSON dosyası eklemeye indirgenmeli.


Notlar

Öncelik 1 tamamlanmadan Öncelik 2'ye geçilmemelidir; derleme ortamı sabit olmalı.
Öncelik 4'teki dil eşleme dosyası (languages.ts) mümkün olan en erken aşamada oluşturulmalı; hem Öncelik 3'teki cache key yapısını hem de Öncelik 4 UI'ını besleyecek.
progress.md her öncelik tamamlandığında güncellenmelidir.


4.5 — API Anahtarı Güvenliği

[] API anahtarları (config.toml'da plaintext) yerine sistem keyring'ine (secret-service / keyutils) taşınacak.
[]Rust tarafı için keyring crate'i (hwchen/keyring-rs) kullanılacak.
[]Tauri frontend'den anahtar kaydetme/okuma işlemleri mevcut save_settings IPC komutu üzerinden yönlendirilecek.
[ÖNERİ] İlk kurulumda keyring erişimi başarısız olursa (örn. gnome-keyring-daemon çalışmıyor — senin Koharu stack'inde de yaşadığın sorun) --no keyring flag ı devreye girecek ve api keyler sadece o anlık kullanılabilecek uygulama her kapandığında api keyde unutulacak.