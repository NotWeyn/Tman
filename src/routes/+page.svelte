<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  // Active Tab State
  let activeTab = 'yakalama';

  const icons = {
    camera: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/><circle cx="12" cy="13" r="3"/></svg>`,
    type: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 7 4 4 20 4 20 7"/><line x1="9" x2="15" y1="20" y2="20"/><line x1="12" x2="12" y1="4" y2="20"/></svg>`,
    languages: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m5 8 6 6"/><path d="m4 14 6-6 2-3"/><path d="M2 5h12"/><path d="M7 2h1"/><path d="m22 22-5-10-5 10"/><path d="M14 18h6"/></svg>`,
    smartphone: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="20" x="5" y="2" rx="2" ry="2"/><path d="M12 18h.01"/></svg>`,
    keyboard: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="16" x="2" y="4" rx="2" ry="2"/><path d="M6 8h.001"/><path d="M10 8h.001"/><path d="M14 8h.001"/><path d="M18 8h.001"/><path d="M8 12h.001"/><path d="M12 12h.001"/><path d="M16 12h.001"/><path d="M7 16h10"/></svg>`,
    clock: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>`,
    settings: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>`,
    monitorPlay: `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="14" x="2" y="3" rx="2"/><path d="M14 9.5 10 7v5z"/><line x1="12" x2="12" y1="17" y2="21"/><line x1="8" x2="16" y1="21" y2="21"/></svg>`
  };

  const tabs = [
    { id: 'yakalama', label: 'Yakalama', icon: icons.camera },
    { id: 'ocr', label: 'OCR', icon: icons.type },
    { id: 'ceviri', label: 'Çeviri', icon: icons.languages },
    { id: 'mobil', label: 'Mobil sunucu', icon: icons.smartphone },
    { id: 'kisayollar', label: 'Kısayollar', icon: icons.keyboard },
    { id: 'gecmis', label: 'Geçmiş', icon: icons.clock },
    { id: 'uygulama', label: 'Uygulama', icon: icons.settings }
  ];

  // --- STATE VARIABLES ---
  
  // 1. Yakalama
  let captureMode = 'manuel'; // manuel, interval, degisim
  let intervalSeconds = 3;
  let changeThreshold = 15;
  let grayscale = false;
  let binarization = false;
  let contrast = 'kapali';
  let scale = '2x';
  let showInGui = false;

  // 2. OCR
  let ocrEngine = 'leptess'; 
  let sourceLang = 'auto'; 
  let autoDetectLang = true;
  let mergeLines = true;
  let mergeParagraphs = false;
  let minCharThreshold = 5;
  let useGpu = false;
  let paddlePath = '';
  let easyPath = '';

  $: isSidecar = ocrEngine === 'paddle' || ocrEngine === 'easy';
  $: isAutoLang = sourceLang === 'auto';

  // 3. Çeviri
  let activeProvider = 'openai'; 
  let targetLang = 'tr';
  let cacheTranslations = true;
  let openaiEndpoint = '';
  let openaiKey = '';
  let openaiModel = '';
  let deeplKey = '';
  let googleKey = '';
  let libreUrl = '';

  // 4. Mobil Sunucu
  let serverActive = false;
  let serverPort = 7070;
  let serverAutoStart = false;
  let serverLocalOnly = false;
  let serverIp = '192.168.1.42'; // Mock IP for display

  // 6. Geçmiş
  let saveHistory = true;
  let maxHistory = 500;

  // 7. Uygulama
  let runOnStartup = false;
  let minimizeToTray = true;
  let logLevel = 'bilgi';

  let qrCodeBase64 = '';
  let isConfigLoaded = false;
  
  let lastRegion = '';
  let configPath = '';

  onMount(async () => {
    try {
      const config = await invoke('get_config');
      captureMode = config.capture_mode;
      intervalSeconds = config.capture_interval_sec;
      changeThreshold = config.capture_change_threshold;
      lastRegion = config.capture_last_region;
      grayscale = config.pre_grayscale;
      binarization = config.pre_binarize;
      contrast = config.pre_contrast;
      scale = config.pre_scale === 1.0 ? '1x' : config.pre_scale === 2.0 ? '2x' : '3x';
      
      ocrEngine = config.ocr_engine;
      paddlePath = config.ocr_paddle_path;
      easyPath = config.ocr_easy_path;
      useGpu = config.ocr_use_gpu;
      sourceLang = config.ocr_source_lang;
      autoDetectLang = config.ocr_auto_detect_lang;
      mergeLines = config.ocr_merge_lines;
      mergeParagraphs = config.ocr_merge_paragraphs;
      minCharThreshold = config.ocr_min_chars;
      
      activeProvider = config.trans_provider;
      targetLang = config.trans_target_lang;
      cacheTranslations = config.trans_cache_enabled;
      openaiEndpoint = config.trans_openai_endpoint;
      openaiModel = config.trans_openai_model;
      libreUrl = config.trans_libre_url;
      
      serverActive = config.server_enabled;
      serverPort = config.server_port;
      serverAutoStart = config.server_auto_start;
      serverLocalOnly = config.server_local_only;
      
      saveHistory = config.history_save;
      maxHistory = config.history_max_records;
      
      runOnStartup = config.app_start_on_boot;
      minimizeToTray = config.app_minimize_to_tray;
      logLevel = config.app_log_level;
      
      openaiKey = await invoke('get_secret', { key: 'openai_key' });
      deeplKey = await invoke('get_secret', { key: 'deepl_key' });
      googleKey = await invoke('get_secret', { key: 'google_key' });
      configPath = await invoke('get_config_path');
      
      if (serverActive) {
        await loadServerInfo();
      }
      isConfigLoaded = true;
    } catch (e) {
      console.error("Failed to load config:", e);
    }
  });

  async function loadServerInfo() {
    try {
      const info = await invoke('get_server_info');
      serverIp = info.ip;
      serverPort = info.port;
      qrCodeBase64 = info.qr_code_base64;
    } catch (e) {
      console.error("Failed to load server info:", e);
    }
  }

  async function saveConfig() {
    if (!isConfigLoaded) return;
    try {
      const scaleFloat = scale === '1x' ? 1.0 : scale === '2x' ? 2.0 : 3.0;
      await invoke('save_config', {
        newConfig: {
          server_enabled: serverActive,
          server_port: parseInt(serverPort),
          server_local_only: serverLocalOnly,
          server_auto_start: serverAutoStart,
          
          capture_mode: captureMode,
          capture_interval_sec: parseInt(intervalSeconds),
          capture_change_threshold: parseInt(changeThreshold),
          capture_last_region: lastRegion,
          
          pre_grayscale: grayscale,
          pre_binarize: binarization,
          pre_contrast: contrast,
          pre_scale: scaleFloat,
          
          ocr_engine: ocrEngine,
          ocr_paddle_path: paddlePath,
          ocr_easy_path: easyPath,
          ocr_rapid_path: "",
          ocr_use_gpu: useGpu,
          ocr_source_lang: sourceLang,
          ocr_auto_detect_lang: autoDetectLang,
          ocr_merge_lines: mergeLines,
          ocr_merge_paragraphs: mergeParagraphs,
          ocr_min_chars: parseInt(minCharThreshold),
          
          trans_provider: activeProvider,
          trans_target_lang: targetLang,
          trans_cache_enabled: cacheTranslations,
          trans_openai_endpoint: openaiEndpoint,
          trans_openai_model: openaiModel,
          trans_libre_url: libreUrl,
          
          history_save: saveHistory,
          history_max_records: parseInt(maxHistory),
          
          app_start_on_boot: runOnStartup,
          app_minimize_to_tray: minimizeToTray,
          app_log_level: logLevel
        }
      });
      await invoke('set_secret', { key: 'openai_key', secret: openaiKey });
      await invoke('set_secret', { key: 'deepl_key', secret: deeplKey });
      await invoke('set_secret', { key: 'google_key', secret: googleKey });
    } catch (e) {
      console.error("Failed to save config:", e);
    }
  }

  $: {
    let _deps = [
      captureMode, intervalSeconds, changeThreshold, lastRegion,
      grayscale, binarization, contrast, scale,
      ocrEngine, paddlePath, easyPath, useGpu, sourceLang, autoDetectLang, mergeLines, mergeParagraphs, minCharThreshold,
      activeProvider, targetLang, cacheTranslations, openaiEndpoint, openaiModel, libreUrl, openaiKey, deeplKey, googleKey,
      serverActive, serverPort, serverAutoStart, serverLocalOnly,
      saveHistory, maxHistory,
      runOnStartup, minimizeToTray, logLevel
    ];
    saveConfig();
  }

  async function handleServerToggle(e) {
    // Server toggle changed
    try {
      await invoke('toggle_server', { active: serverActive });
      if (serverActive) {
        await loadServerInfo();
      }
    } catch (e) {
      console.error("Failed to toggle server:", e);
    }
  }

  async function clearHistory() {
    if(confirm("Tüm çeviri geçmişini silmek istediğinize emin misiniz? Bu işlem geri alınamaz.")) {
      try {
        await invoke('clear_history');
        alert("Geçmiş temizlendi.");
      } catch (e) {
        console.error("Geçmiş silinemedi:", e);
        alert("Hata: " + e);
      }
    }
  }

  let exportFormat = 'JSON';

  async function exportHistory() {
    try {
      const history = await invoke('get_history');
      if (!history || history.length === 0) {
        alert("Geçmiş boş!");
        return;
      }
      
      let dataStr = '';
      let mimeType = '';
      let filename = '';

      if (exportFormat === 'JSON') {
        dataStr = JSON.stringify(history, null, 2);
        mimeType = 'application/json';
        filename = 'tman_history.json';
      } else if (exportFormat === 'CSV') {
        dataStr = 'id,original_text,translated_text,source_lang,target_lang,timestamp\n' + 
                  history.map(r => `${r.id},"${r.original_text.replace(/"/g, '""')}","${r.translated_text.replace(/"/g, '""')}","${r.source_lang}","${r.target_lang}","${r.timestamp}"`).join('\n');
        mimeType = 'text/csv';
        filename = 'tman_history.csv';
      } else { // TXT
        dataStr = history.map(r => `[${r.timestamp}] ${r.source_lang}->${r.target_lang}\nOriginal: ${r.original_text}\nTranslated: ${r.translated_text}\n`).join('\n---\n');
        mimeType = 'text/plain';
        filename = 'tman_history.txt';
      }

      const blob = new Blob([dataStr], { type: mimeType });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = filename;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      URL.revokeObjectURL(url);
    } catch (e) {
      console.error("Dışa aktarma hatası:", e);
      alert("Hata: " + e);
    }
  }

  let captureIntervalId = null;
  let isCapturingLoop = false;

  async function pickRegion() {
    try {
      const region = await invoke('pick_region');
      lastRegion = region;
    } catch (e) {
      console.error("Bölge seçme hatası:", e);
      alert("Bölge seçimi iptal edildi veya hata oluştu.");
    }
  }

  async function toggleCaptureLoop() {
    console.log("Çeviri döngüsü tetiklendi");
    
    if (captureMode === 'manuel') {
      try {
        await invoke('capture_and_translate');
      } catch (e) {
        if (e !== "No significant change") {
          console.error("Çeviri hatası:", e);
          alert("Hata: " + e);
        }
      }
    } else {
      if (isCapturingLoop) {
        clearInterval(captureIntervalId);
        isCapturingLoop = false;
        captureIntervalId = null;
      } else {
        isCapturingLoop = true;
        // Invoke once immediately
        invoke('capture_and_translate').catch(e => {
          if (e !== "No significant change") console.error(e);
        });
        
        let ms = parseInt(intervalSeconds) * 1000;
        captureIntervalId = setInterval(() => {
          invoke('capture_and_translate').catch(e => {
            if (e !== "No significant change") console.error(e);
          });
        }, ms);
      }
    }
  }

  let isInstalling = false;
  let installProgress = '';

  async function installEngine(engine) {
    if (isInstalling) return;
    isInstalling = true;
    installProgress = engine === 'paddle' ? 'PaddleOCR indiriliyor ve kuruluyor...' : 'EasyOCR indiriliyor ve kuruluyor...';
    
    console.log(`${engine} kurulumu simüle ediliyor...`);
    // TODO: Connect to Rust backend to run uv pip install rapidocr-onnxruntime / easyocr
    setTimeout(() => {
      if (engine === 'paddle') {
        paddlePath = './.venv/bin/paddleocr';
      } else {
        easyPath = './.venv/bin/easyocr';
      }
      isInstalling = false;
      installProgress = '';
    }, 2500);
  }
