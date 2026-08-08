# Scorecard: Design System

**Feature ID:** `design-system` (A1)
**Spec file:** gauntlet-output/specs/A1-design-system.md
**Reviewer agent:** Spec Gauntlet verification agent (Claude Opus 5), blind review
**Date:** 2026-08-07
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This is an unusually well-grounded spec. I independently re-derived
its contrast matrix, its 34 `font-size` literals, its six orphan classes, its two
dead selectors, its `#e0a458` literal, its unguarded transition, its stale
favicon, and all eight SOLARCORE_SPEC.md divergences — and nearly every one is
exact. Criterion 2A is satisfied emphatically: the spec found eight divergences
where the criteria named five, resolved every one in favour of the shipped site,
and nothing in it re-asserts the stale document. The most critical gap is that
its headline evidence figure is wrong: it claims **19 contrast failures** eight
times, but its own §7.1.4 table enumerates **14**, and my independent run of the
spec's own `USAGE` matrix also returns **14**. Secondary: every `style.css` line
citation above line 1112 is stale by exactly +31 because the spec was measured
one commit before `5e98092` (the Markdown-table polish pass), which the spec also
never mentions.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 3 | §1.4's ledger gives starting state, target state, and reasoning per divergence; §7.1 gives a quantified starting state; §5 gives verification commands; §8 Q1–Q7 gives what is still unknown. §8 Q7 explicitly maps a possible write-up to the 1A field list ("starting state, target state, what broke, verification, what is still unknown") and labels it **planned**, not shipped. | — |
| 1B. State honesty | 3 | §7.1 labels every capability `implemented` / `stale` / `absent` (7.1.1 implemented-and-good, 7.1.3 implemented-in-markup/stale-in-assets, 7.1.7 absent-for-labs). §6.3 lists the print stylesheet, `--border-strong`, `og-card.png` and the §8 write-up as planned. §4.3 marks `--check` and `--write` **NEW**. Verified: `generate_themes.py:285-299` accepts only `--css/--modes/--icons/--menu`, so both really are absent. | — |
| 1C. Publication gates | 3 | §7.4 "External gates: none — this feature publishes no artifact and makes no claim." Verified correct: the spec proposes no portfolio surface, and GeistScope appears only inside §1.4 D7's citation of the existing anti-overclaim test (`project.rs:110` asserts `!combined.contains("GeistScope")`). | — |
| 1D. Copy currency | 3 | §6.3 "Claim currency" scopes correctly: the design system introduces no certification copy; it explicitly refuses to absorb B2's fix of `pages.rs:92`; and it flags `lab.rs`'s stale `Network+` assertion as **C4's** 1D issue. Verified: `src/models/lab.rs:246` does assert `combined.contains("Network+")`, and `src/handlers/pages.rs:92` does still say "working through the CompTIA stack". | — |
| 1E. Role posture | 3 | §1.4 D7 removes the stale spec's role assertion. Verified: `docs/solarcore/SOLARCORE_SPEC.md:41` reads "machinageist.dev is a systems programmer's portfolio, blog, and tool wiki" — the spec quotes it exactly and deletes it, describing the artifact instead. §6.3 confirms the only user-visible text this feature owns is 24 theme labels, six group labels, and ARIA labels; verified against `templates/base.html:36-79` — none asserts anything about the author. | — |
| 1F. Test-encoded policy | 3 | No anti-overclaim guard is weakened anywhere; §5.1/§5.2 add ten Rust tests and four Python checks on top. Verified `cargo test --all-targets`: 30 + 2 tests pass, including all three anti-overclaim guards the spec cites (`pages.rs`, `project.rs`, `lab.rs`). | — |

