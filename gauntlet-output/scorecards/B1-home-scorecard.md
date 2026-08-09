# Scorecard: Home

**Feature ID:** B1
**Spec file:** gauntlet-output/specs/B1-home.md
**Reviewer agent:** blind-verify-agent (Claude Opus 4.8)
**Date:** 2026-08-08
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This is a disciplined, source-accurate spec that nails the two
criteria it was practically written to answer — the stale "CompTIA study" meta
copy (Lens 1D) and the hidden-coupling `assert!(html.contains("CompTIA"))` test
(Lens 5C), both of which criteria.md names by hand as the failure modes to catch.
It scopes cleanly (reads, never writes, B4's post model; defers tokens/contrast to
A1 and the shell to A2), preserves the anti-overclaim guards rather than weakening
them, and correctly designs the empty-state resilience. The most material residual
gap is that the home page is a direct consumer of the small-`--text-faint` pairings
A1 flags as failing AA — correctly delegated to A1 with a "no new failing pairing"
invariant, so it does not trip the accessibility auto-fail, but it means B1's own
contrast is not yet met and remains dependency-gated.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 2 | B1 proposes **no** new resume-facing artifact; §0 scopes the post model to B4 (read-only) and portfolio artifacts to B3. Its job (§1.1) is to route to evidence, and §1.2.1–.2 makes routing-to-evidence the explicit differentiator. Nothing here proposes publishing an unevidenced artifact. | — (criterion is largely N/A for an orientation page; correctly handled by scoping) |
| 1B. State honesty | 3 | §7.1 marks every capability **implemented** explicitly; §3.7A marks no-JS "✅ Shipped"; the two proposed edits (§4.2, §5.1) are clearly deltas, never presented as shipped. §1.3 separates current state (`contains("CompTIA")` test lies) from target. | — |
| 1C. Publication gates | 2 | The "Lately" list is WIP on a permitted progress surface, not portfolio; §6.3 explicitly refuses to build an activity feed and cites `docs/plans/deferred-dashboard-notes.md:7` (verified — that line forbids adding a homepage section until Jeff owns the concept). No GeistScope content on `/`. | — (gate only tangentially engaged; the one relevant slice is handled correctly) |
| 1D. Copy currency | 3 | §4.2 Change 2 + §6.3 correctly flag `pages.rs:44` "CompTIA study" as stale against the 2026-08-02 RHCSA→CCNA→Security+ re-lock, verify the README no-cert rule (`README.md:14-16`, confirmed), and propose a claim-safe fix that **drops** the cert reference rather than swapping in "RHCSA" without a voucher (Q2). Also flags the time-anchored lede/Lately rot (§6.3, Q4). | — |
| 1E. Role posture | 3 | §1.2.3 mandates leading with owned scope; the proposed description leads with "Homelab, networking, and Linux notes … a Proxmox cluster, the operations work it generates." §5.1 preserves the anti-overclaim test forbidding "security engineer"/"offensive security"/"red-team" (verified `pages.rs:164-166`). | — |
| 1F. Test-encoded policy | 3 | Correctly distinguishes the **positive** stale-copy assertion `contains("CompTIA")` (`pages.rs:158`, which it removes) from the **anti-overclaim guards** (`pages.rs:160-166`, which it explicitly preserves — §5.1, §6.3) and *strengthens* discipline by adding `home_description_carries_no_retired_claims`. The removal decision is recorded (§5.1, §7.2), not silent. | — |

**Lens average:** 2.67
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation | 2 | 2A is blocking **for the design-system feature (A1)**, not B1. §0 and §3.7E defer the token/theme roster/contrast matrix to A1; B1 touches no theme and never re-asserts the stale SOLARCORE spec. | — (N/A to B1; correctly avoided) |
| 2B. Typographic craft | 2 | Reading measure is disciplined (`.hero-lede` max-width 55ch, verified `style.css:935-940`); heading hierarchy is clean (one h1, three h2, no skips, §3.7B). But the home font sizes are ad-hoc literals (1.75rem/0.9rem/0.875rem/0.78rem/0.85rem) that the spec pushes to A1's tokenization (§7.2.3) rather than systematizing here. | Confirm A1's Layer-3 tokenization actually absorbs the home-class size literals so the type scale becomes systematic (tracked as the A1 dependency; no B1-owned change) |
| 2C. Pedagogical depth | 2 | B1 authors no education content — it points to `/learn` with a plain descriptive pointer (§3.3, `index.html:52-56`), not a bullet-dump. | — (N/A; correctly routes to the wiki) |
| 2D. Scannability and structure | 3 | The page *is* the switchboard: hero-actions nav + two section pointers + post teaser, mapped to reader paths (§1.2.2) and made machine-checkable (§5.1 asserts `/portfolio`, `/blog`, `/learn` present). Four clearly-headed stacked sections; home correctly not a nav item so nothing registers in SIDEBAR/WIKI_SLUGS. | — |
| 2E. Restraint | 3 | §3.5 adds no home-body motion and argues a landing page animating its copy would violate 2E/3E; §6.3 refuses a dashboard/feed; list architecture (`.about-list`/`.post-list`), no cards, no fake metrics. Honors the quiet copy-voice rule. | — |
| 2F. Theme integrity | 3 | B1 proposes **no** CSS change (§7.2.3 "No B1-owned change required"); home classes consume tokens (`--text-muted`/`--text-faint`/`--accent`), size stays literal (non-theme), so nothing requires per-theme edits. | — |

