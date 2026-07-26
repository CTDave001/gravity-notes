# Security Policy

## Supported versions

Security fixes are applied to the latest released version of Gravity. Users should update to the newest release offered by the application or the GitHub Releases page.

## Reporting a vulnerability

Please report suspected vulnerabilities privately through [GitHub Security Advisories](https://github.com/CTDave001/gravity-notes/security/advisories/new). Do not include private notes, signing keys, access tokens, credentials, or other sensitive user data in a public issue.

Include:

- the affected Gravity version and operating system;
- clear reproduction steps;
- the expected and actual behavior;
- the security impact;
- any minimal proof of concept that does not expose real user data.

## Security model

Gravity stores notes locally as plain Markdown. It does not provide accounts, cloud storage, synchronization, or collaboration.

The Tauri frontend uses a restricted capability set and Content Security Policy. Filesystem mutations are performed through narrow Rust commands that validate note IDs, filenames, extensions, sizes, and destinations. Updates require a valid Tauri updater signature; Windows installer publisher signing is handled separately through Azure Artifact Signing.

Users and contributors should never commit updater private keys, Azure credentials, passwords, real user notes, or generated secret material.
