---
title: "mg-oob"
date: 2026-05-18
summary: "Out-of-band HTTP callback listener that generates tokens, captures inbound requests, and writes results for blind vulnerability correlation."
tags: [geistscope, cli, ssrf]
---

## Purpose

Runs a long-lived HTTP listener that generates unique 32-character hex tokens, captures inbound callback requests from OOB payloads, and writes results to the engagement directory. Used as the callback server for blind SSRF, XXE, CMDi, and SQLi probes sent by other tools. DNS listener is a planned future enhancement; HTTP only for now.

## Output

- `oob/callbacks-<timestamp>.json` — captured callbacks with token, source IP, timestamp, and raw request headers.

## CLI

```bash
mg-oob acme-bounty --port 8080
mg-oob acme-bounty --port 8080 --timeout-secs 120 --token abc123
```

## Notes

- Start mg-oob before running [mg-ssrf](/wiki/mg-ssrf), [mg-xxe](/wiki/mg-xxe), [mg-cmdinject](/wiki/mg-cmdinject), or [mg-xss](/wiki/mg-xss) with `--oob-url`.
- The listener must be reachable from the target server; ensure firewall rules permit inbound connections on `--port`.
- `--timeout-secs` controls how long the listener waits for callbacks before shutting down.
- Token can be pre-set with `--token` for consistent payload building across tools.
