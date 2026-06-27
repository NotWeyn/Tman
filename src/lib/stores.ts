import { writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// Active Tab
export const activeTab = writable('yakalama');

// Capture
export const captureMode = writable('manual');
export const intervalSeconds = writable(3);
export const changeThreshold = writable(15);
export const grayscale = writable(false);
export const contrast = writable('off');
export const scale = writable('2x');
export const lastRegion = writable('');

// OCR
export const sourceLang = writable('auto');
export const autoDetectLang = writable(true);
export const mergeLines = writable(true);
export const mergeParagraphs = writable(false);
export const minCharThreshold = writable(5);

// Translate
export const activeProvider = writable('openai');
export const targetLang = writable('tr');
export const cacheTranslations = writable(true);
export const openaiEndpoint = writable('');
export const openaiKey = writable('');
export const openaiModel = writable('');
export const deeplKey = writable('');
export const googleKey = writable('');

// Server
export const serverActive = writable(false);
export const serverPort = writable(7070);
export const serverAutoStart = writable(false);
export const serverLocalOnly = writable(false);
export const serverIp = writable('192.168.1.42');
export const qrCodeBase64 = writable('');

// History
export const saveHistory = writable(true);
export const maxHistory = writable(500);

// App & Options
export const logLevel = writable('info');
export const appLang = writable('en');
export const soundCapture = writable(true);
export const soundComplete = writable(true);
export const autoCopy = writable(false);
export const configPath = writable('');
export const appVersion = writable('0.1.0');

export const customClickAudio = writable<string | null>(null);
export const customPopAudio = writable<string | null>(null);

let isConfigLoaded = false;

export async function loadConfig() {
  try {
    customClickAudio.set((await invoke('get_custom_sound', { soundType: 'click' }).catch(() => null)) as string | null);
    customPopAudio.set((await invoke('get_custom_sound', { soundType: 'pop' }).catch(() => null)) as string | null);

    const config: any = await invoke('get_config');
    captureMode.set(config.capture_mode);
    intervalSeconds.set(config.capture_interval_sec);
    changeThreshold.set(config.capture_change_threshold);
    lastRegion.set(config.capture_last_region);
    grayscale.set(config.pre_grayscale);
    contrast.set(config.pre_contrast);
    scale.set(config.pre_scale === 1.0 ? '1x' : config.pre_scale === 2.0 ? '2x' : '3x');
    
    sourceLang.set(config.ocr_source_lang);
    autoDetectLang.set(config.ocr_auto_detect_lang);
    mergeLines.set(config.ocr_merge_lines);
    mergeParagraphs.set(config.ocr_merge_paragraphs);
    minCharThreshold.set(config.ocr_min_chars);
    
    activeProvider.set(config.trans_provider);
    targetLang.set(config.trans_target_lang);
    cacheTranslations.set(config.trans_cache_enabled);
    openaiEndpoint.set(config.trans_openai_endpoint);
    openaiModel.set(config.trans_openai_model);
    
    serverActive.set(config.server_enabled);
    serverPort.set(config.server_port);
    serverAutoStart.set(config.server_auto_start);
    serverLocalOnly.set(config.server_local_only);
    
    saveHistory.set(config.history_save);
    maxHistory.set(config.history_max_records);
    
    logLevel.set(config.app_log_level);
    appLang.set(config.app_lang || 'tr');
    autoCopy.set(config.ui_auto_copy);
    soundCapture.set(config.ui_sound_capture ?? true);
    soundComplete.set(config.ui_sound_complete ?? true);

    openaiKey.set(await invoke('get_secret', { key: 'openai_key' }));
    deeplKey.set(await invoke('get_secret', { key: 'deepl_key' }));
    googleKey.set(await invoke('get_secret', { key: 'google_key' }));
    configPath.set(await invoke('get_config_path'));
    appVersion.set(await invoke('get_app_version'));

    if (get(serverActive)) {
      await loadServerInfo();
    }
    isConfigLoaded = true;
  } catch (e) {
    console.error("Failed to load config:", e);
  }
}

export async function loadServerInfo() {
  try {
    const info: any = await invoke('get_server_info');
    serverIp.set(info.ip);
    serverPort.set(info.port);
    qrCodeBase64.set(info.qr_code_base64);
  } catch (e) {
    console.error("Failed to load server info:", e);
  }
}

export async function saveConfig() {
  if (!isConfigLoaded) return;
  try {
    const scaleVal = get(scale);
    const scaleFloat = scaleVal === '1x' ? 1.0 : scaleVal === '2x' ? 2.0 : 3.0;
    
    await invoke('save_config', {
      newConfig: {
        server_enabled: get(serverActive),
        server_port: Number(get(serverPort)),
        server_local_only: get(serverLocalOnly),
        server_auto_start: get(serverAutoStart),
        
        capture_mode: get(captureMode),
        capture_interval_sec: Number(get(intervalSeconds)),
        capture_change_threshold: Number(get(changeThreshold)),
        capture_last_region: get(lastRegion),
        
        pre_grayscale: get(grayscale),
        pre_contrast: get(contrast),
        pre_scale: scaleFloat,
        
        ocr_source_lang: get(sourceLang),
        ocr_auto_detect_lang: get(autoDetectLang),
        ocr_merge_lines: get(mergeLines),
        ocr_merge_paragraphs: get(mergeParagraphs),
        ocr_min_chars: Number(get(minCharThreshold)),
        
        trans_provider: get(activeProvider),
        trans_target_lang: get(targetLang),
        trans_cache_enabled: get(cacheTranslations),
        trans_openai_endpoint: get(openaiEndpoint),
        trans_openai_model: get(openaiModel),
        
        history_save: get(saveHistory),
        history_max_records: Number(get(maxHistory)),
        
        app_log_level: get(logLevel),
        app_lang: get(appLang),
        ui_auto_copy: get(autoCopy),
        ui_sound_capture: get(soundCapture),
        ui_sound_complete: get(soundComplete)
      }
    });
    
    await invoke('set_secret', { key: 'openai_key', secret: get(openaiKey) });
    await invoke('set_secret', { key: 'deepl_key', secret: get(deeplKey) });
    await invoke('set_secret', { key: 'google_key', secret: get(googleKey) });
  } catch (e) {
    console.error("Failed to save config:", e);
  }
}

// Overlay Store
export const overlayState = writable({
  visible: false,
  title: '',
  message: '',
  type: 'error'
});

export function showOverlay(msg: string, title = 'HATA', type = 'error') {
  let highlightedMsg = msg.replace(/(Hata:|Warning:|Error:|Failed to|Bölge|Geçmiş)/gi, '<strong class="hl-keyword">$1</strong>');
  highlightedMsg = highlightedMsg.replace(/'([^']+)'/g, '<strong class="hl-quote">\'$1\'</strong>');
  overlayState.set({ visible: true, title, message: highlightedMsg, type });
}

export function closeOverlay() {
  overlayState.update(s => ({ ...s, visible: false }));
}
