# Gravity

A minimal, beautiful note-taking app that stays out of your way.

![Gravity Notes](https://img.shields.io/badge/version-1.0.0-blue)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- **Distraction-free writing** - Clean, minimal interface focused on your content
- **Quick capture** - Global hotkey (`Ctrl+Shift+N`) to instantly capture thoughts
- **Pop-out notes** - Open notes in separate floating windows
- **Auto-save** - Never lose your work with automatic saving
- **Live sync** - Changes sync instantly across all open windows
- **Markdown support** - Write in Markdown with syntax highlighting
- **Dark mode** - Easy on the eyes with automatic theme support
- **Card view** - Visual overview of all your notes with expandable previews
- **Search** - Find notes quickly with instant search
- **Export** - Export notes as Markdown or PDF
- **Minimal sidebar** - Auto-hiding sidebar that appears on hover
- **Cross-platform** - Works on Windows, macOS, and Linux

## Installation

### Download

Download the latest release for your platform from the [Releases](https://github.com/CTDave001/gravity-notes/releases) page:

- **Windows**: `Gravity_x.x.x_x64-setup.exe` or `.msi`
- **macOS**: `Gravity_x.x.x_aarch64.dmg` (Apple Silicon) or `Gravity_x.x.x_x64.dmg` (Intel)
- **Linux**: `.deb`, `.rpm`, or `.AppImage`

### Build from Source

Requirements:
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.70+
- [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)

```bash
# Clone the repository
git clone https://github.com/CTDave001/gravity-notes.git
cd gravity-notes

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Usage

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Shift+N` | Quick capture (global) |
| `Ctrl+N` | New note |
| `Ctrl+W` | Close window |
| `Ctrl+F` | Search notes |
| `Escape` | Close popout/capture window |

### Editor Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+B` | Bold |
| `Ctrl+I` | Italic |
| `Ctrl+K` | Link |

### Quick Capture

Press `Ctrl+Shift+N` from anywhere to open a floating capture window. Write your thought and close - it automatically saves and syncs to your notes.

### Pop-out Windows

Double-click any note in the sidebar or card view to open it in a separate floating window. Great for reference while working in other apps.

## Tech Stack

- [Tauri v2](https://v2.tauri.app/) - Rust-based desktop framework
- [Svelte 5](https://svelte.dev/) - Reactive UI framework
- [CodeMirror 6](https://codemirror.net/) - Extensible code editor
- [Tailwind CSS v4](https://tailwindcss.com/) - Utility-first CSS

## Data Storage

Notes are stored locally on your machine:

- **Windows**: `%APPDATA%\Gravity\notes\`
- **macOS**: `~/Library/Application Support/Gravity/notes/`
- **Linux**: `~/.local/share/Gravity/notes/`

Each note is saved as a `.md` file, so you can access them directly if needed.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) for details.

---

Made with care for writers who value simplicity.
