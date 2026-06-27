<script lang="ts">
  import { languages, targetLanguages, getFlag } from '$lib/languages';
  import { createEventDispatcher, tick } from 'svelte';
  import { t } from '$lib/i18n';

  export let value: string = '';
  export let showAuto: boolean = false;
  export let label: string = 'Dil seçin';

  const dispatch = createEventDispatcher();

  let isOpen = false;
  let search = '';
  let container: HTMLElement | null = null;
  let dropdownEl: HTMLElement | null = null;
  let dropdownStyle = 'left: 0; right: auto;';

  $: langList = showAuto ? languages : targetLanguages;
  
  $: getDisplayName = (lang: any) => {
    if (!lang) return value;
    if (lang.code === 'auto') return $t('common.auto', 'Auto');
    return lang.name;
  };

  $: filtered = search.trim()
    ? langList.filter(l =>
        getDisplayName(l).toLowerCase().includes(search.toLowerCase()) ||
        l.code.toLowerCase().includes(search.toLowerCase())
      )
    : langList;
  $: currentFlag = getFlag(value);
  $: currentLang = langList.find(l => l.code === value);

  function select(code: string) {
    value = code;
    isOpen = false;
    search = '';
    dispatch('change', code);
  }

  function adjustPosition() {
    if (!container || !dropdownEl) return;
    const rect = container.getBoundingClientRect();
    const dropdownRect = dropdownEl.getBoundingClientRect();
    const windowWidth = window.innerWidth;
    
    let leftOffset = 0;
    
    if (rect.left + dropdownRect.width > windowWidth) {
      let overflow = (rect.left + dropdownRect.width) - windowWidth;
      leftOffset = -(overflow + 10);
      
      const maxShiftLeft = rect.width - dropdownRect.width;
      if (leftOffset < maxShiftLeft) {
        leftOffset = maxShiftLeft;
      }
    }
    
    if (rect.left + leftOffset < 0) {
      leftOffset = -rect.left + 10;
      if (leftOffset > 0) {
        leftOffset = 0;
      }
    }
    
    dropdownStyle = `left: ${leftOffset}px; right: auto;`;
  }

  async function toggle() {
    isOpen = !isOpen;
    if (isOpen) {
      search = '';
      await tick();
      adjustPosition();
      // Focus search input after DOM update
      setTimeout(() => {
        const input = container?.querySelector('.lp-search') as HTMLElement | null;
        if (input) input.focus();
      }, 50);
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

<div class="lang-picker" bind:this={container}>
  <button class="lp-trigger" on:click={toggle} type="button" title={label}>
    <span class="lp-flag">{currentFlag}</span>
    <span class="lp-code">{getDisplayName(currentLang)}</span>
    <span class="lp-arrow" class:open={isOpen}>▾</span>
  </button>

  {#if isOpen}
    <div class="lp-dropdown" style={dropdownStyle} bind:this={dropdownEl}>
      <input
        type="text"
        class="lp-search"
        placeholder={$t('common.search_placeholder', 'Search...')}
        bind:value={search}
      />
      <div class="lp-list">
        {#each filtered as lang (lang.code)}
          <button
            class="lp-item"
            class:active={lang.code === value}
            on:click={() => select(lang.code)}
            type="button"
          >
            <span class="lp-item-flag">{lang.flag}</span>
            <span class="lp-item-name">{getDisplayName(lang)}</span>
            <span class="lp-item-code">{lang.code}</span>
          </button>
        {/each}
        {#if filtered.length === 0}
          <div class="lp-empty">{$t('common.no_results')}</div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<style>
  .lang-picker {
    position: relative;
    display: inline-block;
  }

  .lp-trigger {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 6px 10px;
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.12);
    border-radius: 8px;
    color: inherit;
    cursor: pointer;
    font-size: 0.85rem;
    transition: all 0.15s ease;
    min-width: 140px;
  }

  .lp-trigger:hover {
    background: rgba(255,255,255,0.1);
    border-color: rgba(255,255,255,0.2);
  }

  .lp-flag {
    font-size: 1.15rem;
    line-height: 1;
  }

  .lp-code {
    flex: 1;
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .lp-arrow {
    font-size: 0.7rem;
    opacity: 0.5;
    transition: transform 0.2s ease;
  }

  .lp-arrow.open {
    transform: rotate(180deg);
  }

  .lp-dropdown {
    position: absolute;
    top: calc(100% + 4px);
    z-index: 100;
    min-width: 240px;
    max-height: 320px;
    background: #1e1e2e;
    border: 1px solid rgba(255,255,255,0.12);
    border-radius: 10px;
    box-shadow: 0 8px 32px rgba(0,0,0,0.4);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: lp-fade-in 0.15s ease;
  }

  @keyframes lp-fade-in {
    from { opacity: 0; transform: translateY(-4px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .lp-search {
    padding: 8px 12px;
    border: none;
    border-bottom: 1px solid rgba(255,255,255,0.08);
    background: rgba(255,255,255,0.04);
    color: inherit;
    font-size: 0.82rem;
    outline: none;
  }

  .lp-search::placeholder {
    color: rgba(255,255,255,0.3);
  }

  .lp-search:focus {
    background: rgba(255,255,255,0.07);
  }

  .lp-list {
    overflow-y: auto;
    flex: 1;
    padding: 4px;
  }

  .lp-list::-webkit-scrollbar {
    width: 4px;
  }

  .lp-list::-webkit-scrollbar-thumb {
    background: rgba(255,255,255,0.15);
    border-radius: 2px;
  }

  .lp-item {
    display: flex;
    align-items: center;
    gap: 8px;
    width: 100%;
    padding: 7px 10px;
    background: transparent;
    border: none;
    border-radius: 6px;
    color: rgba(255,255,255,0.8);
    cursor: pointer;
    font-size: 0.82rem;
    text-align: left;
    transition: background 0.1s ease;
  }

  .lp-item:hover {
    background: rgba(255,255,255,0.08);
    color: #fff;
  }

  .lp-item.active {
    background: rgba(139, 92, 246, 0.2);
    color: #c4b5fd;
  }

  .lp-item-flag {
    font-size: 1.05rem;
    flex-shrink: 0;
  }

  .lp-item-name {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .lp-item-code {
    font-size: 0.72rem;
    opacity: 0.4;
    font-family: monospace;
    flex-shrink: 0;
  }

  .lp-empty {
    padding: 16px;
    text-align: center;
    color: rgba(255,255,255,0.3);
    font-size: 0.82rem;
  }
</style>
