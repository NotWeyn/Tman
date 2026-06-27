<script>
  import { t } from '$lib/i18n';
  import { invoke } from '@tauri-apps/api/core';
  import CustomSelect from '$lib/components/CustomSelect.svelte';
  import { spring } from 'svelte/motion';
  import { 
    captureMode, intervalSeconds, changeThreshold, lastRegion,
    grayscale, contrast, scale, autoCopy, showOverlay 
  } from '$lib/stores';

  const captureScale = spring(1, { stiffness: 0.1, damping: 0.4 });

  const icons = {
    camera: `<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 4h-5L7 7H4a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V9a2 2 0 0 0-2-2h-3l-2.5-3z"/><circle cx="12" cy="13" r="3"/></svg>`
  };

  async function pickRegion() {
    try {
      const region = await invoke('pick_region');
      $lastRegion = region;
    } catch (e) {
      console.error("Bölge seçme hatası:", e);
      showOverlay($t('capture.select_error') + e, $t('overlay.warning'), "warning");
    }
  }
</script>

<div class="tab-content">
  <div class="action-header">
    <button class="btn btn-primary btn-large w-100" 
            on:click={pickRegion}
            on:mousedown={() => captureScale.set(0.95)}
            on:mouseup={() => captureScale.set(1)}
            on:mouseleave={() => captureScale.set(1)}
            style="transform: scale({$captureScale})">
      {@html icons.camera}
      {$t('capture.capture_btn')}
    </button>
    <div class="row" style="margin-top: 15px; flex-direction: column; align-items: stretch; gap: 5px;">
      <div class="label-desc text-center" style="opacity: 0.8; font-size: 0.85em;">{$t('capture.region_hint')}</div>
      <input type="text" class="form-input code-font text-center" bind:value={$lastRegion} placeholder={$t('capture.region_placeholder')} />
    </div>
  </div>

  <section class="section">
    <h3 class="section-title">{$t('capture.title')}</h3>
    
    <div class="row">
      <div class="row-info">
        <label>{$t('capture.mode')}</label>
      </div>
      <CustomSelect 
        bind:value={$captureMode} 
        options={[
          { value: 'manual', label: $t('capture.mode_manual') },
          { value: 'interval', label: $t('capture.mode_interval') },
          { value: 'change', label: $t('capture.mode_change') }
        ]} 
      />
    </div>

    {#if $captureMode === 'interval' || $captureMode === 'change'}
      <div class="row sub-row">
        <div class="row-info">
          <label>{$t('capture.interval')}</label>
        </div>
        <div class="input-with-unit">
          <input type="number" bind:value={$intervalSeconds} class="form-input number" min="1" max="60" />
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
        <input type="checkbox" bind:checked={$grayscale} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>

    <div class="row">
      <div class="row-info">
        <label>{$t('preprocessing.contrast')}</label>
      </div>
      <CustomSelect 
        bind:value={$contrast} 
        options={[
          { value: 'off', label: $t('preprocessing.contrast_off') },
          { value: 'light', label: $t('preprocessing.contrast_light') },
          { value: 'strong', label: $t('preprocessing.contrast_strong') }
        ]} 
      />
    </div>

    <div class="row">
      <div class="row-info">
        <label>{$t('preprocessing.scale')}</label>
        <div class="label-desc">{$t('preprocessing.scale_desc')}</div>
      </div>
      <CustomSelect 
        bind:value={$scale} 
        options={[
          { value: '1x', label: '1x' },
          { value: '2x', label: $t('preprocessing.scale_recommended') },
          { value: '3x', label: '3x' }
        ]} 
      />
    </div>
  </section>

  <section class="section">
    <div class="row">
      <div class="row-info">
        <label>{$t('output.auto_copy') || 'Otomatik Panoya Kopyala'}</label>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$autoCopy} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
  </section>
</div>
