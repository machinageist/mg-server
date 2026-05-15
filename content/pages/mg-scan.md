---
title: "mg-scan"
date: 2026-05-15
summary: "Async TCP port scanner with banner grabbing, rate controls, jitter, random order, and optional source-port binding."
tags: [geistscope, cli, recon, network]
---

## Purpose

Scan TCP ports on scoped hosts. Built for engagements where the operator
must run slowly and politely against production targets.

## Output

- `recon/mg-scan.json` — per-host port list, open/closed/filtered status,
  and best-effort banner string when one is offered.

## CLI

```bash
mg-scan acme-bounty --ports 1-1024 --concurrency 100
mg-scan acme-bounty --ports 80,443,8080-8090 --jitter-ms 50 --random
mg-scan acme-bounty --ports 443 --source-port 53
```

## Notes

- Hostnames are scope-checked against `scope.json` before any SYN goes out.
  IP literals bypass the hostname check (useful for VPS targets).
- Concurrency and jitter exist to keep WAFs calm; default rates are
  conservative.
- Used as a stage inside [mg-recon](/wiki/mg-recon).
