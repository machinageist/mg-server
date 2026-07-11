---
title: "mg-privesc-windows"
date: 2026-05-18
summary: "Windows local privilege escalation enumeration tool that runs post-access on the target machine."
tags: [geistscope, cli, post-access]
---

## Purpose

Enumerates common Windows privilege escalation vectors after gaining initial access.
Checks unquoted service paths, writable service binary directories, the
`AlwaysInstallElevated` registry key, scheduled tasks accessible to the current user,
token privileges (`SeImpersonatePrivilege`, `SeDebugPrivilege`, etc.), and auto-run
registry keys. Has no engagement workspace dependency.

## Output

- `privesc-windows-<timestamp>.json` — written locally on the target at the path
  specified by `--output`, or printed to stdout when omitted.

## CLI

```bash
mg-privesc-windows --output C:\Users\Public\privesc.json
mg-privesc-windows
```

## Notes

- Command execution is `cfg(target_os = "windows")`-gated: on non-Windows hosts the
  binary prints a message and exits cleanly.
- Output printed to stdout when `--output` is omitted.
- Token privilege checks enumerate the current process token; no elevation is
  attempted.