</script>

<div class="layout">
  <aside class="sidebar">
    <div class="sidebar-header" style="flex-direction: column; gap: 10px; align-items: stretch;">
      <span class="title text-center">AYARLAR</span>
      <button class="btn {captureMode === 'manuel' ? 'btn-primary' : (isCapturingLoop ? 'btn-danger' : 'btn-success')} w-100" on:click={toggleCaptureLoop}>
        {#if captureMode === 'manuel'}
          Çevir
        {:else}
          {isCapturingLoop ? 'Durdur' : 'Başlat'}
        {/if}
      </button>
    </div>
    <nav class="nav-menu">
      {#each tabs as tab}
        <button 
          class="nav-item {activeTab === tab.id ? 'active' : ''}" 
          on:click={() => activeTab = tab.id}
        >
          <div class="nav-icon-wrapper">{@html tab.icon}</div>
          <span>{tab.label}</span>
        </button>
      {/each}
    </nav>
  </aside>

  <main class="content-area">
    <div class="content-scroll">
      
      {#if activeTab === 'yakalama'}
        <div class="tab-content">
          <div class="action-header">
            <button class="btn btn-primary btn-large w-100" on:click={pickRegion}>
              {@html icons.camera}
              Ekranı Yakala
            </button>
            <div class="row" style="margin-top: 15px; flex-direction: column; align-items: stretch; gap: 5px;">
              <div class="label-desc text-center" style="opacity: 0.8; font-size: 0.85em;">Son yakalanan bölge koordinatları (Düzenleyebilirsiniz)</div>
              <input type="text" class="form-input code-font text-center" bind:value={lastRegion} placeholder="Örn: 1000,300 200x50" />
            </div>
          </div>

          <section class="section">
            <h3 class="section-title">YAKALAMA MODU</h3>
            
            <div class="row">
              <div class="row-info">
                <label>Mod</label>
                <div class="label-desc">Ekranın nasıl yakalanacağını seçin.</div>
              </div>
              <select bind:value={captureMode} class="form-select w-auto">
                <option value="manuel">Manuel</option>
                <option value="interval">Interval (Aralıklı)</option>
                <option value="degisim">Değişim algılama</option>
              </select>
            </div>

            {#if captureMode === 'interval' || captureMode === 'degisim'}
              <div class="row sub-row">
                <div class="row-info">
                  <label>Interval süresi</label>
                </div>
                <div class="input-with-unit">
                  <input type="number" bind:value={intervalSeconds} class="form-input number" min="1" max="60" />
                  <span class="unit">sn</span>
                </div>
              </div>
            {/if}

            {#if captureMode === 'degisim'}
              <div class="row sub-row">
                <div class="row-info">
                  <label>Değişim eşiği</label>
                  <div class="label-desc">Sadece bu oranın üzerinde görsel değişim olursa çevirir.</div>
                </div>
                <div class="input-with-unit">
                  <input type="number" bind:value={changeThreshold} class="form-input number" min="1" max="100" />
                  <span class="unit">%</span>
                </div>
              </div>
            {/if}
          </section>

          <section class="section">
            <h3 class="section-title">GÖRÜNTÜ ÖN İŞLEME</h3>
            
            <div class="row">
              <div class="row-info">
                <label>Gri tonlama (Grayscale)</label>
                <div class="label-desc">Görüntüyü siyah-beyaza çevirerek OCR doğruluğunu artırır.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={grayscale} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>

            <div class="row">
              <div class="row-info">
                <label>Binarization</label>
                <div class="label-desc">Gürültüyü azaltır, metni keskinleştirir.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={binarization} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>

            <div class="row">
              <div class="row-info">
                <label>Kontrast artırma</label>
              </div>
              <select bind:value={contrast} class="form-select w-auto">
                <option value="kapali">Kapalı</option>
                <option value="hafif">Hafif</option>
                <option value="guclu">Güçlü</option>
              </select>
            </div>

            <div class="row">
              <div class="row-info">
                <label>Ölçekleme</label>
                <div class="label-desc">Küçük fontları büyütmek için.</div>
              </div>
              <select bind:value={scale} class="form-select w-auto">
                <option value="1x">1x</option>
                <option value="2x">2x (Önerilen)</option>
                <option value="3x">3x</option>
              </select>
            </div>
          </section>

          <section class="section">
            <h3 class="section-title">ÇIKTI</h3>
            <div class="row">
              <div class="row-info">
                <label>GUI'de göster</label>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={showInGui} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
          </section>
        </div>
      {/if}

      {#if activeTab === 'ocr'}
        <div class="tab-content">
          <section class="section">
            <h3 class="section-title">MOTOR</h3>
            <div class="row">
              <div class="row-info">
                <label>OCR motoru</label>
              </div>
              <select bind:value={ocrEngine} class="form-select w-auto">
                <option value="leptess">Tesseract (leptess)</option>
                <option value="ocrs">ocrs (Rust native)</option>
                <option value="paddle">PaddleOCR (Sidecar)</option>
                <option value="easy">EasyOCR (Sidecar)</option>
              </select>
            </div>

            <div class="row">
              <div class="row-info">
                <label>Kaynak dil</label>
                <div class="label-desc">Beklenen metnin dili.</div>
              </div>
              <select bind:value={sourceLang} class="form-select w-auto">
                <option value="auto">Otomatik</option>
                <option value="en">İngilizce</option>
                <option value="jp">Japonca</option>
                <option value="kr">Korece</option>
                <option value="de">Almanca</option>
              </select>
            </div>

            <div class="row {isAutoLang ? '' : 'disabled'}">
              <div class="row-info">
                <label>Otomatik dil tespiti</label>
                <div class="label-desc">Whichlang ile offline dil tahmini. Sadece 'Otomatik' seçiliyken çalışır.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={autoDetectLang} disabled={!isAutoLang} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
          </section>

          <section class="section">
            <h3 class="section-title">TEXT POST-PROCESSING</h3>
            <div class="row">
              <div class="row-info">
                <label>Satır birleştirme</label>
                <div class="label-desc">Yanlış kesilmiş cümleleri tek satır yapar.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={mergeLines} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            <div class="row">
              <div class="row-info">
                <label>Paragraf birleştirme</label>
                <div class="label-desc">Dikey akan metin bloklarını birleştirir (Örn. Manga).</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={mergeParagraphs} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            <div class="row">
              <div class="row-info">
                <label>Minimum karakter eşiği</label>
                <div class="label-desc">Anlamsız tekil karakterleri yoksaymak için.</div>
              </div>
              <input type="number" bind:value={minCharThreshold} class="form-input number" />
            </div>
          </section>

          {#if isSidecar}
          <section class="section">
            <h3 class="section-title">SIDECAR AYARLARI</h3>
            <div class="row">
              <div class="row-info">
                <label>GPU kullan (CUDA)</label>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={useGpu} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            
            {#if ocrEngine === 'paddle'}
            <div class="row full-col">
              <label>PaddleOCR binary yolu</label>
              <div class="input-with-button w-100">
                <input type="text" bind:value={paddlePath} placeholder="Yüklü değilse indirebilirsiniz..." class="form-input" />
                {#if !paddlePath}
                  <button class="btn btn-primary" on:click={() => installEngine('paddle')} disabled={isInstalling}>
                    {isInstalling && installProgress.includes('Paddle') ? 'İndiriliyor...' : 'İndir & Kur'}
                  </button>
                {/if}
              </div>
            </div>
            {/if}
            
            {#if ocrEngine === 'easy'}
            <div class="row full-col">
              <label>EasyOCR binary yolu</label>
              <div class="input-with-button w-100">
                <input type="text" bind:value={easyPath} placeholder="Yüklü değilse indirebilirsiniz..." class="form-input" />
                {#if !easyPath}
                  <button class="btn btn-primary" on:click={() => installEngine('easy')} disabled={isInstalling}>
                    {isInstalling && installProgress.includes('Easy') ? 'İndiriliyor...' : 'İndir & Kur'}
                  </button>
                {/if}
              </div>
            </div>
            {/if}

            {#if isInstalling}
              <div class="install-status">{installProgress}</div>
            {/if}
          </section>
          {/if}
        </div>
      {/if}

      {#if activeTab === 'ceviri'}
        <div class="tab-content">
          <section class="section">
            <h3 class="section-title">PROVIDER</h3>
            <div class="row">
              <div class="row-info">
                <label>Aktif provider</label>
              </div>
              <select bind:value={activeProvider} class="form-select w-auto">
                <option value="openai">OpenAI-compatible</option>
                <option value="google">Google Translate</option>
                <option value="deepl">DeepL</option>
                <option value="libre">LibreTranslate (Local)</option>
              </select>
            </div>

            <div class="row">
              <div class="row-info">
                <label>Hedef dil</label>
              </div>
              <select bind:value={targetLang} class="form-select w-auto">
                <option value="tr">Türkçe</option>
                <option value="en">İngilizce</option>
                <option value="de">Almanca</option>
                <option value="fr">Fransızca</option>
                <option value="es">İspanyolca</option>
              </select>
            </div>

            <div class="row">
              <div class="row-info">
                <label>Önbellekleme</label>
                <div class="label-desc">Aynı metni tekrar çevirirken API yerine DB'den getirir.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={cacheTranslations} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
          </section>

          <!-- Provider Specific Settings -->
          {#if activeProvider === 'openai'}
          <section class="section">
            <h3 class="section-title">LOCAL Aİ AYARLARI</h3>
            <div class="row full-col">
              <label>API endpoint</label>
              <input type="text" bind:value={openaiEndpoint} placeholder="http://localhost:5000" class="form-input" />
            </div>
            <div class="row full-col">
              <label>API anahtarı</label>
              <input type="password" bind:value={openaiKey} placeholder="sk-..." class="form-input" />
            </div>
            <div class="row full-col">
              <label>Model</label>
              <input type="text" bind:value={openaiModel} placeholder="local-model" class="form-input" />
            </div>
          </section>
          {/if}

          {#if activeProvider === 'deepl'}
          <section class="section">
            <h3 class="section-title">DEEPL AYARLARI</h3>
            <div class="row full-col">
              <label>API anahtarı</label>
              <input type="password" bind:value={deeplKey} placeholder="Auth key..." class="form-input" />
            </div>
          </section>
          {/if}

          {#if activeProvider === 'google'}
          <section class="section">
            <h3 class="section-title">GOOGLE TRANSLATE AYARLARI</h3>
            <div class="row full-col">
              <label>API anahtarı (Opsiyonel)</label>
              <input type="password" bind:value={googleKey} placeholder="Key..." class="form-input" />
            </div>
          </section>
          {/if}

          {#if activeProvider === 'libre'}
          <section class="section">
            <h3 class="section-title">LIBRETRANSLATE AYARLARI</h3>
            <div class="row full-col">
              <label>Sunucu URL'i</label>
              <input type="text" bind:value={libreUrl} placeholder="http://localhost:5000" class="form-input" />
            </div>
          </section>
          {/if}
        </div>
      {/if}

      {#if activeTab === 'mobil'}
        <div class="tab-content">
          <div class="server-card">
            <div class="server-info">
              <div class="server-icon">{@html icons.monitorPlay}</div>
              <div>
                <h2 class="server-title">Yerel web sunucu</h2>
                <div class="server-url">{serverActive ? `http://${serverIp}:${serverPort}` : 'Bağlantı bekleniyor...'}</div>
              </div>
            </div>
            <div class="server-actions">
              <div class="badge {serverActive ? 'badge-active' : 'badge-inactive'}">
                {serverActive ? 'Aktif' : 'Kapalı'}
              </div>
              <label class="toggle-wrapper server-toggle">
                <input type="checkbox" bind:checked={serverActive} on:change={handleServerToggle} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
          </div>

          {#if serverActive}
            <div class="qr-container">
              <div class="qr-placeholder">
                <div class="qr-box">
                  {#if qrCodeBase64}
                    <img src="{qrCodeBase64}" alt="QR Code" style="width: 150px; height: 150px; image-rendering: pixelated;" />
                  {:else}
                    <svg width="150" height="150" viewBox="0 0 100 100">
                      <rect width="100" height="100" fill="white" />
                      <text x="50" y="50" dominant-baseline="middle" text-anchor="middle" font-size="10" fill="black">Yükleniyor...</text>
                    </svg>
                  {/if}
                </div>
                <p>Kameranızla okutarak bağlanın</p>
              </div>
            </div>
          {/if}

          <section class="section">
            <h3 class="section-title">SUNUCU AYARLARI</h3>
            <div class="row">
              <div class="row-info">
                <label>Port</label>
              </div>
              <input type="number" bind:value={serverPort} class="form-input number" />
            </div>
            <div class="row">
              <div class="row-info">
                <label>Uygulama açılışında başlat</label>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={serverAutoStart} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            <div class="row">
              <div class="row-info">
                <label>Sadece yerel ağ (127.0.0.1)</label>
                <div class="label-desc">Kapalıysa ağdaki diğer cihazlar (telefon) erişebilir.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={serverLocalOnly} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
          </section>
        </div>
      {/if}

      {#if activeTab === 'kisayollar'}
        <div class="tab-content">
          <section class="section">
            <h3 class="section-title">HYPRLAND KISAYOLLARI</h3>
            <p class="section-desc">Aşağıdaki satırları <code>hyprland.conf</code> dosyanıza yapıştırarak uygulamayı global kısayollarla yönetebilirsiniz.</p>
            
            <div class="code-block">
<pre><code>bind = $mainMod, T, exec, screen-translator --capture
bind = $mainMod, R, exec, screen-translator --repeat
bind = $mainMod, I, exec, screen-translator --toggle-interval</code></pre>
            </div>
            <p class="note">Not: Uygulamanın binary adını değiştirdiyseniz veya PATH'te değilse komutları güncellemeyi unutmayın.</p>

            <div class="shortcuts-list">
              <div class="shortcut-item">
                <span class="action-name">Bölge seç ve çevir</span>
                <kbd>$mod + T</kbd>
              </div>
              <div class="shortcut-item">
                <span class="action-name">Son bölgeyi tekrar çevir</span>
                <kbd>$mod + R</kbd>
              </div>
              <div class="shortcut-item">
                <span class="action-name">Interval modu aç/kapa</span>
                <kbd>$mod + I</kbd>
              </div>
              <div class="shortcut-item">
                <span class="action-name">Çeviriyi panoya kopyala</span>
                <kbd>$mod + C</kbd>
              </div>
            </div>
          </section>
        </div>
      {/if}

      {#if activeTab === 'gecmis'}
        <div class="tab-content">
          <section class="section">
            <h3 class="section-title">GEÇMİŞ AYARLARI</h3>
            <div class="row">
              <div class="row-info">
                <label>Geçmişi kaydet</label>
                <div class="label-desc">Çeviriler SQLite veritabanına yazılır.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={saveHistory} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            
            <div class="row">
              <div class="row-info">
                <label>Maksimum kayıt</label>
                <div class="label-desc">Aşıldığında en eski kayıtlar silinir.</div>
              </div>
              <input type="number" bind:value={maxHistory} class="form-input number" />
            </div>

            <div class="row">
              <div class="row-info">
                <label>Önbellekleme</label>
                <div class="label-desc">Aynı metni tekrar çevirirken API yerine DB'den getirir.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={cacheTranslations} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>

            <div class="row">
              <div class="row-info">
                <label>Geçmişi temizle</label>
                <div class="label-desc">Tüm kayıtları veritabanından kalıcı olarak siler.</div>
              </div>
              <button class="btn btn-danger" on:click={clearHistory}>Geçmişi Temizle</button>
            </div>
          </section>

          <section class="section">
            <h3 class="section-title">DIŞA AKTARMA</h3>
            <div class="row">
              <div class="row-info">
                <label>Format ve Aktarım</label>
              </div>
              <div class="export-actions">
                <select bind:value={exportFormat} class="form-select w-auto">
                  <option value="JSON">JSON</option>
                  <option value="CSV">CSV</option>
                  <option value="TXT">TXT</option>
                </select>
                <button class="btn btn-primary" on:click={exportHistory}>Dışa aktar</button>
              </div>
            </div>
          </section>
        </div>
      {/if}

      {#if activeTab === 'uygulama'}
        <div class="tab-content">
          <section class="section">
            <h3 class="section-title">GENEL</h3>
            <div class="row">
              <div class="row-info">
                <label>Sistem başlangıcında çalıştır</label>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={runOnStartup} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            
            <div class="row">
              <div class="row-info">
                <label>Sistem tepsisine küçült</label>
                <div class="label-desc">Pencere kapatılınca uygulamayı açık tutar.</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={minimizeToTray} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>

            <div class="row">
              <div class="row-info">
                <label>Log seviyesi</label>
              </div>
              <select bind:value={logLevel} class="form-select w-auto">
                <option value="hata">Hata</option>
                <option value="bilgi">Bilgi</option>
                <option value="hata_ayiklama">Hata ayıklama (Debug)</option>
              </select>
            </div>

            <div class="row full-col">
              <div class="row-info" style="margin-bottom: 5px;">
                <label>Config dosyası konumu</label>
                <div class="label-desc">Değiştirip "Uygula" butonuna basarak config dosyasını taşıyabilirsiniz. Varsayılan için "config.json" yapın.</div>
              </div>
              <div style="display: flex; gap: 8px;">
                <input type="text" bind:value={configPath} class="form-input code-font" style="flex: 1;" />
                <button class="btn btn-primary" on:click={async () => {
                  try {
                    await invoke('set_config_path', { newPath: configPath });
                    alert("Config yolu güncellendi ve ayarlar buraya kopyalandı!");
                  } catch (e) {
                    alert("Hata: " + e);
                  }
                }}>Uygula</button>
              </div>
            </div>
          </section>

          <section class="section">
            <h3 class="section-title">GÜNCELLEMELER</h3>
            <div class="row">
              <div class="row-info">
                <label>Sürüm</label>
              </div>
              <div class="badge badge-version">v0.1.0</div>
            </div>
            <div class="row">
              <div class="row-info">
                <label>Güncellemeleri kontrol et</label>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
          </section>
        </div>
      {/if}

    </div>
  </main>
</div>

<style>
  .layout {
    display: flex;
    width: 100%;
    height: 100vh;
    background-color: var(--bg-main);
  }

  .sidebar {
    width: 240px;
    flex-shrink: 0;
    background-color: var(--bg-sidebar);
    border-right: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    padding: 1.5rem 0;
  }

  .sidebar-header {
    padding: 0 1.5rem 1rem;
  }

  .sidebar-header .title {
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .nav-menu {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0 0.75rem;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    padding: 0.625rem 0.75rem;
    background: transparent;
    border: 1px solid transparent;
    color: var(--text-secondary);
    font-size: 0.875rem;
    font-weight: 500;
    text-align: left;
    border-radius: var(--radius-md);
    cursor: pointer;
    transition: var(--transition);
  }

  .nav-item:hover {
    background-color: var(--bg-surface);
    color: var(--text-primary);
  }

  .nav-item.active {
    background-color: white;
    color: #000;
    border-color: rgba(255, 255, 255, 0.9);
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
  }

  .nav-item.active :global(svg) {
    color: #000;
  }

  .content-area {
    flex: 1;
    overflow: hidden;
    position: relative;
  }

  .content-scroll {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    overflow-y: auto;
    padding: 2rem 3rem;
  }

  .tab-content {
    max-width: 600px;
    animation: fadeIn 0.3s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(5px); }
    to { opacity: 1; transform: translateY(0); }
  }

  .section {
    margin-bottom: 2.5rem;
  }

  .section.disabled {
    opacity: 0.5;
    pointer-events: none;
    filter: grayscale(1);
  }

  .section-title {
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  .section-desc {
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    min-height: 40px;
  }

  .row.sub-row {
    padding-left: 1.5rem;
    border-left: 2px solid var(--border-color);
    margin-left: 0.5rem;
  }

  .row.full-col {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .row-info {
    flex: 1;
    padding-right: 2rem;
  }

  .w-auto {
    width: auto;
    min-width: 140px;
  }

  .input-with-unit {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .unit {
    font-size: 0.875rem;
    color: var(--text-muted);
  }

  /* Server Card */
  .server-card {
    background: linear-gradient(145deg, var(--bg-surface), #1a1a24);
    border: 1px solid var(--border-active);
    border-radius: var(--radius-lg);
    padding: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    box-shadow: var(--shadow-lg);
  }

  .server-info {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .server-icon {
    width: 48px;
    height: 48px;
    border-radius: var(--radius-md);
    background: rgba(99, 102, 241, 0.1);
    color: var(--accent-color);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .server-title {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
  }

  .server-url {
    font-size: 0.875rem;
    color: var(--accent-hover);
    font-family: monospace;
  }

  .server-actions {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .badge {
    padding: 0.25rem 0.625rem;
    border-radius: 999px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  .badge-active {
    background: rgba(16, 185, 129, 0.1);
    color: var(--success-color);
    border: 1px solid rgba(16, 185, 129, 0.2);
  }

  .badge-inactive {
    background: rgba(113, 113, 122, 0.1);
    color: var(--text-muted);
    border: 1px solid rgba(113, 113, 122, 0.2);
  }

  .badge-version {
    background: var(--bg-surface-hover);
    color: var(--text-primary);
  }

  .server-toggle .toggle-bg {
    width: 48px;
    height: 26px;
  }
  .server-toggle .toggle-dot {
    width: 20px;
    height: 20px;
  }
  .server-toggle .toggle-input:checked + .toggle-bg .toggle-dot {
    transform: translateX(22px);
  }

  .qr-container {
    display: flex;
    justify-content: center;
    margin-bottom: 2rem;
  }

  .qr-placeholder {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .qr-box {
    background: white;
    padding: 1rem;
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-glow);
  }

  .qr-placeholder p {
    font-size: 0.875rem;
  }

  /* Shortcuts */
  .code-block {
    background: #000;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    padding: 1rem;
    margin-bottom: 0.75rem;
    overflow-x: auto;
  }
  
  .code-block pre {
    margin: 0;
    color: #e2e8f0;
    font-family: 'Courier New', Courier, monospace;
    font-size: 0.875rem;
  }

  .note {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-bottom: 2rem;
  }

  .shortcuts-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .shortcut-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-surface);
    padding: 0.75rem 1rem;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
  }

  .action-name {
    font-size: 0.875rem;
    font-weight: 500;
  }

  kbd {
    background: var(--bg-sidebar);
    border: 1px solid var(--border-active);
    border-radius: var(--radius-sm);
    padding: 0.25rem 0.5rem;
    font-family: monospace;
    font-size: 0.75rem;
    color: var(--accent-hover);
  }

  .export-actions {
    display: flex;
    gap: 0.5rem;
  }

  .code-font {
    font-family: 'Courier New', Courier, monospace;
    color: var(--text-muted);
  }

  .action-header {
    margin-bottom: 2.5rem;
  }

  .btn-large {
    padding: 1rem 2rem;
    font-size: 1.1rem;
    font-weight: 600;
    justify-content: center;
  }

  .w-100 {
    width: 100%;
  }

  .input-with-button {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .install-status {
    font-size: 0.875rem;
    color: var(--accent-color);
    margin-top: 0.5rem;
    font-weight: 500;
  }
</style>
