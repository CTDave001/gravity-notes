# Card View Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Transform the grid view from a sidebar layout into a full-window card view with smooth animations and hover previews.

**Architecture:** Toggle between two modes - list view (sidebar + editor) and card view (full-width card grid). The sidebar animates to full width, list items transform to cards, and the search bar expands.

**Tech Stack:** Svelte 5 with runes, CSS transitions/transforms, existing CSS custom properties for theming

---

## Task 1: Create CardView Component

**Files:**
- Create: `src/lib/components/CardView.svelte`

**Step 1: Create the CardView component**

```svelte
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
        <button
          class="card"
          class:expanded={hoveredId === note.id}
          onmouseenter={() => hoveredId = note.id}
          onmouseleave={() => hoveredId = null}
          onclick={() => handleSelect(note)}
        >
          <div class="card-header">
            <span class="card-title">{note.title || 'Untitled'}</span>
            <span class="card-date">{formatRelativeDate(note.modified_at)}</span>
          </div>
          <div class="card-preview">
            {note.preview || 'No content'}
          </div>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .card-view {
    display: flex;
    flex-direction: column;
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
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;
    padding: 8px 24px 24px;
    overflow-y: auto;
    flex: 1;
  }

  .card {
    position: relative;
    display: flex;
    flex-direction: column;
    padding: 16px;
    background: var(--bg-surface);
    border: 1px solid var(--border-color);
    border-radius: 12px;
    cursor: pointer;
    text-align: left;
    transition: all 200ms ease;
    min-height: 120px;
    max-height: 140px;
    overflow: hidden;
  }

  .card:hover {
    border-color: var(--text-muted);
    box-shadow: 0 4px 20px var(--shadow-color);
  }

  .card.expanded {
    transform: scale(1.03);
    max-height: 220px;
    z-index: 10;
    box-shadow: 0 8px 32px var(--shadow-color);
  }

  .card:active {
    transform: scale(0.98);
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
    line-height: 1.5;
    overflow: hidden;
    flex: 1;
  }

  .card:not(.expanded) .card-preview {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
  }

  .card.expanded .card-preview {
    -webkit-line-clamp: 8;
    line-clamp: 8;
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
```

**Step 2: Verify TypeScript compiles**

Run: `npm run check`
Expected: No errors related to CardView.svelte

**Step 3: Commit**

```bash
git add src/lib/components/CardView.svelte
git commit -m "feat: add CardView component with expandable cards"
```

---

## Task 2: Update MainWindow for View Mode Switching

**Files:**
- Modify: `src/lib/windows/MainWindow.svelte`

**Step 1: Add imports and state for card view mode**

At the top of the script, add the CardView import:

```typescript
import CardView from '../components/CardView.svelte';
```

**Step 2: Update the template to conditionally render CardView or sidebar+editor**

Replace the main-content div structure. The key change: when `viewMode === 'grid'`, show full-width CardView instead of sidebar+editor.

Find this section (around line 343):
```svelte
<div class="main-content">
```

Replace the entire main-content div with:

