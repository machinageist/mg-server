---
title: "mg-traversal"
date: 2026-05-18
summary: "Path traversal and LFI detector that probes URL path segments and GET parameters."
tags: [geistscope, cli, web, injection]
---

## Purpose

Detects path traversal and local file inclusion vulnerabilities by injecting traversal sequences into URL path segments and GET parameters. Injection is limited to parameters whose names suggest file handling, and path segments that look like filenames (containing a dot). Probes run concurrently via a bounded JoinSet.

## Output

- `traversal/results-<timestamp>.json` — per-parameter findings with traversal sequence, matched file content, and URL.

## CLI

```bash
mg-traversal acme-bounty
mg-traversal acme-bounty --concurrency 15
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) first to populate the endpoint corpus.
- Only parameters with file-suggestive names or dotted path segments are probed, reducing noise.
- Concurrency is bounded; default is tuned to avoid rate limiting on most targets.
