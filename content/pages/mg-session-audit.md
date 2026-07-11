---
title: "mg-session-audit"
date: 2026-05-18
summary: "Session management auditor testing fixation, cookie security flags, token entropy, and logout invalidation."
tags: [geistscope, cli, auth]
---

## Purpose

Audits session management by testing: session fixation (pre-auth token survives post-auth), cookie security flags (HttpOnly, Secure, SameSite), session token entropy (bit-length estimate), and server-side session invalidation on logout. Cookie handling uses reqwest's built-in cookie store; Set-Cookie headers are also parsed manually to inspect raw flag attributes that the jar hides.

## Output

- `session-audit/results-<timestamp>.json` — per-check findings with severity, cookie attributes observed, entropy estimate, and fixation status.

## CLI

```bash
mg-session-audit acme-bounty
mg-session-audit acme-bounty --login-url https://acme.example.com/login
```

## Notes

- `--login-url` is required for fixation and invalidation tests that need a full login/logout cycle.
- Token entropy is estimated from the observed token length and character set; it is a lower bound, not a guarantee.
- Related: [mg-csrf](/wiki/mg-csrf) for cookie SameSite overlap, [mg-jwt](/wiki/mg-jwt) if sessions are JWT-based.
