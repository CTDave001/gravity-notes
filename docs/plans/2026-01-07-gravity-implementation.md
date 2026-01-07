# Gravity Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a fast note capture app with global hotkeys, auto-save, markdown support, and PDF export.

**Architecture:** Tauri v2 desktop app with Svelte frontend. Notes stored as plain markdown files in user's app data folder. Two window types: minimal capture window (spawned by hotkey) and main window (for browsing/search/export). System tray for background operation.

**Tech Stack:** Tauri v2, Svelte 5, TypeScript, Rust, CodeMirror 6 (editor), Tailwind CSS (styling)

---

## Phase 1: Foundation

### Task 1.1: Add Required Dependencies

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `package.json`

**Step 1: Add Tauri plugins to Cargo.toml**

Add these dependencies to `src-tauri/Cargo.toml` under `[dependencies]`:

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-fs = "2"
tauri-plugin-global-shortcut = "2"
tauri-plugin-shell = "2"
tauri-plugin-store = "2"
tauri-plugin-log = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
```

**Step 2: Add frontend dependencies**

Run:
```bash
cd /c/Users/David/Desktop/simpletools/gravity-notes
npm add @codemirror/state @codemirror/view @codemirror/lang-markdown @codemirror/language @codemirror/commands @codemirror/theme-one-dark @lezer/highlight
npm add -D tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

**Step 3: Verify dependencies installed**

Run: `npm list @codemirror/view`
Expected: Shows installed version

---

### Task 1.2: Configure Tailwind CSS

**Files:**
- Modify: `tailwind.config.js`
- Modify: `src/app.css`

**Step 1: Configure Tailwind**

Replace `tailwind.config.js`:
```js
/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}', './index.html'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        surface: {
          50: '#fafafa',
          100: '#f4f4f5',
          200: '#e4e4e7',
          300: '#d4d4d8',
          400: '#a1a1aa',
          500: '#71717a',
          600: '#52525b',
          700: '#3f3f46',
          800: '#27272a',
          900: '#18181b',
          950: '#09090b',
        },
        accent: {
          DEFAULT: '#6366f1',
          hover: '#4f46e5',
        }
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Consolas', 'monospace'],
      }
    },
  },
  plugins: [],
}
```

**Step 2: Update app.css**

Replace `src/app.css`:
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  :root {
    --background: 255 255 255;
    --foreground: 9 9 11;
  }

  .dark {
    --background: 9 9 11;
    --foreground: 250 250 250;
  }

  body {
    @apply bg-white dark:bg-surface-950 text-surface-900 dark:text-surface-50;
    font-family: 'Inter', system-ui, sans-serif;
  }
}

/* Editor styling */
.cm-editor {
  height: 100%;
  font-size: 14px;
}

.cm-editor .cm-scroller {
  font-family: 'JetBrains Mono', 'Consolas', monospace;
}

.cm-editor.cm-focused {
  outline: none;
}
```

**Step 3: Commit**

```bash
git init
git add -A
git commit -m "chore: add dependencies and configure tailwind"
```

---

### Task 1.3: Create Storage Service (Rust Backend)

**Files:**
- Create: `src-tauri/src/storage.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Create storage module**

Create `src-tauri/src/storage.rs`:
```rust
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteMeta {
    pub id: String,
    pub path: String,
    pub title: String,
    pub preview: String,
    pub created_at: String,
    pub modified_at: String,
    pub word_count: usize,
    pub char_count: usize,
}

pub fn get_notes_dir(app: &AppHandle) -> PathBuf {
    let app_data = app.path().app_data_dir().expect("Failed to get app data dir");
    app_data.join("notes")
}

pub fn ensure_notes_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let notes_dir = get_notes_dir(app);
    if !notes_dir.exists() {
        fs::create_dir_all(&notes_dir).map_err(|e| e.to_string())?;
    }
    Ok(notes_dir)
}

pub fn generate_note_filename() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d_%H-%M-%S-%3f.md").to_string()
}

pub fn extract_title(content: &str) -> String {
    content
        .lines()
        .find(|line| !line.trim().is_empty())
        .map(|line| {
            let trimmed = line.trim().trim_start_matches('#').trim();
            if trimmed.len() > 50 {
                format!("{}...", &trimmed[..47])
            } else {
                trimmed.to_string()
            }
        })
        .unwrap_or_else(|| "Untitled".to_string())
}

pub fn extract_preview(content: &str) -> String {
    let preview: String = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");

    if preview.len() > 150 {
        format!("{}...", &preview[..147])
    } else {
        preview
    }
}

pub fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn is_note_empty(content: &str) -> bool {
    content.trim().is_empty()
}
```

**Step 2: Update lib.rs to include module**

Add at the top of `src-tauri/src/lib.rs`:
```rust
mod storage;
```

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: add storage module with note utilities"
```

---

### Task 1.4: Create Note Commands (Rust Backend)

**Files:**
- Create: `src-tauri/src/commands.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Create commands module**

