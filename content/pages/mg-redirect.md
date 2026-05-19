---
title: "mg-redirect"
date: 2026-05-18
summary: "Open redirect detector that tests URL parameters and builds redirect chains by inspecting Location headers directly."
tags: [geistscope, cli, web]
---

## Purpose

Tests URL parameters for unvalidated redirect vulnerabilities by injecting attacker-controlled domains and following redirect chains manually. reqwest is configured with redirect policy `none` so each Location header can be inspected individually. Findings are classified as `open_redirect` (3xx to attacker domain) or `potential` (body contains attacker domain).

## Output

- `redirect/results-<timestamp>.json` — per-parameter findings with classification, redirect chain, and final destination.

## CLI

```bash
mg-redirect acme-bounty
mg-redirect acme-bounty --evil-domain evil.example.com
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) first to source URL parameters from the corpus.
- Redirect chains are followed manually, step by step, to capture each intermediate hop.
- Default evil domain is a placeholder; set `--evil-domain` to a domain you control for confirmed exploitability.
