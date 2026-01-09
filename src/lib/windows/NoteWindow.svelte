<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
  import { emit } from '@tauri-apps/api/event';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import { getNote, saveNote } from '../api';
  import type { EditorStats } from '../types';

  let currentWindow = getCurrentWindow();

  function startDrag(e: MouseEvent) {
    if (e.button === 0 && !(e.target as HTMLElement).closest('button')) {
      currentWindow.startDragging();
    }
  }

  async function startResize(e: MouseEvent, direction: string) {
    e.preventDefault();
    e.stopPropagation();
    // @ts-ignore
    await currentWindow.startResizeDragging(direction);
  }

  function handleKeydown(e: KeyboardEvent) {
    if ((e.ctrlKey || e.metaKey) && e.key === 'w') {
      e.preventDefault();
      closeWindow();
    }
    if (e.key === 'Escape') {
      e.preventDefault();
      closeWindow();
    }
  }

  // State
  let noteId: string | null = $state(null);
  let noteTitle: string = $state('Untitled');
  let content: string = $state('');
  let stats: EditorStats = $state({ wordCount: 0, charCount: 0, line: 1, column: 1 });
  let saved: boolean = $state(true);
  let loading: boolean = $state(true);
  let error: string | null = $state(null);

  let editor: Editor | undefined = $state();
  let saveTimeout: ReturnType<typeof setTimeout> | null = null;
  let pollInterval: ReturnType<typeof setInterval> | null = null;
  let isEditing: boolean = false;
  let editingTimeout: ReturnType<typeof setTimeout> | null = null;
  let unlisten: (() => void) | null = null;
  let lastSavedContent: string = '';
  let updatingFromExternal: boolean = false;

  const SAVE_DELAY = 200;
  const POLL_INTERVAL = 1500;

  function getNoteIdFromUrl(): string | null {
    const params = new URLSearchParams(window.location.search);
    return params.get('id');
  }

  function extractTitle(text: string): string {
    if (!text.trim()) return 'Untitled';
    const headingMatch = text.match(/^#\s+(.+)$/m);
    if (headingMatch) return headingMatch[1].slice(0, 50);
    const firstLine = text.split('\n')[0].trim();
    return firstLine.slice(0, 50) || 'Untitled';
  }

  async function loadNote() {
    if (!noteId) return;

    try {
      loading = true;
      error = null;
      const noteContent = await getNote(noteId);
      content = noteContent;
      lastSavedContent = noteContent;
      noteTitle = extractTitle(noteContent);
      stats = {
        wordCount: noteContent.trim().split(/\s+/).filter((w: string) => w.length > 0).length,
        charCount: noteContent.length,
        line: 1,
        column: 1,
      };
      saved = true;

      if (editor) {
        updatingFromExternal = true;
        editor.setContent(noteContent);
      }

      document.title = `${noteTitle} - Gravity`;
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to load note';
      console.error('Failed to load note:', err);
    } finally {
      loading = false;
    }
  }

  async function checkForExternalChanges() {
    if (!noteId || isEditing || !saved) return;

    try {
      const currentContent = await getNote(noteId);

      if (currentContent !== lastSavedContent) {
        content = currentContent;
        lastSavedContent = currentContent;
        noteTitle = extractTitle(currentContent);
        stats = {
          wordCount: currentContent.trim().split(/\s+/).filter((w: string) => w.length > 0).length,
          charCount: currentContent.length,
          line: stats.line,
          column: stats.column,
        };

        if (editor) {
          updatingFromExternal = true;
          editor.setContent(currentContent);
        }

        document.title = `${noteTitle} - Gravity`;
      }
    } catch (err) {
      console.error('Failed to check for changes:', err);
    }
  }

  function handleChange(data: { content: string; stats: EditorStats }) {
    // Skip save logic if we're updating from external changes
    if (updatingFromExternal) {
      updatingFromExternal = false;
      return;
    }

    content = data.content;
    stats = data.stats;
    saved = false;
    isEditing = true;

    noteTitle = extractTitle(content);
    document.title = `${noteTitle} - Gravity`;

    if (editingTimeout) clearTimeout(editingTimeout);
    editingTimeout = setTimeout(() => {
      isEditing = false;
    }, 2000);

    if (saveTimeout) clearTimeout(saveTimeout);
    saveTimeout = setTimeout(async () => {
      if (noteId) {
        try {
          await saveNote(noteId, content);
          lastSavedContent = content;
          saved = true;
          // Notify other windows about the update
          await emit('notes-changed');
        } catch (err) {
          console.error('Failed to save note:', err);
        }
      }
    }, SAVE_DELAY);
  }

  function handleCursorChange(data: { line: number; column: number }) {
    stats = { ...stats, line: data.line, column: data.column };
  }

  async function handleClose() {
    if (saveTimeout) clearTimeout(saveTimeout);
    if (noteId && content && !saved) {
      await saveNote(noteId, content);
    }
  }

  async function closeWindow() {
    await handleClose();
    await currentWindow.destroy();
  }

  async function minimizeWindow() {
    await currentWindow.minimize();
  }

  async function handleExport() {
    if (!noteId) return;

    // Save any pending changes first
    if (saveTimeout) clearTimeout(saveTimeout);
    if (!saved && content) {
      await saveNote(noteId, content);
    }

    // Emit event to main window with note info
    await emit('export-note', {
      id: noteId,
      title: noteTitle,
      content: content
    });

    // Focus the main window
    const mainWindow = await WebviewWindow.getByLabel('main');
    if (mainWindow) {
      await mainWindow.setFocus();
    }

    // Close this popout window
    await currentWindow.destroy();
  }

  onMount(async () => {
    document.body.classList.add('transparent');
    window.addEventListener('keydown', handleKeydown);

    noteId = getNoteIdFromUrl();

    if (noteId) {
      await loadNote();
      pollInterval = setInterval(checkForExternalChanges, POLL_INTERVAL);
    } else {
      error = 'No note ID provided';
      loading = false;
    }

    unlisten = await currentWindow.onCloseRequested(async (event) => {
      event.preventDefault();
      await handleClose();
      await currentWindow.destroy();
    });
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleKeydown);
    if (saveTimeout) clearTimeout(saveTimeout);
    if (pollInterval) clearInterval(pollInterval);
    if (editingTimeout) clearTimeout(editingTimeout);
    if (unlisten) unlisten();
  });
