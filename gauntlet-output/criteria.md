# mg-server / machinageist.dev — Quality Criteria

**Generated:** 2026-08-07
**Based on interview with:** Jeff Cincoski (owner, sole operator)
**Criteria version:** 1

---

## Scoring

Each criterion is graded 0–3:

- **0 — Missing:** Not addressed.
- **1 — Inadequate:** Addressed but wrong, superficial, or contradicts constraints.
- **2 — Acceptable:** Correct and functional; minor gaps.
- **3 — Excellent:** Would ship in a best-in-class product; no meaningful gap.

**Pass threshold:** No criterion scores 0. Every lens average ≥ 2.0. No more than
two criteria at 1 per lens.

---

## Auto-Fail Rules

These override all other scoring. A spec violating any one of them fails outright
regardless of how well it scores elsewhere.

1. **Unearned claims.** Any spec that introduces a certification claim without a
   booked exam voucher, asserts an offensive-security / red-team / pentest
   identity, or presents a capability as built when it is planned, prototyped, or
   absent. The site already encodes this as tests — see
   `src/models/project.rs`, `src/handlers/pages.rs`, and `src/models/lab.rs`.
2. **Accessibility floor.** Any spec whose UI fails WCAG 2.1 AA contrast at its
   usage size, removes focus states, communicates state by hue alone, or ignores
   `prefers-reduced-motion`.
3. **No-JS floor.** Any spec whose *core* function is unreachable with JavaScript
   disabled. JS may sharpen an experience; it may not be the only way to use one.

**Not an auto-fail, but binding:** the GeistScope publication gate (see Lens 1C).
It is enforced by existing tests and reviewed as a Claim Integrity criterion
rather than as an outright spec killer.

---

## Lens 1: Claim Integrity & Evidence (weight: 25%)

**Governing standards:** `docs/public-portfolio-structure.md` (evidence standard
§76–94, claim discipline), `~/mg-coreforge/bootcamp/` PUBLIC_FACE and DEBT_REGISTER,
and the anti-overclaim tests already in the codebase.

**Context:** the site's core asset is that everything on it is true and defensible
in an interview. This lens outranks aesthetics because a single overclaim costs
more than a hundred design nits.

### 1A. Evidence standard
Every resume-facing artifact the spec proposes must be able to state: why it
matters for the target role, starting state, target state, tools used, real
evidence, what broke, verification, and what is still unknown. A spec that
proposes publishing an artifact without a path to those fields scores ≤ 1.

### 1B. State honesty
The spec distinguishes **implemented / prototyped / planned / gated / absent** for
every capability it references, and never lets a planned thing read as a shipped
thing in user-visible copy.

### 1C. Publication gates
The spec respects the GeistScope gate (full pipeline + human and AI operation +
sanitized evidence from an authorized engagement). Work in progress may appear on
a progress surface; it may not enter the portfolio or imply portfolio status.

### 1D. Copy currency
User-visible copy matches current reality — including the live certification
spine (RHCSA → CCNA → Security+, re-locked 2026-08-02). Stale claims that were
true once still score 0 here. *This criterion exists because `pages.rs` currently
says "working through the CompTIA stack," which the 2026-08-02 re-lock made
misleading.*

### 1E. Role posture
Copy leads with what the claim discipline permits (systems/NOC in training,
homelab operations, owned scope) and not with what it forbids (senior DevOps/SRE,
production-grade, enterprise, AI infrastructure engineer).

### 1F. Test-encoded policy
Where a claim boundary is already encoded as a test, the spec satisfies the test
rather than weakening or deleting it. Proposing to relax an anti-overclaim guard
without an explicit, recorded decision scores 0.

---

## Lens 2: Design & Craft Excellence (weight: 25%)

**Standard:** `docs/solarcore/SOLARCORE_SPEC.md`, **as reconciled** — see 2A.
**Benchmark tier:** Bartosz Ciechanowski and Nicky Case (pedagogical depth),
Julia Evans / Dan Luu / Simon Willison (honest concrete technical writing),
Arch Wiki and MDN (scannability, cross-linking, stable structure), Brandur and
the minimal-personal-site lineage (speed, restraint, typographic discipline).

### 2A. Spec reconciliation (blocking for the design-system feature)
SOLARCORE_SPEC.md currently contradicts the shipped site in at least five places:
Solarcore is specified as a night theme but ships light (Lunarcore is the dark
one); §10 forbids scanlines/CRT and a theme toggle, but both ship; the split-color
`MACHINA`/`GEIST` wordmark never shipped; `--sc-*` tokens ship as `--bg`/`--surface`;
and the magenta structural role is absent.

**Resolved direction (Jeff, 2026-08-07): the shipped site wins.** The theme roster,
the Lunarcore/Solarcore naming, and the real-data vitals strip are treated as
deliberate improvements. The design-system spec's job is to rewrite
SOLARCORE_SPEC.md to describe what actually shipped, then layer best practices on
top. A spec that silently re-asserts the stale spec scores 0.

### 2B. Typographic craft
Type scale, vertical rhythm, and reading measure are systematic rather than ad
hoc. Prose caps at a comfortable measure; code keeps the full column and scrolls.
Heading hierarchy is legible without relying on color.

