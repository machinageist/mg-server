# Scorecard: Ops and Observability

**Feature ID:** ops-and-observability
**Spec file:** gauntlet-output/specs/A3-ops-and-observability.md
**Reviewer agent:** Spec Gauntlet verification agent (blind review)
**Date:** 2026-08-07
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This spec's dominant quality is that its empirical claims survive independent
re-verification. I reproduced its headline finding on the wire (a throttled `429` carries
zero security headers, no `Content-Type`, no `Retry-After`), reproduced the blank
`/static/<missing>` 404 at `content-length: 0`, reproduced the silent access log
(~80 requests, zero request lines at the default filter), reproduced the `bind`
misreport, and independently recomputed all 23 themes' WCAG ratios — matching the
spec's numbers to two decimals. Of roughly seventy file-and-line citations I checked,
three were wrong, and one of those three turned out to be correct against the spec's
base commit and shifted by an intervening CSS commit. The most critical gap is not in
the analysis but in the delta: §4.7's Phase 1 `SetResponseHeaderLayer` needs
tower-http's `set-header` feature, which is **not** enabled, while §4.5 asserts the
current feature set is sufficient and "Added: none"; and §4.2's `BindMode::description`
rewrite silently breaks the existing passing test
`state::tests::bind_description_comes_from_the_resolved_listener_address`, which §7.2
never lists as edited.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 3 | §7.5 maps each proposed public artifact to the evidence standard and gates the one that cannot fill it. **Verified:** `docs/public-portfolio-structure.md:80-92` does contain exactly thirteen bullets, so the "all thirteen fields" claim is accurate. The 502/`203/EXEC` report is held back pending real journald output or a labeled reproduction — which matches the published post's own refusal at `content/posts/hosting-machinageist-dev.md:89-90` ("I am not going to reconstruct log timestamps from memory here"), verified verbatim. §7.5's public/private rule ("may name a mechanism, a command, and a unit name; may not name a path, host, IP, or identifier") is a usable test. | §7.5 asserts the gap-analysis post has "all thirteen fillable today" without enumerating them. Enumerate the thirteen for that artifact so a writer can execute it without re-deriving. |
| 1B. State honesty | 3 | §0 defines implemented / prototyped / planned / gated / absent and applies it throughout. Per-IP limiting is `gated` with the gate written down in §4.3 (loopback bind → socket peer is always Caddy; a forwarded header is attacker-settable without a trusted-hop config). `AppState::hits()` is correctly labeled `prototyped` — **verified** at `src/state.rs:91-94`, `#[allow(dead_code)]`. §7.1's "Absent (and not proposed)" list is explicit. §1.3 states its own primary success signal is **false today** rather than asserting it. | §6.3 audits `src/state.rs:91`'s `/stats` comment for claim posture but misses `src/middleware/rate_limit.rs:17-20` ("Red team context… Tools like hydra and ffuf") and `security_headers.rs:13` ("blue team / red team") — in a file §7.2 already opens for a header-comment rewrite. Add both to the audit. |
| 1C. Publication gates | 3 | §6.4 states the GeistScope gate is N/A and gives the reason (`/status` shows process facts only, no tool claims). **Verified:** no A3 surface touches GeistScope. Separately, §3.1 and §7.5 respect the dashboard gate — **verified verbatim** against `docs/plans/deferred-dashboard-notes.md:29-34` (exactly six revisit questions) and its prohibition on "a dashboard route, navigation item, homepage section, data model, or public claim". | — |
| 1D. Copy currency | 3 | §6.3 audits eleven user-visible strings plus one code comment and finds two real defects. **Both verified:** `templates/status.html:29` says "RSS readout requires Linux" while `src/state.rs:262-272` returns `None` on any read/parse failure *including on Linux*; and `templates/status.html:44-46`'s retention paragraph becomes materially incomplete once access logging lands. §6.3 flags the superseded four-CompTIA spine at `IMPROVEMENT_PLAN.md:15` and `:41-47` (**verified verbatim**) rather than propagating it, and routes it to Q7. | The audit claims to cover "every user-visible string this feature owns" but omits the six `<dt>` labels, four remaining glosses (`status.html:18,21,33,36`), the `robots.txt` header comment, and the 404/500 boot-log copy (`error_404.html:5-9`). Either complete the table or narrow the claim. |
| 1E. Role posture | 3 | §6.3's forbidden-word list is **verified verbatim** against `IMPROVEMENT_PLAN.md:57-66` and `:409-417`. The framing is owned-scope: §1.2 names the threat model as "background internet noise", §6.3 as "one Rust process on a mini-PC in a house, behind a tunnel, with no alerting and no tested restore". §6.3 declines to upgrade the published safe claim, quoting `content/posts/security-headers-on-machinageist-dev.md:92-94` — **verified verbatim**. | — |
| 1F. Test-encoded policy | 3 | §6.4 and §7.2 keep every existing anti-leak test and add strengthening ones (T8 allowlist key-set guard, T9 broader leak assertions on both surfaces, T15 stack-level header proof). **Verified against the hard constraint:** `state::tests::status_snapshot_has_version_and_no_secrets` (`src/state.rs:333-338`) currently passes, and §4.2's proposed `BindMode::description` change *strengthens* it — `"loopback (IPv4)"` discloses strictly less than the current `"loopback (127.0.0.1)"`, and none of the five target strings contains `0.0.0.0`. Nothing is weakened or deleted. §3.7 explicitly instructs "Keep that test" for `src/errors.rs:160-169`. | — |

