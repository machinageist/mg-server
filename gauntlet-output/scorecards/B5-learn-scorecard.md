# Scorecard: Learn (education wiki)

**Feature ID:** B5 / learn
**Spec file:** gauntlet-output/specs/B5-learn.md
**Reviewer agent:** verify-agent (Claude Opus 4.8), blind review
**Date:** 2026-08-08
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This is a rigorous, source-accurate spec whose pedagogical core
(criterion 2C) genuinely clears the Ciechanowski / Julia Evans / Arch Wiki bar —
verified against `osi-model.md` and the full 12-page corpus. Accessibility,
maintainability, and competitive-differentiation lenses are near-flawless. The
one real weakness, and the most important fix, is a systematic copy-currency
failure: 11 of 12 topic pages carry a user-visible `network-plus` tag for a
certification the live spine dropped on 2026-08-02, and §6.3 affirmatively
mislabels that tag as "aligned with the live cert spine." That finding sits at
the borderline of criterion 1D's "stale claims score 0" clause — scored 1 here
because currency is substantively addressed elsewhere, but a stricter reading
would score it 0 and flip the verdict to FAIL. It must be remediated before ship.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 3 | §1.2.2, §6.3, §7.1: no new resume-facing artifact is proposed (§7.3 "no new page content in scope"); the one linked evidence artifact is real owned work (`osi-model.md:257-260` → `/blog/hosting-machinageist-dev` with captured `dig`/`curl`). The Understand→Practice→**Evidence** contract (§6.3, LEARN.md) ties applied claims to documented work with a "separate what output proves from what you infer" rule (`osi-model.md:254`). | — |
| 1B. State honesty | 2 | §7.1 is a model implemented/gap/dead-code/defect state table; user-visible copy honestly frames the corpus as "an intentionally small first release" (`index.md:66`). **But** the spec's own current-state accounting miscounts: §6.3/§7.1 claim "~5 of 12" pages reach the Evidence part — verified actual is **3/12** (only `osi-model`, `network-applications`, `ipv4-addressing` carry `/blog/` links); and §7.1 claims "11/12 carry Related pages" — verified actual is **12/12**. Two counting errors in the state section; the 5/12 one mildly overstates shipped evidence. | Correct §6.3/§7.1 counts to 3/12 evidence pages and 12/12 related-pages sections. |
| 1C. Publication gates | 3 | §0 and §4.4 explicitly keep `content/drafts/geist-wiki/` (GeistScope docs) out of scope and unrouted ("no handler reads it"); verified `router.rs` has no drafts route and `content/drafts/` is not served. GeistScope gate respected. | — |
| 1D. Copy currency | 1 | §6.3 has a dedicated currency audit and invokes A2's U-7 retired-claims guard against "Network+" in summaries/meta — so currency **is addressed**. But it fails systematically: verified **11 of 12 topic pages carry a `network-plus` tag** (`osi-model.md:5` etc.; all networking pages), which renders as a user-visible `.tag` pill (`wiki_page.html:34-38`). Network+ was **dropped** from the live spine (RHCSA → CCNA → Security+, re-locked 2026-08-02). §6.3 affirmatively states these tags are "aligned with the live cert spine (RHCSA → CCNA → Security+)" — a false currency claim inside the spec. The U-7 guard covers summaries but **not tags**, and the spec proposes no tag scrub. This is the exact "CompTIA stack" failure 1D was written to catch, at 11× scale. **Borderline 0**: criterion states "stale claims that were true once still score 0 here." | **P1.** Retag the 11 networking pages to the live spine (drop `network-plus`; use `ccna` where apt) and correct §6.3 to stop asserting `network-plus` is spine-aligned. Extend the U-7-style guard to cover `tags`, not only `summary`. |
| 1E. Role posture | 3 | §6.3 verifies copy leads with "an early career in Linux systems administration and network operations" (`index.md:12-13`, confirmed) and confirms no senior/DevOps/SRE/enterprise/AI-infra language. FOSS/owned-scope framing (`index.md:76-79`). | — |
| 1F. Test-encoded policy | 3 | The spec preserves all existing anti-overclaim tests and only **adds** guards (T-B5-1…7); it proposes weakening none. §6.3 routes summaries through A2's U-7 retired-claims guard rather than relaxing it. | — |

