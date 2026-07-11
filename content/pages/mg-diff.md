---
title: "mg-diff"
date: 2026-05-18
summary: "Scan-over-scan diff that surfaces new hosts, port changes, tech changes, and HTTP status changes between two engagement snapshots."
tags: [geistscope, cli, workflow]
---

## Purpose

Compares two recon snapshots for an engagement to show what changed between runs.
Diffs `recon/summary.json` files for new or removed hosts, open port changes, and
technology stack changes. Optionally diffs crawl endpoint lists for new or removed
HTTP paths. Useful for tracking scope drift and re-checking recently added assets.

## Output

- `diff/diff-<timestamp>.json` — structured diff with added/removed/changed entries
  per category: hosts, ports, technologies, HTTP statuses.

## CLI

```bash
mg-diff acme-bounty
mg-diff acme-bounty --baseline recon/2026-05-01/ --current recon/2026-05-18/
```

## Notes

- When `--baseline` and `--current` are omitted, the two most recent timestamped
  subdirectories under `recon/` are selected automatically.
- HTTP status changes are only diffed when crawl endpoint lists exist for both
  snapshots.
- Output is also printed as a human-readable summary to stdout.