**Lens average:** 3.00
**Lens pass:** Yes — avg ≥ 2.0, zero 1s, zero 0s

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation | 3 | A3 is not the design-system feature, and it does not re-assert the stale spec anywhere. It treats the shipped site as ground truth throughout — every claim is measured against live CSS, live templates, and a running binary. §7.6 requires that the `--text-faint` token change land inside A1's `SOLARCORE_SPEC.md` rewrite rather than "silently in CSS", which is the correct direction under the 2026-08-07 resolution. The real-data vitals strip (named in 2A as a deliberate improvement) is preserved and strengthened. | — |
| 2B. Typographic craft | 2 | §3.3 documents the 700px status column and §3.4 requires no horizontal body scroll at 320px / 200% zoom with `rem`-based type. Correctly defers visual ownership to A1/A2 per §3.1. But the spec touches `.status-note` copy and cites its `0.8rem` size (**verified** `style.css:875-879`) without saying whether `0.8rem` sits on the type scale introduced in commit `3f96165` ("add a type scale and vertical rhythm, cap the reading measure"). It contributes nothing systematic to scale or rhythm. | In §3.3, state whether `.status-note`'s `0.8rem` and `.vitals-strip`'s `0.75rem` are on the site's type scale or are ad-hoc values, and hand the reconciliation to A1 if they are off-scale. |
| 2C. Pedagogical depth | 2 | A3 owns no `/learn` page, glossary, or study tool, so this criterion is mostly out of scope. It does specify two public posts. §7.5's outline for "What This Server Does Not Tell Me" is content-level ("journald is the only log and its retention is systemd's default; there is no alerting, so discovery is 'I looked'…") with a genuinely good pedagogical hook: "the verification is 'here is the command that shows you nothing is there.'" But it stays an outline — no structure, no build-from-the-ground-up sequence, no reader model. | If §7.5's posts are to be graded as teaching artifacts, add a section outline and name the concept each section builds before jargon (journald retention, what an access log is and is not, what "monitoring" would actually require). Otherwise state explicitly that post authorship is out of A3's scope and only the gating rule is owned here. |
| 2D. Scannability and structure | 2 | §3.1's surface inventory is a clean table with explicit ownership columns; §3.6's error table and §7.1's findings list are scannable. Cross-links are stated: `/status` → `/status.json`, footer strip → `/status`. §3.1 correctly states no new navigable screen is introduced, so no `SIDEBAR` / `WIKI_SLUGS` registration is required. | §7.2 proposes new posts under `content/posts/` but never states the `category` frontmatter value. **Verified:** `src/handlers/blog.rs:32-58` groups the blog index by a post's `category` into named pillars and dumps unmatched posts into "Other writing". Name the pillar for each proposed post so it lands where it is meant to. |
| 2E. Restraint | 3 | §6.4: "No dashboard, no gauges, no sparklines, no badge; the a11y fix is a copy edit rather than new markup; `429` stays plain text." §3.3 argues the plain-text `429` as a stated tradeoff (rendering ~10 KB of themed HTML into the flood path would burn the CPU the limiter exists to protect). §3.5 budgets animation to the 404/500 chrome and to nothing else. §7.5 explicitly refuses an uptime badge, a status-history graph, and "a personal site that displays a 99.9% uptime figure it cannot measure has spent its credibility on decoration." This is a direct, unhedged answer to "no dashboard cosplay, no fake metrics". | — |
| 2F. Theme integrity | 3 | §6.4: "This feature adds no per-theme rule." The one per-theme change proposed is a `--text-faint` **token value**, which 2F explicitly permits as a genuine palette concern. The spec measured all 23 themes rather than sampling, and T17 makes the property enforceable across the whole roster. **Verified:** 23 theme blocks exist in `static/css/style.css`; A3 adds no theme-scoped selector. | — |

