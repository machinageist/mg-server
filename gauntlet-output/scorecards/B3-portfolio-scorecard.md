# Scorecard: Portfolio

**Feature ID:** B3-portfolio
**Spec file:** gauntlet-output/specs/B3-portfolio.md
**Reviewer agent:** blind-verifier (B3)
**Date:** 2026-08-08
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This is a claim-integrity exemplar: the spec makes the one-entry
minimalism its differentiator, satisfies (never weakens) the shipped
anti-overclaim test, and maps the single live entry to a real evidence path whose
two target posts actually exist. Its most critical gap is a factual slip in the
typographic reasoning — §3.7 cites a non-existent `--text-2xs: 0.70rem` token as
the type-scale floor when the real floor is `--text-xs: 0.75rem` — which, together
with two off-by-one line citations, is a quality/accuracy issue, not a blocker.
No auto-fail triggers; all lens averages clear 2.0.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 3 | §6.4 maps the live `mg-server` entry to every field of `docs/public-portfolio-structure.md`'s evidence standard, and both cited evidence posts exist in `content/posts/` (`hosting-machinageist-dev.md`, `security-headers-on-machinageist-dev.md`). States "a card without a path to these fields must not ship (criterion 1A ≤1 otherwise)." Proposes the `evidence` field to make the mapping first-class. | — |
| 1B. State honesty | 3 | §7.1 table and §3.3 keep implemented / prototyped (unused status styling) / archived-absent (drafts) explicitly distinct; §1.2 and §6.3 never let a planned entry read as shipped. | — |
| 1C. Publication gates | 3 | §6.4 upholds the GeistScope gate — barred by name in the anti-overclaim test (verified `src/models/project.rs:113`), enters portfolio only after full pipeline + human/AI operation + sanitized authorized-engagement evidence. | — |
| 1D. Copy currency | 3 | §6.3 copy-currency note: portfolio carries no cert copy so it is unaffected by the stale "working through the CompTIA stack" line (verified `src/handlers/pages.rs:91-92`); B3 must not introduce cert copy; live spine RHCSA→CCNA→Security+ noted as internal/unpublished. Live copy ("verified, evidenced work only") matches reality. | — |
| 1E. Role posture | 3 | §6.3: no senior/SRE/production-grade framing; tags describe capabilities not credentials (verified `src/models/project.rs:82` — `rust`, `axum`, `linux-service`, `self-hosting`, `headers`). | — |
| 1F. Test-encoded policy | 3 | §5.1 reproduces the shipped test's assertions accurately (verified `src/models/project.rs:92-116`) and commits to satisfying it; §7.4 makes relaxing `len()==1` a recorded-decision gate, "a criterion 1F zero" otherwise. | — |

**Lens average:** 3.00
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation | 2 | Not the design-system feature; §0 treats A1 as a fixed input and does not re-assert the stale SOLARCORE spec (would score 0 if it did). Describes shipped reality (23 themes, tokens) and defers reconciliation to A1 correctly — correct handling but no deep demonstration on this surface. | — |
| 2B. Typographic craft | 2 | §3.3 documents the hierarchy (`--text-2xl` h1, `--measure` cap — both verified); §3.7/§7.2 identify off-scale literals (0.72rem, 0.875rem, 0.95rem, 0.85rem — all verified in CSS) and prescribe Layer-2 tokenization. **Undercut by §3.7 fabricating `--text-2xs: 0.70rem` as an existing floor** (verified absent; real floor is `--text-xs: 0.75rem`), and the cleanup is deferred to the A1 sweep rather than owned here. | Fix §3.7: the type-scale floor is `--text-xs: 0.75rem`; there is no `--text-2xs`. Map 0.72rem to an existing/created token explicitly. |
| 2C. Pedagogical depth | 3 | N/A — portfolio is not a `/learn` page, glossary, or study tool. Spec correctly keeps body copy quiet and does not fabricate an education surface (§2E). No gap. | — (N/A) |
| 2D. Scannability and structure | 3 | Single-column scannable list; nav registration and active-state verified (`base.html` Portfolio link + `section()=="portfolio"`, `pages.rs:116-118`). Existing page, no new SIDEBAR/WIKI_SLUGS registration needed; §3.1/§4.1 confirm. | — |
| 2E. Restraint | 3 | §1.2 "subtraction, not addition"; §3.5 hover is "chrome-level, not body motion." Despite the `.project-card` name, the shipped CSS (verified `style.css:1215-1218`) is a padded list row with a bottom border, not a boxed card — no dashboard cosplay, no fake metrics. | — |
| 2F. Theme integrity | 3 | §5.4 mandates all-23-theme render + contrast check; §4.5/§4.7 require every visual rule to resolve to A1 tokens (colour/font own themes, size/spacing do not). §3.7 correctly notes `--accent-border` is decorative tint per A1. | — |

**Lens average:** 2.67
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

