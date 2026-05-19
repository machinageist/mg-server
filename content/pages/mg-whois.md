---
title: "mg-whois"
date: 2026-05-18
summary: "WHOIS, ASN, and BGP prefix lookup for a target domain via raw TCP and public APIs."
tags: [geistscope, cli, recon, osint]
---

## Purpose

Pull registration data, ASN ownership, and BGP prefix information for a target
domain. Useful early in recon to understand the network footprint and identify
the registrar and hosting org.

## Output

- `recon/whois-<domain>.json` — raw WHOIS text, parsed registrar and expiry
  fields, ASN number and org name, and the list of BGP prefixes announced for
  that ASN.

## CLI

```bash
mg-whois acme-bounty
mg-whois acme-bounty --target api.acme.example.com
```

## Notes

- WHOIS is a two-hop raw TCP process: first connection to `whois.iana.org:43`
  to find the authoritative server, then a second TCP connection to that server.
- ASN data comes from ipinfo.io (free tier, no key required).
- BGP prefix data comes from api.bgpview.io.
- The `--target` flag lets you query a specific subdomain rather than the root
  domain derived from the engagement scope.
