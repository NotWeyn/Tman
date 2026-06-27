<script>
  import { t } from '$lib/i18n';
  import LanguagePicker from '$lib/LanguagePicker.svelte';
  import CustomSelect from '$lib/components/CustomSelect.svelte';
  import { 
    activeProvider, targetLang, cacheTranslations, 
    openaiEndpoint, openaiKey, openaiModel, 
    deeplKey, googleKey 
  } from '$lib/stores';
</script>

<div class="tab-content">
  <section class="section">
    <h3 class="section-title">{$t('translate.title')}</h3>
    <div class="row">
      <div class="row-info">
        <label>{$t('translate.provider')}</label>
      </div>
      <CustomSelect 
        bind:value={$activeProvider} 
        options={[
          { value: 'openai', label: 'OpenAI-compatible' },
          { value: 'google', label: 'Google Translate' },
          { value: 'deepl', label: 'DeepL' }
        ]} 
      />
    </div>

    <div class="row">
      <div class="row-info">
        <label>{$t('translate.target_lang')}</label>
      </div>
      <LanguagePicker bind:value={$targetLang} showAuto={false} label={$t('translate.target_lang_pick')} />
    </div>

    <div class="row">
      <div class="row-info">
        <label>{$t('translate.cache')}</label>
        <div class="label-desc">{$t('translate.cache_desc')}</div>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$cacheTranslations} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
  </section>

  <!-- Provider Specific Settings -->
  {#if $activeProvider === 'openai'}
  <section class="section">
    <h3 class="section-title">{$t('provider.openai_title')}</h3>
    <div class="row full-col">
      <label>{$t('provider.openai_endpoint')}</label>
      <input type="text" bind:value={$openaiEndpoint} placeholder="http://localhost:5000" class="form-input" />
    </div>
    <div class="row full-col">
      <label>{$t('provider.openai_key')}</label>
      <input type="password" bind:value={$openaiKey} placeholder="sk-..." class="form-input" />
    </div>
    <div class="row full-col">
      <label>{$t('provider.openai_model')}</label>
      <input type="text" bind:value={$openaiModel} placeholder="local-model" class="form-input" />
    </div>
  </section>
  {/if}

  {#if $activeProvider === 'deepl'}
  <section class="section">
    <h3 class="section-title">{$t('provider.deepl_title')}</h3>
    <div class="row full-col">
      <label>{$t('provider.deepl_key')}</label>
      <input type="password" bind:value={$deeplKey} placeholder="Auth key..." class="form-input" />
    </div>
  </section>
  {/if}

  {#if $activeProvider === 'google'}
  <section class="section">
    <h3 class="section-title">{$t('provider.google_title')}</h3>
    <div class="row full-col">
      <label>{$t('provider.google_key')}</label>
      <input type="password" bind:value={$googleKey} placeholder="Key..." class="form-input" />
    </div>
  </section>
  {/if}
</div>
