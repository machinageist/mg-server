---
title: "mg-jwt"
date: 2026-05-18
summary: "JWT attack toolkit covering decode, HMAC brute force, claim manipulation, and RS256-to-HS256 algorithm confusion."
tags: [geistscope, cli, auth]
---

## Purpose

Provides four JWT attack modes: decode (pretty-print header and claims), brute-force HMAC secret against a wordlist, claim manipulation (forge arbitrary claims with a known or cracked secret), and RS256-to-HS256 algorithm confusion using a supplied public key. HMAC-SHA256 is implemented directly via the `hmac` and `sha2` crates with no JWT library dependency.

## Output

- `jwt/results-<timestamp>.json` — mode-specific findings: cracked secret, forged token, or confusion token with diff of original vs. forged claims.

## CLI

```bash
mg-jwt acme-bounty decode <token>
mg-jwt acme-bounty brute <token> --wordlist /path/rockyou.txt
mg-jwt acme-bounty confuse <token> --pubkey pub.pem
```

## Notes

- `decode` does not validate signature; use it for quick inspection.
- `brute` is single-threaded wordlist scan; feed it with a focused list for realistic runtimes.
- `confuse` signs the token with the RSA public key as an HMAC secret, exploiting servers that trust the `alg` header.
- Related: [mg-authz](/wiki/mg-authz) for replaying forged tokens against access-controlled endpoints.
