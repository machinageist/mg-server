---
title: "mg-google-dork"
date: 2026-05-18
summary: "Generates and optionally executes Google dork queries for a target using 14 built-in templates covering login pages, config files, and exposed directories."
tags: [geistscope, cli, osint]
---

## Purpose

Generate Google dork queries for the engagement target. In print-only mode,
output is immediately usable in a browser. In API mode, queries are executed
via the Google Custom Search API and results are stored as findings.

## Output

- stdout — all generated dork queries, always printed regardless of mode.
- `recon/google-dork.json` — search results per query (API mode only).

## CLI

```bash
mg-google-dork acme-bounty
mg-google-dork acme-bounty --api-key $KEY --cx $CX
```

## Notes

- 14 built-in dork templates: login pages (`inurl:login`), admin panels,
  exposed config files (`.env`, `wp-config.php`), open directories
  (`intitle:"index of"`), juicy file types (`.sql`, `.bak`, `.log`), error
  messages, and others.
- API mode requires a Google Custom Search Engine (CSE) key (`--api-key`) and
  a search engine ID (`--cx`). Free CSE tier allows 100 queries/day.
- Print-only mode has no external dependencies and works offline.
- Results in API mode are written per-query with the full result list and a
  severity estimate based on template type.