**Lens average:** 2.50
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

criteria.md defines Lens 3 as six criteria (3A–3F); scored as such. The spec's own
§3.7A–G sub-labels are mapped onto these.

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JavaScript | 3 | §3.7A: entire page SSR, every interactive element a native `<a href>` (verified — `index.html` has zero client behaviour); §5.2 HI-2 makes the no-JS floor machine-checkable. Auto-fail rule 3 satisfied. | — |
| 3B. Contrast and color independence | 2 | §3.7D: no state signalled by hue; focus ring global. But §3.7E concedes B1 is a **direct consumer** of the failing small-`--text-faint` pairings (`.post-summary` 0.85rem, `.post-date` 0.78rem, verified `style.css:1011-1022`) among A1's 14 flagged failures. Correctly delegated to A1 (the token owner) with a "introduce no new small-faint pairing" invariant. | Contrast on B1's surface is **not yet met** and is gated on A1 landing the `--text-faint` ≥4.5:1 remediation (§7.4). B1 must re-verify `/` in Tier-1 themes after A1 lands (§5.4) and add no new small-faint pairing. |
| 3C. Keyboard and focus | 3 | §3.7G: native anchors in DOM order, no `tabindex`, no trap, visible focus ring. Verified all links native; the global `:focus-visible` rule exists (though cited line is off — see Feasibility). | — |
| 3D. Semantics and assistive technology | 3 | §3.7C diagnoses a **real** current defect: the three `<section aria-label="…">` (`index.html:20,33,50`) duplicate their own `<h2>` text, so each becomes a `region` landmark announced twice; two concrete fixes given (`aria-labelledby`, or drop the label). Clean heading outline (§3.7B); "→ " noted as read-as-content, not `aria-hidden` (§3.7D). | Implement one of the two §3.7C options in `templates/index.html` (recommended; the spec marks it non-blocking) |
| 3E. Motion and sensory safety | 3 | §3.5/§6.4: no home-body motion; all motion is the shell's, behind `prefers-reduced-motion: no-preference` (verified `style.css:735`). Auto-fail rule 2 (motion) N/A. | — |
| 3F. Responsive and resilient | 3 | §3.4: single column reflows 320→wide, `.hero-actions` `flex-wrap:wrap` (verified `style.css:948`); §3.6 HE-1/HE-2 designed empty state (omit section **and its `<hr>`**) pinned by test `pages.rs:186-202`. §5.4 lists 320/640/1280 + 200% zoom checks. | — |

**Lens average:** 2.83
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s
**Auto-fail triggered:** No. Rule 3 (no-JS) satisfied (§3.7A); rule 2 (a11y floor) not tripped — the small-faint contrast defect is a token-level issue owned by A1, explicitly delegated with a no-new-failure invariant and a stated blocking dependency (§3.7E, §7.4), and B1 removes/adds no focus state, signals no state by hue, and ignores no reduced-motion preference. This was the closest call; it does not trigger because the spec addresses the defect via correct ownership rather than ignoring it.

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 3 | §1.1 states the target impression directly ("whose site is this, what do they operate, where do I go next"); §1.2.1 names the differentiator explicitly: a grounded lede naming a real operated system vs the field's enthusiasm/aspiration, "tell in one screen that this person runs something real." | — |
| 4B. Evidence over enthusiasm | 2 | Home teases real posts (the network-migration outage post exists — verified `content/posts/management-layer-first-network-migration.md`) and routes to `/blog`/`/portfolio`; §1.2.2 leans on the shell vitals strip as proof-of-operation. But the actual verification/failure/recovery content lives in B4's posts, not on `/`. | — (home's role is to surface/route to the evidence, which it does; the demonstration itself is B4's) |
| 4C. Original explanation | 2 | B1 authors no educational material; it points to `/learn` without restating or bullet-dumping content. | — (N/A; correctly routes) |
| 4D. Depth of a real system | 3 | Hero lede + "Lately" name concrete operated work — three-node Proxmox cluster on a managed switch, internal DNS + subnet/VLAN map, self-hosting behind Caddy + Cloudflare Tunnel (`index.html:23-25`, verified); §6.3 confirms these against owned hardware and shipped posts. | — |
| 4E. Reviewer paths | 3 | §1.2.2 maps all three readers to routes: hiring manager → Portfolio + one-line what; engineer peer → Writing + vitals strip; learner → Learn wiki. Made machine-checkable in §5.1 (hero-nav contains all three routes). §2 user stories cover each path plus edge/a11y. | — |

