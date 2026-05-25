---
title: "mg-webscan"
date: 2026-05-24
summary: "Merged active web-vulnerability scanner: XSS, SQLi, SSRF, SSTI, XXE, traversal, open redirect, CSRF, command injection, CORS, cache poisoning, prototype pollution, deserialization, and request smuggling in one subcommand-routed CLI."
tags: [geistscope, cli, web, active-scan, webscan]
---

## Purpose

`mg-webscan` collapses the fourteen active web-vulnerability scanners into one subcommand-routed CLI. It applies the same consolidation pattern as [mg-artifact-audit](/wiki/mg-artifact-audit): one binary, one harness tool pack, one module per vulnerability class. Each scanner reads injectable endpoints from the crawl corpus, exercises them against scoped targets, and writes findings into the engagement workspace.

The harness endpoints are unchanged — only the underlying binary and a subcommand changed:

| Endpoint | Subcommand |
|---|---|
| `xss.scan` | `mg-webscan xss` |
| `sqli.scan` | `mg-webscan sqli` |
| `ssrf.scan` | `mg-webscan ssrf` |
| `ssti.scan` | `mg-webscan ssti` |
| `xxe.scan` | `mg-webscan xxe` |
| `traversal.scan` | `mg-webscan traversal` |
| `redirect.scan` | `mg-webscan redirect` |
| `csrf.scan` | `mg-webscan csrf` |
| `cmdinject.scan` | `mg-webscan cmdinject` |
| `cors.scan` | `mg-webscan cors` |
| `cache.poison` | `mg-webscan cache-poison` |
| `proto.pollute` | `mg-webscan proto-pollute` |
| `deser.scan` | `mg-webscan deser` |
| `smuggle.scan` | `mg-webscan smuggle` |

## CLI

```bash
mg-webscan xss acme-bounty
mg-webscan sqli acme-bounty
mg-webscan ssrf acme-bounty --oob-url https://oob.example
mg-webscan cors acme-bounty
mg-webscan smuggle acme-bounty
```

Each subcommand takes the engagement name as a positional argument and shares the standard `--engagements-dir` flag; class-specific flags (e.g. `--oob-url`, `--concurrency`, `--timeout`) carry over from the original tools unchanged.

## Output

Each class writes `results-<timestamp>.json` under its own subdirectory in the engagement workspace and emits `ToolFinding` records that flow into `ai-prioritize` and the security graph.

## Harness behavior

`mg-harness` routes each endpoint to `mg-webscan` with the matching subcommand. These are active endpoints in the `vuln_scan` pack: they check engagement scope before touching a target, and the high-active ones require operator confirmation in the chat REPL.

## Why this exists

The active scanners were fourteen near-identical single-purpose crates — each one a `Cargo.toml`, a `main.rs`, a compile unit, and an install. Folding them into one subcommand binary cuts the binary count and CI surface without changing what the AI catalog or operator sees. It is the active-testing counterpart to the `mg-artifact-audit` merge.

## Notes

- Retired standalone binaries: `mg-xss`, `mg-sqli`, `mg-ssrf`, `mg-ssti`, `mg-xxe`, `mg-traversal`, `mg-redirect`, `mg-csrf`, `mg-cmdinject`, `mg-cors-exploit`, `mg-cache-poison`, `mg-proto-pollute`, `mg-deser`, `mg-smuggle`.
- Their wiki pages are kept as compatibility/reference pages and point here.
- Logic is unchanged from the originals — the per-class detection, payloads, and tests were ported verbatim into modules.
