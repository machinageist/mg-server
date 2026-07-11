---
title: "mg-privesc-linux"
date: 2026-05-18
summary: "Linux local privilege escalation enumeration tool that runs post-access on the target machine."
tags: [geistscope, cli, post-access]
---

## Purpose

Enumerates common Linux privilege escalation vectors after gaining initial access.
Checks SUID/SGID binaries, writable directories in `$PATH`, `sudo -l` output, cron
jobs readable by the current user, world-writable files, and the kernel version
against known local exploit patterns. Makes no network connections and has no
engagement workspace dependency.

## Output

- `privesc-linux-<timestamp>.json` — written locally on the target at the path
  specified by `--output`, or printed to stdout when omitted.

## CLI

```bash
mg-privesc-linux --output /tmp/privesc.json
mg-privesc-linux
```

## Notes

- Runs entirely from filesystem reads and subprocess calls to standard Unix tools.
- Output printed to stdout when `--output` is omitted, suitable for piping or
  copy-paste exfil.
- Kernel version is recorded; matching against exploit databases is left to the
  analyst.
