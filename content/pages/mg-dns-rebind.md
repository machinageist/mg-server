---
title: "mg-dns-rebind"
date: 2026-05-18
summary: "DNS rebinding payload generator that resolves target TTL, checks for private IP, and writes a ready-to-use HTML attack payload."
tags: [geistscope, cli, recon]
---

## Purpose

Generates a DNS rebinding attack payload for a target domain. Resolves the domain's
current DNS TTL, checks whether the resolved IP falls in a private range, and writes
an HTML file that exploits the rebinding window. The HTML payload assumes the attacker
controls a domain that can flip DNS between the attacker IP and the target IP after
the TTL expires. Active probing is limited to DNS resolution and the private-IP check;
no HTTP requests are sent to the target.

## Output

- `dns-rebind/payload.html` — ready-to-use rebinding payload; attacker serves this
  from their controlled domain.
- `dns-rebind/findings.json` — target domain, resolved IP, private-IP flag, TTL,
  and attacker domain used in the payload.

## CLI

```bash
mg-dns-rebind acme-bounty
mg-dns-rebind acme-bounty --attacker-domain attack.example.com
```

## Notes

- `--attacker-domain` defaults to a placeholder if omitted; edit the HTML before
  use.
- The private-IP check uses standard RFC1918 and RFC4193 ranges.
- TTL is recorded as a reference for timing the rebinding window; actual attack
  timing depends on the target resolver's caching behavior.
