---
title: "mg-timeline"
date: 2026-05-18
summary: "Chronological activity timeline that walks the engagement workspace and produces a sorted log of tool runs and findings."
tags: [geistscope, cli, workflow]
---

## Purpose

Walks the entire engagement workspace directory, reads each JSON file's metadata and
content, and produces a sorted timeline of tool executions and findings. Primary
timestamp source is file mtime; falls back to a `"timestamp"` field inside the JSON
when mtime is unavailable or unreliable. Useful for reconstructing session activity
in reports or reviewing what ran and when.

## Output

- `timeline/timeline-<timestamp>.json` — sorted list of events: timestamp, tool
  name, file path, and a short description.
- Human-readable summary printed to stdout as well.

## CLI

```bash
mg-timeline acme-bounty
mg-timeline acme-bounty --since 2026-05-01 --format json
```

## Notes

- `--since` filters events to only those after the given date.
- `--format json` suppresses the stdout summary and writes JSON only.
- File mtime is used as-is; if files were copied or synced, mtimes may not reflect
  the original run time.
