---
title: "mg-csrf"
date: 2026-05-18
summary: "CSRF protection analyzer that checks token presence, SameSite cookie attributes, and Origin validation on state-changing endpoints."
tags: [geistscope, cli, web, auth]
---

## Purpose

Identifies state-changing endpoints and audits their CSRF defenses. For each endpoint, a GET to the same path probes for a rendered form and token. Checks cover: CSRF token presence in forms and headers, SameSite cookie attribute, and missing Origin header validation. Severity is HIGH when both CSRF token and SameSite are absent, MEDIUM when either is missing alone.

## Output

- `csrf/results-<timestamp>.json` — per-endpoint findings with severity, missing controls, and cookie flag details.

## CLI

```bash
mg-csrf acme-bounty
mg-csrf acme-bounty --concurrency 10
```

## Notes

- State-changing endpoints are identified by HTTP method (POST, PUT, PATCH, DELETE) from the crawl corpus.
- Run [mg-crawl](/wiki/mg-crawl) first to populate the endpoint list.
- SameSite=Strict or SameSite=Lax both satisfy the SameSite check; None (with Secure) is flagged.
