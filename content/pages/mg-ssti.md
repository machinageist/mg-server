---
title: "mg-ssti"
date: 2026-05-18
summary: "Server-side template injection detector with engine fingerprinting and RCE verification."
tags: [geistscope, cli, web, injection]
---

## Purpose

Detects server-side template injection in GET parameters and POST fields sourced from the crawl corpus. Engine identification uses the 7*7 / 7*'7' differential matrix to distinguish Jinja2, Twig, Freemarker, and similar engines. RCE verification looks for `id` command output in the response.

## Output

- `ssti/results-<timestamp>.json` — per-parameter findings with detected engine, payload, and response excerpt.

## CLI

```bash
mg-ssti acme-bounty
mg-ssti acme-bounty --concurrency 10 --timeout-ms 5000
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) first to populate GET and POST parameter corpus.
- Engine fingerprinting uses the 7*7 / 7*'7' polyglot matrix; results are best-effort.
- RCE verification is a secondary probe run only when a template expression evaluates.
