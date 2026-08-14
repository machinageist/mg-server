# Gauntlet Manifest — mg-server

**Run:** 1 (first run)
**Criteria version:** 1 (2026-08-07)
**Status:** Phase 0 confirmed (full tree). Foundation (A1–A3) and content surfaces
(B1–B6) all pass. C1 and C2 have specs awaiting blind verification; C3 and C4 are
not yet spec'd.
**Batch policy:** concurrency 3. Foundation (A) → content surfaces (B) → new
capabilities (C).

Status flow: `pending` → `spec-in-progress` → `spec-complete` →
`verify-in-progress` → `pass` / `fail` → `remediation-{n}` → `pass` / `escalated`

| Feature ID | Name | Status | Spec | Scorecard | Score | Iterations |
|---|---|---|---|---|---|---|
| A1 | design-system | pass | `specs/A1-design-system.md` | `scorecards/A1-design-system-scorecard.md` | 2.62 | 1 |
| A2 | site-shell | pass | `specs/A2-site-shell.md` | `scorecards/A2-site-shell-scorecard.md` | 2.53 | 1 |
| A3 | ops-and-observability | pass | `specs/A3-ops-and-observability.md` | `scorecards/A3-ops-and-observability-scorecard.md` | 2.76 | 1 |
| B1 | home | spec-complete — **re-verify** | `specs/B1-home.md` | `scorecards/B1-home-scorecard.md` | 2.62 *(graded the superseded text)* | 2 |
| B2 | about | spec-complete — **re-verify** | `specs/B2-about.md` | `scorecards/B2-about-scorecard.md` | 2.95 *(graded the superseded text)* | 2 |
| B3 | portfolio | pass | `specs/B3-portfolio.md` | `scorecards/B3-portfolio-scorecard.md` | 2.79 | 1 |
| B4 | writing | pass | `specs/B4-writing.md` | `scorecards/B4-writing-scorecard.md` | 2.76 | 1 |
| B5 | learn | pass | `specs/B5-learn.md` | `scorecards/B5-learn-scorecard.md` | 2.83 | 1 |
| B6 | releases | pass | `specs/B6-releases.md` | `scorecards/B6-releases-scorecard.md` | 2.88 | 1 |
| C1 | search | spec-complete | `specs/C1-search.md` | — | — | 1 |
| C2 | glossary | spec-complete | `specs/C2-glossary.md` | — | — | 1 |
| C3 | study-tools | pending | — | — | — | 0 |
| C4 | progress | pending | — | — | — | 0 |

## Run history

**2026-08-07 21:00 PT.** All three batch-1 spec agents reported an API
session-limit termination. The specs were already on disk and complete — the
error landed after the write. Verified by inspection before continuing.

**2026-08-08 14:0x PT.** Batch 2 ran as a workflow at concurrency 3. Chunk 1
(B1–B3) completed and passed. Chunk 2 (B4–B6) spec agents all failed with
"session limit · resets 6:40pm PT" — a hard usage cap, no specs written. They
were re-dispatched after the reset and B4–B6 landed.

**2026-08-11.** Commit `c176cab` ("new features and merge") merged two branches
that had each run part of the gauntlet, and **committed the conflict unresolved**.
`manifest.md`, `specs/B1-home.md`, and `specs/B2-about.md` carried `<<<<<<<`
markers for three days. See the resolution note below.

**2026-08-14.** Conflicts resolved by hand. Several outstanding correction items
applied. C1 and C2 specs committed (they had been sitting untracked on disk).

## The B1 / B2 merge resolution

Two different spec agents wrote a complete spec for each of these features: one
on `main` (`43ef1ea`, Opus 5) and one on the branch (`a0bff14`, Opus 4.8). The
botched merge interleaved both across ~11 hunks per file.

Resolved in favour of the **Opus 5 text** after comparing section by section and
diffing the two sides' citation sets. The Opus 5 side is a superset in substance —
`path:line` scope boundaries, severity-ranked findings tables, ordered commit
sequences, cross-feature requests. The 4.8 side cited exactly one source each
that the Opus 5 side missed; both were pulled forward and are marked
*(merged from the 4.8 spec)* in place.

**Consequence:** the B1 and B2 scorecards on file graded the *branch* text, not
what is now on disk. Their scores record that the feature was analysed; they are
not a verification of the current document. Re-run Phase 2 on both before
treating them as passing.

The merge also surfaced a live defect that had been sitting unread behind the
conflict markers: B2's audit found `/about` claiming five capabilities the site's
own blog posts record as absent or planned. Fixed 2026-08-14 (`b3afbc9`).

## Correction passes

No feature entered a Phase 3 remediation loop — all graded features passed on the
first attempt. Priority 1 items are still worked, because a passing spec with
wrong numbers still misleads whoever implements it.

| Feature | Applied | Outstanding |
|---|---|---|
| A1 | contrast count 19→14, ten `style.css` citations rebased +31, asset table remeasured, `lab.rs` marked tracked, blog slug corrected | P2 coverage gaps (table rules, reviewer paths, 65ch measures) |
| A2 | — | 4 blocking feasibility defects, 3 drift guards red on arrival, citation drift. See `REMEDIATION-BRIEF.md` |
| A3 | all Priority 1 applied (`f718f26`) except the verbatim CI commands in §5 | reviewer path for the self-directed learner (4E); §3.7/F6/T17 now describe a contrast problem `0cdbbea` eliminated |
| B1 | — | scorecard graded superseded text — re-verify first |
| B2 | the `/about` capability overclaims it identified (`b3afbc9`) | scorecard graded superseded text — re-verify first |
| B3 | — | designed empty state, external-link cue, anti-overclaim test. **Two of its "corrections" are wrong** — the nav link really is `base.html:25` and the card transition really is `style.css:745`. See `REMEDIATION-BRIEF.md` |
| B4 | heading-id pass for deep-linkable anchors, its P3 item 2 (`cca5910`) | P2: summary-guard mismatch, designed empty state, pillar landmark over-labeling |
| B5 | stale `network-plus` tags dropped from all learn pages (`9664566`) — this was its only P1 | G1–G12 refinements remain open |
| B6 | — | none recorded |

**On the "+31" drift note.** Earlier revisions of this manifest recorded that
commit `5e98092` inserted 31 lines into `style.css` above line 1112, staling A1's
and A3's citations. **That rule is now obsolete** — `style.css` has grown across
four further commits and every affected citation has been re-verified against
source in `gauntlet-output/REMEDIATION-BRIEF.md`. Use the brief, not the
scorecards' own line numbers.

## Open decision for Jeff

**B1 §8 Q1 — may the home page name RHCSA?** `criteria.md` auto-fail rule 1
forbids a certification claim without a booked voucher, and `CERT_PLAN.md`
confirms none is booked. But `PUBLIC_FACE.md:15-23` was loosened on 2026-08-03 to
permit naming RHCSA *with its status attached*. The spec did not decide silently:
it recorded the conflict and took the conservative path (name no exam on `/`),
reasoning that the status clause is a hedging sentence the page's own tests treat
as defensive meta-copy, that `/about` is the right surface for it, and that a
cert-in-progress is the competitor set's signal rather than the differentiator.

Still open, still overrulable, nothing downstream blocked on it. `/about` now
carries an inline comment recording the same trade-off at the point of decision.
