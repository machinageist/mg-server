# Scorecard: Releases

**Feature ID:** B6
**Spec file:** gauntlet-output/specs/B6-releases.md
**Reviewer agent:** verify-agent (Claude Opus 4.8), blind review
**Date:** 2026-08-08
**Spec iteration reviewed:** 1

---

## Verdict: PASS

**Summary:** This is an exceptionally disciplined spec: it correctly reframes the
strongest asset on the page (the streaming SHA-256 mechanism) as worth keeping
while treating the *content* it currently distributes as a claim-integrity
violation, and it makes the fix self-enforcing with tests (T-1 no-uncleared-artifact,
T-2 no-forbidden-claim, I-3 not-in-nav). Every source citation I checked was
accurate to the line. The single most important non-blocking gap is that the
`.release-meta`/`.release-sha` small text (0.78rem on `--text-faint`) is not yet
AA-compliant across all themes and depends on an unlanded A1 token re-tune — the
spec discloses this and routes it correctly, so it does not trip the accessibility
auto-fail, but it is the one open contrast risk to track.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 1A. Evidence standard | 3 | §4.3 admission policy requires every published artifact be "defensible against the evidence standard (`public-portfolio-structure.md:76-94`)"; §1.2.1 maps the checksum mechanism to the standard's *verification* field. Default state is empty-until-cleared; the one optional artifact (mg-server snapshot, §4.5/§8 Q1b) is the Active portfolio entry with an existing evidence path. Verified: `public-portfolio-structure.md:76-94` is the "Evidence standard" section with exactly those fields. | — |
| 1B. State honesty | 3 | §7.1 marks each item implemented/gated/absent (✅/❌); it catches shipped copy claiming "compiled binaries" that do not exist and flags it as a defect (§6.3); T-2 guards it. No planned thing reads as shipped. | — |
| 1C. Publication gates | 3 | Core of the spec. Respects the GeistScope gate verbatim (`geistscope-page-triage.md:5-8`, verified), removes the four tarballs, refuses to re-litigate the gate (§0), and enforces it by test T-1. Verified: four `geistscope-*-source.tar.gz` present; tarball contents include `crates/mg-exploitgen`, `mg-breach`, `mg-brute`, `mg-leak-monitor` (archived/unsafe set per triage). | — |
| 1D. Copy currency | 3 | Catches the stale `description()` = "GeistScope source tarballs and compiled binaries." (verified `releases.rs:39-40`) and the "binary artifacts" intro; proposes current copy (§6.3). Page carries no cert copy so the spine is not applicable here, but user-visible copy is brought to reality. | — |
| 1E. Role posture | 3 | §4.3 forbids any capability the claim discipline bars leading with (pentest/red-team/offensive/production-grade/enterprise/SRE, `public-portfolio-structure.md:106-113`, verified); T-2 tests for those exact tokens; target copy leads with the mechanism (checksum/provenance). | — |
| 1F. Test-encoded policy | 3 | Adds T-1/T-2 to encode the gate; weakens no guard; makes real a guard the reorg docs falsely claim exists (I-3). Verified: `REORG_CHANGELOG.md:90` and `REORG_HANDOFF_PROMPT.md:57` both say "a test asserts that" but no test in `src/` references releases nav. | — |

**Lens average:** 3.00
**Lens pass:** Yes (avg ≥ 2.0, zero 1s, no 0s)

---

