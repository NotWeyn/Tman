<script lang="ts">
  import { createEventDispatcher, tick } from 'svelte';
  import { t } from '$lib/i18n';
  
  export let value: string;
  export let options: { value: string, label: string, icon?: string, flag?: string }[] = [];
  export let title: string = '';
  export let searchable: boolean | undefined = undefined;

  const dispatch = createEventDispatcher();

  let isOpen = false;
  let search = '';
  let container: HTMLElement | null = null;
  let dropdownEl: HTMLElement | null = null;
  let dropdownStyle = 'left: 0; right: auto;';

  $: isSearchVisible = searchable ?? options.length > 5;
  $: currentOption = options.find(o => o.value === value) || options[0];

  $: filteredOptions = search.trim() && isSearchVisible
    ? options.filter(o => o.label.toLowerCase().includes(search.toLowerCase()) || o.value.toLowerCase().includes(search.toLowerCase()))
    : options;

  function select(optValue: string) {
    value = optValue;
    isOpen = false;
    search = '';
    dispatch('change', value);
  }

  function adjustPosition() {
    if (!container || !dropdownEl) return;
    const rect = container.getBoundingClientRect();
    const dropdownRect = dropdownEl.getBoundingClientRect();
    const windowWidth = window.innerWidth;
    const windowHeight = window.innerHeight;
    const margin = 8;
    
    // Ideal position: centered relative to the trigger button
    const triggerCenter = rect.left + (rect.width / 2);
    const idealLeft = triggerCenter - (dropdownRect.width / 2);
    
    // Clamp position within viewport boundaries
    const minLeft = margin;
    const maxLeft = Math.max(margin, windowWidth - dropdownRect.width - margin);
    const constrainedLeft = Math.max(minLeft, Math.min(idealLeft, maxLeft));
    
    const leftOffset = constrainedLeft - rect.left;
    
    let topStyle = 'top: calc(100% + 4px); bottom: auto;';
    if (rect.bottom + dropdownRect.height > windowHeight && rect.top > dropdownRect.height) {
      topStyle = 'bottom: calc(100% + 4px); top: auto;';
    }
    
    dropdownStyle = `left: ${leftOffset}px; right: auto; ${topStyle}`;
  }

  async function toggle() {
    isOpen = !isOpen;
    if (isOpen) {
      search = '';
      await tick();
      adjustPosition();
      if (isSearchVisible) {
        setTimeout(() => {
          const input = container?.querySelector('.cs-search') as HTMLElement | null;
          if (input) input.focus();
        }, 50);
      }
    }
  }

  function handleClickOutside(e: MouseEvent) {
    if (container && !container.contains(e.target as Node)) {
      isOpen = false;
      search = '';
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      isOpen = false;
      search = '';
    }
  }
</script>

<svelte:window on:click={handleClickOutside} on:keydown={handleKeydown} on:resize={isOpen ? adjustPosition : undefined} />

<div class="custom-select" bind:this={container}>
  <button class="cs-trigger" on:click={toggle} type="button" {title}>
    {#if currentOption?.flag}
      <span class="cs-flag">{currentOption.flag}</span>
    {:else if currentOption?.icon}
      <span class="cs-icon">{@html currentOption.icon}</span>
    {/if}
    <span class="cs-label">{currentOption ? currentOption.label : ''}</span>
    <span class="cs-arrow" class:open={isOpen}>▾</span>
  </button>

  {#if isOpen}
    <div class="cs-dropdown" style={dropdownStyle} bind:this={dropdownEl}>
      {#if isSearchVisible}
        <input
          type="text"
          class="cs-search"
          placeholder={$t('common.search_placeholder', 'Ara...')}
          bind:value={search}
        />
      {/if}
      <div class="cs-list">
        {#each filteredOptions as opt (opt.value)}
          <button
            class="cs-item"
            class:active={opt.value === value}
            on:click={() => select(opt.value)}
            type="button"
          >
            {#if opt.flag}
              <span class="cs-item-flag">{opt.flag}</span>
            {:else if opt.icon}
              <span class="cs-item-icon">{@html opt.icon}</span>
            {/if}
            <span class="cs-item-name">{opt.label}</span>
          </button>
        {/each}
        {#if filteredOptions.length === 0}
          <div class="cs-empty">{$t('common.no_results', 'Sonuç bulunamadı')}</div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .custom-select {
    position: relative;
    display: inline-block;
  }

  .cs-trigger {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 8px;
    color: inherit;
    cursor: pointer;
    font-size: 0.85rem;
    transition: all 0.15s ease;
    min-width: 140px;
  }

  .cs-trigger:hover {
    background: rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.2);
  }

  .cs-trigger:focus {
    outline: none;
    border-color: var(--accent-color, #6366f1);
    box-shadow: 0 0 0 2px rgba(99, 102, 241, 0.2);
  }

  .cs-flag {
    font-size: 1.15rem;
    line-height: 1;
  }

  .cs-icon {
    display: flex;
    align-items: center;
    font-size: 1rem;
    opacity: 0.8;
  }

  .cs-label {
    flex: 1;
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cs-arrow {
    font-size: 0.7rem;
    opacity: 0.5;
    transition: transform 0.2s ease;
  }

  .cs-arrow.open {
    transform: rotate(180deg);
  }

  .cs-dropdown {
    position: absolute;
    top: calc(100% + 4px);
    z-index: 100;
    min-width: 180px;
    max-height: 320px;
    background: #1e1e2e;
    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 10px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: cs-fade-in 0.15s ease;
  }

  @keyframes cs-fade-in {
    from { opacity: 0; transform: translateY(-4px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .cs-search {
    padding: 8px 12px;
    border: none;
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.04);
    color: inherit;
    font-size: 0.82rem;
    outline: none;
  }

  .cs-search::placeholder {
    color: rgba(255, 255, 255, 0.3);
  }

  .cs-search:focus {
    background: rgba(255, 255, 255, 0.07);
  }

  .cs-list {
    overflow-y: auto;
    flex: 1;
    padding: 4px;
  }

  .cs-list::-webkit-scrollbar {
    width: 4px;
  }

  .cs-list::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.15);
    border-radius: 2px;
  }

  .cs-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 7px 10px;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: rgba(255, 255, 255, 0.8);
    cursor: pointer;
    font-size: 0.82rem;
    text-align: left;
    transition: background 0.1s ease;
  }

  .cs-item:hover {
    background: rgba(255, 255, 255, 0.08);
    color: #fff;
  }

  .cs-item.active {
    background: rgba(139, 92, 246, 0.2);
    color: #c4b5fd;
    font-weight: 500;
  }

  .cs-item-flag {
    font-size: 1.05rem;
    flex-shrink: 0;
  }

  .cs-item-icon {
    display: flex;
    align-items: center;
    font-size: 0.9rem;
    flex-shrink: 0;
  }

  .cs-item-name {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cs-empty {
    padding: 16px;
    text-align: center;
    color: rgba(255, 255, 255, 0.3);
    font-size: 0.82rem;
  }
</style>

