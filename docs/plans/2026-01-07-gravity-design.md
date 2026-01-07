# Gravity - Design Document

> Fast capture app with global hotkeys, auto-save architecture, markdown support, polished minimal UI, and one-click PDF export. Part of the Gravity → Orbit → Momentum ecosystem.

## Ecosystem

| App | Purpose | Hotkey |
|-----|---------|--------|
| **Gravity** | Notes capture | `Ctrl+Alt+G` (main), `Ctrl+Alt+N` (new note) |
| **Orbit** | Knowledge/thinking | `Ctrl+Alt+O` (Phase 2) |
| **Momentum** | Tasks/kanban | `Ctrl+Alt+M` (Phase 2) |

---

## Core Interaction

### Two Windows

1. **Capture window** - spawned by `Ctrl+Alt+N`, minimal, just write
2. **Main window** - opened via `Ctrl+Alt+G` or system tray, for search/browse/export

### Capture Window Behavior

- Opens instantly with cursor ready
- Standard window chrome (draggable, resizable)
- Subtle status bar: word count, character count
- Markdown syntax highlighting (subtle, not live preview)
- Close window → note saved automatically
- Empty/whitespace-only → deleted on close
- Startup cleanup: delete empty notes older than 15 minutes

### Hotkeys (defaults, all configurable)

- `Ctrl+Alt+N` → new capture window
- `Ctrl+Alt+G` → open/focus main window

---

## Main Window

### Layout

- Two-pane default: note list (left), editor/preview (right)
- Toggle to card grid view for visual scanning
- Notes ordered by recency (most recent first)

### Left Pane (Note List)

- Each note shows: first line as title (or timestamp if empty), date/time
- Filter-as-you-type search bar at top
- Instant filtering as you type

### Right Pane (Editor/Preview)

- Full markdown editor with syntax highlighting
- Toggle to rendered preview mode
- PDF export button accessible here

### Deep Search

- Triggered by `/` or dedicated shortcut
- Search overlay with content snippets
- Shows matches with context, not just titles

### System Tray

- App lives in tray when closed
- Left-click → open main window
- Right-click → menu (new note, search, quit)

---

## Storage & Files

### Location

- Default: `%APPDATA%/Gravity/notes/`
- User can change in settings
- Changing location auto-migrates all existing notes

### File Format

- Plain markdown files (`.md`)
- Filename: timestamp with milliseconds, e.g., `2024-01-07_14-32-45-127.md`
- No frontmatter required, just content
- Compatible with Orbit later (same folder if desired)

### User Never Sees Files

- No "save as" dialogs
- No file picker for notes
- Notes are notes, not files
- Only exposed when exporting (PDF, markdown, txt)

### Auto-save Architecture

- File created the moment capture window opens
- Content written on every change (throttled ~200ms)
- No "save" button, no "unsaved changes" warning
- Crash-safe: file always exists on disk

---

## PDF Export

### One-click Default

- Export button → PDF generated with default template
- No dialogs for simple case
- Opens file location or preview after export

### Template Presets

- **Academic** - clean serif font, proper margins, title page option
- **Minimal** - simple, modern, no frills
- **Code-focused** - optimized for syntax highlighting, monospace

### Custom Template Option

- Accessible via dropdown or settings
- Configurable: fonts, margins, headers/footers, title page
- Save custom templates for reuse

### Export Formats

- PDF (primary)
- Markdown (raw `.md` file)
- Plain text (`.txt`, strips formatting)

### Export Location

- Default: system Downloads folder
- Option to "export to..." for custom location
- Remember last used location (optional setting)

---

## Visual Design

### Overall Aesthetic

- Custom minimal (Linear/Notion-inspired)
- Clean, modern, polished but not flashy
- Light and dark theme support (follows system or manual toggle)

### Typography

- Clean sans-serif for UI (Inter, SF Pro, or similar)
- Monospace option for editor (user preference)
- Clear hierarchy: titles, body, subtle metadata

### Color Palette

- Neutral base (whites, grays, near-blacks)
- Single accent color for interactive elements
- Minimal use of color - lets content breathe

### Capture Window

- Clean white/dark background
- Editor takes full focus
- Status bar: muted, doesn't compete with content
- No toolbar - just write

### Main Window

- Subtle sidebar/list separation
- Gentle hover states
- Smooth transitions (nothing jarring)
- Cards (in grid view): light shadow, rounded corners, content preview

### Iconography

- Simple, consistent line icons
- Minimal - only where needed
- No emoji, no decoration

---

## Tech Architecture

### Stack

- **Tauri** - Rust backend, web frontend
- **Frontend** - HTML/CSS/JS (or lightweight framework like Svelte/Vue)
- **Storage** - plain markdown files, no database
- **Search index** - lightweight local index for fast search (Phase 2 if needed)

### Startup Behavior

- App launches to system tray (not main window)
- Global hotkeys registered immediately
- Main window opens on demand
- Cold start target: < 500ms

### Performance Targets

- Capture window opens: < 100ms
- Search filters: instant (< 50ms)
- Typing latency: imperceptible
- Memory footprint: < 50MB idle

### Windows Integration

- System tray with icon
- Global hotkeys via Windows API
- Native notifications (optional, for future)
- Proper light/dark mode detection

### File Watching

- Monitor notes folder for external changes
- Sync UI if files added/modified outside app
- Enables Orbit compatibility later

---

## Scope Boundaries

### What's IN v1

- Global hotkeys (new note, open main)
- Capture window with auto-save
- Empty note auto-deletion
- Main window with two-pane + card grid toggle
- Recency-ordered note list
- Filter search + deep search
- Markdown syntax highlighting
- PDF export with presets + custom templates
- System tray integration
- Settings (hotkeys, storage location, theme)

### What's NOT in v1 (Phase 2+)

- AI auto-naming
- AI natural language search
- Tags/folders organization
- Note linking
- Orbit integration
- Cloud sync
- Mobile companion
- Plugins/extensions

### What's NEVER

- Accounts/login
- Collaboration features
- Bloated "workspace" concepts
- Anything that slows down capture
