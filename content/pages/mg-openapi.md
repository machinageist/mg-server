---
title: "mg-openapi"
date: 2026-05-18
summary: "OpenAPI spec parser and security prober that builds minimal requests for every endpoint and flags auth bypass and sensitive exposure."
tags: [geistscope, cli, api]
---

## Purpose

Fetches or reads a JSON OpenAPI spec, builds minimal requests for every defined endpoint, and flags auth bypass, sensitive data exposure, and slow endpoint findings. JSON specs only; YAML conversion is out of scope. `--unauthenticated` replays each endpoint without auth headers to detect missing access controls. Numeric parameters substitute `1`; text parameters substitute `"test"`.

## Output

- `openapi/results-<timestamp>.json` — per-endpoint findings with finding type, status codes (authenticated vs. unauthenticated), and response time.

## CLI

```bash
mg-openapi acme-bounty --spec https://api.acme.example.com/openapi.json
mg-openapi acme-bounty --spec ./api.json --unauthenticated
```

## Notes

- `--spec` accepts a URL or local file path.
- Auth bypass detection compares authenticated vs. unauthenticated status codes; a 200 on both is a finding.
- Related: [mg-fuzz](/wiki/mg-fuzz) for parameter-level fuzzing after endpoint map is built, [mg-authz](/wiki/mg-authz) for multi-role access testing.
