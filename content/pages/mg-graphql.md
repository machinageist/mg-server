---
title: "mg-graphql"
date: 2026-05-18
summary: "GraphQL security tester covering endpoint detection, introspection, schema extraction, dangerous mutation flagging, batching abuse, and depth-limit checks."
tags: [geistscope, cli, api, graphql]
---

## Purpose

Detects GraphQL endpoints, runs introspection to extract the full schema, flags dangerous mutations (delete, admin, password reset patterns), tests query batching abuse, and checks for absent depth limits. Endpoint candidates come from the `--endpoint` flag, crawl `endpoints.json` (graphql:true or `/graphql` path suffix), or recon summary hosts. Session auth headers are applied to all requests.

## Output

- `graphql/results-<timestamp>.json` — introspection schema, dangerous mutation list, batching test result, and depth-limit finding.

## CLI

```bash
mg-graphql acme-bounty
mg-graphql acme-bounty --endpoint https://api.acme.example.com/graphql
```

## Notes

- Run [mg-crawl](/wiki/mg-crawl) and [mg-recon](/wiki/mg-recon) first to maximize endpoint candidate coverage.
- Introspection disabled on the target is a finding in itself; note it and test blind query patterns manually.
- Batching abuse test sends an array of queries in a single request; flag if all are processed.
- Related: [mg-fuzz](/wiki/mg-fuzz) for field-level fuzzing after schema is known.
