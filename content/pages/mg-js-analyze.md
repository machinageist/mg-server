---
title: "mg-js-analyze"
date: 2026-05-18
summary: "Static JavaScript analysis that extracts endpoints, secrets, credentials, and source map references from crawl-collected JS files."
tags: [geistscope, cli, static-analysis, web]
---

> As of 2026-05-22 this tool is a subcommand of `mg-artifact-audit`. The standalone
> `mg-js-analyze` binary has been retired; behavior is unchanged.

## Purpose

Fetches and analyzes JavaScript files referenced in the crawl corpus. Extracts API
endpoint paths, hardcoded credentials, secret candidates, and `.map` source map
references. Runs concurrently via `JoinSet`. Findings are deduplicated by
`(source_url, finding_type, value)` before writing. Complements
[mg-crawl](/wiki/mg-crawl) and feeds source map URLs to
`mg-artifact-audit sourcemap`.

## Output

- `js-analyze/results.json` — deduplicated findings per JS file: endpoints, secret
  candidates (masked to first 8 chars), and source map URLs found.

## CLI

```bash
mg-artifact-audit js acme-bounty
mg-artifact-audit js acme-bounty --concurrency 10
```

## Notes

- JS files are fetched from URLs recorded during crawl; no additional spidering.
- Secret masking applies before writing: only the first 8 characters of a matched
  value are stored.
- Source map references are collected into a list that `mg-artifact-audit sourcemap`
  reads directly.