**Lens average:** 2.50
**Lens pass:** Yes — avg ≥ 2.0, one 1 (≤ two allowed), no 0s. (Note: 1D is a borderline 0; see remediation.)

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation | 3 | B5 is not the design-system feature; §0's out-of-scope table defers all tokens/type-scale/theme-roster/SOLARCORE concerns to A1 and adds no colour literal. It actively improves reconciliation by deleting the only literal on the surface — `.wiki-disclaimer`'s `#e0a458` (verified `style.css:1046`). No stale-spec re-assertion. | — |
| 2B. Typographic craft | 3 | §3.3: prose capped at `--measure` (72ch); `pre`/tables keep the full column and scroll (verified `style.css:1124-1195` region); heading hierarchy addressed in §3.7D. Sidebar heading size fix (G9) raises 0.68rem → `--text-2xs`. | — |
| 2C. Pedagogical depth | 3 | **Verified against source, the centre of gravity.** `osi-model.md` builds Physical→Application in plain language, states where the model breaks ("error correction hides the mechanism" `:116-117`; "SSL is TLS's obsolete predecessor" `:162-163`; "UDP is not always faster" `:126-127`), gives an ASCII encapsulation diagram (`:188-194`), contrasts OSI/TCP-IP, turns the model into a troubleshooting procedure, ends with a FOSS `curl -v` practice on an owned site, and cites ISO/RFCs. `linux-abstraction-layers.md:10-11` leads concept-before-jargon. All 12 pages verified to carry Overview + Suggested-practice + Related-pages + Sources. Not a bullet-dump. T-B5-5 + LEARN.md codify the contract to keep it there. | — |
| 2D. Scannability & structure | 2 | Sidebar sections, cross-links (Related pages), sources, and the SIDEBAR↔WIKI_SLUGS↔disk three-way agreement guard (T-B5-4) are strong. **Gap:** criterion 2D names "index listings" as a place new pages must register; `index.md` manually lists every topic in its body (`index.md:36-63`), but no drift guard covers that listing. A page added to SIDEBAR/WIKI_SLUGS/disk could still be omitted from the overview body. | Extend the agreement guard (or LEARN.md pipeline) to also assert `index.md`'s body listing covers every SIDEBAR slug. |
| 2E. Restraint | 3 | Deletes the disclaimer banner per the copy-voice memory (§7.1 G3), keeps body copy quiet, no cards/fake-metrics/dashboard cosplay; only motion is the disclosure triangle, which it guards. Empty-adjacent state handled quietly (`index.md:66`). | — |
| 2F. Theme integrity | 3 | Changes are role-correct: delete the `#e0a458` literal (removes a per-theme hazard), raise a **size** token (`--text-2xs`, cross-theme), add `aria-current` (no colour). No per-theme edits required. §3.3 asserts no component reads a literal after the disclaimer deletion. | — |

**Lens average:** 2.83
**Lens pass:** Yes

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JavaScript | 3 | §3.7A + T-B5-6: native `<details>`/`<summary>` (verified `wiki_page.html:5-6`), real `<a href>`, server-rendered Markdown, zero JS. Test strips `<script>` and asserts every href + body survive. The headline property. | — |
| 3B. Contrast & colour independence | 3 | §3.7B identifies the 0.68rem sidebar-heading violation of A1's `--text-2xs` floor (verified `style.css:1425`) and targets a raise to an audited 4.5:1 token; active entry carries a non-colour 2px accent left-border (verified `style.css:1449-1452`). State never by hue alone. | — |
| 3C. Keyboard & focus | 3 | §3.7C: all-native controls, A1 global `:focus-visible` ring, DOM-order focus (back-link → article; sidebar precedes article in source). No custom shortcuts. | — |
| 3D. Semantics & AT | 3 | §3.7D catches a real outline defect — sidebar `<h2>`s precede the article `<h1>` (verified `wiki_page.html:10` before `:31`) — and remediates by demoting sidebar labels to non-heading elements (T-B5-7). Adds `aria-current="page"` (T-B5-2) for the AT-invisible active state. Landmark `<aside aria-label>`; naming-split (§7.1 G2) noted. | — |
| 3E. Motion & sensory safety | 3 | §3.5 identifies the single unguarded transition — the disclosure-triangle rotation (verified `style.css:1490-1494`, inside the 800px media but **not** inside any reduced-motion block) — and moves it under `prefers-reduced-motion: no-preference`. Verified it is the only B5 motion. | — |
| 3F. Responsive & resilient | 3 | §3.7F/§3.3: 320px→wide, 800px grid collapse, 200% zoom reflow (rem sizing), designed empty states (zero-entry section omitted; `index.md` names the small release). | — |

