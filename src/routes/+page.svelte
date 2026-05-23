<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import LanguagePicker from '$lib/LanguagePicker.svelte';
  import { t, locale, setLocale } from '$lib/i18n';

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

  $: tabs = [
    { id: 'yakalama', label: $t('tabs.capture'), icon: icons.camera },
    { id: 'ocr', label: $t('tabs.ocr'), icon: icons.type },
    { id: 'ceviri', label: $t('tabs.translate'), icon: icons.languages },
    { id: 'mobil', label: $t('tabs.server'), icon: icons.smartphone },
    { id: 'kisayollar', label: $t('tabs.shortcuts'), icon: icons.keyboard },
    { id: 'gecmis', label: $t('tabs.history'), icon: icons.clock },
    { id: 'uygulama', label: $t('tabs.app'), icon: icons.settings }
  ];

  // --- OVERLAY STATE ---
  let overlayVisible = false;
  let overlayTitle = '';
  let overlayMessage = '';
  let overlayType = 'error';

  /**
   * @param {string} msg
   * @param {string} title
   * @param {string} type
   */
  function showOverlay(msg, title = 'HATA', type = 'error') {
    let highlightedMsg = msg.replace(/(Hata:|Warning:|Error:|Failed to|Bölge|Geçmiş)/gi, '<strong class="hl-keyword">$1</strong>');
    highlightedMsg = highlightedMsg.replace(/'([^']+)'/g, '<strong class="hl-quote">\'$1\'</strong>');
    
    overlayMessage = highlightedMsg;
    overlayTitle = title;
    overlayType = type;
    overlayVisible = true;
  }

  function closeOverlay() {
    overlayVisible = false;
  }

  // --- STATE VARIABLES ---
  
  // 1. Capture
  let captureMode = 'manual'; // manual, interval, change
  let intervalSeconds = 3;
  let changeThreshold = 15;
  let grayscale = false;
  let binarization = false;
  let contrast = 'off';
  let scale = '2x';
  let showInGui = false;

  // 2. OCR
  let sourceLang = 'auto'; 
  let autoDetectLang = true;
  let mergeLines = true;
  let mergeParagraphs = false;
  let minCharThreshold = 5;

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
  let logLevel = 'info';
  let appLang = 'en';

  // Sync i18n locale when appLang changes
  $: setLocale(appLang);

  // Update state
  let updateChecking = false;
  let updateAvailable = false;
  let updateVersion = '';
  let updateBody = '';
  let updateInstalling = false;
  let appVersion = '0.1.0';

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
      
      logLevel = config.app_log_level;
      appLang = config.app_lang || 'tr';
      
      openaiKey = await invoke('get_secret', { key: 'openai_key' });
      deeplKey = await invoke('get_secret', { key: 'deepl_key' });
      googleKey = await invoke('get_secret', { key: 'google_key' });
      configPath = await invoke('get_config_path');
      appVersion = await invoke('get_app_version');
      
      if (serverActive) {
        await loadServerInfo();
      }
      isConfigLoaded = true;

      // Auto-check for updates on startup (silent, no overlay on failure)
      try {
        const info = await invoke('check_for_update');
        if (info.available) {
          updateAvailable = true;
          updateVersion = info.version;
          updateBody = info.body;
        }
      } catch (_) {
        // Silent fail — don't bother user if check fails on startup
      }
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
          server_port: Number(serverPort),
          server_local_only: serverLocalOnly,
          server_auto_start: serverAutoStart,
          
          capture_mode: captureMode,
          capture_interval_sec: Number(intervalSeconds),
          capture_change_threshold: Number(changeThreshold),
          capture_last_region: lastRegion,
          
          pre_grayscale: grayscale,
          pre_binarize: binarization,
          pre_contrast: contrast,
          pre_scale: scaleFloat,
          
          ocr_source_lang: sourceLang,
          ocr_auto_detect_lang: autoDetectLang,
          ocr_merge_lines: mergeLines,
          ocr_merge_paragraphs: mergeParagraphs,
          ocr_min_chars: Number(minCharThreshold),
          
          trans_provider: activeProvider,
          trans_target_lang: targetLang,
          trans_cache_enabled: cacheTranslations,
          trans_openai_endpoint: openaiEndpoint,
          trans_openai_model: openaiModel,
          trans_libre_url: libreUrl,
          
          history_save: saveHistory,
          history_max_records: Number(maxHistory),
          
          app_log_level: logLevel,
          app_lang: appLang
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
      sourceLang, autoDetectLang, mergeLines, mergeParagraphs, minCharThreshold,
      activeProvider, targetLang, cacheTranslations, openaiEndpoint, openaiModel, libreUrl, openaiKey, deeplKey, googleKey,
      serverActive, serverPort, serverAutoStart, serverLocalOnly,
      saveHistory, maxHistory,
      logLevel, appLang
    ];
    saveConfig();
  }

  /** @param {Event} e */
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
        showOverlay("Geçmiş başarıyla temizlendi.", "BİLGİ", "success");
      } catch (e) {
        console.error("Geçmiş silinemedi:", e);
        showOverlay("Geçmiş silinemedi: " + e, "HATA", "error");
      }
    }
  }

  let exportFormat = 'JSON';

  async function exportHistory() {
    try {
      const history = await invoke('get_history');
      if (!history || history.length === 0) {
        showOverlay("Dışa aktarılacak geçmiş bulunamadı!", "UYARI", "warning");
        return;
      }
      
      const savedPath = await invoke('export_history_to_file', { format: exportFormat });
      showOverlay(`Geçmiş başarıyla kaydedildi:<br><br><b>${savedPath}</b>`, "BAŞARILI", "success");
    } catch (e) {
      console.error("Dışa aktarma hatası:", e);
      showOverlay("Dışa aktarılırken hata oluştu: " + e, "HATA", "error");
    }
  }

  /** @type {ReturnType<typeof setTimeout> | undefined} */
  let captureIntervalId = undefined;
  let isCapturingLoop = false;

  async function pickRegion() {
    try {
      const region = await invoke('pick_region');
      lastRegion = region;
    } catch (e) {
      console.error("Bölge seçme hatası:", e);
      showOverlay("Bölge seçimi iptal edildi veya hata oluştu: " + e, "UYARI", "warning");
    }
  }

  async function toggleCaptureLoop() {
    console.log("Çeviri döngüsü tetiklendi");
    
    if (captureMode === 'manual') {
      try {
        await invoke('capture_and_translate');
      } catch (e) {
        if (e !== "No significant text change") {
          console.error("Çeviri hatası:", e);
          showOverlay("Çeviri başarısız: " + e, "HATA", "error");
        }
      }
    } else {
      if (isCapturingLoop) {
        clearTimeout(captureIntervalId);
        isCapturingLoop = false;
        captureIntervalId = undefined;
        // Çeviri durduğunda belleği boşalt
        invoke('unload_ocr').catch(e => console.error(e));
      } else {
        isCapturingLoop = true;
        let ms = Number(intervalSeconds) * 1000;
        
        async function captureLoop() {
          if (!isCapturingLoop) return;
          try {
            await invoke('capture_and_translate');
          } catch (e) {
            if (e !== "No significant text change") console.error(e);
          }
          if (isCapturingLoop) {
            captureIntervalId = setTimeout(captureLoop, ms);
          }
        }
        
        captureLoop();
      }
    }
  }