**Lens average:** 3.00
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation *(blocking)* | 3 | §1.4 enumerates **D1–D8** — three more than the criteria's five — each with reconciliation and a design reason, all resolving to shipped. I verified all eight against `SOLARCORE_SPEC.md`: D1 line 18 ("Solarcore is the same future at night") + line 76 `--sc-void: #010915`; D2 line 261 ("scanlines, CRT filters") + line 264 ("No lightening/theming toggle for now"); D3 lines 193-194 split-colour `MACHINA`/`GEIST` + line 178 "12-tooth industrial cog"; D4 lines 76-87 `--sc-*`; D5 lines 55-56 principle 3 "Three neons, three jobs"; D6 lines 158/186-190 `vine-trace.svg`/`mark.svg`/`og-card.png`; D7 line 41; D8 line 168 `SYS: … STATUS: ONLINE … UPDATED: 2026-05-22`. §1.4's disposition retires the doc to a stub and demotes its philosophy prose to a **non-normative** Origins section. Nothing re-asserts the stale spec. | — |
| 2B. Typographic craft | 2 | §4.2's Layer 2 **extends** the shipped bare-`:root` block rather than replacing it, and §4.2 states "which is why the type scale landed outside the roster in commit `3f96165` and why it must stay there" — verified against `style.css:472-500`. §3.3's `ch`-over-`rem` argument is correct and non-obvious. §7.1.2 explicitly preserves prose-caps-at-`--measure` and `pre`-uncapped-and-scrolling (verified `:1102` and `:1081-1089`). §3.7E documents the deliberate h2-smaller-than-h3 inversion and pins it as an invariant. **Gaps:** (a) `.post-content table/th/td` shipped in `5e98092` and follows the identical "not continuous prose → full column, scrolls inside itself" rule as `pre`, and the spec never mentions tables — absent from §3.1's surface inventory, §3.3's component hierarchy, and §3.5/§4.2's rules; (b) §4.2's Layer 2 offers only `--measure: 72ch` and `--measure-narrow: 55ch`, which cannot absorb the two shipped `65ch` measures at `style.css:922` and `:1154`, so §7.2's "the four hardcoded `ch`/`px` measures" is unsatisfiable as written. | Add `table` alongside `pre` in §3.1's Article row, §3.3's hierarchy, and as a named exception to the `--measure` cap in §4.2 Layer 3. Add a third measure token (e.g. `--measure-mid: 65ch`) or state which of `:922` / `:1154` collapses to `--measure-narrow`. |
| 2C. Pedagogical depth | 2 | The spec governs the education surface structurally — §3.1 lists the Learn shell (13rem sidebar + article grid, `<details>` collapse below 800px; verified `style.css:1331-1469`) and names the article "the core reading surface"; §1.2 point 1 makes measure, line-height, contrast and quiet body copy load-bearing for twenty-minute reading; §3.7E's aria-current addition improves sidebar wayfinding. But there is zero engagement with explanation quality (0 occurrences of "pedagog", "teach", "learner", "glossary" as a design concern), and the spec never declares 2C delegated. | Add one line to §7.4 stating that 2C (concept-building, ordinary-language-before-jargon, FOSS practice) is owned by B*/C2/C3 and that A1 supplies only the reading surface. |
| 2D. Scannability and structure | 2 | §3.1's eleven-surface inventory maps to the real router (verified `src/router.rs:37-57`); §3.3 gives a top-to-bottom component hierarchy; §3.7E pins a heading-outline invariant ("no page may skip a level"); §7.1.6 correctly flags `post-group` and `post-group-heading` as unstyled, which is why `/blog`'s group headings currently have no visual hierarchy — verified: neither class has any rule in `style.css`. **Gap:** criterion 2D's named registration requirement is untouched. `WIKI_SLUGS` appears 0 times; every "sidebar" hit is the CSS class `.wiki-sidebar`. T2 checks the five *theme* registries only, not `wiki.rs::SIDEBAR` ↔ `tests/wiki_pages.rs::WIKI_SLUGS` ↔ `content/pages/`. | Either add a test to §5.1 asserting the `SIDEBAR` / `WIKI_SLUGS` / on-disk triple agree, or state in §7.4 that `tests/wiki_pages.rs` already guards it and A1 inherits that guard unchanged. |
| 2E. Restraint | 3 | §1.4 D2 rewrites the anti-goal rather than deleting it — spectacle is opt-in per theme, never the default, never the `system` resolution; verified `style.css:434-440`, the glow selector list genuinely excludes `lunarcore`. §1.4 D8 names `STATUS: ONLINE` as "dashboard cosplay" and cites 2E/4B directly. §3.5 rule 3 caps infinite motion at one chrome element. §3.6 forbids a toast/snackbar class outright on no-JS grounds. §4.7 records a rejected optimisation with its reason. §3.1 keeps divider lists rather than cards. | — |
| 2F. Theme integrity | 3 | §4.2 states the governing rule — "themes own colour and font role; they never own size or spacing" — and its corollary, and enforces it from **both** directions (T5: no theme block declares a length; T3/T4: no colour or `font-size` literal below the layer boundary). The one new theme-owned token, `--border-strong`, is justified as the criterion's own carve-out: "one edit producing 23 blocks — not 23 edits". The `--shadow` split is a real fix, not a cosmetic one: verified `generate_themes.py:178` emits `0 8px 24px rgba(0,0,0,{0.5\|0.12})`, fusing geometry with colour and duplicating the geometry 23 times. §4.3 keeps the `MENU_GROUPS` drift guard (verified `generate_themes.py:262-268`). | — |