## Lens 2: Design & Craft Excellence (weight: 25%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 2A. Spec reconciliation | 3 | B6 is not the design-system feature; it scopes tokens/themes to A1 (§0) and does not re-assert the stale SOLARCORE_SPEC. Aligns with shipped-wins: the units fix (§4.2) explicitly matches the shipped vitals strip's "MiB" convention rather than an aspirational spec. | — |
| 2B. Typographic craft | 3 | Reading measure handled (`section-intro` `max-width: 65ch`/`--measure`, §3.3; verified `.section-intro` at `style.css:1198`); hash treated as code-like content that breaks anywhere rather than forcing scroll (§3.4); one `h1`, no color-dependent hierarchy. | — |
| 2C. Pedagogical depth | 2 | N/A — the releases page is a provenance surface, not a `/learn` page or study tool; the spec correctly makes no pedagogical claim and fakes no depth. Scored neutral (not penalized, not inflated) because the criterion does not apply to this surface. | — (no education content is expected here) |
| 2D. Scannability and structure | 3 | Deliberately not in primary nav (verified: `base.html:23-27` nav-links has only About/Portfolio/Writing/Learn); discoverability handled honestly via §7.4 cross-feature request to link from the mg-server portfolio entry, plus the `Section` enum registration. Guard I-3 keeps the not-in-nav decision honest. | — |
| 2E. Restraint | 3 | Exemplary: single quiet `--text-muted` empty-state line, explicit rejection of heavier empty states/spinners/cards (§3.3), no decorative motion, no fake metrics. Verified `.releases-empty` at `style.css:1272`. | — |
| 2F. Theme integrity | 3 | Color/font are theme-owned tokens; size/spacing are not per-theme (§3.4/§6.4). The `--text-faint` contrast concern is correctly identified as a genuine palette matter deferred to A1, requiring no per-theme B6 edit. Copy/units changes are theme-agnostic. | — |

**Lens average:** 2.83
**Lens pass:** Yes (avg ≥ 2.0, zero 1s, no 0s)

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 3A. Works without JS | 3 | §3.7 A: zero page JS, server-rendered Askama list, native `<a href download>` (verified `releases.html:20`); pinned by I-2 which strips `<script>` and asserts hrefs/`sha256:`/intro survive. Auto-fail rule 3 not triggered. | — |
| 3B. Contrast and color independence | 2 | §3.7 B / §6.4 3B honestly disclose that `.release-meta`/`.release-sha` at 0.78rem on `--text-faint` (verified `style.css:1312-1323`) must clear 4.5:1 and are currently non-compliant in the themes A1 enumerates until A1 re-tunes the token. Color independence is met (literal "download" + `↓`, literal `sha256:` prefix — verified template lines 20/24). The residual gap is real but external and blocked-on (§7.4). | Confirm A1's `--text-faint` ≥4.5:1-at-≤0.8rem fix lands before B6 ships as compliant; if A1 slips, consider a B6-local minimum size bump for the meta/hash rows. |
| 3C. Keyboard and focus | 3 | §3.7 C/D: native links, global `:focus-visible` never removed, natural DOM order, no `tabindex`/trap. | — |
| 3D. Semantics and AT | 3 | Real `<ul class="releases-list">` (verified line 15), one `<h1>`; and the spec *adds* meaningful AT fixes — `.vh` filename context per download link and an aria-label/`.vh` "checksum:" so the 64-char hash is announced as a checksum, correctly noting `download=` is not an accessible name (§3.7 E). | — |
| 3E. Motion and sensory safety | 3 | §3.5: no page motion; hover color change uses the global reduced-motion-gated transition; nothing flashes or autoplays. | — |
| 3F. Responsive and resilient | 3 | §3.4/§3.7 F: header stacks ≤640px (verified `style.css:1554-1560`), meta wraps, hash `word-break: break-all` so no horizontal scroll at 320px/400% zoom; designed empty state. | — |

**Lens average:** 2.83
**Lens pass:** Yes (avg ≥ 2.0, one 1-or-below? no — one 2, zero 1s, no 0s)
**Auto-fail triggered:** No — rule 3 (no-JS) passes cleanly; rule 2 (accessibility floor) considered against 3B and does not trigger: B6 introduces no new failing contrast pair, discloses the small-text/`--text-faint` risk, and routes the fix to its token owner (A1) as a blocking dependency rather than shipping it as final or hiding it. Focus states kept, no hue-only state, reduced-motion honored.

