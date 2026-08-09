# Gauntlet Manifest — mg-server

**Run:** 1 (first run)
**Criteria version:** 1 (2026-08-07)
**Status:** Phase 0 confirmed (full tree). Batch 1 (foundation A1–A3) PASS. Batch 2
first chunk (B1–B3) PASS. B4–B6 spec agents died on the account session limit
mid-run (not a quality failure) and remain pending. C1–C4 pending.
**Batch policy:** concurrency 3. Foundation (A) → content surfaces (B) → new capabilities (C).

**Run note (2026-08-07 21:00 PT):** all three batch-1 spec agents reported an API
session-limit termination. The specs were already on disk and complete — the
error landed after the write. Verified by inspection before continuing.

**Run note (2026-08-08 14:0x PT):** batch-2 ran as a workflow at concurrency 3.
Chunk 1 (B1–B3) completed and all passed. Chunk 2 (B4–B6) spec agents all failed
with "session limit · resets 6:40pm PT" — a hard usage cap, no specs written, no
partial files left. Re-dispatch B4–B6 after the reset; B1–B3 output committed.

Status flow: `pending` → `spec-in-progress` → `spec-complete` →
`verify-in-progress` → `pass` / `fail` → `remediation-{n}` → `pass` / `escalated`

| Feature ID | Name | Status | Spec | Scorecard | Score | Iterations |
|---|---|---|---|---|---|---|
| A1 | design-system | pass | `specs/A1-design-system.md` | `scorecards/A1-design-system-scorecard.md` | 2.62 | 1 |
| A2 | site-shell | pass | `specs/A2-site-shell.md` | `scorecards/A2-site-shell-scorecard.md` | 2.53 | 1 |
| A3 | ops-and-observability | pass | `specs/A3-ops-and-observability.md` | `scorecards/A3-ops-and-observability-scorecard.md` | 2.76 | 1 |
| B1 | home | pass | `specs/B1-home.md` | `scorecards/B1-home-scorecard.md` | 2.62 | 1 |
| B2 | about | pass | `specs/B2-about.md` | `scorecards/B2-about-scorecard.md` | 2.95 | 1 |
| B3 | portfolio | pass | `specs/B3-portfolio.md` | `scorecards/B3-portfolio-scorecard.md` | 2.79 | 1 |
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
| A2 | — | 4 blocking feasibility defects + 3 drift guards red on arrival + citation drift |
| A3 | — | `set-header` feature gate, `BindMode::description` test collision, contrast count 6→7 |
| B1 | — | none (scorecard reported no P1 items) |
| B2 | — | none (scorecard reported no P1 items) |
| B3 | — | `--text-2xs` token cited but absent from `style.css` (real floor is `--text-xs` 0.75rem) — reconcile via A1 sweep or map 0.72rem literals to `--text-xs`; add designed empty-state to `portfolio.html`; add external-link cue to entry link; add rendered-HTML anti-overclaim test in `pages.rs`; fix line citations (nav link base.html:26, card transition style.css:744) |

**Note on the +31 drift:** A1's and A3's `style.css` citations above line 1112
were correct when written. Commit `5e98092` (Markdown table rules) inserted 31
lines and staled them. Verified by A3's agent independently.
