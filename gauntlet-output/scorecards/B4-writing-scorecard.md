# Scorecard: Writing

**Feature ID:** B4-writing
**Spec file:** gauntlet-output/specs/B4-writing.md
**Reviewer agent:** verify-agent-B4 (Claude Opus 4.8, blind review)
**Date:** 2026-08-09
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This is a disciplined, source-accurate spec: nearly every claim cites
a real line that checks out, and its central strength is exemplary state honesty —
every capability is tagged implemented/planned/gated/absent and no planned surface
(feed, empty state, `og:type`, anchor links) reads as shipped. The most critical
gap is a mismatch between the new summary-length drift guard (`50 ≤ len ≤ 160`) and
the remediation it schedules: three of four live posts exceed 160 characters, yet
§7.2 shortens only one, so the guard as specified fails CI on introduction. That is
a quality fix, not a blocking one — no criterion scores 0, every lens clears 2.0,
and no auto-fail rule fires.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 3 | §1.2 names the network-migration post as the model (recovery sequence, quorum/`/etc/pve` circular dependency, DNS failure); §6.3 audits every post as first-person ops work with reproducible evidence. Verified: `hosting-machinageist-dev.md` carries real `dig`/`curl` output; `management-layer…md` is a 13 KB end-to-end outage. The publish pipeline gives every post a path to why/start/target/tools/evidence/broke/verify/unknown. | — |
| 1B. State honesty | 3 | §0 mandates implemented/prototyped/planned/gated/absent tags; §7.1 is an exhaustive inventory; §6.3 explicitly forbids planned items reading as shipped. Feed, empty state, `og:type`, anchors, tag pages, metadata-load all clearly labeled planned/absent. | — |
| 1C. Publication gates | 3 | §6.4 restates the GeistScope gate (full pipeline + human & AI operation + sanitized evidence from an authorized engagement); §7.1 lists `content/drafts/geistscope-retrospective.md` as gated. Verified: that draft (and siblings) exist under `content/drafts/`, outside the routed `content/posts/`. | — |
| 1D. Copy currency | 3 | §7.1 "Copy currency" and §6.3 confirm B4's own copy carries no cert claim and does **not** carry the stale "CompTIA stack" line (correctly attributed to `pages.rs`, owned by B1/B2). Verified: `blog_list.html:6-9` intro and `blog.rs:67-69` `description()` are clean. | — |
| 1E. Role posture | 3 | §6.3 audits domain language: no cert/offensive/"secured/production/SRE/enterprise" framing; intro says "a bit of defensive security"; the Security post is scoped as "header hardening on a personal site, not a claim the application is 'secured'." Verified in `security-headers-on-machinageist-dev.md` frontmatter/intro. | — |
| 1F. Test-encoded policy | 3 | §5.1 preserves the existing grouping test (`blog.rs:178-196`, verified) and adds two drift guards; no anti-overclaim guard is weakened or deleted. B4 does not touch `project.rs`/`pages.rs`/`lab.rs`. | — |

**Lens average:** 3.00
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation | 3 | §0 defers all tokens/type-scale/measure to A1 and treats shipped `style.css` as canonical (cites real line numbers as reality); §3.3 says "reconcile in the A1 sweep, not per-page." Does not re-assert the stale SOLARCORE_SPEC. | — |
| 2B. Typographic craft | 3 | §3.3/§3.7: prose caps at `--measure` (72ch, verified `style.css:499`), `pre`/tables keep full column and scroll; article restores true heading order (`--text-xl` h2 > `--text-lg` h3, verified `style.css:1090-1103`), reversing the listing small-caps convention; hierarchy carried by weight/case/size. | — |
| 2C. Pedagogical depth | 2 | Posts teach method not just fix (user story 3; §1.2), but the criterion's core (glossaries/study tools/ground-up scaffolding) is B5/`/learn`, explicitly out of B4's scope (§0). B4 hosts pedagogical writing rather than building pedagogical structure. | Out of B4's charter; no action required for B4. Any teaching aids (TOC, cross-links) would land via 2D. |
| 2D. Scannability & structure | 2 | Index is pillar-grouped (§3.3, verified). But within-post navigation is absent: no heading `id`s/anchors (§7.1, verified `style.css:1157-1158` "Headings carry no ids today"), no cross-linking, no feed yet. Spec honestly flags the anchor gap "vs the Arch Wiki/MDN benchmark." | Implement the `pulldown-cmark` heading-id pass (Q2) so posts deep-link; consider cross-linking related posts. |
| 2E. Restraint | 3 | §3.1 "No modal, sheet, popover, drawer… entirely page-based"; §3.3 uses divider `<ul>` rows, not cards (verified `.post-item` border-bottom, not card); tags are inert pills, deliberately not links (§7.1); §3.5 one hover transition only; Q3 leans defer tag pages for restraint. | — |
| 2F. Theme integrity | 3 | §3.7/§6.4: all colors are theme tokens under A1's `--check`; zero per-theme edits introduced; §5.4 checks Lunarcore/Solarcore/solarized/CRT(scanlines over `pre`)/Paper(serif measure). | — |

