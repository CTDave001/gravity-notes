<script lang="ts">
  let {
    wordCount = 0,
    charCount = 0,
    saved = true,
  }: {
    wordCount?: number;
    charCount?: number;
    saved?: boolean;
  } = $props();
</script>

<div class="status-bar flex items-center justify-between px-4 py-2 text-xs text-surface-500 dark:text-surface-400 bg-surface-100 dark:bg-surface-900 border-t border-surface-200 dark:border-surface-800 select-none">
  <div class="flex items-center gap-4">
    <span>{wordCount} {wordCount === 1 ? 'word' : 'words'}</span>
    <span>{charCount} {charCount === 1 ? 'character' : 'characters'}</span>
  </div>
  <div class="flex items-center gap-2">
    {#if saved}
      <span class="text-green-600 dark:text-green-400">Saved</span>
    {:else}
      <span class="text-amber-600 dark:text-amber-400">Saving...</span>
    {/if}
  </div>
</div>
