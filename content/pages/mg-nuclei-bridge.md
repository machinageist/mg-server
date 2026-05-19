---
title: "mg-nuclei-bridge"
date: 2026-05-18
summary: "Runs Nuclei against engagement scope and imports findings into the engagement workspace as normalized structured JSON."
tags: [geistscope, cli, workflow]
---

## Purpose

Wraps the Nuclei scanner to run against engagement scope hosts and normalize its
output into the GeistScope findings format. Streams Nuclei's JSONL stdout line by
line so findings appear in `findings/` in real time rather than after the full scan
completes. Each finding gets its own timestamped file.

## Output

- `findings/nuclei-<id>.json` — one file per Nuclei finding, normalized to the
  standard GeistScope finding schema: severity, title, host, matched URL, and raw
  Nuclei output.

## CLI

```bash
mg-nuclei-bridge acme-bounty
mg-nuclei-bridge acme-bounty --tags cve,rce --nuclei-path /usr/local/bin/nuclei
```

## Notes

- Nuclei binary must be in `$PATH` or specified via `--nuclei-path`.
- `--tags` is passed directly to Nuclei; any valid Nuclei tag expression works.
- Because `mg-notify` watches `findings/`, each imported finding triggers a
  notification immediately if `mg-notify` is running alongside.
