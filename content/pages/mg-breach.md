---
title: "mg-breach"
date: 2026-05-18
summary: "HIBP v3 breach lookup for a target domain, with detail fetched for each breach that includes password data."
tags: [geistscope, cli, osint]
---

## Purpose

Check Have I Been Pwned for known breaches associated with a target domain.
Fetches full breach detail for any breach that contains password data, so you
can assess credential reuse risk for the engagement.

## Output

- `recon/breach-<domain>.json` — list of breaches, breach metadata (date,
  data classes, count), and a flag for whether password data was included.

## CLI

```bash
mg-breach acme-bounty --api-key $HIBP_KEY
mg-breach acme-bounty --domain acme.example.com --api-key $HIBP_KEY
```

## Notes

- API key is mandatory. HIBP v3 requires authentication for domain-level
  queries.
- Rate limit on the HIBP free tier is approximately 1 request per 1.5
  seconds. The tool sleeps between requests to stay within it.
- The `--domain` flag lets you query a specific domain instead of the root
  domain derived from the engagement scope.
- Breaches flagged as containing passwords are highlighted in the output.
  Cross-reference usernames against discovered email addresses from
  [mg-social](/wiki/mg-social).
