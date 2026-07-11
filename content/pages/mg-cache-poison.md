---
title: "mg-cache-poison"
date: 2026-05-18
summary: "Web cache poisoning detector that tests unkeyed header injection, fat GET, and response splitting on endpoints confirmed to have a cache."
tags: [geistscope, cli, web]
---

## Purpose

Tests cacheable endpoints for web cache poisoning via three techniques: unkeyed header injection, fat GET (body ignored by cache but processed by origin), and response splitting. Findings are only emitted for endpoints confirmed to have a cache, identified by the presence of Age, X-Cache, CF-Cache-Status, or Via headers. Detection uses a two-request pattern: a poison request followed by a clean request to check if the poisoned response was served cached.

## Output

- `cache-poison/results-<timestamp>.json` — per-endpoint findings with technique, injected header/value, and cached response excerpt.

## CLI

```bash
mg-cache-poison acme-bounty
mg-cache-poison acme-bounty --concurrency 5
```

## Notes

- Low concurrency recommended to avoid poisoning the cache with concurrent in-flight requests interfering with each other.
- Run [mg-crawl](/wiki/mg-crawl) first to source cacheable endpoint candidates.
- Endpoints without cache indicator headers are skipped entirely.
