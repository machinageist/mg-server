---
title: "mg-xxe"
date: 2026-05-18
summary: "XML External Entity injection tester with file disclosure and OOB callback detection."
tags: [geistscope, cli, web, injection]
---

## Purpose

Finds XML-accepting endpoints from crawl corpus headers, injects XXE payloads, and detects both in-band file disclosure and blind OOB callbacks. Blind XXE requires `--oob-url` and reads callback results from `oob/callbacks-*.json` written by [mg-oob](/wiki/mg-oob).

## Output

- `xxe/results-<timestamp>.json` — per-endpoint findings with payload, disclosure content (if any), and OOB correlation.

## CLI

```bash
mg-xxe acme-bounty
mg-xxe acme-bounty --oob-url https://oob.example.com/token
```

## Notes

- Endpoint discovery uses Content-Type headers from the crawl corpus to identify XML-accepting paths.
- Blind XXE callbacks are correlated against `oob/callbacks-*.json`; run [mg-oob](/wiki/mg-oob) before testing.
- In-band file disclosure probes for `/etc/passwd` and similar targets.