**Lens average:** 2.50
**Lens pass:** Yes

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JavaScript | 3 | §3.7 and §2's no-JS user story. **Independently verified:** `curl` (a client with no JS engine) returns the complete `/status` readout, a populated footer strip, `/status.json`, `security.txt`, `robots.txt`, and the 404 body. Every surface is server-rendered via Askama; `templates/vitals_strip.html` carries no script. Site JS totals 95 lines (`main.js` 80 + `theme-init.js` 15) — **counted, exact** — theme selection only. Nothing in this feature reads or writes it. **Auto-fail rule 3 does not trigger.** | — |
| 3B. Contrast and color independence | 3 | §3.7 reports a measured failure instead of papering over it. **I recomputed WCAG 2.1 ratios for all 23 theme blocks independently: every ratio in the spec's table matches to two decimals** (c64 3.27, solarized 3.15, gameboy 3.43, cloud 4.28, blueprint 4.41, lunarcore 4.47, nes 4.47 on `--surface`; c64 3.93, gameboy 3.97, solarized 3.64 on `--bg`; `--text-muted` clean on `--bg` in all 23 and failing only solarized at 4.13 on `--surface`). §3.7 states a binding 4.5:1 requirement across all 23 themes and T17 enforces it in CI, so the **target** UI passes AA — **auto-fail rule 2 does not trigger.** Color independence is verified: `style.css:870-873` gives `.status-readout dt` both `var(--accent)` and `font-weight: 700`, and the MEM unavailable state is the literal words "not available". | **Count error:** the spec says "**6 of 23**" for `--text-faint` on `--surface` in §3.7, §5.1 T17, §7.1 F6, and derives "reduces failures from 6 to 1" — but it lists seven themes and the true count is **7**. Fix the count everywhere; the interim then reduces 7 → 1, not 6 → 1. **Sequencing contradiction:** §3.7 offers `--text-muted` as an interim that still fails solarized at 4.13, while §5.1 T17 says the test "must be introduced *with* the palette fix, in the same commit, or CI goes red." State explicitly that the interim ships *without* T17, that it is not a valid final state, and that solarized-on-`--surface` remains open until A1's palette fix lands with T17. |
| 3C. Keyboard and focus | 3 | §3.5 finds a real defect and owns it ("One defect, and it is mine"). **Verified:** `templates/error_404.html:9` is `.boot-line-5`, and `style.css:1319` gives it `animation-delay: 1.8s` with `animation: boot-line-in 0.2s steps(1,end) both` at `:1313` — so a keyboard user tabbing immediately onto the 404 focuses a link at `opacity: 0` for up to 1.8s. The proposed one-line fix (`.error-page a:focus-visible { animation: none; opacity: 1; }`) is correct: dropping the animation drops `fill-mode: both`, restoring the base opacity. §3.4 confirms two links total, DOM order, no interception, no custom widget. | — |
| 3D. Semantics and assistive technology | 3 | §3.7 correctly identifies that `aria-label` on a bare `<div>` with no role is dropped by most AT. **Verified:** `templates/vitals_strip.html:7` is exactly `<div class="vitals-strip" aria-label="Server vitals">`, and the `·` separators at `:9,:13,:17` are correctly `aria-hidden="true"`. The terse-label fix is a noun-leading copy rewrite, and the spec argues *against* `<abbr title>` (inconsistent announcement, unreachable on touch) and against inventing a `.visually-hidden` utility — the restrained choice. Page-title fix is real: **verified** `src/errors.rs:73` returns bare `"404"`. `<dl>/<dt>/<dd>` is the correct structure and the outline is one `<h1>` deep. | §3.7 leaves the strip's mechanism open (`role="group"` **or** a semantic `<p>`/`<ul>`). Pick one so A2 does not have to make an a11y judgment call — `role="group"` + the existing `aria-label` is the minimal change. Also correct the citation `src/errors.rs:92` for the 500 title: `"500"` is at **line 90**; line 92 is `description()`. |
| 3E. Motion and sensory safety | 3 | §3.5's construction analysis is correct and I verified it: the entire staged reveal sits inside `@media (prefers-reduced-motion: no-preference)` (now `style.css:1310`), the keyframe is `from { opacity: 0 } to { opacity: 1 }` (`:1322-1325`) with `animation-fill-mode: both` in the shorthand (`:1313`), and the source comment (`:1306-1309`) states exactly the consequence the spec quotes — "if animations never run the page is simply fully visible." No `opacity: 0` is stranded in a base rule. Delays 0.15 / 0.55 / 1.0 / 1.35 / 1.8s match `:1315-1319` exactly. §3.5 confirms no animation anywhere else in the feature; §3.7 confirms no autoplay, no flashing, no polling. | Bookkeeping only, and **not a spec error**: §3.5's `style.css:1279-1290` / `:1275-1278` and §5.4's `:1279` were correct against the spec's base commit `854e443`. Commit `5e98092` ("style: give Markdown tables real cell structure") then inserted exactly 31 lines at `style.css:1112`, shifting everything below it. Re-anchor these three citations to `1306-1309`, `1310-1320`, and `1310`. |
| 3F. Responsive and resilient | 3 | §3.4 requires no horizontal body scroll at 320px / 200% zoom and reasons from the actual CSS: `.status-readout` is `grid-template-columns: max-content 1fr` with a `<dt>` column at most six characters (**verified** `style.css:858-868`), and `.vitals-strip` sets `flex-wrap: wrap` (**verified** `:826`). §3.3 and §3.6 do exactly what 3F asks: the MEM `None` branch is named as the *designed* empty state, while `/static/<missing>` is named as an **accidental** one and fixed. **Verified accidental:** `curl -sS -D- http://…/static/nope.css` returns `404` with `content-length: 0` and a zero-byte body. §5.4 lists the manual matrix including MEM present/absent and fresh-restart/long-uptime. | — |

