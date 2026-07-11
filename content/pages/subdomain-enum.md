---
title: "subdomain-enum"
date: 2026-05-15
summary: "Passive and active subdomain discovery via Certificate Transparency logs and DNS brute force."
tags: [geistscope, cli, recon]
---

## Purpose

Find subdomains for a target without sending anything aggressive against
the target's authoritative DNS or web stack. Runs CT-log queries first
(passive) and DNS brute force second (bounded active).

## Inputs

- A target domain.
- Optional wordlist for the active phase.

## Output

- `recon/subdomain-enum.json` — flat list of discovered names plus their
  source (CT, DNS, etc.) and a normalized hostname.

## CLI

```bash
subdomain-enum acme.example.com
subdomain-enum acme.example.com --wordlist /opt/wordlists/dns-common.txt
```

## Notes

- Used as a stage inside [mg-recon](/wiki/mg-recon); standalone is useful when
  you only need fresh hostnames.
- Out-of-scope hostnames are reported but not exercised by downstream active
  tools because every active tool re-checks `scope.json`.