**Lens average:** 2.67
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JavaScript | 3 | §3.7/§6.4: index, article, feed fully server-rendered; §5.2 test `blog_index_needs_no_javascript` strips `<script>` and asserts links/structure remain. Verified: only `theme-init.js`/`main.js` load (`base.html:12,99`), pure enhancement. **Auto-fail rule 3 not triggered.** | — |
| 3B. Contrast & color independence | 2 | §3.7: colors are theme tokens under A1's audit; hierarchy via weight/case/size; in-prose links keep the underline (verified `.post-content a` overrides color only, global `a` never sets `text-decoration:none`, `style.css:534-537`). Contrast of `.post-date`/`.post-summary` `--text-faint` at 0.78/0.85rem is deferred to A1's active 14-failure remediation (not asserted passing); index title links use `text-decoration:none` (verified `style.css:1005`), leaning on position over affordance (spec flags this honestly). | Confirm `--text-faint` pairs clear AA at 0.78rem across all 23 themes in the A1 sweep; consider an underline/hover affordance on index title links. |
| 3C. Keyboard & focus | 3 | §3.4/§3.7: every interactive element is a native `<a>`; visible 2px `--accent` focus ring at 2px offset (verified `:focus-visible`, `style.css:710`); DOM focus order; no custom key handling; ring 3:1 audit deferred to A2. | — |
| 3D. Semantics & assistive tech | 2 | §3.7: real heading outline (one `h1` → `h2` per pillar → article `h2`/`h3`); titles as links inside `<li>`, not headings (correct). Spec catches a genuine defect: each pillar `<section aria-label>` (verified `blog_list.html:12`) promotes every group to a duplicate region landmark; target is `aria-labelledby`/drop the role — still pending in §7.2. | Replace `aria-label` on pillar `<section>` with `aria-labelledby` referencing an `id` on the `h2`, or drop the section wrapper's region role. |
| 3E. Motion & sensory safety | 3 | §3.5: only `.post-item` hover motion; no autoplay/body animation/flashing. Verified: transitions sit inside `@media (prefers-reduced-motion: no-preference)` (`style.css:735`) — the exact posture criterion 3E requires. | Fix §3.5's wording: the mechanism is a `no-preference` **enable-gate**, not a "reduce block that disables it (738-744)"; the citation inverts the actual gate type (compliance is correct, description is wrong). |
| 3F. Responsive & resilient | 2 | §3.4 fluid column, `pre`/tables scroll rather than force page horizontal scroll (verified `overflow-x:auto`, `style.css:1124-1130,1167-1176`); §3.6 honest 404/500; §5.4 checks 320px→wide and 200% zoom. But the designed empty state is **planned** — verified the template loop today renders nothing, leaving a dangling `h1` (the "accidental" empty state the criterion warns against). | Implement the planned empty-state branch in `blog_list.html` so zero posts renders one designed line, not `h1`+intro over blank space. |