**Lens average:** 2.50
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JavaScript *(auto-fail gate)* | 3 | §3.2's "Branch — no JavaScript" separates current from target behaviour and §6.4's 3A row states the principle ("JS adds *choice*, never *access*"). The specificity argument is **correct**: `:root[data-theme="lunarcore"]` is (0,2,0) and beats a media-query `:root` at (0,1,0) — verified that `style.css:17` is the selector list `:root, :root[data-theme="lunarcore"]`, whose members are scored independently, so a later `@media (prefers-color-scheme: light) { :root {…} }` wins the bare case and loses to every explicit `[data-theme]` block regardless of order. The third branch is a genuine find: verified `theme-init.js:7-14` wraps `setAttribute` **inside** the `try`, so a throwing `getItem` skips the attribute — which under the proposed `[data-theme]` gate would hide the control from JS-capable readers. The spec catches this and narrows the `try`. | — |
| 3B. Contrast and colour independence *(auto-fail gate)* | 2 | The framing is right — §3.7A replaces the shipped token-vs-`--bg` list with a (foreground × background × rendered size) matrix, thresholds by rendered size not token name, and raises `--text-faint` from AA-large 3.0 to 4.5 (verified `generate_themes.py:150` holds `faint` at 3.0). Every **individual** measurement I re-derived is exact: faint/bg solarized 3.64, c64 3.93, gameboy 3.97; faint/surface solarized 3.15, c64 3.27, gameboy 3.43, cloud 4.28, blueprint 4.41, lunarcore 4.47, nes 4.47; muted/surface solarized 4.13; accent/surface cloud 4.32, solarized 4.12; code/surface solarized 4.06. So are the range claims: `--border` 1.38–2.18 vs `--bg`, `--accent-border` 1.41–2.08, accent-vs-bg min cloud 4.70, accent-vs-surface min solarized 4.12, and "generator reports 0 failures at its own thresholds". §3.7B's non-hue enumeration is complete and verified (no `text-decoration: none` exists anywhere inside `.post-content`, so prose links really are underlined by UA default). **The defect:** the headline total is wrong. Those enumerated failures sum to **14**, and my independent run of the spec's own §4.2 `USAGE` matrix over all 23 themes also returns **14** (`accent_hover` contributes none). "19" is asserted at §1.3, §3.7A, §5.2 P1, §7.3, §7.4, §8 Q2 and §8 Q4 — including as §1.3's *measurable success signal* and as P1's test baseline. The sub-counts are both right ("solarized accounts for five" = 5 ✓; "`--text-faint` is the source of ten" = 10 ✓), which is what exposes the total as an arithmetic error rather than a different matrix. | Replace **19 → 14** at spec lines 51, 341, 729, 909, 919 ("five of the fourteen"), 1024, 1060, 1086, 1102 ("ten of the fourteen"). Re-verify by running the §4.2 `USAGE` matrix against `generate_themes.py::contrast` before re-submitting. |
| 3C. Keyboard and focus | 3 | §3.2's flow and §3.7C–D describe the shipped APG roving-focus model accurately — verified `main.js:43` (`tabindex="-1"` on every item), `:55` (open moves focus to `checkedIndex()`), `:59-66` (arrows wrap, Home/End jump, Escape and Tab close), `:60` (Escape returns focus to the button), `:74` (selection returns focus). §3.7C: `:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px }` verified at `style.css:685`, never removed; the skip-link's use of `:focus` rather than `:focus-visible` verified at `:539` and correctly defended. The ≥3:1 focus-ring claim on both `--bg` and `--surface` across all 23 themes verified. §3.4's 44px hit-area target addresses a real gap — verified `.theme-btn` is `width: 2rem; height: 2rem` at `style.css:603-604`. §3.4 also correctly refuses to add single-letter accelerators. | — |
| 3D. Semantics and assistive technology | 3 | Every §3.7E claim verified in `templates/base.html`: `lang="en"` (:2); `header` / `nav[aria-label]` / `main#content` / `footer` landmarks (:19-20, :85, :89, :92); brand SVG `aria-hidden="true" focusable="false"` (:21); all six `role="group"` + `aria-label` wrappers (:37, 43, 50, 57, 63, 70) with the visible label `aria-hidden` (:38 etc.); `role="menu"` (:36), `role="menuitemradio"` + `aria-checked` on all 24 items, `aria-haspopup` + `aria-expanded` (:30). The `aria-current="page"` gap is real — verified zero occurrences anywhere in `templates/`, `src/`, or `static/`. The h2-as-section-label inversion is documented in-file (verified `style.css:757-758`) and pinned as an invariant. | — |
| 3E. Motion and sensory safety *(auto-fail gate)* | 3 | §3.5's nine-row inventory is verified complete against `style.css`: the only three `@media` motion contexts are `prefers-reduced-motion: reduce` at `:461` and `no-preference` at `:710` and `:1310`. The single unguarded transition is real and uniquely identified — `.wiki-nav > summary::before { transition: transform 0.15s ease }`, which the spec cites as `:1407-1411` and which actually sits at `:1438-1442` (see Feasibility caveat 1). §3.5's four rules are sound; rule 2 (hidden state inside the keyframe with `fill-mode: both`) is verified as the shipped pattern at `:1313`. `cursor-blink` at 1.2s `step-end` is correctly argued to be a two-state toggle well under the 3 Hz photosensitive threshold. §3.5 rule 4 (reduced-motion alternative is *absence*, never a substitute animation) is the right call. | — |
| 3F. Responsive and resilient | 3 | §3.4's "two breakpoints total" verified exactly — `max-width: 800px` at `:1404` and `max-width: 640px` at `:1475`, and nothing else. §3.4 honestly states the limitation that custom properties cannot be used inside media queries rather than pretending otherwise. §5.4's configuration extremes cover 320/800/1280/2560px, 24px browser default **and** 200% zoom, forced `reduce`, and the exact `prefers-color-scheme: light` + JS-off case this spec exists to fix. §3.3's empty-state invariants are concrete (omit the section entirely, never a placeholder card, never a spinner) with `.releases-empty` as the reference implementation and `templates/index.html:30` + `pages.rs:186-202` as the shipped precedent — both verified. §3.6 E3 is real: the document is server-rendered Askama with genuine landmarks, so a failed stylesheet degrades to readable HTML. | — |

