# Scorecard: About

**Feature ID:** B2
**Spec file:** gauntlet-output/specs/B2-about.md
**Reviewer agent:** Blind verification agent (Claude Opus 4.8)
**Date:** 2026-08-08
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This is a disciplined, source-grounded spec whose primary job — retiring
the stale `CompTIA` copy (criterion 1D) — is handled with exceptional care: it names
both live occurrences (`pages.rs:81`, `:92`), reasons correctly about why the live cert
spine must *not* be published (naming an exam pre-voucher would trip auto-fail rule 1),
and adds a test against the *real* handler output that closes the exact hidden-coupling
blind spot 5C names. The weakest point is that some AA/typography guarantees are
deferred to the A1 design-system feature (CFR-1) rather than owned in B2 — which is the
correct architecture for a content feature, but leaves the small-text (`0.875rem` at
`--text-muted`) contrast guarantee resting on an audit B2 asserts rather than proves.
No auto-fail triggers; all lens averages clear 2.0.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 3 | §4.2 evidence-links table + §6.3 + §6.4(1A): each named capability links to a writeup that can state the evidence-standard fields; capabilities without a published writeup (Automation) are stated as activities, not artifacts, and carry no link implying one. Verified: `hosting` and `security-headers` posts exist and are the modelled artifacts. | — |
| 1B. State honesty | 3 | §7.1 ("implemented, with one live claim defect"), §7.2 (current vs target delta), §6.3: "Further out" content is labelled aspiration, not shipped capability; implemented/absent distinguished throughout. | — |
| 1C. Publication gates | 3 | §6.4(1C) + token lists in T-B2-1/T-B2-3: about names no GeistScope tool and implies no offensive-security identity; the exclusion is machine-tested. Verified: no `geistscope`/`offensive`/`red-team` token in about copy. | — |
| 1D. Copy currency | 3 | **The feature's primary job.** §1.2, §4.2 copy table + "Why not name the live spine", §5.1 T-B2-1, §7.1: stale `CompTIA` removed from both `description()` (`pages.rs:81`) and `bio` (`pages.rs:92`); replacement is capability-led and cert-agnostic; the RHCSA→CCNA→Security+ tension is resolved correctly (describe capability studied, name no exam pre-voucher). Verified: the stale strings are real at the cited lines; governing docs (`about.html:16-18`, README, `public-portfolio-structure.md`) confirm the no-cert-until-voucher rule. | — |
| 1E. Role posture | 3 | §6.3, §5.1 T-B2-3/T-B2-5: copy leads with owned/operated capability (homelab, request path, CLI diagnostics, defensive on owned scope); forbidden identities excluded and tested; "Further out" AI-infra interest pinned as aspiration by T-B2-5 so it cannot drift present-tense. | — |
| 1F. Test-encoded policy | 3 | §5.1 (final para) + §6.4(1F): the existing anti-overclaim test (`pages.rs:204-223`) is **kept and strengthened, not weakened** ("No guard is relaxed"); only the injected-fixture bio is updated to drop `CompTIA`, and T-B2-1 adds a guard on real output. | — |

**Lens average:** 3.0
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation | 2 | §0 scope table + §7.4 CFR-1: correctly defers SOLARCORE_SPEC.md reconciliation to A1 and does **not** silently re-assert the stale spec (which would score 0); uses only shipped role tokens. This is correct handling by non-interference, but 2A is fundamentally an A1 criterion — B2 has little to actively demonstrate here. | Nothing blocking; 2A is graded primarily against the A1 design-system spec. |
| 2B. Typographic craft | 3 | §3.3 (prose capped at `--measure`; `.bio` 65ch), §3.7 (heading hierarchy legible by weight/case, not colour; code keeps column via inline `<code>`), §7.4 CFR-1 (size literals handed to A1 to tokenise). Verified: `.bio` 65ch at `style.css:963`, `.about-list li` 0.875rem at `:973`. | — |
| 2C. Pedagogical depth | N/A | §6.4(4C) explicitly: about is an identity page, not an education surface (that is B5/`/learn`); about *points* to learning material. Criterion 2C applies to `/learn` pages, glossaries, study tools. Correctly scoped out — **excluded from lens average** (scoring it 0 would be a category error). | — |
| 2D. Scannability & structure | 3 | §1.2 (thesis: every capability named points at operated evidence), §4.2 evidence links, §3.2 branch-to-evidence flow: about adds cross-links from each proven capability to its writeup and to `/portfolio` — the differentiator vs. an unlinked skills list. About is an existing page, so SIDEBAR/WIKI_SLUGS registration is N/A. | — |
| 2E. Restraint | 3 | §3.3 (no cards, no fake metrics), §3.5 (zero animation), §6.4(2E) + T-B2-3: quiet prose, show-don't-tell voice pinned by test. Exemplary restraint. | — |
| 2F. Theme integrity | 3 | §6.4(2F), §5.4 (Tier-1 six-theme visual pass), §3.7: about uses only role tokens (`--text`, `--text-muted`, `--border-subtle`); size/spacing literals correctly kept out of themes and flagged to A1. Works across all 23 themes with no per-theme edit. | — |

