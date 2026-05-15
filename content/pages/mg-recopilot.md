---
title: "mg-recopilot"
date: 2026-05-15
summary: "Reverse-engineering copilot: reads decompiled pseudocode and writes a structured analysis (Function Purpose, Variable Map, Control Flow Notes, Suspicious Logic, Exploit Primitives, Next Steps)."
tags: [geistscope, cli, reverse-engineering, ai, phase-2]
---

## Purpose

`mg-recopilot` is the first Phase-2 (Cyberpunk wishlist) crate from
`HANDOFF.md` §14 S2 — item #08, the "binary RE copilot". The operator
drops decompiled pseudocode from Ghidra, Binary Ninja, IDA, or radare2
into the engagement workspace and the tool produces a structured Markdown
+ JSON analysis pair.

## Inputs

```text
engagements/<name>/re/<binary>/
|-- manifest.json        # optional: { binary_name, arch, mitigations[], notes }
`-- raw/<func>.c         # operator-supplied decompiled pseudocode
```

The manifest tells the model which mitigations are active (NX, ASLR, CFI,
W^X, ...). The prompt instructs the model to mark any primitive blocked by
those mitigations rather than suggest it.

## Outputs

```text
engagements/<name>/re/<binary>/
|-- <func>.md            # Markdown sections, one per topic
`-- <func>.json          # same fields structured for the harness
```

Sections produced for every function:

- `function_purpose`
- `variable_map`
- `control_flow_notes`
- `suspicious_logic`
- `exploit_primitives`
- `suggested_next_steps`

## CLI

```bash
mg-recopilot analyze acme-bounty libfoo parse_header
mg-recopilot analyze acme-bounty libfoo parse_header --offline --force
```

`--offline` writes a deterministic placeholder document with the manifest
hint embedded — useful for smoke tests and pipelines without an LLM
backend.

## Safety

- `binary` and `function` arguments are rejected if they contain `/`, `\`,
  `..`, or any control character — pseudocode never escapes the
  engagement's `re/` subdirectory.
- Pseudocode and manifest are wrapped as untrusted `<pseudocode>` /
  `<manifest>` evidence in the prompt.
- Bounded reads: pseudocode is capped at 128 KiB, manifest at 16 KiB, model
  response at 256 KiB.

## Harness

[mg-harness](/wiki/mg-harness) exposes `re.analyze` (ReadOnly) and
`re.read` (ReadOnly, bounded UTF-8-safe read of the result pair).
