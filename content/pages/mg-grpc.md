---
title: "mg-grpc"
date: 2026-05-18
summary: "gRPC service discovery via HTTP/2 reflection that enumerates services and tests unauthenticated method calls."
tags: [geistscope, cli, api, network]
---

## Purpose

Probes hosts for gRPC endpoints, enumerates services via server reflection, and tests unauthenticated method calls against discovered methods. No gRPC library is used; raw HTTP/2 is sent via reqwest with `http2_prior_knowledge()` for cleartext and normal HTTPS for TLS. gRPC framing is handled manually: 1-byte compression flag followed by a 4-byte big-endian message length.

## Output

- `grpc/results-<timestamp>.json` — discovered services and methods, unauthenticated call results, and reflection availability status.

## CLI

```bash
mg-grpc acme-bounty
mg-grpc acme-bounty --host api.acme.example.com --port 50051
```

## Notes

- Server reflection must be enabled on the target for service enumeration to work; note its absence as a finding if discovery fails.
- Unauthenticated call tests send empty proto messages; useful for finding methods that accept anonymous requests.
- Related: [mg-fuzz](/wiki/mg-fuzz) for message-level fuzzing after service map is known.
