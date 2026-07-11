---
title: "mg-fuzz"
date: 2026-05-15
summary: "Burp Intruder-style request templating with stack-aware payload selection through payload-engine."
tags: [geistscope, cli, fuzz, active]
---

## Purpose

Send templated HTTP requests with `§marker§` positions filled from a chosen
payload set. Supports the standard Intruder attack modes and prefers
stack-aware payload variants when recon fingerprint data is available.

## Template format

```http
GET /api/v1/users/§id§ HTTP/1.1
Host: api.acme.example.com
Authorization: Bearer §token§
```

Markers are deduplicated in first-appearance order. Payload lists align to
the position order.

## Attack modes

- `sniper` — one position at a time.
- `battering-ram` — same payload across all positions.
- `pitchfork` — one payload from each list in lockstep.
- `cluster-bomb` — Cartesian product of payload lists.

## CLI

```bash
mg-fuzz acme-bounty --template idor.txt --payloads numbers:1-200 --mode sniper
mg-fuzz acme-bounty --template sqli.txt --payloads sqli --mode sniper --context-aware
```

## Output

- `recon/fuzz-<timestamp>.json` — request and response summary per attempt
  with response-length / status diff signals.

## Notes

- `--context-aware` (default when `recon/summary.json` exists) loads
  fingerprint data and asks `payload-engine` for stack-specific payloads
  (e.g. MySQL `LOAD_FILE` for a MySQL-detected target).
- Session-aware: env-var-backed headers from `session.json` are injected
  unless the template sets its own `Authorization` header.
- For LLM-endpoint fuzzing, use [mg-aifuzz](/wiki/mg-aifuzz) instead.
