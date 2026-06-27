<script>
  import { t } from '$lib/i18n';
  import LanguagePicker from '$lib/LanguagePicker.svelte';
  import { 
    sourceLang, autoDetectLang, mergeLines, mergeParagraphs, minCharThreshold 
  } from '$lib/stores';

  $: isAutoLang = $sourceLang === 'auto';
</script>

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
      <LanguagePicker bind:value={$sourceLang} showAuto={true} label={$t('ocr.source_lang_pick')} />
    </div>

    <div class="row {isAutoLang ? '' : 'disabled'}">
      <div class="row-info">
        <label>{$t('ocr.auto_detect')}</label>
        <div class="label-desc">{$t('ocr.auto_detect_desc')}</div>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$autoDetectLang} disabled={!isAutoLang} class="toggle-input" />
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
        <input type="checkbox" bind:checked={$mergeLines} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
    <div class="row">
      <div class="row-info">
        <label>{$t('text.merge_paragraphs')}</label>
        <div class="label-desc">{$t('text.merge_paragraphs_desc')}</div>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$mergeParagraphs} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
    <div class="row">
      <div class="row-info">
        <label>{$t('text.min_chars')}</label>
        <div class="label-desc">{$t('text.min_chars_desc')}</div>
      </div>
      <input type="number" bind:value={$minCharThreshold} class="form-input number" />
    </div>
  </section>
</div>
