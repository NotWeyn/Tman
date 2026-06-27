<script>
  import { t } from '$lib/i18n';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { 
    soundCapture, soundComplete, customClickAudio, customPopAudio, showOverlay 
  } from '$lib/stores';

  /** @param {string} type */
  async function setCustomSound(type) {
    try {
      const filePath = await open({
        multiple: false,
        filters: [{ name: 'Audio', extensions: ['mp3', 'wav', 'ogg'] }]
      });
      if (filePath) {
        await invoke('save_custom_sound', { soundType: type, sourcePath: filePath });
        const b64 = await invoke('get_custom_sound', { soundType: type });
        if (type === 'click') $customClickAudio = b64;
        else $customPopAudio = b64;
        showOverlay($t('options.custom_success_msg'), $t('options.success_title'), "success");
      }
    } catch(e) {
      showOverlay($t('options.error_title') + ": " + e, $t('options.error_title'), "error");
    }
  }

  /** @param {string} type */
  async function resetCustomSound(type) {
    try {
      await invoke('reset_custom_sound', { soundType: type });
      if (type === 'click') $customClickAudio = null;
      else $customPopAudio = null;
      showOverlay($t('options.reset_success_msg'), $t('options.reset_title'), "info");
    } catch(e) {
      showOverlay($t('options.error_title') + ": " + e, $t('options.error_title'), "error");
    }
  }
</script>

<div class="tab-content">
  <section class="section">
    <h3 class="section-title">{$t('options.title') || 'Geri Bildirim ve Sesler'}</h3>

    <div class="row">
      <div class="row-info">
        <label>{$t('options.sound_capture') || 'Çeviri Başlatma Sesi'}</label>
        <div class="label-desc">{$t('options.sound_capture_desc') || 'Ekran seçildiğinde hafif bir klik sesi çalar.'}</div>
        <div style="margin-top: 5px; display: flex; gap: 8px; align-items: center;">
          <button class="btn btn-secondary" style="font-size: 11px; padding: 4px 8px;" on:click={() => setCustomSound('click')}>{$t('options.custom_sound_btn')}</button>
          {#if $customClickAudio}
            <span class="reset-text" on:click={() => resetCustomSound('click')} on:keydown={(e) => e.key === 'Enter' && resetCustomSound('click')} role="button" tabindex="0" title={$t('options.reset_desc_click')} style="font-size: 10px; color: var(--accent); cursor: pointer; text-decoration: underline;">{$t('options.reset')}</span>
          {/if}
        </div>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$soundCapture} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>

    <div class="row">
      <div class="row-info">
        <label>{$t('options.sound_complete') || 'Çeviri Tamamlama Sesi'}</label>
        <div class="label-desc">{$t('options.sound_complete_desc') || 'Çeviri bittiğinde yumuşak bir pop sesi çalar.'}</div>
        <div style="margin-top: 5px; display: flex; gap: 8px; align-items: center;">
          <button class="btn btn-secondary" style="font-size: 11px; padding: 4px 8px;" on:click={() => setCustomSound('pop')}>{$t('options.custom_sound_btn')}</button>
          {#if $customPopAudio}
            <span class="reset-text" on:click={() => resetCustomSound('pop')} on:keydown={(e) => e.key === 'Enter' && resetCustomSound('pop')} role="button" tabindex="0" title={$t('options.reset_desc_pop')} style="font-size: 10px; color: var(--accent); cursor: pointer; text-decoration: underline;">{$t('options.reset')}</span>
          {/if}
        </div>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$soundComplete} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
  </section>
</div>