**Lens average:** 2.60
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 3 | §4.2 Change 1 (Q1) identifies "machinageist" duplicated across the `name` field, `title()`, header brand, and footer (verified: literal at `pages.rs:41,60`, `base.html:21,91`), proposes a shared site-name constant as the SoT-correct move, and honestly names "keep it" as the harmless minimal option — decision deferred to Q1/A2 with the cross-cut named. | — |
| 5B. Drift guards | 2 | Adds a genuine loud-failing guard: `home_description_carries_no_retired_claims` asserting cert terms are absent from `description()` (§5.1). But the time-anchored lede ("Right now…") and static "Lately" bullets have **no** loud guard — the spec's own recommendation is re-phrasing or a manual copy-review checklist (§6.3, Q4), which is the silent-drift mode it can't fully solve without over-engineering (correctly refused). | Adopt Q4 option (a) — re-phrase the lede/Lately to timeless statements of what the lab *is* — so the copy has no recency assertion to rot, since no automated guard is proposed |
| 5C. No hidden coupling | 3 | Exact match to the criterion's named example: §5.1 explains `assert!(html.contains("CompTIA"))` (`pages.rs:158`) reads as a body claim but passes only via the `<meta description>` (verified — "CompTIA" appears 0× in `index.html`, only via `base.html:6`), removes it, and replaces it with a test that names its surface (`description()`) directly. | — |
| 5D. Verification is stated | 2 | §1.3 names `cargo test --all-targets` and §5.1–5.2 name the specific tests, but the spec does not enumerate the full CI quartet from criteria 5D (`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo build --release`) despite editing Rust (the `description()` string + tests). | Add the fmt/clippy/build commands to the verification statement (§1.3/§5), since the copy and test edits are Rust changes that CI gates |
| 5E. Documentation follows behavior | 2 | §6.3 checks the copy edit against `README.md:14-16` (already consistent with the no-cert rule) and flags Q4's copy-review checklist as the maintenance mechanism; but the spec does not crisply name a long-lived doc to update *in this change* — arguably because none is strictly required for a copy/test/aria edit. | If the description wording lands, state whether `README.md` / PUBLIC_FACE wording needs a matching touch, or record that it is already consistent |

**Lens average:** 2.40
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, no 0s

---

## Feasibility Check

