---
title: "GeistScope Part 11: AI-Powered Analysis and Reporting"
date: 2026-05-16
summary: "Three tools that close the loop between findings and action: mg-report generates HackerOne-ready reports with CVSS scoring. mg-recopilot analyzes decompiled pseudocode for exploit primitives. mg-exploitgen scaffolds a complete exploit project from a CVE description and target-env spec."
tags: [rust, security, bug-bounty, geistscope, ai, reporting, reverse-engineering]
---

## After the Pipeline Runs

Parts 1–10 of this series covered how GeistScope discovers, probes, and documents
vulnerabilities. The pipeline ends with a `findings/` directory of markdown files.
Three tools handle what comes next: turning a finding into a reportable submission,
understanding what a decompiled binary is doing, and building an exploit scaffold
from a CVE.

All three use the same `llm-client` backend — Anthropic's Claude Sonnet 4.6 when
`ANTHROPIC_API_KEY` is set, a local Ollama model otherwise, and a deterministic
offline mode for testing.

---

## `mg-report`: From Finding to Submission

HackerOne reports follow a structure: title, severity with CVSS vector, description,
steps to reproduce, impact, and evidence. Writing that from scratch for every finding
is time-consuming and inconsistent. `mg-report` generates a ready-to-submit draft.

```bash
mg-report generate target-bounty 2026-05-15-003
```

The tool reads the finding markdown, the engagement `scope.json`, and the
fingerprinter's `summary.json`. It passes all three to the LLM with a structured
prompt that asks for each section of the report explicitly. The response gets written
to `findings/reports/2026-05-15-003.md`.

CVSS scoring is computed in Rust, not by the model. The `cvss` module takes the
finding's severity and category and returns a vector string and numeric score:

```
CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N → 9.1 Critical
```

The score feeds into the report template so the model doesn't have to reason about
CVSS math — it just fills in the narrative sections.

**Bulk mode.** When you want to generate reports for everything that's ready:

```bash
mg-report generate target-bounty --all-unconfirmed
```

This iterates findings with status `confirmed` or `triaged` and generates reports
for any that don't have one yet. `--force` overwrites existing reports if you want
to regenerate after changing the finding description.

**Disclosure.** For findings that need coordinated disclosure rather than a bug
bounty submission:

```bash
mg-report disclose target-bounty 2026-05-15-003 \
    --vendor "ACME Corp" \
    --contact security@acme.example.com \
    --timeline-days 90
```

This writes two files: a CVE-formatted writeup in `findings/disclosures/` and a
disclosure email draft ready to send. The 90-day timeline is the default; it goes
into the email with the calculated disclosure date.

---

## `mg-recopilot`: Reverse-Engineering with Context

Binary analysis — specifically, reading decompiler output — is one of those tasks
where an LLM is genuinely useful. The variable names are garbage, the control flow
is tangled, and pattern-matching against known vulnerability classes is exactly
what models are good at.

`mg-recopilot` operates on decompiled pseudocode dropped into the engagement
directory under `re/<binary>/raw/<function>.c`. The output goes next to the input:
`<function>.md` (human-readable analysis) and `<function>.json` (structured data
for the harness).

```bash
mg-recopilot analyze target-bounty \
    --binary target-api \
    --function process_user_input
```

The analysis prompt instructs the model to produce six structured sections:

- **Function Purpose** — what this function is supposed to do
- **Variable Map** — local variables and their likely semantic meaning
- **Control Flow Notes** — branches, loops, edge cases
- **Suspicious Logic** — anything that looks like it could be a vulnerability
- **Exploit Primitives** — concrete primitives present: buffer overflows, use-after-free,
  integer overflow, format string, type confusion, etc.
- **Suggested Next Steps** — what to look at next: which call sites to check,
  which inputs to trace, which offsets to verify in a debugger

The model is given the raw pseudocode (capped at 128KB) plus any binary manifest
dropped in `re/<binary>/manifest.json` — architecture, compiler flags, linked
libraries, known CVEs for the binary version. That context changes what counts as
suspicious: a `memcpy` without bounds check is more interesting in a network-facing
binary than in an offline utility.

---

## `mg-exploitgen`: Scaffolding From a CVE

When you have a CVE description and a target environment, the gap between "I know
this is vulnerable" and "I have a working exploit" is mostly research and boilerplate.
`mg-exploitgen` generates the scaffold that fills that gap.

```bash
mg-exploitgen scaffold target-bounty \
    --cve CVE-2026-0001 \
    --cve-description cve.md \
    --target-env target-env.json
```

`cve.md` is a plain text description of the vulnerability — the NVD advisory,
the researcher's writeup, the vendor's patch notes. `target-env.json` describes
the deployment: OS, architecture, compiler version, ASLR/NX status, whether PIE
is enabled, what protections the binary has.

The model generates a scaffold under `engagements/<name>/exploits/CVE-2026-0001/`:

```
exploits/CVE-2026-0001/
├── runbook.md          ← step-by-step exploitation notes
├── exploit.py          ← skeleton exploit with offsets TBD
├── notes.md            ← researcher notes, open questions
└── references.md       ← CVE links, patch commits, related writeups
```

No network access. The tool reads from disk and calls the LLM. It doesn't verify
the CVE is real, doesn't query external databases, and doesn't download anything.
The quality of the scaffold depends entirely on the quality of the description and
env spec you provide.

The runbook is the most useful output — it's a prose explanation of the exploitation
path given what the model knows about the vulnerability class and the target
constraints. An experienced researcher will recognize what's plausible and what
needs verification. A less experienced one gets a structured starting point rather
than a blank page.

---

## The Pattern Across All Three

Each tool reads engagement context, calls `llm-client.complete(system, user)`,
and writes structured output. The `--offline` flag skips the LLM call and writes
a deterministic placeholder — useful in CI and for testing the pipeline without
API access.

The harness exposes all three as endpoints: `report.generate`, `recopilot.analyze`,
`exploitgen.scaffold`. An AI operator working through the harness can trigger report
generation, request pseudocode analysis, and scaffold exploits in the same JSON
conversation it uses for recon and fuzzing.

That's the full loop: discover, probe, document, analyze, report — all from one
engagement directory, all with structured data the AI can read and act on.

---

*This is the final post in the GeistScope series. The full codebase is on GitHub.*