**Lens average:** 2.83
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s
**Auto-fail triggered:** No

- **Rule 1 (unearned claims):** Not triggered. The spec proposes no certification claim, no offensive-security identity, and consistently labels planned work as planned (§6.3, §7.1). The numeric errors documented below are accuracy defects, not capability overclaims.
- **Rule 2 (accessibility floor):** Not triggered. The spec neither removes focus states, nor signals state by hue alone, nor ignores `prefers-reduced-motion`. It commits to a zero-failure contrast matrix enforced by a non-zero-exiting `--check` (§5.2 P1, §7.3 step 2), and §8 Q2 explicitly rejects option (d) "accept a documented exception, which criteria rule 2 forbids". Leaving *how* solarized is fixed open (Q2) does not breach the floor because the outcome — zero failures — is not optional in the spec.
- **Rule 3 (no-JS floor):** Not triggered. The spec strengthens the no-JS path in two ways and gates only the theme *chooser*, which is enhancement, not access.

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 2 | §1.2 point 2 states a real, defensible differentiator: "The site is itself the portfolio entry… An engineer who opens devtools *is* reviewing the work. The stylesheet, the generator, and the no-JS fallback are the artifact." Verified against `src/models/project.rs:92-116`, which does pin the portfolio at exactly one entry (`mg-server`, Active, with a URL). That is precisely the gap the criteria table names against both competitor groups. But the spec never states what the thirty-second impression *is* — 0 occurrences of "differentiat", "competitor", "thirty". | Add a two-sentence "thirty-second impression" statement to §1.2 or §1.3: what a reviewer concludes in thirty seconds and which shipped surface delivers it. |
| 4B. Evidence over enthusiasm | 3 | §1.4 D8 draws the fake-metric/real-metric line explicitly and correctly: verified `templates/vitals_strip.html` renders `crate::state::Status::current()` at request time with no JS and no polling, and `src/state.rs:262` really does read `/proc/self/status` for `VmRSS`, with `build.rs` stamping `BUILD_TS`. §7.1.4–7.1.6 publish quantified failure rather than green screenshots — including that the `?v=` cache-buster is **already stale**, which I verified: `base.html:11-13` still reads `20260719-spectrum` even though `3f96165` changed `style.css`. §3.6 designs the failure modes with recovery and data-loss columns. §8 publishes seven open questions rather than hiding them. §4.7 records a rejected optimisation with its cost. | — |
| 4C. Original explanation | 2 | The spec's *own* reasoning is the quality the criterion asks for — the `ch`-vs-`rem` measure argument (§3.3), the specificity proof (§3.2), the per-theme-stylesheet rejection (§4.7), and every "Why shipped wins" cell in §1.4 are original technical explanation a working engineer would respect (D4's "`--sc-cyan` is a lie the moment a second theme exists — in Gruvbox it is yellow" and D5's "two roles is the largest number every palette can carry" are both correct and non-obvious). But the criterion targets *educational material shipped to readers*, and A1 produces none and delegates none explicitly. | Note in §7.4 that A1 supplies the reading surface and that explanation quality is C2/C3/B*'s criterion; optionally promote §8 Q7's write-up from "worth writing?" to a named downstream deliverable. |
| 4D. Depth of a real system | 2 | §1.4 D8 and §4.5 anchor decisions in the real deployed process rather than in preference: the CSP is quoted as the *reason* a CDN is impossible, not as a style choice (verified `src/middleware/security_headers.rs:39-51`), and the vitals strip's provenance is real. §4.6's `:has()` browser floor and §4.7's byte budgets are grounded in the actual shipped files. But the connection is to *this server process* only — 0 occurrences of Proxmox, Caddy, or Cloudflare, so the criterion's named systems (Proxmox cluster, DNS) go untouched. Defensible for a stylesheet feature, but it is a partial answer. | Optional: one line in §1.2 connecting the design system to the operated stack (Caddy + Cloudflare Tunnel on the Proxmox VM) as the reason zero-build-step and zero-external-request are constraints rather than preferences. |
| 4E. Reviewer paths | 1 | Addressed only incidentally. Two arrival routes appear with needs attached — §2's first user story ("found a `/learn` page from a search result… comfortable to read for twenty minutes") is the self-directed learner, and §1.2 point 2's devtools-opening engineer is the peer reviewer. But the **hiring-manager path is entirely absent** (0 occurrences of "hiring", "peer", "learner", "reviewer"), reviewer paths are nowhere treated as a design input, and §3.1's surface inventory is organised by page rather than by arrival route. §7.4's "Downstream" list is a feature-dependency graph, not a reader map. | Add a short §3.1 subsection or §2 story set naming the three reader paths, where each lands first (hiring manager → `/` then `/portfolio`; engineer peer → devtools, `/status`, the GitHub source link in `templates/base.html:93`; self-directed learner → a `/learn` page from search), and which design-system surface carries each one's first impression. |

