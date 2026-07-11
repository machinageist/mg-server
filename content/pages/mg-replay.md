---
title: "mg-replay"
date: 2026-05-15
summary: "Replay curl evidence from findings to produce current exploitability verdicts."
tags: [geistscope, cli, replay, verification]
---

## Purpose

Re-run the `## Evidence` curl commands embedded in a finding and compare
the current response against the original. Useful for confirming a finding
just before submission and for keeping a longitudinal record of when a bug
was patched.

## Output

- `findings/<id>-<slug>-replay-<date>.json` — captured request, current
  response, optional baseline diff, and a verdict.

## Verdicts

- `still_vulnerable` — current response matches the original signal.
- `appears_fixed` — signal is gone.
- `indeterminate` — heuristics could not decide; manual review needed.

## CLI

```bash
mg-replay acme-bounty 20260514-probe-001
mg-replay acme-bounty 20260514-probe-001 --baseline previous-replay.json
```

## Notes

- Session-aware: env-var-backed headers from `session.json` are added unless
  the curl command already sets `Authorization`.
- Replay reports never overwrite the original evidence. They accumulate
  alongside the finding markdown.
