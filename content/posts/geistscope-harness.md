---
title: "GeistScope Part 10: The AI Harness"
date: 2026-05-16
summary: "mg-harness is the JSON API layer between an AI operator and the GeistScope toolchain. It accepts typed invocations, applies scope and risk policy, dispatches the right tool, and returns bounded structured results — so the AI works on findings, not raw terminal output."
tags: [rust, security, bug-bounty, geistscope, ai, tooling]
---

## The Problem With AI + CLI Tools

The obvious way to give an AI access to a security toolchain is shell commands:
let the model run `mg-fuzz --target api.example.com --payloads sqli` and read
stdout. That works until it doesn't:

- The model doesn't know which engagement it's in context of
- Stdout is unstructured; parsing it is fragile
- Nothing enforces that the AI only touches in-scope hosts
- There's no way to require human confirmation before an active attack runs
- The output might be 50KB of fuzz results that wastes the model's context window

`mg-harness` is the answer to all of those. It's a JSON-in, JSON-out dispatcher
that wraps every tool endpoint with scope enforcement, risk classification, output
bounding, and redaction.

---

## The Invocation Model

Every call to the harness is one JSON document:

```json
{
  "endpoint": "recon.subdomain_enum",
  "engagement": "target-bounty",
  "risk": "passive_remote",
  "reason": "Initial subdomain discovery before any active testing",
  "confirmed": false,
  "args": {
    "domain": "target.example.com",
    "mode": "All"
  }
}
```

And every response is one JSON document:

```json
{
  "endpoint": "recon.subdomain_enum",
  "status": "ok",
  "risk": "passive_remote",
  "summary": "Found 47 subdomains for target.example.com",
  "output_files": ["recon/subdomain-enum.json"],
  "data": { "count": 47, "new": 12 }
}
```

The AI reads the result, decides what to do next, and issues another invocation.
No shell parsing, no regex on stdout, no context blowout from raw tool output.

---

## Risk Classes and the Confirmation Gate

Every endpoint is registered with a risk class. The classes form an ordered scale:

| Class | Description |
|---|---|
| `read_only` | Reads engagement files only — no network |
| `passive_remote` | Reads public records: CT logs, DNS, Wayback — no target contact |
| `low_active` | Sends non-attack requests: port scan, HTTP probe |
| `high_active` | Sends attack payloads: fuzzing, injection testing |
| `state_change` | Modifies engagement state: adds findings, updates scope |
| `destructive` | Deletes engagement data — never dispatched automatically |

When an invocation arrives with `risk: high_active` or above, the harness checks
`confirmed: true`. If it's false, the endpoint returns a `blocked` status with a
policy explanation rather than dispatching:

```json
{
  "status": "blocked",
  "risk": "high_active",
  "policy": "high_active endpoints require confirmed: true in the invocation",
  "reason": null
}
```

The AI operator has to re-submit with `confirmed: true` after reasoning explicitly
about whether this action is appropriate. This is a forcing function, not just a
flag — the model has to produce the `reason` field and set `confirmed` deliberately.

---

## Output Bounding and Redaction

Tool output can be large. A fuzz run against a large API produces megabytes of
request/response pairs. Dumping all of that into the model's context would waste
tokens and make it harder to reason about what matters.

The harness applies a hard cap: `MAX_MODEL_VISIBLE_BYTES = 256KB`. Anything beyond
that is written to a file in the engagement directory and referenced by path in
`output_files`. The model gets a summary and a pointer; it can request a specific
section if it needs more.

Redactions track what got removed. If a large field was truncated, `redactions`
in the response maps field names to the number of bytes that were cut. The model
knows data was elided and can ask for it specifically.

---

## Scope Enforcement at the Dispatcher

Every active endpoint that takes a target URL or hostname runs it through the
engagement's scope rules before dispatching. If the target isn't in scope, the
harness returns `blocked` with a scope explanation, regardless of what `confirmed`
says. There's no override for scope — if it's not in scope, it doesn't run.

This matters because the AI may be operating across multiple endpoints in a single
engagement session and may infer a hostname from crawl results that isn't in the
original program scope. The harness is the last line before the tool runs.

---

## Endpoint Categories

Endpoints are grouped by tool:

- **engagement.*** — read/write engagement metadata, notes, findings, scope
- **recon.*** — subdomain enumeration, port scan, fingerprint, full pipeline
- **crawl.*** — crawl from URL, crawl status, corpus query
- **probe.*** — security posture check, result read
- **fuzz.*** — fuzz a target with a payload set, result read
- **replay.*** — re-verify a finding's curl evidence
- **graph.*** — query the security graph by node kind or neighbor relationship
- **report.*** — generate a finding report, list reportable findings
- **recopilot.*** — analyze decompiled pseudocode function
- **exploitgen.*** — scaffold an exploit project for a CVE

The dispatch function is a match on `invocation.endpoint`. Each arm parses
its `args` from the invocation JSON, checks scope, and calls the appropriate
library function. The result comes back as an `EndpointResult` that the binary
serializes to stdout.

---

## Using the Harness

From the command line, you can feed an invocation from a file or stdin:

```bash
echo '{"endpoint":"engagement.list_findings","engagement":"target-bounty","risk":"read_only"}' \
  | mg-harness --pretty

mg-harness --input invocation.json --pretty
```

From a Claude session with tool use, the model generates invocation JSON, the tool
call executes `mg-harness`, and the response comes back as a tool result. The entire
toolchain becomes callable from a structured conversation without any special
integration — just JSON over stdin/stdout.

---

*Part 11 covers the three AI analysis tools: `mg-report` for generating HackerOne-ready
finding reports, `mg-recopilot` for reverse-engineering analysis of decompiled
pseudocode, and `mg-exploitgen` for scaffolding exploit projects from CVE descriptions.*
