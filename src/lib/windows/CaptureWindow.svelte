<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { emit } from '@tauri-apps/api/event';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import { createNote, saveNote, deleteIfEmpty } from '../api';
  import type { NoteMeta, EditorStats } from '../types';

  let currentWindow = getCurrentWindow();

  function startDrag(e: MouseEvent) {
    // Only drag on left mouse button and not on buttons
    if (e.button === 0 && !(e.target as HTMLElement).closest('button')) {
      currentWindow.startDragging();
    }
  }

  // Resize handlers
  async function startResize(e: MouseEvent, direction: string) {
    e.preventDefault();
    e.stopPropagation();
    // @ts-ignore - startResizeDragging is available in Tauri v2
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
  let saved: boolean = $state(true);
  let saveTimeout: ReturnType<typeof setTimeout> | null = $state(null);
  let editor: Editor | undefined = $state();
  let unlisten: (() => void) | null = null;

  const SAVE_DELAY = 200; // ms

  async function initNote() {
    try {
      const note = await createNote();
      noteId = note.id;
      // Notify other windows about the new note
      await emit('notes-changed');
    } catch (err) {
      console.error('Failed to create note:', err);
    }
  }

  function handleChange(data: { content: string; stats: EditorStats }) {
    content = data.content;
    stats = data.stats;
    saved = false;

    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }

    saveTimeout = setTimeout(async () => {
      if (noteId) {
        try {
          await saveNote(noteId, content);
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
    // Save any pending changes
    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }

    if (noteId && content && !saved) {
      await saveNote(noteId, content);
    }

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
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleKeydown);
    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }
    if (unlisten) {
      unlisten();
    }
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
    <!-- Ultra-minimal Titlebar -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="titlebar" onmousedown={startDrag}>
      <div class="titlebar-controls">
        <button class="titlebar-btn close" onclick={() => closeWindow()} aria-label="Close">
          <span class="dot">
            <svg class="icon" viewBox="0 0 8 8" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M1.5 1.5L6.5 6.5M6.5 1.5L1.5 6.5" />
            </svg>
          </span>
        </button>
        <button class="titlebar-btn minimize" onclick={() => minimizeWindow()} aria-label="Minimize">
          <span class="dot">
            <svg class="icon" viewBox="0 0 8 8" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M1.5 4H6.5" />
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
    <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} line={stats.line} column={stats.column} {saved} />
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
    background: white;
    border-radius: 12px;
    overflow: hidden;
  }

  :global(.dark) .capture-window {
    background: #1a1a1e;
  }

  .titlebar {
    display: flex;
    align-items: center;
    height: 32px;
    padding: 0 12px;
    background: transparent;
    user-select: none;
    flex-shrink: 0;
  }

  .titlebar-controls {
    display: flex;
    gap: 7px;
  }

  .titlebar-btn {
    width: 12px;
    height: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    border: none;
    background: transparent;
    border-radius: 50%;
    cursor: pointer;
    padding: 0;
  }

  .titlebar-btn .dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background 0.15s ease;
    background: #e0e0e0;
  }

  .titlebar-btn .dot .icon {
    width: 6px;
    height: 6px;
    opacity: 0;
    transition: opacity 0.15s ease;
    color: rgba(0, 0, 0, 0.5);
  }

  .titlebar-btn:hover .dot .icon {
    opacity: 1;
  }

  .titlebar-btn.close:hover .dot {
    background: #ff5f57;
  }

  .titlebar-btn.close:hover .dot .icon {
    color: rgba(0, 0, 0, 0.4);
  }

  .titlebar-btn.minimize:hover .dot {
    background: #febc2e;
  }

  .titlebar-btn.minimize:hover .dot .icon {
    color: rgba(0, 0, 0, 0.4);
  }

  :global(.dark) .titlebar-btn .dot {
    background: #3a3a3f;
  }

  :global(.dark) .titlebar-btn .dot .icon {
    color: rgba(255, 255, 255, 0.5);
  }

  :global(.dark) .titlebar-btn.close:hover .dot {
    background: #ff5f57;
  }

  :global(.dark) .titlebar-btn.minimize:hover .dot {
    background: #febc2e;
  }

  :global(.dark) .titlebar-btn.close:hover .dot .icon,
  :global(.dark) .titlebar-btn.minimize:hover .dot .icon {
    color: rgba(0, 0, 0, 0.4);
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
