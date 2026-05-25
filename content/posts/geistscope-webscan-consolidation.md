---
title: "GeistScope: Folding Fourteen Web Scanners into mg-webscan"
date: 2026-05-24
summary: "The second consolidation pass: the active web-vulnerability scanners collapse from fourteen single-purpose crates into one subcommand-routed binary, three low-signal tools get cut, and the DNS-backed tools move to hickory-resolver 0.26. The harness catalog the AI sees never changes."
tags: [rust, security, bug-bounty, geistscope, tooling, refactor]
---

The tool chest grew faster than it should have. A few months ago every new
idea became its own crate — its own `Cargo.toml`, its own `main.rs`, its own
compile unit and install step. That was fine at twenty tools and untenable at
eighty. The [first cleanup pass](/wiki/mg-artifact-audit) merged six passive
artifact analyzers into `mg-artifact-audit`. This is the second pass, and it
targets the noisiest part of the catalog: the active web scanners.

## The problem wasn't the harness — it was the crate count

`mg-harness` already solved dispatch. Every tool is one row in a table:
binary, endpoint, risk class, description. The AI never gets shell access; it
calls a typed endpoint and the harness enforces scope, redaction, risk, and
audit logging. Adding a tool was already a one-row edit.

So the cost of "eighty tools" was never dispatch complexity. It was eighty
manifests, eighty boilerplate entry points, eighty compile units, and eighty
`cargo install` passes. The fix is to group related single-purpose binaries
into one subcommand-routed binary — exactly what `mg-artifact-audit` proved
out.

## mg-webscan

[mg-webscan](/wiki/mg-webscan) merges the fourteen active web-vulnerability
scanners — XSS, SQLi, SSRF, SSTI, XXE, traversal, open redirect, CSRF, command
injection, CORS, cache poisoning, prototype pollution, deserialization, and
request smuggling — into one binary with one module per class. The detection
logic, payload sets, and unit tests were ported over verbatim; 108 tests came
along unchanged.

The harness endpoints are byte-for-byte identical. `xss.scan` still exists;
it just resolves to `mg-webscan xss` instead of a standalone `mg-xss` binary.
An operator or the AI sees the same catalog it always did:

```text
xss.scan      -> mg-webscan xss
sqli.scan     -> mg-webscan sqli
ssrf.scan     -> mg-webscan ssrf
cache.poison  -> mg-webscan cache-poison
...
```

The retired single-purpose pages stay in the wiki as redirects, the same way
the artifact-analyzer pages point at `mg-artifact-audit`.

## Cutting the low-signal tools

Consolidation is also a chance to ask which tools earn their keep. Three came
out entirely:

- **mg-social** — social-media profile enumeration is typically low signal for
  bounty and pentest work.
- **mg-screenshot** — needs a headless browser to be useful; heavy for the
  payoff.
- **mg-nuclei-bridge** — wrapped an external binary and added a runtime
  dependency for capability already covered elsewhere.

## A quieter fix: hickory-resolver 0.26

The DNS-backed tools — `mg-dns-enum`, `mg-cname-chain`, `mg-dns-rebind`,
`mg-shodan`, `mg-takeover` — moved from the 0.24 resolver API to 0.26. The
shape changed more than a version bump suggests: `TokioAsyncResolver` became
`TokioResolver` built through a builder, `Lookup` exposes `answers()` instead
of `iter()`, record data is a field matched against `RData` variants, and
NXDOMAIN detection runs through `NetError::is_no_records_found()`. Mechanical,
but spread across five crates.

## Where this lands

The net effect: roughly seventy-six binaries heading toward about twenty-two,
without changing a single endpoint the AI catalog or the operator sees. CI
builds fewer compile units, `cargo install` runs a fraction of the passes, and
the catalog is easier to reason about before the next pruning pass. The
directory layout is still the contract; the tools writing into it are just
better organized.
