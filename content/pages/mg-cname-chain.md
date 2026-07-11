---
title: "mg-cname-chain"
date: 2026-05-18
summary: "Resolves full CNAME chains for all engagement subdomains, detecting circular refs, NXDOMAIN intermediates, and external delegations."
tags: [geistscope, cli, recon, dns]
---

## Purpose

Walk the complete CNAME chain for every subdomain in the engagement and flag
conditions that indicate misconfiguration or takeover potential: external
CNAMEs pointing to third-party services, NXDOMAIN intermediates, circular
references, and chains longer than 5 hops.

## Output

- `recon/cname-chains.json` — full chain per subdomain, resolution status at
  each hop, and a list of flagged conditions with severity.

## CLI

```bash
mg-cname-chain acme-bounty
mg-cname-chain acme-bounty --concurrency 50
```

## Notes

- Reads `recon/subdomain-enum.json` first. Falls back to `recon/summary.json`
  when the subdomain enum file is absent.
- External CNAMEs are any hop that resolves to a domain outside the engagement
  scope. These are flagged INFO and passed to [mg-takeover](/wiki/mg-takeover)
  for fingerprinting.
- NXDOMAIN at any intermediate hop is flagged HIGH as a potential takeover.
- Circular references are detected by tracking visited names per chain.
- Works alongside [mg-takeover](/wiki/mg-takeover) in the subdomain takeover
  discovery workflow.