Create `src-tauri/src/commands.rs`:
```rust
use crate::storage::{
    self, count_words, ensure_notes_dir, extract_preview, extract_title,
    generate_note_filename, is_note_empty, NoteMeta,
};
use std::fs;
use std::time::UNIX_EPOCH;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn create_note(app: AppHandle) -> Result<NoteMeta, String> {
    let notes_dir = ensure_notes_dir(&app)?;
    let filename = generate_note_filename();
    let path = notes_dir.join(&filename);

    fs::write(&path, "").map_err(|e| e.to_string())?;

    let id = filename.trim_end_matches(".md").to_string();
    let now = chrono::Local::now().to_rfc3339();

    Ok(NoteMeta {
        id,
        path: path.to_string_lossy().to_string(),
        title: "Untitled".to_string(),
        preview: String::new(),
        created_at: now.clone(),
        modified_at: now,
        word_count: 0,
        char_count: 0,
    })
}

#[tauri::command]
pub async fn save_note(app: AppHandle, id: String, content: String) -> Result<NoteMeta, String> {
    let notes_dir = ensure_notes_dir(&app)?;
    let path = notes_dir.join(format!("{}.md", id));

    fs::write(&path, &content).map_err(|e| e.to_string())?;

    let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;
    let modified = metadata
        .modified()
        .map_err(|e| e.to_string())?
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?;

    Ok(NoteMeta {
        id,
        path: path.to_string_lossy().to_string(),
        title: extract_title(&content),
        preview: extract_preview(&content),
        created_at: String::new(), // Will be filled by frontend
        modified_at: chrono::Local::now().to_rfc3339(),
        word_count: count_words(&content),
        char_count: content.chars().count(),
    })
}

#[tauri::command]
pub async fn delete_note(app: AppHandle, id: String) -> Result<(), String> {
    let notes_dir = storage::get_notes_dir(&app);
    let path = notes_dir.join(format!("{}.md", id));

    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_note(app: AppHandle, id: String) -> Result<String, String> {
    let notes_dir = storage::get_notes_dir(&app);
    let path = notes_dir.join(format!("{}.md", id));

    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_notes(app: AppHandle) -> Result<Vec<NoteMeta>, String> {
    let notes_dir = ensure_notes_dir(&app)?;
    let mut notes = Vec::new();

    let entries = fs::read_dir(&notes_dir).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path).unwrap_or_default();
            let metadata = fs::metadata(&path).map_err(|e| e.to_string())?;

            let filename = path.file_stem().unwrap().to_string_lossy().to_string();
            let modified = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| {
                    chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            let created = metadata
                .created()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| {
                    chrono::DateTime::from_timestamp(d.as_secs() as i64, 0)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_default()
                })
                .unwrap_or_default();

            notes.push(NoteMeta {
                id: filename,
                path: path.to_string_lossy().to_string(),
                title: extract_title(&content),
                preview: extract_preview(&content),
                created_at: created,
                modified_at: modified,
                word_count: count_words(&content),
                char_count: content.chars().count(),
            });
        }
    }

    // Sort by modified date, most recent first
    notes.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));

    Ok(notes)
}

#[tauri::command]
pub async fn delete_if_empty(app: AppHandle, id: String) -> Result<bool, String> {
    let notes_dir = storage::get_notes_dir(&app);
    let path = notes_dir.join(format!("{}.md", id));

    if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        if is_note_empty(&content) {
            fs::remove_file(&path).map_err(|e| e.to_string())?;
            return Ok(true);
        }
    }
    Ok(false)
}

#[tauri::command]
pub async fn cleanup_empty_notes(app: AppHandle, max_age_minutes: u64) -> Result<u32, String> {
    let notes_dir = storage::get_notes_dir(&app);
    let mut deleted = 0;

    if !notes_dir.exists() {
        return Ok(0);
    }

    let entries = fs::read_dir(&notes_dir).map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "md") {
            let content = fs::read_to_string(&path).unwrap_or_default();

            if is_note_empty(&content) {
                if let Ok(metadata) = fs::metadata(&path) {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(age) = now.duration_since(modified) {
                            if age.as_secs() > max_age_minutes * 60 {
                                if fs::remove_file(&path).is_ok() {
                                    deleted += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(deleted)
}
```

**Step 2: Register commands in lib.rs**

Replace `src-tauri/src/lib.rs`:
```rust
mod commands;
mod storage;

use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_note,
            save_note,
            delete_note,
            get_note,
            list_notes,
            delete_if_empty,
            cleanup_empty_notes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Step 3: Build to verify**

Run: `cd /c/Users/David/Desktop/simpletools/gravity-notes && npm run tauri build -- --debug 2>&1 | head -50`

Note: First build will take a while to compile Rust dependencies.

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: add Tauri commands for note CRUD operations"
```

---

## Phase 2: Capture Window

### Task 2.1: Create TypeScript Types and API

**Files:**
- Create: `src/lib/types.ts`
- Create: `src/lib/api.ts`

**Step 1: Create types**

Create `src/lib/types.ts`:
```typescript
export interface NoteMeta {
  id: string;
  path: string;
  title: string;
  preview: string;
  created_at: string;
  modified_at: string;
  word_count: number;
  char_count: number;
}

export interface EditorStats {
  wordCount: number;
  charCount: number;
}
```

**Step 2: Create API wrapper**