*(criteria.md defines 3A–3F for this lens; the template's 3G row has no
corresponding criterion and is omitted.)*

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JavaScript | 3 | §3.2/§4.6/§6.4: zero JS, fully server-rendered (verified `templates/portfolio.html` has no script); §5.4 mandates a JS-disabled manual check. Auto-fail rule 3 not triggered. | — |
| 3B. Contrast and color independence | 3 | §3.7: status is a text word (`active`/`in progress`/`complete`), not hue (verified template renders `{{ project.status }}`). §5.4 requires 4.5:1 at the 0.72rem usage size across all 23 themes, spot-checking tight light themes. | — |
| 3C. Keyboard and focus | 3 | §3.7: entry links receive the shell's global focus ring; focus order is DOM order; no focus removed anywhere (respects auto-fail rule 2). Status pill non-interactive. | — |
| 3D. Semantics and assistive technology | 2 | §3.7: one `<h1>`, real `<ul>`/`<li>`, status as a `<span>` whose text is the state. Correct — but the spec itself flags an unremediated gap: the `target="_blank"` link (verified `portfolio.html:17`) gives no new-tab cue to screen-reader/sighted users; deferred to §7.2 target work. | Add the external-link cue (visible ↗ and/or `aria-label` suffix) to the entry-name link, per §7.2. |
| 3E. Motion and sensory safety | 3 | §3.5: hover state (bg + inset accent bar) sits at `style.css:730-733` **outside** the media query; the 0.2s transition sits **inside** `@media (prefers-reduced-motion: no-preference)` — verified (transition at `style.css:744`). Under `reduce` the hover applies instantly. No autoplay/flashing. | — |
| 3F. Responsive and resilient | 2 | §3.4 header stacks ≤640px (verified `style.css:1555-1559`); tags wrap; §5.4 reflow at 320px/400%. But the designed empty state is an acknowledged GAP — the template has no `{% else %}` (verified); today a bare heading renders over an empty `<ul>`. Spec diagnoses it and supplies honest copy but defers the build to §7.2. | Add the `{% else %}` designed empty state to `templates/portfolio.html` with the §3.3 copy ("No entries meet the evidence bar yet."). |

**Lens average:** 2.67
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s
**Auto-fail triggered:** No — 3A (no-JS) satisfied, contrast specced at usage size, focus never removed, motion behind `prefers-reduced-motion`

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 3 | §1.2/§1.3/§2 name the impression explicitly: within 30s a reviewer can name "a self-hosted Rust service on owned infrastructure," and §1.2 contrasts it with competitors who list fifteen tutorials and lose credibility on the first probe. | — |
| 4B. Evidence over enthusiasm | 3 | §6.4 maps the entry to verification/failure/recovery via the hosting and security-headers posts (README confirms real `dig`/`curl -I` output); §1.2 frames the discipline as surviving a probe. | — |
| 4C. Original explanation | 2 | The portfolio index surfaces original explanation by reference (the evidence posts) rather than carrying it — appropriate for an index, so not a gap, but not a strong demonstration of explanation quality on this surface. | — |
| 4D. Depth of a real system | 3 | §4.2/§6.4: the sole entry is `mg-server` itself — the running service on a Proxmox Debian VM behind Caddy + Cloudflare Tunnel — maximally "a system genuinely operated," matching the criterion's exemplar. | — |
| 4E. Reviewer paths | 3 | §2 enumerates seven reader stories (hiring manager 30s skim, engineer peer wanting evidence links, site owner publishing-gate, screen-reader user, no-JS visitor, narrow-phone visitor, nav arriver), covering the criterion's hiring-manager and engineer-peer paths richly; the learner is served via the evidence-link path. | — |

