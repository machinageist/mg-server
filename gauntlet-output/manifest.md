# Gauntlet Manifest — mg-server

**Run:** 1 (first run)
**Criteria version:** 1 (2026-08-07)
**Status:** Phase 0 confirmed (full tree). Batch 1 (foundation) verified — A1, A2
and A3 all PASS. Correction passes queued; Phase 1 batch 2 (B1–B3) next.
**Batch policy:** concurrency 3. Foundation (A) → content surfaces (B) → new capabilities (C).

**Run note (2026-08-07 21:00 PT):** all three batch-1 spec agents reported an API
session-limit termination. The specs were already on disk and complete — the
error landed after the write. Verified by inspection before continuing.

Status flow: `pending` → `spec-in-progress` → `spec-complete` →
`verify-in-progress` → `pass` / `fail` → `remediation-{n}` → `pass` / `escalated`

| Feature ID | Name | Status | Spec | Scorecard | Score | Iterations |
|---|---|---|---|---|---|---|
| A1 | design-system | pass | `specs/A1-design-system.md` | `scorecards/A1-design-system-scorecard.md` | 2.62 | 1 |
| A2 | site-shell | pass, correcting | `specs/A2-site-shell.md` | `scorecards/A2-site-shell-scorecard.md` | 2.53 | 1 |
| A3 | ops-and-observability | pass | `specs/A3-ops-and-observability.md` | `scorecards/A3-ops-and-observability-scorecard.md` | 2.76 | 1 |
| B1 | home | verify-in-progress | `specs/B1-home.md` | — | — | 1 |
| B2 | about | spec-in-progress | — | — | — | 1 |
| B3 | portfolio | pending | — | — | — | 0 |
| B4 | writing | pending | — | — | — | 0 |
| B5 | learn | pending | — | — | — | 0 |
| B6 | releases | pending | — | — | — | 0 |
| C1 | search | pending | — | — | — | 0 |
| C2 | glossary | pending | — | — | — | 0 |
| C3 | study-tools | pending | — | — | — | 0 |
| C4 | progress | pending | — | — | — | 0 |

## Correction passes

All three passed, so none entered a Phase 3 remediation loop. Each scorecard's
Priority 1 list is still applied, because a passing spec with wrong numbers still
misleads whoever implements it.

| Feature | Applied | Outstanding |
|---|---|---|
| A1 | contrast count 19→14, ten `style.css` citations rebased +31, asset table remeasured, `lab.rs` marked tracked, blog slug corrected | P2 coverage gaps (table rules, reviewer paths, 65ch measures) |
| A2 | in progress (first attempt died at the session limit before writing) | 4 blocking feasibility defects + 3 drift guards red on arrival + citation drift |
| A3 | all Priority 1 applied (`f718f26`) | reviewer path for the self-directed learner (4E) |

**Note on the +31 drift:** A1's and A3's `style.css` citations above line 1112
were correct when written. Commit `5e98092` (Markdown table rules) inserted 31
lines and staled them. Verified by A3's agent independently.

## Open decision for Jeff

**B1 §8 Q1 — may the home page name RHCSA?** `criteria.md` auto-fail rule 1
forbids a certification claim without a booked voucher, and `CERT_PLAN.md:86`
confirms none is booked. But `PUBLIC_FACE.md:15-23` was loosened on 2026-08-03 to
permit naming RHCSA *with its status attached*. The spec did not decide silently:
it recorded the conflict and took the conservative path (name no exam on `/`),
reasoning that the status clause is a hedging sentence the page's own tests treat
as defensive meta-copy, that `/about` is the right surface for it, and that a
cert-in-progress is the competitor set's signal rather than the differentiator.

Overrulable. Nothing downstream is blocked on it.
