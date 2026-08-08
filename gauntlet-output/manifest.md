# Gauntlet Manifest — mg-server

**Run:** 1 (first run)
**Criteria version:** 1 (2026-08-07)
**Status:** Phase 0 confirmed (full tree). Batch 1 (foundation) specs written.
Phase 2 verification of A1–A3 running. Batch 2 dispatches when it returns.
**Batch policy:** concurrency 3. Foundation (A) → content surfaces (B) → new capabilities (C).

**Run note (2026-08-07 21:00 PT):** all three batch-1 spec agents reported an API
session-limit termination. The specs were already on disk and complete — the
error landed after the write. Verified by inspection before continuing.

Status flow: `pending` → `spec-in-progress` → `spec-complete` →
`verify-in-progress` → `pass` / `fail` → `remediation-{n}` → `pass` / `escalated`

| Feature ID | Name | Status | Spec | Scorecard | Score | Iterations |
|---|---|---|---|---|---|---|
| A1 | design-system | verify-in-progress | `specs/A1-design-system.md` | — | — | 1 |
| A2 | site-shell | verify-in-progress | `specs/A2-site-shell.md` | — | — | 1 |
| A3 | ops-and-observability | verify-in-progress | `specs/A3-ops-and-observability.md` | — | — | 1 |
| B1 | home | pending | — | — | — | 0 |
| B2 | about | pending | — | — | — | 0 |
| B3 | portfolio | pending | — | — | — | 0 |
| B4 | writing | pending | — | — | — | 0 |
| B5 | learn | pending | — | — | — | 0 |
| B6 | releases | pending | — | — | — | 0 |
| C1 | search | pending | — | — | — | 0 |
| C2 | glossary | pending | — | — | — | 0 |
| C3 | study-tools | pending | — | — | — | 0 |
| C4 | progress | pending | — | — | — | 0 |
