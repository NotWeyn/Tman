<script>
  import { t, setLocale } from '$lib/i18n';
  import { invoke } from '@tauri-apps/api/core';
  import CustomSelect from '$lib/components/CustomSelect.svelte';
  import { 
    logLevel, appLang, configPath, appVersion, showOverlay 
  } from '$lib/stores';

  let updateChecking = false;
  let updateAvailable = false;
  let updateVersion = '';

  // Ensure language updates on change
  $: setLocale($appLang);

  async function applyConfigPath() {
    try {
      await invoke('set_config_path', { newPath: $configPath });
      showOverlay($t('app.config_success'), $t('overlay.info'), "success");
    } catch (e) {
      showOverlay($t('app.config_error') + e, $t('overlay.error'), "error");
    }
  }

  async function checkForUpdate() {
    updateChecking = true;
    try {
      const info = await invoke('check_for_update');
      if (info.available) {
        updateAvailable = true;
        updateVersion = info.version;
      } else {
        showOverlay($t('app.up_to_date'), $t('overlay.info'), 'success');
      }
    } catch (e) {
      showOverlay($t('app.update_check_failed') + e, $t('overlay.error'), 'error');
    }
    updateChecking = false;
  }
</script>

<div class="tab-content">
  <section class="section">
    <h3 class="section-title">{$t('app.general_title')}</h3>

    <div class="row">
      <div class="row-info">
        <label>{$t('app.log_level')}</label>
      </div>
      <CustomSelect 
        bind:value={$logLevel} 
        options={[
          { value: 'error', label: $t('app.log_error') },
          { value: 'info', label: $t('app.log_info') },
          { value: 'debug', label: $t('app.log_debug') }
        ]} 
      />
    </div>

    <div class="row">
      <div class="row-info">
        <label>{$t('app.ui_lang')}</label>
        <div class="label-desc">{$t('app.ui_lang_desc')}</div>
      </div>
      <CustomSelect 
        bind:value={$appLang} 
        options={[
          { value: 'en', label: '🇬🇧 English' },
          { value: 'tr', label: '🇹🇷 Türkçe' },
          { value: 'de', label: '🇩🇪 Deutsch' },
          { value: 'es', label: '🇪🇸 Español' },
          { value: 'ru', label: '🇷🇺 Русский' },
          { value: 'ja', label: '🇯🇵 日本語' },
          { value: 'zh', label: '🇨🇳 简体中文' }
        ]} 
      />
    </div>

    <div class="row full-col">
      <div class="row-info" style="margin-bottom: 5px;">
        <label>{$t('app.config_path')}</label>
        <div class="label-desc">{$t('app.config_path_desc')}</div>
      </div>
      <div style="display: flex; gap: 8px;">
        <input type="text" bind:value={$configPath} class="form-input code-font" style="flex: 1;" />
        <button class="btn btn-primary" on:click={applyConfigPath}>{$t('app.apply')}</button>
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
        <div class="badge badge-version">v{$appVersion}</div>
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
      <button class="btn btn-primary" on:click={checkForUpdate} disabled={updateChecking}>
        {updateChecking ? $t('app.checking') : $t('app.check_btn')}
      </button>
    </div>
    {/if}
  </section>
</div>
