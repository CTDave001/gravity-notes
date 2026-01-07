<script lang="ts">
  import type { NoteMeta } from '../types';

  // Svelte 5 props using $props() rune
  let {
    notes = [],
    selectedId = null,
    viewMode = 'list',
    onselect,
  }: {
    notes?: NoteMeta[];
    selectedId?: string | null;
    viewMode?: 'list' | 'grid';
    onselect?: (note: NoteMeta) => void;
  } = $props();

  function formatRelativeDate(dateStr: string): string {
    if (!dateStr) return '';

    const date = new Date(dateStr);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffSecs = Math.floor(diffMs / 1000);
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffSecs < 60) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    if (diffDays < 30) return `${Math.floor(diffDays / 7)}w ago`;

    return date.toLocaleDateString();
  }

  function handleSelect(note: NoteMeta) {
    onselect?.(note);
  }
</script>

{#if notes.length === 0}
  <div class="empty-state flex flex-col items-center justify-center h-full p-8 text-center">
    <svg class="w-12 h-12 text-surface-300 dark:text-surface-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
    </svg>
    <p class="text-surface-500 dark:text-surface-400 text-sm">No notes yet</p>
    <p class="text-surface-400 dark:text-surface-500 text-xs mt-1">Create a note to get started</p>
  </div>
{:else if viewMode === 'list'}
  <div class="note-list flex flex-col">
    {#each notes as note (note.id)}
      <button
        class="note-item p-3 text-left border-b border-surface-200 dark:border-surface-800 hover:bg-surface-100 dark:hover:bg-surface-900 transition-colors"
        class:bg-accent={selectedId === note.id}
        class:bg-opacity-10={selectedId === note.id}
        class:dark:bg-opacity-20={selectedId === note.id}
        onclick={() => handleSelect(note)}
      >
        <div class="font-medium text-sm text-surface-900 dark:text-surface-100 truncate">
          {note.title || 'Untitled'}
        </div>
        <div class="text-xs text-surface-500 dark:text-surface-400 mt-1 truncate">
          {note.preview || 'No content'}
        </div>
        <div class="text-xs text-surface-400 dark:text-surface-500 mt-1">
          {formatRelativeDate(note.modified_at)}
        </div>
      </button>
    {/each}
  </div>
{:else}
  <div class="note-grid grid grid-cols-2 gap-3 p-3">
    {#each notes as note (note.id)}
      <button
        class="note-card p-4 text-left rounded-lg border border-surface-200 dark:border-surface-800 hover:border-surface-300 dark:hover:border-surface-700 hover:shadow-sm transition-all bg-white dark:bg-surface-900"
        class:ring-2={selectedId === note.id}
        class:ring-accent={selectedId === note.id}
        onclick={() => handleSelect(note)}
      >
        <div class="font-medium text-sm text-surface-900 dark:text-surface-100 truncate">
          {note.title || 'Untitled'}
        </div>
        <div class="text-xs text-surface-500 dark:text-surface-400 mt-2 line-clamp-3">
          {note.preview || 'No content'}
        </div>
        <div class="text-xs text-surface-400 dark:text-surface-500 mt-2">
          {formatRelativeDate(note.modified_at)}
        </div>
      </button>
    {/each}
  </div>
{/if}

<style>
  .line-clamp-3 {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