</script>

<div class="layout">
  <aside class="sidebar">
    <div class="sidebar-header" style="flex-direction: column; gap: 10px; align-items: stretch;">
      <span class="title text-center">{$t('common.settings')}</span>
      <button class="btn {captureMode === 'manual' ? 'btn-primary' : (isCapturingLoop ? 'btn-danger' : 'btn-success')} w-100" on:click={toggleCaptureLoop}>
        {#if captureMode === 'manual'}
          {$t('capture.single')}
        {:else}
          {isCapturingLoop ? $t('capture.stop') : $t('capture.start')}
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
              {$t('capture.capture_btn')}
            </button>
            <div class="row" style="margin-top: 15px; flex-direction: column; align-items: stretch; gap: 5px;">
              <div class="label-desc text-center" style="opacity: 0.8; font-size: 0.85em;">{$t('capture.region_hint')}</div>
              <input type="text" class="form-input code-font text-center" bind:value={lastRegion} placeholder={$t('capture.region_placeholder')} />
            </div>
          </div>

          <section class="section">
            <h3 class="section-title">{$t('capture.title')}</h3>
            
            <div class="row">
              <div class="row-info">
                <label>{$t('capture.mode')}</label>
              </div>
              <select bind:value={captureMode} class="form-select w-auto">
                <option value="manual">{$t('capture.mode_manual')}</option>
                <option value="interval">{$t('capture.mode_interval')}</option>
                <option value="change">{$t('capture.mode_change')}</option>
              </select>
            </div>

            {#if captureMode === 'interval' || captureMode === 'change'}
              <div class="row sub-row">
                <div class="row-info">
                  <label>{$t('capture.interval')}</label>
                </div>
                <div class="input-with-unit">
                  <input type="number" bind:value={intervalSeconds} class="form-input number" min="1" max="60" />
                  <span class="unit">{$t('capture.interval_unit')}</span>
                </div>
              </div>
            {/if}


          </section>

          <section class="section">
            <h3 class="section-title">{$t('preprocessing.title')}</h3>
            
            <div class="row">
              <div class="row-info">
                <label>{$t('preprocessing.grayscale')}</label>
                <div class="label-desc">{$t('preprocessing.grayscale_desc')}</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={grayscale} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>

            <div class="row">
              <div class="row-info">
                <label>{$t('preprocessing.binarize')}</label>
                <div class="label-desc">{$t('preprocessing.binarize_desc')}</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={binarization} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>

            <div class="row">
              <div class="row-info">
                <label>{$t('preprocessing.contrast')}</label>
              </div>
              <select bind:value={contrast} class="form-select w-auto">
                <option value="off">{$t('preprocessing.contrast_off')}</option>
                <option value="light">{$t('preprocessing.contrast_light')}</option>
                <option value="strong">{$t('preprocessing.contrast_strong')}</option>
              </select>
            </div>

            <div class="row">
              <div class="row-info">
                <label>{$t('preprocessing.scale')}</label>
                <div class="label-desc">{$t('preprocessing.scale_desc')}</div>
              </div>
              <select bind:value={scale} class="form-select w-auto">
                <option value="1x">1x</option>
                <option value="2x">{$t('preprocessing.scale_recommended')}</option>
                <option value="3x">3x</option>
              </select>
            </div>
          </section>

          <section class="section">
            <h3 class="section-title">{$t('output.title')}</h3>
            <div class="row">
              <div class="row-info">
                <label>{$t('output.show_in_gui')}</label>
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
            <h3 class="section-title">{$t('ocr.title')}</h3>
            <div class="row">
              <div class="row-info">
                <label>{$t('ocr.engine')}</label>
                <div class="label-desc">{$t('ocr.engine_desc')}</div>
              </div>
              <span class="badge badge-active">OAR-OCR</span>
            </div>

            <div class="row">
              <div class="row-info">
                <label>{$t('ocr.source_lang')}</label>
                <div class="label-desc">{$t('ocr.source_lang_desc')}</div>
              </div>
              <LanguagePicker bind:value={sourceLang} showAuto={true} label={$t('ocr.source_lang_pick')} />
            </div>

            <div class="row {isAutoLang ? '' : 'disabled'}">
              <div class="row-info">
                <label>{$t('ocr.auto_detect')}</label>
                <div class="label-desc">{$t('ocr.auto_detect_desc')}</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={autoDetectLang} disabled={!isAutoLang} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
          </section>

          <section class="section">
            <h3 class="section-title">{$t('text.title')}</h3>
            <div class="row">
              <div class="row-info">
                <label>{$t('text.merge_lines')}</label>
                <div class="label-desc">{$t('text.merge_lines_desc')}</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={mergeLines} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            <div class="row">
              <div class="row-info">
                <label>{$t('text.merge_paragraphs')}</label>
                <div class="label-desc">{$t('text.merge_paragraphs_desc')}</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={mergeParagraphs} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            <div class="row">
              <div class="row-info">
                <label>{$t('text.min_chars')}</label>
                <div class="label-desc">{$t('text.min_chars_desc')}</div>
              </div>
              <input type="number" bind:value={minCharThreshold} class="form-input number" />
            </div>
          </section>


        </div>
      {/if}

      {#if activeTab === 'ceviri'}
        <div class="tab-content">
          <section class="section">
            <h3 class="section-title">{$t('translate.title')}</h3>
            <div class="row">
              <div class="row-info">
                <label>{$t('translate.provider')}</label>
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
                <label>{$t('translate.target_lang')}</label>
              </div>
              <LanguagePicker bind:value={targetLang} showAuto={false} label={$t('translate.target_lang_pick')} />
            </div>

            <div class="row">
              <div class="row-info">
                <label>{$t('translate.cache')}</label>
                <div class="label-desc">{$t('translate.cache_desc')}</div>
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
            <h3 class="section-title">{$t('provider.openai_title')}</h3>
            <div class="row full-col">
              <label>{$t('provider.openai_endpoint')}</label>
              <input type="text" bind:value={openaiEndpoint} placeholder="http://localhost:5000" class="form-input" />
            </div>
            <div class="row full-col">
              <label>{$t('provider.openai_key')}</label>
              <input type="password" bind:value={openaiKey} placeholder="sk-..." class="form-input" />
            </div>
            <div class="row full-col">
              <label>{$t('provider.openai_model')}</label>
              <input type="text" bind:value={openaiModel} placeholder="local-model" class="form-input" />
            </div>
          </section>
          {/if}

          {#if activeProvider === 'deepl'}
          <section class="section">
            <h3 class="section-title">{$t('provider.deepl_title')}</h3>
            <div class="row full-col">
              <label>{$t('provider.deepl_key')}</label>
              <input type="password" bind:value={deeplKey} placeholder="Auth key..." class="form-input" />
            </div>
          </section>
          {/if}

          {#if activeProvider === 'google'}
          <section class="section">
            <h3 class="section-title">{$t('provider.google_title')}</h3>
            <div class="row full-col">
              <label>{$t('provider.google_key')}</label>
              <input type="password" bind:value={googleKey} placeholder="Key..." class="form-input" />
            </div>
          </section>
          {/if}

          {#if activeProvider === 'libre'}
          <section class="section">
            <h3 class="section-title">{$t('provider.libre_title')}</h3>
            <div class="row full-col">
              <label>{$t('provider.libre_url')}</label>
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
                <h2 class="server-title">{$t('server.title')}</h2>
                <div class="server-url">{serverActive ? `http://${serverIp}:${serverPort}` : $t('server.waiting')}</div>
              </div>
            </div>
            <div class="server-actions">
              <div class="badge {serverActive ? 'badge-active' : 'badge-inactive'}">
                {serverActive ? $t('server.active') : $t('server.inactive')}
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
                      <text x="50" y="50" dominant-baseline="middle" text-anchor="middle" font-size="10" fill="black">{$t('server.loading')}</text>
                    </svg>
                  {/if}
                </div>
                <p>{$t('server.qr_scan')}</p>
              </div>
            </div>
          {/if}

          <section class="section">
            <h3 class="section-title">{$t('server.settings_title')}</h3>
            <div class="row">
              <div class="row-info">
                <label>{$t('server.port')}</label>
              </div>
              <input type="number" bind:value={serverPort} class="form-input number" />
            </div>
            <div class="row">
              <div class="row-info">
                <label>{$t('server.auto_start')}</label>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={serverAutoStart} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            <div class="row">
              <div class="row-info">
                <label>{$t('server.local_only')}</label>
                <div class="label-desc">{$t('server.local_only_desc')}</div>
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
            <h3 class="section-title">{$t('shortcuts.title')}</h3>
            <p class="section-desc">{@html $t('shortcuts.desc')}</p>
            
            <div class="code-block">
<pre><code>bind = $mainMod, T, exec, screen-translator --capture
bind = $mainMod, R, exec, screen-translator --repeat
bind = $mainMod, I, exec, screen-translator --toggle-interval</code></pre>
            </div>
            <p class="note">{$t('shortcuts.note')}</p>

            <div class="shortcuts-list">
              <div class="shortcut-item">
                <span class="action-name">{$t('shortcuts.select_translate')}</span>
                <kbd>$mod + T</kbd>
              </div>
              <div class="shortcut-item">
                <span class="action-name">{$t('shortcuts.repeat_last')}</span>
                <kbd>$mod + R</kbd>
              </div>
              <div class="shortcut-item">
                <span class="action-name">{$t('shortcuts.toggle_interval')}</span>
                <kbd>$mod + I</kbd>
              </div>
              <div class="shortcut-item">
                <span class="action-name">{$t('shortcuts.copy_clipboard')}</span>
                <kbd>$mod + C</kbd>
              </div>
            </div>
          </section>
        </div>
      {/if}

      {#if activeTab === 'gecmis'}
        <div class="tab-content">
          <section class="section">
            <h3 class="section-title">{$t('history.settings_title')}</h3>
            <div class="row">
              <div class="row-info">
                <label>{$t('history.save')}</label>
                <div class="label-desc">{$t('history.save_desc')}</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={saveHistory} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>
            
            <div class="row">
              <div class="row-info">
                <label>{$t('history.max_records')}</label>
                <div class="label-desc">{$t('history.max_records_desc')}</div>
              </div>
              <input type="number" bind:value={maxHistory} class="form-input number" />
            </div>

            <div class="row">
              <div class="row-info">
                <label>{$t('history.cache')}</label>
                <div class="label-desc">{$t('history.cache_desc')}</div>
              </div>
              <label class="toggle-wrapper">
                <input type="checkbox" bind:checked={cacheTranslations} class="toggle-input" />
                <div class="toggle-bg"><div class="toggle-dot"></div></div>
              </label>
            </div>

            <div class="row">
              <div class="row-info">
                <label>{$t('history.clear')}</label>
                <div class="label-desc">{$t('history.clear_desc')}</div>
              </div>
              <button class="btn btn-danger" on:click={clearHistory}>{$t('history.clear_btn')}</button>
            </div>
          </section>

          <section class="section">
            <h3 class="section-title">{$t('history.export_title')}</h3>
            <div class="row">
              <div class="row-info">
                <label>{$t('history.export_format')}</label>
              </div>
              <div class="export-actions">
                <select bind:value={exportFormat} class="form-select w-auto">
                  <option value="JSON">JSON</option>
                  <option value="CSV">CSV</option>
                  <option value="TXT">TXT</option>
                </select>
                <button class="btn btn-primary" on:click={exportHistory}>{$t('history.export_btn')}</button>
              </div>
            </div>
          </section>
        </div>
      {/if}

      {#if activeTab === 'uygulama'}
        <div class="tab-content">
          <section class="section">
            <h3 class="section-title">{$t('app.general_title')}</h3>

            <div class="row">
              <div class="row-info">
                <label>{$t('app.log_level')}</label>
              </div>
              <select bind:value={logLevel} class="form-select w-auto">
                <option value="error">{$t('app.log_error')}</option>
                <option value="info">{$t('app.log_info')}</option>
                <option value="debug">{$t('app.log_debug')}</option>
              </select>
            </div>

            <div class="row">
              <div class="row-info">
                <label>{$t('app.ui_lang')}</label>
                <div class="label-desc">{$t('app.ui_lang_desc')}</div>
              </div>
              <select bind:value={appLang} class="form-select w-auto">
                <option value="en">🇬🇧 English</option>
                <option value="tr">🇹🇷 Türkçe</option>
                <option value="de">🇩🇪 Deutsch</option>
                <option value="es">🇪🇸 Español</option>
                <option value="ru">🇷🇺 Русский</option>
                <option value="ja">🇯🇵 日本語</option>
                <option value="zh">🇨🇳 简体中文</option>
              </select>
            </div>

            <div class="row full-col">
              <div class="row-info" style="margin-bottom: 5px;">
                <label>{$t('app.config_path')}</label>
                <div class="label-desc">{$t('app.config_path_desc')}</div>
              </div>
              <div style="display: flex; gap: 8px;">
                <input type="text" bind:value={configPath} class="form-input code-font" style="flex: 1;" />
                <button class="btn btn-primary" on:click={async () => {
                  try {
                    await invoke('set_config_path', { newPath: configPath });
                    showOverlay($t('app.config_success'), $t('overlay.info'), "success");
                  } catch (e) {
                    showOverlay($t('app.config_error') + e, $t('overlay.error'), "error");
                  }
                }}>{$t('app.apply')}</button>
              </div>
            </div>
          </section>

          <section class="section">
            <h3 class="section-title">{$t('app.updates_title')}</h3>
            <div class="row">
              <div class="row-info">
                <label>{$t('app.current_version')}</label>
              </div>
              <div class="version-status">
                <div class="badge badge-version">v{appVersion}</div>
                {#if updateAvailable}
                  <div class="update-subtitle">{$t('app.update_new_version').replace('{version}', updateVersion)}</div>
                {/if}
              </div>
            </div>
            {#if !updateAvailable}
            <div class="row">
              <div class="row-info">
                <label>{$t('app.check_updates')}</label>
              </div>
              <button class="btn btn-primary" on:click={async () => {
                updateChecking = true;
                try {
                  const info = await invoke('check_for_update');
                  if (info.available) {
                    updateAvailable = true;
                    updateVersion = info.version;
                    updateBody = info.body;
                  } else {
                    showOverlay($t('app.up_to_date'), $t('overlay.info'), 'success');
                  }
                } catch (e) {
                  showOverlay($t('app.update_check_failed') + e, $t('overlay.error'), 'error');
                }
                updateChecking = false;
              }} disabled={updateChecking}>
                {updateChecking ? $t('app.checking') : $t('app.check_btn')}
              </button>
            </div>
            {/if}
          </section>
        </div>
      {/if}

    </div>
  </main>
</div>

{#if overlayVisible}
<div class="overlay-backdrop" on:click={closeOverlay} role="presentation">
  <div class="overlay-panel type-{overlayType}" on:click|stopPropagation role="presentation">
    <h2 class="overlay-title">{overlayTitle}</h2>
    <p class="overlay-message">{@html overlayMessage}</p>
    <button class="btn btn-primary" style="margin-top: 15px;" on:click={closeOverlay}>{$t('common.ok')}</button>
  </div>
</div>
{/if}

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

  .version-status {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
  }

  .update-subtitle {
    font-size: 0.72em;
    color: var(--accent-color);
    opacity: 0.9;
    white-space: nowrap;
    animation: subtlePulse 2s ease-in-out infinite;
  }

  @keyframes subtlePulse {
    0%, 100% { opacity: 0.7; }
    50% { opacity: 1; }
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
    color: var(--text-primary);
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
