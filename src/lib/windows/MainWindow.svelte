<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import NoteList from '../components/NoteList.svelte';
  import SearchBar from '../components/SearchBar.svelte';
  import ExportModal from '../components/ExportModal.svelte';
  import { listNotes, getNote, saveNote, createNote, deleteNote } from '../api';
  import type { NoteMeta, EditorStats } from '../types';

  // Svelte 5 state using $state() rune
  let notes: NoteMeta[] = $state([]);
  let filteredNotes: NoteMeta[] = $state([]);
  let selectedNote: NoteMeta | null = $state(null);
  let content: string = $state('');
  let stats: EditorStats = $state({ wordCount: 0, charCount: 0 });
  let saved: boolean = $state(true);
  let searchQuery: string = $state('');
  let viewMode: 'list' | 'grid' = $state('list');
  let saveTimeout: ReturnType<typeof setTimeout> | null = $state(null);

  let editor: Editor | undefined = $state();
  let showExportModal: boolean = $state(false);

  const SAVE_DELAY = 200;

  async function loadNotes() {
    try {
      notes = await listNotes();
      filterNotes();
    } catch (err) {
      console.error('Failed to load notes:', err);
    }
  }

  function filterNotes() {
    if (!searchQuery.trim()) {
      filteredNotes = notes;
    } else {
      const query = searchQuery.toLowerCase();
      filteredNotes = notes.filter(
        (note) =>
          note.title.toLowerCase().includes(query) ||
          note.preview.toLowerCase().includes(query)
      );
    }
  }

  async function handleSelectNote(note: NoteMeta) {
    // Save current note first if there's content
    if (selectedNote && content) {
      try {
        await saveNote(selectedNote.id, content);
      } catch (err) {
        console.error('Failed to save note:', err);
      }
    }

    selectedNote = note;
    try {
      content = await getNote(note.id);
      stats = {
        wordCount: content.trim().split(/\s+/).filter((w) => w.length > 0).length,
        charCount: content.length,
      };
      saved = true;

      if (editor) {
        editor.setContent(content);
      }
    } catch (err) {
      console.error('Failed to load note:', err);
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
      if (selectedNote) {
        try {
          const updated = await saveNote(selectedNote.id, content);
          // Preserve original created_at if not in response
          selectedNote = { ...selectedNote, ...updated };
          saved = true;
          await loadNotes();
        } catch (err) {
          console.error('Failed to save note:', err);
        }
      }
    }, SAVE_DELAY);
  }

  function handleSearch(value: string) {
    searchQuery = value;
    filterNotes();
  }

  async function handleNewNote() {
    try {
      const note = await createNote();
      await loadNotes();
      selectedNote = note;
      content = '';
      stats = { wordCount: 0, charCount: 0 };
      if (editor) {
        editor.setContent('');
        editor.focus();
      }
    } catch (err) {
      console.error('Failed to create note:', err);
    }
  }

  async function handleDeleteNote() {
    if (selectedNote) {
      try {
        await deleteNote(selectedNote.id);
        selectedNote = null;
        content = '';
        stats = { wordCount: 0, charCount: 0 };
        await loadNotes();
      } catch (err) {
        console.error('Failed to delete note:', err);
      }
    }
  }

  function toggleViewMode() {
    viewMode = viewMode === 'list' ? 'grid' : 'list';
  }

  onMount(() => {
    loadNotes();
  });

  onDestroy(() => {
    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }
  });
</script>

<div class="main-window flex h-screen bg-white dark:bg-surface-950">
  <!-- Sidebar -->
  <div class="sidebar w-72 flex flex-col border-r border-surface-200 dark:border-surface-800 bg-surface-50 dark:bg-surface-900">
    <!-- Header -->
    <div class="p-3 border-b border-surface-200 dark:border-surface-800">
      <div class="flex items-center justify-between mb-3">
        <h1 class="text-lg font-semibold text-surface-900 dark:text-surface-100">Gravity</h1>
        <div class="flex items-center gap-1">
          <!-- View toggle button -->
          <button
            class="p-2 rounded-lg hover:bg-surface-200 dark:hover:bg-surface-800 text-surface-600 dark:text-surface-400 transition-colors"
            onclick={toggleViewMode}
            title={viewMode === 'list' ? 'Switch to grid view' : 'Switch to list view'}
          >
            {#if viewMode === 'list'}
              <!-- Grid icon -->
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
              </svg>
            {:else}
              <!-- List icon -->
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
              </svg>
            {/if}
          </button>
          <!-- New note button -->
          <button
            class="p-2 rounded-lg hover:bg-surface-200 dark:hover:bg-surface-800 text-surface-600 dark:text-surface-400 transition-colors"
            onclick={handleNewNote}
            title="New note"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
          </button>
        </div>
      </div>
      <!-- Search bar -->
      <SearchBar value={searchQuery} oninput={handleSearch} />
    </div>

    <!-- Note list -->
    <div class="flex-1 overflow-y-auto">
      <NoteList
        notes={filteredNotes}
        selectedId={selectedNote?.id ?? null}
        {viewMode}
        onselect={handleSelectNote}
      />
    </div>
  </div>

  <!-- Editor pane -->
  <div class="flex-1 flex flex-col">
    {#if selectedNote}
      <!-- Editor toolbar -->
      <div class="flex items-center justify-between px-4 py-2 border-b border-surface-200 dark:border-surface-800">
        <div class="text-sm font-medium text-surface-700 dark:text-surface-300 truncate">
          {selectedNote.title || 'Untitled'}
        </div>
        <div class="flex items-center gap-1">
          <button
            class="p-1.5 rounded hover:bg-surface-100 dark:hover:bg-surface-800 text-surface-500 hover:text-surface-700 dark:text-surface-400 dark:hover:text-surface-200 transition-colors"
            onclick={() => showExportModal = true}
            title="Export note"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
            </svg>
          </button>
          <button
            class="p-1.5 rounded hover:bg-red-100 dark:hover:bg-red-900/30 text-surface-500 hover:text-red-600 dark:text-surface-400 dark:hover:text-red-400 transition-colors"
            onclick={handleDeleteNote}
            title="Delete note"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>
      <!-- Editor -->
      <div class="flex-1 overflow-hidden">
        <Editor bind:this={editor} {content} onchange={handleChange} />
      </div>
      <!-- Status bar -->
      <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} {saved} />
    {:else}
      <!-- Empty state -->
      <div class="flex-1 flex items-center justify-center bg-surface-50 dark:bg-surface-950">
        <div class="text-center">
          <svg class="w-16 h-16 mx-auto text-surface-300 dark:text-surface-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <p class="text-lg text-surface-500 dark:text-surface-400 mb-2">No note selected</p>
          <p class="text-sm text-surface-400 dark:text-surface-500 mb-4">Select a note from the sidebar or create a new one</p>
          <button
            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-medium text-white bg-accent hover:bg-accent-hover rounded-lg transition-colors"
            onclick={handleNewNote}
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
            Create a new note
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Export Modal -->
<ExportModal
  show={showExportModal}
  noteTitle={selectedNote?.title ?? 'Untitled'}
  noteContent={content}
  onclose={() => showExportModal = false}
/>
