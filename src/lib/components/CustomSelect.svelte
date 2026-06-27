<script lang="ts">
  import { createEventDispatcher, tick } from 'svelte';
  
  export let value: string;
  export let options: { value: string, label: string }[] = [];
  export let title: string = '';

  const dispatch = createEventDispatcher();

  let isOpen = false;
  let container: HTMLElement | null = null;
  let dropdownEl: HTMLElement | null = null;
  let dropdownStyle = 'left: 0; right: auto;';

  $: currentOption = options.find(o => o.value === value) || options[0];

  function select(optValue: string) {
    value = optValue;
    isOpen = false;
    dispatch('change', value);
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
      await tick();
      adjustPosition();
    }
  }

  function handleClickOutside(e: MouseEvent) {
    if (container && !container.contains(e.target as Node)) {
      isOpen = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      isOpen = false;
    }
  }
</script>

<svelte:window on:click={handleClickOutside} on:keydown={handleKeydown} />

<div class="custom-select" bind:this={container}>
  <button class="cs-trigger" on:click={toggle} type="button" {title}>
    <span class="cs-label">{currentOption ? currentOption.label : ''}</span>
    <span class="cs-arrow" class:open={isOpen}>▾</span>
  </button>

  {#if isOpen}
    <div class="cs-dropdown" style={dropdownStyle} bind:this={dropdownEl}>
      <div class="cs-list">
        {#each options as opt}
          <button
            class="cs-item"
            class:active={opt.value === value}
            on:click={() => select(opt.value)}
            type="button"
          >
            {opt.label}
          </button>
        {/each}
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
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.12);
    border-radius: 8px;
    color: inherit;
    cursor: pointer;
    font-size: 0.85rem;
    transition: all 0.15s ease;
    min-width: 140px;
  }

  .cs-trigger:hover {
    background: rgba(255,255,255,0.1);
    border-color: rgba(255,255,255,0.2);
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
    min-width: 160px;
    background: #1e1e2e;
    border: 1px solid rgba(255,255,255,0.12);
    border-radius: 10px;
    box-shadow: 0 8px 32px rgba(0,0,0,0.4);
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: cs-fade-in 0.15s ease;
  }

  @keyframes cs-fade-in {
    from { opacity: 0; transform: translateY(-4px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .cs-list {
    overflow-y: auto;
    flex: 1;
    padding: 4px;
    max-height: 240px;
  }

  .cs-list::-webkit-scrollbar {
    width: 4px;
  }

  .cs-list::-webkit-scrollbar-thumb {
    background: rgba(255,255,255,0.15);
    border-radius: 2px;
  }

  .cs-item {
    display: block;
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

  .cs-item:hover {
    background: rgba(255,255,255,0.08);
    color: #fff;
  }

  .cs-item.active {
    background: rgba(139, 92, 246, 0.2);
    color: #c4b5fd;
  }
</style>
