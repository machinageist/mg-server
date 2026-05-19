---
title: "mg-ssrf"
date: 2026-05-18
summary: "SSRF detector that injects OOB payloads into crawl corpus parameters and probes cloud metadata endpoints."
tags: [geistscope, cli, ssrf]
---

## Purpose

Detects SSRF by injecting OOB callback URLs and cloud metadata endpoint URLs into injectable parameters sourced from `crawl/endpoints.json`. OOB correlation checks `oob/callbacks-*.json` for token hits written by [mg-oob](/wiki/mg-oob). Cloud metadata probes cover AWS, GCP, and Azure IMDS endpoints.

## Output

- `ssrf/results-<timestamp>.json` — per-parameter findings with injected URL, OOB correlation status, and metadata response excerpt.

## CLI

```bash
mg-ssrf acme-bounty
mg-ssrf acme-bounty --oob-url https://oob.example.com/token --concurrency 10
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) first to populate injectable parameter corpus.
- OOB mode requires [mg-oob](/wiki/mg-oob) running to capture callbacks; callbacks are correlated by token.
- Confirmed SSRF: use [mg-aws](/wiki/mg-aws), [mg-gcp](/wiki/mg-gcp), or [mg-azure](/wiki/mg-azure) to extract credentials.
