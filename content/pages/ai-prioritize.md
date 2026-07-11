---
title: "ai-prioritize"
date: 2026-05-15
summary: "LLM-ranked attack surface and exploit-chain reasoning from recon and probe artifacts."
tags: [geistscope, cli, ai, prioritization]
---

## Purpose

Read `recon/summary.json` together with bug-hunting skill files and ask an
LLM to rank targets by likely impact and exploitability. A second pass
reasons about exploit chains across the ranked findings.

Anthropic is preferred when `ANTHROPIC_API_KEY` is set; otherwise the tool
falls back to local Ollama.

## Output

- `recon/priorities.md` — human-readable ranked notes appended per run.
- `recon/priorities.json` — latest structured priority list.
- `recon/chain-analysis.md` and `recon/chain-analysis.json` — exploit-chain
  hypotheses and missing-evidence notes for follow-up testing.

## CLI

```bash
ai-prioritize acme-bounty
ai-prioritize acme-bounty --model sonnet-4-6 --ollama-model llama3.2
```

## Notes

- The output is advisory. It does not replace scope checks or manual
  validation.
- Probe-report evidence is included when present and treated as untrusted
  data inside `<probe_report>` blocks.
- See [mg-harness](/wiki/mg-harness) `chain.read` for the AI-callable read
  endpoint over the chain analysis artifacts.
