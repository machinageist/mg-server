---
title: "mg-tui"
date: 2026-05-15
summary: "Ratatui dashboard for engagements, hosts, findings, fuzz results, audit logs, harness activity, and browser-style page inspection."
tags: [geistscope, tui, dashboard]
---

## Purpose

`mg-tui` is the terminal dashboard. It reads the same engagement files as
the CLIs, so any file produced by another tool is visible without any IPC.
It is also the prototype surface for the eventual TUI bug-hunting browser.

## Views

- **Engagements** — list and pick from `engagements/` on disk.
- **Hosts** — service inventory from `recon/summary.json`. Enter pivots into
  the Browser tab against the selected host.
- **Findings** — tabular view over `findings/*.md`, with details pane.
- **Fuzz results** — newest `recon/fuzz-<timestamp>.json` entries.
- **Audit log** — tail of `audit.log` with harness-only filter.
- **Harness** — current/last endpoint, queue depth placeholder, endpoint
  registry status, harness-only audit tail.
- **Browser** — page GET/POST with session-aware auth header injection,
  response inspector (status, MIME, headers, redacted Set-Cookie inventory),
  page-search overlay (`/`, `n`, `N`), and request preview.

## Notes

- The browser applies env-backed session headers from `session.json` when
  available and shows only a redacted "auth status" indicator — no secret
  values reach the screen.
- The dashboard never writes secret material into the terminal recording.
- Roadmap: traffic-corpus navigation, replay/fuzz actions from the browser,
  natural-language pivot to [mg-harness](/wiki/mg-harness) endpoints.
