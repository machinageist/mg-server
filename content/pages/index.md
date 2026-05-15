---
title: "GeistScope Tool Suite — Overview"
date: 2026-05-15
summary: "Index page for the GeistScope wiki: workspace layout, product direction, and the typical workflow. Each tool has its own page in the sidebar."
tags: [rust, security, bug-bounty, geistscope, wiki]
---

## What this is

GeistScope is a Rust-based bug bounty and red-team toolchain built around one
shared engagement workspace. Every binary writes to the same directory layout
so the human operator, the CLI tools, the TUI, and an AI assistant can read
and write the same files. The directory layout is the contract.

Each tool has its own wiki page — pick one from the sidebar.

The next product layer is a TUI-based bug-hunting browser backed by a scoped
AI harness. The AI never gets raw shell access; it calls typed tool endpoints
through `mg-harness`, which enforces scope, redaction, risk class, and audit
logging before anything runs.

## Workspace layout

Every target starts with `mg-engagement init`. That creates:

```text
engagements/<name>/
|-- engagement.json
|-- scope.json
|-- notes.md
|-- audit.log
|-- recon/
|   |-- subdomain-enum.json
|   |-- fingerprint.json
|   |-- mg-scan.json
|   |-- summary.json
|   |-- probe-report.json
|   |-- priorities.md
|   |-- priorities.json
|   |-- chain-analysis.md
|   |-- chain-analysis.json
|   `-- fuzz-<timestamp>.json
|-- crawl/<host>/
|   |-- pages/
|   |-- js/
|   |-- index.json
|   |-- endpoints.json
|   |-- secrets.json
|   |-- internal-refs.json
|   |-- vulnerable-libraries.json
|   |-- graphql-candidates.json
|   `-- graphql-schema.json
|-- findings/
|   |-- <id>-<slug>.md
|   |-- <id>-<slug>-report.md
|   |-- <id>-<slug>-cve.md
|   |-- <id>-<slug>-disclosure.eml
|   `-- <id>-<slug>-replay-<date>.json
|-- re/<binary>/
|   |-- manifest.json
|   |-- raw/<func>.c
|   |-- <func>.md
|   `-- <func>.json
|-- aifuzz/
|   |-- CONSENT
|   |-- sentinels.txt
|   `-- <run-id>.jsonl
`-- exploits/<cve>/
    |-- Cargo.toml
    |-- runbook.md
    |-- src/
    `-- tests/smoke.rs
```

`scope.json` is default-deny. Active tools check the scope before touching
any target.

## Product direction

```text
TUI browser
  -> local AI harness
       -> scoped Rust tool endpoints
            -> engagement workspace
```

The CLIs stay useful standalone. The harness is the safe adapter between an
AI API and the tools: strict schemas, narrow allowed tools, scope checks
before active traffic, bounded output capture, and redaction before
model-visible context. See [mg-harness](/wiki/mg-harness) for the endpoint
contract.

## Typical workflow

```bash
# Create and scope
mg-engagement init acme-bounty --target acme.example.com --platform hackerone
mg-engagement scope-add acme-bounty "*.acme.example.com"
mg-engagement scope-deny acme-bounty "*.dev.acme.example.com"

# Recon
mg-recon acme-bounty --ports 1-1024 --concurrency 100
mg-crawl acme-bounty https://www.acme.example.com https://api.acme.example.com

# Prioritize
ai-prioritize acme-bounty

# Posture and active checks
mg-probe acme-bounty --active

# Fuzz, replay
mg-fuzz acme-bounty --template idor.txt --payloads numbers:1-200 --mode sniper
mg-replay acme-bounty 20260514-probe-001

# Report and disclose
mg-report generate acme-bounty 20260514-probe-001
mg-report disclose acme-bounty 20260514-probe-001 \
    --vendor "Acme Corp" --contact security@acme.example
```

## Phase 2 — Cyberpunk wishlist

The first slice of the `CYBERPUNK_WISHLIST.md` Tier-S items has landed:

- [mg-recopilot](/wiki/mg-recopilot) — decompiled-pseudocode RE copilot.
- [mg-aifuzz](/wiki/mg-aifuzz) — adversarial prompt-injection fuzzer.
- [mg-exploitgen](/wiki/mg-exploitgen) — CVE-driven exploit project scaffold.
- [mg-report disclose](/wiki/mg-report) — CVE writeup and responsible-disclosure
  email pair.

The remaining Tier-A and Tier-B items are tracked in `HANDOFF.md` §14.

## Build and install

```bash
cd engine-rust
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings

for crate in engagement subdomain-enum mg-scan fingerprint mg-recon \
             ai-prioritize mg-crawl mg-probe mg-fuzz mg-replay mg-report \
             mg-recopilot mg-aifuzz mg-exploitgen mg-tui mg-harness; do
    cargo install --path "$crate"
done
```

## Governing docs

The source repo carries the docs that govern implementation:

- `docs/PRODUCT_DOCTRINE.md`
- `docs/BUG_HUNTING_METHODOLOGY.md`
- `docs/AI_TOOL_ENDPOINTS.md`
- `docs/FEATURE_ROADMAP.md`
- `HANDOFF.md` (§14 Phase 2 roadmap)