**Lens average:** 2.00
**Lens pass:** Yes — avg ≥ 2.0 (exactly at threshold), one criterion at 1, no 0s

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 2 | §4.1 and §4.2 name `generate_themes.py` the SOURCE OF TRUTH; §4.3's `--write` with region markers collapses the four-registry hand-splice into one command; T2 asserts all five registries agree (verified they currently do: `theme-init.js` MODES = 24, `main.js` MODES = 24, `base.html` `data-mode` count = 24, 23 CSS `[data-theme=]` blocks + the `system` pseudo-mode). I confirmed `emit_css()` output is already byte-identical to `style.css:15-470` and `emit_menu()` to `base.html:37-79` modulo indentation, so P4 is achievable today. **Docked to 2** because the spec's own quantitative baseline is internally inconsistent — §7.1.4's table enumerates 14 failures under a "**19 failures**" heading, and that 19 is then propagated into §1.3's success signal and §5.2 P1's test baseline. A spec whose lens is "does this create future drift" cannot ship a headline number that disagrees with its own table. | Fix the 19 → 14 propagation (see 3B). Additionally: §7.1.7 states "`src/models/lab.rs` is untracked" — it is **tracked**, committed in `e0b6c8b` ("labs page"). The rest of that sentence (12 entries all `Queued`, three tests, no handler/route/template) is verified correct. |
| 5B. Drift guards | 3 | §7.1.5's four-vector table is exactly right and I verified all four independently, including the fourth: `base.html:11-13` still carries `?v=20260719-spectrum` while `3f96165` changed `style.css` and touched nothing else, so the cache-buster is already stale as claimed. The response is complete: `--check` exits non-zero (P1), `--write` + `git diff --exit-code` (P4) is the guard that would have caught `fbc6c2e`, the `MENU_GROUPS` `SystemExit` guard is explicitly **kept** (verified `generate_themes.py:262-268`), a second marker-balance guard is added, and T1–T10 catch the text-file drift class from ten angles. `asset_version()` replaces the hand-typed literal with `env!("BUILD_TS")` — verified `build.rs` emits it and `src/state.rs:34` already reads it. | — |
| 5C. No hidden coupling | 3 | §8's second flagged observation reproduces criterion 5C's named example and I verified it exactly: `src/handlers/pages.rs:158` asserts `html.contains("CompTIA")`, and "CompTIA" appears **nowhere** in `templates/` or `content/` — only in the meta description rendered from `pages.rs:44`. The spec assigns it to **B1** rather than absorbing it, which is the correct boundary. T8 and T9 attack the same class bidirectionally (markup→CSS and CSS→markup), and both are honestly reported as currently failing with the exact offenders — verified: all six orphan classes (`article-page`, `bio-loc`, `brand-word`, `post-group`, `post-group-heading`, `vitals-item`) exist in templates with zero CSS rules, and `.pm-status`/`.box` appear only inside the transition selector list at `style.css:714`. | — |
| 5D. Verification is stated | 3 | §5's preamble names the four CI commands verbatim and they match `.github/workflows/ci.yml` exactly (`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo build --release`, on `ubuntu-latest`). §5.2's added CI step is three lines of YAML requiring no new tooling. §7.3 gives a per-step verify command for all six sequenced steps. §5.3 states the E2E absence as a **decision with a reason** rather than an omission, and names what covers it instead. `tests/design_system.rs` is feasible: `tests/wiki_pages.rs` already proves an integration test can read repo files. | — |
| 5E. Documentation follows behavior | 3 | §1.4's disposition, §4.1's artifact map, and §7.2's modified-files list all commit `docs/design/DESIGN_SYSTEM.md`, the `SOLARCORE_SPEC.md` stub, and `docs/themes/README.md` as same-change updates. §7.3 step 6 gives the verification: "every §1.4 divergence appears in the new document with its reconciliation." §6.2 additionally schedules palette attributions (Dracula, Solarized, Nord, Gruvbox — all MIT) into the new document. This is the strongest part of the spec. | — |

