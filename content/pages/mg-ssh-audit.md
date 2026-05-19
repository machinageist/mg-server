---
title: "mg-ssh-audit"
date: 2026-05-18
summary: "SSH configuration auditor that reads banner and KEXINIT from raw bytes to flag deprecated algorithms and weak configurations."
tags: [geistscope, cli, network]
---

## Purpose

Audit SSH servers in the engagement for deprecated key exchange algorithms,
weak MACs, old cipher suites, and configuration indicators that suggest an
outdated installation.

## Output

- `recon/ssh-audit.json` — per-host SSH result: banner string, server's
  advertised algorithm lists (kex, host-key, cipher, MAC), and flagged issues
  with severity.

## CLI

```bash
mg-ssh-audit acme-bounty
mg-ssh-audit acme-bounty --port 2222 --concurrency 10
```

## Notes

- No SSH library is used. The tool sends a KEXINIT packet constructed as raw
  bytes per RFC 4253 §7.1 and parses the server's KEXINIT response from the
  wire.
- Deprecated algorithms flagged: `diffie-hellman-group1-sha1`, `arcfour*`,
  `hmac-md5`, `hmac-sha1-96`, and similar.
- Banner is read from the initial TCP response before any handshake packet is
  sent, per the SSH protocol identification string exchange.
- Hosts are taken from `recon/summary.json` filtered to port 22 (or the port
  passed via `--port`).