**Lens average:** 3.00
**Lens pass:** Yes
**Auto-fail triggered:** No — rule 2 does not trigger (the spec's target state binds 4.5:1 across all 23 themes and enforces it via T17; the failure it reports is pre-existing and is fixed, not inherited). Rule 3 does not trigger (every surface verified reachable by `curl`). Rule 1 does not trigger (§6.3 labels every proposed capability and §1.3 declares its own primary claim currently false).

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 3 | §1.2 states the intended impression precisely and makes it falsifiable: "It is one of the few things on a personal site that can be *falsified by the reader* — refresh it and the request counter moves, or the claim was a lie." §2's hiring-manager story sharpens it: real moving numbers "rather than a screenshot of somebody else's dashboard." §1.2 also names the anti-trophy framing — a readout that resets to `UP 00:00:00` after a restart "is an admission, not a trophy." | — |
| 4B. Evidence over enthusiasm | 3 | The spec's strongest lens. F1 is a failure the author found in his own published claim, and §7.6 turns it into the artifact: update the security-headers post with "the `429` bypassed the header layer, here is the ordering fix", with before/after `curl`. §7.5's flagship is "What This Server Does Not Tell Me" — an absence inventory, explicitly "the post the competitor set does not write, because absence is not flattering." §7.5 refuses to publish the 502 incident without real logs. **I reproduced F1, F2, F3, and F14 on a running binary**, so the failures being surfaced are genuine, not rhetorical. | — |
| 4C. Original explanation | 2 | The spec's *own* reasoning is explanatory at a high level — the layer-ordering analysis in §4.4, the per-IP gate argument in §4.3 ("it would key a bucket on an attacker-chosen string"), and the §3.3 tradeoff for a plain-text `429`. But 4C grades the educational material the feature *ships*, and what ships is an outline (§7.5 item 2) plus a paragraph (item 3). No explanation is actually drafted or structured. | Draft or outline the explanatory spine of "What This Server Does Not Tell Me" — at minimum, name the five sections and the one concept each teaches. The §4.4 layer-ordering analysis is already publishable-quality material; say whether it becomes reader-facing content or stays internal. |
| 4D. Depth of a real system | 3 | Everything is anchored to the operated system, not a tutorial: Debian VM on Proxmox, Caddy, Cloudflare Tunnel, systemd/journald, the real `203/EXEC` incident, `MG_BIND_ADDR`, the loopback bind. §4.3's per-IP gate turns on a genuine property of *this* deployment (loopback bind ⇒ socket peer is always Caddy) — **verified** at `src/main.rs:41-45`. §4.6 names Linux-vs-macOS as the live compatibility axis, grounded in `src/state.rs:16-17`. §7.5's public/private boundary is drawn from the real runbook contents. | — |
| 4E. Reviewer paths | 2 | §2 supplies seven user stories: operator on a phone, hiring manager, engineer peer who does not trust portfolio sites, security researcher, screen reader user, no-JS visitor, badly-behaved crawler. The engineer-peer story is well aimed ("so that the writing is checkable rather than assertable"). | The criterion names three readers: hiring manager, engineer peer, **and self-directed learner**. The learner has no story and no path. §7.5's gap-analysis post is the obvious learner surface — add a story for the reader who arrives to learn what observability on a one-process site actually costs, and say what that reader needs from `/status` that the other two do not. |

**Lens average:** 2.60
**Lens pass:** Yes

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 3 | §4.2 names `uptime_secs` / `uptime` as deliberate duplication and shows SSoT is preserved because `uptime` is *computed from* `uptime_secs` via `format_uptime` (**verified** `src/state.rs:275-281`). §4.3 names `Status` as the allowlist with T8 as the guard. §4.1 explains the binary-crate constraint and why `tests/wiki_pages.rs` duplicates `WIKI_SLUGS` on purpose — **verified**: `Cargo.toml` declares no `[lib]`, `src/main.rs:16-21` declares modules privately, and `tests/wiki_pages.rs:11-13` states the decoupling rationale verbatim. §7.6 requires `src/router.rs`'s header comment to move with the layers. §4.7's `asset_version()` replaces four hand-typed strings with one derived value — **verified**: `templates/base.html` hard-codes `v=20260719-spectrum` in exactly **4** places (lines 11, 12, 13, 99). | §4.7 says "this spec provides an `asset_version()` value from `BUILD_TS`" without naming the mechanism. Askama has no global context and `base.html` is extended by every page. Name the pattern — the same one `templates/vitals_strip.html:6` already uses (`{% let av = crate::state::asset_version() %}`) — so A2 is not left to invent it. Also name the guard that keeps all four sites using it. |
| 5B. Drift guards | 3 | Directly answers the `generate_themes.py` reference. T8 makes the privacy allowlist fail loudly on any added field; T11 turns the "renew yearly" comment at `src/handlers/well_known.rs:7-8` into a red CI run 30 days early (**verified**: `SECURITY_TXT_EXPIRES` is hard-coded at `:17` with nothing guarding it); T13 guards the robots group list (**verified**: 18 named agents, all with `Disallow: /static/`); T17 makes `style.css:13`'s "23 themes, all WCAG-AA validated" comment enforceable rather than aspirational; T4 encodes the layer-order invariant so a future reorder of `vitals` outside `rate_limit` fails. §5.1's stated limit on T13 ("no test can know about a crawler that does not exist yet") is honest rather than overclaimed. | — |
| 5C. No hidden coupling | 2 | The named criterion is handled excellently. §4.4 enumerates three concrete costs of the process-global (a template reaching into a global, one-publisher-per-process `init_global`, two independent snapshots in one document) — all **verified** at `templates/vitals_strip.html:6`, `src/state.rs:140-157`, and `src/handlers/status.rs:137-139`. T15 independently rediscovers the criterion's own reference failure mode: "the existing `internal_error_page_leaks_nothing` calls `into_response()` directly and therefore proves nothing about the middleware stack" — **verified** at `src/errors.rs:187`. §5.1 T9 even flags the deliberate tension that `templates/status.html:26` prints the literal `/proc/self/status`. **But the spec introduces two couplings it does not name** (see remediation). | (a) §4.2's `BindMode::description` rewrite **breaks the existing passing test** `state::tests::bind_description_comes_from_the_resolved_listener_address` (`src/state.rs:348-370`), which asserts `"loopback (127.0.0.1)"` and `"loopback (::1)"`. §7.2's `src/state.rs` row says only "add T5–T7" and §6.4 claims "none is weakened or deleted." Add the test edit to §7.2 explicitly and state that T7 supersedes those two assertions. (b) §4.7 Phase 1 uses `SetResponseHeaderLayer`, which lives in `tower_http::set_header` behind `#[cfg(feature = "set-header")]` (**verified** in tower-http 0.5.2 `src/lib.rs:232-233`); `Cargo.toml:14` enables only `["fs", "trace"]`. Add `"set-header"` to the §7.2 `Cargo.toml` row and correct §4.5's feature-sufficiency claim. |
| 5D. Verification is stated | 2 | §5.2 gives a complete, runnable out-of-process script (start on an alternate loopback, header check, 404 check, static-miss byte count, 70-request flood, `429` inspection, allowlist read, `security.txt` read, `POST` → 405) plus the live-site `curl \| grep` and `jq .build` commands. §5.1 names 17 tests with setup, assertion, and the edge each covers. §5.3 justifies the absence of a browser harness rather than hand-waving it. `cargo test --all-targets` and `cargo build --release` are both named in prose. | **The spec never writes out `cargo fmt --all -- --check` or `cargo clippy --all-targets -- -D warnings`.** §5.3 refers to "four cargo commands (`.github/workflows/ci.yml:26-36`)" by citation only. Criterion 5D requires the exact commands be named. Add the four-command block to the head of §5. (**Verified** those are exactly the four gates at `.github/workflows/ci.yml:26-36`, with `RUSTFLAGS: -D warnings` set globally at `:11`.) |
| 5E. Documentation follows behavior | 3 | §7.6 is a seven-row table of document / trigger / update, tied to specific slices. It routes the `--text-faint` change through `docs/solarcore/SOLARCORE_SPEC.md` under A1's ownership rather than letting it land silently in CSS, and it requires `src/router.rs`'s layer-order rationale to move with the layers. **Verified** that the README does need the limiter correction: `README.md:138-150`'s Security section lists the headers and never describes the rate limiter, whose only mention is a one-line dependency-table entry at `:42`. | Correct one factual error: F15 and §7.6 claim the README project tree omits `handlers/releases.rs` — it is present at `README.md:72`. The other three omissions (`state.rs`, `middleware/vitals.rs`, `handlers/status.rs`) are real. |

**Lens average:** 2.60
**Lens pass:** Yes

---

## Feasibility Check

I read `src/router.rs`, `src/state.rs`, `src/middleware/security_headers.rs`,
`src/middleware/rate_limit.rs`, `src/middleware/vitals.rs`, `src/handlers/status.rs`,
`src/handlers/well_known.rs`, `src/errors.rs`, `src/main.rs`, `Cargo.toml`,
`templates/status.html`, `templates/vitals_strip.html`, `templates/base.html`,
`templates/error_404.html`, `static/css/style.css`, `tests/wiki_pages.rs`, and
`.github/workflows/ci.yml`; recomputed all 23 themes' contrast ratios; ran
`cargo test --all-targets` (30 unit + 2 integration, all green); and probed a running
debug binary on an alternate loopback address.

**Base-commit note.** The spec was authored against `854e443`. The only source change
since then is `static/css/style.css` (+31 lines inserted at `:1112` by `5e98092`), plus
new content pages and one `src/handlers/wiki.rs` slug. `README.md` and every theme token
block are byte-identical to the spec's base — so the `releases.rs` and "6 of 23" errors
below are genuine spec errors, while the `style.css:1279` family is rebase drift.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | `Status` verified at `src/state.rs:226-235` with exactly the seven fields §4.2 lists, in the order the live JSON returns them. `BindMode` verified at `:188-220`. The `parse_vm_rss` extraction is mechanical and preserves the `None`-on-any-failure contract. |
| API/interface changes are feasible with current architecture | ✓ | The layer reorder is one `.layer()` call moved in `src/router.rs`. **The spec's axum ordering analysis is correct and I confirmed the consequence empirically**: last-applied layer is outermost, so today's chain is TraceLayer → rate_limit → vitals → security_headers → route, and `rate_limit` short-circuits at `:66-72` without calling `next.run()`. A live `429` returned `content-length: 17` and **no** CSP, HSTS, `nosniff`, `Content-Type`, `Retry-After`, or `Cache-Control` — F1 reproduced exactly. The proposed target order preserves both stated invariants. |
| Views/screens fit current navigation pattern | ✓ | No new route, no nav item, no template file. §3.1's inventory matches the shipped surfaces. The dashboard gate is respected verbatim. |
| Dependencies are available and version-compatible | ✗ | `governor` 0.6.3 does expose `NotUntil::wait_time_from` (verified in `gcra.rs:45`), so `Retry-After` needs no new crate. `ServeDir::not_found_service` exists in tower-http 0.5.2 under `fs` (verified `serve_dir/mod.rs:240`). `axum-client-ip = "0.5"` has **zero** references in `src/` — F5 confirmed. **But** `SetResponseHeaderLayer` (§4.7 Phase 1) is gated behind tower-http's `set-header` feature, which `Cargo.toml:14` does not enable. §4.5's "Added: none" and its feature-sufficiency enumeration are incomplete. |
| Platform/renderer requirements are realistic | ✓ | §4.6's Linux/macOS analysis holds: `/proc/self/status` gates the `Some` branch, CI is `ubuntu-latest`, and T5/T6 cover the `None` branch as pure units on any OS. Askama compiles templates at build time, so `cargo build --release` catches template breakage as §5.3 claims. |
| Test strategy is executable with current infrastructure | ✓ | Binary-crate constraint verified — no `[lib]`, private modules, so `tests/` cannot reach `crate::router`; every proposed test correctly lives in `src/`. All APIs the tests need are `pub` (`router::build`, `AppState::new`, `requests_total`, `build_limiter`, `rate_limit`). Each `router::build` constructs its own limiter, so T1's 10-path sweep will not collide with T2/T3's floods. Caveats: T9's "output of `hostname`" has no std API and no `hostname` binary exists on this workstation — specify `/etc/hostname` with a skip-if-absent and a minimum-length guard; T9's bare `10.` substring needs anchoring to avoid false positives; T7 requires editing an existing test the spec does not list. |
| Performance budget is realistic for target hardware | ✓ | All figures reproduced. RSS read live at **10 MiB** against a 30 MiB budget; `/status.json` measured **142 B** against a 512 B budget (spec said 146 B — digit-count dependent, well inside); `429` body **17 B** against 256 B. The self-DoS risk in §4.7 is real: `templates/base.html` pulls four subresources and `ServeDir` sets no `Cache-Control`, so a warm reader spends ~5 of 60 tokens per view. |
| No undeclared dependency on unbuilt features | ✓ | §7.4 declares every cross-agent blocker with an owner and states that S1–S3 are unblocked. Nothing depends on a B- or C-tier feature. Caveat: `asset_version()`'s delivery mechanism into `base.html` is unnamed (see 5A). |

**Feasibility verdict:** Feasible with caveats
**Caveats:** (1) `tower-http`'s `set-header` feature must be added to `Cargo.toml` for §4.7 Phase 1 — the spec's "no dependency changes" claim is incomplete. (2) `state::tests::bind_description_comes_from_the_resolved_listener_address` must be updated alongside §4.2's `BindMode::description` change; the spec does not list it. (3) T9's hostname and private-range assertions need a stated, portable mechanism. (4) `asset_version()`'s template delivery mechanism is unspecified.

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 3.00 | 25% | 0.750 |
| 2. Design & Craft Excellence | 2.50 | 25% | 0.625 |
| 3. Accessibility & Progressive Enhancement | 3.00 | 20% | 0.600 |
| 4. Competitive Depth & Differentiation | 2.60 | 20% | 0.520 |
| 5. Accuracy & Maintainability | 2.60 | 10% | 0.260 |
| **Composite** | | | **2.76** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 — 2.76
- [x] All lens averages ≥ 2.0 — lowest is 2.50 (Lens 2)
- [x] No criterion scores 0
- [x] No more than two criteria at 1 per lens — zero criteria at 1
- [x] All auto-fail rules pass — rule 1 (unearned claims), rule 2 (accessibility floor), rule 3 (no-JS floor) all clear
- [x] Feasibility ≠ Infeasible — Feasible with caveats

**All conditions met:** Yes → PASS

---

## Remediation Brief (non-blocking — spec passes; these raise quality)

### Priority 1 — Correctness errors that would mislead an implementer

1. **§4.5 / §7.2 `Cargo.toml` row — add the `set-header` feature.** §4.7 Phase 1 proposes
   `SetResponseHeaderLayer` on the `/static` service. That type lives in
   `tower_http::set_header`, gated by `#[cfg(feature = "set-header")]` (tower-http 0.5.2
   `src/lib.rs:232-233`). `Cargo.toml:14` currently reads
   `tower-http = { version = "0.5", features = ["fs", "trace"] }`. Change §4.5 from
   "Added: none" to "Added: none; one tower-http feature enabled (`set-header`)" and add
   the feature to §7.2's `Cargo.toml` row alongside the `axum-client-ip` removal.

2. **§7.2 `src/state.rs` row — declare the existing test edit.** §4.2's
   `BindMode::description` rewrite (`"loopback (127.0.0.1)"` → `"loopback (IPv4)"`,
   `"loopback (::1)"` → `"loopback (IPv6)"`) breaks
   `state::tests::bind_description_comes_from_the_resolved_listener_address` at
   `src/state.rs:348-370`, which currently passes and asserts the old literals. Add
   "update the two loopback assertions in `bind_description_comes_from_the_resolved_listener_address`;
   T7 supersedes them" to §7.2, and soften §6.4's "none is weakened or deleted" to
   distinguish anti-leak tests (untouched) from behavior-pinning tests (one updated,
   in the privacy-strengthening direction).

3. **§3.7 / §5.1 T17 / §7.1 F6 / §5.4 — the contrast failure count is 7, not 6.** The
   spec says "6 of 23" but lists seven themes (lunarcore, cloud, gameboy, c64, nes,
   solarized, blueprint). I recomputed all 23 independently: seven fail
   `--text-faint` on `--surface`. Every individual ratio in the spec is correct; only
   the count is wrong. Fix it in all four places, and correct the derived claim
   "reduces failures from 6 to 1" → "from 7 to 1".

4. **§5 — write out the four CI commands verbatim.** Criterion 5D requires the exact
   commands. Add to the head of §5:
   `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
   `cargo test --all-targets`, `cargo build --release`. §5.3's citation of
   "four cargo commands" is not a substitute.

5. **§7.1 F15 / §7.6 — `handlers/releases.rs` is already in the README tree**
   (`README.md:72`). Drop it from the omission list; `state.rs`,
   `middleware/vitals.rs`, and `handlers/status.rs` are genuinely missing.

### Priority 2 — Should fix for quality

6. **§3.5 / §5.4 — re-anchor the CSS animation citations after the rebase (not a spec
   error).** The spec cites `style.css:1279-1290`, `:1275-1278`, and `:1279`. These were
   **correct** against the spec's base commit `854e443`; commit `5e98092` then inserted
   exactly 31 lines at `style.css:1112`, shifting every line below it. Current numbers:
   reduced-motion comment `1306-1309`, `@media (prefers-reduced-motion: no-preference)`
   `1310`, the shared `animation` shorthand `1313`, delays `1315-1319`,
   `@keyframes boot-line-in` `1322-1325`. Separately, fix `src/errors.rs:92` → `:90` for
   the 500 `title()` — that one is a genuine off-by-two (`:92` is `description()`).

7. **§3.7 — resolve the interim-vs-T17 contradiction.** The `--text-muted` interim
   still fails solarized at 4.13, while T17 asserts 4.5:1 for `--text-muted` too and
   "must be introduced *with* the palette fix." State the sequence explicitly: interim
   ships without T17; T17 lands only in the same commit as A1's `--text-faint` fix,
   which must also raise solarized's `--text-muted` above 4.5:1 on `--surface`; the
   interim is never a valid end state.

8. **§5.1 T9 — make the leak assertions portable and non-flaky.** Replace "the output
   of `hostname`" with reading `/etc/hostname` and skipping if absent or shorter than
   4 characters (no `hostname` binary exists on the dev workstation and std has no
   API for it). Replace the bare `10.` substring with an anchored regex for
   private-range octets so `/status`'s uptime, request count, and version strings
   cannot false-positive.

9. **§3.7 — name the vitals-strip mechanism.** Choose `role="group"` on the existing
   `<div>` (keeping the current `aria-label="Server vitals"`) rather than offering A2
   three options for an accessibility decision this spec claims to own.

10. **§4.7 — name the `asset_version()` delivery mechanism.** `base.html` is extended
    by every page and Askama has no global context. Point at the pattern already in
    the repo: `templates/vitals_strip.html:6` calls `crate::state::Status::current()`
    directly from the template. Say whether `asset_version()` follows it, and name the
    guard that keeps all four `base.html` sites on the derived value.

11. **§6.3 — extend the claim audit to the two red-team code comments.**
    `src/middleware/rate_limit.rs:17-20` ("Red team context… Tools like hydra and
    ffuf…") and `src/middleware/security_headers.rs:13` ("blue team / red team") sit in
    files §7.2 already opens for comment rewrites, and auto-fail rule 1 forbids
    asserting an offensive-security identity. They are not user-visible, so this is not
    a violation — but a spec that audits `src/state.rs:91` for exactly this class should
    catch them in the same pass.

12. **§4.5 / §6.4 — complete the 1D copy audit.** §6.3 claims to cover "every
    user-visible string this feature owns" but omits the six `<dt>` labels, the four
    unchanged glosses at `templates/status.html:18,21,33,36`, the `robots.txt` header
    comment, and the 404/500 boot-log copy at `templates/error_404.html:5-9`.

### Priority 3 — Consider for excellence (2 → 3)

13. **§2 — add a self-directed-learner story (4E).** The criterion names three readers;
    the learner has none. §7.5's gap-analysis post is the natural surface.

14. **§7.5 — outline the gap-analysis post rather than listing its topics (4C, 2C).**
    Name the five sections and the one concept each builds. §4.4's layer-ordering
    analysis is already publishable-quality; say whether it becomes reader-facing.

15. **§7.2 — give each proposed post a `category` (2D).** `src/handlers/blog.rs:32-58`
    groups the index by `category` frontmatter and dumps unmatched posts into "Other
    writing". Name the pillar so the artifacts land where intended.

16. **§3.3 — reconcile the readout's type sizes with the site type scale (2B).**
    `.status-note` is `0.8rem` and `.vitals-strip` is `0.75rem`. Commit `3f96165`
    introduced a type scale; state whether these are on it or hand off the
    reconciliation to A1.
