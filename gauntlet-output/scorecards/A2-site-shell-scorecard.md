# Scorecard: Site Shell

**Feature ID:** `A2` / `site-shell`
**Spec file:** gauntlet-output/specs/A2-site-shell.md
**Reviewer agent:** verification-agent-A2 (Claude Opus 5), blind review
**Date:** 2026-08-07
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This is a strong, source-grounded spec whose centre of gravity is exactly
right — it treats the no-JS floor as the auto-fail gate it is (§3.6 E-05, §4.2, I-2,
§6.4 3A), and F-01/F-03 are real, independently confirmed defects (`grep
prefers-color-scheme static/css/style.css` returns zero matches; the theme button at
`base.html:30` is genuinely dead with JS off). Its most critical weakness is Lens 5B:
**three of its own proposed drift guards are red-on-arrival and the spec does not say
so** — U-10 asserts a theme-order equality that is false today, U-6 fails on three
descriptions the spec itself owns, and U-7's substring ban fires on shipped
educational copy. The single most important fix is to correct those three test
definitions (§5.1) before any implementer trusts them.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 3 | The one artifact A2 puts in front of a reviewer is the vitals strip, and §1.2 point 2 supplies the full evidence frame: why it matters, that it is read from the live process at render time, and that it costs zero JS. Verified: `vitals_strip.html:6` calls `crate::state::Status::current()`, `state.rs:239-254` is the snapshot, `state.rs:262-272` reads `/proc/self/status`. §7.1's F-01…F-19 table is the "what broke" record with a citation per row; §8's seven open questions are the "still unknown" column. | — |
| 1B. State honesty | 3 | §7.1 buckets the codebase into "Implemented and correct", "Prototyped / partial", "Planned / absent" and gets each right — verified that `{% block head_extra %}` (`base.html:14`) and `{% block scripts %}` (`:100`) have zero consumers, and that `static/img/` holds only `favicon.svg`, `mark.svg`, `mark-sm.svg`, `vine-trace.svg` with no `og-card.png`. §6.3 binds the rule: planned items "produce **no user-visible copy** until it lands." §3.1 marks the static-asset 404 as **New**. | — |
| 1C. Publication gates | 3 | §6.3 "Releases copy note" catches that `ReleasesTemplate::description()` publishes "GeistScope source tarballs and compiled binaries" into `<meta>` and `og:description` through the shell (verified at `releases.rs:40`), records the exposure, and hands it to B6 rather than resolving it in A2's territory. §8 Q2 re-uses the gate as an argument against promoting `/releases` into the nav. | — |
| 1D. Copy currency | 3 | §6.3 "The inherited violation A2 must guard" names `pages.rs:44`, `pages.rs:81`, `pages.rs:91-93` (all verified verbatim), states the 2026-08-02 re-lock and the live spine, declines to rewrite copy it does not own, and contributes U-7 as the CI guard. It also names the ordering trap — "U-7 will fail CI the moment it lands, so it must be committed *with* or *after* the copy fix" (§7.4). This is the best-handled criterion in the document. | — |
| 1E. Role posture | 2 | The posture is right and A2 authors no role claim of its own; §6.3's copy inventory is conservative (MG-BIOS classed as "Costume, not a capability claim"). But the **mechanism is over-broad and unverified**: U-7 (§5.1) bans the substring `"enterprise"` across *every* `description()`, and `content/pages/network-topologies.md:4` summary reads "…the tiered designs used in **enterprise** and data center networks", which reaches `<meta>` via `WikiPageTemplate::description()` (`wiki.rs:114-116`). U-7 conflates first-person role vocabulary with topic vocabulary. | In §5.1 U-7, split the list: keep `"production-grade"`, `"SRE"`, `"red-team"`, `"pentest"`, `"offensive security"`, `"Network+"`, `"the CompTIA stack"` as unconditional bans; drop bare `"enterprise"` and bare `"A+"` or scope them to first-person constructions (e.g. ban `"enterprise-grade"` and the regex `\b(I|we)\b[^.]*\benterprise\b`). Verify the final list against every shipped `description()` and every `content/**/*.md` `summary:` before the test is written. |
| 1F. Test-encoded policy | 3 | No anti-overclaim guard is weakened. I-6 (§5.2) *extends* the existing `internal_error_page_leaks_nothing` allowlist (verified at `errors.rs:184-193`) with `"panicked"`, `"askama"`, `"axum"`, `"tower"`. The one guard the spec proposes to change — the `html.contains("CompTIA")` assertion in `home_page_shows_concrete_work_without_strategy_narration` — is *relocated*, not relaxed, and the decision is explicitly recorded with rationale in F-17 and cross-feature request 2 (§7.4). | Minor: reconcile §7.4 requests 1 and 2 with each other. Request 1 has B1 remove "CompTIA study" from `IndexTemplate::description()`; request 2 says the `"CompTIA"` assertion "belongs in a metadata test". After request 1 lands there is no CompTIA string to assert. State that the relocated assertion must pin the *new* spine wording, not the retired one. |

