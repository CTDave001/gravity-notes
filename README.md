# Gravity

**Press a key. Write. Close. Your thought is already saved.**

Most note apps make you think about notes.
Gravity lets you **think instead**.

## How it works

```
Ctrl+Alt+N  →  type anything  →  close  →  done
```

No save button. No file picker. No organizing.
Just capture and move on.

## Why Gravity exists

Every note app eventually becomes a chore:
- "Where should I put this?"
- "What folder?"
- "Should I tag it?"
- "Did I save?"

Gravity removes all of that.
Your notes are plain markdown files on your disk.
You own them forever.

## Features

- **Global hotkey** - `Ctrl+Alt+N` captures thoughts from anywhere
- **Auto-save** - Close the window, it's already saved
- **Local markdown** - Plain `.md` files you control
- **Zero accounts** - No signup, no cloud, no sync
- **Dark mode** - Easy on the eyes
- **Pop-out windows** - Keep notes visible while working
- **Card view** - Visual overview of all your notes
- **Full-text search** - Find text anywhere in any note
- **Image drops** - Paste or drop images directly into notes
- **Flexible editor** - Choose your theme, font, size, and spacing
- **PDF export** - Offline Unicode PDF templates, Markdown, or plain text
- **Signed updates** - Install verified releases from inside the app

## What Gravity is NOT

- Not a "second brain"
- Not a workspace
- Not collaborative
- Not cloud-first
- Not something you organize

**Gravity is for capturing, not curating.**

## Install

### Download
[**Releases page →**](https://github.com/CTDave001/gravity-notes/releases)

### Build from source
```bash
git clone https://github.com/CTDave001/gravity-notes.git
cd gravity-notes
npm ci
npm run tauri build
```

Requires [Node.js 20.19+ or 22.12+](https://nodejs.org/) and [Rust](https://rustup.rs/). Node.js 22 LTS is recommended.

## Your notes live here

| Platform | Location |
|----------|----------|
| Windows | `%APPDATA%\com.gravity.app\notes\` |
| macOS | `~/Library/Application Support/com.gravity.app/notes/` |
| Linux | `~/.local/share/com.gravity.app/notes/` |

Plain markdown. Always accessible. Always yours. The exact path is also shown in **Settings → Notes folder**.

## Keyboard shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Alt+N` | Quick capture (global) |
| `Ctrl+Alt+G` | Focus Gravity |
| `Ctrl+N` | New note |
| `Ctrl+F` | Search all note content |
| `Ctrl+\` | Toggle notes list |
| `Ctrl+Shift+E` | Export note |
| `Ctrl+W` / `Esc` | Close window |
| `Ctrl+B/I/K` | Bold / Italic / Link |

Use `Cmd` and `Option` instead of `Ctrl` and `Alt` on macOS. On Windows, `Ctrl+Alt+V` converts clipboard HTML to Markdown.

## Built with

[Tauri](https://tauri.app) · [Svelte](https://svelte.dev) · [CodeMirror](https://codemirror.net)

## Development

```bash
npm ci
npm run check
npm test
npm run tauri dev
```

On Windows, use the MSVC Rust toolchain. If it is not your default:

```powershell
$env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-msvc"
npm run tauri dev
```

Current technical documentation:

- [Architecture](docs/ARCHITECTURE.md)
- [Development and testing](docs/DEVELOPMENT.md)
- [Release process](docs/RELEASING.md)
- [Changelog](CHANGELOG.md)
- [Security policy](SECURITY.md)

Files under [`docs/plans`](docs/plans) are retained as historical design and implementation records; they are not current instructions.

## License

MIT

---

**If Gravity saves you time, consider giving it a star.**
