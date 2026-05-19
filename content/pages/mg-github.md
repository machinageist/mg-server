---
title: "mg-github"
date: 2026-05-18
summary: "GitHub OSINT code search for secrets, internal domains, and config files exposed in repos tied to a target org or domain."
tags: [geistscope, cli, osint]
---

## Purpose

Search GitHub for code belonging to a target org or referencing a target
domain. Catches secrets, internal hostnames, and misconfigured config files
committed to public repos.

## Output

- `recon/github-findings.json` — matched file URLs, matched content snippets,
  repo name, and a severity label for each result.

## CLI

```bash
mg-github acme-bounty --token $GH_TOKEN
mg-github acme-bounty --org acme-corp --token $GH_TOKEN
```

## Notes

- Uses the GitHub code search API (`/search/code`).
- Unauthenticated requests are limited to 10 req/min. A token raises this to
  30 req/min. The tool reads `X-RateLimit-*` headers and sleeps until the
  reset timestamp when quota is exhausted.
- Default query set includes patterns for API keys, connection strings,
  internal hostnames matching the scope, `.env` files, and common credential
  file names.
- The `--org` flag restricts searches to repos under that org. Without it,
  searches run across all of GitHub and will produce noisier results.
- For continuous monitoring rather than a one-shot search, use
  [mg-leak-monitor](/wiki/mg-leak-monitor).
