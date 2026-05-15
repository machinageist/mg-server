---
title: "mg-probe"
date: 2026-05-15
summary: "Passive posture plus bounded active endpoint checks for reflected markers, SQL errors, and no-follow open redirects."
tags: [geistscope, cli, active, posture]
---

## Purpose

Two phases share one binary. The passive phase grades security posture
(missing headers, weak cookies, CORS, debug paths). The `--active` phase
sends harmless probes — never weaponized payloads — and writes any signals
as draft findings for follow-up.

## Output

- `recon/probe-report.json` — passive posture grades and active-check
  observations.
- `findings/*.md` — candidate finding skeletons created when an active
  check signals a hit (e.g. reflected marker, SQL error string, off-origin
  redirect).

## CLI

```bash
mg-probe acme-bounty                # passive only
mg-probe acme-bounty --active       # adds active checks
mg-probe acme-bounty --active --rate-ms 200 --max-requests 200
```

## Active checks today

- Reflected-marker probe (no JavaScript execution payloads).
- Single-quote SQL error probe.
- No-follow open-redirect probe.
- Bounded debug-path enumeration (`/.env`, `/actuator`, `/__debug__`, ...).

## Notes

- Session-aware: reads env-var-backed headers from `session.json` when
  configured.
- Two-session IDOR diff, subdomain takeover, and OOB-driven SSRF remain
  open in `HANDOFF.md` §4.
- Picks nonstandard ports from `recon/summary.json` so internal-test targets
  (e.g. localhost:8080) are still probeable.