**Lens average:** 2.83
**Lens pass:** Yes — avg ≥ 2.0, one criterion at 1, no 0s

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation | 3 | The closing "Documents that must be updated" section names the two SOLARCORE_SPEC.md contradictions inside A2's own territory — §6.4's `SYS: … │ OPERATOR: … │ STATUS: ONLINE` footer readout and §7's split-colour `MACHINA`/`GEIST` wordmark — states plainly that neither shipped, and applies the resolved direction: "Per criterion 2A the **shipped site wins** — A1 owns the rewrite, and A2's shipped shell is the reference it must be rewritten against." Verified: SOLARCORE_SPEC.md §6.4 is at line 164, the wordmark spec at §7, and neither matches `base.html:21` or `vitals_strip.html`. Nowhere does the spec re-assert the stale document. | — |
| 2B. Typographic craft | 2 | §3.3 and §3.7 describe the shipped system accurately — 15px body (`style.css:513`), 1.125-ratio scale (`:480-500`), `--measure: 72ch` (`:497-499`), "code keeps the full column" — and correctly defer token values to A1 (§0 table). **Gap:** the shell's own chrome bypasses the very scale the polish pass just introduced. `.nav-link` is `0.875rem`, `.site-footer` `0.8rem`, `.theme-menu button` `0.8rem`, `.vitals-strip` `0.75rem`, `.theme-group-label` `0.65rem`, `.brand` `1rem` — none of these are `--text-xs/sm/md/lg`. A2 owns every one of those elements and §4.7's "no growth" budget and §3.7's text-scaling paragraph never mention it. | Add to §7.2 (`static/css/style.css` row) a task to migrate the shell chrome's hard-coded `rem` sizes onto the `--text-*` scale at `style.css:480-500`: `.nav-link`, `.brand`, `.theme-btn`, `.theme-menu button`, `.theme-group-label`, `.site-footer`, `.vitals-strip`. Where no scale step fits (0.65rem), state that a new step is an A1 request rather than a local literal. |
| 2C. Pedagogical depth | 2 | Largely out of A2's territory and §0 says so. Where the shell does serve `/learn`, it serves it well: `Section::Learn` (§4.2), F-08's diagnosis of the `self.section() == "wiki"` leftover (verified at `base.html:27` and `wiki.rs:118-120`), U-2 as the guard, and `/learn` plus a learn page in the §5.4 manual pass. **Gap:** the education surface's actual navigation — `wiki_page.html:4` `<aside class="wiki-sidebar" aria-label="Education wiki navigation">` wrapping a `<details class="wiki-nav">` disclosure and a bare `<nav>` at `:7` — appears nowhere in §3.1's surface inventory, §3.4's keyboard table, or §3.7's landmark table. | In §3.1 and §3.7, either claim the wiki sidebar as an A2 surface (it is chrome, it appears on 13 `/learn` URLs, and it is a `navigation` landmark subject to A2's naming contract) or add an explicit row to §0's out-of-scope table assigning it to a B-feature with the landmark-naming rule handed over. Do not leave it unowned. |
| 2D. Scannability and structure | 2 | Strong on the primary nav: §1.2 point 1 argues the nav *is* the site map for a deep-link arrival, §4.2 makes `NAV` a single `const` definition, U-1 proves every `NAV.href` resolves to a registered route, U-2/U-3 pin the active state, and §8 Q2 debates promoting `/status` with a real argument on both sides. **Gap:** §3.1 claims to enumerate every surface the shell owns, yet two of the site's four navigation surfaces are absent — the `/learn` sidebar above, and `index.html:11` `<nav class="hero-actions" aria-label="Quick navigation">`. For a spec whose stated job (§1.1) is orientation, that is a structural omission rather than a nit. | Extend §3.1's surface table with rows for the `/learn` sidebar `<nav>` and the home page `hero-actions` `<nav>`, each marked owned-or-delegated, and extend I-1 to assert that no page renders a `navigation` landmark whose `aria-label` ends in the word "navigation" (see 3D). |
| 2E. Restraint | 3 | §3.5 inventories every animation on the site and gates each one; §3.2 "Cues" declines haptics and sound; §3.4 declines single-key shortcuts on WCAG 2.1.4 grounds; §5.3 declines a browser harness with a maintenance argument; §8 Q1 declines a cookie-backed theme route; Q6 declines `<noscript>`. §4.7 imposes a hard 150-line JS ceiling with real teeth — "if the shell's JS crosses 150 lines, the identity claim in §1.2 is no longer true and the copy must change." §1.2 point 2 explicitly distinguishes the vitals strip from dashboard cosplay. | — |
| 2F. Theme integrity | 3 | §4.2 specifies CSS *mechanism only* and marks values as A1's. §5.4 requires a pass across all 23 themes with Lunarcore and Solarcore getting the full pass every time. Most importantly, §7.4 blocks commit 1 on A1 supplying the Solarcore token set "as a reusable declaration … or A1 restructures so a theme's tokens can be applied under two selectors **without duplication**" — the spec recognises that hand-copying a palette into a `@media` block is exactly the per-theme edit 2F forbids, and refuses to do it. `.vh` and `:root:not([data-theme])` are theme-agnostic. | — |

**Lens average:** 2.50
**Lens pass:** Yes

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JavaScript | 3 | The spec's spine. F-01 and F-03 are both independently confirmed: `base.html:30-36` ships the menu with a server-rendered `hidden` attribute and `main.js:55` is its only remover, so the button is focusable, named, and inert with JS off; and `grep -c prefers-color-scheme static/css/style.css` returns **0**, so a light-preference visitor with JS off gets bare `:root` = Lunarcore dark (`style.css:16-33`). The fallback is fully specified (§4.2's `:root:not([data-theme])` block under `prefers-color-scheme: light`, plus `:root:not([data-js]) .theme-select { display: none }`), given a flow (§3.2 B′), an error-state row (§3.6 E-05), a machine check (I-2, which strips scripts from the served bytes), a manual check (§5.4), and a budget ceiling (§4.7). §8 Q1/Q6 record the declined alternatives. Auto-fail rule 3 is not merely satisfied — the spec exists largely to close it. | — |
| 3B. Contrast and colour independence | 2 | Correctly delegates token contrast to A1's real audit, preserves the `✓` pseudo-element at `style.css:678` as the non-colour checked cue, and adds `aria-current="page"`. **Two problems.** (a) The spec's one original contrast contribution is wrong: §5.4 and §7.4 request "a focus-ring row (`--accent` on `--bg`, 3:1)" as "currently unaudited", but `generate_themes.py:143-155` already audits `accent` against `bg` at **4.5**, which is stricter than 1.4.11's 3:1. The pair that genuinely is unaudited is `--accent` against `--surface` — `style.css:685` sets `outline-offset: 2px`, so on `.theme-menu button` (menu background is `var(--surface)`, `:634`) and `.project-card:hover` the ring is drawn on surface, not bg. (b) F-04 correctly diagnoses that `style.css:699` gives hover and active the *identical* underline, then fixes it only for assistive tech via `aria-current`; §3.7's colour-independence table still records "currently indistinguishable from active" as a live state with no visual remedy. | (a) In §5.4 and §7.4, change the A1 audit request from `--accent` vs `--bg` to **`--accent` vs `--surface` at ≥ 3:1**, and note that accent-vs-bg is already covered at 4.5 by `generate_themes.py:143-155`. (b) Add a visual differentiator to §3.3/§3.7 so hover ≠ active for sighted users — e.g. active keeps the full-width 1.5px underline and hover uses a 1px underline at reduced opacity, or active gains a leading marker. Specify it as a size/weight change, not a colour change, so 2F still holds. |
| 3C. Keyboard and focus | 3 | §3.4's key table is complete and matches the APG button-menu pattern; the four proposed fixes each correspond to a confirmed defect — `<main tabindex="-1">` (`base.html:85` has no tabindex, F-14), skip-link legibility (`style.css:534-539` sets only `left: 1rem` on focus, no background/padding/z-index, F-02), Tab-close focus restore (`main.js:61` calls `close()` while focus is on a menu item, F-06), and the `focusedIndex() == -1` clamp (F-15's arithmetic checks out: `(-2 + 24) % 24 == 22`, the second-to-last item). DOM focus order in §3.7 is verified exact: `:17` → `:21` → `:24-27` → `:30` → `:85` → `:89`. No `tabindex > 0` anywhere. | Polish only: §3.2 step 3 and §3.4 introduce typeahead in one sentence with no buffer timeout, no same-character cycling rule, and no reset behaviour. Specify the APG defaults (≈500 ms buffer; repeated same character cycles through items starting with it) so the implementer is not inventing them. |
| 3D. Semantics and AT | 2 | The strongest AT analysis in the document. F-09 is correct — `vitals_strip.html:7` is a bare `<div>` carrying `aria-label="Server vitals"`, which ARIA does not permit on `role="generic"` — and F-10 is correct: `base.html:20-82` wraps the brand *and* the theme selector inside `<nav aria-label="Primary navigation">`, and the label double-announces. The per-element AT contract table is thorough and the one-`<h1>`-per-page claim is verified true (all 10 content templates have exactly one; `base.html` has zero). **Gap:** §3.7 presents its landmark table as the target contract while two live violations of that same contract sit outside it — `index.html:11` `<nav aria-label="Quick navigation">` and `wiki_page.html:4` `<aside aria-label="Education wiki navigation">` containing an unnamed `<nav>` at `:7`. Both reproduce exactly the "…navigation, navigation" double-announce F-10 exists to eliminate, and no proposed test would catch a third. | Add to §5.2 a test `landmark_names_follow_the_shell_contract`: for every HTML route, assert that no `aria-label` on a `<nav>` or `<aside>` matches `/navigation"?$/i`, and that each page renders at most one unnamed `<nav>`. Then add `index.html` and `wiki_page.html` label fixes to §7.2's modified-file table (or file them as cross-feature requests to B1/B4 in §7.4 with the exact target labels). |
| 3E. Motion and sensory safety | 3 | §3.5's inventory is complete and every entry verified: the `@media (prefers-reduced-motion: no-preference)` block at `style.css:710-726` carries the colour transitions, the underline sweep, `theme-pop`, and `cursor-blink`; the boot-line stagger sits in a second `no-preference` block with `steps(1, end)` and `both` fill-mode; the CRT scanline overlay is *removed* under `reduce` at `:461-470`. The arithmetic is right — the stagger totals 1.8s (< 5s, WCAG 2.2.2) and the blink is ~0.83 Hz (< 3 Hz, WCAG 2.3.1). §3.7's "Known residual risk" discloses the 2.2.2 exposure on the infinite blink instead of hiding it, and Q3 offers a one-token `animation-iteration-count` fix. No autoplay, no body-content animation. | — |
| 3F. Responsive and resilient | 2 | Resilience is excellent: §3.3's degraded-state table is accurate (`state.rs:240-253` returns zeros and never panics; the `{% match %}` at `vitals_strip.html:11-16` drops the `MEM` item *and* its separator), U-11 pins the no-global-state render, and "the shell has no empty state of its own: it always renders, on every route, including 500" is a real property. **Two gaps.** (a) The site has **three** breakpoints — `@media (max-width: 800px)` at `style.css:1404` (wiki layout and sidebar collapse) and `@media (max-width: 640px)` at `:1475` — and §3.4's responsive table documents only the 640px one. (b) Every late-file line citation in that table is wrong by ~31 lines (see Feasibility). | Add a `≤ 800px` row to §3.4's responsive table covering `.wiki-layout` collapsing to one column and `.wiki-sidebar` losing its sticky rail (`style.css:1404-1470`), and correct the `≤ 640px` citation from `1444-1461` to `1475-1502`. |

