---
title: "mg-shodan"
date: 2026-05-18
summary: "Shodan host lookup, DNS resolve, and facet search for engagement targets, with CVE flagging."
tags: [geistscope, cli, recon, osint]
---

## Purpose

Query Shodan for open ports, banners, and known CVEs on engagement hosts.
Domain targets are resolved to IP before the lookup so you can pass either
a hostname or an IP literal.

## Output

- `recon/shodan-<host>.json` — open ports, service banners, detected
  software versions, and any CVEs Shodan has flagged for that host.

## CLI

```bash
mg-shodan acme-bounty --api-key $KEY
mg-shodan acme-bounty --target 1.2.3.4 --api-key $KEY
```

## Notes

- API key is required. Pass it via `--api-key` or set `$MG_SHODAN_KEY` in the
  environment.
- Domain targets are resolved to IP first via hickory-resolver before the
  Shodan API call.
- CVEs present in the Shodan response are flagged in the output for easy
  triage; check them against the engagement scope before filing.
- Facet search is run against the target org's netblock when ASN data is
  available from a prior `mg-whois` run.
