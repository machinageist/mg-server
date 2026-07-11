---
title: "mg-xss"
date: 2026-05-18
summary: "Two-phase XSS detector for reflected and DOM-based vulnerabilities, sourcing injectable parameters from the crawl corpus."
tags: [geistscope, cli, web, xss]
---

## Purpose

Detects reflected and DOM-based XSS vulnerabilities using a two-phase approach: a marker probe first confirms parameter reflection in the response, then payload injection runs only on confirming parameters. Injectable parameters are sourced from `crawl/endpoints.json`. Blind XSS is supported via `--oob-url`.

## Output

- `xss/results-<timestamp>.json` — per-parameter findings with payload, reflected context, and severity.

## CLI

```bash
mg-xss acme-bounty
mg-xss acme-bounty --oob-url https://oob.example.com/token --concurrency 20
```

## Notes

- Parameters sourced from the crawl corpus; run [mg-crawl](/wiki/mg-crawl) first.
- OOB blind XSS payloads require [mg-oob](/wiki/mg-oob) running to capture callbacks.
- Marker probe phase keeps noise low by skipping payload injection on non-reflecting params.
