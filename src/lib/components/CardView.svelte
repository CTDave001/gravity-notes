<script lang="ts">
  import type { NoteMeta } from '../types';

  let {
    notes = [],
    selectedId = null,
    searchQuery = '',
    onselect,
    onsearch,
  }: {
    notes?: NoteMeta[];
    selectedId?: string | null;
    searchQuery?: string;
    onselect?: (note: NoteMeta) => void;
    onsearch?: (query: string) => void;
  } = $props();

  let hoveredId: string | null = $state(null);

  function formatRelativeDate(dateStr: string): string {
    if (!dateStr) return '';
    const date = new Date(dateStr);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    if (diffMins < 1) return 'Just now';
    if (diffMins < 60) return `${diffMins}m ago`;
    if (diffHours < 24) return `${diffHours}h ago`;
    if (diffDays < 7) return `${diffDays}d ago`;
    return date.toLocaleDateString();
  }

  function handleSelect(note: NoteMeta) {
    onselect?.(note);
  }

  function handleSearchInput(e: Event) {
    const target = e.target as HTMLInputElement;
    onsearch?.(target.value);
  }

  function handleSearchClear() {
    onsearch?.('');
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      handleSearchClear();
    }
  }

  function escapeHtml(text: string): string {
    return text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;')
      .replace(/"/g, '&quot;');
  }

  function highlightText(text: string, query: string): string {
    const escaped = escapeHtml(text);
    if (!query.trim()) return escaped;
    const queryEscaped = query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const regex = new RegExp(`(${queryEscaped})`, 'gi');
    return escaped.replace(regex, '<mark>$1</mark>');
  }
</script>

<div class="card-view">
  <!-- Expanded Search Bar -->
  <div class="search-container">
    <div class="search-bar-expanded">
      <svg class="search-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
      </svg>
      <input
        type="text"
        class="search-input"
        placeholder="Search notes..."
        value={searchQuery}
        oninput={handleSearchInput}
        onkeydown={handleKeydown}
      />
      {#if searchQuery}
        <button class="clear-btn" onclick={handleSearchClear} aria-label="Clear search">
          <svg width="14" height="14" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        </button>
      {/if}
    </div>
  </div>

  <!-- Card Grid -->
  {#if notes.length === 0}
    <div class="empty-state">
      <svg class="empty-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
      </svg>
      <p class="empty-title">No notes found</p>
      <p class="empty-subtitle">Try a different search or create a new note</p>
    </div>
  {:else}
    <div class="card-grid">
      {#each notes as note (note.id)}
        <div class="card-slot">
          <button
            class="card"
            class:expanded={hoveredId === note.id}
            class:selected={selectedId === note.id}
            onmouseenter={() => hoveredId = note.id}
            onmouseleave={() => hoveredId = null}
            onclick={() => handleSelect(note)}
          >
            <div class="card-header">
              <span class="card-title">{@html highlightText(note.title || 'Untitled', searchQuery)}</span>
              <span class="card-date">{formatRelativeDate(note.modified_at)}</span>
            </div>
            <div class="card-preview">
              {@html highlightText(note.preview || 'No content', searchQuery)}
            </div>
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .card-view {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    background: var(--bg-primary);
    overflow: hidden;
  }

  /* Search Container */
  .search-container {
    padding: 24px 24px 16px;
    flex-shrink: 0;
  }

  .search-bar-expanded {
    position: relative;
    display: flex;
    align-items: center;
    max-width: 600px;
    margin: 0 auto;
  }

  .search-icon {
    position: absolute;
    left: 16px;
    width: 20px;
    height: 20px;
    color: var(--text-muted);
    pointer-events: none;
  }

  .search-input {
    width: 100%;
    padding: 14px 44px 14px 48px;
    font-size: 15px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 12px;
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
    right: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 6px;
    border: none;
    background: transparent;
    border-radius: 6px;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 150ms ease;
  }

  .clear-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }

  /* Card Grid */
  .card-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 20px;
    padding: 8px 32px 32px;
    overflow-y: auto;
    flex: 1;
  }

  .card-slot {
    position: relative;
    height: 110px;
  }

  .card {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    display: flex;
    flex-direction: column;
    padding: 18px 20px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 14px;
    cursor: pointer;
    text-align: left;
    min-height: 100%;
    max-height: 110px;
    overflow: hidden;
    transform-origin: center top;
    transition:
      all 350ms cubic-bezier(0.4, 0, 0.2, 1);
  }

  .card:hover {
    border-color: var(--text-muted);
  }

  .card.expanded {
    top: -15px;
    left: -12px;
    right: -12px;
    max-height: 420px;
    z-index: 100;
    border-color: var(--accent);
    box-shadow:
      0 20px 50px -12px rgba(0, 0, 0, 0.3),
      0 0 0 1px var(--accent);
  }

  .card:active {
    transform: scale(0.98);
    transition: transform 100ms ease;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 12px;
    margin-bottom: 8px;
    flex-shrink: 0;
  }

  .card-title {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .card-date {
    font-size: 11px;
    color: var(--text-muted);
    white-space: nowrap;
    flex-shrink: 0;
  }

  .card-preview {
    font-size: 13px;
    color: var(--text-secondary);
    line-height: 1.6;
    overflow: hidden;
    flex: 1;
    white-space: pre-line;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  .card.expanded .card-preview {
    display: block;
    -webkit-line-clamp: unset;
    line-clamp: unset;
    overflow-y: auto;
    max-height: 340px;
    scrollbar-width: thin;
    scrollbar-color: var(--border-color) transparent;
  }

  .card.expanded .card-preview::-webkit-scrollbar {
    width: 6px;
  }

  .card.expanded .card-preview::-webkit-scrollbar-track {
    background: transparent;
  }

  .card.expanded .card-preview::-webkit-scrollbar-thumb {
    background: var(--border-color);
    border-radius: 3px;
  }

  .card.expanded .card-preview::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted);
  }

  .card.selected {
    border-color: var(--accent);
    box-shadow: 0 0 0 2px var(--accent-muted);
  }

  /* Search highlight */
  .card :global(mark) {
    background: var(--accent-muted);
    color: var(--text-primary);
    border-radius: 2px;
    padding: 0 2px;
  }

  /* Empty State */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    padding: 48px;
    text-align: center;
  }

  .empty-icon {
    width: 64px;
    height: 64px;
    color: var(--text-muted);
    opacity: 0.5;
    margin-bottom: 16px;
  }

  .empty-title {
    font-size: 18px;
    font-weight: 500;
    color: var(--text-secondary);
    margin: 0 0 4px 0;
  }

  .empty-subtitle {
    font-size: 14px;
    color: var(--text-muted);
    margin: 0;
  }
</style>
