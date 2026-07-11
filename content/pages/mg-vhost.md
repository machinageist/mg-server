---
title: "mg-vhost"
date: 2026-05-18
summary: "Virtual host enumeration and host-header injection prober."
tags: [geistscope, cli, web, recon]
---

## Purpose

Probes a target IP or domain with candidate Host headers to find virtual hosts that
diverge from the default response. A baseline is established against the bare
base-domain header, and any candidate that returns a different HTTP status or a
sufficiently different body length is flagged. Also sends an attacker.com Host header
to catch misconfigured reverse proxies that forward arbitrary values downstream.

## Output

- `vhost/results-<timestamp>.json` — candidate hosts with status, content-length
  delta, and injection probe result.

## CLI

```bash
mg-vhost acme-bounty
mg-vhost acme-bounty --target 1.2.3.4 --wordlist vhosts.txt
```

## Notes

- Built-in wordlist covers common internal subdomains: `admin`, `internal`, `dev`,
  `staging`, `api`, and similar.
- `--wordlist` appends to or replaces the built-in list depending on flags.
- Host injection finding is written to `findings/` as MEDIUM severity when the
  attacker.com response diverges from baseline.
