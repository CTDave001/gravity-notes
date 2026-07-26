# Gravity Architecture

This document describes the current application. Files under `docs/plans/` are historical records and may describe behavior that was changed or deferred.

## Product model

Gravity is a local-first desktop notes application:

- Notes are plain Markdown files with no database or account.
- `Ctrl+Alt+N` (`Cmd+Option+N` on macOS) opens quick capture.
- The main window provides a notes list, card view, full-text search, editing, export, settings, and help.
- Individual notes can open in separate windows.
- Closing the main window hides it to the system tray; choosing **Quit** exits the application after pending saves receive a short flush window.

## Runtime architecture

The application is a Tauri 2 desktop binary with a Svelte 5 frontend.

### Frontend

`src/App.svelte` loads settings first, determines the window type from the URL, applies theme/editor preferences, and dynamically imports only the relevant window:

- `MainWindow.svelte` — browsing, search, editing, cards, settings, onboarding, updates, and export.
- `CaptureWindow.svelte` — minimal global-hotkey capture.
- `NoteWindow.svelte` — independent note editing and export handoff.

CodeMirror provides Markdown editing and a deliberately limited set of fenced-code languages. PDF generation and its renderer are dynamically loaded only when needed.

### Rust backend

The Tauri command layer is split by responsibility:

- `storage.rs` — paths, metadata extraction, Unicode-safe previews, filename generation, and atomic writes.
- `commands.rs` — note CRUD, cleanup, full-text search, and image import/storage.
- `export.rs` — Markdown, text, and PDF file export plus reveal-in-folder.
- `clipper.rs` — Windows clipboard HTML-to-Markdown conversion.
- `lib.rs` — application setup, shortcuts, windows, tray behavior, plugins, and command registration.

## Storage

Notes live under the platform app-data directory:

| Platform | Notes directory |
| --- | --- |
| Windows | `%APPDATA%\com.gravity.app\notes\` |
| macOS | `~/Library/Application Support/com.gravity.app/notes/` |
| Linux | `~/.local/share/com.gravity.app/notes/` |

Images are stored in the sibling `images/` directory. Settings use Tauri Store in `settings.json`.

Note filenames contain local date/time plus nanoseconds. Note IDs are validated as single path components before any filesystem operation. Saves write a new temporary file, synchronize it, and rename it into place so an interrupted write does not leave a partially written note.

Unreadable Markdown files are skipped and logged. Cleanup only removes readable, whitespace-only notes older than the configured cleanup age; an unreadable file is never interpreted as empty.

Image imports are limited to 25 MB and the supported extensions `png`, `jpg`, `jpeg`, `gif`, `webp`, and `svg`.

## Autosave and concurrency

Each editing window owns a `DebouncedTaskQueue`:

- Changes are debounced for 200 ms.
- Every queued task captures an immutable note ID and content snapshot.
- Writes are serialized even when an earlier write is still running.
- Switching notes, exporting, closing a window, and tray Quit flush pending content.
- Empty content is saved correctly.
- Failed saves produce a visible retry control and do not silently close a non-empty quick-capture window.

The main and pop-out windows poll the current note every 1.5 seconds for external edits. Polling does not replace content while the user is editing or a save is pending.

## Search

Search is backend-powered and case-insensitive across complete note contents. Results are sorted by modification time and show the first matching line as context. Search currently scans Markdown files directly; there is no persistent index.

## Export

Markdown and plain-text exports are written by the Rust backend. PDF files are rendered in the frontend with jsPDF and offline DejaVu fonts, then atomically written by Rust.

PDF rendering supports Unicode, wrapped prose and code, page breaks inside long code blocks, tables, and unclosed code fences. Export filenames reject path separators, control characters, and Windows-reserved filename characters.

## Security boundaries

- Production Content Security Policy permits only bundled application resources, Tauri IPC, local asset images, blobs, and data images/fonts.
- Development CSP additionally permits the local Vite server and websocket.
- The asset protocol is scoped to the application image directory.
- The frontend does not receive broad filesystem or shell plugin permissions.
- Note IDs and export filenames are validated by Rust rather than trusted from the webview.
- Updater packages require the configured Tauri signing key.

## Updates and releases

The app checks the configured GitHub `latest.json` endpoint. Available updates are shown to the user and are downloaded and installed only after confirmation, followed by an application relaunch.

GitHub Actions builds macOS ARM64/x64, Linux, and Windows artifacts. Tauri updater signatures are separate from Windows Authenticode signing. Windows installers are signed during bundling through Azure Artifact Signing so later signing does not invalidate updater artifacts.

See `docs/RELEASING.md` for the operational procedure.

## Current constraints

- Notes are local to one device; Gravity does not provide sync or collaboration.
- Notes and shortcut locations are fixed; settings currently cover theme, editor typography, onboarding, and default notes-list visibility.
- Clipboard HTML conversion is Windows-only.
- Full-text search is intentionally simple and may need indexing for very large note collections.
- The existing application identifier `com.gravity.app` is retained for installation and data-path compatibility, despite Tauri's macOS naming recommendation.
- CodeMirror remains the largest initial shared frontend dependency; optional PDF and language code is split into separate chunks.
