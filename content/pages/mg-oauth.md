---
title: "mg-oauth"
date: 2026-05-18
summary: "OAuth 2.0 flow analyzer testing state CSRF, redirect_uri bypass, PKCE downgrade, and implicit flow misconfigurations."
tags: [geistscope, cli, auth]
---

## Purpose

Tests OAuth 2.0 authorization flows for the most common misconfigurations: state parameter CSRF (missing or static state), redirect_uri bypass (open redirect or path traversal variants), PKCE downgrade (omitting code_challenge), and implicit flow enablement. Candidate OAuth URLs are read from the crawl corpus; `--auth-url` overrides for direct targeting. Uses `redirect::Policy::none()` to inspect Location headers at each step.

## Output

- `oauth/results-<timestamp>.json` — per-flow findings with misconfiguration type, request/response detail, and severity.

## CLI

```bash
mg-oauth acme-bounty
mg-oauth acme-bounty --auth-url https://auth.acme.example.com/authorize
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) first to source OAuth endpoint candidates from the corpus.
- Redirect_uri bypass variants include subdomain injection and path traversal suffixes.
- PKCE downgrade test omits `code_challenge` and checks whether the server rejects the request.
- Coverage based on common HackerOne OAuth misconfiguration patterns.