```svelte
  <div class="main-content" class:card-mode={viewMode === 'grid'}>
    {#if viewMode === 'grid'}
      <!-- Full-width Card View -->
      <div class="card-view-container">
        <CardView
          notes={filteredNotes}
          selectedId={selectedNote?.id ?? null}
          {searchQuery}
          onselect={handleCardSelect}
          onsearch={handleSearch}
        />
      </div>
    {:else}
      <!-- Sidebar -->
      <div
        class="sidebar"
        class:visible={sidebarVisible || sidebarPinned}
      >
        <!-- Sidebar Header -->
        <div class="sidebar-header">
          <div class="sidebar-header-top">
            <span class="sidebar-title">Notes</span>
            <button
              class="pin-btn"
              class:pinned={sidebarPinned}
              onclick={toggleSidebarPin}
              title={sidebarPinned ? 'Unpin sidebar (Ctrl+B)' : 'Pin sidebar (Ctrl+B)'}
            >
              <svg class="icon" viewBox="0 0 24 24" fill={sidebarPinned ? 'currentColor' : 'none'} stroke="currentColor" stroke-width="2">
                <path d="M12 17v5M9 3h6l1 7h1a2 2 0 0 1 2 2v1H5v-1a2 2 0 0 1 2-2h1l1-7z" />
              </svg>
            </button>
          </div>
          <SearchBar value={searchQuery} oninput={handleSearch} />
        </div>

        <!-- Note list -->
        <div class="note-list-container">
          <NoteList
            notes={filteredNotes}
            selectedId={selectedNote?.id ?? null}
            viewMode="list"
            onselect={handleSelectNote}
          />
        </div>
      </div>

      <!-- Editor pane -->
      <div
        class="editor-pane"
        class:sidebar-visible={sidebarVisible || sidebarPinned}
        bind:this={editorContainer}
      >
        {#if selectedNote}
          <!-- Editor toolbar -->
          <div class="editor-toolbar">
            <div class="note-title">
              {selectedNote.title || 'Untitled'}
            </div>
            <div class="toolbar-actions">
              <button
                class="toolbar-btn"
                onclick={() => showExportModal = true}
                title="Export note (Ctrl+E)"
              >
                <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5" />
                </svg>
              </button>
              <button
                class="toolbar-btn delete"
                onclick={handleDeleteNote}
                title="Delete note"
              >
                <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                </svg>
              </button>
            </div>
          </div>
          <!-- Editor -->
          <div class="editor-container" onclick={handleEditorClick}>
            <Editor bind:this={editor} {content} onchange={handleChange} />
          </div>
          <!-- Status bar -->
          <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} {saved} />
        {:else}
          <!-- Empty state -->
          <div class="empty-state">
            <div class="empty-content">
              <svg class="empty-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
                <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
              </svg>
              <p class="empty-title">No note selected</p>
              <p class="empty-subtitle">Select a note or create a new one</p>
              <button class="create-btn" onclick={handleNewNote}>
                <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                </svg>
                New note
              </button>
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
```

**Step 3: Add handleCardSelect function**

Add this function in the script section (after handleSelectNote):

```typescript
async function handleCardSelect(note: NoteMeta) {
  // Switch to list view and select the note
  viewMode = 'list';
  await handleSelectNote(note);
}
```

**Step 4: Add CSS for card-view-container and transitions**

Add these styles in the style section:

```css
/* Card View Container */
.card-view-container {
  flex: 1;
  display: flex;
  overflow: hidden;
}

/* Transition for mode switching */
.main-content {
  transition: all 300ms ease;
}

.main-content.card-mode .sidebar,
.main-content.card-mode .editor-pane {
  display: none;
}
```

**Step 5: Verify TypeScript compiles**

Run: `npm run check`
Expected: No errors

**Step 6: Commit**

```bash
git add src/lib/windows/MainWindow.svelte
git commit -m "feat: integrate CardView with view mode switching"
```

---

## Task 3: Add Smooth Animation Between Modes

**Files:**
- Modify: `src/lib/windows/MainWindow.svelte`

**Step 1: Add animation state**

Add this state variable after the viewMode declaration:

```typescript
let isAnimating: boolean = $state(false);
```

**Step 2: Update toggleViewMode to trigger animation**

Replace the toggleViewMode function:

```typescript
function toggleViewMode() {
  isAnimating = true;
  viewMode = viewMode === 'list' ? 'grid' : 'list';

  // Reset animation state after transition completes
  setTimeout(() => {
    isAnimating = false;
  }, 300);
}
```

**Step 3: Update CSS for smoother transitions**

Update these CSS rules for better animation:

