# Gravity on iOS

Gravity's shared Rust and Svelte code is prepared for an iOS target. This document covers the remaining native workflow and the checks required before an App Store submission.

## What is already implemented

- Desktop-only updater, process relaunch, global shortcuts, tray, clipboard conversion, secondary-window creation, and close-to-tray behavior are compile-time gated.
- iOS has a single-window list/editor navigation model.
- The mobile shell handles iPhone safe areas, touch-sized primary actions, and the software viewport.
- Markdown, text, and PDF exports use the native save picker and security-scoped Files access.
- Desktop and iOS capabilities are separate.
- GitHub self-updates are disabled in the iOS configuration.
- `Info.ios.plist` and `PrivacyInfo.xcprivacy` sources are committed.

## Prerequisites

iOS development requires macOS somewhere in the build pipeline. A local Mac is
optional because the repository includes a manual GitHub Actions workflow backed
by a hosted macOS runner.

1. Install full Xcode and accept its license.
2. Install the Xcode command-line tools and CocoaPods.
3. Install Rust through rustup and add the iOS targets required by Tauri.
4. Enroll in the Apple Developer Program.
5. Register a final iOS bundle identifier for the intended Apple Developer team.

The existing `com.gravity.app` identifier is retained for desktop data-path
compatibility. The cloud workflow requires an explicit iOS identifier that does
not end in `.app`, such as `com.example.gravitynotes`, and applies it only to the
generated iOS project.

## Build on GitHub and test on an iPhone

The `.github/workflows/ios-testflight.yml` workflow validates the application,
builds and signs it on a GitHub-hosted Mac, preserves the IPA as a private
workflow artifact, and optionally uploads it to TestFlight. It is manual-only so
a normal push cannot publish an iOS build.

### One-time Apple setup

1. In Apple Developer Certificates, Identifiers & Profiles, register an explicit
   App ID for the final iOS bundle identifier.
2. In App Store Connect, create the Gravity app with exactly that bundle ID.
3. In App Store Connect **Users and Access → Integrations**, create a Team API key
   with **Admin** access, which Tauri's automatic CI signing currently requires.
   Download its `.p8` file immediately; Apple permits only one download.
4. Find the 10-character Team ID on the Apple Developer membership page.

### One-time GitHub setup

In the GitHub repository, create an environment named `testflight`. Add a
required reviewer to the environment if the repository plan supports it, then
configure:

| Type | Name | Value |
| --- | --- | --- |
| Environment secret | `APPLE_API_ISSUER` | App Store Connect Issuer ID |
| Environment secret | `APPLE_API_KEY` | App Store Connect Key ID |
| Environment secret | `APPLE_API_KEY_P8_BASE64` | Base64-encoded `.p8` contents |
| Environment variable | `APPLE_DEVELOPMENT_TEAM` | 10-character Developer Team ID |
| Environment variable | `IOS_BUNDLE_ID` | Registered explicit iOS bundle ID |

Encode the `.p8` file on Windows without exposing it to the repository:

```powershell
[Convert]::ToBase64String(
  [IO.File]::ReadAllBytes("C:\path\to\AuthKey_KEYID.p8")
) | Set-Clipboard
```

Paste the clipboard value directly into the GitHub environment secret. Never
commit the key or paste it into an issue, log, or chat.

### Send a build to the phone

1. Open the repository's **Actions → iOS TestFlight** workflow.
2. Select **Run workflow**, leave **Upload the signed IPA** enabled, and approve
   the `testflight` environment if prompted.
3. Wait for Apple to process the accepted upload in App Store Connect.
4. Add the Apple account as an internal TestFlight tester and install the build
   from the TestFlight app on the iPhone.

Each workflow run uses the GitHub run number as part of `CFBundleVersion`, so
repeat uploads do not reuse an App Store Connect build number.

## Generate and run

From the repository root on macOS:

```bash
npm ci
npm run check
npm test
npm run ios:init
npm run ios:dev
```

`ios:init` runs Tauri's project generator and copies `PrivacyInfo.xcprivacy` into `src-tauri/gen/apple/`. The generated Apple project is machine/team-specific and is not committed.

Open the generated project through Tauri when native signing or Xcode settings need attention:

```bash
npm run tauri -- ios build --open
```

## Device test matrix

Before TestFlight, verify at least:

- Current shipping iOS on a recent iPhone.
- The oldest supported iOS version (`15.0`) on a simulator or device.
- A compact-width iPhone and a large iPhone.
- Portrait and landscape.
- Light, dark, and system appearance.
- Large accessibility text, VoiceOver labels, and Reduce Motion.
- Software keyboard appearance, dismissal, selection, copy/paste, and autocorrection.
- Creating, editing, switching, deleting, and undoing notes while offline.
- Backgrounding and force-quitting during and immediately after an autosave.
- Markdown, text, and PDF export to local Files and iCloud Drive.
- Image paste/import and rendering from the application container.
- A collection large enough to exercise search and scrolling.

Use disposable fixtures. Never point development builds at a user's only copy of their notes.

## App Store preparation

1. Create the App Store Connect application using the final bundle identifier.
2. Configure the Apple Development and Distribution certificates, team, and provisioning.
3. Add the required app icon, screenshots, description, support URL, privacy policy URL, age rating, category, and review notes.
4. Complete App Privacy declarations. Local-only Gravity does not transmit notes; update the answers before enabling any sync or analytics service.
5. Confirm `ITSAppUsesNonExemptEncryption` remains accurate for the shipped networking/encryption implementation.
6. Archive for App Store Connect locally:

   ```bash
   npm run ios:build:store
   ```

7. Upload locally or run the GitHub **iOS TestFlight** workflow, distribute
   through TestFlight, resolve device and review feedback, and only then submit
   for review.

## Release boundary

The code in this repository is mobile-prepared, but an iOS release is not considered ready until the generated Xcode project builds on macOS, all device tests pass, signing succeeds, metadata/privacy answers are complete, and the exact archive passes TestFlight validation.

Apple-distributed builds must continue using App Store updates. Do not enable Gravity's GitHub updater or download-and-install behavior on iOS.
