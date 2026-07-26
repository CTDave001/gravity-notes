<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { emit, listen } from '@tauri-apps/api/event';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import { DebouncedTaskQueue } from '../autosave';
  import { createNote, saveNote, deleteIfEmpty } from '../api';
  import type { EditorStats, ResizeDirection, SaveStatus } from '../types';

  let currentWindow = getCurrentWindow();

  function startDrag(e: MouseEvent) {
    // Only drag on left mouse button and not on buttons
    if (e.button === 0 && !(e.target as HTMLElement).closest('button')) {
      currentWindow.startDragging();
    }
  }

  // Resize handlers
  async function startResize(e: MouseEvent, direction: ResizeDirection) {
    e.preventDefault();
    e.stopPropagation();
    await currentWindow.startResizeDragging(direction);
  }

  function handleKeydown(e: KeyboardEvent) {
    // Ctrl+W (Windows/Linux) or Cmd+W (Mac) to close
    if ((e.ctrlKey || e.metaKey) && e.key === 'w') {
      e.preventDefault();
      closeWindow();
    }
    // Escape to close (optional, very common pattern)
    if (e.key === 'Escape') {
      e.preventDefault();
      closeWindow();
    }
  }

  // Svelte 5 state using $state() rune
  let noteId: string | null = $state(null);
  let content: string = $state('');
  let stats: EditorStats = $state({ wordCount: 0, charCount: 0, line: 1, column: 1 });
  let saveStatus: SaveStatus = $state('saved');
  let editor: Editor | undefined = $state();
  let unlisten: (() => void) | null = null;
  let unlistenQuit: (() => void) | null = null;
  let lastSavedContent = '';

  const SAVE_DELAY = 200; // ms
  const saver = new DebouncedTaskQueue(SAVE_DELAY);

  async function initNote() {
    saveStatus = 'saving';
    try {
      const note = await createNote();
      noteId = note.id;
      if (content !== lastSavedContent) {
        scheduleSave(note.id, content);
      } else {
        saveStatus = 'saved';
      }
      // Notify other windows about the new note
      await emit('notes-changed');
    } catch (err) {
      saveStatus = 'error';
      console.error('Failed to create note:', err);
    }
  }

  function handleChange(data: { content: string; stats: EditorStats }) {
    content = data.content;
    stats = data.stats;
    saveStatus = noteId ? 'saving' : 'error';

    if (noteId) {
      scheduleSave(noteId, data.content);
    }
  }

  function scheduleSave(id: string, snapshot: string) {
    saver.schedule(() => persistSnapshot(id, snapshot));
  }

  async function persistSnapshot(id: string, snapshot: string) {
    try {
      await saveNote(id, snapshot);
      if (noteId === id && content === snapshot) {
        lastSavedContent = snapshot;
        saveStatus = 'saved';
      }
      await emit('notes-changed');
    } catch (err) {
      if (noteId === id) {
        saveStatus = 'error';
      }
      console.error('Failed to save note:', err);
      throw err;
    }
  }

  async function flushPendingSave() {
    if (!noteId) {
      await saver.drain();
      if (content.trim()) {
        saveStatus = 'error';
        throw new Error('The note has not been created yet');
      }
      return;
    }
    if (content === lastSavedContent && saveStatus === 'saved') {
      await saver.drain();
      return;
    }
    const id = noteId;
    const snapshot = content;
    await saver.flush(() => persistSnapshot(id, snapshot));
  }

  async function retrySave() {
    if (!noteId) {
      await initNote();
      return;
    }
    await flushPendingSave();
  }

  function handleCursorChange(data: { line: number; column: number }) {
    stats = { ...stats, line: data.line, column: data.column };
  }

  async function handleClose() {
    await flushPendingSave();

    // Delete if empty
    if (noteId) {
      await deleteIfEmpty(noteId);
    }
  }

  async function closeWindow() {
    await handleClose();
    await currentWindow.destroy();
  }

  async function minimizeWindow() {
    await currentWindow.minimize();
  }

  onMount(async () => {
    // Set transparent background for capture window
    document.body.classList.add('transparent');

    // Add keyboard shortcut listener
    window.addEventListener('keydown', handleKeydown);

    await initNote();

    // Listen for window close - prevent default to ensure cleanup completes
    unlisten = await currentWindow.onCloseRequested(async (event) => {
      event.preventDefault();
      await handleClose();
      await currentWindow.destroy();
    });
    unlistenQuit = await listen('prepare-to-quit', () => {
      void flushPendingSave();
    });
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleKeydown);
    saver.dispose();
    if (unlisten) {
      unlisten();
    }
    unlistenQuit?.();
  });
</script>

<div class="capture-wrapper">
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

  <div class="capture-window">
    <!-- Compact custom titlebar -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="titlebar" onmousedown={startDrag}>
      <span class="titlebar-title">Quick capture</span>
      <div class="titlebar-controls">
        <button class="titlebar-btn minimize" onclick={() => minimizeWindow()} aria-label="Minimize">
          <span>
            <svg class="icon" viewBox="0 0 8 8" fill="none" stroke="currentColor" stroke-width="1.25">
              <path d="M1.5 4H6.5" />
            </svg>
          </span>
        </button>
        <button class="titlebar-btn close" onclick={() => closeWindow()} aria-label="Close quick capture">
          <span>
            <svg class="icon" viewBox="0 0 8 8" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M1.5 1.5L6.5 6.5M6.5 1.5L1.5 6.5" />
            </svg>
          </span>
        </button>
      </div>
    </div>

    <!-- Editor -->
    <div class="editor-area">
      <Editor bind:this={editor} {content} autofocus onchange={handleChange} oncursorchange={handleCursorChange} />
    </div>

    <!-- Status Bar -->
    <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} line={stats.line} column={stats.column} status={saveStatus} onretry={() => void retrySave()} />
  </div>
</div>

<style>
  .capture-wrapper {
    height: 100vh;
    width: 100vw;
    padding: 8px;
    background: transparent;
    box-sizing: border-box;
    /* Clip to hide any window artifacts at corners */
    clip-path: inset(0 round 14px);
  }

  .capture-window {
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
    background: var(--bg-primary);
    border: 1px solid var(--border-color);
    box-sizing: border-box;
    border-radius: 12px;
    overflow: hidden;
  }

  .titlebar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    height: 32px;
    padding: 0 6px 0 12px;
    background: var(--bg-sidebar);
    border-bottom: 1px solid var(--border-color);
    user-select: none;
    flex-shrink: 0;
  }

  .titlebar-controls {
    display: flex;
    gap: 2px;
  }

  .titlebar-title {
    color: var(--text-muted);
    font-size: 11px;
    font-weight: 500;
    letter-spacing: .01em;
  }

  .titlebar-btn {
    width: 30px;
    height: 25px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    color: var(--text-muted);
    border-radius: 6px;
    cursor: pointer;
    padding: 0;
  }

  .titlebar-btn span {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .titlebar-btn .icon {
    width: 9px;
    height: 9px;
  }

  .titlebar-btn:hover {
    color: var(--text-primary);
    background: var(--hover-bg);
  }

  .titlebar-btn.close:hover {
    color: white;
    background: #e81123;
  }

  .editor-area {
    flex: 1;
    overflow: hidden;
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
