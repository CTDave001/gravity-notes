# Gravity

**Press a key. Write. Close. Your thought is already saved.**

<!-- TODO: Add a 5-second GIF demo here - this is your #1 conversion tool -->
<!-- Record: Ctrl+Alt+N → type a thought → close window → show it saved -->

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
npm install
npm run tauri build
```

Requires [Node.js 18+](https://nodejs.org/) and [Rust](https://rustup.rs/).

## Your notes live here

| Platform | Location |
|----------|----------|
| Windows | `%APPDATA%\Gravity\notes\` |
| macOS | `~/Library/Application Support/Gravity/notes/` |
| Linux | `~/.local/share/Gravity/notes/` |

Plain markdown. Always accessible. Always yours.

## Keyboard shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Alt+N` | Quick capture (global) |
| `Ctrl+Alt+G` | Focus Gravity |
| `Ctrl+N` | New note |
| `Ctrl+W` / `Esc` | Close window |
| `Ctrl+B/I/K` | Bold / Italic / Link |

## Built with

[Tauri](https://tauri.app) · [Svelte](https://svelte.dev) · [CodeMirror](https://codemirror.net)

## License

MIT

---

**If Gravity saves you time, consider giving it a star.**
