---
title: "mg-report"
date: 2026-05-15
summary: "Generate HackerOne-style reports with locally computed CVSS, plus CVE writeups and disclosure email drafts."
tags: [geistscope, cli, reporting, cvss, disclosure]
---

## Purpose

Turn one finding markdown file into a polished submission. Two modes:

- `generate` — HackerOne-formatted report (`<id>-<slug>-report.md`) with a
  locally computed CVSS 3.1 score.
- `disclose` — CVE-style writeup (`<id>-<slug>-cve.md`) and a deterministic
  responsible-disclosure email envelope (`<id>-<slug>-disclosure.eml`).

The LLM is only used to draft prose. CVSS scoring is local Rust math, not
model output.

## CLI

```bash
mg-report generate acme-bounty 20260514-probe-001
mg-report generate acme-bounty --all-unconfirmed

mg-report disclose acme-bounty 20260514-probe-001 \
    --vendor "Acme Corp" \
    --contact security@acme.example \
    --timeline-days 90
```

`--offline` produces a deterministic skeleton without calling any LLM.

## Output

- `findings/<id>-<slug>-report.md` — bounty-ready Markdown with severity,
  CVSS vector and score, summary, repro, impact, PoC, fix, references.
- `findings/<id>-<slug>-cve.md` — CVE-style writeup with **Affected
  Versions**, **Vulnerability Type**, **Technical Description**,
  **Reproduction Steps**, **Impact**, **CWE**, **Patch Guidance**.
- `findings/<id>-<slug>-disclosure.eml` — RFC-822 email with custom
  `X-GeistScope-Meta` header carrying `vendor`, `timeline_days`, and
  `reported_on`. The body is rendered locally — no LLM creativity in the
  email.

## Safety

- Vendor and contact strings are rejected if they contain CR/LF — no
  RFC-822 header injection from a malicious finding.
- All finding, engagement, and fingerprint inputs are wrapped as
  `<finding_markdown>` / `<engagement_context>` / `<fingerprint>` untrusted
  blocks before reaching the model.
- The disclosure `.eml` is a draft. The human operator reviews and sends it.

## Harness

[mg-harness](/wiki/mg-harness) exposes `report.generate` and
`report.disclose` (both ReadOnly).