</script>

<div class="note-wrapper">
  <!-- Resize handles -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle resize-n" onmousedown={(e) => startResize(e, 'North')}></div>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle resize-s" onmousedown={(e) => startResize(e, 'South')}></div>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle resize-e" onmousedown={(e) => startResize(e, 'East')}></div>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle resize-w" onmousedown={(e) => startResize(e, 'West')}></div>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle resize-nw" onmousedown={(e) => startResize(e, 'NorthWest')}></div>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle resize-ne" onmousedown={(e) => startResize(e, 'NorthEast')}></div>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle resize-sw" onmousedown={(e) => startResize(e, 'SouthWest')}></div>
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="resize-handle resize-se" onmousedown={(e) => startResize(e, 'SouthEast')}></div>

  <div class="note-window">
    <!-- Titlebar -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="titlebar" onmousedown={startDrag}>
      <div class="titlebar-title">{noteTitle}</div>
      <div class="titlebar-controls">
        <button class="control-btn export" onclick={() => handleExport()} aria-label="Export" title="Export note">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5" />
          </svg>
        </button>
        <button class="control-btn" onclick={() => minimizeWindow()} aria-label="Minimize">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
            <path d="M2 6h8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
        <button class="control-btn close" onclick={() => closeWindow()} aria-label="Close">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
            <path d="M2.5 2.5l7 7M9.5 2.5l-7 7" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
          </svg>
        </button>
      </div>
    </div>

    <!-- Content -->
    {#if loading}
      <div class="loading-area">
        <div class="loading-text">Loading...</div>
      </div>
    {:else if error}
      <div class="error-area">
        <div class="error-text">{error}</div>
      </div>
    {:else}
      <div class="editor-area">
        <Editor bind:this={editor} {content} onchange={handleChange} oncursorchange={handleCursorChange} />
      </div>
      <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} line={stats.line} column={stats.column} {saved} />
    {/if}
  </div>
</div>

<style>
  .note-wrapper {
    height: 100vh;
    width: 100vw;
    padding: 8px;
    background: transparent;
    box-sizing: border-box;
    clip-path: inset(0 round 14px);
  }

  .note-window {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
    border-radius: 12px;
    overflow: hidden;
    border: 1px solid var(--border-color);
  }

  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 36px;
    padding: 0 12px;
    background: var(--bg-sidebar);
    border-bottom: 1px solid var(--border-color);
    user-select: none;
    flex-shrink: 0;
  }

  .titlebar-title {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    flex: 1;
    margin-right: 12px;
  }

  .titlebar-controls {
    display: flex;
    align-items: center;
    gap: 2px;
    margin-right: -8px;
  }

  .control-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    cursor: pointer;
    border-radius: 6px;
    transition: all 150ms ease;
  }

  .control-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }

  .control-btn.close:hover {
    background: #e81123;
    color: white;
  }

  .control-btn.export:hover {
    color: var(--accent);
  }

  .control-btn:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }

  .editor-area {
    flex: 1;
    overflow: hidden;
  }

  .loading-area,
  .error-area {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .loading-text {
    color: var(--text-muted);
    font-size: 14px;
  }

  .error-text {
    color: #ef4444;
    font-size: 14px;
  }

  /* Resize handles */
  .resize-handle {
    position: absolute;
    z-index: 100;
  }

  .resize-n {
    top: 0;
    left: 12px;
    right: 12px;
    height: 6px;
    cursor: ns-resize;
  }

  .resize-s {
    bottom: 0;
    left: 12px;
    right: 12px;
    height: 6px;
    cursor: ns-resize;
  }

  .resize-e {
    top: 12px;
    right: 0;
    bottom: 12px;
    width: 6px;
    cursor: ew-resize;
  }

  .resize-w {
    top: 12px;
    left: 0;
    bottom: 12px;
    width: 6px;
    cursor: ew-resize;
  }

  .resize-nw {
    top: 0;
    left: 0;
    width: 12px;
    height: 12px;
    cursor: nwse-resize;
  }

  .resize-ne {
    top: 0;
    right: 0;
    width: 12px;
    height: 12px;
    cursor: nesw-resize;
  }

  .resize-sw {
    bottom: 0;
    left: 0;
    width: 12px;
    height: 12px;
    cursor: nesw-resize;
  }

  .resize-se {
    bottom: 0;
    right: 0;
    width: 12px;
    height: 12px;
    cursor: nwse-resize;
  }
</style>
