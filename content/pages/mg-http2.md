---
title: "mg-http2"
date: 2026-05-18
summary: "HTTP/2 misconfiguration detector: h2c cleartext upgrade, ALPN negotiation, rapid reset heuristic (CVE-2023-44487), and HPACK header size acceptance."
tags: [geistscope, cli, network]
---

## Purpose

Check HTTP/2 support and known misconfigurations on engagement hosts.
Identifies servers that accept cleartext HTTP/2 upgrades, servers that may be
vulnerable to rapid reset (CVE-2023-44487), and servers with permissive HPACK
header size limits.

## Output

- `recon/http2.json` — per-host HTTP/2 result: h2c upgrade accepted (yes/no),
  ALPN h2 negotiated (yes/no), rapid reset heuristic flag (INFO), HPACK max
  header list size, and flagged findings.

## CLI

```bash
mg-http2 acme-bounty
mg-http2 acme-bounty --concurrency 10
```

## Notes

- h2c cleartext upgrade is tested by sending an HTTP/1.1 `Upgrade: h2c`
  request and checking for a `101 Switching Protocols` response.
- ALPN h2 negotiation is checked during the TLS handshake on HTTPS hosts.
- Rapid reset (CVE-2023-44487) cannot be confirmed without sending raw RST
  frames. The tool flags it as INFO based on server version heuristics only:
  unpatched versions of known servers are noted, not confirmed vulnerable.
- HPACK header size is tested by sending a request with an oversized header
  block and checking whether the server rejects or accepts it.
- Hosts are sourced from `recon/summary.json`.
