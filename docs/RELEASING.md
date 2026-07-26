# Releasing Gravity

Releases are created by `.github/workflows/release.yml` when a `v*` tag is pushed. The workflow validates the repository, builds all supported desktop targets, signs update artifacts, and creates a draft GitHub release.

## Required repository secrets

| Secret | Purpose |
| --- | --- |
| `TAURI_SIGNING_PRIVATE_KEY` | Signs Tauri updater artifacts |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | Unlocks the updater signing key |
| `AZURE_CLIENT_ID` | Azure Artifact Signing authentication on Windows |
| `AZURE_CLIENT_SECRET` | Azure Artifact Signing authentication on Windows |
| `AZURE_TENANT_ID` | Azure tenant used by Artifact Signing |

`GITHUB_TOKEN` is supplied automatically by GitHub Actions. The Windows signing account, certificate profile, and endpoint are configured in `src-tauri/tauri.windows-signing.conf.json`.

## Versioning

Keep the application version aligned in:

- `package.json` and the root entry in `package-lock.json`
- `src-tauri/Cargo.toml` and Gravity's entry in `src-tauri/Cargo.lock`
- `src-tauri/tauri.conf.json`

Use semantic versioning without a leading `v` in those files. The Git tag uses `v`, for example `v1.1.0`.

## Pre-release checklist

```bash
npm ci
npm run check
npm test
npm audit
npm run build
```

From `src-tauri/`:

```bash
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Also verify in the native application:

- quick capture opens through the global shortcut;
- empty quick captures are removed and non-empty captures survive closing;
- note switching and window closing flush autosave;
- full-text search returns content matches;
- Markdown, text, and Unicode PDF export work;
- main-window close hides to tray and tray Quit exits;
- update prompts can be dismissed and do not install without confirmation.

When Azure credentials, role assignments, or the certificate profile have changed,
run the **Windows Signing Check** workflow manually before tagging. It signs and
verifies a disposable runner executable so authentication and signing failures are
reported without waiting for a full Windows application build.

## Publish procedure

1. Update the version in every location listed above.
2. Update user-facing release notes or README content if behavior changed.
3. Commit the release preparation.
4. Create and push the tag:

   ```bash
   git tag v1.1.0
   git push origin main
   git push origin v1.1.0
   ```

5. Wait for the **Release** workflow to complete on every platform.
6. Inspect the draft release before publishing it:
   - expected macOS ARM64 and x64 artifacts;
   - Linux artifacts;
   - Windows installer and Authenticode signature;
   - Tauri updater archives and `.sig` files;
   - `latest.json` with valid version, URL, and signature values.
7. Test an installer and an update from a previous released version.
8. Publish the draft release.

## Signing model

Tauri updater signing verifies that a downloaded update was produced by the project. Windows Authenticode signing verifies the installer publisher to Windows. They are separate signatures.

Windows Authenticode signing runs inside Tauri's bundle step through `artifact-signing-cli`. Do not add a post-build installer-signing step: changing an installer after Tauri creates its updater signature can invalidate the update.

Never commit private signing keys, passwords, Azure credentials, or generated secret material.

## Recovery

If one matrix build fails, leave the release as a draft, correct the failure, and rerun the workflow. Do not publish a partial release or manually construct `latest.json` with guessed URLs or signature contents.

For Windows signing failures, run **Windows Signing Check** first. The workflow
prints the signing client's direct error output, which the Tauri bundler may
otherwise summarize as only `failed to run artifact-signing-cli`.