**Lens average:** 2.80
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Feasibility Check

Verified empirically against the working tree at `aee8a4b` before filling this table.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | §4.2's Layer 1 contract matches `generate_themes.py::_block` exactly — 14 tokens + `color-scheme`. `asset_version()` sits beside an existing `env!("BUILD_TS")` read at `src/state.rs:34`; `build.rs` emits epoch seconds, valid as a query string. |
| API/interface changes are feasible with current architecture | ✓ | `--check` / `--write` are argv additions to the existing `__main__` block (`generate_themes.py:285-299`). Region markers are new, but I confirmed `emit_css()` output already matches `style.css:15-470` byte-for-byte and `emit_menu()` matches `base.html:37-79` modulo leading indentation — so P4 (`--write` then `git diff --exit-code`) is achievable without first re-baselining. Caveat: the spec never says how `--write` reproduces `base.html`'s 12-space indentation. |
| Views/screens fit current navigation pattern | ✓ | No new screens. §3.1's eleven surfaces map to `src/router.rs:37-57`. |
| Dependencies are available and version-compatible | ✓ | Zero new packages, confirmed against `Cargo.toml` (axum 0.7, askama 0.12, edition 2024 — untouched). `tests/design_system.rs` is auto-discovered as an integration target. CI runs `ubuntu-latest`, which ships `python3`. No CSS framework, no build step, no webfonts — and the CSP would block a CDN anyway. |
| Platform/renderer requirements are realistic | ✓ | `:has()` verified at `style.css:740`; `::details-content` verified at `:1361`. The "no `@media print` block" claim is verified: the only `@media` rules in the file are reduced-motion (`:461`, `:710`, `:1310`) and the two width breakpoints (`:1404`, `:1475`). §4.6's degradation rule (no CSS feature whose absence breaks reading) is sound. |
| Test strategy is executable with current infrastructure | ✓ | `cargo test --all-targets` passes today (30 unit + 2 integration). Caveat: T3, T4, T8, T9 and T10 are specified as currently failing, so they must land red-then-green or be gated behind the cleanup step — §7.3's sequencing handles this correctly (guards first, cleanup in steps 3–5). |
| Performance budget is realistic for target hardware | ✗ | The §4.7 "Measured today" figures are one commit stale. Actual: `style.css` is **42,369 B / 8,544 B gzipped / 1,502 lines**, not 41,272 / 8,248 / 1,471. Headroom under the 48 KB cap is ~5.6 KB, not ~6.7 KB. The budget itself still holds comfortably; the measurements do not. `main.js` 4,664 B / 80 lines, `theme-init.js` 960 B / 15 lines, favicon 2,387 B, and the 14,588 B orphan total are all **exact**. |
| No undeclared dependency on unbuilt features | ✓ | §7.4 declares A1 the root with no upstream, verified. Nothing it proposes needs C4; §7.1.7 correctly identifies `.status-queued` / `.status-completed` as the only thing C4 needs *from* A1 (verified `LabStatus::class_name` at `src/models/lab.rs:50-57` emits `queued`/`completed`, and neither has a CSS rule). |