**Lens average:** 3.00
**Lens pass:** Yes
**Auto-fail triggered:** No — no-JS floor met (rule 3), accessibility floor met/remediated (rule 2: contrast fix, focus preserved, colour-independence, reduced-motion fix).

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 3 | §1.2.1 + the engineer-peer user story (§2) state the impression explicitly: "in thirty seconds I can tell this is understanding, not a memorised acronym" — writing that survives scrutiny and corrects the study guide. | — |
| 4B. Evidence over enthusiasm | 3 | §1.2, §6.3 lean on the real hosting walkthrough (an outage/real request worked end-to-end with captured commands) — the criteria's in-repo model; practice sections emphasize verification and "separate proof from inference" (`osi-model.md:254`). | — |
| 4C. Original explanation | 3 | The differentiator, verified strong (see 2C). §1.2.1 names it the entire point; the OSI corrections are genuine original explanation a working engineer would respect. | — |
| 4D. Depth of a real system | 3 | §4D/§1.2: connects to the genuinely operated stack (this server, DNS, Cloudflare, tunnel, Caddy, systemd via the hosting walkthrough) and mandates practice on hardware the reader already owns; honest concept-vs-operated distinction (`index.md:76-79`). (Weakest lens-4 3: only 3/12 pages currently tie to owned work — but honestly framed, not overclaimed.) | — |
| 4E. Reviewer paths | 3 | §2 user stories explicitly cover all three reviewer types the criteria name — self-directed learner, engineer peer, hiring manager — plus edge cases (stale/legacy slug), a11y readers, and the maintainer. | — |

**Lens average:** 3.00
**Lens pass:** Yes

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 3 | §5.1 note + §4.2 document the deliberate SIDEBAR/WIKI_SLUGS duplication ("so the test crate does not depend on the bin," verified `wiki_pages.rs:11-13`) **and name the guard** (T-B5-4: `pub fn sidebar_slugs()` → checked duplication). Exactly what 5A asks. | — |
| 5B. Drift guards | 3 | Existing E-1/E-2 tie each copy to disk (verified `wiki_pages.rs:30-71`); new T-B5-4 closes the un-guarded cross-copy drift (G4, verified real: nothing today compares SIDEBAR to WIKI_SLUGS directly), T-B5-5 lints structure, T-B5-7 pins heading order. The `section() -> "wiki"` magic-string coupling (G1) is flagged with an interim two-file fix. (The `index.md` listing gap is captured under 2D.) | — |
| 5C. No hidden coupling | 3 | §4.2 catches the stale `#[allow(dead_code)]` on `summary` — verified used via `WikiPageTemplate::description()` (`wiki.rs:114-116`) and rendered at `base.html:8` — "an annotation that lies about usage," and removes it (G10). G1 flags the magic-string coupling behind the nav highlight. New tests assert the thing they name (T-B5-2 aria-current directly). | — |
| 5D. Verification is stated | 3 | §5 intro + §1.3 name the exact CI commands: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo build --release` — matching criterion 5D. | — |
| 5E. Documentation follows behavior | 3 | §5.5/§7.2: new `docs/design/LEARN.md` updated in the same change; `README.md:71` corrected from "archive index" (verified current text: "archive index + pages, hardcoded SIDEBAR") to the education-wiki role. | — |

**Lens average:** 3.00
**Lens pass:** Yes

---

## Feasibility Check

Verified against source before filling.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | `Page`/`Frontmatter` (`page.rs:19-37`), `SidebarSection`/`SidebarEntry` + `SIDEBAR` (`wiki.rs:22-99`), `WikiPageTemplate` (`wiki.rs:101-121`) all verified as cited. |
| API/interface changes feasible | ✓ | `section()→Section::Learn` (A2 dep, declared), `title()` suffix, `pub fn sidebar_slugs()`, `aria-current`, `head_extra` og:type — all feasible; `head_extra` block exists empty (`base.html:14`). |
| Views/screens fit navigation | ✓ | `/learn`, `/learn/:slug`, `/wiki`, `/wiki/:slug` present (`router.rs`); `wiki_page.html` renders through `base.html`. |
| Dependencies available/version-compatible | ✓ | `gray_matter` + `pulldown_cmark` already vendored (`page.rs:12-14`); no new deps. |
| Platform/renderer requirements realistic | ✓ | `::details-content` override present (`style.css:1413`), `:has()` degradation, `Options::all()` (`page.rs:58`) verified. |
| Test strategy executable | ✓ | `cargo test --all-targets`; router `oneshot` and structure lints run with current infra; T-B5-4 needs a `pub fn` the test can call (trivial). |
| Performance budget realistic | ✓ | Text; `transmission-media.md` = 14,905 B = 14.9 KB matches the spec's cited "14.9 KB". gzip at Caddy. |
| No undeclared dependency on unbuilt features | ✓ (caveat) | Deps on A1 (`--text-2xs`, `--text-faint` 4.5:1 audit) and A2 (`Section::Learn`, `head_extra`, U-5/U-7 guards) are all **declared** (§0, §7.4). Note: `--text-2xs` does **not yet exist** in `style.css` (only `--text-faint`) — G9 is blocked on A1 introducing it; G1 has a documented interim two-file fix. |

**Feasibility verdict:** Feasible with caveats

**Caveats:**
1. `--text-2xs` token is currently absent from `style.css` (only `--text-faint` exists across the 23 theme blocks); G9's fix waits on A1. Declared, not undeclared.
2. §7.1/§6.3 state miscounts: evidence pages are **3/12** not "~5/12"; related-pages sections are **12/12** not "11/12".
3. `network-plus` tag is on **11/12** pages (user-visible), stale vs the live spine; §6.3 wrongly certifies it compliant. (Scored under 1D.)
4. Deleting `.wiki-disclaimer` (§7.2) should also drop its reference at `style.css:739` (shared transition selector list), not only the `1039-1054` block, or a dead selector fragment lingers.
5. §3.2 Flow A step 5 cites `scroll-margin-top` (`style.css:1160`) as clearing the header for "anchor targets"; that rule targets `#content` (the skip-link), and headings carry no ids yet (per the in-file comment), so in-page heading anchors don't yet benefit. Minor.

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 2.50 | 25% | 0.625 |
| 2. Design & Craft Excellence | 2.83 | 25% | 0.708 |
| 3. Accessibility & Progressive Enhancement | 3.00 | 20% | 0.600 |
| 4. Competitive Depth & Differentiation | 3.00 | 20% | 0.600 |
| 5. Accuracy & Maintainability | 3.00 | 10% | 0.300 |
| **Composite** | | | **2.83** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 (2.83)
- [x] All lens averages ≥ 2.0 (min 2.50)
- [x] No criterion scores 0 (1D = 1, borderline)
- [x] No more than two criteria at 1 per lens (Lens 1 has one; others none)
- [x] All auto-fail rules pass (none triggered)
- [x] Feasibility ≠ Infeasible (Feasible with caveats)

