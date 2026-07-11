---
title: "mg-websocket"
date: 2026-05-18
summary: "WebSocket security tester covering CSWSH detection, unauthenticated access, and optional message fuzzing."
tags: [geistscope, cli, api, web]
---

## Purpose

Detects WebSocket endpoints, tests cross-site WebSocket hijacking (CSWSH) by sending a foreign Origin header, checks for unauthenticated access without auth headers, and optionally fuzzes messages. CSWSH is flagged when a 101 Switching Protocols response is returned with `Origin: https://attacker.com` and no auth header present. Uses tokio-tungstenite for handshake and framing.

## Output

- `websocket/results-<timestamp>.json` — per-endpoint findings with CSWSH status, auth requirement, and fuzz results if `--fuzz` was used.

## CLI

```bash
mg-websocket acme-bounty
mg-websocket acme-bounty --endpoint wss://api.acme.example.com/ws --fuzz
```

## Notes

- WebSocket endpoint candidates sourced from crawl corpus; run [mg-crawl](/wiki/mg-crawl) first.
- `--fuzz` sends a set of common injection payloads as WebSocket messages and checks for error or reflection.
- CSWSH requires the target to accept the connection without validating the Origin header.
- Related: [mg-xss](/wiki/mg-xss) if WebSocket messages are reflected into page DOM.
