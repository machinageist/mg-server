---
title: "Shared libraries"
date: 2026-05-15
summary: "Workspace crates shared across the GeistScope binaries: engagement, session, payload-engine, http-client, llm-client, fingerprint."
tags: [geistscope, library, crate]
---

## Purpose

These crates carry the load-bearing logic that the CLI binaries and the
harness reuse. Each one is small, opinionated, and exposes a public Rust
API rather than a CLI surface (with the exception of `fingerprint`, which
ships both).

## Crates

### `engagement`

Workspace model. `Engagement::init`, `Engagement::load_named`, scope
matching, audit logging, finding read/write, and the directory-helper
methods (`recon_dir`, `crawl_dir`, `findings_dir`, `re_dir`).

### `session`

Session-profile storage backed by environment-variable references —
**never** plaintext secrets in `session.json`. Public API:

```rust
pub async fn get_auth_headers(eng: &Engagement) -> Result<HeaderMap>;
pub async fn refresh_if_needed(eng: &Engagement) -> Result<()>;
pub async fn test_session(eng: &Engagement, test_url: &str) -> Result<bool>;
```

Form login, OAuth refresh, and stored-cookie encryption remain on the
roadmap.

### `payload-engine`

Stack-aware payload selection for fuzzing and harness planning. Knows
about MySQL vs Postgres vs MSSQL SQLi shapes, Jinja2/Twig/Freemarker SSTI
forms, AWS/GCP/Azure SSRF metadata URLs, and the
`PromptInjectionCategory` corpus used by [mg-aifuzz](/wiki/mg-aifuzz).

### `http-client`

Shared `reqwest` wrapper. User-agent rotation, rate limiting, retries,
timeout, and redirect controls. Accepts default headers from
[session](#session) so authenticated tools all behave consistently.

### `llm-client`

Unified Anthropic + Ollama client. Public surface is small:
`LlmClient::anthropic(key, model)`, `LlmClient::ollama(model)`,
`complete(system, user) -> String`.

### `fingerprint`

Reusable HTTP-response fingerprinting logic used by
[mg-fingerprint](/wiki/mg-fingerprint) and [mg-recon](/wiki/mg-recon).
Ships as both a library and a CLI binary.

## Conventions

All workspace crates follow the same conventions:

- File header block: filename, author, date, description, non-obvious notes.
- `// Verb + noun` comment above every function and major block.
- `ALL_CAPS_SNAKE_CASE` for constants — no inline magic strings.
- `cargo clippy -- -D warnings` must pass before commit.
