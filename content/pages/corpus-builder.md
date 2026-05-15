---
title: "corpus-builder"
date: 2026-05-15
summary: "Mine Certificate Transparency logs and Wayback data into a reusable SQLite corpus of domains, subdomains, and paths."
tags: [geistscope, cli, recon, corpus]
---

## Purpose

Build a long-lived, reusable hunting corpus. CT-log queries and Wayback
archive crawls are slow and rate-limited, so we cache them in SQLite and
let the other tools query the cache locally.

## Output

- A SQLite database (default `~/.geistscope/corpus.sqlite`) with tables for
  domains, subdomains, and observed paths.

## CLI

```bash
corpus-builder ingest --target acme.example.com
corpus-builder query  --domain acme.example.com --kind paths
```

## Notes

- Corpus is engagement-agnostic. Use it to bootstrap a new engagement's
  wordlists from real prior data instead of generic lists.
- Source attributions are stored so the operator can re-check origins if
  a hit looks suspicious.
