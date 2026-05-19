---
title: "mg-smuggle"
date: 2026-05-18
summary: "HTTP/1.1 request smuggling detector covering CL.TE, TE.CL, and TE.TE variants via raw TCP/TLS connections."
tags: [geistscope, cli, web, network]
---

## Purpose

Tests for HTTP/1.1 request smuggling by sending CL.TE, TE.CL, and TE.TE variant probes over raw TCP or TLS connections. Detection is timing-based (response time greater than 2x baseline) combined with status-code analysis of the second pipelined response. Each probe is self-contained with no shared connection state. TLS connections use tokio-rustls.

## Output

- `smuggle/results-<timestamp>.json` — per-variant findings with timing delta, response status, and raw request excerpt.

## CLI

```bash
mg-smuggle acme-bounty
mg-smuggle acme-bounty --host api.acme.example.com --port 443
```

## Notes

- Probes bypass reqwest and use raw TCP/TLS to control chunked encoding headers directly.
- Timing threshold is 2x the measured baseline for the target host.
- Each probe is isolated; a failed or hanging probe does not affect other variants.
- Related: [mg-crawl](/wiki/mg-crawl) for endpoint discovery, [mg-replay](/wiki/mg-replay) for manual request replay.
