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

<div class="search-bar relative">
  <svg
    class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400"
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
    {placeholder}
    {value}
    oninput={handleInput}
    onkeydown={handleKeydown}
    class="w-full pl-10 pr-8 py-2 text-sm bg-surface-100 dark:bg-surface-900 border border-surface-200 dark:border-surface-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent focus:border-transparent text-surface-900 dark:text-surface-100 placeholder-surface-400"
  />
  {#if value}
    <button
      class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-surface-400 hover:text-surface-600 dark:hover:text-surface-300"
      onclick={handleClear}
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  {/if}
</div>
