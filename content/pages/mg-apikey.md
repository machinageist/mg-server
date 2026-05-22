---
title: "mg-apikey"
date: 2026-05-18
summary: "Static API key and secret extractor that scans HTML, JS, and response-header data in the crawl corpus."
tags: [geistscope, cli, auth, static-analysis]
---

> As of 2026-05-22 this tool is a subcommand of `mg-artifact-audit`. The standalone
> `mg-apikey` binary has been retired; behavior is unchanged.

## Purpose

Scans the crawl corpus (HTML, JS, and response headers) for API keys, tokens, and secrets matching known patterns. No active HTTP requests are made; this is purely filesystem analysis of crawl output. The regex catalog is compiled once via `OnceLock`. Findings are de-duplicated by `(pattern_type, masked_value)`; secrets are masked to the first 8 characters in output.

## Output

- `apikey/results.json` — de-duplicated findings with pattern type, masked value, source file, and line number.

## CLI

```bash
mg-artifact-audit apikey acme-bounty
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) first; this tool reads only from crawl corpus files on disk.
- Pattern catalog covers common key formats: AWS, GCP, Stripe, GitHub, Slack, generic bearer tokens, and others.
- Masking in output is intentional; retrieve the full value from the source file at the reported line.
