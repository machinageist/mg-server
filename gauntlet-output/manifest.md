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
| A2 | site-shell | pass, correcting | `specs/A2-site-shell.md` | `scorecards/A2-site-shell-scorecard.md` | 2.53 | 1 |
| A3 | ops-and-observability | pass | `specs/A3-ops-and-observability.md` | `scorecards/A3-ops-and-observability-scorecard.md` | 2.76 | 1 |
<<<<<<< HEAD
| B1 | home | verify-in-progress | `specs/B1-home.md` | — | — | 1 |
| B2 | about | spec-in-progress | — | — | — | 1 |
| B3 | portfolio | pending | — | — | — | 0 |
| B4 | writing | pending | — | — | — | 0 |
| B5 | learn | pending | — | — | — | 0 |
| B6 | releases | pending | — | — | — | 0 |
=======
| B1 | home | pass | `specs/B1-home.md` | `scorecards/B1-home-scorecard.md` | 2.62 | 1 |
| B2 | about | pass | `specs/B2-about.md` | `scorecards/B2-about-scorecard.md` | 2.95 | 1 |
| B3 | portfolio | pass | `specs/B3-portfolio.md` | `scorecards/B3-portfolio-scorecard.md` | 2.79 | 1 |
| B4 | writing | spec-complete | `specs/B4-writing.md` | — | — | 0 |
| B5 | learn | pass | `specs/B5-learn.md` | `scorecards/B5-learn-scorecard.md` | 2.83 | 1 |
| B6 | releases | pass | `specs/B6-releases.md` | `scorecards/B6-releases-scorecard.md` | 2.88 | 1 |
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d
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
<<<<<<< HEAD
| A2 | in progress (first attempt died at the session limit before writing) | 4 blocking feasibility defects + 3 drift guards red on arrival + citation drift |
| A3 | all Priority 1 applied (`f718f26`) | reviewer path for the self-directed learner (4E) |
=======
| A2 | — | 4 blocking feasibility defects + 3 drift guards red on arrival + citation drift |
| A3 | — | `set-header` feature gate, `BindMode::description` test collision, contrast count 6→7 |
| B1 | — | none (scorecard reported no P1 items) |
| B2 | — | none (scorecard reported no P1 items) |
| B3 | — | `--text-2xs` token cited but absent from `style.css` (real floor is `--text-xs` 0.75rem) — reconcile via A1 sweep or map 0.72rem literals to `--text-xs`; add designed empty-state to `portfolio.html`; add external-link cue to entry link; add rendered-HTML anti-overclaim test in `pages.rs`; fix line citations (nav link base.html:26, card transition style.css:744) |
| B5 | — | stale `network-plus` tag on 11 of 12 learn pages (criterion 1D, borderline 0): retag networking pages to the live spine (drop `network-plus`, use `ccna` where apt), correct spec §6.3's false "aligned with the live cert spine" claim, extend the A2 retired-claims guard to cover the tags field |
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

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