**Lens average:** 2.33
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s
**Auto-fail triggered:** No — rule 3 (no-JS) satisfied; rule 2 (a11y floor) not violated (focus present, no hue-only state, motion gated, no new failing contrast pairs introduced by B4)

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 3 | §1.2 states the impression (real ops work shown in full vs. green-screenshot homelab portfolios and cert-track candidates); §1.3 success signal: cold deep-link reader reads the whole post, sees its pillar, gets back to the index without JS. | — |
| 4B. Evidence over enthusiasm | 3 | §1.2 centers the network-migration outage post (recovery, circular dependency, DNS failure) as the model; §6.3 cites real `dig`/`curl` output. Verified the post exists and is an end-to-end outage. | — |
| 4C. Original explanation | 3 | §1.2 frames original explanation as the differentiator; user story 3 (learner learns the debugging method, not the fix). Posts are first-person operations writing, not restated tutorials. | — |
| 4D. Depth of a real system | 3 | §1.2/§7.1: posts connect to a genuinely operated system (Proxmox cluster, DNS, Cloudflare Tunnel, Caddy, corosync on this actual server), verified across the four post frontmatters/bodies. | — |
| 4E. Reviewer paths | 3 | §2 enumerates hiring manager, peer engineer (feed), self-directed learner, keyboard/SR reader, no-JS reader, and author; §7.1 ties the planned feed to the peer-engineer path. | — |

**Lens average:** 3.00
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 3 | §4.1: `POSTS_DIR` `pub(crate)`, one definition shared with B1 (verified `blog.rs:30`); §4.4: `PILLARS` (`blog.rs:35-40`) is the one definition, and the planned guard closes its soft coupling to free-text `category:` strings; slug derived once from filename. | — |
| 5B. Drift guards | 2 | §5.1 adds `every_post_category_is_a_known_pillar_or_none` and `every_post_summary_fits_meta_description`; malformed posts fail the whole index loudly (§3.6) — directly answers the criteria's `generate_themes.py` reference. **But** the summary guard (`50 ≤ len ≤ 160`) fails on **three** live posts (measured: hosting 220, network-migration 227, security-headers 217; solarpunk 156 passes), while §4.2 calls only "the longest… ~220 chars" a failure and §7.2 schedules shortening only `management-layer…md`. As specified, the guard fails CI on introduction. | §4.2 should say three of four exceed 160; §7.2 must shorten `hosting-machinageist-dev.md`, `security-headers-on-machinageist-dev.md`, **and** `management-layer-first-network-migration.md` summaries, or the new guard reds CI immediately. |
| 5C. No hidden coupling | 3 | §5.2 tests assert against what they name: `blog_index_lists_present_pillars` asserts pillar headings + post links in body; `blog_post_renders_full_article` asserts title + `<h2`/`<table` markup; `blog_index_needs_no_javascript` strips scripts then asserts links; `article_declares_og_type_article` is named for the head tag it checks. Avoids the criteria's `contains("CompTIA")`-via-meta trap. | — |
| 5D. Verification is stated | 3 | §5.4 names the exact four CI commands: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo build --release`. | — |
| 5E. Documentation follows behavior | 3 | §7.2 "Docs that must follow behavior": `README.md:50-62` "lists only three posts and omits the network-migration post — update the count/listing." Verified: README's project-structure block lists hosting/security-headers/solarpunk and omits `management-layer-first-network-migration.md`. | — |

**Lens average:** 2.80
**Lens pass:** Yes — avg ≥ 2.0, one 1? no (zero 1s), no 0s

---

## Feasibility Check

Read the actual source files referenced in the spec before filling this table.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | `BlogPost`, `Frontmatter`, `PostGroup`, and `SiteError::{PostNotFound,MissingFrontmatter,FrontmatterParse,DateParse,InvalidPath,Io}` all verified present (`post.rs`, `blog.rs`, `errors.rs`). Planned `load_all_meta`/`blog::feed` clearly specified. |
| API/interface changes feasible with current architecture | ✓ | `/feed.xml` = one route + handler + Askama template, trivially feasible. `og:type=article` needs A2 to make `base.html:9` conditional — see caveats. |
| Views/screens fit current navigation pattern | ✓ | All page-based; both templates `{% extends "base.html" %}`; feed is non-shell XML. Verified. |
| Dependencies are available and version-compatible | ✓ | `pulldown-cmark` w/ `Options::all()`, `gray_matter` YAML, `chrono`, `askama` all in use and verified. Feed hand-rolled — no new crate. |
| Platform/renderer requirements are realistic | ✓ | Plain HTML+CSS, Askama compile-time templates, CSP `default-src 'self'`-safe. |
| Test strategy is executable with current infrastructure | ✓ | `tower::ServiceExt::oneshot` round-trip pattern already used (`errors.rs`, `status.rs:114`); unit tests in `#[cfg(test)]`. `feed_is_valid…` and `article_declares_og_type…` depend on unbuilt surfaces (spec labels them planned/after-§7.2). |
| Performance budget is realistic for target hardware | ✓ | Four posts, kilobytes/request; wasted `content_html` on the list path acknowledged, metadata-load optimization correctly deferred. |
| No undeclared dependency on unbuilt features | ✓ | A1 (`--measure-narrow`, group-heading) and A2 (`Section` enum, `og:type`, `head_extra`/feed advertise) dependencies are all declared in §7.4 with pass status; feed/`og:type`/empty-state labeled planned. |