**Feasibility verdict:** Feasible with caveats

**Caveats — all verified against the tree, listed so a remediating agent can fix them without re-deriving:**

1. **Every `style.css` citation above line 1112 is stale by exactly +31.** The spec was measured before `5e98092` ("style: give Markdown tables real cell structure"), which inserted 31 lines at `:1112`. Corrections: unguarded transition `:1407-1411` → **`:1438-1442`**; `.wiki-sidebar h2` 0.68rem `:1342` → **`:1373`**; `.status-*` `:1185-1187` → **`:1216-1218`**; `.releases-empty` `:1193-1196` → **`:1224-1227`**; `::details-content` `:1330` → **`:1361`**; wiki layout `:1300-1438` → **`:1331-1469`**; 640px breakpoint `:1444-1471` → **`:1475-1502`**; boot-log `:1275-1289` → **`:1306-1320`**; `--text-3xl` user `:1291` → **`:1322`**; Layer 3 range `502-1471` → **`502-1502`**. Every citation at or below `:1112` is **exact** — I spot-checked `:513` (`font-size: 15px`), `:534-539` (skip-link), `:609` (`.theme-btn` border), `:619-637` (theme menu), `:656`, `:678`, `:685`, `:705-708`, `:710`, `:740`, `:757-766`, `:794`, `:822`, `:973`, `:998-1013` including `#e0a458` at `:1005`, `:1044`, `:1049-1062`, `:1077`, `:1081-1089`, `:434-442`, `:445-459`, `:461-469`, and `:472-500`. The **34 `font-size` literals** count is exact. `theme-init.js`, `main.js`, `base.html`, and `generate_themes.py` citations are all correct.
2. **The "19 failures" figure is wrong; the correct count is 14.** Independently re-derived twice (from the spec's own §7.1.4 table, and by running its §4.2 `USAGE` matrix against `generate_themes.py::contrast` over all 23 themes).
3. **`src/models/lab.rs` is tracked, not untracked** (§7.1.7). It was committed in `e0b6c8b`.
4. **`/blog/network-migration` does not exist** (§5.4's Tier-1 article surface). The shipped slug is `management-layer-first-network-migration`; `network-migration` is only a test fixture inside `pages.rs`.
5. **The shipped `.post-content table/th/td` rules are unaccounted for** (see 2B).
6. §4.5's CSP quotation omits `img-src 'self' data:` and `connect-src 'self'`; the file is `src/middleware/security_headers.rs` and the directive block is `:39-51`, not the bare `security_headers.rs:38-50` cited. Does not change any conclusion the spec draws from it.
7. `generate_themes.py:146-148` is cited for the faint-3.0 threshold; the `checks` dict is actually `:148-152` with `faint` at `:150`.

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 3.00 | 25% | 0.750 |
| 2. Design & Craft Excellence | 2.50 | 25% | 0.625 |
| 3. Accessibility & Progressive Enhancement | 2.83 | 20% | 0.567 |
| 4. Competitive Depth & Differentiation | 2.00 | 20% | 0.400 |
| 5. Accuracy & Maintainability | 2.80 | 10% | 0.280 |
| **Composite** | | | **2.62** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 — 2.62
- [x] All lens averages ≥ 2.0 — 3.00 / 2.50 / 2.83 / 2.00 / 2.80
- [x] No criterion scores 0
- [x] No more than two criteria at 1 per lens — one criterion at 1 total (4E)
- [x] All auto-fail rules pass — rules 1, 2 and 3 all clear
- [x] Feasibility ≠ Infeasible — Feasible with caveats

**All conditions met:** Yes → **PASS**

---

## Improvement Brief (PASS — not blocking, ordered by cost of leaving it)

### Priority 1 — Correctness defects that will mislead a downstream implementer

1. **Fix the failure count: 19 → 14.** Edit spec lines 51, 341, 729, 909, 1024, 1060; and lines 919 and 1102 where the prose reads "five of the nineteen" and "ten of the nineteen" (the sub-counts 5 and 10 are correct, only the denominator is wrong). §1.3's success signal and §5.2's P1 baseline are the two that matter most, since a test written against "19" would be wrong on day one.
2. **Rebase every `style.css` line citation above 1112 by +31.** Full correction list in Feasibility caveat 1. The spec was written against `7029d0c`/`3f96165` state; the tree is at `5e98092`+. Citations at or below 1112 need no change.
3. **Correct §4.7's "Measured today" table** to `style.css` 42,369 B / 8,544 B gzipped / 1,502 lines, and the headroom sentence to ~5.6 KB. The 48 KB / 10 KB budgets still hold and need no change.
4. **Correct §7.1.7:** `src/models/lab.rs` is tracked (committed `e0b6c8b`). The substantive claim — 12 `Queued` entries, three tests, no handler/route/template, so C4 is blocked on `.status-queued`/`.status-completed` — is verified correct and should stand.
5. **Correct §5.4's Tier-1 article path** to `/blog/management-layer-first-network-migration`.

### Priority 2 — Coverage gaps against criteria that cost real score

6. **Account for `.post-content table/th/td` (shipped in `5e98092`).** Add `table` beside `pre` in §3.1's Article row ("prose column capped at `--measure`; `pre` **and `table`** uncapped and scrolling"), in §3.3's component notes, and as a named `--measure` exception in §4.2's Layer 3. The rules use only theme tokens and `var(--text-sm)`, so they already satisfy T3 and T4 — this is a documentation gap, not a conflict, but the spec currently reads as though the newest component does not exist. (2B)
7. **Resolve the 65ch measures.** §4.2 offers `--measure: 72ch` and `--measure-narrow: 55ch`, but `style.css:922` and `:1154` both hardcode `65ch`. Either add a third token or state which existing token absorbs them, so §7.2's "the four hardcoded `ch`/`px` measures" is actionable. (2B)
8. **Add reviewer paths.** Name the three arrival routes and what each needs, and map each to the design-system surface that carries its first impression: hiring manager → `/` then `/portfolio` (status pills, the single-entry discipline); engineer peer → devtools, `/status`, the source link at `templates/base.html:93`; self-directed learner → a `/learn` page arrived at from search (sidebar hierarchy, active-page marker, measure). This is the spec's only criterion scoring 1. (4E)
9. **State the 30-second impression explicitly** in §1.2 or §1.3 — one or two sentences on what a skimming reviewer concludes and which shipped surface delivers it. §1.2 point 2 has the raw material; it just is not stated as an impression. (4A)
10. **Delegate 2C and 2D's registration requirement explicitly.** Add to §7.4 that pedagogical depth is B\*/C2/C3's criterion and that A1 supplies only the reading surface; and either add a `SIDEBAR` ↔ `WIKI_SLUGS` ↔ `content/pages/` agreement test to §5.1 or state that `tests/wiki_pages.rs` already guards it and A1 inherits it unchanged. (2C, 2D)

### Priority 3 — Polish

11. **Specify `--write`'s indentation handling** for `base.html` (12 spaces) and `style.css`, since P4 (`git diff --exit-code`) is byte-exact. `emit_css()` already matches the committed CSS byte-for-byte, so only the menu emitter needs an indent parameter.
12. **Quote the CSP in full** in §4.5 (`img-src 'self' data:` and `connect-src 'self'` are omitted) and correct the path to `src/middleware/security_headers.rs:39-51`.
13. **Correct the `generate_themes.py:146-148` citation** to `:148-152` (the `checks` dict), with `faint`'s 3.0 threshold at `:150`.
14. **Ground §1.2 in the operated stack** — one line connecting zero-build-step and zero-external-request to Caddy + Cloudflare Tunnel on the Proxmox VM would move 4D from 2 to 3 at negligible cost.
15. **Consider promoting §8 Q5 (`prefers-contrast: more`) to in-scope**, as the spec itself recommends. It is ~8 lines once `--border-strong` exists and adds a real accessibility affordance no competitor in the field set ships.
