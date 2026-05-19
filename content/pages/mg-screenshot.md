---
title: "mg-screenshot"
date: 2026-05-18
summary: "Headless bulk screenshotter for HTTP/HTTPS hosts, producing per-host PNGs and an HTML index for visual triage."
tags: [geistscope, cli, recon, web]
---

## Purpose

Capture screenshots of every HTTP/HTTPS host in the engagement for rapid
visual triage. An HTML index lets you scan all pages at once without opening
each host individually.

## Output

- `screenshots/<host>.png` — one screenshot per host.
- `screenshots/index.html` — grid layout linking all screenshots with the
  host URL and HTTP status code.

## CLI

```bash
mg-screenshot acme-bounty
mg-screenshot acme-bounty --width 1280 --height 800 --timeout-ms 8000
```

## Notes

- Reads HTTP/HTTPS hosts from `recon/summary.json`. Only hosts with port 80
  or 443 open (or custom ports discovered during probing) are included.
- Requires Chromium on PATH. Accepts `chrome`, `chromium`, or
  `chromium-browser`. Set `$CHROME_PATH` to override.
- Uses chromiumoxide 0.7 for browser control. Each tab gets its own timeout;
  a hung tab does not block the rest of the queue.
- Default viewport is 1440x900. Default timeout is 10000ms.
