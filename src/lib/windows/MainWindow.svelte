<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { check } from '@tauri-apps/plugin-updater';
  import { relaunch } from '@tauri-apps/plugin-process';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import NoteList from '../components/NoteList.svelte';
  import SearchBar from '../components/SearchBar.svelte';
  import ExportModal from '../components/ExportModal.svelte';
  import HelpModal from '../components/HelpModal.svelte';
  import TitleBar from '../components/TitleBar.svelte';
  import Toast from '../components/Toast.svelte';
  import CardView from '../components/CardView.svelte';
  import { listNotes, getNote, saveNote, createNote, deleteNote } from '../api';
  import type { NoteMeta, EditorStats } from '../types';

  // Svelte 5 state using $state() rune
  let notes: NoteMeta[] = $state([]);
  let filteredNotes: NoteMeta[] = $state([]);
  let selectedNote: NoteMeta | null = $state(null);
  let content: string = $state('');
  let stats: EditorStats = $state({ wordCount: 0, charCount: 0, line: 1, column: 1 });
  let saved: boolean = $state(true);
  let searchQuery: string = $state('');
  let viewMode: 'list' | 'grid' = $state('list');
  let isAnimating: boolean = $state(false);
  let saveTimeout: ReturnType<typeof setTimeout> | null = $state(null);

  let editor: Editor | undefined = $state();
  let showExportModal: boolean = $state(false);
  let showHelpModal: boolean = $state(false);

  // Toast state
  let showToast: boolean = $state(false);
  let toastFilePath: string = $state('');

  // Sidebar state - visible on app start
  let sidebarVisible: boolean = $state(true);
  let sidebarPinned: boolean = $state(false);
  let editorContainer: HTMLElement | undefined = $state();

  // Sync state for external changes
  let lastSavedContent: string = '';
  let isEditing: boolean = false;
  let editingTimeout: ReturnType<typeof setTimeout> | null = null;
  let pollInterval: ReturnType<typeof setInterval> | null = null;
  let updatingFromExternal: boolean = false;

  // Export from popup window
  let exportNoteTitle: string = $state('');
  let exportNoteContent: string = $state('');
  let unlistenExport: UnlistenFn | null = null;
  let unlistenNotesChanged: UnlistenFn | null = null;

  // Update state
  let updateAvailable: boolean = $state(false);
  let updateVersion: string = $state('');
  let isUpdating: boolean = $state(false);

  const POLL_INTERVAL = 1500;
  const EDGE_ZONE_WIDTH = 20;
  const EDGE_HOVER_DELAY = 350; // ms to hover at edge before showing sidebar
  const CLICK_COOLDOWN = 500; // ms after click before edge hover works

  let edgeHoverTimeout: ReturnType<typeof setTimeout> | null = null;
  let isInEdgeZone = false;
  let isMouseDown = false;
  let recentlyClicked = false;
  let clickCooldownTimeout: ReturnType<typeof setTimeout> | null = null;

  // Sidebar auto-hide logic - only hide when clicking editor, not on hover
  function handleMouseMove(e: MouseEvent) {
    if (sidebarPinned) return;

    // Don't show sidebar while: actively editing, holding mouse, or recently clicked
    if (isEditing || isMouseDown || recentlyClicked) {
      // Clear any pending hover timeout
      if (edgeHoverTimeout) {
        clearTimeout(edgeHoverTimeout);
        edgeHoverTimeout = null;
      }
      isInEdgeZone = false;
      return;
    }

    const inEdge = e.clientX <= EDGE_ZONE_WIDTH;

    // Entering edge zone
    if (inEdge && !isInEdgeZone) {
      isInEdgeZone = true;
      // Start delay timer - only show after sustained hover
      if (!sidebarVisible && !edgeHoverTimeout) {
        edgeHoverTimeout = setTimeout(() => {
          sidebarVisible = true;
          edgeHoverTimeout = null;
        }, EDGE_HOVER_DELAY);
      }
    }
    // Leaving edge zone
    else if (!inEdge && isInEdgeZone) {
      isInEdgeZone = false;
      // Cancel pending show if we leave before delay completes
      if (edgeHoverTimeout) {
        clearTimeout(edgeHoverTimeout);
        edgeHoverTimeout = null;
      }
    }
  }

  function handleMouseDown() {
    isMouseDown = true;
    recentlyClicked = true;
    // Clear any pending hover
    if (edgeHoverTimeout) {
      clearTimeout(edgeHoverTimeout);
      edgeHoverTimeout = null;
    }
  }

  function handleMouseUp() {
    isMouseDown = false;
    // Start cooldown timer
    if (clickCooldownTimeout) clearTimeout(clickCooldownTimeout);
    clickCooldownTimeout = setTimeout(() => {
      recentlyClicked = false;
      clickCooldownTimeout = null;
    }, CLICK_COOLDOWN);
  }

  function handleEditorClick() {
    // Hide sidebar when clicking in editor area (unless pinned)
    if (!sidebarPinned && sidebarVisible) {
      sidebarVisible = false;
    }
  }

  function toggleSidebarPin() {
    sidebarPinned = !sidebarPinned;
    if (sidebarPinned) {
      sidebarVisible = true;
    }
  }

  function showSidebar() {
    sidebarVisible = true;
  }

  async function checkForExternalChanges() {
    if (!selectedNote || isEditing || !saved) return;

    try {
      const currentContent = await getNote(selectedNote.id);

      if (currentContent !== lastSavedContent) {
        content = currentContent;
        lastSavedContent = currentContent;
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

        await loadNotes();
      }
    } catch (err) {
      console.error('Failed to check for changes:', err);
    }
  }

  function startPolling() {
    stopPolling();
    pollInterval = setInterval(checkForExternalChanges, POLL_INTERVAL);
  }

  function stopPolling() {
    if (pollInterval) {
      clearInterval(pollInterval);
      pollInterval = null;
    }
  }

  // Global keyboard shortcuts
  function handleGlobalKeydown(e: KeyboardEvent) {
    // F1 - Help
    if (e.key === 'F1') {
      e.preventDefault();
      showHelpModal = true;
    }
    // Ctrl+N - New note
    if (e.ctrlKey && e.key === 'n' && !e.shiftKey && !e.altKey) {
      e.preventDefault();
      handleNewNote();
    }
    // Ctrl+E - Export (when note selected)
    if (e.ctrlKey && e.key === 'e' && !e.shiftKey && !e.altKey && selectedNote) {
      e.preventDefault();
      showExportModal = true;
    }
  }

  function handleExportSuccess(path: string) {
    toastFilePath = path;
    showToast = true;

    setTimeout(() => {
      showToast = false;
    }, 5000);
  }

  async function checkForUpdates() {
    try {
      const update = await check();
      if (update) {
        updateAvailable = true;
        updateVersion = update.version;
      }
    } catch (err) {
      console.log('Update check failed:', err);
    }
  }

  async function installUpdate() {
    if (isUpdating) return;
    isUpdating = true;
    try {
      const update = await check();
      if (update) {
        await update.downloadAndInstall();
        await relaunch();
      }
    } catch (err) {
      console.error('Update failed:', err);
      isUpdating = false;
    }
  }

  function dismissUpdate() {
    updateAvailable = false;
  }

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
    if (selectedNote && content && !saved) {
      try {
        await saveNote(selectedNote.id, content);
      } catch (err) {
        console.error('Failed to save note:', err);
      }
    }

    stopPolling();

    selectedNote = note;
    try {
      content = await getNote(note.id);
      lastSavedContent = content;
      stats = {
        wordCount: content.trim().split(/\s+/).filter((w) => w.length > 0).length,
        charCount: content.length,
        line: 1,
        column: 1,
      };
      saved = true;
      isEditing = false;

      if (editor) {
        updatingFromExternal = true;
        editor.setContent(content);
      }

      startPolling();
    } catch (err) {
      console.error('Failed to load note:', err);
    }
  }

  async function handleCardSelect(note: NoteMeta) {
    // Switch to list view and select the note
    viewMode = 'list';
    await handleSelectNote(note);
  }

  function handleChange(data: { content: string; stats: EditorStats }) {
    if (updatingFromExternal) {
      updatingFromExternal = false;
      return;
    }

    content = data.content;
    stats = data.stats;
    saved = false;
    isEditing = true;

    if (editingTimeout) clearTimeout(editingTimeout);
    editingTimeout = setTimeout(() => {
      isEditing = false;
    }, 2000);

    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }

    saveTimeout = setTimeout(async () => {
      if (selectedNote) {
        try {
          const updated = await saveNote(selectedNote.id, content);
          lastSavedContent = content;
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

  function handleCursorChange(data: { line: number; column: number }) {
    stats = { ...stats, line: data.line, column: data.column };
  }

  async function handleNewNote() {
    stopPolling();

    try {
      const note = await createNote();
      await loadNotes();
      selectedNote = note;
      content = '';
      lastSavedContent = '';
      stats = { wordCount: 0, charCount: 0, line: 1, column: 1 };
      saved = true;
      isEditing = false;
      if (editor) {
        updatingFromExternal = true;
        editor.setContent('');
        editor.focus();
      }
      startPolling();
    } catch (err) {
      console.error('Failed to create note:', err);
    }
  }

  async function handleDeleteNote() {
    if (selectedNote) {
      stopPolling();

      try {
        await deleteNote(selectedNote.id);
        selectedNote = null;
        content = '';
        lastSavedContent = '';
        stats = { wordCount: 0, charCount: 0, line: 1, column: 1 };
        saved = true;
        isEditing = false;
        await loadNotes();
      } catch (err) {
        console.error('Failed to delete note:', err);
      }
    }
  }

  function toggleViewMode() {
    isAnimating = true;
    viewMode = viewMode === 'list' ? 'grid' : 'list';

    // Reset animation state after transition completes
    setTimeout(() => {
      isAnimating = false;
    }, 300);
  }

  onMount(async () => {
    loadNotes();

    // Check for updates on app start (don't block)
    checkForUpdates();

    // Listen for export requests from popup windows
    unlistenExport = await listen<{ id: string; title: string; content: string }>('export-note', async (event) => {
      const { id, title, content: noteContent } = event.payload;

      // Small delay so user sees the transition
      await new Promise(r => setTimeout(r, 150));

      // Find and select the note in the main window first
      const note = notes.find(n => n.id === id);
      if (note) {
        await handleSelectNote(note);
      }

      // Another small delay before showing export modal
      await new Promise(r => setTimeout(r, 300));

      // Set export data and show modal
      exportNoteTitle = title || 'Untitled';
      exportNoteContent = noteContent || '';
      showExportModal = true;
    });

    // Listen for notes changes from other windows (quick note, popout)
    unlistenNotesChanged = await listen('notes-changed', () => {
      loadNotes();
    });
  });

  onDestroy(() => {
    if (saveTimeout) clearTimeout(saveTimeout);
    if (editingTimeout) clearTimeout(editingTimeout);
    if (edgeHoverTimeout) clearTimeout(edgeHoverTimeout);
    if (clickCooldownTimeout) clearTimeout(clickCooldownTimeout);
    stopPolling();
    if (unlistenExport) unlistenExport();
    if (unlistenNotesChanged) unlistenNotesChanged();
  });
</script>

<svelte:window onkeydown={handleGlobalKeydown} onmousedown={handleMouseDown} onmouseup={handleMouseUp} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="main-window" onmousemove={handleMouseMove}>
  <!-- Update Banner -->
  {#if updateAvailable}
    <div class="update-banner">
      <span class="update-text">
        Version {updateVersion} is available
      </span>
      <div class="update-actions">
        <button class="update-btn" onclick={installUpdate} disabled={isUpdating}>
          {isUpdating ? 'Updating...' : 'Update now'}
        </button>
        <button class="dismiss-btn" onclick={dismissUpdate} aria-label="Dismiss">
          <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>
      </div>
    </div>
  {/if}

  <!-- Title Bar -->
  <TitleBar
    {viewMode}
    onhelp={() => showHelpModal = true}
    ontoggleview={toggleViewMode}
    onnewnote={handleNewNote}
  />

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
              title={sidebarPinned ? 'Unpin sidebar' : 'Pin sidebar'}
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
            <Editor bind:this={editor} {content} onchange={handleChange} oncursorchange={handleCursorChange} />
          </div>
          <!-- Status bar -->
          <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} line={stats.line} column={stats.column} {saved} />
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
</div>

<!-- Export Modal -->
<ExportModal
  show={showExportModal}
  noteTitle={exportNoteTitle || selectedNote?.title || 'Untitled'}
  noteContent={exportNoteContent || content}
  onclose={() => {
    showExportModal = false;
    // Clear external export data
    exportNoteTitle = '';
    exportNoteContent = '';
  }}
  onsuccess={handleExportSuccess}
/>

<!-- Toast notification -->
<Toast
  show={showToast}
  message="File saved to Downloads"
  filePath={toastFilePath}
  onclose={() => showToast = false}
/>

<!-- Help Modal -->
<HelpModal
  show={showHelpModal}
  onclose={() => showHelpModal = false}
/>

<style>
  .main-window {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: var(--bg-primary);
    overflow: hidden;
  }

  .main-content {
    display: flex;
    flex: 1;
    overflow: hidden;
    position: relative;
    transition: all 300ms ease;
  }

  /* Card View Container */
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

  /* Transition for mode switching */
  .main-content.card-mode .sidebar,
  .main-content.card-mode .editor-pane {
    display: none;
  }

  /* Sidebar */
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

  .sidebar-header {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    border-bottom: 1px solid var(--border-color);
  }

  .sidebar-header-top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .sidebar-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .pin-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    border-radius: 6px;
    cursor: pointer;
    transition: all 150ms ease;
  }

  .pin-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }

  .pin-btn.pinned {
    color: var(--accent);
  }

  .pin-btn .icon {
    width: 16px;
    height: 16px;
  }

  .note-list-container {
    flex: 1;
    overflow-y: auto;
  }

  /* Editor pane */
  .editor-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    margin-left: 0;
    transition: margin-left 200ms ease-out, opacity 300ms ease;
    background: var(--bg-primary);
  }

  .editor-pane.sidebar-visible {
    margin-left: 280px;
  }

  .editor-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-primary);
  }

  .note-title {
    font-size: 14px;
    font-weight: 500;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .toolbar-actions {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .toolbar-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 32px;
    border: none;
    background: transparent;
    color: var(--text-muted);
    border-radius: 6px;
    cursor: pointer;
    transition: all 150ms ease;
  }

  .toolbar-btn:hover {
    background: var(--hover-bg);
    color: var(--text-primary);
  }

  .toolbar-btn.delete:hover {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
  }

  .toolbar-btn .icon {
    width: 16px;
    height: 16px;
  }

  .editor-container {
    flex: 1;
    overflow: hidden;
  }

  /* Empty state */
  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-primary);
  }

  .empty-content {
    display: flex;
    flex-direction: column;
    align-items: center;
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
    margin: 0 0 20px 0;
  }

  .create-btn {
    display: inline-flex;
    align-items: center;
    gap: 8px;
    padding: 10px 20px;
    background: var(--accent);
    color: white;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 150ms ease;
  }

  .create-btn:hover {
    filter: brightness(1.1);
    transform: translateY(-1px);
  }

  .create-btn .icon {
    width: 16px;
    height: 16px;
  }

  /* Update Banner */
  .update-banner {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 16px;
    padding: 8px 16px;
    background: var(--accent);
    color: white;
    font-size: 13px;
    flex-shrink: 0;
  }

  .update-text {
    font-weight: 500;
  }

  .update-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .update-btn {
    padding: 4px 12px;
    background: white;
    color: var(--accent);
    border: none;
    border-radius: 4px;
    font-size: 12px;
    font-weight: 600;
    cursor: pointer;
    transition: all 150ms ease;
  }

  .update-btn:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.9);
  }

  .update-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .dismiss-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 24px;
    height: 24px;
    background: transparent;
    border: none;
    color: white;
    opacity: 0.7;
    cursor: pointer;
    border-radius: 4px;
    transition: all 150ms ease;
  }

  .dismiss-btn:hover {
    opacity: 1;
    background: rgba(255, 255, 255, 0.1);
  }
</style>