**All conditions met:** Yes → PASS

---

## Remediation Brief (priority items even on pass)

### Priority 1 — Must fix (borderline verdict-flipping)
1. **Scrub the stale `network-plus` tag (criterion 1D, borderline 0).** 11 of 12
   topic pages (`osi-model.md:5` and every networking page) display a
   user-visible `.tag` pill for a certification dropped from the live spine on
   2026-08-02 (RHCSA → CCNA → Security+). Retag networking pages to the live
   spine (drop `network-plus`; use `ccna` where the objective maps), and **correct
   §6.3**, which currently asserts these tags are "aligned with the live cert
   spine" — a false currency claim in the spec itself. Extend the retired-claims
   guard (A2 U-7) to cover the `tags` field, not only `summary`/`meta`. A strict
   reading of 1D's "stale claims score 0" scores this 0 and fails the spec.

### Priority 2 — Should fix for quality
2. **Correct §7.1/§6.3 state counts (1B).** Evidence pages are 3/12 (not ~5/12);
   Related-pages sections are 12/12 (not 11/12). On a claim-integrity-first site,
   the current-state section must count accurately.
3. **Guard the `index.md` body listing (2D/5B).** No drift guard covers the
   overview's manual topic list; extend T-B5-4 (or the LEARN.md pipeline step) so
   a page in SIDEBAR/WIKI_SLUGS/disk cannot be silently missing from `index.md`.

### Priority 3 — Consider for excellence
4. When deleting `.wiki-disclaimer`, also remove its reference at `style.css:739`
   (shared transition selector), not just the `1039-1054` block.
5. Tighten §3.2 Flow A step 5: `scroll-margin-top` (`style.css:1160`) targets
   `#content`, not in-page heading anchors (headings have no ids yet).