**Lens average:** 2.80
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 2 | §4.2 "the compiler is the schema," entries edited only in `all()`; §3.3 each field maps 1:1 to `Project`; the `evidence` proposal mirrors the verified `Lab.writeup_url` pattern (`src/models/lab.rs:34`). Excellent SSOT design — but §3.7's fabricated `--text-2xs` and §7.2's `--measure-narrow` reference tokens that do not exist, so a maintainer following that CSS guidance would hit a dead source. | Correct the token references in §3.7/§7.2: either create `--text-2xs`/`--measure-narrow` in the A1 sweep before citing them, or point 0.72rem at the existing `--text-xs`. |
| 5B. Drift guards | 3 | §4.2/§5.1: adding a `ProjectStatus` variant without updating both `Display` and `class_name()` is a compile error (verified exhaustive matches, `project.rs:48-54`, `60-65`); target test `status_display_and_class_agree` guards the pair; §7.4 recorded-decision gate on `len()`. Real, loud guards. | — |
| 5C. No hidden coupling | 3 | §5.2 identifies the exact coupling risk — the shipped test asserts on the *model* (`name + description`, verified `project.rs:103-107`), so a template-only overclaim slips past — and proposes a rendered-HTML integration test to close it, keeping the test's name honest. Mirrors the criterion's `CompTIA`-meta-tag reference precisely. | — |
| 5D. Verification is stated | 3 | §5.4 names the exact CI commands: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo build --release` — matching criteria.md 5D. | — |
| 5E. Documentation follows behavior | 3 | §7.2 names the long-lived docs to update on behaviour change: `docs/public-portfolio-structure.md` "Still open" section on archived→published flips, and `content/drafts/portfolio-entries.md` if the growth workflow changes. | — |

**Lens average:** 2.80
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Feasibility Check

Read the actual source files referenced in the spec before filling this table.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | `Project`/`ProjectStatus`/`all()` verified `src/models/project.rs:21-86`; struct/enum/line refs accurate. Proposed `evidence: Option<&'static str>` mirrors real `Lab.writeup_url` (`lab.rs:34`). |
| API/interface changes are feasible with current architecture | ✓ | Handler + template struct verified `pages.rs:98-126`; route verified `router.rs:39`; infallible owned-Vec contract accurate. |
| Views/screens fit current navigation pattern | ✓ | Existing `/portfolio` in nav + router; `section()=="portfolio"` drives active-state. Minor: Portfolio nav link is `base.html:26` (spec cites :25 — line 25 is the About link). |
| Dependencies are available and version-compatible | ✓ | No new packages; askama/askama_axum/axum already present. |
| Platform/renderer requirements are realistic | ✓ | Zero-JS server render under `default-src 'self'` CSP; verified no script in template. |
| Test strategy is executable with current infrastructure | ✓ | Shipped anti-overclaim test verified; proposed model + `#[cfg(test)]` render tests match the existing home/about pattern (`pages.rs:146-223`). |
| Performance budget is realistic for target hardware | ✓ | One `Vec` of one `&'static str`-backed `Project` per request; no I/O — accurate. |
| No undeclared dependency on unbuilt features | ✗ | §3.7 asserts `--text-2xs: 0.70rem` **exists** (it does not — verified absent in `static/css/style.css` and repo-wide); §7.2 CSS cleanup depends on A1-created tokens (`--text-2xs`, `--measure-narrow`) — declared as an A1 dependency in §7.4, so the *behavioural* work is unblocked but the CSS-token target references values not yet defined. |

**Feasibility verdict:** Feasible with caveats
**Caveats:** (1) §3.7's `--text-2xs: 0.70rem` is a fabricated present-state token — real floor is `--text-xs: 0.75rem`; 0.72rem is genuinely below it, so the direction is right but the citation is wrong. (2) §7.2 target tokens `--text-2xs`/`--measure-narrow` must be created by the A1 sweep before they can be referenced. (3) Two off-by-one citations: Portfolio nav link is `base.html:26` not :25; card transition is `style.css:744` not :745. None block implementation.

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 3.00 | 25% | 0.750 |
| 2. Design & Craft Excellence | 2.67 | 25% | 0.667 |
| 3. Accessibility & Progressive Enhancement | 2.67 | 20% | 0.534 |
| 4. Competitive Depth & Differentiation | 2.80 | 20% | 0.560 |
| 5. Accuracy & Maintainability | 2.80 | 10% | 0.280 |
| **Composite** | | | **2.79** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 (2.79)
- [x] All lens averages ≥ 2.0 (min 2.67)
- [x] No criterion scores 0
- [x] No more than two criteria at 1 per lens (zero 1s anywhere)
- [x] All auto-fail rules pass
- [x] Feasibility ≠ Infeasible (Feasible with caveats)

**All conditions met:** Yes → PASS

---

## Remediation Brief (quality — spec already PASSES; no Priority-1 blockers)

### Priority 1 — Must fix to pass
None. All pass conditions are met and no auto-fail triggers.

### Priority 2 — Should fix for quality
1. **Correct the fabricated type token (§3.7, folds into §7.2/5A/2B).** There is no
   `--text-2xs: 0.70rem` in `static/css/style.css`; the type scale floor is
   `--text-xs: 0.75rem` (tokens: xs .75 / sm .85 / md .95 / lg 1.05 / xl 1.3 /
   2xl 1.6). Rewrite §3.7 to state the real floor, and in §7.2 either (a) have the
   A1 sweep create `--text-2xs`/`--measure-narrow` before B3 references them, or
   (b) map the `.project-status`/`.tag` 0.72rem literals to the existing
   `--text-xs`. Do not leave a spec instruction that points at a non-existent token.
2. **Close the two acknowledged a11y/resilience gaps before the list grows.** Add
   the `{% else %}` designed empty state to `templates/portfolio.html` with the
   §3.3 honest copy, and add the external-link cue (visible ↗ and/or `aria-label`
   suffix) to the entry-name link at `portfolio.html:17` (§7.2, criteria 3D/3F).
3. **Add the rendered-HTML integration test (§5.2)** in `src/handlers/pages.rs`
   `#[cfg(test)]` so the anti-overclaim guard covers the template, not just the
   model — the shipped test asserts on `name + description` only
   (`project.rs:103-107`).

### Priority 3 — Consider for excellence
1. Fix minor line citations so a downstream implementer lands correctly: Portfolio
   nav link is `base.html:26` (not :25); card transition is `style.css:744`
   (not :745).
2. Adopt the proposed `evidence: Option<&'static str>` field (§4.2, Q1) to make the
   criterion-1A evidence path first-class, and add `status_display_and_class_agree`
   (§5.1) to legitimise and exercise the `#[allow(dead_code)]` status variants.
```
