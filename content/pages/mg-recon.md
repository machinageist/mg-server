---
title: "mg-recon"
date: 2026-05-15
summary: "Four-stage recon orchestrator: subdomains, fingerprinting, port scanning, and summary merge."
tags: [geistscope, cli, recon, orchestrator]
---

## Purpose

Run the standard discovery sequence end-to-end and produce one merged
`summary.json` that every downstream tool — `mg-probe`, `mg-crawl`,
`ai-prioritize`, the TUI — consumes.

## Pipeline

1. `subdomain-enum` — CT logs + DNS brute force.
2. `mg-fingerprint` — HTTP fingerprint per reachable host.
3. `mg-scan` — TCP port scan per host.
4. Merge — write `recon/summary.json` keyed by host.

## CLI

```bash
mg-recon acme-bounty --ports 1-1024 --concurrency 100
mg-recon acme-bounty --skip-scan
```

## Output

- `recon/subdomain-enum.json`, `recon/fingerprint.json`, `recon/mg-scan.json`
- `recon/summary.json` — the primary AI input.

## Notes

- `summary.json` is the contract for downstream tools. Schema drift here
  cascades.
- Available via [mg-harness](/wiki/mg-harness) `recon.run` (HighActive — the
  AI must request `confirmed: true`).
- `mg-recon` is exposed as a library so harness calls dispatch in-process
  instead of shelling out.
