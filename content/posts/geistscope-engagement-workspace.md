---
title: "GeistScope Part 1: The Engagement Workspace"
date: 2026-05-01
summary: "How I designed a bug bounty toolchain where every tool writes to the same file layout — and why that makes AI collaboration file-native with no custom IPC."
tags: [rust, security, bug-bounty, geistscope, tooling]
---

## The Problem With Toolchains

Most bug bounty recon looks like a pile of terminal tabs.
One window running `subfinder`, another with `httpx`, a third with `nuclei`,
outputs scattered across your home directory, nothing talking to anything else.
When you come back to a target a week later, you're piecing together what you ran
from bash history.

I wanted something different: a single workspace layout that every tool writes to,
a shared contract for what data lives where, and an audit trail that shows exactly
what ran and when. And I wanted it to be readable by an AI operator without custom
integration code — just files.

That's the core idea behind GeistScope.

---

## The Engagement Directory

Every target gets an *engagement directory*. When you initialize one:

```bash
mg-engagement init target-bounty \
    --target target.example.com \
    --platform hackerone
```

You get a directory like this:

```
engagements/target-bounty/
├── engagement.json     ← metadata: name, target, platform, created_at
├── scope.json          ← in_scope and out_of_scope patterns
├── notes.md            ← human-written notes, appended with mg-engagement note
├── audit.log           ← append-only record of every tool invocation
└── recon/              ← populated by mg-recon, mg-probe, ai-prioritize
└── crawl/              ← populated by mg-crawl, one subdirectory per host
└── findings/           ← populated by mg-probe, mg-fuzz, manually
```

The shape is the API. Every tool in the pipeline reads and writes to this
same layout using the paths defined by the `engagement` library.
When you run `mg-probe`, it looks for `recon/summary.json` — which `mg-recon` wrote.
When `ai-prioritize` runs, it reads `summary.json` and writes `priorities.md`.
No configuration files that say "here's where tool A puts its output for tool B."
The layout *is* the configuration.

---

## Scope Enforcement

One of the early design decisions was making scope enforcement a first-class feature
rather than something you remember to check manually.

`scope.json` stores two lists: `in_scope` and `out_of_scope`. Patterns are wildcard-
matched against hostnames — `*.target.example.com` covers all subdomains,
and explicit denies in `out_of_scope` let you carve out exclusions.

Every active tool — `subdomain-enum`, `mg-scan`, `mg-fingerprint`, `mg-crawl` — calls
`scope.is_in_scope(hostname)` before doing anything to a target. If a discovered
subdomain falls outside the pattern, it's dropped before any network probe goes out.
This matters in bug bounty: probing out-of-scope assets is a program violation.

You can manage scope from the CLI:

```bash
mg-engagement scope-add target-bounty "*.target.example.com"
mg-engagement scope-deny target-bounty "legacy.target.example.com"
mg-engagement check target-bounty "api.target.example.com"
# → IN SCOPE   api.target.example.com
```

The check subcommand returns exit code 2 for out-of-scope targets, which makes
it scriptable. If you're writing a wrapper script and need to gate on scope,
just check the exit code.

---

## The Audit Log

Every tool call appends a line to `audit.log`:

```
2026-05-08T14:23:01Z  subdomain-enum  target.example.com  count=47 mode=All
2026-05-08T14:24:18Z  mg-scan         api.target.example.com  open=3
2026-05-08T14:25:07Z  mg-fingerprint  api.target.example.com
2026-05-08T14:31:44Z  mg-recon        target-bounty
```

This is an append-only file. The `mg-engagement` library opens it with `OpenOptions::append(true)`
every time, so there's no way to overwrite a prior entry even by mistake.

The audit log serves two purposes. The practical one: when you come back to an
engagement days later, you can see exactly what ran, in what order, and when.
The security one: if you're running a toolchain against a real target, the audit log
is your record of what touched what. Bug bounty programs occasionally ask for
evidence of what you ran.

---

## Findings

A finding is a markdown file in `findings/` with structured frontmatter:

```markdown
---
title: "Missing HSTS on api.target.example.com"
severity: medium
status: draft
target: api.target.example.com
created: 2026-05-08T14:45:00Z
---

## Description

...

## Evidence

```bash
curl -I https://api.target.example.com
```
```

The ID is date-based and sequential: `2026-05-08-001`, `2026-05-08-002`.
The Evidence section uses literal `curl` commands — which `mg-replay` later
extracts and re-executes to verify the finding is still valid before submission.

`mg-probe` and `mg-fuzz` create findings automatically when they detect issues.
You can also create them manually:

```bash
mg-engagement finding target-bounty \
    "Missing HSTS on api subdomain" \
    api.target.example.com \
    --severity medium
```

The CLI enforces scope here too: if the target isn't in scope, the finding is
refused. That check is worth having — it's easy to accidentally attribute a
finding to a host you weren't authorized to test.

---

## Why File-Native AI Collaboration

The "no custom IPC" decision was deliberate. When I run a recon pipeline and want
an AI operator to help analyze results, I don't want to build a protocol for that.
The AI just reads the same files the tools wrote.

`summary.json` is a structured JSON document that describes every discovered host:
subdomains found, IPs resolved, open ports, fingerprinted tech stack, HTTP
accessibility status. An AI reading that file has everything it needs to suggest
what to look at first.

`priorities.md` is what `ai-prioritize` produces after sending that summary to
Claude or a local Ollama model alongside 18 bug-hunting skill files. It comes
back with a ranked table of attack surface, highest payout × exploitability first.
No API call from the AI operator to the toolchain — it reads a file.

That design keeps the AI collaboration stateless and auditable. You can read
`priorities.md` yourself. You can disagree with the ranking. You can see
exactly what the model was given, because the input is `summary.json` sitting
right there.

---

## What Comes Next

This post covers the workspace layer — the foundation everything else builds on.
The next posts in this series cover the actual pipeline: subdomain enumeration,
port scanning, fingerprinting, crawling, security posture checking, fuzzing,
and the terminal dashboard that ties it together.

Each one of those tools is its own Rust binary in the `engine-rust` workspace.
Each one reads the engagement directory, does its job, writes structured output,
and logs to the audit trail. The workspace contract is what makes them composable
without glue code.

*GeistScope is available on GitHub. All binaries install via `cargo install`.*
