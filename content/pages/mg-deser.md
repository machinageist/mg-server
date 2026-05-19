---
title: "mg-deser"
date: 2026-05-18
summary: "Deserialization vulnerability detector that scans the crawl corpus for serialized data magic bytes and known patterns."
tags: [geistscope, cli, web]
---

## Purpose

Scans crawl corpus JSON for serialized data magic bytes and patterns across Java, PHP, and Python. Java findings generate ysoserial command strings for manual use. PHP sends a harmless `stdClass` payload and checks for a 500 response. Python candidates are flagged for manual inspection. Java and Python payloads are not executed automatically.

## Output

- `deser/results-<timestamp>.json` — per-endpoint findings with language, pattern matched, and ysoserial command (Java only).

## CLI

```bash
mg-deser acme-bounty
mg-deser acme-bounty --concurrency 5
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) first; this tool reads crawl corpus files from disk.
- Java findings include a ysoserial command string; you supply the gadget chain and execute manually.
- PHP active probing sends a minimal serialized object; does not attempt RCE payloads.
- Python candidates require manual verification; no active probe is sent.
