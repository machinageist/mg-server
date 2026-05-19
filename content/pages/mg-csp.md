---
title: "mg-csp"
date: 2026-05-18
summary: "Content Security Policy analyzer that flags dangerous directives and suggests bypass chains."
tags: [geistscope, cli, web, static-analysis]
---

## Purpose

Parses CSP headers from crawl output or live probes and flags directives that weaken
policy: `unsafe-inline`, `unsafe-eval`, wildcard sources (`*`), missing `default-src`,
and overly permissive `script-src`. For each dangerous configuration, suggests known
bypass techniques. Reads stored headers from the crawl corpus first; falls back to a
fresh GET request per recon host when crawl data is absent.

## Output

- `csp/results.json` — per-host findings: raw CSP header, parsed directives,
  flagged issues, and suggested bypass chains.

## CLI

```bash
mg-csp acme-bounty
mg-csp acme-bounty --host api.acme.example.com
```

## Notes

- Directive parsing splits on semicolons, then whitespace within each directive.
- `--host` targets a single host and does a live GET rather than reading crawl output.
- Missing CSP headers are recorded as an INFO-level finding, not an error.
