<script>
  import { t } from '$lib/i18n';
  import { invoke } from '@tauri-apps/api/core';
  import { 
    serverActive, serverPort, serverAutoStart, serverLocalOnly,
    serverIp, qrCodeBase64, loadServerInfo 
  } from '$lib/stores';

  const icons = {
    monitorPlay: `<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect width="20" height="14" x="2" y="3" rx="2"/><path d="M14 9.5 10 7v5z"/><line x1="12" x2="12" y1="17" y2="21"/><line x1="8" x2="16" y1="21" y2="21"/></svg>`
  };

  /** @param {Event} e */
  async function handleServerToggle(e) {
    try {
      await invoke('toggle_server', { active: $serverActive });
      if ($serverActive) {
        await loadServerInfo();
      }
    } catch (err) {
      console.error("Failed to toggle server:", err);
    }
  }
</script>

<div class="tab-content">
  <div class="server-card">
    <div class="server-info">
      <div class="server-icon">{@html icons.monitorPlay}</div>
      <div>
        <h2 class="server-title">{$t('server.title')}</h2>
        <div class="server-url">{$serverActive ? `http://${$serverIp}:${$serverPort}` : $t('server.waiting')}</div>
      </div>
    </div>
    <div class="server-actions">
      <div class="badge {$serverActive ? 'badge-active' : 'badge-inactive'}">
        {$serverActive ? $t('server.active') : $t('server.inactive')}
      </div>
      <label class="toggle-wrapper server-toggle">
        <input type="checkbox" bind:checked={$serverActive} on:change={handleServerToggle} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
  </div>

  {#if $serverActive}
    <div class="qr-container">
      <div class="qr-placeholder">
        <div class="qr-box">
          {#if $qrCodeBase64}
            <img src="{$qrCodeBase64}" alt="QR Code" style="width: 150px; height: 150px; image-rendering: pixelated;" />
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
      <input type="number" bind:value={$serverPort} class="form-input number" />
    </div>
    <div class="row">
      <div class="row-info">
        <label>{$t('server.auto_start')}</label>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$serverAutoStart} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
    <div class="row">
      <div class="row-info">
        <label>{$t('server.local_only')}</label>
        <div class="label-desc">{$t('server.local_only_desc')}</div>
      </div>
      <label class="toggle-wrapper">
        <input type="checkbox" bind:checked={$serverLocalOnly} class="toggle-input" />
        <div class="toggle-bg"><div class="toggle-dot"></div></div>
      </label>
    </div>
  </section>
</div>