*(Template row 3G is not defined in criteria.md — Lens 3 has criteria 3A–3F only.)*

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 4A. Thirty-second differentiation | 3 | The impression is stated explicitly: "disciplined housekeeping," a "provenance surface," "integrity as a demonstrated habit" (§1.2.1, engineer-peer user story §2). Positions the checksum-next-to-download as interview-defensible ops behavior. | — |
| 4B. Evidence over enthusiasm | 3 | The entire feature is verification-first (checksums), surfaces failure/recovery (E-1..E-5, §3.6), and its success signal is an executable test; removes overclaiming copy. | — |
| 4C. Original explanation | 2 | N/A — no educational material on this surface; intro is honest framing, not a restated source note. Scored neutral (criterion targets teaching surfaces, which this is not). | — |
| 4D. Depth of a real system | 3 | Ties to a genuinely operated system: the mechanism is real operated engineering (`spawn_blocking` + streaming SHA-256, verified `releases.rs:49`,`:101-115`), and the recommended content is a snapshot of mg-server itself — the running server. | — |
| 4E. Reviewer paths | 3 | Explicitly accounts for verifier, engineer peer, empty-state visitor, screen-reader user, no-JS user, and operator/Jeff (§2); routes hiring managers away ("not a primary reviewer path — start at Portfolio or Writing") and proposes honest discovery via a portfolio link (§7.4). | — |

**Lens average:** 2.80
**Lens pass:** Yes (avg ≥ 2.0, zero 1s, no 0s)

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| 5A. Single source of truth | 3 | §3.3/§4.2: the filesystem (`static/releases/`) is the sole source of truth; size and hash are computed, never typed; the shown checksum is of the byte-identical file `ServeDir` serves (provenance by construction). No hand-maintained artifact list. | — |
| 5B. Drift guards | 3 | T-1 (no uncleared artifact), T-2 (no forbidden claim), I-3 (not in nav) all fail loudly in CI; the checksum-cache staleness obligation (key on size+mtime, test-pinned) is spelled out (E-5, §4.7). | — |
| 5C. No hidden coupling | 3 | Explicitly calls out the existing failure mode (home `assert!(contains("CompTIA"))` passing only via `<meta>`), and designs I-1/I-2 to assert against the page body; notes the docs claim a nonexistent test and makes I-3 the real guard (verified `REORG_CHANGELOG.md:90`, `REORG_HANDOFF_PROMPT.md:57`, and absence of any such test). | — |
| 5D. Verification is stated | 3 | Names the exact CI chain "fmt → clippy -D warnings → test → build --release" and `cargo test --all-targets` (§5), matching criteria.md 5D verbatim. | — |
| 5E. Documentation follows behavior | 3 | §7.2 Docs: update `public-portfolio-structure.md` "Still open," and correct `REORG_CHANGELOG.md:90` / `REORG_HANDOFF_PROMPT.md:57` from "a test asserts that" to reference the now-real I-3, in the same change. | — |

**Lens average:** 3.00
**Lens pass:** Yes (avg ≥ 2.0, zero 1s, no 0s)

---

## Feasibility Check

Read the actual source files referenced in the spec before filling this table.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | ✓ | `ReleaseArtifact` verified `releases.rs:21-26`; `ReleasesTemplate` `:29-33`; no new type needed. |
| API/interface changes are feasible with current architecture | ✓ | `GET /releases → releases::list` verified `router.rs:48`; `list()` signature `releases.rs:48`; error path `SiteError::Io` verified `errors.rs:56` + `other =>` arm `:113`. `section()` enum migration is an A2-owned enhancement, declared as a dependency, not required for B6's core. |
| Views/screens fit current navigation pattern | ✓ | Deliberately not in nav — verified `base.html:23-27` nav-links has only About/Portfolio/Writing/Learn; no `/releases`. Template structure matches `releases.html` exactly. |
| Dependencies are available and version-compatible | ✓ | `sha2 = "0.10"`, `hex = "0.4"` verified `Cargo.toml:54-55`. No new packages. |
| Platform/renderer requirements are realistic | ✓ | Reads ordinary files (no `/proc`), portable across macOS dev and Debian VM; `download` attribute universally supported with graceful degradation. |
| Test strategy is executable with current infrastructure | ✓ | `#[cfg(test)]` absent from `releases.rs` today (verified) — the gap the spec fills. Integration tests reuse the `tower::ServiceExt::oneshot` pattern verified at `errors.rs:171-193` (incl. the no-leak test the spec cites). |
| Performance budget is realistic for target hardware | ✓ | O(total artifact bytes) per request with the global rate limiter capping it (`router.rs:72-75`); artifact set is gate-limited to a handful; optional `(path,len,mtime)` cache is correctly deferred. |
| No undeclared dependency on unbuilt features | ✓ | A1 (`--text-faint` re-tune) and A2 (`Section::Releases`, `ServeDir::not_found_service`) are explicitly declared as blocking dependencies (§7.4); B6's core (remove tarballs, fix `description()`/units, add tests) is feasible today without them. |