Read `templates/index.html`, `src/handlers/pages.rs`, `src/router.rs`,
`src/models/post.rs`, `templates/base.html`, `static/css/style.css`, `README.md`,
`content/posts/`, and `docs/plans/deferred-dashboard-notes.md`.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | `IndexTemplate { name, posts }` (`pages.rs:30-36`) and `BlogPost { slug,title,date,summary,tags,category,content_html }` (`post.rs:53-62`) match the spec's reproductions exactly. |
| API/interface changes feasible | ✓ | Only a `description()` string edit + test edits; no signature change (the `section() -> Section` enum is A2's, correctly deferred). |
| Views/screens fit navigation pattern | ✓ | `/` → `pages::home` (`router.rs:37`); home is not a nav item — `base.html` header has no `/` link, so nothing highlights on `/` as the spec claims. |
| Dependencies available/version-compatible | ✓ | No new packages; askama + existing `BlogPost` only. |
| Platform/renderer requirements realistic | ✓ | Home body uses only flexbox + `::before` (verified `style.css:942-955`); no `:has()`/newer CSS in home rules. |
| Test strategy executable | ✓ | Three render tests exist at `pages.rs:146-202` (exact sub-ranges confirmed); the proposed `description()` string test is trivial. Integration tests are A2's; the `oneshot` pattern does exist (`errors.rs:175`, `status.rs:71`). |
| Performance budget realistic | ✓ | `load_all` reads the dir and parses every post then `truncate(3)` (verified `post.rs:117-135`, `pages.rs:56-57`) — 4 posts today; the "parse all, keep 3" inefficiency is real and correctly flagged Q3. |
| No undeclared dependency on unbuilt features | ✓ | Depends on A1 (contrast/tokens) and A2 (`section()` enum, integration iteration) — both **declared** blocking deps (§7.4). All routed targets `/portfolio`, `/blog`, `/learn` exist (`router.rs:39-41`). |

**Feasibility verdict:** Feasible with caveats
**Caveats:** Three minor line-number citation drifts — underlying claims all true, only the `path:line` is off:
1. §3.7G / §7.2 cite the global `:focus-visible` rule at `style.css:685`; it is actually at **`style.css:710`** (685 sits inside the `.theme-menu button` block). The rule exists and applies globally.
2. §3.6 HE-4 cites "`summary` is a required frontmatter field (`post.rs:39,46-47`)"; `summary` is at **`post.rs:40`** (and `:58`). Line 39 is the `date` field and lines 46-47 are `category` (which is `Option` — optional, the opposite). `summary` *is* required (no `serde(default)`), so the claim holds; the refs are wrong.
3. §5.2 cites the `oneshot` integration pattern at `status.rs:113-123`; the actual usage is at **`status.rs:71`** (the paired `errors.rs:171-182` bracket is correct, ~line 175). This is A2-owned test territory.

Non-spec note: `README.md:50-62` lists only 3 files under `content/posts/`, but the directory holds **4** (`hosting-…`, `management-layer-first-network-migration`, `security-headers-…`, `solarpunk-…`). The spec's "four posts today" claim (§2.3 branch, §3.2, §5.4) is **correct against the filesystem**; the README is the stale artifact, not the spec.

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 2.67 | 25% | 0.667 |
| 2. Design & Craft Excellence | 2.50 | 25% | 0.625 |
| 3. Accessibility & Progressive Enhancement | 2.83 | 20% | 0.567 |
| 4. Competitive Depth & Differentiation | 2.60 | 20% | 0.520 |
| 5. Accuracy & Maintainability | 2.40 | 10% | 0.240 |
| **Composite** | | | **2.62** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 — 2.62
- [x] All lens averages ≥ 2.0 — 2.67 / 2.50 / 2.83 / 2.60 / 2.40
- [x] No criterion scores 0
- [x] No more than two criteria at 1 per lens — zero 1s in every lens
- [x] All auto-fail rules pass — no unearned claim (spec removes a stale one); no-JS floor met; a11y floor not tripped (contrast delegated to A1 with no-new-failure invariant)
- [x] Feasibility ≠ Infeasible — Feasible with caveats

**All conditions met:** Yes → PASS

---

## Remediation Brief

Verdict is PASS; no Priority-1 (must-fix-to-pass) items. The following would raise
quality and are worth doing when the feature is implemented.

### Priority 1 — Must fix to pass
None — all pass conditions met.

### Priority 2 — Should fix for quality
1. **(3B / A1 dependency)** Do not implement B1's copy/test/aria changes as
   "contrast-complete" until A1 lands the `--text-faint` ≥4.5:1 remediation; then
   re-run the §5.4 Tier-1-theme contrast check on `/` and confirm B1 introduced no
   new small-faint pairing (`.post-summary` 0.85rem, `.post-date` 0.78rem).
2. **(5B, Q4)** Prefer re-phrasing the time-anchored hero lede ("Right now I'm
   building…") and the static "Lately" bullets to timeless statements of what the
   lab *is*, so the recency assertions have nothing to rot — no automated guard is
   proposed for this surface.
3. **(5D)** Extend §1.3's verification statement to name the full CI quartet
   (`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test --all-targets`, `cargo build --release`), since the description and
   test edits are Rust changes CI gates.

### Priority 3 — Consider for excellence
1. **(3D)** Resolve the region-label redundancy in `templates/index.html`: switch
   the three `<section aria-label>` (`:20,33,50`) to `aria-labelledby` on their
   `<h2>` (preferred if the regions are wanted as landmarks) or drop the labels.
2. **(5A, Q1)** Decide whether B1 or A2 consolidates the "machinageist" literal
   into a single shared site-name constant.
3. **Fix the three line-number citation drifts** (see Feasibility caveats:
   `style.css:685`→710, `post.rs:39,46-47`→40, `status.rs:113-123`→71) so a future
   implementer following the refs lands on the right code.
4. Note that `README.md`'s `content/posts/` listing is stale (3 files listed, 4 on
   disk) — outside B1's scope, but the discrepancy is worth flagging to the owner.
