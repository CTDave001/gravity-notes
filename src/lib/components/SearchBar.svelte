<script lang="ts">
  let {
    value = '',
    placeholder = 'Search notes...',
    oninput,
    onclear,
  }: {
    value?: string;
    placeholder?: string;
    oninput?: (value: string) => void;
    onclear?: () => void;
  } = $props();

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    value = target.value;
    oninput?.(value);
  }

  function handleClear() {
    value = '';
    onclear?.();
    oninput?.('');
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      handleClear();
    }
  }
</script>

<div class="search-bar">
  <svg
    class="search-icon"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
    />
  </svg>
  <input
    type="text"
    class="search-input"
    {placeholder}
    {value}
    oninput={handleInput}
    onkeydown={handleKeydown}
  />
  {#if value}
    <button
      class="clear-btn"
      onclick={handleClear}
      aria-label="Clear search"
    >
      <svg width="14" height="14" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  {/if}
</div>

<style>
  .search-bar {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    width: 16px;
    height: 16px;
    color: var(--text-muted);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding: 10px 32px 10px 38px;
    font-size: 13px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 8px;
    color: var(--text-primary);
    transition: all 150ms ease;
  }

  .search-input::placeholder {
    color: var(--text-muted);
  }

  .search-input:hover {
    border-color: var(--text-muted);
  }

  .search-input:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 3px var(--accent-muted);
  }

  .clear-btn {
    position: absolute;
    right: 8px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 4px;
    border: none;
    background: transparent;
    border-radius: 4px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 150ms ease;
  }

  .clear-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }
</style>
