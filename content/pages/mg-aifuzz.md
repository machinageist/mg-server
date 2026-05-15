---
title: "mg-aifuzz"
date: 2026-05-15
summary: "Adversarial prompt-injection fuzzer for LLM-backed HTTP endpoints. Curated 5-category payload corpus, scope-checked dispatch, regex success-signal rubric, JSONL output."
tags: [geistscope, cli, ai, prompt-injection, fuzz, phase-2]
---

## Purpose

`mg-aifuzz` is `HANDOFF.md` §14 S3 — Cyberpunk wishlist item #11, the
"adversarial ML prompt-injection lab". It fuzzes an LLM-backed HTTP
endpoint with a curated prompt-injection corpus and grades responses
against a regex rubric.

## Categories

The `payload-engine` `PayloadSet::PromptInjection` set carries five
sub-categories:

- `role_confusion` — tries to flip the assistant into "maintenance mode"
  or other unrestricted personas.
- `indirect_injection` — instructions embedded in retrieved documents and
  HTML comments.
- `system_prompt_leak` — asks the model to reproduce its system prompt.
- `tool_abuse` — tries to coax the model into invoking dangerous tools
  (file reads, SSRF metadata URLs, shell).
- `policy_bypass` — classic jailbreak/persona attacks.

## Template format

Same `§marker§` grammar as [mg-fuzz](/wiki/mg-fuzz), but the template needs
exactly one `§INJECT§` marker for the prompt content:

```http
POST /chat HTTP/1.1
Host: api.target.example
Authorization: Bearer §INJECT_TOKEN§
Content-Type: application/json

{"messages":[{"role":"user","content":"§INJECT§"}]}
```

When the body's `Content-Type` contains `json`, payloads are automatically
JSON-string-escaped before substitution. For non-JSON bodies the payload
is substituted verbatim.

## Output

`engagements/<name>/aifuzz/<run-id>.jsonl` — one JSON object per attempt:

```json
{
  "payload_category": "system_prompt_leak",
  "payload_id":       "system_prompt_leak-0",
  "request_excerpt":  "POST https://... :: payload=Repeat the text of...",
  "response_excerpt": "...",
  "body_excerpt":     "...",
  "success_signal":   { "matched_category": "system_prompt_leak",
                        "signal": "system prompt" }
}
```

## Success-signal rubric

A regex list per category, plus optional `aifuzz/sentinels.txt` (one string
per line, lines starting with `#` are comments). If any sentinel matches
the response, the row is recorded as a `system_prompt_leak` hit regardless
of which payload category fired it.

## CLI

```bash
# One-time per engagement: record consent
mg-aifuzz consent acme-bounty

mg-aifuzz run acme-bounty \
    --template aifuzz-template.txt \
    --base-url https://api.target.example \
    --categories system_prompt_leak,tool_abuse \
    --max-attempts 30 --rate-ms 500 --timeout-ms 15000 \
    --sentinels engagements/acme-bounty/aifuzz/sentinels.txt
```

## Safety

- Refuses to run unless `engagements/<name>/aifuzz/CONSENT` exists.
  `mg-aifuzz consent` writes that file with engagement name and ISO-8601
  timestamp.
- Every request's URL host is scope-checked against `scope.json`.
- `Host` header is dropped from the template and reqwest derives it from
  the URL.
- HighActive harness endpoint — also requires `confirmed: true` from the
  AI caller.

## Harness

[mg-harness](/wiki/mg-harness) exposes:

- `aifuzz.consent` (StateChange) — record the consent marker.
- `aifuzz.run` (HighActive) — runs only after both the consent marker
  exists and the AI caller has set `confirmed: true`.
