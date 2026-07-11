---
title: "mg-tls-scan"
date: 2026-05-18
summary: "TLS/SSL configuration auditor that inspects negotiated parameters, cert validity, and cipher suites for HTTPS hosts in the engagement."
tags: [geistscope, cli, network, tls]
---

## Purpose

Audit TLS configuration on HTTPS hosts discovered during recon. Flags weak
cipher suites, expired or self-signed certificates, deprecated protocol
versions, and short key lengths.

## Output

- `recon/tls-scan.json` — per-host TLS result: negotiated protocol version,
  cipher suite, certificate chain summary (subject, issuer, expiry, SANs),
  and a list of flagged issues with severity.

## CLI

```bash
mg-tls-scan acme-bounty
mg-tls-scan acme-bounty --concurrency 20 --timeout-ms 5000
```

## Notes

- Uses rustls 0.23 and tokio-rustls 0.26 for the TLS handshake. Inspection
  is done directly, not routed through reqwest, so raw negotiated parameters
  are available.
- Certificate parsing uses x509-parser 0.16 on the DER-encoded cert from the
  handshake.
- TLS 1.0 and 1.1 are flagged as deprecated based on the negotiated version
  returned by rustls.
- Hosts are sourced from `recon/summary.json`. Only hosts with port 443 open
  (or HTTPS on alternate ports) are included.
- Self-signed certificates are detected by comparing issuer and subject CN,
  not by chain validation. This avoids false positives on internal CAs.
