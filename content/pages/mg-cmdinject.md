---
title: "mg-cmdinject"
date: 2026-05-18
summary: "OS command injection detector with blind OOB, error-based, and time-based strategies."
tags: [geistscope, cli, web, injection]
---

## Purpose

Detects OS command injection using three strategies: blind OOB detection (reads callbacks from [mg-oob](/wiki/mg-oob)), error-based detection via known error strings in responses, and time-based blind detection with sequential per-parameter baselining. Endpoints are sourced from the crawl corpus.

## Output

- `cmdinject/results-<timestamp>.json` — per-parameter findings with strategy, payload, and evidence.

## CLI

```bash
mg-cmdinject acme-bounty
mg-cmdinject acme-bounty --oob-url https://oob.example.com/token
```

## Notes

- Blind OOB mode requires [mg-oob](/wiki/mg-oob) running to capture inbound callbacks.
- Time-based probes baseline each parameter first, then run sequentially to avoid load-induced false positives.
- Run [mg-crawl](/wiki/mg-crawl) first to populate the endpoint corpus.