Create `src/lib/api.ts`:
```typescript
import { invoke } from '@tauri-apps/api/core';
import type { NoteMeta } from './types';

export async function createNote(): Promise<NoteMeta> {
  return invoke('create_note');
}

export async function saveNote(id: string, content: string): Promise<NoteMeta> {
  return invoke('save_note', { id, content });
}

export async function deleteNote(id: string): Promise<void> {
  return invoke('delete_note', { id });
}

export async function getNote(id: string): Promise<string> {
  return invoke('get_note', { id });
}

export async function listNotes(): Promise<NoteMeta[]> {
  return invoke('list_notes');
}

export async function deleteIfEmpty(id: string): Promise<boolean> {
  return invoke('delete_if_empty', { id });
}

export async function cleanupEmptyNotes(maxAgeMinutes: number = 15): Promise<number> {
  return invoke('cleanup_empty_notes', { maxAgeMinutes });
}
```

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: add TypeScript types and Tauri API wrapper"
```

---

### Task 2.2: Create CodeMirror Editor Component

**Files:**
- Create: `src/lib/components/Editor.svelte`

**Step 1: Create Editor component**

Create `src/lib/components/Editor.svelte`:
```svelte
<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { EditorState } from '@codemirror/state';
  import { EditorView, keymap, placeholder } from '@codemirror/view';
  import { markdown } from '@codemirror/lang-markdown';
  import { defaultKeymap, history, historyKeymap } from '@codemirror/commands';
  import { syntaxHighlighting, defaultHighlightStyle } from '@codemirror/language';
  import type { EditorStats } from '../types';

  export let content: string = '';
  export let autofocus: boolean = false;

  const dispatch = createEventDispatcher<{
    change: { content: string; stats: EditorStats };
  }>();

  let editorContainer: HTMLDivElement;
  let view: EditorView;

  function countWords(text: string): number {
    return text.trim().split(/\s+/).filter(w => w.length > 0).length;
  }

  function createEditor() {
    const updateListener = EditorView.updateListener.of((update) => {
      if (update.docChanged) {
        const newContent = update.state.doc.toString();
        dispatch('change', {
          content: newContent,
          stats: {
            wordCount: countWords(newContent),
            charCount: newContent.length,
          },
        });
      }
    });

    const state = EditorState.create({
      doc: content,
      extensions: [
        history(),
        keymap.of([...defaultKeymap, ...historyKeymap]),
        markdown(),
        syntaxHighlighting(defaultHighlightStyle),
        placeholder('Start writing...'),
        updateListener,
        EditorView.lineWrapping,
        EditorView.theme({
          '&': {
            height: '100%',
            fontSize: '15px',
          },
          '.cm-content': {
            fontFamily: "'JetBrains Mono', 'Consolas', monospace",
            padding: '16px',
          },
          '.cm-line': {
            padding: '0 4px',
          },
          '&.cm-focused': {
            outline: 'none',
          },
          '.cm-placeholder': {
            color: '#a1a1aa',
            fontStyle: 'italic',
          },
        }),
      ],
    });

    view = new EditorView({
      state,
      parent: editorContainer,
    });

    if (autofocus) {
      view.focus();
    }
  }

  export function getContent(): string {
    return view?.state.doc.toString() ?? content;
  }

  export function setContent(newContent: string) {
    if (view) {
      view.dispatch({
        changes: {
          from: 0,
          to: view.state.doc.length,
          insert: newContent,
        },
      });
    }
  }

  export function focus() {
    view?.focus();
  }

  onMount(() => {
    createEditor();
  });

  onDestroy(() => {
    view?.destroy();
  });
</script>

<div bind:this={editorContainer} class="editor-container h-full w-full"></div>

<style>
  .editor-container {
    height: 100%;
    width: 100%;
  }

  .editor-container :global(.cm-editor) {
    height: 100%;
  }
</style>
```

**Step 2: Commit**

```bash
git add -A
git commit -m "feat: add CodeMirror editor component"
```

---

### Task 2.3: Create Status Bar Component

**Files:**
- Create: `src/lib/components/StatusBar.svelte`

**Step 1: Create StatusBar component**

Create `src/lib/components/StatusBar.svelte`:
```svelte
<script lang="ts">
  export let wordCount: number = 0;
  export let charCount: number = 0;
  export let saved: boolean = true;
</script>

<div class="status-bar flex items-center justify-between px-4 py-2 text-xs text-surface-500 dark:text-surface-400 bg-surface-100 dark:bg-surface-900 border-t border-surface-200 dark:border-surface-800 select-none">
  <div class="flex items-center gap-4">
    <span>{wordCount} {wordCount === 1 ? 'word' : 'words'}</span>
    <span>{charCount} {charCount === 1 ? 'character' : 'characters'}</span>
  </div>
  <div class="flex items-center gap-2">
    {#if saved}
      <span class="text-green-600 dark:text-green-400">Saved</span>
    {:else}
      <span class="text-amber-600 dark:text-amber-400">Saving...</span>
    {/if}
  </div>
</div>
```

**Step 2: Commit**

```bash
git add -A
git commit -m "feat: add status bar component"
```

---

### Task 2.4: Create Capture Window

**Files:**
- Create: `src/lib/windows/CaptureWindow.svelte`
- Modify: `src/App.svelte`
- Modify: `index.html`

**Step 1: Create CaptureWindow component**

Create `src/lib/windows/CaptureWindow.svelte`:
```svelte
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import { createNote, saveNote, deleteIfEmpty } from '../api';
  import type { NoteMeta, EditorStats } from '../types';

  let noteId: string | null = null;
  let content: string = '';
  let stats: EditorStats = { wordCount: 0, charCount: 0 };
  let saved: boolean = true;
  let saveTimeout: ReturnType<typeof setTimeout> | null = null;
  let editor: Editor;

  const SAVE_DELAY = 200; // ms

  async function initNote() {
    try {
      const note = await createNote();
      noteId = note.id;
    } catch (err) {
      console.error('Failed to create note:', err);
    }
  }

  function handleChange(event: CustomEvent<{ content: string; stats: EditorStats }>) {
    content = event.detail.content;
    stats = event.detail.stats;
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
    const unlisten = await currentWindow.onCloseRequested(async (event) => {
      await handleClose();
    });

    return () => {
      unlisten();
    };
  });

  onDestroy(async () => {
    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }
  });
