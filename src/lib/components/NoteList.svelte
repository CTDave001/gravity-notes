<script lang="ts">
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
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

  async function openNoteInNewWindow(e: MouseEvent, note: NoteMeta) {
    e.stopPropagation(); // Don't trigger note selection

    try {
      const windowLabel = `note-${note.id}-${Date.now()}`;
      const noteTitle = note.title || 'Untitled';

      // Build URL based on current origin (works in dev and production)
      const baseUrl = window.location.origin;
      const noteUrl = `${baseUrl}?window=note&id=${note.id}`;

      const webview = new WebviewWindow(windowLabel, {
        url: noteUrl,
        title: `${noteTitle} - Gravity`,
        width: 700,
        height: 550,
        minWidth: 300,
        minHeight: 200,
        center: true,
        resizable: true,
        decorations: false,
        transparent: true,
        shadow: false,
        focus: true,
      });

      // Handle window creation errors
      webview.once('tauri://error', (e) => {
        console.error('Failed to create note window:', e);
      });
    } catch (err) {
      console.error('Failed to open note in new window:', err);
    }
  }
</script>

{#if notes.length === 0}
  <div class="empty-state">
    <svg class="empty-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
    </svg>
    <p class="empty-title">No notes yet</p>
    <p class="empty-subtitle">Create a note to get started</p>
  </div>
{:else if viewMode === 'list'}
  <div class="note-list">
    {#each notes as note (note.id)}
      <div class="note-item-wrapper">
        <button
          class="note-item"
          class:selected={selectedId === note.id}
          onclick={() => handleSelect(note)}
        >
          <div class="note-title">
            {note.title || 'Untitled'}
          </div>
          <div class="note-preview">
            {note.preview || 'No content'}
          </div>
          <div class="note-date">
            {formatRelativeDate(note.modified_at)}
          </div>
        </button>
        <!-- Pop-out button -->
        <button
          class="popout-btn"
          onclick={(e) => openNoteInNewWindow(e, note)}
          title="Open in new window"
        >
          <svg width="14" height="14" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
          </svg>
        </button>
      </div>
    {/each}
  </div>
{:else}
  <div class="note-grid">
    {#each notes as note (note.id)}
      <div class="note-card-wrapper">
        <button
          class="note-card"
          class:selected={selectedId === note.id}
          onclick={() => handleSelect(note)}
        >
          <div class="note-title">
            {note.title || 'Untitled'}
          </div>
          <div class="note-preview line-clamp-3">
            {note.preview || 'No content'}
          </div>
          <div class="note-date">
            {formatRelativeDate(note.modified_at)}
          </div>
        </button>
        <!-- Pop-out button -->
        <button
          class="popout-btn"
          onclick={(e) => openNoteInNewWindow(e, note)}
          title="Open in new window"
        >
          <svg width="14" height="14" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
          </svg>
        </button>
      </div>
    {/each}
  </div>
{/if}

<style>
  /* Empty state */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    padding: 32px;
    text-align: center;
  }

  .empty-icon {
    width: 48px;
    height: 48px;
    color: var(--text-muted);
    margin-bottom: 16px;
    opacity: 0.5;
  }

  .empty-title {
    font-size: 14px;
    color: var(--text-secondary);
    margin: 0;
  }

  .empty-subtitle {
    font-size: 12px;
    color: var(--text-muted);
    margin: 4px 0 0 0;
  }

  /* List view */
  .note-list {
    display: flex;
    flex-direction: column;
    padding: 8px;
    gap: 4px;
  }

  .note-item-wrapper {
    position: relative;
  }

  .note-item {
    width: 100%;
    padding: 12px 14px;
    text-align: left;
    background: transparent;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 150ms ease;
  }

  .note-item:hover {
    background: var(--hover-bg);
  }

  .note-item.selected {
    background: var(--accent-muted);
  }

  .note-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    padding-right: 28px;
  }

  .note-preview {
    font-size: 12px;
    color: var(--text-secondary);
    margin-top: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .note-date {
    font-size: 11px;
    color: var(--text-muted);
    margin-top: 6px;
  }

  /* Grid view */
  .note-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
    padding: 12px;
  }

  .note-card-wrapper {
    position: relative;
  }

  .note-card {
    width: 100%;
    padding: 14px;
    text-align: left;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 10px;
    cursor: pointer;
    transition: all 150ms ease;
  }

  .note-card:hover {
    border-color: var(--text-muted);
    box-shadow: 0 2px 8px var(--shadow-color);
  }

  .note-card.selected {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-muted);
  }

  .note-card .note-preview {
    margin-top: 8px;
  }

  .note-card .note-date {
    margin-top: 10px;
  }

  /* Pop-out button */
  .popout-btn {
    position: absolute;
    right: 8px;
    top: 8px;
    padding: 6px;
    border: none;
    background: transparent;
    border-radius: 6px;
    color: var(--text-muted);
    cursor: pointer;
    opacity: 0;
    transition: all 150ms ease;
    z-index: 10;
  }

  .note-item-wrapper:hover .popout-btn,
  .note-card-wrapper:hover .popout-btn {
    opacity: 1;
  }

  .popout-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }

  /* Line clamp utility */
  .line-clamp-3 {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    white-space: normal;
  }
</style>
