---
title: "mg-secret-validate"
date: 2026-05-18
summary: "Classifies and live-validates leaked secrets by probing known APIs to confirm whether a credential is active."
tags: [geistscope, cli, static-analysis]
---

## Purpose

Reads secret candidates from `--secret` arguments and from `secrets.json` files in
the crawl corpus, classifies each by regex pattern (GitHub token, AWS key, Stripe
key, etc.), and probes the relevant API to confirm validity. AWS validation uses STS
`GetCallerIdentity`: an `AuthFailure` response means the key exists but lacks
permission, which counts as VALID. Unknown-classified secrets are recorded but not
live-validated.

## Output

- `secret-validate/results.json` — per-secret entry: classification, masked value
  (first 8 chars), validation status, and API probe response code.

## CLI

```bash
mg-secret-validate acme-bounty
mg-secret-validate acme-bounty --secret ghp_abc123
```

## Notes

- All output masks secrets to the first 8 characters; plaintext values are never
  written to disk.
- `AuthFailure` from AWS STS is treated as VALID because the key was accepted before
  the permission check.
- Multiple `--secret` flags are accepted; they are merged with any crawl
  `secrets.json` input before classification.
