---
title: "mg-sourcemap"
date: 2026-05-18
summary: "Source map downloader and analyzer that extracts original source paths and scans embedded content for secrets."
tags: [geistscope, cli, static-analysis, web]
---

> As of 2026-05-22 this tool is a subcommand of `mg-artifact-audit`. The standalone
> `mg-sourcemap` binary has been retired; behavior is unchanged.

## Purpose

Locates `.js.map` files from three sources: findings from
`mg-artifact-audit js`, `<script>` tags in crawl HTML, and `.map` suffix probes
on known JS URLs. Downloads each map, extracts the `sources[]` path
list and any embedded `sourcesContent[]`, then scans for secrets and internal path
disclosures. The `sourcesContent[]` array may be absent in production maps; this is
handled gracefully.

## Output

- `sourcemap/results.json` — per-map summary: source paths found, secret candidates,
  and internal path patterns.
- `sourcemap/sources/<hash>/` — extracted source files from `sourcesContent[]` when
  present.

## CLI

```bash
mg-artifact-audit sourcemap acme-bounty
mg-artifact-audit sourcemap acme-bounty --concurrency 5
```

## Notes

- Source paths reveal the original project directory layout even when
  `sourcesContent[]` is absent, which can expose framework versions, internal package
  names, and developer paths.
- Secrets in extracted source content are masked to first 8 chars before writing.
- Lower default concurrency (5) avoids hammering CDN rate limits on large JS bundles.
