---
title: "mg-takeover"
date: 2026-05-18
summary: "Subdomain takeover detector that resolves CNAME chains and matches against known-vulnerable service fingerprints."
tags: [geistscope, cli, recon]
---

## Purpose

Reads enumerated subdomains from the engagement workspace, resolves their full CNAME
chains, and checks each terminal target against a built-in fingerprint list covering
GitHub Pages, Heroku, Fastly, Netlify, and similar services. HTTP body content is
also checked as a secondary signal. Confirmed takeovers are written as HIGH severity
findings.

## Output

- `recon/takeover.json` — per-subdomain result: CNAME chain, service match,
  HTTP probe status, and confidence level.
- `findings/<subdomain>-takeover.json` — written for confirmed and potential HIGH
  severity cases.

## CLI

```bash
mg-takeover acme-bounty
mg-takeover acme-bounty --concurrency 50
```

## Notes

- Reads `recon/subdomain-enum.json` first; falls back to `recon/summary.json` if
  that file is absent.
- A CNAME match without a successful HTTP probe is flagged as "potential" rather than
  "confirmed."
- Default concurrency is 20; raise with `--concurrency` for large subdomain sets.
