---
title: "GeistScope Part 5: The Terminal Dashboard"
date: 2026-05-09
summary: "mg-tui is a Ratatui terminal dashboard that gives you a live view across all engagements — hosts, findings, fuzz results, and a built-in browser — without leaving the terminal."
tags: [rust, security, bug-bounty, geistscope, tui, ratatui]
---

## Why a TUI

The GeistScope pipeline generates a lot of structured data. After a recon run
you have `summary.json`. After crawling you have `index.json`, `endpoints.json`,
`secrets.json`. After probing you have `probe-report.json` and a directory of
finding markdown files. After fuzzing you have timestamped fuzz reports.

You can `cat` and `jq` all of it from the command line. But switching between
targets, drilling into findings by severity, following the live output of a
running recon — that's better served by something interactive.

`mg-tui` is a full-screen terminal dashboard built with `ratatui` and `crossterm`.
It reads from the same engagement directory that every other GeistScope tool writes to.
No database, no server process, no API — just file watching.

---

## Tabs

The dashboard has five tabs, navigated with Tab and BackTab:

**Engagements** — a table of all engagement directories.
Each row shows the name, target domain, platform, and creation timestamp.
Press Enter to select an engagement and focus all other views on it.

**Hosts** — the host records from `recon/summary.json` for the selected engagement.
Each row: hostname, resolved IPs, discovery source (ct_log, brute, ct_log+brute),
open port count, detected tech stack. Moving the cursor down expands the port list.

**Findings** — the findings from the selected engagement's `findings/` directory.
Severity, title, target host, status. Press `f` to cycle a severity filter:
all → critical → high → medium → low → info → all.
The most useful filter in practice is `f` twice to jump to high/critical.

**Fuzz** — the fuzz reports from `recon/fuzz-*.json`. Each report shows
the attack mode, template used, total requests, and how many were flagged interesting.
Cursor into a report to see the individual interesting responses.

**Logs** — a live tail of `audit.log`. Every tool invocation appears here in real time.
When `mg-recon` is running in another terminal, you can watch its stages complete
in the log view without `tail -f`.

---

## Mouse Support

The TUI is fully mouse-navigable:
- Scroll wheel scrolls the current view
- Click the tab bar to switch tabs
- Click links in the browser tab to follow them

Most terminal tools ignore the mouse. Adding it was straightforward with crossterm's
`EnableMouseCapture` — you receive `MouseEvent` structs in the same event loop as
key events, with column and row coordinates.

The interesting part is the browser tab, which renders HTML content with
inline link markers like `[1]`, `[2]`, `[3]`. When you click somewhere in the content
area, the code finds the link marker closest to the click column and follows it.
That required mapping click coordinates to rendered content rows, then searching
the spans on that row for the nearest marker — a few dozen lines of geometry.

---

## The Browser Tab

`mg-tui` includes a minimal terminal web browser. Type `u` to edit the URL bar,
Enter to navigate, `b` to go back.

This started as a debugging tool — I wanted to browse crawled pages inside the
TUI without leaving the terminal. It evolved into something more useful: you can
navigate to a host discovered in recon, see the actual rendered page (as text),
and follow links to explore the application structure.

The browser fetches pages with `reqwest`, parses HTML with a simple renderer that
converts the tag tree into styled `ratatui` spans, handles relative URL resolution,
and displays images using Unicode half-block characters (▀) for terminals that
render them as a crude pixel grid.

The image rendering is deliberately low-fidelity — it's enough to see whether
an image is a logo, a photo, or a CAPTCHA. Useful context without leaving the terminal.

---

## Refresh Logic

File data refreshes on a two-second timer. The app calls `app.refresh()` which
re-reads the relevant JSON files from disk. If `mg-recon` is running in another
terminal and writes `summary.json`, the Hosts tab updates within two seconds.

The implementation is simple: poll a timer, reload files, redraw. No filesystem
watchers, no `inotify`, no async file I/O. The polling interval is short enough
that it feels live and cheap enough that it doesn't compete for I/O with the
tools that are actually doing work.

`r` manually triggers a refresh if you don't want to wait.

---

## State Machine

The app state is a single `App` struct in `app.rs`. The main loop:

1. Draw the current state to the terminal
2. Check the refresh timer; reload from disk if due
3. Drain background messages from the channel (page fetches, image fetches)
4. Poll for keyboard/mouse events
5. Mutate app state based on the event
6. Loop

The browser tab uses a background thread per page fetch (spawned with `std::thread::spawn`)
and an `mpsc` channel to return results to the main loop. Image fetches for each
`<img>` slot also spawn separate threads and send results back individually.
The main loop drains the channel non-blocking with `try_recv` each frame.

This keeps the UI responsive during a page load — you can switch tabs, scroll,
or hit `q` to quit while a slow page is fetching.

---

## Installation and Usage

```bash
# Build and install
cd geistscope/engine-rust
cargo install --path mg-tui

# Run from your engagements directory
cd ~/engagements
mg-tui

# Or with explicit path
MG_ENGAGEMENTS_DIR=~/engagements mg-tui
```

Key bindings:

| Key | Action |
|-----|--------|
| Tab / BackTab | Next / previous tab |
| ↑ ↓ or j k | Move cursor |
| Enter | Select engagement / follow link |
| f | Cycle findings severity filter |
| r | Manual refresh |
| u | Edit URL bar (Browser tab) |
| b | Browser back |
| R | Browser reload |
| [ ] | Previous / next link |
| q or Ctrl-C | Quit |

---

## What It Replaces

Before `mg-tui`, checking engagement status meant:

```bash
jq '.hosts | length' engagements/target-bounty/recon/summary.json
ls engagements/target-bounty/findings/ | wc -l
tail -f engagements/target-bounty/audit.log
jq '.results | map(select(.diff.interesting)) | length' engagements/target-bounty/recon/fuzz-*.json
```

All of that is still there if you want it — everything is plain files.
But having one view that shows all of it at once, across all your active engagements,
makes the difference between context-switching and staying focused.

---

## What's Next

The TUI is the current end of the frontend stack. The ULTRAPLAN document has a GUI
layer planned — a Tauri or egui native desktop app with richer visualizations,
a side-by-side response diff viewer for fuzz results, and a template editor
with `§marker§` highlighting.

Beyond the frontend, three features are candidates for the next development cycle:

**OOB interactsh integration** — blind SSRF and blind XSS payloads that call back
to a self-hosted interactsh server. Right now, mg-fuzz can send the payloads
but can't detect out-of-band hits. Integrating interactsh closes that loop
without managing DNS infrastructure.

**Subdomain takeover checks** — DNS CNAME records pointing to services that have
been deprovisioned. The CNAME resolves but the backing service doesn't exist.
Registering the missing resource on the third-party platform gives you control
of the subdomain. Detectable purely from DNS recon data.

**GraphQL introspection and operation fuzzing** — a separate attack surface that
has its own tooling needs. GraphQL endpoints expose their schema through introspection,
which makes attack surface enumeration easier than REST. The fuzzing patterns
are also different.

*GeistScope is available on GitHub. The full toolchain builds with `cargo build --workspace`.*