**Feasibility verdict:** Feasible with caveats
**Caveats:**
1. `og:type=article` cannot be delivered by B4 alone: `base.html:9` hardcodes `content="website"` with no override. `base.html:14` does expose `{% block head_extra %}`, but adding a second `og:type` there yields duplicate/ambiguous meta. A2 must make the existing tag conditional. The spec declares this dependency correctly (§4.3, §7.2, §7.4; "B4 must not edit `base.html`").
2. The new summary-length guard fails CI against three current posts while §7.2 schedules only one summary edit (see 5B).
3. `--measure-narrow` is cited near `style.css:499` in the §0 reference table but does not yet exist there (only `--measure: 72ch`); it is an A1 deliverable (correctly listed as a blocking dependency in §7.4).

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 3.00 | 25% | 0.750 |
| 2. Design & Craft Excellence | 2.67 | 25% | 0.667 |
| 3. Accessibility & Progressive Enhancement | 2.33 | 20% | 0.467 |
| 4. Competitive Depth & Differentiation | 3.00 | 20% | 0.600 |
| 5. Accuracy & Maintainability | 2.80 | 10% | 0.280 |
| **Composite** | | | **2.76** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 (2.76)
- [x] All lens averages ≥ 2.0 (3.00 / 2.67 / 2.33 / 3.00 / 2.80)
- [x] No criterion scores 0
- [x] No more than two criteria at 1 per lens (zero 1s in any lens)
- [x] All auto-fail rules pass (rule 1 unearned claims: clean; rule 2 a11y floor: clean; rule 3 no-JS: clean)
- [x] Feasibility ≠ Infeasible (Feasible with caveats)

**All conditions met:** Yes → PASS

---

## Remediation Brief (quality — not required to pass)

### Priority 2 — Should fix for quality
1. **Fix the summary-guard/remediation mismatch (5B).** Update §4.2 to state three of four summaries exceed 160 chars (hosting 220, network-migration 227, security-headers 217), and expand §7.2's "Modified files" to shorten all three `.md` summaries — otherwise the new `every_post_summary_fits_meta_description` guard reds CI the moment it lands.
2. **Ship the designed empty state (3F).** Move the §3.3 empty-state branch from planned to implemented in `blog_list.html` so zero posts renders one designed line rather than an accidental dangling `h1`.
3. **Resolve the pillar-region landmark over-labeling (3D).** Apply the §7.2 change — `aria-labelledby` on each pillar `<section>` (with an `id` on its `h2`) or drop the region role — so a screen reader's landmark list is not flooded with one region per pillar.

### Priority 3 — Consider for excellence
1. **Correct §3.5's motion description (3E).** The gate is `@media (prefers-reduced-motion: no-preference)` (`style.css:735`) that *enables* motion, not a "reduce block that disables it (738-744)"; fix the wording and line cite so the spec's own accuracy matches Lens 5's standard.
2. **Add within-post scannability (2D/4C).** Implement the `pulldown-cmark` heading-id pass (Q2) for deep-linkable, no-JS `#` anchors; consider cross-linking related posts toward the Arch Wiki/MDN benchmark.
3. **Add an affordance to index title links (3B).** They currently rely on position (`text-decoration:none`, `style.css:1005`); a hover/underline cue would strengthen the click affordance without leaning on layout alone.
