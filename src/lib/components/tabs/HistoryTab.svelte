<script>
  import { t } from '$lib/i18n';
  import { invoke } from '@tauri-apps/api/core';
  import CustomSelect from '$lib/components/CustomSelect.svelte';
  import { save } from '@tauri-apps/plugin-dialog';
  import { 
    saveHistory, maxHistory, cacheTranslations, showOverlay 
  } from '$lib/stores';

  let exportFormat = 'JSON';

  async function clearHistory() {
    if(confirm($t('history.clear_confirm'))) {
      try {
        await invoke('clear_history');
        showOverlay($t('history.clear_success'), $t('overlay.info'), "success");
      } catch (e) {
        console.error("Geçmiş silinemedi:", e);
        showOverlay($t('history.clear_error') + e, $t('overlay.error'), "error");
      }
    }
  }

  async function exportHistory() {
    try {
      const history = await invoke('get_history');
      if (!history || history.length === 0) {
        showOverlay($t('history.no_records'), $t('overlay.warning'), "warning");
        return;
      }
      
      const defaultExt = exportFormat === 'CSV' ? 'csv' : (exportFormat === 'Anki' ? 'tsv' : (exportFormat === 'JSON' ? 'json' : 'txt'));
      const savePath = await save({
        defaultPath: 'tman_history.' + defaultExt,
        filters: [{
          name: 'Tman Export',
          extensions: [defaultExt]
        }]
      });

      if (!savePath) return;

      const savedPath = await invoke('export_history_to_file', { format: exportFormat, savePath });
      showOverlay($t('history.export_success').replace('{path}', savedPath), $t('overlay.success'), "success");
    } catch (e) {
      console.error("Dışa aktarma hatası:", e);
      showOverlay($t('history.export_error') + e, $t('overlay.error'), "error");
    }
  }
</script>

<div class="tab-content">
  <section class="section">
    <h3 class="section-title">{$t('history.settings_title')}</h3>
    <div class="row">
      <div class="row-info">
        <label>{$t('history.save')}</label>
        <div class="label-desc">{$t('history.save_desc')}</div>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$saveHistory} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
    
    <div class="row">
      <div class="row-info">
        <label>{$t('history.max_records')}</label>
        <div class="label-desc">{$t('history.max_records_desc')}</div>
      </div>
      <input type="number" bind:value={$maxHistory} class="form-input number" />
    </div>

    <div class="row">
      <div class="row-info">
        <label>{$t('history.cache')}</label>
        <div class="label-desc">{$t('history.cache_desc')}</div>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$cacheTranslations} class="toggle-input" />
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
        <CustomSelect 
          bind:value={exportFormat} 
          options={[
            { value: 'JSON', label: 'JSON' },
            { value: 'CSV', label: 'CSV' },
            { value: 'Anki', label: 'Anki (TSV)' },
            { value: 'TXT', label: 'TXT' }
          ]} 
        />
        <button class="btn btn-primary" on:click={exportHistory}>{$t('history.export_btn')}</button>
      </div>
    </div>
  </section>
</div>