```css
/* Sidebar with animation */
.sidebar {
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 280px;
  display: flex;
  flex-direction: column;
  background: var(--bg-sidebar);
  border-right: 1px solid var(--border-color);
  box-shadow: 4px 0 24px var(--shadow-color);
  transform: translateX(-100%);
  transition: transform 200ms ease-out, width 300ms ease, opacity 300ms ease;
  z-index: 10;
}

.sidebar.visible {
  transform: translateX(0);
}

/* Card view container animation */
.card-view-container {
  flex: 1;
  display: flex;
  overflow: hidden;
  animation: fadeIn 300ms ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: scale(0.98);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* Editor pane transition */
.editor-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
  margin-left: 0;
  transition: margin-left 200ms ease-out, opacity 300ms ease;
  background: var(--bg-primary);
}
```

**Step 4: Verify TypeScript compiles**

Run: `npm run check`
Expected: No errors

**Step 5: Test visually**

Run: `npm run tauri dev`
- Toggle between list and card view
- Verify smooth fade/scale animation
- Verify cards display correctly
- Verify clicking a card switches back to list view with note open

**Step 6: Commit**

```bash
git add src/lib/windows/MainWindow.svelte
git commit -m "feat: add smooth animation between view modes"
```

---

## Task 4: Polish Card Hover Animation

**Files:**
- Modify: `src/lib/components/CardView.svelte`

**Step 1: Enhance card hover styles**

Update the card styles for smoother, more polished hover effect:

```css
.card {
  position: relative;
  display: flex;
  flex-direction: column;
  padding: 16px;
  background: var(--bg-surface);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  cursor: pointer;
  text-align: left;
  min-height: 120px;
  max-height: 140px;
  overflow: hidden;
  transition:
    transform 200ms cubic-bezier(0.34, 1.56, 0.64, 1),
    max-height 250ms ease,
    box-shadow 200ms ease,
    border-color 150ms ease;
}

.card:hover {
  border-color: var(--accent);
}

.card.expanded {
  transform: scale(1.03) translateY(-4px);
  max-height: 240px;
  z-index: 10;
  box-shadow:
    0 8px 32px var(--shadow-color),
    0 0 0 1px var(--accent);
}

.card:active {
  transform: scale(0.98);
  transition: transform 100ms ease;
}
```

**Step 2: Add subtle background highlight on hover**

```css
.card::before {
  content: '';
  position: absolute;
  inset: 0;
  background: linear-gradient(135deg, var(--accent-muted) 0%, transparent 50%);
  opacity: 0;
  transition: opacity 200ms ease;
  border-radius: 11px;
  pointer-events: none;
}

.card.expanded::before {
  opacity: 1;
}
```

**Step 3: Verify TypeScript compiles**

Run: `npm run check`
Expected: No errors

**Step 4: Test visually**

Run: `npm run tauri dev`
- Hover over cards and verify smooth expansion
- Check the scale, shadow, and border highlight
- Verify the expanded card shows more preview text

**Step 5: Commit**

```bash
git add src/lib/components/CardView.svelte
git commit -m "feat: polish card hover animation with modern effects"
```

---

## Task 5: Final Testing and Cleanup

**Step 1: Full integration test**

Run: `npm run tauri dev`

Test checklist:
- [ ] App starts in list view with sidebar visible
- [ ] Toggle to card view - smooth animation
- [ ] Search works in card view
- [ ] Card hover expands to show more preview
- [ ] Click card - animates back to list view with note open
- [ ] Toggle back to list view - smooth animation
- [ ] Sidebar behavior unchanged in list view
- [ ] New note button works from both views
- [ ] Export works from both views

**Step 2: TypeScript check**

Run: `npm run check`
Expected: No errors (warnings OK)

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat: complete card view redesign with animations"
```

---

## Summary

This implementation:
1. Creates a new `CardView` component with expandable cards
2. Integrates it into `MainWindow` with conditional rendering
3. Adds smooth animations for view mode transitions
4. Polishes card hover effects for a modern feel
5. Maintains all existing functionality (search, select, sidebar behavior)
