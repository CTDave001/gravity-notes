<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import { createNote, saveNote, deleteIfEmpty } from '../api';
  import type { NoteMeta, EditorStats } from '../types';

  // Svelte 5 state using $state() rune
  let noteId: string | null = $state(null);
  let content: string = $state('');
  let stats: EditorStats = $state({ wordCount: 0, charCount: 0 });
  let saved: boolean = $state(true);
  let saveTimeout: ReturnType<typeof setTimeout> | null = $state(null);
  let editor: Editor | undefined = $state();
  let unlisten: (() => void) | null = null;

  const SAVE_DELAY = 200; // ms

  async function initNote() {
    try {
      const note = await createNote();
      noteId = note.id;
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
        } catch (err) {
          console.error('Failed to save note:', err);
        }
      }
    }, SAVE_DELAY);
  }

  async function handleClose() {
    // Save any pending changes
    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }

    if (noteId && content) {
      await saveNote(noteId, content);
    }

    // Delete if empty
    if (noteId) {
      await deleteIfEmpty(noteId);
    }
  }

  onMount(async () => {
    await initNote();

    // Listen for window close
    const currentWindow = getCurrentWindow();
    unlisten = await currentWindow.onCloseRequested(async (event) => {
      await handleClose();
    });
  });

  onDestroy(() => {
    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }
    if (unlisten) {
      unlisten();
    }
  });
</script>

<div class="capture-window flex flex-col h-screen bg-white dark:bg-surface-950">
  <div class="flex-1 overflow-hidden">
    <Editor bind:this={editor} {content} autofocus onchange={handleChange} />
  </div>
  <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} {saved} />
</div>

<style>
  .capture-window {
    height: 100vh;
    width: 100vw;
  }
</style>
