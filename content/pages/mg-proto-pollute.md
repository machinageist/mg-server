---
title: "mg-proto-pollute"
date: 2026-05-18
summary: "JavaScript prototype pollution detector that injects __proto__ and constructor.prototype payloads into JSON bodies and URL params at API endpoints."
tags: [geistscope, cli, web]
---

## Purpose

Tests API endpoints from `crawl/endpoints.json` for JavaScript prototype pollution by injecting `__proto__` and `constructor.prototype` payloads into JSON request bodies and URL parameters. Three verdict classes: `reflected` (payload value appears in response, HIGH), `server_error` (500 response indicating processing, MEDIUM), and `no_effect` (INFO).

## Output

- `proto-pollute/results-<timestamp>.json` — per-endpoint findings with payload, verdict class, and response excerpt.

## CLI

```bash
mg-proto-pollute acme-bounty
mg-proto-pollute acme-bounty --concurrency 10
```

## Notes

- Endpoints sourced from `crawl/endpoints.json`; run [mg-crawl](/wiki/mg-crawl) first.
- Both JSON body and URL parameter injection are attempted at each endpoint.
- `reflected` verdict is definitive; `server_error` is a candidate for manual follow-up.