**Lens average:** 2.50
**Lens pass:** Yes
**Auto-fail triggered:** No

- **Rule 1 (unearned claims):** Pass. Every capability the spec introduces is labelled implemented / prototyped / planned / absent (§7.1), and §6.3 binds planned items to producing no user-visible copy until they land. The `~95 lines of JS` identity claim in §1.2 is verified exactly true (`main.js` 80 lines, `theme-init.js` 15).
- **Rule 2 (accessibility floor):** Pass. The spec removes no focus state (`style.css:685` is explicitly preserved), adds a non-colour cue rather than removing one, keeps every animation behind `prefers-reduced-motion`, and delegates AA contrast to A1's existing per-theme audit. The one residual (infinite brand blink, WCAG 2.2.2) is pre-existing, already gated by `prefers-reduced-motion`, and disclosed with a remedy in Q3 rather than concealed.
- **Rule 3 (no-JS floor):** Pass, emphatically. Navigation, content, error pages, and the vitals readout are all server-rendered with zero JS dependency (verified end to end in `base.html`, `router.rs`, `errors.rs`, `vitals_strip.html`). The only JS-dependent affordance — the theme selector — is an enhancement with a specified CSS fallback and is *removed from the page* rather than left dead when JS is unavailable.

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 3 | §1.2 point 2 states the impression outright and positions it against the field: "Every other candidate portfolio in the competitor set shows a screenshot of a dashboard; this one *is* the dashboard, and it costs zero bytes of JS. That is a claim that cannot be faked by copying a template." §4.7 then defends it operationally — the caching handoff to A3 correctly reframes a stale counter as "not a perf bug but a **claim-integrity bug**." F-12 is a sharp reviewer-path observation: with no `og:image`/`og:url`/`twitter:card` (verified absent from `base.html:4-14`), the site previews as a bare text row in "the exact channel a hiring manager receives it through." | — |
| 4B. Evidence over enthusiasm | 2 | §6.3's copy inventory classifies the vitals readout as "**Measured, not asserted** — read from the live process. The strongest kind of claim on the site", and §7.1's F-table is itself a disciplined record of defects with evidence per row. **Gap:** 4B asks the feature to surface *verification, failure, and recovery to the reader*. The shell surfaces operation (uptime/requests/memory/build) and recovery (themed 404/500 with live nav), but nothing about verification or failure reaches the page, and the spec never states that boundary or argues it is correct for chrome. | Add one paragraph to §1.2 or §4A-adjacent copy stating explicitly which of the three (verification / failure / recovery) the shell carries and which it delegates to B-features — e.g. that recovery is shell territory (E-01/E-02), operation is the vitals strip, and verification/failure live in post bodies and `/portfolio`. Without it the shell reads as claiming the whole 4B surface. |
| 4C. Original explanation | 2 | A2 authors no explanatory content and §0 scopes it out. Its real contribution is indirect but substantive: §4.3 Contract S-1 turns per-page `description()` into a compile-enforced, length-bounded, claim-audited artifact, which is the only shell-routed prose on each page. Beyond that the criterion is untouched. | — (out of A2's territory; no action required of this spec) |
| 4D. Depth of a real system | 3 | Everything the shell displays is read from a system genuinely operated: `Status::current()` → `AtomicU64` counter, `Instant::elapsed`, `/proc/self/status` behind a 5s cache (`state.rs:42`, `:169-186`), `build.rs`-stamped `BUILD_TS`, and a bind *classification* rather than an address (`state.rs:209-219`). §4.7's server-CPU row costs the render honestly; §4.7's caching requirement engages the real Caddy/Cloudflare deployment; §6.1 records the operational-intelligence trade of publishing a request counter and accepts it deliberately. | — |
| 4E. Reviewer paths | 2 | §2's eight user stories are unusually well-differentiated: hiring manager on a deep link, engineer peer, keyboard-only user, screen reader user, JS-disabled reader, mistyped-URL visitor, the operator, and a 360px/200%-zoom mobile reader. §1.2 point 1 models the resume→deep-link arrival concretely. **Gap:** criteria 4E names three reader types and the **self-directed learner** has no story — nobody in §2 arrives at `/learn` to study, which is also why the `/learn` sidebar went unnoticed in §3.1 (see 2C). | Add a user story to §2 for a self-directed learner arriving at `/learn/osi-model` from a search result, stating what the shell owes them (where they are in the curriculum, how to reach adjacent pages, how to get back to the index) — and reconcile it with the `/learn` sidebar ownership question raised in 2C. |

**Lens average:** 2.40
**Lens pass:** Yes

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 2 | `src/shell.rs` with `Section`, `NavItem`, and `NAV` is the right fix for F-08, and §4.1 justifies the new module rather than assuming it: "four separate concerns … are currently hand-duplicated across `base.html` and six handler files with nothing keeping them honest." The `Section` enum genuinely removes the stringly-typed `"wiki"` comparison (verified at `base.html:27` vs `wiki.rs:118-120`). **Two gaps.** (a) §4.2's code block re-declares `const BUILD_TS_EPOCH: &str = env!("BUILD_TS");` in `shell.rs` while `state.rs:34` already declares it — a second definition introduced inside the section arguing against duplication, with no guard named, which is precisely what 5A requires when duplication is deliberate. (b) The spec never engages the `SIDEBAR`/`WIKI_SLUGS` split that 5A names by name, despite proposing `tests/shell.rs` in the very test crate that split exists to keep decoupled. | (a) In §4.2, either have `shell::asset_version()` read a single `pub const BUILD_TS_EPOCH` re-exported from `state.rs`, or state in a comment why two `env!("BUILD_TS")` sites cannot drift (same compile-time var) and mark the duplication deliberate. (b) Add a sentence to §4.1 or §7.2 acknowledging the `SIDEBAR`/`WIKI_SLUGS` decoupling convention and stating whether `Section`/`NAV` follow it or deliberately break it — this is load-bearing for the test-placement fix below. |
| 5B. Drift guards | 1 | The guard *set* is the densest part of the spec (U-1, U-2, U-5, U-7, U-8, U-9, U-10, I-7), and F-16's diagnosis is correct — `generate_themes.py:256-282` emits the menu and has an internal `MENU_GROUPS`/`THEMES` guard at `:260-268`, but nothing compares its output to the shipped files. **However, three of the spec's own guards are red on arrival and none is flagged as such:** (1) **U-10** asserts the `base.html` `[data-mode]` list, `main.js` `MODES`, and `theme-init.js` `MODES` are "equal, **in order**, length 24" — verified false: the sets are equal but the orders diverge from index 5 (`base.html` runs …`dark, solarized, nord, gruvbox, crt`…; the JS arrays run …`dark, light, crt, amber, paper`…). The menu grouping deliberately reordered the buttons; U-10 as written would force that grouping to be undone. (2) **U-6** requires `50 <= len(description()) <= 160` — verified failing today on `releases.rs:40` (49 chars), `errors.rs:76` (38), `errors.rs:93` (42), *two of which are files A2 itself modifies*, plus five `content/**` summaries over 160 (up to 258 in `management-layer-first-network-migration.md`). (3) **U-7** fires on `content/pages/network-topologies.md` (see 1E). The spec explicitly flags the U-7 *ordering* trap in §7.4 but never notices that three guards fail on landing. | See Corrections 1–3 below. In addition, add a rule to §5.1 that every proposed guard must be stated as "green today" or "red today, lands with commit N" — the spec already does this for I-4 ("currently fails") and should do it for all of them. |
| 5C. No hidden coupling | 3 | F-17 is the sharpest analysis in the document and every element of it is verified: `pages.rs:158` asserts `html.contains("CompTIA")` inside `home_page_shows_concrete_work_without_strategy_narration`; `templates/index.html` contains **zero** occurrences of "CompTIA"; the assertion passes only through `base.html:6`/`:8` rendering `IndexTemplate::description()` (`pages.rs:44`). The spec then generalises correctly — the same shell coupling silently affects `"Proxmox"` (`:157`) and `"homelab"` (`:156`, `:214`), and every negative assertion is an implicit guard over all shell copy including 24 theme names. Its own tests obey the rule: U-7 asserts against `description()` directly, not against a rendered page. | — |
| 5D. Verification is stated | 2 | The five commands are named and verified to match `.github/workflows/ci.yml` exactly (`cargo fmt --all -- --check` → `cargo clippy --all-targets -- -D warnings` → `cargo test --all-targets` → `cargo build --release`), the §5.4 manual matrix is unusually concrete, and §7.2's nine-commit sequence is genuinely atomic with the auto-fail gate shipped first. **Two defects.** (a) §1.3's secondary success signal — "a keyboard-only user can go from page load to any nav destination in ≤ 3 Tab presses" — is false by the spec's own §3.4 tab-order table: skip link, brand, About is 3, so Portfolio is 4, Writing 5, Learn 6. A stated measurable signal that cannot measure true. (b) The verification block lists `python3 docs/themes/generate_themes.py --menu   # must match templates/base.html`, but the script only *prints* (`:293`); nothing compares, so that command can never fail. The comparison lives in U-9, which §8 Q7 leaves unresolved. | (a) Restate §1.3's secondary signal as "≤ 6 Tab presses to the furthest nav destination, with the skip link reachable on the first", or measure it as "≤ 2 Tab presses to leave the chrome entirely". (b) Either delete that line from the verification block or replace it with a command that exits non-zero on mismatch (e.g. `python3 docs/themes/generate_themes.py --menu \| diff - <(extracted block)`), and resolve Q7 before U-9 is written. |
| 5E. Documentation follows behavior | 3 | Names `docs/solarcore/SOLARCORE_SPEC.md` §6.4 and §7 as the sections that must be rewritten, with the correct reason (both describe things that never shipped — verified) and the correct owner (A1, per 2A). Names `README.md` conditionally. And correctly reports that `docs/agent-context/README.md` **does not exist** despite being referenced by the global `CLAUDE.md` index — verified: `docs/agent-context/` is absent from the repo entirely — while declining to create it as out of scope and naming the three durable constraints it should carry. | — |

**Lens average:** 2.20
**Lens pass:** Yes — avg ≥ 2.0, one criterion at 1, no 0s

---

## Feasibility Check

Read against `templates/base.html`, `templates/vitals_strip.html`, `templates/error_404.html`,
`templates/error_500.html`, `templates/wiki_page.html`, `src/router.rs`, `src/errors.rs`,
`src/state.rs`, `src/main.rs`, `src/handlers/*.rs`, `static/js/main.js`,
`static/js/theme-init.js`, `static/css/style.css`, `build.rs`, `Cargo.toml`, `Cargo.lock`,
`docs/themes/generate_themes.py`, `docs/solarcore/generate_brand.py`, `.github/workflows/ci.yml`.
Baseline `cargo test --all-targets` passes: 30 + 3 + 2 = 35 tests, 0 failures.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | `Section`, `NavItem`, `NAV`, `asset_version()` are fully specified in §4.2 and compile-plausible. `{% for item in crate::shell::NAV %}` follows the pattern already proven by `{% let vitals = crate::state::Status::current() %}` at `vitals_strip.html:6`. `BUILD_TS` is really stamped by `build.rs`. |
| API/interface changes are feasible with current architecture | ✗ | **Contract S-2 cannot do what §4.3 says it does.** `base.html` emits `<meta property="og:type" content="website">` at line **9** and `{% block head_extra %}` at line **14**. A `head_extra` override can only *append* a second `og:type` tag, never replace the first — and I-7 asks for the required meta set to appear once per route while I-8 asks blog/learn pages to declare `article`. These two tests are mutually unsatisfiable under the proposed mechanism. Separately, §7.2 adds `og:url` to `base.html` and I-7 asserts it on every route, but Contract S-1 defines only `title()`/`description()`/`section()` — nothing tells `base.html` the current path. |
| Views/screens fit current navigation pattern | ✓ | Header, nav, main, footer, vitals strip, 404 and 500 all exist as described; the proposed changes are narrowing and relabeling, not restructuring. The spec does not contradict the restored footer (§7.1 cites commit `f8553d5`, verified: "fix: restore the site footer") or the six `role="group"` theme wrappers (§3.3 and §7.1 both describe them correctly, including that `main.js` needed no change because roving focus reads `[data-mode]` in document order). |
| Dependencies are available and version-compatible | ✓ (with one caveat) | "New packages: none" holds. Verified in `Cargo.lock`: `axum 0.7.9`, `tower-http 0.5.2`, `askama 0.12.1` — matching `Cargo.toml:10-21` as the spec claims. `ServeDir::not_found_service` does exist in tower-http 0.5. Caveat: the §4.3 snippet `get(errors::fallback_404).into_service()` is not valid axum 0.7 — `into_service()` is on `HandlerWithoutStateExt`, not `MethodRouter`; write `errors::fallback_404.into_service()` with `use axum::handler::HandlerWithoutStateExt`. §4.6 already hedges this ("confirm the exact method name at implementation time"). |
| Platform/renderer requirements are realistic | ✓ | `matchMedia().addEventListener` (Safari ≥ 14), `clip-path: inset(50%)`, and `100svh`-after-`100vh` are all correctly characterised. The ES5 house style of `main.js` is real (`var`, `function`, no arrows) and the instruction to keep it is right for an untranspiled, unbundled file under `script-src 'self'` (verified at `security_headers.rs:41-50`). |
| Test strategy is executable with current infrastructure | ✗ | **`tests/shell.rs` cannot exist as specified.** `mg-server` is a **binary-only crate** — there is no `src/lib.rs`, and `src/main.rs:17-22` declares the modules privately. Integration tests under `tests/` cannot reach `crate::router::build` or `crate::shell::NAV`. That is exactly why the only existing file in `tests/` (`tests/wiki_pages.rs`) re-declares `WIKI_SLUGS` and reads files from disk, with the comment "duplicated here on purpose so the test crate stays decoupled from the bin", and why every router-level test in the repo lives in `#[cfg(test)]` modules inside `src/` (`errors.rs:171-193`, `handlers/status.rs:84-135`). I-1 through I-8 all require `oneshot` against the built router and must move into `src/`. |
| Performance budget is realistic for target hardware | ✓ | Every figure checks out to the byte: `base.html` 9,845 B, `vitals_strip.html` 881 B, inline brand SVG 2,624 B, theme menu block 4,972 B (the two together are 77% of the shell, as claimed), `main.js` 4,664 B / 80 lines, `theme-init.js` 960 B / 15 lines (5.6 KB, 95 lines), `style.css` 42,369 B ≈ 41 KB. The ≤ 11 KB rendered and ≤ 6.5 KB JS targets leave real headroom. |
| No undeclared dependency on unbuilt features | ✗ | **The `og:image` plan has no working generator.** §4.5 states `static/img/og-card.png` (1200×630) is "Generated from the existing `mark.svg` by `docs/solarcore/generate_brand.py` — no third-party art", and §4.5 also declares "Infrastructure: none". Verified: `generate_brand.py:231` writes **`og-card.svg`**, built from its own internal geometry rather than from `mark.svg`, and the script imports only `math`, `re`, `sys`, `os` — there is no rasteriser anywhere in the repo. Producing a PNG needs an undeclared tool (`rsvg-convert`, `cairosvg`, Inkscape, or a headless browser), and Slack/LinkedIn/Discord generally will not render an SVG `og:image`. I-7 asserts `og:image` on every route, so commit 6 cannot land until this is resolved. |

**Additional citation errors found (none change a score, but an implementer will trip on them):**

- `router.rs` citations are consistently off by one: routes are at **37-57** (spec says 38-58), `nest_service("/static", …)` is at **59** (spec says 60), `.fallback(errors::fallback_404)` is at **61** (spec says 62), the `/wiki` legacy routes end at **45** (spec says 41-46).
- Route counts are wrong: §3.1 says "all 13 routes" and I-1 says "all 11 HTML routes". `router.rs` registers **15** routes, of which **9** render the shell (`/`, `/about`, `/portfolio`, `/blog`, `/blog/:slug`, `/learn`, `/learn/:slug`, `/releases`, `/status`); `/wiki` and `/wiki/:slug` are 3xx redirects, `/status.json` is JSON, and three are `text/plain`. I-1's route list should be 9 HTML routes + 404 + forced 500 = 11 responses.
- `wiki.rs:106-108` is cited for `title()`; the real location is **110-112**. `wiki.rs:114-116` is cited in F-08 for `section()`; **114-116 is `description()`** and `section()` is at **118-120**.
- Late-file `style.css` citations are off by ~30 lines: the boot-line stagger block is at **1309-1319** (spec says 1279-1294 and 1279-1289) and the `≤ 640px` block is at **1475-1502** (spec says 1444-1461, 1447-1457, 1447-1450). F-03 calls it a "1,471-line stylesheet"; it is **1,502** lines. Early and mid-file citations (`16-33`, `449`, `461-470`, `480-500`, `497-499`, `510`, `513`, `534-539`, `545-597`, `619-678`, `685`, `699`, `702`, `710-726`, `732-738`, `790-835`) are all exact.
- §5.1 U-4 says "each of the 10 content templates **+ both error templates**", but the repo has exactly 10 non-shell templates *including* the two error templates (§3.7 states this correctly). Pick one count.
- §1.3 says "every route in `router.rs:38-58` renders complete, readable, navigable HTML" — `/robots.txt`, `/security.txt`, and `/status.json` do not render HTML.
- Every other verified claim held: `base.html` line numbers (2, 6, 8, 9, 11-14, 17, 19-27, 30, 36-38, 85, 89-97, 99, 100), all `errors.rs` citations (78-80, 95-97, 113-123, 117-121, 139-141, 160-169, 171-182, 184-193), all `state.rs` citations (8-14, 16-18, 34, 42, 107-112, 137, 169-186, 209-219, 239-254, 240-253, 262-272, 284-288, 332-338), all `status.rs` citations (40-42, 84-89, 97-100, 113-123, 125-135), `pages.rs` (10-12, 44, 52-57, 77, 81, 146-167, 156-158, 161, 204-223), `blog.rs` (64, 134-136), `releases.rs` (40, 42-44), `main.js` (12, 22, 25-40, 43, 45-48, 55-57, 60-65, 68-77), `theme-init.js` (2-4, 5-15, 6, 7-14), `vitals_strip.html` (6, 7, 8-18, 11-16, 18), `Cargo.toml:14`, `error_404.html:5-9`, `error_500.html:5-8`, "35 tests in the repo", "24 `[data-mode]` buttons", "23 themes", and the absence of `docs/agent-context/`.

**Feasibility verdict:** Feasible with caveats
**Caveats:** Four items must be corrected before implementation begins — the `tests/shell.rs` placement (bin-only crate), the `head_extra` `og:type` override mechanism, the missing `og:url` data source, and the `og-card.png` generator. None is architecturally hard; all four are specification errors rather than impossibilities.

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 2.83 | 25% | 0.708 |
| 2. Design & Craft Excellence | 2.50 | 25% | 0.625 |
| 3. Accessibility & Progressive Enhancement | 2.50 | 20% | 0.500 |
| 4. Competitive Depth & Differentiation | 2.40 | 20% | 0.480 |
| 5. Accuracy & Maintainability | 2.20 | 10% | 0.220 |
| **Composite** | | | **2.53** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 — 2.53
- [x] All lens averages ≥ 2.0 — 2.83 / 2.50 / 2.50 / 2.40 / 2.20
- [x] No criterion scores 0
- [x] No more than two criteria at 1 per lens — one 1 total (5B)
- [x] All auto-fail rules pass — rules 1, 2, and 3 all clear
- [x] Feasibility ≠ Infeasible — Feasible with caveats

**All conditions met:** Yes → **PASS**

---

## Corrections Required Before Implementation

The spec passes, so this is not a remediation brief. These are the defects an
implementing agent would otherwise discover as red tests or compiler errors. Items 1–7
must be fixed in the spec before commit 1; items 8–12 are quality; items 13–15 are polish.

### Must fix before an implementer starts

1. **§5.1 U-10 asserts a false fact.** Change "All three lists are equal, in order,
   length 24" to **set** equality plus length 24, and add a separate assertion that
   `main.js` `MODES` and `theme-init.js` `MODES` are identical **in order** to each
   other (they are). The `base.html` `[data-mode]` order intentionally differs because
   the menu is grouped — `base.html` runs `system, lunarcore, solarcore, dark,
   solarized, nord, gruvbox, crt, amber, matrix, teletext, gameboy, c64, nes,
   synthwave, vaporwave, cyberpunk, tron, light, paper, dawn, cloud, blueprint, sepia`
   while both JS arrays run `system, lunarcore, solarcore, dark, light, crt, amber,
   paper, dawn, cloud, gameboy, c64, teletext, nes, matrix, solarized, nord, gruvbox,
   synthwave, vaporwave, cyberpunk, tron, blueprint, sepia`. As written U-10 would
   force the grouping to be undone.
2. **§5.1 U-6 is red today on files A2 owns.** `50 <= len <= 160` fails on
   `releases.rs:40` (49), `errors.rs:76` (38) and `errors.rs:93` (42), and on five
   `content/**` frontmatter summaries above 160 (max 258, in
   `content/posts/management-layer-first-network-migration.md`). Either (a) shorten the
   two error-page descriptions and widen the upper bound to 200 for post/page
   summaries, or (b) scope U-6 to the eight static template descriptions only and file
   the frontmatter length rule as a separate content guard. Whichever is chosen, state
   in §5.1 which commit turns it green — the spec already does this for I-4.
3. **§5.1 U-7 is red today.** `"enterprise"` appears in
   `content/pages/network-topologies.md:4`'s summary, which reaches `<meta>` through
   `WikiPageTemplate::description()`. See the 1E remediation for the split.
4. **§7.2 test placement is impossible.** `mg-server` has no `src/lib.rs`;
   `src/main.rs:17-22` declares all modules privately, so `tests/shell.rs` cannot reach
   `router::build` or `shell::NAV`. Move I-1 through I-8 into a `#[cfg(test)] mod
   tests` inside `src/` — `src/shell.rs::tests` is the natural home and matches the
   existing convention at `errors.rs:171-193` and `handlers/status.rs:84-135`. Delete
   the `tests/shell.rs` row from §7.2.
5. **§4.3 Contract S-2 cannot override `og:type`.** `base.html` emits `og:type` at line
   9 and `{% block head_extra %}` at line 14; appending a second meta tag is not an
   override. Replace the mechanism with either a fourth metadata-contract method
   `fn og_type(&self) -> &str { "website" }` on Contract S-1 (overridden by
   `BlogPostTemplate` and `WikiPageTemplate`), or a dedicated
   `{% block og_type %}website{% endblock %}` at `base.html:9`. Then I-7 and I-8 become
   mutually satisfiable.
6. **§7.2 adds `og:url` with no data source.** Add a `fn canonical_path(&self) -> &str`
   (or a `path: String` field) to Contract S-1 in §4.3, and state the site-origin
   constant it is joined to. Without it I-7's `og:url` assertion cannot be implemented.
7. **§4.5 `og-card.png` has no generator.** `docs/solarcore/generate_brand.py:231`
   writes `og-card.svg` from its own internal geometry — not from `mark.svg` — and the
   script imports only `math`, `re`, `sys`, `os`. Either declare the rasteriser
   (`rsvg-convert` or `cairosvg`) as a documented dev-time tool and correct the §4.5
   "Infrastructure: none" line, or change the deliverable to a checked-in PNG produced
   once by hand. Note that SVG `og:image` is not reliably rendered by Slack, LinkedIn,
   or Discord — the exact channels F-12 is about.

### Should fix for quality

8. Correct the citation errors listed in the Feasibility section — particularly the
   `router.rs` off-by-one set, `wiki.rs:106-108` → `110-112`, `wiki.rs:114-116` →
   `118-120` for `section()`, the ~30-line drift in the late `style.css` citations, and
   the route counts ("13 routes" → 15 registered / 9 rendering the shell; I-1's "11
   HTML routes" → 9 routes + 404 + 500).
9. Redirect the A1 focus-ring audit request from `--accent` vs `--bg` (already audited
   at 4.5 by `generate_themes.py:143-155`) to `--accent` vs `--surface`, which is the
   pair `outline-offset: 2px` actually produces over `.theme-menu` and `.project-card`.
10. Give F-04 a visual remedy, not only `aria-current` — hover and active are currently
    identical to a sighted user and §3.7's own table records that as an unresolved state.
11. Claim or delegate the two navigation surfaces missing from §3.1/§3.7:
    `wiki_page.html:4-7` and `index.html:11`. Add the landmark-naming test described in
    the 3D remediation so the convention A2 establishes is enforced site-wide.
12. Migrate the shell chrome's ad-hoc `rem` sizes onto the `--text-*` scale
    (`style.css:480-500`) as described in the 2B remediation.

### Consider for excellence

13. Specify the typeahead's buffer timeout and same-character cycling behaviour (§3.2
    step 3, §3.4) rather than leaving APG defaults implicit.
14. Fix §1.3's "≤ 3 Tab presses to any nav destination" — it is 6 to Learn by the
    spec's own §3.4 order table — and either remove or make executable the
    `generate_themes.py --menu` line in the verification block, which currently cannot
    fail because the script only prints.
15. Add the self-directed-learner user story to §2 (4E), and state in §1.2 or §4A which
    of verification / failure / recovery the shell carries versus delegates (4B).
