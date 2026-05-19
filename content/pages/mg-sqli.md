---
title: "mg-sqli"
date: 2026-05-18
summary: "SQL injection detector covering error-based and time-based blind techniques, sourcing parameters from the crawl corpus."
tags: [geistscope, cli, web, injection]
---

## Purpose

Detects SQL injection vulnerabilities using two strategies: error-based probes run concurrently, and time-based blind probes run sequentially after baselining to avoid false positives from concurrent load. Parameters are sourced from the crawl corpus.

## Output

- `sqli/results-<timestamp>.json` — per-parameter findings with technique, payload, and evidence.

## CLI

```bash
mg-sqli acme-bounty
mg-sqli acme-bounty --time-threshold-ms 3000 --concurrency 10
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) first to populate the parameter corpus.
- Time-based probes baseline each parameter before injecting timed payloads to reduce false positives.
- Error-based probes run concurrently; time-based probes run sequentially per parameter.
