---
title: "mg-brute"
date: 2026-05-18
summary: "Login endpoint credential brute forcer with rate-limit detection, lockout detection, and bounded concurrency."
tags: [geistscope, cli, auth]
---

## Purpose

Brute forces login endpoints with bounded concurrency via JoinSet. Built-in credential lists are used when no list files are provided. Found credentials are printed immediately as discovered. Lockout detection monitors 429 and 403 response rates and backs off automatically. Scope is checked before any requests go out.

## Output

- `brute/results-<timestamp>.json` — found credentials, lockout events, and rate-limit observations.

## CLI

```bash
mg-brute acme-bounty --login-url https://acme.example.com/login
mg-brute acme-bounty --login-url https://acme.example.com/api/auth --users-file users.txt --passwords-file passwords.txt
```

## Notes

- Always confirm the target is in scope before running; scope check is enforced at startup.
- Built-in lists are intentionally small; supply `--passwords-file` for realistic coverage.
- Backoff on lockout detection is automatic but does not guarantee accounts are safe; use with care.
- Related: [mg-session-audit](/wiki/mg-session-audit) for post-auth session analysis.
