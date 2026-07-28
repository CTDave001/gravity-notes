# Changelog

All notable changes to Gravity are documented here. The project follows semantic versioning.

## Unreleased

### Added

- An iPhone list/editor navigation shell with safe-area handling and touch-sized primary controls.
- Native iOS Files export for Markdown, text, and PDF.
- Separate mobile capabilities, iOS configuration, Info.plist additions, and privacy manifest preparation.
- macOS-only scripts for generating, running, and producing an App Store Connect iOS build.
- A manual hosted-macOS GitHub Actions workflow for signed IPA artifacts and TestFlight uploads.
- iOS release guidance and a provider-neutral, end-to-end encrypted cross-device sync design.

### Changed

- Rebuilt the first-run experience with an iPhone-first layout, native system
  typography, clearer privacy messaging, and a focused first-note action.
- Restored Gravity's original gravity-well identity across desktop, iOS, and
  Android assets.
- Made iOS preparation overwrite Tauri's generated placeholder icon catalog
  with the committed Gravity icon set.
- Split desktop-only updater, process relaunch, global shortcut, tray, clipboard, secondary-window, and close-to-tray behavior from the mobile runtime.
- Made the iOS frontend build remove the GitHub updater and relaunch code entirely.
- Adapted onboarding, settings, exports, status UI, and delete/export toasts for mobile.
- Moved the macOS private API flag into macOS-only configuration.

## 1.1.0 - 2026-07-25

### Added

- Persistent theme, editor font, font-size, line-spacing, notes-list, and onboarding settings.
- First-run onboarding with global shortcut and tray guidance.
- Full-text backend search with matching-line context.
- Visible save states and retry controls.
- Keyboard-accessible settings, help, export, cards, note actions, and dialog focus trapping.
- Autosave queue unit tests and Rust tests for storage, Unicode, paths, and export filenames.
- Current architecture, development, release, and security documentation.

### Changed

- Replaced the invisible auto-hide sidebar with explicit, responsive navigation.
- Simplified the editor toolbar and clarified quick-capture window controls.
- Made window-specific frontend code load dynamically.
- Limited CodeMirror fenced-code support to commonly used languages and split optional language code.
- Made PDF generation lazy and fully offline with Unicode-capable fonts.
- Updated Tauri, frontend, and lockfile dependencies.
- Rebuilt the GitHub release workflow around the official Tauri action, signed updater artifacts, and validation gates.

### Fixed

- Autosave no longer retargets a queued write after switching notes.
- Empty notes are persisted when intentionally cleared.
- Window close, note switching, export, and tray Quit flush pending saves.
- Failed quick-capture creation cannot silently discard non-empty content.
- Failed note selection cannot associate the previous note's content with a new ID.
- Tray Quit and updater relaunch are no longer blocked by the keep-alive behavior.
- Cleanup skips unreadable notes instead of treating read failures as empty content.
- Note title and preview truncation is safe for Unicode text.
- Note and export paths reject traversal and invalid filenames.
- Note and export writes use temporary files and atomic replacement.
- PDF code and prose wrap and paginate instead of truncating.
- Unclosed code fences and tables are included in PDF output.

### Security

- Removed broad frontend filesystem and shell permissions.
- Tightened production and development Content Security Policies.
- Scoped asset access to the application image directory.
- Added image size and extension validation.
- Moved Windows Authenticode signing into the Tauri bundle step so it does not invalidate updater signatures.
- Updated dependencies to a lockfile state with no known npm audit vulnerabilities.

### Removed

- Unused scaffold components and assets.
- External Google Font and CDN dependencies.
- Unused filesystem, shell, image-processing, theme, and all-language packages.
