---
title: "mg-harness"
date: 2026-05-19
summary: "Scoped endpoint dispatcher and chat REPL between an AI/TUI/operator and the GeistScope toolchain. JSON dispatch for single-shot calls; an interactive coding-agent loop with tool use for ongoing engagements."
tags: [geistscope, cli, harness, ai, safety, chat]
---

## Purpose

`mg-harness` is the safe adapter between an AI agent (or a human operator
working interactively) and the GeistScope CLIs. Callers send a JSON
invocation that names an endpoint and arguments; the harness validates types,
applies risk policy, checks scope, dispatches to a Rust library or subprocess,
and returns a bounded JSON result.

It exists so an AI assistant can drive an engagement without raw shell access,
and so a human can do the same through one consistent interface.

## Two modes

```bash
# Single-shot — read one Invocation from stdin (or --input file), print one result
echo '{"endpoint":"engagement.status","engagement":"acme-bounty"}' | mg-harness dispatch --pretty
mg-harness dispatch --input invocation.json --pretty

# Chat REPL — interactive coding-agent loop pinned to one engagement
mg-harness chat acme-bounty --backend ollama --model qwen2.5-coder
mg-harness chat acme-bounty --backend openai --base-url http://localhost:1234/v1 --model llama-3-70b
mg-harness chat acme-bounty --backend anthropic --model claude-opus-4-7
```

## Endpoint contract

Every endpoint declares a risk class:

- `read_only` — no network, no state change.
- `passive_remote` — outbound HTTP to third-party intel services, no target contact.
- `low_active` — sends a small number of bounded requests to in-scope targets.
- `high_active` — bulk fuzzing, auth probes, exploitation payloads; requires `confirmed: true`.
- `state_change` — writes engagement state (e.g. records consent, imports traffic).
- `destructive` — post-exploitation; blocked unless the chat REPL is in `--unsafe-mode` or the JSON caller is explicitly authorized.

90+ endpoints are registered today. The core control surface:

| Endpoint | Risk | What it does |
|---|---|---|
| `endpoint.registry` | read | List every registered endpoint with its risk class and status. |
| `engagement.open` / `engagement.status` | read | Workspace metadata and output-file summary. |
| `scope.check` | read | Test a host or URL against `scope.json`. |
| `recon.run` | high_active | Run the [mg-recon](/wiki/mg-recon) pipeline after confirmation. |
| `graph.summary` / `graph.neighbors` | read | Read the local security graph with bounded sampling. |
| `request.import` / `request.search` / `request.replay` | mixed | Bring HAR / Burp / Caido traffic in; search the corpus; replay one. |
| `finding.create` / `finding.read` | read / state | Scoped finding creation and bounded reads. |
| `chain.read` | read | Bounded read of `recon/chain-analysis.{md,json}`. |
| `report.generate` / `report.disclose` | read | Run [mg-report](/wiki/mg-report) generate / disclose. |
| `re.analyze` / `re.read` | read | Drive [mg-recopilot](/wiki/mg-recopilot). |
| `aifuzz.consent` / `aifuzz.run` | state / high_active | Record consent then run [mg-aifuzz](/wiki/mg-aifuzz). |
| `exploit.scaffold` | read | Scaffold an [mg-exploitgen](/wiki/mg-exploitgen) tree. |
| `session.set` / `session.get_headers` | state / read | Manage env-var-backed session profiles; return only redacted header metadata. |

The 67 subprocess-dispatched tool endpoints (`xss.scan`, `sqli.scan`,
`jwt.analyze`, `tls.scan`, `aws.chain`, `graphql.scan`, `shodan.lookup`,
`takeover.scan`, …) all live in one `SUBPROCESS_TOOLS` const slice that pairs
each endpoint with its binary, risk class, and short description. Dispatch,
registry, and binary lookup all read from that single row, so adding a new
tool is a one-line change. Invoke `endpoint.registry` to enumerate the full
list at runtime.

## Standardized findings

Tools that detect a vulnerability emit a `ToolFinding` JSON record to
`engagements/<name>/findings/<tool>-<id>.json`. The id is a deterministic
SHA-256 over `(tool, url, parameter, title, discriminator)` so re-running
the same tool against the same target is idempotent. Evidence is capped at
2 KiB on disk with a `... [truncated]` marker.

`ai-prioritize` reads these findings sorted by severity and surfaces the top
50 as structured context to the LLM. `security-graph` ingests them as
`Finding` nodes with `DiscoveredBy` edges to the tool that produced them.

## Chat REPL

Tool calls flow through the same risk policy as the JSON path:

- `read_only` and `passive_remote` endpoints run immediately.
- `low_active`, `high_active`, and `state_change` endpoints prompt the user
  in the REPL with the proposed args before running. Pass `--yes` to skip
  the prompt for unattended runs.
- `destructive` endpoints (`privesc.linux`, `privesc.windows`, `loot.run`)
  are excluded from the tool catalog by default. Pass `--unsafe-mode` to
  include them.

Slash commands inside the REPL:

```
/help    show this help
/tools   list available tools and their risk classes
/reset   clear conversation history (keeps system prompt)
/quit    exit
```

The agent loop is capped at 20 tool turns per user message — long-running
investigations should be split across messages so the operator stays in the
loop.

## Docker

The shipped container image has `mg-harness` as its entrypoint:

```bash
docker run --rm -it --network host \
  -v "$PWD/engagements:/workspace/engagements" \
  ghcr.io/machinageist/geistscope:latest chat acme-bounty \
  --backend ollama --base-url http://localhost:11434 --model qwen2.5-coder
```

## Notes

- Every dispatch — allow, block, and error — is appended to `audit.log`.
- High-active endpoints refuse to run unless the caller sets `confirmed: true`
  in the JSON path. The chat REPL sets it automatically after the operator's
  in-REPL confirmation.
- Subprocess tool calls run with a 5-minute wall-clock timeout and capture at
  most 64 KiB of stdout / stderr each — runaway tools cannot starve the
  harness or OOM the agent.
- Model-visible response bodies are truncated to 256 KiB on UTF-8 char
  boundaries with a recorded byte-count.
- Credentials never appear in dispatch output. `session.get_headers` returns
  `<redacted>` for every value and a header count.
