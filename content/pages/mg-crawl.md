---
title: "mg-crawl"
date: 2026-05-15
summary: "Same-origin BFS crawler with enriched endpoint extraction, GraphQL signals, internal-reference detection, and library CVE hints."
tags: [geistscope, cli, crawl, javascript, graphql]
---

## Purpose

Walk a target's HTML and JavaScript and produce an inventory of pages, URLs,
endpoints, parameters, GraphQL signals, internal-host references, vulnerable
library hints, and redacted secret candidates. Authenticated where session
headers exist.

## Output (under `crawl/<host>/`)

- `pages/<sha256>.html`, `js/<sha256>.js` — content-addressed bodies.
- `index.json` — URL → content-hash map.
- `endpoints.json` — enriched API rows: `{ url, method, source, body_format,
  params, graphql, ... }`.
- `secrets.json` — regex-matched secret candidates with SHA-256 fingerprints
  (no plaintext).
- `internal-refs.json` — `*.internal`, `*.corp`, RFC1918 references for SSRF
  follow-up.
- `vulnerable-libraries.json` — embedded library versions with CVE hints.
- `graphql-candidates.json` — GraphQL signals. `graphql-schema.json` is
  written when a bounded in-scope introspection POST succeeds.

## CLI

```bash
mg-crawl acme-bounty https://www.acme.example.com
mg-crawl acme-bounty https://www.acme.example.com https://api.acme.example.com --depth 4
```

## Notes

- Authenticated mode uses `session.json` env-var-backed headers when present.
  Secrets never appear in `audit.log` or crawl output.
- Cross-host absolute URLs found in JavaScript are stored in
  `internal-refs.json`, never injected into the active-endpoint rows.
- GraphQL introspection is attempted only against in-scope hosts and only
  when JS signals warrant it.
