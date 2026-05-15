---
title: "mg-harness"
date: 2026-05-15
summary: "Scoped JSON endpoint dispatcher between the AI/TUI and the rest of the GeistScope toolchain."
tags: [geistscope, cli, harness, ai, safety]
---

## Purpose

`mg-harness` is the safe adapter between an AI API (or the TUI) and the
GeistScope CLIs. Callers send a JSON invocation that names an endpoint and
arguments; the harness validates types, applies risk policy, checks scope,
dispatches to a Rust library, and returns a bounded JSON result.

It exists so an AI assistant can drive an engagement without raw shell access.

## Endpoint contract

Every endpoint declares a risk class:

- `read_only` — no network, no state change.
- `passive_remote` — outbound HTTP, no payloads.
- `low_active` — sends a small number of bounded payloads.
- `high_active` — bulk fuzzing or auth probes; requires `confirmed: true`.
- `state_change` — writes engagement state (e.g. records consent).
- `destructive` — bulk destructive operations; never used yet.

Endpoints implemented today:

| Endpoint | Risk | What it does |
|---|---|---|
| `endpoint.registry` | read | List endpoints with risk classes and status. |
| `engagement.open` / `engagement.status` | read | Workspace metadata and output-file summary. |
| `scope.check` | read | Test a host or URL against `scope.json`. |
| `recon.run` | high_active | Run the [mg-recon](/wiki/mg-recon) pipeline after confirmation. |
| `finding.create` / `finding.read` | read | Scoped finding creation and bounded reads. |
| `chain.read` | read | Bounded read of `recon/chain-analysis.{md,json}`. |
| `report.generate` | read | Run [mg-report generate](/wiki/mg-report). |
| `report.disclose` | read | Run [mg-report disclose](/wiki/mg-report). |
| `re.analyze` / `re.read` | read | Drive [mg-recopilot](/wiki/mg-recopilot). |
| `aifuzz.consent` | state_change | Record adversarial AI-fuzz consent. |
| `aifuzz.run` | high_active | Run [mg-aifuzz](/wiki/mg-aifuzz). |
| `exploit.scaffold` | read | Scaffold an [mg-exploitgen](/wiki/mg-exploitgen) tree. |
| `session.set` / `session.get_headers` | state / read | Manage env-var-backed session profiles, return only redacted header metadata. |

## Invocation

```bash
mg-harness call --engagement acme-bounty --endpoint engagement.status
echo '{"endpoint":"scope.check","engagement":"acme-bounty","args":{"target":"api.acme.example.com"}}' \
    | mg-harness stdin
```

## Notes

- Every dispatch — allow, block, and error — is appended to `audit.log`.
- High-active endpoints refuse to run unless the caller sets `confirmed: true`.
- Model-visible bodies are truncated to 256 KiB on UTF-8 char boundaries.
- Credentials never appear in dispatch output. `session.get_headers` returns
  `<redacted>` for every value and a header count.