**Feasibility verdict:** Feasible
**Caveats:** Full AA-contrast compliance of the meta/hash rows depends on A1's `--text-faint` token re-tune landing; the themed static-asset 404 (E-4) and the `Section::Releases` enum depend on A2. All are declared, and none block the core content/claim/test work.

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 3.00 | 25% | 0.750 |
| 2. Design & Craft Excellence | 2.83 | 25% | 0.708 |
| 3. Accessibility & Progressive Enhancement | 2.83 | 20% | 0.567 |
| 4. Competitive Depth & Differentiation | 2.80 | 20% | 0.560 |
| 5. Accuracy & Maintainability | 3.00 | 10% | 0.300 |
| **Composite** | | | **2.88** |

**Pass conditions (from criteria.md):**
- [x] Composite ≥ 2.0 (2.88)
- [x] All lens averages ≥ 2.0 (3.00 / 2.83 / 2.83 / 2.80 / 3.00)
- [x] No criterion scores 0
- [x] No more than two criteria at 1 per lens (zero 1s anywhere)
- [x] All auto-fail rules pass (rule 1 unearned-claims: spec *removes* the claim; rule 2 a11y floor: not triggered — disclosed + routed to A1; rule 3 no-JS: clean)
- [x] Feasibility ≠ Infeasible (Feasible)

**All conditions met:** Yes → PASS

---

## Remediation Brief

### Priority 1 — Must fix to pass
None. The spec meets all pass conditions.

### Priority 2 — Should fix for quality
1. **Contrast dependency risk (3B).** The spec ships as fully AA-compliant only
   after A1 re-tunes `--text-faint` to ≥4.5:1 at ≤0.8rem. Add an explicit gate:
   B6 should not mark `/releases` accessibility-complete until that A1 change is
   verified, and if A1 slips, adopt a B6-local minimum size for `.release-meta`/
   `.release-sha` (e.g. ≥0.875rem) as an interim mitigation rather than leaving a
   known-failing state live. This is the one open a11y gap.
2. **Q1 product decision is a genuine blocker for final content (§8).** The spec
   correctly leaves empty-vs-mg-server-snapshot-vs-retire to Jeff, but the
   implementer cannot finalize §4.5/§7.4 until it resolves. Flag it for an
   explicit decision before the test module is written (T-1 behaves identically
   for empty and for a single cleared artifact, so the tests are safe either way).

### Priority 3 — Consider for excellence
1. **Enumerate the mg-server snapshot's evidence-standard fields (1A/4B).** If Q1
   resolves to option (b), spell out the eight-plus evidence fields for the
   snapshot inline (it currently gestures at them via "the Active portfolio
   entry"), turning the page into the fully-worked integrity demo the spec
   envisions.
2. **Confirm IEC-vs-decimal unit choice (§4.2).** The spec offers both KiB/MiB and
   KB/MB-with-correct-divisor; pin the single choice so T-3's expected strings are
   unambiguous for the implementer (the spec recommends IEC to match the vitals
   strip — carry that through as the sole assertion).
