---
title: "mg-dns-enum"
date: 2026-05-18
summary: "Active DNS enumeration: zone transfer, wildcard detection, DNSSEC check, SRV discovery, and PTR reverse sweep."
tags: [geistscope, cli, recon, dns]
---

## Purpose

Enumerate DNS records for the engagement domain using active techniques.
Covers both common misconfigurations (zone transfers, missing DNSSEC) and
infrastructure discovery (SRV records, reverse PTR sweep).

## Output

- `recon/dns-enum-<domain>.json` — zone transfer results if successful,
  wildcard detection outcome, DNSSEC presence, discovered SRV records,
  and PTR results for the swept range.

## CLI

```bash
mg-dns-enum acme-bounty
mg-dns-enum acme-bounty --ptr-cidr 10.0.1.0/24
```

## Notes

- Zone transfer (AXFR) is attempted via raw TCP to port 53 on each NS server
  for the domain. A successful transfer is written as a HIGH finding.
- PTR sweep defaults to the /24 containing the domain's primary A record.
  Override with `--ptr-cidr` to target an internal range.
- Wildcard detection sends a query for a randomly generated label and checks
  whether it resolves.
- DNSSEC check confirms whether the zone is signed and whether the DS record
  is present at the parent.