**Lens average:** 2.8 (2A=2, 2B=3, 2D=3, 2E=3, 2F=3; 2C excluded as N/A)
**Lens pass:** Yes — avg ≥ 2.0, one criterion at 2, zero 1s, no 0s (among applicable criteria)

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JavaScript | 3 | §4.6 (about ships zero JS), §5.2 I-B2-2 (strip all `<script>`, assert bio + four capability labels + every evidence-link href remain). Machine-checked no-JS floor. | — |
| 3B. Contrast & colour independence | 3 | §3.7 (pairs `--text`/`--text-muted` on `--bg` in A1's audited matrix; state via weight + underline, never hue; in-body links stay underlined as an invariant). The `0.875rem` small-text 4.5:1 requirement is flagged and the guarantee deferred to A1 (CFR-1). Correct architecture for a content feature; the AA-at-usage-size guarantee is asserted via A1's audit rather than proven in B2 (see Feasibility caveat). | — (guarantee lands with A1 CFR-1) |
| 3C. Keyboard & focus | 3 | §3.4 + §3.7: only natively-focusable links, global `:focus-visible` ring (`style.css:685`), DOM order, no `tabindex`/trap/custom widget. | — |
| 3D. Semantics & assistive tech | 3 | §3.7: one `<h1>`, two `<h2>` (no skipped level), real `<ul>` (ul-vs-dl decision reasoned + logged Q1), self-describing evidence-link text as an invariant, `.bio-loc` meaningful and not `aria-hidden`. | — |
| 3E. Motion & sensory safety | 3 | §3.5: about contributes zero motion; inherited chrome gated by `prefers-reduced-motion: no-preference` in A1/A2 territory. | — |
| 3F. Responsive & resilient | 3 | §3.3 (empty state marked intentionally-N/A: no dynamic collection), §3.4 (single reflowing column, two shell breakpoints, no about-specific `@media` — verified `style.css:957-982` has none), §5.4 (320/800/1280px, 200%/400% zoom), B2-E3 (readable unstyled). | — |

**Lens average:** 3.0
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s
**Auto-fail triggered:** No — rule 2 (accessibility floor: contrast deferred correctly to A1 audit, focus preserved, no hue-only state, reduced-motion respected) and rule 3 (no-JS floor: zero JS, machine-checked) both pass.

*Note: criteria.md defines Lens 3 as 3A–3F (6 criteria); the universal template's phantom 3G row is not part of this criteria set and is omitted.*

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 3 | §1.2 + §6.4(4A): the stated impression is a real operated system (Proxmox lab) and a real request path (Tunnel→Caddy→mg-server), explicitly contrasted with competitor junior portfolios' unlinked skills word cloud. | — |
| 4B. Evidence over enthusiasm | 3 | §4.2 + §6.4(4B): capabilities link to writeups including the migration-outage post (the in-repo 4B model). Verified: `management-layer-first-network-migration.md` exists — an outage worked end to end (incident/recovery/revised-plan). | — |
| 4C. Original explanation | N/A | §6.4(4C) explicitly: N/A for an identity page; educational material is B5, which about points to. **Excluded from lens average.** | — |
| 4D. Depth of a real system | 3 | §6.4(4D) + §3.3 + template: copy references the Proxmox lab, DNS/VLANs, the Tunnel→Caddy→mg-server path, and this server itself — a genuinely operated system, not a followed tutorial. | — |
| 4E. Reviewer paths | 3 | §2 user stories (hiring manager, engineer peer, self-directed learner, plus operator and a11y personas) + §6.4(4E): each reader's arrival and need is accounted for, with evidence links / concrete tools / `/learn` pointer respectively. | — |

**Lens average:** 3.0 (4A=3, 4B=3, 4D=3, 4E=3; 4C excluded as N/A)
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 3 | §4.4 (observation: `bio` and `description()` are two copies of the same "what Jeff studies" claim and must be edited/guarded together) + §6.4(5A) + T-B2-1 (co-guards both). Links reference canonical routes. | — |
| 5B. Drift guards | 3 | §5.1: T-B2-1 (asserts real handler output, not a fixture → catches reintroduction of any retired claim), T-B2-4 (every evidence-link href must resolve `200` → catches slug typo / link-before-post), T-B2-2 (meta length window). Loud-failing guards for each drift vector. | — |
| 5C. No hidden coupling | 3 | §5.1 (latent-defect paragraph) + §6.4(5C) + §7.4 CFR-3: directly fixes the criterion's named failure — the existing test's injected-fixture blind spot — by asserting against real output; also documents the home page's `assert!(html.contains("CompTIA"))`-via-meta coupling (`pages.rs:158`) as a cross-feature request. Verified: the existing test does inject a bio at `pages.rs:207`. | — |
| 5D. Verification is stated | 3 | §5 intro (the four CI commands: fmt / clippy -D warnings / test --all-targets / build --release) + exact new test names T-B2-1..5, I-B2-1..2. | — |
| 5E. Documentation follows behavior | 3 | §7.4 + §6.4(5E): names `docs/public-portfolio-structure.md` (stale) and `mg-coreforge/PUBLIC_FACE.md` (wording authority) as docs to update in the same change; notes README is already correct. Verified: `public-portfolio-structure.md` still says "Network+ then RHCSA," contradicting the 2026-08-02 re-lock — the spec's catch is accurate. | — |

**Lens average:** 3.0
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Feasibility Check

Read the actual source files referenced in the spec before filling this table.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | `AboutTemplate` struct + `title`/`description`/`section` + `about()` handler at `pages.rs:69-95`, exactly as cited. |
| API/interface changes are feasible with current architecture | ✓ | Two string edits + inline template links + tests. `/about` route at `router.rs:38`; signature unchanged. |
| Views/screens fit current navigation pattern | ✓ | `/about` is an existing single-column page in the A2 shell; nav highlight via `section()=="about"` matches `base.html:24`. |
| Dependencies are available and version-compatible | ✓ | No new packages; askama / axum / askama_axum already in use (code confirms). Stated versions (askama 0.12 / axum 0.7) not independently checked but non-load-bearing. |
| Platform/renderer requirements are realistic | ✓ | Plain HTML + inherited shell CSS; zero JS; degrades to readable unstyled HTML. |
| Test strategy is executable with current infrastructure | ✓ | `oneshot` router-drive pattern already used in `status.rs:71`, `errors.rs:175`, `vitals.rs`; T-B2-4's per-href 200 check is feasible against `router::build`. |
| Performance budget is realistic for target hardware | ✓ | One struct construction + one Askama render, no I/O — trivially within budget. |
| No undeclared dependency on unbuilt features | ✓ | The `section() → Section::About` change depends on A2's not-yet-built `Section` enum (no `src/shell.rs` today) — correctly declared as cross-feature CFR-2 with the `&str "about"` path working until then. Evidence links only ship if target exists (gated by T-B2-4). |

**Feasibility verdict:** Feasible
**Caveats:**
- The AA-contrast guarantee for `.about-list li` small text (`0.875rem` at `--text-muted`) across all 23 themes rests on A1's audited matrix (CFR-1), which B2 asserts but does not itself prove. Correct architecture, but the guarantee is not owned within B2.
- Minor internal inconsistency: §4.2 leaves the network-migration evidence link as "…network-migration…" with a "verify slug (Q3)" note, while §3.2 already names the correct slug `/blog/management-layer-first-network-migration` (verified — the post exists). T-B2-4 would catch any mismatch, so this is polish, not a defect.

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 3.0 | 25% | 0.75 |
| 2. Design & Craft Excellence | 2.8 | 25% | 0.70 |
| 3. Accessibility & Progressive Enhancement | 3.0 | 20% | 0.60 |
| 4. Competitive Depth & Differentiation | 3.0 | 20% | 0.60 |
| 5. Accuracy & Maintainability | 3.0 | 10% | 0.30 |
| **Composite** | | | **2.95** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 (2.95)
- [x] All lens averages ≥ 2.0 (min 2.8)
- [x] No criterion scores 0 (N/A criteria 2C, 4C correctly scoped out, excluded from averages)
- [x] No more than two criteria at 1 per lens (zero 1s in any lens)
- [x] All auto-fail rules pass (rule 1 unearned claims: the spec *removes* a stale claim and forbids naming any exam pre-voucher; rule 2 accessibility floor: pass; rule 3 no-JS floor: pass)
- [x] Feasibility ≠ Infeasible (Feasible)

**All conditions met:** Yes → PASS

---

## Remediation Brief (advisory — spec PASSES)

### Priority 1 — Must fix to pass
None. No blocking gaps.

### Priority 2 — Should fix for quality
1. Resolve the §4.2 vs §3.2 slug inconsistency: replace the "…network-migration…" placeholder + Q3 verify-note in the §4.2 evidence table with the confirmed slug `/blog/management-layer-first-network-migration` (verified present at `content/posts/management-layer-first-network-migration.md`), so the implementing agent does not have to re-derive it.
2. Because 3B's AA guarantee for the `0.875rem` / `--text-muted` small text is deferred to A1 (CFR-1), state a fallback if A1 has not audited that pairing by ship time — e.g. ship the copy fix (which carries no contrast risk) independently and hold the visual sign-off until CFR-1 lands. The spec already says the copy fix can ship first; make the contrast dependency explicit as a ship-gate note.

### Priority 3 — Consider for excellence
1. Sequencing (Q5 / CFR-3): confirm B2's `CompTIA` removal and B1's (`pages.rs:44` description + the `pages.rs:158` `assert!(html.contains("CompTIA"))` test) land as one change or adjacent changes, so a site-wide `grep CompTIA` returns zero only after both — avoiding a half-corrected site.
2. Q1 (`<ul>` vs `<dl>`): the spec's default (keep `<ul>`, matching A1's site-wide "definition-style list" pattern) is well-reasoned; no action needed unless Jeff prefers term-by-term AT navigation.

---

**End of scorecard.**