</script>

<div class="capture-window flex flex-col h-screen bg-white dark:bg-surface-950">
  <div class="flex-1 overflow-hidden">
    <Editor bind:this={editor} {content} autofocus on:change={handleChange} />
  </div>
  <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} {saved} />
</div>

<style>
  .capture-window {
    height: 100vh;
    width: 100vw;
  }
</style>
```

**Step 2: Update App.svelte**

Replace `src/App.svelte`:
```svelte
<script lang="ts">
  import './app.css';
  import CaptureWindow from './lib/windows/CaptureWindow.svelte';

  // For now, default to capture window
  // Later we'll use URL params to determine window type
  let windowType: 'capture' | 'main' = 'capture';
</script>

{#if windowType === 'capture'}
  <CaptureWindow />
{:else}
  <div>Main window coming soon...</div>
{/if}
```

**Step 3: Update index.html title**

Replace `index.html`:
```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Gravity</title>
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;500&display=swap" rel="stylesheet">
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
  </body>
</html>
```

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: add capture window with auto-save"
```

---

### Task 2.5: Test Capture Window

**Step 1: Run dev server**

Run: `cd /c/Users/David/Desktop/simpletools/gravity-notes && npm run tauri dev`

**Step 2: Verify behavior**

- [ ] Window opens
- [ ] Can type in editor
- [ ] Word/char count updates
- [ ] "Saved" indicator appears after typing stops
- [ ] Note file created in app data folder

**Step 3: Commit if working**

```bash
git add -A
git commit -m "test: verify capture window functionality"
```

---

## Phase 3: Main Window

### Task 3.1: Create Note List Component

**Files:**
- Create: `src/lib/components/NoteList.svelte`

**Step 1: Create NoteList component**

Create `src/lib/components/NoteList.svelte`:
```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { NoteMeta } from '../types';

  export let notes: NoteMeta[] = [];
  export let selectedId: string | null = null;
  export let viewMode: 'list' | 'grid' = 'list';

  const dispatch = createEventDispatcher<{
    select: NoteMeta;
  }>();

  function formatDate(dateStr: string): string {
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
    dispatch('select', note);
  }
</script>

{#if viewMode === 'list'}
  <div class="note-list flex flex-col">
    {#each notes as note (note.id)}
      <button
        class="note-item p-3 text-left border-b border-surface-200 dark:border-surface-800 hover:bg-surface-100 dark:hover:bg-surface-900 transition-colors"
        class:bg-accent={selectedId === note.id}
        class:bg-opacity-10={selectedId === note.id}
        on:click={() => handleSelect(note)}
      >
        <div class="font-medium text-sm text-surface-900 dark:text-surface-100 truncate">
          {note.title}
        </div>
        <div class="text-xs text-surface-500 dark:text-surface-400 mt-1 truncate">
          {note.preview || 'No content'}
        </div>
        <div class="text-xs text-surface-400 dark:text-surface-500 mt-1">
          {formatDate(note.modified_at)}
        </div>
      </button>
    {/each}

    {#if notes.length === 0}
      <div class="p-4 text-center text-surface-500 dark:text-surface-400">
        No notes yet
      </div>
    {/if}
  </div>
{:else}
  <div class="note-grid grid grid-cols-2 gap-3 p-3">
    {#each notes as note (note.id)}
      <button
        class="note-card p-4 text-left rounded-lg border border-surface-200 dark:border-surface-800 hover:border-surface-300 dark:hover:border-surface-700 hover:shadow-sm transition-all bg-white dark:bg-surface-900"
        class:ring-2={selectedId === note.id}
        class:ring-accent={selectedId === note.id}
        on:click={() => handleSelect(note)}
      >
        <div class="font-medium text-sm text-surface-900 dark:text-surface-100 truncate">
          {note.title}
        </div>
        <div class="text-xs text-surface-500 dark:text-surface-400 mt-2 line-clamp-3">
          {note.preview || 'No content'}
        </div>
        <div class="text-xs text-surface-400 dark:text-surface-500 mt-2">
          {formatDate(note.modified_at)}
        </div>
      </button>
    {/each}
  </div>
{/if}

<style>
  .line-clamp-3 {
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }
</style>
```

**Step 2: Commit**

```bash
git add -A
git commit -m "feat: add note list component with list/grid views"
```

---

### Task 3.2: Create Search Bar Component

**Files:**
- Create: `src/lib/components/SearchBar.svelte`

**Step 1: Create SearchBar component**

Create `src/lib/components/SearchBar.svelte`:
```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let value: string = '';
  export let placeholder: string = 'Search notes...';

  const dispatch = createEventDispatcher<{
    input: string;
    clear: void;
  }>();

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
    value = target.value;
    dispatch('input', value);
  }

  function handleClear() {
    value = '';
    dispatch('clear');
    dispatch('input', '');
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      handleClear();
    }
  }
</script>

<div class="search-bar relative">
  <svg
    class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-surface-400"
    fill="none"
    stroke="currentColor"
    viewBox="0 0 24 24"
  >
    <path
      stroke-linecap="round"
      stroke-linejoin="round"
      stroke-width="2"
      d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
    />
  </svg>
  <input
    type="text"
    {placeholder}
    {value}
    on:input={handleInput}
    on:keydown={handleKeydown}
    class="w-full pl-10 pr-8 py-2 text-sm bg-surface-100 dark:bg-surface-900 border border-surface-200 dark:border-surface-700 rounded-lg focus:outline-none focus:ring-2 focus:ring-accent focus:border-transparent text-surface-900 dark:text-surface-100 placeholder-surface-400"
  />
  {#if value}
    <button
      class="absolute right-2 top-1/2 -translate-y-1/2 p-1 text-surface-400 hover:text-surface-600 dark:hover:text-surface-300"
      on:click={handleClear}
    >
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
      </svg>
    </button>
  {/if}
</div>
```

**Step 2: Commit**

```bash
git add -A
git commit -m "feat: add search bar component"
```

---

### Task 3.3: Create Main Window

**Files:**
- Create: `src/lib/windows/MainWindow.svelte`

**Step 1: Create MainWindow component**

Create `src/lib/windows/MainWindow.svelte`:
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import Editor from '../components/Editor.svelte';
  import StatusBar from '../components/StatusBar.svelte';
  import NoteList from '../components/NoteList.svelte';
  import SearchBar from '../components/SearchBar.svelte';
  import { listNotes, getNote, saveNote, createNote, deleteNote } from '../api';
  import type { NoteMeta, EditorStats } from '../types';

  let notes: NoteMeta[] = [];
  let filteredNotes: NoteMeta[] = [];
  let selectedNote: NoteMeta | null = null;
  let content: string = '';
  let stats: EditorStats = { wordCount: 0, charCount: 0 };
  let saved: boolean = true;
  let searchQuery: string = '';
  let viewMode: 'list' | 'grid' = 'list';
  let saveTimeout: ReturnType<typeof setTimeout> | null = null;
  let editor: Editor;

  const SAVE_DELAY = 200;

  async function loadNotes() {
    notes = await listNotes();
    filterNotes();
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

  async function handleSelectNote(event: CustomEvent<NoteMeta>) {
    // Save current note first
    if (selectedNote && content) {
      await saveNote(selectedNote.id, content);
    }

    selectedNote = event.detail;
    content = await getNote(selectedNote.id);
    stats = {
      wordCount: content.split(/\s+/).filter((w) => w.length > 0).length,
      charCount: content.length,
    };
    saved = true;

    if (editor) {
      editor.setContent(content);
    }
  }

  function handleChange(event: CustomEvent<{ content: string; stats: EditorStats }>) {
    content = event.detail.content;
    stats = event.detail.stats;
    saved = false;

    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }

    saveTimeout = setTimeout(async () => {
      if (selectedNote) {
        try {
          const updated = await saveNote(selectedNote.id, content);
          selectedNote = { ...selectedNote, ...updated };
          saved = true;
          await loadNotes();
        } catch (err) {
          console.error('Failed to save note:', err);
        }
      }
    }, SAVE_DELAY);
  }

  function handleSearch(event: CustomEvent<string>) {
    searchQuery = event.detail;
    filterNotes();
  }

  async function handleNewNote() {
    const note = await createNote();
    await loadNotes();
    selectedNote = note;
    content = '';
    stats = { wordCount: 0, charCount: 0 };
    if (editor) {
      editor.setContent('');
      editor.focus();
    }
  }

  async function handleDeleteNote() {
    if (selectedNote) {
      await deleteNote(selectedNote.id);
      selectedNote = null;
      content = '';
      stats = { wordCount: 0, charCount: 0 };
      await loadNotes();
    }
  }

  function toggleViewMode() {
    viewMode = viewMode === 'list' ? 'grid' : 'list';
  }

  onMount(() => {
    loadNotes();
  });
</script>

<div class="main-window flex h-screen bg-white dark:bg-surface-950">
  <!-- Sidebar -->
  <div class="sidebar w-72 flex flex-col border-r border-surface-200 dark:border-surface-800">
    <!-- Header -->
    <div class="p-3 border-b border-surface-200 dark:border-surface-800">
      <div class="flex items-center justify-between mb-3">
        <h1 class="text-lg font-semibold text-surface-900 dark:text-surface-100">Gravity</h1>
        <div class="flex items-center gap-1">
          <button
            class="p-2 rounded-lg hover:bg-surface-100 dark:hover:bg-surface-800 text-surface-600 dark:text-surface-400"
            on:click={toggleViewMode}
            title={viewMode === 'list' ? 'Grid view' : 'List view'}
          >
            {#if viewMode === 'list'}
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
              </svg>
            {:else}
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
              </svg>
            {/if}
          </button>
          <button
            class="p-2 rounded-lg hover:bg-surface-100 dark:hover:bg-surface-800 text-surface-600 dark:text-surface-400"
            on:click={handleNewNote}
            title="New note"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
            </svg>
          </button>
        </div>
      </div>
      <SearchBar value={searchQuery} on:input={handleSearch} />
    </div>

    <!-- Note list -->
    <div class="flex-1 overflow-y-auto">
      <NoteList
        notes={filteredNotes}
        selectedId={selectedNote?.id ?? null}
        {viewMode}
        on:select={handleSelectNote}
      />
    </div>
  </div>

  <!-- Editor pane -->
  <div class="flex-1 flex flex-col">
    {#if selectedNote}
      <div class="flex-1 overflow-hidden">
        <Editor bind:this={editor} {content} on:change={handleChange} />
      </div>
      <StatusBar wordCount={stats.wordCount} charCount={stats.charCount} {saved} />
    {:else}
      <div class="flex-1 flex items-center justify-center text-surface-400">
        <div class="text-center">
          <p class="text-lg mb-2">No note selected</p>
          <button
            class="text-accent hover:text-accent-hover"
            on:click={handleNewNote}
          >
            Create a new note
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>
```

**Step 2: Commit**

```bash
git add -A
git commit -m "feat: add main window with two-pane layout"
```

---

### Task 3.4: Add Window Type Routing

**Files:**
- Modify: `src/App.svelte`
- Modify: `src-tauri/tauri.conf.json`

**Step 1: Update App.svelte with URL-based routing**

Replace `src/App.svelte`:
```svelte
<script lang="ts">
  import './app.css';
  import { onMount } from 'svelte';
  import CaptureWindow from './lib/windows/CaptureWindow.svelte';
  import MainWindow from './lib/windows/MainWindow.svelte';
  import { cleanupEmptyNotes } from './lib/api';

  let windowType: 'capture' | 'main' = 'main';

  onMount(async () => {
    // Determine window type from URL params
    const params = new URLSearchParams(window.location.search);
    const type = params.get('window');

    if (type === 'capture') {
      windowType = 'capture';
    } else {
      windowType = 'main';
      // Cleanup empty notes on main window load
      try {
        const deleted = await cleanupEmptyNotes(15);
        if (deleted > 0) {
          console.log(`Cleaned up ${deleted} empty notes`);
        }
      } catch (err) {
        console.error('Failed to cleanup notes:', err);
      }
    }

    // Detect dark mode
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      document.documentElement.classList.add('dark');
    }

    // Listen for theme changes
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
      if (e.matches) {
        document.documentElement.classList.add('dark');
      } else {
        document.documentElement.classList.remove('dark');
      }
    });
  });
