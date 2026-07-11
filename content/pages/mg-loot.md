---
title: "mg-loot"
date: 2026-05-18
summary: "Post-access credential harvester that scans common filesystem paths for secrets, tokens, and sensitive config files."
tags: [geistscope, cli, post-access]
---

## Purpose

Scans a target filesystem for credentials and sensitive material after initial
access. Covers `~/.ssh/`, `~/.aws/`, `~/.config/`, environment files (`.env`,
`.env.local`), shell history files, and common application config locations.
Secrets are masked to the first 8 characters before writing. Has no engagement
workspace dependency.

## Output

- `loot-<timestamp>.json` — written locally at the path specified by `--output`,
  or to the current directory when omitted. Contains file path, match type, and
  masked value for each finding.

## CLI

```bash
mg-loot --output /tmp/loot.json
mg-loot --root /home/ubuntu --output loot.json
```

## Notes

- `--root` sets the scan root; defaults to `/` when omitted.
- Masked values store only the first 8 characters; the full value is never written.
- Shell history files (`.bash_history`, `.zsh_history`) are scanned for inline
  credentials passed as CLI arguments.
