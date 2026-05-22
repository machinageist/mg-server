---
title: "GeistScope Tool Suite — Overview"
date: 2026-05-19
summary: "Index page for the GeistScope wiki: workspace layout, product direction, chat REPL, and the typical workflow. Each tool has its own page in the sidebar."
tags: [rust, security, bug-bounty, geistscope, wiki]
---

## What this is

GeistScope is a Rust-based bug bounty and red-team toolchain built around one
shared engagement workspace. Every binary writes to the same directory layout
so the human operator, the CLI tools, the TUI, and an AI assistant can read
and write the same files. The directory layout is the contract.

Each tool has its own wiki page — pick one from the sidebar.

The product layer is a TUI-based bug-hunting browser backed by a scoped AI
harness. The AI never gets raw shell access; it calls typed tool endpoints
through `mg-harness`, which enforces scope, redaction, risk class, and audit
logging before anything runs. `mg-harness chat <engagement>` opens an
interactive coding-agent REPL bound to one engagement, with backends for
Ollama (local), any OpenAI-compatible HTTP server, or Anthropic — see
[mg-harness](/wiki/mg-harness).

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

## Additional tools

**Analysis and exploit dev:**
[mg-recopilot](/wiki/mg-recopilot), [mg-aifuzz](/wiki/mg-aifuzz), [mg-exploitgen](/wiki/mg-exploitgen), [mg-report](/wiki/mg-report)

**Recon expansion:**
[mg-whois](/wiki/mg-whois), [mg-shodan](/wiki/mg-shodan), [mg-dns-enum](/wiki/mg-dns-enum), [mg-dns-history](/wiki/mg-dns-history), [mg-cloud-enum](/wiki/mg-cloud-enum), [mg-cname-chain](/wiki/mg-cname-chain), [mg-udp-scan](/wiki/mg-udp-scan), [mg-screenshot](/wiki/mg-screenshot), [mg-takeover](/wiki/mg-takeover), [mg-vhost](/wiki/mg-vhost)

**OSINT:**
[mg-github](/wiki/mg-github), [mg-breach](/wiki/mg-breach), [mg-social](/wiki/mg-social), [mg-google-dork](/wiki/mg-google-dork), [mg-leak-monitor](/wiki/mg-leak-monitor)

**Vulnerability testing:**
[mg-xss](/wiki/mg-xss), [mg-sqli](/wiki/mg-sqli), [mg-ssti](/wiki/mg-ssti), [mg-cmdinject](/wiki/mg-cmdinject), [mg-xxe](/wiki/mg-xxe), [mg-traversal](/wiki/mg-traversal), [mg-redirect](/wiki/mg-redirect), [mg-csrf](/wiki/mg-csrf), [mg-smuggle](/wiki/mg-smuggle), [mg-cors-exploit](/wiki/mg-cors-exploit), [mg-cache-poison](/wiki/mg-cache-poison), [mg-proto-pollute](/wiki/mg-proto-pollute), [mg-deser](/wiki/mg-deser)

**Auth and session:**
[mg-jwt](/wiki/mg-jwt), [mg-authz](/wiki/mg-authz), [mg-oauth](/wiki/mg-oauth), [mg-session-audit](/wiki/mg-session-audit), [mg-brute](/wiki/mg-brute)

**SSRF and cloud metadata:**
[mg-ssrf](/wiki/mg-ssrf), [mg-oob](/wiki/mg-oob), [mg-aws](/wiki/mg-aws), [mg-gcp](/wiki/mg-gcp), [mg-azure](/wiki/mg-azure), [mg-serverless](/wiki/mg-serverless), [mg-k8s](/wiki/mg-k8s), [mg-docker](/wiki/mg-docker)

**Network services:**
[mg-tls-scan](/wiki/mg-tls-scan), [mg-ssh-audit](/wiki/mg-ssh-audit), [mg-smtp](/wiki/mg-smtp), [mg-snmp](/wiki/mg-snmp), [mg-smb](/wiki/mg-smb), [mg-http2](/wiki/mg-http2)

**API surface:**
[mg-graphql](/wiki/mg-graphql), [mg-openapi](/wiki/mg-openapi), [mg-grpc](/wiki/mg-grpc), [mg-websocket](/wiki/mg-websocket)

**Mobile and static analysis:**
[mg-artifact-audit](/wiki/mg-artifact-audit), [mg-secret-validate](/wiki/mg-secret-validate), [mg-csp](/wiki/mg-csp)

**Post-access:**
[mg-privesc-linux](/wiki/mg-privesc-linux), [mg-privesc-windows](/wiki/mg-privesc-windows), [mg-loot](/wiki/mg-loot)

**Workflow:**
[mg-diff](/wiki/mg-diff), [mg-notify](/wiki/mg-notify), [mg-timeline](/wiki/mg-timeline), [mg-nuclei-bridge](/wiki/mg-nuclei-bridge), [mg-dns-rebind](/wiki/mg-dns-rebind)

## Build and install

```bash
cd crates
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Install all binaries
cargo install --path engagement
for crate in subdomain-enum mg-scan fingerprint mg-recon corpus-builder \
             mg-crawl mg-probe mg-fuzz mg-replay mg-tui mg-harness mg-report \
             mg-recopilot mg-aifuzz mg-exploitgen \
             mg-whois mg-shodan mg-dns-enum mg-dns-history mg-cloud-enum \
             mg-cname-chain mg-udp-scan mg-screenshot mg-takeover mg-vhost \
             mg-github mg-breach mg-social mg-google-dork \
             mg-leak-monitor mg-xss mg-sqli mg-ssti mg-cmdinject mg-xxe \
             mg-traversal mg-redirect mg-csrf mg-smuggle mg-cors-exploit \
             mg-cache-poison mg-proto-pollute mg-deser mg-jwt mg-authz \
             mg-oauth mg-session-audit mg-brute mg-ssrf mg-oob \
             mg-aws mg-gcp mg-azure mg-serverless mg-k8s mg-docker \
             mg-tls-scan mg-ssh-audit \
             mg-smtp mg-snmp mg-smb mg-http2 mg-graphql mg-openapi mg-grpc \
             mg-websocket mg-artifact-audit \
             mg-secret-validate mg-csp mg-privesc-linux mg-privesc-windows \
             mg-loot mg-diff mg-notify mg-timeline mg-nuclei-bridge \
             mg-dns-rebind; do
    cargo install --path "$crate"
done
```

## Docker

A multi-stage `Dockerfile` ships all 79 workspace binaries in one
`debian:bookworm-slim` runtime image. CI publishes it to
`ghcr.io/machinageist/geistscope` on every push to `main` and on tagged
releases. Mount a host directory at `/workspace/engagements` to persist
findings between runs.

```bash
docker pull ghcr.io/machinageist/geistscope:latest

# Single-shot endpoint dispatch
echo '{"endpoint":"endpoint.registry","engagement":"foo"}' | \
  docker run --rm -i -v "$PWD/engagements:/workspace/engagements" \
    ghcr.io/machinageist/geistscope:latest dispatch

# Chat REPL against a local Ollama on the host
docker run --rm -it --network host \
  -v "$PWD/engagements:/workspace/engagements" \
  ghcr.io/machinageist/geistscope:latest chat my-engagement \
  --backend ollama --model qwen2.5-coder
```

## Source repo

[github.com/machinageist/geistscope](https://github.com/machinageist/geistscope).
Continuous integration runs build / clippy / cargo test / `cargo-audit` /
integration smoke against a deliberately-vulnerable Docker target. Tagged
releases produce Linux x86_64 and macOS (Intel + Apple Silicon) binary
tarballs alongside the container image.