</script>

{#if windowType === 'capture'}
  <CaptureWindow />
{:else}
  <MainWindow />
{/if}
```

**Step 2: Update tauri.conf.json window config**

Replace the `app.windows` section in `src-tauri/tauri.conf.json`:
```json
{
  "$schema": "../node_modules/@tauri-apps/cli/config.schema.json",
  "productName": "Gravity",
  "version": "0.1.0",
  "identifier": "com.gravity.app",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "label": "main",
        "title": "Gravity",
        "width": 1000,
        "height": 700,
        "minWidth": 600,
        "minHeight": 400,
        "resizable": true,
        "fullscreen": false,
        "center": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: add window type routing and dark mode detection"
```

---

## Phase 4: System Integration

### Task 4.1: Add Global Shortcut Plugin

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/capabilities/default.json`

**Step 1: Ensure global-shortcut plugin is in Cargo.toml**

Verify `tauri-plugin-global-shortcut = "2"` is in dependencies.

**Step 2: Create capabilities file**

Create `src-tauri/capabilities/default.json`:
```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capability for the main window",
  "windows": ["main", "capture"],
  "permissions": [
    "core:default",
    "core:window:allow-create",
    "core:window:allow-close",
    "core:window:allow-set-focus",
    "fs:default",
    "fs:allow-read",
    "fs:allow-write",
    "fs:allow-exists",
    "fs:allow-mkdir",
    "fs:allow-remove",
    "fs:allow-rename",
    "shell:allow-open",
    "global-shortcut:default",
    "global-shortcut:allow-register",
    "global-shortcut:allow-unregister"
  ]
}
```

**Step 3: Update lib.rs to register global shortcuts**

Update `src-tauri/src/lib.rs`:
```rust
mod commands;
mod storage;

use commands::*;
use tauri::{Emitter, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Register global shortcuts
            let handle = app.handle().clone();

            // Ctrl+Alt+N - New capture window
            let new_note_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyN);

            // Ctrl+Alt+G - Open/focus main window
            let main_window_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::ALT), Code::KeyG);

            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app, shortcut, event| {
                        if event.state() == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                            if shortcut == &new_note_shortcut {
                                // Create new capture window
                                let _ = create_capture_window(app);
                            } else if shortcut == &main_window_shortcut {
                                // Focus main window
                                if let Some(window) = app.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                    })
                    .build(),
            )?;

            let app_handle = app.handle();
            app_handle.global_shortcut().register(new_note_shortcut)?;
            app_handle.global_shortcut().register(main_window_shortcut)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_note,
            save_note,
            delete_note,
            get_note,
            list_notes,
            delete_if_empty,
            cleanup_empty_notes,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_capture_window(app: &tauri::AppHandle) -> Result<(), String> {
    use tauri::WebviewWindowBuilder;
    use tauri::WebviewUrl;

    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S%3f").to_string();
    let label = format!("capture_{}", timestamp);

    WebviewWindowBuilder::new(
        app,
        &label,
        WebviewUrl::App("index.html?window=capture".into()),
    )
    .title("Quick Note")
    .inner_size(500.0, 400.0)
    .min_inner_size(300.0, 200.0)
    .resizable(true)
    .center()
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}
```

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: add global shortcuts for new note and main window"
```

---

### Task 4.2: Add System Tray

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Create: `src-tauri/icons/tray.png` (32x32 icon)

**Step 1: Update lib.rs with system tray**

Add tray setup to the `setup` function in `src-tauri/src/lib.rs` (add after shortcut registration):

```rust
// Add to imports at top
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

// Add in setup function, after global shortcuts:

// System tray
let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
let new_note = MenuItem::with_id(app, "new_note", "New Note", true, None::<&str>)?;
let show = MenuItem::with_id(app, "show", "Show Gravity", true, None::<&str>)?;

let menu = Menu::with_items(app, &[&show, &new_note, &quit])?;

let app_handle = app.handle().clone();
let _tray = TrayIconBuilder::new()
    .icon(app.default_window_icon().unwrap().clone())
    .menu(&menu)
    .on_menu_event(move |app, event| match event.id.as_ref() {
        "quit" => {
            app.exit(0);
        }
        "new_note" => {
            let _ = create_capture_window(app);
        }
        "show" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        _ => {}
    })
    .on_tray_icon_event(|tray, event| {
        if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        } = event
        {
            let app = tray.app_handle();
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    })
    .build(app)?;
```

**Step 2: Commit**

```bash
git add -A
git commit -m "feat: add system tray with menu"
```

---

## Phase 5: PDF Export

### Task 5.1: Add PDF Export Backend

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/export.rs`
- Modify: `src-tauri/src/lib.rs`

**Step 1: Add print-to-pdf capability**

For PDF export, we'll use the browser's print functionality via Tauri. Add to commands:

Create `src-tauri/src/export.rs`:
```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportOptions {
    pub format: String, // "pdf", "md", "txt"
    pub template: String, // "academic", "minimal", "code"
}

#[tauri::command]
pub async fn export_note_file(
    content: String,
    filename: String,
    format: String,
    destination: PathBuf,
) -> Result<String, String> {
    let final_content = match format.as_str() {
        "txt" => {
            // Strip markdown formatting (basic)
            content
                .lines()
                .map(|line| line.trim_start_matches('#').trim())
                .collect::<Vec<_>>()
                .join("\n")
        }
        _ => content, // md keeps as-is
    };

    let ext = match format.as_str() {
        "txt" => "txt",
        _ => "md",
    };

    let output_path = destination.join(format!("{}.{}", filename, ext));
    fs::write(&output_path, final_content).map_err(|e| e.to_string())?;

    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_downloads_dir() -> Result<String, String> {
    dirs::download_dir()
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| "Could not find downloads directory".to_string())
}
```

**Step 2: Add dirs crate to Cargo.toml**

Add: `dirs = "5"`

**Step 3: Update lib.rs**

Add module and register commands:
```rust
mod export;

// Add to invoke_handler:
export::export_note_file,
export::get_downloads_dir,
```

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: add export commands for md/txt formats"
```

---

### Task 5.2: Add Export UI

**Files:**
- Create: `src/lib/components/ExportModal.svelte`
- Modify: `src/lib/windows/MainWindow.svelte`

**Step 1: Create ExportModal component**

Create `src/lib/components/ExportModal.svelte`:
```svelte
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-shell';

  export let show: boolean = false;
  export let noteTitle: string = '';
  export let noteContent: string = '';

  const dispatch = createEventDispatcher<{ close: void }>();

  let format: 'pdf' | 'md' | 'txt' = 'pdf';
  let exporting = false;
  let error = '';

  async function handleExport() {
    exporting = true;
    error = '';

    try {
      const downloadsDir = await invoke<string>('get_downloads_dir');
      const filename = noteTitle.replace(/[^a-z0-9]/gi, '_').toLowerCase() || 'note';

      if (format === 'pdf') {
        // For PDF, we'll use window.print() for now
        // A more sophisticated solution would use a PDF library
        window.print();
      } else {
        const path = await invoke<string>('export_note_file', {
          content: noteContent,
          filename,
          format,
          destination: downloadsDir,
        });

        // Open the folder containing the file
        await open(downloadsDir);
      }

      dispatch('close');
    } catch (err) {
      error = String(err);
    } finally {
      exporting = false;
    }
  }

  function handleClose() {
    dispatch('close');
  }

  function handleBackdropClick(event: MouseEvent) {
    if (event.target === event.currentTarget) {
      handleClose();
    }
  }
</script>

{#if show}
  <div
    class="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
    on:click={handleBackdropClick}
    on:keydown={(e) => e.key === 'Escape' && handleClose()}
    role="dialog"
    tabindex="-1"
  >
    <div class="bg-white dark:bg-surface-900 rounded-lg shadow-xl w-96 p-6">
      <h2 class="text-lg font-semibold text-surface-900 dark:text-surface-100 mb-4">
        Export Note
      </h2>

      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-surface-700 dark:text-surface-300 mb-2">
            Format
          </label>
          <div class="flex gap-2">
            <button
              class="flex-1 py-2 px-3 rounded-lg border text-sm transition-colors"
              class:bg-accent={format === 'pdf'}
              class:text-white={format === 'pdf'}
              class:border-accent={format === 'pdf'}
              class:border-surface-300={format !== 'pdf'}
              class:dark:border-surface-600={format !== 'pdf'}
              on:click={() => (format = 'pdf')}
            >
              PDF
            </button>
            <button
              class="flex-1 py-2 px-3 rounded-lg border text-sm transition-colors"
              class:bg-accent={format === 'md'}
              class:text-white={format === 'md'}
              class:border-accent={format === 'md'}
              class:border-surface-300={format !== 'md'}
              class:dark:border-surface-600={format !== 'md'}
              on:click={() => (format = 'md')}
            >
              Markdown
            </button>
            <button
              class="flex-1 py-2 px-3 rounded-lg border text-sm transition-colors"
              class:bg-accent={format === 'txt'}
              class:text-white={format === 'txt'}
              class:border-accent={format === 'txt'}
              class:border-surface-300={format !== 'txt'}
              class:dark:border-surface-600={format !== 'txt'}
              on:click={() => (format = 'txt')}
            >
              Text
            </button>
          </div>
        </div>

        {#if error}
          <p class="text-sm text-red-500">{error}</p>
        {/if}

        <div class="flex justify-end gap-2 mt-6">
          <button
            class="px-4 py-2 text-sm text-surface-600 dark:text-surface-400 hover:bg-surface-100 dark:hover:bg-surface-800 rounded-lg"
            on:click={handleClose}
          >
            Cancel
          </button>
          <button
            class="px-4 py-2 text-sm bg-accent text-white rounded-lg hover:bg-accent-hover disabled:opacity-50"
            on:click={handleExport}
            disabled={exporting}
          >
            {exporting ? 'Exporting...' : 'Export'}
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}
```

**Step 2: Add export button and modal to MainWindow**

Add to MainWindow.svelte imports:
```svelte
import ExportModal from '../components/ExportModal.svelte';
```

Add state:
```svelte
let showExportModal = false;
```

Add modal at the end of the component:
```svelte
<ExportModal
  show={showExportModal}
  noteTitle={selectedNote?.title ?? ''}
  noteContent={content}
  on:close={() => (showExportModal = false)}
/>
```

Add export button in the editor pane header (you'll need to add a toolbar).

**Step 3: Commit**

```bash
git add -A
git commit -m "feat: add export modal with PDF/MD/TXT options"
```

---

## Phase 6: Polish

### Task 6.1: Add Settings Store

**Files:**
- Create: `src/lib/stores/settings.ts`
- Create: `src/lib/components/SettingsModal.svelte`

**Step 1: Create settings store**

Create `src/lib/stores/settings.ts`:
```typescript
import { writable } from 'svelte/store';
import { Store } from '@tauri-apps/plugin-store';

export interface Settings {
  theme: 'system' | 'light' | 'dark';
  editorFontSize: number;
  newNoteShortcut: string;
  mainWindowShortcut: string;
}

const defaultSettings: Settings = {
  theme: 'system',
  editorFontSize: 15,
  newNoteShortcut: 'Ctrl+Alt+N',
  mainWindowShortcut: 'Ctrl+Alt+G',
};

export const settings = writable<Settings>(defaultSettings);

let store: Store | null = null;

export async function loadSettings() {
  store = await Store.load('settings.json');
  const saved = await store.get<Settings>('settings');
  if (saved) {
    settings.set({ ...defaultSettings, ...saved });
  }
}

export async function saveSettings(newSettings: Settings) {
  settings.set(newSettings);
  if (store) {
    await store.set('settings', newSettings);
    await store.save();
  }
}
```

**Step 2: Commit**

```bash
git add -A
git commit -m "feat: add settings store with persistence"
```

---

### Task 6.2: Final Integration Test

**Step 1: Run full dev build**

```bash
cd /c/Users/David/Desktop/simpletools/gravity-notes
npm run tauri dev
```

**Step 2: Test checklist**

- [ ] App starts to system tray
- [ ] Ctrl+Alt+G opens main window
- [ ] Ctrl+Alt+N opens capture window
- [ ] Typing in capture window auto-saves
- [ ] Closing empty capture window deletes note
- [ ] Notes appear in main window list
- [ ] Search filters notes
- [ ] Grid/list view toggle works
- [ ] Dark mode follows system
- [ ] Export to MD/TXT works

**Step 3: Final commit**

```bash
git add -A
git commit -m "feat: complete Gravity v1 implementation"
```

---

## Summary

This plan covers:

1. **Foundation** - Dependencies, Tailwind, storage service, CRUD commands
2. **Capture Window** - CodeMirror editor, auto-save, status bar
3. **Main Window** - Two-pane layout, note list, search
4. **System Integration** - Global shortcuts, system tray
5. **PDF Export** - Export modal with format options
6. **Polish** - Settings persistence, theming

**Estimated tasks:** ~25 discrete implementation steps
**Key dependencies:** Tauri v2, Svelte 5, CodeMirror 6, Tailwind CSS
