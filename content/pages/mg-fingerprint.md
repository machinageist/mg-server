---
title: "mg-fingerprint"
date: 2026-05-15
summary: "Detect server, framework, CMS, CDN, and cloud signals from HTTP responses."
tags: [geistscope, cli, recon, fingerprint]
---

## Purpose

Probe HTTP-accessible hosts and produce a structured tech-stack fingerprint
for the engagement. Used by [payload-engine](/wiki/libraries) and
[ai-prioritize](/wiki/ai-prioritize) to pick stack-aware payloads and to
explain the attack surface to the LLM.

## Output

- `recon/fingerprint.json` — per-host `{ server, framework, cms, cloud,
  powered_by, headers_seen, hints }`.

## CLI

```bash
mg-fingerprint acme-bounty
mg-fingerprint acme-bounty --only https://api.acme.example.com
```

## Notes

- Same-origin only; never follows out-of-scope redirects.
- Output is advisory. A `framework: laravel` hint nudges payload selection
  toward Twig SSTI variants but does not change scope behavior.
- Library `fingerprint` (a workspace crate) is the shared logic. See
  [Shared libraries](/wiki/libraries).
