<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount } from 'svelte';
  import { t } from '$lib/i18n';
  import { 
    activeTab, captureMode, autoCopy, soundCapture, soundComplete, customClickAudio, customPopAudio,
    loadConfig, saveConfig, overlayState, closeOverlay
  } from '$lib/stores';

  import CaptureTab from '$lib/components/tabs/CaptureTab.svelte';
  import OcrTab from '$lib/components/tabs/OcrTab.svelte';
  import TranslateTab from '$lib/components/tabs/TranslateTab.svelte';
  import ServerTab from '$lib/components/tabs/ServerTab.svelte';
  import ShortcutsTab from '$lib/components/tabs/ShortcutsTab.svelte';
  import HistoryTab from '$lib/components/tabs/HistoryTab.svelte';
  import OptionsTab from '$lib/components/tabs/OptionsTab.svelte';
  import AppTab from '$lib/components/tabs/AppTab.svelte';

  const icons = {
    camera: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/><circle cx="12" cy="13" r="3"/></svg>`,
    type: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 7 4 4 20 4 20 7"/><line x1="9" x2="15" y1="20" y2="20"/><line x1="12" x2="12" y1="4" y2="20"/></svg>`,
    languages: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m5 8 6 6"/><path d="m4 14 6-6 2-3"/><path d="M2 5h12"/><path d="M7 2h1"/><path d="m22 22-5-10-5 10"/><path d="M14 18h6"/></svg>`,
    smartphone: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="14" height="20" x="5" y="2" rx="2" ry="2"/><path d="M12 18h.01"/></svg>`,
    keyboard: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="16" x="2" y="4" rx="2" ry="2"/><path d="M6 8h.001"/><path d="M10 8h.001"/><path d="M14 8h.001"/><path d="M18 8h.001"/><path d="M8 12h.001"/><path d="M12 12h.001"/><path d="M16 12h.001"/><path d="M7 16h10"/></svg>`,
    clock: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>`,
    settings: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>`,
    sliders: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="4" y1="21" x2="4" y2="14"/><line x1="4" y1="10" x2="4" y2="3"/><line x1="12" y1="21" x2="12" y2="12"/><line x1="12" y1="8" x2="12" y2="3"/><line x1="20" y1="21" x2="20" y2="16"/><line x1="20" y1="12" x2="20" y2="3"/><line x1="1" y1="14" x2="7" y2="14"/><line x1="9" y1="8" x2="15" y2="8"/><line x1="17" y1="16" x2="23" y2="16"/></svg>`,
  };

  $: tabs = [
    { id: 'yakalama', label: $t('tabs.capture'), icon: icons.camera, component: CaptureTab },
    { id: 'ocr', label: $t('tabs.ocr'), icon: icons.type, component: OcrTab },
    { id: 'ceviri', label: $t('tabs.translate'), icon: icons.languages, component: TranslateTab },
    { id: 'mobil', label: $t('tabs.server'), icon: icons.smartphone, component: ServerTab },
    { id: 'kisayollar', label: $t('tabs.shortcuts'), icon: icons.keyboard, component: ShortcutsTab },
    { id: 'gecmis', label: $t('tabs.history'), icon: icons.clock, component: HistoryTab },
    { id: 'secenekler', label: $t('tabs.options') || 'Seçenekler', icon: icons.sliders, component: OptionsTab },
    { id: 'uygulama', label: $t('tabs.app'), icon: icons.settings, component: AppTab }
  ];

  function playSound(type: string) {
    if (type === 'click' && !$soundCapture) return;
    if (type === 'pop' && !$soundComplete) return;

    let customAudioStr = type === 'click' ? $customClickAudio : $customPopAudio;
    if (customAudioStr) {
      try {
        const audio = new window.Audio("data:audio/mp3;base64," + customAudioStr);
        audio.play().catch(e => console.error("Özel ses çalma hatası:", e));
      } catch (e) {
        console.error("Özel ses çalma hatası:", e);
      }
      return;
    }

    try {
      // @ts-ignore
      const AudioCtx = window.AudioContext || window['webkitAudioContext'];
      const ctx = new AudioCtx();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.connect(gain);
      gain.connect(ctx.destination);

      if (type === 'click') {
        osc.type = 'sine';
        osc.frequency.setValueAtTime(600, ctx.currentTime);
        osc.frequency.exponentialRampToValueAtTime(100, ctx.currentTime + 0.05);
        gain.gain.setValueAtTime(0.5, ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.05);
        osc.start();
        osc.stop(ctx.currentTime + 0.05);
      } else if (type === 'pop') {
        osc.type = 'sine';
        osc.frequency.setValueAtTime(250, ctx.currentTime);
        osc.frequency.exponentialRampToValueAtTime(400, ctx.currentTime + 0.1);
        gain.gain.setValueAtTime(0.3, ctx.currentTime);
        gain.gain.exponentialRampToValueAtTime(0.01, ctx.currentTime + 0.1);
        osc.start();
        osc.stop(ctx.currentTime + 0.1);
      }
    } catch (e) {
      console.error("Audio playback failed", e);
    }
  }

  onMount(async () => {
    await loadConfig();

    listen('capture-done', () => {
      playSound('click');
    });
    listen('translation-update', (event) => {
      playSound('pop');
      const payload = event.payload as any;
      if ($autoCopy && payload && payload.translated_text) {
        navigator.clipboard.writeText(payload.translated_text).catch(e => console.error("Clipboard error", e));
      }
    });

    // We can auto-save config on beforeunload or use reactive statement in stores if preferred,
    // but the original code had a reactive block saving config whenever any field changes.
    // That is better handled by a subscription to the stores, but doing it manually via a button
    // or unmount is also fine. Svelte 5 runes simplify this, but with stores we'd subscribe.
  });

  // Watch stores and save automatically (like original reactive block)
  import { captureMode as cMode, intervalSeconds, changeThreshold, lastRegion, grayscale, contrast, scale, sourceLang, autoDetectLang, mergeLines, mergeParagraphs, minCharThreshold, activeProvider, targetLang, cacheTranslations, openaiEndpoint, openaiModel, serverActive, serverPort, serverAutoStart, serverLocalOnly, saveHistory, maxHistory, logLevel, appLang, soundCapture as sCapture, soundComplete as sComplete, autoCopy as aCopy } from '$lib/stores';

  $: {
    let _deps = [
      $cMode, $intervalSeconds, $changeThreshold, $lastRegion,
      $grayscale, $contrast, $scale,
      $sourceLang, $autoDetectLang, $mergeLines, $mergeParagraphs, $minCharThreshold,
      $activeProvider, $targetLang, $cacheTranslations, $openaiEndpoint, $openaiModel,
      $serverActive, $serverPort, $serverAutoStart, $serverLocalOnly,
      $saveHistory, $maxHistory,
      $logLevel, $appLang, $aCopy, $sCapture, $sComplete
    ];
    saveConfig();
  }

  let captureIntervalId: ReturnType<typeof setTimeout> | undefined = undefined;
  let isCapturingLoop = false;

  async function toggleCaptureLoop() {
    console.log("Çeviri döngüsü tetiklendi");
    
    if ($captureMode === 'manual') {
      try {
        await invoke('capture_and_translate');
      } catch (e) {
        if (e !== "No significant text change") {
          console.error("Çeviri hatası:", e);
          import('$lib/stores').then(m => m.showOverlay($t('translate.failed') + e, $t('overlay.error'), "error"));
        }
      }
    } else {
      if (isCapturingLoop) {
        clearTimeout(captureIntervalId);
        isCapturingLoop = false;
        captureIntervalId = undefined;
        invoke('unload_ocr').catch(e => console.error(e));
      } else {
        isCapturingLoop = true;
        let ms = Number($intervalSeconds) * 1000;
        
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
      <button class="btn {$captureMode === 'manual' ? 'btn-primary' : (isCapturingLoop ? 'btn-danger' : 'btn-success')} w-100" on:click={toggleCaptureLoop}>
        {#if $captureMode === 'manual'}
          {$t('capture.single')}
        {:else}
          {isCapturingLoop ? $t('capture.stop') : $t('capture.start')}
        {/if}
      </button>
    </div>
    <nav class="nav-menu">
      {#each tabs as tab}
        <button 
          class="nav-item {$activeTab === tab.id ? 'active' : ''}" 
          on:click={() => $activeTab = tab.id}
        >
          <div class="nav-icon-wrapper">{@html tab.icon}</div>
          <span>{tab.label}</span>
        </button>
      {/each}
    </nav>
  </aside>

  <main class="content-area">
    <div class="content-scroll">
      {#each tabs as tab}
        {#if $activeTab === tab.id}
          <svelte:component this={tab.component} />
        {/if}
      {/each}
    </div>
  </main>
</div>

{#if $overlayState.visible}
<div class="overlay-backdrop" on:click={closeOverlay} role="presentation">
  <div class="overlay-panel type-{$overlayState.type}" on:click|stopPropagation role="presentation">
    <h2 class="overlay-title">{$overlayState.title}</h2>
    <p class="overlay-message">{@html $overlayState.message}</p>
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

  :global(.tab-content) {
    max-width: 600px;
    animation: fadeIn 0.3s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(5px); }
    to { opacity: 1; transform: translateY(0); }
  }

  :global(.section) {
    margin-bottom: 2.5rem;
  }

  :global(.section.disabled) {
    opacity: 0.5;
    pointer-events: none;
    filter: grayscale(1);
  }

  :global(.section-title) {
    font-size: 0.75rem;
    font-weight: 700;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin-bottom: 1rem;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }

  :global(.section-desc) {
    font-size: 0.875rem;
    margin-bottom: 1rem;
  }

  :global(.row) {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    min-height: 40px;
  }

  :global(.row.sub-row) {
    padding-left: 1.5rem;
    border-left: 2px solid var(--border-color);
    margin-left: 0.5rem;
  }

  :global(.row.full-col) {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  :global(.row-info) {
    flex: 1;
    padding-right: 2rem;
  }

  :global(.w-auto) {
    width: auto;
    min-width: 140px;
  }

  :global(.input-with-unit) {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  :global(.unit) {
    font-size: 0.875rem;
    color: var(--text-muted);
  }

  /* Server Card Styles moved globally so ServerTab can use them */
  :global(.server-card) {
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

  :global(.server-info) {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  :global(.server-icon) {
    width: 48px;
    height: 48px;
    border-radius: var(--radius-md);
    background: rgba(99, 102, 241, 0.1);
    color: var(--accent-color);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  :global(.server-title) {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 0.25rem;
  }

  :global(.server-url) {
    font-size: 0.875rem;
    color: var(--accent-hover);
    font-family: monospace;
  }

  :global(.server-actions) {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  :global(.badge) {
    padding: 0.25rem 0.625rem;
    border-radius: 999px;
    font-size: 0.75rem;
    font-weight: 600;
  }

  :global(.badge-active) {
    background: rgba(16, 185, 129, 0.1);
    color: var(--success-color);
    border: 1px solid rgba(16, 185, 129, 0.2);
  }

  :global(.badge-inactive) {
    background: rgba(113, 113, 122, 0.1);
    color: var(--text-muted);
    border: 1px solid rgba(113, 113, 122, 0.2);
  }

  :global(.badge-version) {
    background: var(--bg-surface-hover);
    color: var(--text-primary);
  }

  :global(.version-status) {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 4px;
  }

  :global(.update-subtitle) {
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

  :global(.server-toggle .toggle-bg) {
    width: 48px;
    height: 26px;
  }
  :global(.server-toggle .toggle-dot) {
    width: 20px;
    height: 20px;
  }
  :global(.server-toggle .toggle-input:checked + .toggle-bg .toggle-dot) {
    transform: translateX(22px);
  }

  :global(.qr-container) {
    display: flex;
    justify-content: center;
    margin-bottom: 2rem;
  }

  :global(.qr-placeholder) {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  :global(.qr-box) {
    background: white;
    padding: 1rem;
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-glow);
  }

  :global(.qr-placeholder p) {
    font-size: 0.875rem;
  }

  /* Shortcuts */
  :global(.code-block) {
    background: #000;
    border: 1px solid var(--border-color);
    border-radius: var(--radius-md);
    padding: 1rem;
    margin-bottom: 0.75rem;
    overflow-x: auto;
  }
  
  :global(.code-block pre) {
    margin: 0;
    color: #e2e8f0;
    font-family: 'Courier New', Courier, monospace;
    font-size: 0.875rem;
  }

  :global(.note) {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-bottom: 2rem;
  }

  :global(.shortcuts-list) {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  :global(.shortcut-item) {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--bg-surface);
    padding: 0.75rem 1rem;
    border-radius: var(--radius-md);
    border: 1px solid var(--border-color);
  }

  :global(.action-name) {
    font-size: 0.875rem;
    font-weight: 500;
  }

  :global(kbd) {
    background: var(--bg-sidebar);
    border: 1px solid var(--border-active);
    border-radius: var(--radius-sm);
    padding: 0.25rem 0.5rem;
    font-family: monospace;
    font-size: 0.75rem;
    color: var(--accent-hover);
  }

  :global(.export-actions) {
    display: flex;
    gap: 0.5rem;
  }

  :global(.code-font) {
    font-family: 'Courier New', Courier, monospace;
    color: var(--text-primary);
  }

  :global(.action-header) {
    margin-bottom: 2.5rem;
  }

  :global(.btn-large) {
    padding: 1rem 2rem;
    font-size: 1.1rem;
    font-weight: 600;
    justify-content: center;
  }

  .w-100 {
    width: 100%;
  }

  :global(.input-with-button) {
    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  :global(.install-status) {
    font-size: 0.875rem;
    color: var(--accent-color);
    margin-top: 0.5rem;
    font-weight: 500;
  }
</style>
