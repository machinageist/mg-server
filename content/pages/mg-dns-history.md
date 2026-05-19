---
title: "mg-dns-history"
date: 2026-05-18
summary: "Historical DNS records via SecurityTrails or HackerTarget, with cross-reference against current recon to surface stale IPs."
tags: [geistscope, cli, recon, dns, osint]
---

## Purpose

Retrieve historical DNS records for the engagement domain to find IP addresses
that once pointed to the target but are no longer in the current scan.
Old IPs may host staging environments or bypass CDN/WAF layers.

## Output

- `recon/dns-history-<domain>.json` — historical A/CNAME records per
  subdomain, timestamps where available, and a list of IPs that appear in
  history but not in `recon/summary.json`.

## CLI

```bash
mg-dns-history acme-bounty
mg-dns-history acme-bounty --api-key $KEY
```

## Notes

- SecurityTrails is the primary source when `--api-key` or
  `$MG_SECURITYTRAILS_KEY` is set.
- Without a key, the tool falls back to HackerTarget's free tier, which has
  lower rate limits and less history depth.
- Cross-referencing against `recon/summary.json` is automatic; the diff is
  listed under `stale_ips` in the output.
- Stale IPs are not automatically added to scope; review them manually before
  scanning.
