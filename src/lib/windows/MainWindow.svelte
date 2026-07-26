<script lang="ts">
  import { onMount, onDestroy, tick } from 'svelte';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import type { Update } from '@tauri-apps/plugin-updater';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import NoteList from '../components/NoteList.svelte';
  import SearchBar from '../components/SearchBar.svelte';
  import ExportModal from '../components/ExportModal.svelte';
  import HelpModal from '../components/HelpModal.svelte';
  import TitleBar from '../components/TitleBar.svelte';
  import Toast from '../components/Toast.svelte';
  import DeleteToast from '../components/DeleteToast.svelte';
  import CardView from '../components/CardView.svelte';
  import SettingsModal from '../components/SettingsModal.svelte';
  import WelcomeModal from '../components/WelcomeModal.svelte';
  import UpdateToast from '../components/UpdateToast.svelte';
  import MobileHeader from '../components/MobileHeader.svelte';
  import { DebouncedTaskQueue } from '../autosave';
  import { listNotes, searchNotes, getNote, saveNote, createNote, deleteNote } from '../api';
  import { getRuntimeInfo, isIosBuild } from '../platform';
  import { get } from 'svelte/store';
  import { saveSettings, settings, type Settings } from '../stores/settings';
  import type { NoteMeta, EditorStats, SaveStatus } from '../types';

  // Svelte 5 state using $state() rune
  let notes: NoteMeta[] = $state([]);
  let filteredNotes: NoteMeta[] = $state([]);
  let selectedNote: NoteMeta | null = $state(null);
  let content: string = $state('');
  let stats: EditorStats = $state({ wordCount: 0, charCount: 0, line: 1, column: 1 });
  let saveStatus: SaveStatus = $state('saved');
  let searchQuery: string = $state('');
  let viewMode: 'list' | 'grid' = $state('list');
  let isAnimating: boolean = $state(false);

  let editor: Editor | undefined = $state();
  let searchBar: SearchBar | undefined = $state();
  let showExportModal: boolean = $state(false);
  let showHelpModal: boolean = $state(false);
  let showSettingsModal: boolean = $state(false);
  let showWelcomeModal: boolean = $state(!get(settings).onboardingComplete);
  let pendingUpdate: Update | null = $state(null);
  let showUpdateToast = $state(false);
  let updateInstalling = $state(false);
  let updateError = $state('');
  let isMobile = $state(document.documentElement.classList.contains('mobile'));

  // Toast state
  let showToast: boolean = $state(false);
  let toastFilePath: string = $state('');

  // Delete toast state
  let showDeleteToast: boolean = $state(false);
  let deletedNoteTitle: string = $state('');
  let deletedNoteContent: string = $state('');

  let sidebarVisible: boolean = $state(get(settings).sidebarDefaultOpen);

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
  let unlistenQuit: UnlistenFn | null = null;
  let searchRequest = 0;

  
  const POLL_INTERVAL = 1500;
  const SAVE_DELAY = 200;
  const saver = new DebouncedTaskQueue(SAVE_DELAY);

  async function checkForExternalChanges() {
    if (!selectedNote || isEditing || saveStatus !== 'saved') return;

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
    if ((e.ctrlKey || e.metaKey) && e.key === 'n' && !e.shiftKey && !e.altKey) {
      e.preventDefault();
      handleNewNote();
    }
    // Ctrl/Cmd+Shift+E - Export (when note selected)
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'e' && e.shiftKey && !e.altKey && selectedNote) {
      e.preventDefault();
      void openExportModal();
    }
    // Ctrl/Cmd+F - Search all note content
    if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'f' && !e.altKey) {
      e.preventDefault();
      viewMode = 'list';
      sidebarVisible = true;
      void tick().then(() => searchBar?.focus());
    }
    // Ctrl/Cmd+\ - Toggle notes list
    if ((e.ctrlKey || e.metaKey) && e.key === '\\' && !e.altKey) {
      e.preventDefault();
      viewMode = 'list';
      sidebarVisible = !sidebarVisible;
    }
  }

  function handleExportSuccess(path: string) {
    toastFilePath = path;
    showToast = true;

    setTimeout(() => {
      showToast = false;
    }, 5000);
  }

  async function checkForUpdate() {
    if (isIosBuild || isMobile) return;

    try {
      const { check } = await import('@tauri-apps/plugin-updater');
      pendingUpdate = await check({ timeout: 30_000 });
      if (pendingUpdate) {
        showUpdateToast = true;
      }
    } catch (err) {
      console.warn('Update check failed:', err);
    }
  }

  async function installUpdate() {
    if (isIosBuild || !pendingUpdate) return;
    updateInstalling = true;
    updateError = '';
    try {
      await pendingUpdate.downloadAndInstall();
      const { relaunch } = await import('@tauri-apps/plugin-process');
      await relaunch();
    } catch (err) {
      updateError = err instanceof Error ? err.message : 'Could not install the update.';
      updateInstalling = false;
    }
  }

  async function loadNotes() {
    const request = ++searchRequest;
    try {
      const result = searchQuery.trim()
        ? await searchNotes(searchQuery.trim())
        : await listNotes();
      if (request !== searchRequest) return;
      notes = result;
      filteredNotes = result;
    } catch (err) {
      console.error('Failed to load notes:', err);
    }
  }

  async function handleSelectNote(note: NoteMeta) {
    try {
      await flushSelectedNote();
    } catch {
      return;
    }

    stopPolling();

    try {
      const nextContent = await getNote(note.id);
      selectedNote = note;
      content = nextContent;
      lastSavedContent = content;
      stats = {
        wordCount: content.trim().split(/\s+/).filter((w) => w.length > 0).length,
        charCount: content.length,
        line: 1,
        column: 1,
      };
      saveStatus = 'saved';
      isEditing = false;

      if (editor) {
        updatingFromExternal = true;
        editor.setContent(content);
      }

      if (isMobile) {
        sidebarVisible = false;
      }
      startPolling();
    } catch (err) {
      console.error('Failed to load note:', err);
      if (selectedNote) {
        startPolling();
      }
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
    saveStatus = 'saving';
    isEditing = true;

    if (editingTimeout) clearTimeout(editingTimeout);
    editingTimeout = setTimeout(() => {
      isEditing = false;
    }, 2000);

    if (selectedNote) {
      const id = selectedNote.id;
      const snapshot = data.content;
      saver.schedule(() => persistSnapshot(id, snapshot));
    }
  }

  async function persistSnapshot(id: string, snapshot: string) {
    try {
      const updated = await saveNote(id, snapshot);
      if (selectedNote?.id === id) {
        selectedNote = { ...selectedNote, ...updated };
        if (content === snapshot) {
          lastSavedContent = snapshot;
          saveStatus = 'saved';
        }
      }
      await loadNotes();
    } catch (err) {
      if (selectedNote?.id === id) {
        saveStatus = 'error';
      }
      console.error('Failed to save note:', err);
      throw err;
    }
  }

  async function flushSelectedNote() {
    if (!selectedNote) {
      await saver.drain();
      return;
    }

    const id = selectedNote.id;
    const snapshot = content;
    if (snapshot === lastSavedContent && saveStatus === 'saved') {
      await saver.drain();
      return;
    }

    await saver.flush(() => persistSnapshot(id, snapshot));
  }

  function handleSearch(value: string) {
    searchQuery = value;
    void loadNotes();
  }

  function handleCursorChange(data: { line: number; column: number }) {
    stats = { ...stats, line: data.line, column: data.column };
  }

  async function handleNewNote() {
    try {
      await flushSelectedNote();
      const note = await createNote();
      stopPolling();
      await loadNotes();
      selectedNote = note;
      content = '';
      lastSavedContent = '';
      stats = { wordCount: 0, charCount: 0, line: 1, column: 1 };
      saveStatus = 'saved';
      isEditing = false;
      if (editor) {
        updatingFromExternal = true;
        editor.setContent('');
        editor.focus();
      }
      if (isMobile) {
        sidebarVisible = false;
      }
      startPolling();
    } catch (err) {
      console.error('Failed to create note:', err);
      if (selectedNote) {
        startPolling();
      }
    }
  }

  async function handleDeleteNote() {
    if (selectedNote) {
      stopPolling();
      saver.cancelPending();
      await saver.drain();

      // Store note data for potential undo
      deletedNoteTitle = selectedNote.title || 'Untitled';
      deletedNoteContent = content;

      try {
        await deleteNote(selectedNote.id);
        selectedNote = null;
        content = '';
        lastSavedContent = '';
        stats = { wordCount: 0, charCount: 0, line: 1, column: 1 };
        saveStatus = 'saved';
        isEditing = false;
        await loadNotes();
        if (isMobile) {
          sidebarVisible = true;
        }

        // Show delete toast with undo option
        showDeleteToast = true;
      } catch (err) {
        console.error('Failed to delete note:', err);
      }
    }
  }

  async function handleUndoDelete() {
    showDeleteToast = false;

    if (deletedNoteContent) {
      try {
        // Create a new note with the deleted content
        const newNote = await createNote();
        await saveNote(newNote.id, deletedNoteContent);
        await loadNotes();

        // Select the restored note
        const restoredNote = notes.find(n => n.id === newNote.id);
        if (restoredNote) {
          await handleSelectNote(restoredNote);
        }
      } catch (err) {
        console.error('Failed to restore note:', err);
      }
    }

    // Clear deleted note data
    deletedNoteTitle = '';
    deletedNoteContent = '';
  }

  function toggleViewMode() {
    if (isMobile) return;

    isAnimating = true;
    viewMode = viewMode === 'list' ? 'grid' : 'list';

    // Reset animation state after transition completes
    setTimeout(() => {
      isAnimating = false;
    }, 300);
  }

  function handleSettingsSaved(value: Settings) {
    if (!isMobile) {
      sidebarVisible = value.sidebarDefaultOpen;
    }
  }

  async function completeOnboarding() {
    const current = get(settings);
    showWelcomeModal = false;
    void saveSettings({ ...current, onboardingComplete: true }).catch((error) => {
      console.error('Failed to save onboarding status:', error);
    });

    if (!selectedNote && notes.length === 0) {
      await handleNewNote();
    }
  }

  async function openExportModal() {
    try {
      await flushSelectedNote();
      showExportModal = true;
    } catch {
      // The status bar exposes the save failure; keep the user in the editor.
    }
  }

  onMount(async () => {
    const runtime = await getRuntimeInfo();
    isMobile = runtime.mobile;
    if (isMobile) {
      viewMode = 'list';
      sidebarVisible = true;
    }

    await loadNotes();

    if (!isIosBuild && !isMobile) {
      void checkForUpdate();
    }

    // Listen for export requests from popup windows
    unlistenExport = await listen<{ id: string; title: string; content: string }>('export-note', async (event) => {
      const { id, title, content: noteContent } = event.payload;

      // Small delay so user sees the transition
      await new Promise(r => setTimeout(r, 150));

      // Find and select the note in the main window first
      searchQuery = '';
      await loadNotes();
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

    unlistenQuit = await listen('prepare-to-quit', () => {
      void flushSelectedNote();
    });
  });

  onDestroy(() => {
    saver.dispose();
    if (editingTimeout) clearTimeout(editingTimeout);
    stopPolling();
    if (unlistenExport) unlistenExport();
    if (unlistenNotesChanged) unlistenNotesChanged();
    if (unlistenQuit) unlistenQuit();
  });
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class="main-window" class:mobile={isMobile}>
  <!-- Title Bar -->
  {#if isMobile}
    <MobileHeader
      title={selectedNote?.title || 'Untitled'}
      showingList={sidebarVisible}
      onnotes={() => sidebarVisible = true}
      onsettings={() => showSettingsModal = true}
      onnewnote={handleNewNote}
    />
  {:else}
    <TitleBar
      {viewMode}
      {sidebarVisible}
      onhelp={() => showHelpModal = true}
      onsettings={() => showSettingsModal = true}
      onsidebar={() => sidebarVisible = !sidebarVisible}
      ontoggleview={toggleViewMode}
      onnewnote={handleNewNote}
    />
  {/if}

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
        class:visible={sidebarVisible}
      >
        <!-- Sidebar Header -->
        <div class="sidebar-header">
          <div class="sidebar-header-top">
            <span class="sidebar-title">Notes</span>
            <button class="pin-btn" onclick={() => sidebarVisible = false} title="Hide notes list" aria-label="Hide notes list">
              <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" d="M6 6l12 12M18 6L6 18" />
              </svg>
            </button>
          </div>
          <SearchBar bind:this={searchBar} value={searchQuery} oninput={handleSearch} />
        </div>

        <!-- Note list -->
        <div class="note-list-container">
          <NoteList
            notes={filteredNotes}
            selectedId={selectedNote?.id ?? null}
            viewMode="list"
            allowPopout={!isMobile}
            onselect={handleSelectNote}
          />
        </div>
      </div>

      <!-- Editor pane -->
      <div
        class="editor-pane"
        class:sidebar-visible={sidebarVisible}
      >
        {#if selectedNote}
          <!-- Editor toolbar -->
          <div class="editor-toolbar">
            <div class="toolbar-actions">
              <button
                class="toolbar-btn"
                onclick={openExportModal}
                title="Export note (Ctrl+Shift+E)"
                aria-label="Export note"
              >
                <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5" />
                </svg>
              </button>
              <button
                class="toolbar-btn delete"
                onclick={handleDeleteNote}
                title="Delete note"
                aria-label="Delete note"
              >
                <svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
                </svg>
              </button>
            </div>
          </div>
          <!-- Editor -->
          <div class="editor-container">
            <Editor bind:this={editor} {content} onchange={handleChange} oncursorchange={handleCursorChange} />
          </div>
          <!-- Status bar -->
          <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} line={stats.line} column={stats.column} status={saveStatus} onretry={() => void flushSelectedNote()} />
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
              <p class="shortcut-hint">
                {isMobile ? 'Tap + to capture your first thought.' : 'Tip: use Ctrl+Alt+N for quick capture from anywhere.'}
              </p>
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
  mobile={isMobile}
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
  message={isMobile ? 'File exported' : 'File saved to Downloads'}
  filePath={toastFilePath}
  canReveal={!isMobile}
  onclose={() => showToast = false}
/>

<!-- Delete toast with undo -->
<DeleteToast
  show={showDeleteToast}
  noteTitle={deletedNoteTitle}
  onundo={handleUndoDelete}
  onclose={() => { showDeleteToast = false; deletedNoteTitle = ''; deletedNoteContent = ''; }}
/>

<!-- Help Modal -->
<HelpModal
  show={showHelpModal}
  onclose={() => showHelpModal = false}
/>

<SettingsModal
  show={showSettingsModal}
  mobile={isMobile}
  onclose={() => showSettingsModal = false}
  onsave={handleSettingsSaved}
/>

<WelcomeModal show={showWelcomeModal} mobile={isMobile} oncomplete={completeOnboarding} />

{#if !isIosBuild && !isMobile}
  <UpdateToast
    show={showUpdateToast}
    version={pendingUpdate?.version ?? ''}
    installing={updateInstalling}
    error={updateError}
    oninstall={installUpdate}
    ondismiss={() => showUpdateToast = false}
  />
{/if}

<style>
  .main-window {
    display: flex;
    flex-direction: column;
    height: 100vh;
    height: 100dvh;
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
    justify-content: flex-end;
    padding: 8px 16px;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-primary);
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

  .shortcut-hint {
    margin: 18px 0 0;
    color: var(--text-muted);
    font-size: 12px;
  }

  @media (max-width: 720px) {
    .sidebar {
      width: min(86vw, 300px);
      box-shadow: 10px 0 40px rgb(0 0 0 / .25);
    }

    .editor-pane.sidebar-visible {
      margin-left: 0;
    }
  }

  .main-window.mobile .main-content {
    min-height: 0;
  }

  .main-window.mobile .sidebar {
    width: 100%;
    border-right: 0;
    box-shadow: none;
    z-index: 15;
  }

  .main-window.mobile .sidebar-header {
    padding: 14px 16px 12px;
  }

  .main-window.mobile .pin-btn {
    display: none;
  }

  .main-window.mobile .note-list-container {
    padding-bottom: max(16px, env(safe-area-inset-bottom));
  }

  .main-window.mobile .editor-pane,
  .main-window.mobile .editor-pane.sidebar-visible {
    margin-left: 0;
    min-width: 0;
  }

  .main-window.mobile .editor-pane.sidebar-visible {
    visibility: hidden;
    pointer-events: none;
  }

  .main-window.mobile .editor-toolbar {
    min-height: 48px;
    padding: 2px 10px;
  }

  .main-window.mobile .toolbar-btn {
    width: 44px;
    height: 44px;
    border-radius: 12px;
  }

  .main-window.mobile .toolbar-btn .icon {
    width: 20px;
    height: 20px;
  }

  .main-window.mobile .empty-content {
    padding: 24px;
  }

  </style>