### 2C. Pedagogical depth (education surfaces)
For `/learn` pages, glossaries, and study tools: concepts are built from the
ground up in ordinary language before jargon, connected to the larger system, and
paired with practice that works on hardware the reader already owns using FOSS
tools. Bullet-dumping a source note scores ≤ 1.

### 2D. Scannability and structure
Navigation, sidebars, cross-links, and page structure let a reader find and
re-find material. New pages register in every place that must know about them
(`SIDEBAR`, `WIKI_SLUGS`, index listings).

### 2E. Restraint
Spectacle is budgeted to chrome; body copy stays quiet. No dashboard cosplay, no
fake metrics, no card-ification of list architecture, no decorative motion.

### 2F. Theme integrity
Any visual change works across all 23 themes. Color and font role belong to
themes; size and spacing do not. A change requiring per-theme edits scores ≤ 1
unless it genuinely is a palette concern.

---

## Lens 3: Accessibility & Progressive Enhancement (weight: 20%)

**Standard:** WCAG 2.1 AA, plus the site's own no-JS identity (80 lines of
JavaScript total, theme selector only).

### 3A. Works without JavaScript
Core function is reachable with JS disabled — server-rendered, plain forms, real
URLs. JS is an enhancement layer with a defined fallback. *Auto-fail if violated.*

### 3B. Contrast and color independence
Every text/background pair meets AA at its usage size across all themes. State is
never communicated by hue alone.

### 3C. Keyboard and focus
Every interactive element is reachable and operable by keyboard, with a visible
focus indicator and a sensible focus order. Menu/widget patterns follow the ARIA
APG (the theme menu's roving-focus model is the in-repo reference).

### 3D. Semantics and assistive technology
Correct roles, labels, and landmarks. Decorative content is hidden from assistive
tech; meaningful content is not. Headings form a real outline.

### 3E. Motion and sensory safety
All motion sits behind `prefers-reduced-motion: no-preference`. No autoplay, no
body-content animation, no flashing.

### 3F. Responsive and resilient
Works from narrow mobile to wide desktop, at large text sizes, and when content is
missing or fails to load (empty states are designed, not accidental).

---

## Lens 4: Competitive Depth & Differentiation (weight: 20%)

**Competitor set:** a blend of (1) other junior/self-taught homelab portfolios and
(2) bootcamp and certification-track candidates. Both are large fields with
similar surface artifacts.

| Competitor | Strengths to match | Gaps to exploit |
|---|---|---|
| Junior homelab portfolios | A real lab, screenshots, a GitHub, enthusiasm | Rarely show verification, failure, or recovery; claims outrun evidence; no writing that survives scrutiny |
| Cert-track candidates | Structured curriculum, clear milestones, recognizable credentials | Course completion substitutes for operated systems; little original explanation; no public artifact of judgment |

### 4A. Thirty-second differentiation
A reviewer skimming for thirty seconds can tell what this person actually operates
and why it is different from the field. The spec states what that impression is.

### 4B. Evidence over enthusiasm
The feature surfaces verification, failure, and recovery — not just green
screenshots. The network-migration post (an outage worked end to end) is the
in-repo model.

### 4C. Original explanation
Educational material teaches rather than restates. The differentiator against both
competitor groups is explanation quality that a working engineer would respect.

### 4D. Depth of a real system
Features connect to a system genuinely operated (Proxmox cluster, DNS, this
server), not to a tutorial followed once.

### 4E. Reviewer paths
The spec accounts for how different readers arrive and what each needs: hiring
manager, engineer peer, and self-directed learner.

---

## Lens 5: Accuracy & Maintainability (weight: 10%)

**Context:** this lens exists because the current codebase demonstrates the
failure mode. It grades whether a spec creates future drift.

### 5A. Single source of truth
Data has one definition. Where duplication is deliberate (the SIDEBAR /
WIKI_SLUGS split, which decouples the test crate from the bin), the spec says so
and names the guard that keeps the copies honest.

### 5B. Drift guards
Anything that can silently fall out of sync has a mechanism that fails loudly.
*Reference: `generate_themes.py` emitted a flat theme menu after the menu was
grouped — regenerating would have reverted the grouping with no error.*

### 5C. No hidden coupling
Tests assert against the thing they name. *Reference: the home page's
`assert!(html.contains("CompTIA"))` passes only via the `<meta description>` tag,
so editing unrelated copy breaks a test that appears to be about the page body.*

### 5D. Verification is stated
The spec names the exact commands that prove it works, and they run in CI
(`cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`,
`cargo test --all-targets`, `cargo build --release`).

### 5E. Documentation follows behavior
When shipped behavior changes, the spec says which long-lived document must be
updated in the same change.

---

## Scoring Summary

| Lens | Criteria count | Weight | Auto-fail conditions |
|---|---|---|---|
| 1. Claim Integrity & Evidence | 6 | 25% | Unearned claims (rule 1) |
| 2. Design & Craft Excellence | 6 | 25% | — |
| 3. Accessibility & Progressive Enhancement | 6 | 20% | Accessibility floor (rule 2), No-JS floor (rule 3) |
| 4. Competitive Depth & Differentiation | 5 | 20% | — |
| 5. Accuracy & Maintainability | 5 | 10% | — |

Weights sum to 100%.
