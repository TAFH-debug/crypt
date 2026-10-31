# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

Tauri 2 desktop app (Windows, macOS, Linux) rendering a SvelteKit static build in a webview. Default window 1100×600.

## Users

Developers and privacy-minded people who want an open source, local-only password manager whose crypto they can audit. They store their own logins on one desktop machine. No cloud sync, no sharing, no accounts.

## Product Purpose

Keep a person's passwords encrypted on their own device and get them out fast when needed: unlock, find, copy, lock. Success is that the vault is trusted, quick to use, and invisible the rest of the time.

## Positioning

Local only and open source. The vault never leaves the device. Encryption is AES-256-GCM with keys derived by Argon2id; a random data key is wrapped by the master password and, optionally, by a recovery answer. Small native binary (~9 MB).

## Operating Context

- Opened briefly, many times a day, to copy a credential into another app.
- Keyboard-heavy: `/` focuses search, Ctrl/Cmd+N adds an entry, Escape closes dialogs.
- Auto-locks after 5 minutes without input. Clipboard clears 30 seconds after a copy. Revealed passwords hide after 10 seconds.
- Follows the OS light/dark preference.

## Capabilities and Constraints

- Create vault with a master password (min 8 chars); unlock; lock.
- Recovery question: set/replace in settings; reset the master password from the lock screen with the answer.
- Entries: site, username, password, notes, date added. Add, view, copy username/password, reveal, delete. No edit flow yet.
- Files: store any file up to 100 MB, encrypted with the vault key as a separate blob. Optionally link a file to one login; it then shows on that login's card. Save a decrypted copy, relink, remove. Deleting a login keeps its files, unlinked.
- Password generator (20 chars) and a 0–4 strength meter; weak-entry count on the vault header.
- Settings: change master password, recovery question, export backup, import backup, delete vault (type DELETE to confirm).
- Export writes one encrypted `.cryptbak` file (logins + files, no recovery wrap) that opens with the master password current at export time. Import merges a backup or another device's `vault.json` into the unlocked vault and skips duplicates.
- Legacy vault files migrate automatically on first unlock.
- Stack: Svelte 5, SvelteKit (static adapter, SSR off), Tailwind CSS v4, Tauri 2. The master password never lives in the frontend; Rust keeps the session key.

## Brand Commitments

- Name: Crypt.
- Logo: split arch with keyhole (`img/logo.svg`), brand gradient violet `#a478ff` → pink `#e867c3`. App icons are generated from it.
- Voice: calm and trustworthy. Quiet, confident, nothing flashy; a security tool you forget is there.

## Evidence on Hand

- Open source under MIT; GitHub releases at https://github.com/TAFH-debug/crypt/releases.
- No user counts, audits, testimonials, or benchmarks. Do not invent any.

## Product Principles

1. The vault stays on the device. Never imply sync, accounts, or cloud.
2. Speed to credential: unlock, find, copy in seconds, keyboard first.
3. Show the security truthfully. State what the crypto does; claim nothing it doesn't.
4. Quiet by default. The interface recedes; secrets stay hidden until asked for.
