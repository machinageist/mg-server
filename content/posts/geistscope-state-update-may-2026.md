---
title: "GeistScope State Update: Chat REPL, Docker, and Tool Catalog Cleanup"
date: 2026-05-19
summary: "Six weeks of changes consolidated: a large tool expansion wired through one harness, standardized findings flowing into the prioritizer and security graph, a coding-agent chat REPL with profile-filtered tool packs, a multi-stage Docker image, and the first catalog cleanup pass."
tags: [rust, security, bug-bounty, geistscope, ai, tooling, docker, ci]
---

The platform has moved past being a recon-and-fuzz toolchain. A pile of
changes landed over the last several weeks; this post pulls them into one
picture.

## Big tool expansion, then the first cleanup pass

The workspace grew fast — coverage now spans active
vulnerability detection (`mg-xss`, `mg-sqli`, `mg-ssti`, `mg-xxe`,
`mg-cmdinject`, `mg-traversal`, `mg-smuggle`, `mg-cache-poison`,
`mg-proto-pollute`, `mg-deser`), auth and session (`mg-jwt`, `mg-authz`,
`mg-oauth`, `mg-brute`, `mg-session-audit`), modern protocols
(`mg-graphql`, `mg-openapi`, `mg-grpc`, `mg-websocket`, `mg-http2`), JS and
client-side (`mg-artifact-audit`, `mg-csp`, `mg-cors-exploit`),
infrastructure (`mg-tls-scan`, `mg-ssh-audit`, `mg-udp-scan`, `mg-smtp`,
`mg-snmp`, `mg-smb`), cloud and container (`mg-aws`, `mg-gcp`, `mg-azure`,
`mg-k8s`, `mg-docker`, `mg-serverless`), OSINT (`mg-github`, `mg-shodan`,
`mg-dns-enum`, `mg-dns-history`, `mg-cloud-enum`, `mg-breach`, `mg-social`,
`mg-google-dork`, `mg-leak-monitor`), DNS abuse
(`mg-takeover`, `mg-cname-chain`, `mg-dns-rebind`), mobile artifact checks
(via `mg-artifact-audit apk` and `mg-artifact-audit ipa`), post-access
(`mg-privesc-linux`, `mg-privesc-windows`, `mg-loot`), and engagement workflow
(`mg-diff`, `mg-notify`, `mg-timeline`, `mg-nuclei-bridge`,
`mg-secret-validate`).

The follow-up cleanup collapsed six passive artifact analyzers into
`mg-artifact-audit`: `mg-js-analyze`, `mg-sourcemap`, `mg-apikey`,
`mg-metadata`, `mg-apk`, and `mg-ipa` are no longer standalone workspace
binaries. Their behavior remains available as subcommands and through the
legacy harness endpoints, while `artifact.audit` provides one high-level pass
for mixed artifact work.

The remaining binaries read from and write to the engagement workspace.
Every public capability is also reachable through `mg-harness` as a typed
endpoint with a risk class — see the [wiki](/wiki/mg-harness) for the
contract.

## Findings are now one shape

Every tool that detects something emits a `ToolFinding` JSON record to
`engagements/<name>/findings/<tool>-<id>.json`. The id is a deterministic
SHA-256 over `(tool, url, parameter, title, discriminator)`, so re-running
the same scan against the same target is idempotent — no duplicate records,
no silent overwrites of genuinely different findings. Evidence is capped at
2 KiB with a `... [truncated]` marker so downstream consumers can tell.

`ai-prioritize` now reads that directory, sorts findings by severity, and
ships the top 50 to the LLM as structured context — the model knows what's
already been confirmed before it ranks the rest of the attack surface.
`security-graph` ingests the same files as `Finding` nodes with
`DiscoveredBy` edges to the tools that produced them, so graph queries
finally answer "what have we found on this host, with what tool, at what
time."

## A chat REPL with three backends

`mg-harness chat <engagement>` opens an interactive coding-agent loop bound
to one engagement. The model gets profile-filtered tool-use access instead
of the entire harness catalog at once: `default` keeps common engagement,
request-corpus, graph, recon, and reporting endpoints visible; `advanced`
adds OOB/high-active helpers; `lab` exposes lab-only scaffolding, with
`--unsafe-mode` still required for destructive endpoints. The REPL remains
guarded:

- `read_only` and `passive_remote` endpoints fire immediately.
- `low_active`, `high_active`, and `state_change` endpoints prompt the
  operator with the proposed args before running.
- Noisy/rare endpoints are hidden by `--tool-profile` unless the operator
  selects `advanced` or `lab`; `destructive` endpoints are still hidden unless
  `--unsafe-mode` is passed.

Three backends ship in-tree, all behind a `ChatBackend` trait so the
conversation shape is identical regardless of vendor:

- **Anthropic** — Messages API with `tool_use` / `tool_result` content blocks.
- **OpenAI-compatible HTTP** — `/v1/chat/completions` works against the real
  OpenAI API, LM Studio, vLLM, llama.cpp's server, OpenRouter, etc.
- **Ollama** — native `/api/chat` with tools on `qwen2.5-coder`,
  `llama3.1`, `mistral-large`, and anything else local that supports tool
  calls.

Risk gating, scope enforcement, output bounding, and audit logging happen
inside the harness regardless of which backend the model is on. The agent
loop is capped at 20 tool turns per user message.

## Harness consolidation

The 67 subprocess-dispatched tool endpoints used to live in three places —
the dispatch match arm, the registry vec, and a binary-lookup map. A typo in
any one routed silently to the wrong handler. They're now a single
`SUBPROCESS_TOOLS` const slice with `(endpoint, binary, risk, description)`
per row; dispatch, registry, and binary lookup all read from it. The lib
file dropped about 400 lines. Adding a new tool is one row.

The next cleanup layer is now active: `endpoint.registry` reports each
endpoint's `ToolPack`, `ToolExposure`, and an optional repurpose note for
sharp tools, and `mg-harness chat` applies that metadata through
`--tool-profile default|advanced|lab`. The goal is not to delete useful code
just because the catalog got crowded. GeistScope hides noisy or hazardous
standalone endpoints from the default agent profile first, repurposes their
safe pieces into domain packs (`mg-reconx`, `mg-vuln-scan`,
`mg-identity-audit`, `mg-protocol-audit`, `mg-cloud-audit`,
`mg-artifact-audit`, eventing, and redteam-lab), then prunes standalone
affordances only after the replacement path exists.

The same pass added a 5-minute subprocess timeout with `kill_on_drop(true)`
and a 64 KiB cap on captured stdout/stderr — a runaway tool can no longer
hang the harness or starve the agent's context budget.

## Docker

A multi-stage `Dockerfile` ships every workspace binary in one
`debian:bookworm-slim` runtime image (~200 MB compressed). CI publishes it
to `ghcr.io/machinageist/geistscope` on every push to `main` and on tagged
releases. `mg-harness` is the entrypoint, so

```bash
docker run --rm -it --network host \
  -v "$PWD/engagements:/workspace/engagements" \
  ghcr.io/machinageist/geistscope:latest chat my-engagement \
  --backend ollama --model qwen2.5-coder
```

works straight from a fresh pull. The build runs entirely on `rustls` — no
openssl/native-tls — so the runtime image needs nothing more than
`ca-certificates`.

## CI and releases

- **`ci.yml`** caches the cargo registry and `target/` directory across
  runs; warm-cache builds drop from minutes to under a minute. `--locked`
  is now enforced everywhere.
- **`audit.yml`** runs `cargo audit --deny warnings` on Cargo.lock changes,
  weekly cron, and manual dispatch — RustSec advisories surface in CI
  instead of in build noise.
- **`release.yml`** is tag-triggered (`v*`). It builds stripped binary
  tarballs for x86_64-linux, x86_64-darwin, and aarch64-darwin, each with a
  SHA-256 sidecar, and attaches them to a GitHub release with
  auto-generated notes.
- **`docker.yml`** builds and pushes the container image on `main` and on
  tags, build-only on PRs so Dockerfile regressions don't slip through.

## What's next

The roadmap from here is shorter than the changelog: harden a handful of
detectors that are still thin (the `mg-grpc` protobuf decoder is a byte-scan
heuristic; `mg-http2` rapid-reset is a server-version match rather than a
real frame probe; `mg-cache-poison` status-override logic only catches the
narrow 200→4xx flip), and start using the chat REPL for actual engagements
to learn what the agent loop needs that it doesn't have yet. Streaming,
rustyline history, and per-endpoint JSON-Schema parameters are all
candidates depending on what real usage demands.
