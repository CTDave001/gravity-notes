# Developing Gravity

## Prerequisites

- Node.js 20.19+ or 22.12+; Node.js 22 LTS is recommended.
- Rust 1.77.2 or newer.
- Native Tauri build prerequisites for the target platform.
- On Windows, the `stable-x86_64-pc-windows-msvc` Rust toolchain.

Install dependencies from the lockfile:

```bash
npm ci
```

On Windows, select MSVC when necessary:

```powershell
$env:RUSTUP_TOOLCHAIN = "stable-x86_64-pc-windows-msvc"
```

## Common commands

```bash
# Native development app with Vite hot reload
npm run tauri dev

# Frontend type and Svelte diagnostics
npm run check

# Frontend unit tests
npm test

# Optimized frontend build
npm run build
```

Rust checks run from `src-tauri/`:

```bash
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

A production-mode native build without installer bundling can be used as a final local smoke test:

```bash
npm run tauri -- build --debug --no-bundle
```

## Repository map

| Path | Purpose |
| --- | --- |
| `src/App.svelte` | Shared startup, settings, theme, and window dispatch |
| `src/lib/windows/` | Main, capture, and pop-out note windows |
| `src/lib/components/` | Editor and application UI components |
| `src/lib/autosave.ts` | Serialized debounced save queue |
| `src/lib/stores/settings.ts` | Validated persistent settings |
| `src-tauri/src/` | Rust storage, commands, export, clipboard, and lifecycle |
| `src-tauri/capabilities/` | Tauri frontend permission boundary |
| `.github/workflows/release.yml` | Validation and cross-platform releases |
| `docs/plans/` | Historical design and implementation records |

Generated `dist/`, `node_modules/`, and `src-tauri/target/` directories are not committed. `package-lock.json` and `src-tauri/Cargo.lock` are committed because Gravity is an application and needs reproducible dependency resolution.

## Testing expectations

Before committing a functional change:

1. Run `npm run check` and `npm test`.
2. Run Rust formatting, tests, and strict Clippy.
3. Run `npm run build` for frontend or packaging-related changes.
4. Exercise affected behavior in the native Tauri window; a browser-only preview does not cover Tauri IPC, shortcuts, tray behavior, native dialogs, or secondary windows.
5. For storage changes, test empty content, Unicode, failed reads/writes, and note switching while a save is pending.

Do not use real user notes as disposable test fixtures. Backend unit tests should use temporary files, and native UI smoke tests should avoid changing existing notes.

## Coding conventions

- Keep file operations and trust-boundary validation in Rust.
- Preserve the plain-Markdown storage model.
- Flush autosave before changing note identity or destroying a window.
- Treat unreadable data as unknown, never as empty.
- Add frontend regression tests for queue/state behavior and Rust tests for filesystem validation.
- Keep optional heavy features dynamically imported.
- Do not broaden Tauri permissions when a narrow command can provide the same capability.
