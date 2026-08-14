# Remediation Brief — outstanding scorecard items

**Written:** 2026-08-14
**Covers:** `scorecards/A2-site-shell-scorecard.md`, `scorecards/A3-ops-and-observability-scorecard.md`,
`scorecards/B3-portfolio-scorecard.md`
**Verified against:** working tree at commit `b635b99`
**Status:** analysis only — no source file was edited to produce this brief.

---

## How to read this

Every item below was re-checked against the file it names. Each carries one of three
verdicts:

- **VALID** — still true today. Corrected `file:line` given.
- **RESOLVED** — fixed since the scorecard was written. Evidence given. Do not re-apply.
- **NOT REAL** — the scorecard's own claim is wrong. The spec it "corrected" was right.

Each item also says whether the fix is an edit to **the spec document** or to **source**.
Most A2 and A3 items are spec edits: the specs have not been implemented, so their errors
are errors in instructions, not in shipped code.

### ⚠ Line citations in this repo are volatile

The known `+31` rule (commit `5e98092` inserted 31 lines into `static/css/style.css` above
line 1112) is **no longer sufficient**. Since the A2 scorecard was written, four more
commits touched `style.css` — `0cdbbea`, `a375a14`, `cca5910`, plus `5e98092` — and the file
went from 1,502 lines to **1,592**. `src/handlers/wiki.rs` grew ~30 lines from new `/learn`
pages. **The scorecards' own "corrected" late-file citations are themselves stale now.**

Practical rule: for `style.css` above line ~1100 and for `base.html`, `errors.rs`,
`state.rs`, `router.rs`, `project.rs`, citations are stable and have been verified exact.
For `style.css` below line ~1100 and for `wiki.rs`, re-grep before trusting any number,
including the ones in this brief.

---

## A2 — site-shell

Source: `scorecards/A2-site-shell-scorecard.md`, "Corrections Required Before
Implementation". Target of every fix unless stated: `gauntlet-output/specs/A2-site-shell.md`.

### The three drift guards that are red on arrival

**A2-1 · U-10 asserts a false fact — VALID**

- **Where:** spec §5.1, guard U-10.
- **Wrong:** U-10 asserts the `base.html` `[data-mode]` list, `main.js` `MODES`, and
  `theme-init.js` `MODES` are "equal, in order, length 24". The sets are equal and all
  three are length 24, but the orders diverge at the **fifth** element. Verified today:
  - `templates/base.html` (`data-mode` attributes, document order):
    `system, lunarcore, solarcore, dark, solarized, nord, gruvbox, crt, amber, matrix,
    teletext, gameboy, c64, nes, synthwave, vaporwave, cyberpunk, tron, light, paper,
    dawn, cloud, blueprint, sepia`
  - `static/js/main.js` and `static/js/theme-init.js` (byte-identical to each other):
    `system, lunarcore, solarcore, dark, light, crt, amber, paper, dawn, cloud, gameboy,
    c64, teletext, nes, matrix, solarized, nord, gruvbox, synthwave, vaporwave, cyberpunk,
    tron, blueprint, sepia`
  - The divergence is deliberate: commit `af6566d` ("feat: group the theme menu into
    labeled sections") reordered the buttons into visual groups. U-10 as written would
    force that grouping to be undone.
- **Fix (spec):** split U-10 into two assertions — (a) the three lists are equal as **sets**
  and all have length 24; (b) `main.js` `MODES` and `theme-init.js` `MODES` are identical
  **in order** to each other. Drop the ordered comparison against `base.html`.

**A2-2 · U-6 description-length bound is red on files A2 owns — VALID, numbers corrected**

- **Where:** spec §5.1, guard U-6 (`50 <= len(description()) <= 160`).
- **Wrong:** fails today. Re-measured every `description()` in `src/`:

  | Location | Length | Verdict |
  |---|---|---|
  | `src/handlers/releases.rs:40` | 49 | under |
  | `src/errors.rs:76` | 38 | under |
  | `src/errors.rs:93` | 42 | under |
  | `src/handlers/pages.rs:44` | 124 | ok |
  | `src/handlers/pages.rs:81` | 110 | ok |
  | `src/handlers/pages.rs:114` | 66 | ok |
  | `src/handlers/blog.rs:68` | 121 | ok |
  | `src/handlers/status.rs:38` | 133 | ok |

  Two of the three failures (`errors.rs`) are in a file A2 itself modifies.
- **Correction to the scorecard's own numbers:** it says "five `content/**` summaries over
  160 (max 258, in `content/posts/management-layer-first-network-migration.md`)". Today
  there are **six** shipped ones plus one draft, and the max is not where it says:

  | File | Summary length |
  |---|---|
  | `content/posts/management-layer-first-network-migration.md` | 227 |
  | `content/posts/hosting-machinageist-dev.md` | 220 |
  | `content/posts/security-headers-on-machinageist-dev.md` | 217 |
  | `content/pages/linux-shell.md` | 176 |
  | `content/pages/linux-permissions.md` | 175 |
  | `content/pages/modern-network-environments.md` | 168 |
  | `content/drafts/geistscope-retrospective.md` | 256 *(unrouted, not served)* |

- **Fix (spec):** scope U-6 to the eight static template `description()` methods and set the
  lower bound to 38 or shorten the three short ones; file the frontmatter-summary length
  rule as a separate content guard with an upper bound of 240 if it is wanted at all. State
  in §5.1 which commit turns U-6 green — the spec already does this for I-4.

**A2-3 · U-7's `"enterprise"` ban is red on shipped educational copy — VALID**

- **Where:** spec §5.1, guard U-7 (and scorecard criterion 1E).
- **Corrected citation:** `content/pages/network-topologies.md:4` —
  `summary: "How nodes and links are arranged — mesh, star, ring, spine-and-leaf, and the
  tiered designs used in enterprise and data center networks."` This reaches `<meta>` via
  `WikiPageTemplate::description()`, now at **`src/handlers/wiki.rs:144-146`** (the
  scorecard cites `:114-116`; the file has grown ~30 lines).
- Two other occurrences are body prose and never reach `<meta>`:
  `content/pages/index.md:40`, `content/pages/modern-network-environments.md:245`,
  `content/pages/network-topologies.md:130,143`.
- **Fix (spec):** in U-7, keep `"production-grade"`, `"SRE"`, `"red-team"`, `"pentest"`,
  `"offensive security"`, `"Network+"`, `"the CompTIA stack"` as unconditional bans. Drop
  bare `"enterprise"`; if the concept is worth guarding, ban `"enterprise-grade"` and the
  first-person construction `\b(I|we)\b[^.]*\benterprise\b`. Topic vocabulary is not a role
  claim.

### The four blocking feasibility defects

**A2-4 · `tests/shell.rs` cannot exist as specified — VALID**

- **Verified:** no `src/lib.rs`, no `[lib]` section in `Cargo.toml`, and `src/main.rs:16-21`
  declares every module privately (`mod errors; mod handlers; mod middleware; mod models;
  mod router; mod state;`). An integration test under `tests/` cannot reach
  `crate::router::build` or a proposed `crate::shell::NAV`.
- This is exactly why `tests/wiki_pages.rs:11-13` re-declares `WIKI_SLUGS` with the comment
  "duplicated here on purpose so the test crate stays decoupled from the bin", and why every
  router-level test lives in `#[cfg(test)]` inside `src/` (`src/errors.rs`,
  `src/handlers/status.rs`, `src/handlers/pages.rs`, `src/models/project.rs`).
- **Fix (spec):** move I-1 … I-8 into `#[cfg(test)] mod tests` inside `src/shell.rs`. Delete
  the `tests/shell.rs` row from §7.2.

**A2-5 · Contract S-2 cannot override `og:type` — VALID**

- **Verified at current HEAD:** `templates/base.html:9` emits
  `<meta property="og:type" content="website">`; `templates/base.html:14` is
  `{% block head_extra %}{% endblock %}`. A `head_extra` override can only append a second
  `og:type`, never replace the first. I-7 (required meta appears once per route) and I-8
  (blog/learn declare `article`) are therefore mutually unsatisfiable under the proposed
  mechanism.
- **Fix (spec):** add a fourth metadata-contract method `fn og_type(&self) -> &str
  { "website" }` to Contract S-1, overridden by `BlogPostTemplate` and `WikiPageTemplate`;
  or add `{% block og_type %}website{% endblock %}` at `base.html:9`. Either makes I-7 and
  I-8 satisfiable.

**A2-6 · `og:url` has no data source — VALID**

- **Verified:** `templates/base.html:4-14` contains no `og:url`, and the metadata contract
  every template implements is only `title()` / `description()` / `section()` (confirmed in
  `src/handlers/pages.rs`, `blog.rs`, `wiki.rs`, `releases.rs`, `status.rs`, `errors.rs`).
  Nothing tells `base.html` the current path.
- **Fix (spec):** add `fn canonical_path(&self) -> &str` (or a `path: String` field) to
  Contract S-1 in §4.3 and name the site-origin constant it is joined to. Without it, I-7's
  `og:url` assertion is unimplementable.

**A2-7 · `og-card.png` has no generator — VALID**

- **Verified:** `docs/solarcore/generate_brand.py:231` writes **`og-card.svg`**, built from
  the script's own internal geometry rather than from `mark.svg`. Its only imports are
  `math` (line 6) and `re, sys, os` (line 222) — there is no rasteriser in the repo, and
  `static/img/` contains no `og-card.png`.
- **Fix (spec):** either declare the rasteriser (`rsvg-convert` or `cairosvg`) as a
  documented dev-time tool and correct §4.5's "Infrastructure: none", or change the
  deliverable to a PNG produced once by hand and checked in. Note that SVG `og:image` is not
  reliably rendered by Slack, LinkedIn, or Discord — the exact channels F-12 is about.

### Citation drift (A2 quality item 8)

**A2-8 · Citation corrections — PARTLY VALID, partly superseded**

Re-verified at `b635b99`:

| Claim | Verdict | Current truth |
|---|---|---|
| `router.rs` routes are at 37-57 (spec said 38-58) | **VALID** | `.route("/")` at **:37**, `.route("/robots.txt")` at **:57** |
| `nest_service("/static")` at 59 (spec said 60) | **VALID** | **:59** |
| `.fallback(errors::fallback_404)` at 61 (spec said 62) | **VALID** | **:61** |
| legacy `/wiki` routes end at 45 (spec said 41-46) | **VALID** | `/wiki` **:44**, `/wiki/:slug` **:45** |
| 15 registered routes, 9 render the shell | **VALID** | Counted: 15 registered; shell-rendering = `/`, `/about`, `/portfolio`, `/blog`, `/blog/:slug`, `/learn`, `/learn/:slug`, `/releases`, `/status` = 9 |
| `wiki.rs` `title()` is 110-112, `section()` is 118-120 | **SUPERSEDED** | Now `title()` **:140-142**, `description()` **:144-146**, `section()` **:148-150** |
| boot-line stagger at `style.css:1309-1319` | **SUPERSEDED** | Now **:1389-1399**; `@keyframes boot-line-in` **:1401-1404** |
| `≤ 640px` block at `style.css:1475-1502` | **SUPERSEDED** | Now **:1558-1592** |
| "1,502-line stylesheet" | **SUPERSEDED** | Now **1,592** lines |
| add a `≤ 800px` row (`style.css:1404-1470`) | **VALID, renumbered** | The `@media (max-width: 800px)` block is now **:1487-1552** |
| U-4 "10 content templates + both error templates" double-counts | **VALID** | `templates/` holds 12 files: `base.html` + `vitals_strip.html` (shell) + **10** others, of which 2 *are* the error pages |
| §1.3 "every route in `router.rs:38-58` renders HTML" is false | **VALID** | `/robots.txt`, `/security.txt`, `/status.json` do not |

- **Fix (spec):** apply the VALID rows; re-grep rather than copying the SUPERSEDED rows —
  they will move again.

### A2 quality items 9–12

**A2-9 · Redirect the focus-ring contrast request — RESOLVED**

- The scorecard asked to change the A1 audit request from `--accent` vs `--bg` to `--accent`
  vs `--surface`, on the grounds that accent-vs-surface was unaudited.
- **It is audited now.** `docs/themes/generate_themes.py` `USAGE` holds every text token to
  **4.5 against both `bg` and `surface`**, `accent` included, and the header comment records
  the change ("The old audit checked five tokens against `--bg` only… So every text token is
  held to 4.5 against both backgrounds"). `python3 docs/themes/generate_themes.py --check`
  reports `contrast: all pairs clear across 23 themes`.
- **Action:** none. Delete the request from the spec rather than implementing it.

**A2-10 · Hover and active need a visual differentiator — VALID, refined**

- **Corrected citation:** the scorecard cites `style.css:699`. The rule is now at
  **`static/css/style.css:724`**: `.nav-link:hover::after, .nav-link.is-active::after
  { right: 0; }`.
- **Refinement:** hover and active are *not* fully identical — `:621` gives hover
  `color: var(--text)` and `:622` gives active `color: var(--accent)`. But that difference is
  hue-only, which criterion 3B disallows as the sole channel, and the non-hue channel (the
  underline sweep) *is* identical. The finding stands; the scorecard overstates it.
- **Fix (source, `static/css/style.css`):** give active and hover different underline
  geometry — e.g. active keeps the full-width 1.5px rule and hover uses 1px at reduced
  opacity, or active gains a leading marker. Specify it as a size/weight change so criterion
  2F (themes own colour, not size) still holds.

**A2-11 · Two navigation surfaces are unowned — VALID, citations exact**

- **Verified:** `templates/index.html:11` is
  `<nav class="hero-actions" aria-label="Quick navigation">`, and
  `templates/wiki_page.html:4` is
  `<aside class="wiki-sidebar" aria-label="Education wiki navigation">` wrapping a
  `<details>` at `:5` and a bare, unnamed `<nav>` at `:7`.
- Both reproduce the "…navigation, navigation" double-announce that F-10 exists to
  eliminate, and neither appears in the spec's §3.1 surface inventory or §3.7 landmark table.
- **Fix (spec + source):** add both to §3.1/§3.7 as owned-or-delegated; add the test
  `landmark_names_follow_the_shell_contract` asserting that no `<nav>`/`<aside>`
  `aria-label` matches `/navigation"?$/i` and that each page renders at most one unnamed
  `<nav>`; then relabel the two elements.

**A2-12 · Migrate shell chrome onto the `--text-*` scale — VALID, one item wrong**

Re-measured at `b635b99`. The type scale lives at `static/css/style.css:482-487`
(`--text-xs: 0.75rem` … `--text-2xl: 1.6rem`), `--measure: 72ch` at `:499`.

| Element | Current | Line | Nearest token |
|---|---|---|---|
| `.brand` | `1rem` | `:590` | none (between `--text-lg` 1.05 and `--text-md` 0.95) |
| `.nav-link` | `0.875rem` | `:617` | none (between `--text-sm` 0.85 and `--text-md` 0.95) |
| `.theme-group-label` | `0.65rem` | `:681` | none — below the `--text-xs` floor |
| `.theme-menu button` | `0.8rem` | `:696` | none (between `--text-xs` 0.75 and `--text-sm` 0.85) |
| `.site-footer` | `0.8rem` | `:822` | none |
| `.vitals-strip` | `0.75rem` | `:863` | **`--text-xs`** — direct swap |

- **`.theme-btn` is listed in the scorecard but has no font-size at all** — `:638` sets
  `font: inherit`. Drop it from the migration list.
- **Fix (spec + source):** only `.vitals-strip` maps cleanly today. The other five need
  either new scale steps from A1 or a decision to round them onto existing steps. State that
  in §7.2 as an A1 request rather than shipping five new literals.

---

## A3 — ops and observability

Source: `scorecards/A3-ops-and-observability-scorecard.md`, "Priority 1". Target:
`gauntlet-output/specs/A3-ops-and-observability.md`.

**`manifest.md` disagrees with itself about A3** — one side of an unresolved git conflict
says "all Priority 1 applied (`f718f26`)", the other says the three items are outstanding.
Resolving that here: **commit `f718f26` ("gauntlet: correct the A3 ops spec") applied items
1, 2, 3 and part of 5. Item 4 was never applied.** Its message even says the correction
agent hit a session limit partway through.

**A3-1 · `set-header` feature gate — RESOLVED (in the spec)**

- The spec now carries the manifest edit at `specs/A3-ops-and-observability.md:747-768`:
  `tower-http = { version = "0.5", features = ["fs", "trace", "set-header"] }`, plus the
  rationale (`set-header = []`, no transitive crates, lockfile unchanged), and repeats the
  gate at `:850-851`.
- **Still true of source:** `Cargo.toml:14` reads `features = ["fs", "trace"]`. That is
  correct — the spec is not implemented yet. The feature must be added *when* §4.7 Phase 1
  ships, not before.
- **Action:** none on the spec. Carry the `Cargo.toml` edit into the implementation commit.

**A3-2 · `BindMode::description` test collision — RESOLVED (in the spec)**

- Recorded at `specs/A3-ops-and-observability.md:1351` (the `src/state.rs` file-edit row now
  names the test and the new literals) and at `:1220` (criterion 1F now distinguishes
  anti-leak tests from behaviour-pinning tests).
- **Verified still accurate:** `src/state.rs:213-214` returns `"loopback (127.0.0.1)"` /
  `"loopback (::1)"`, and `bind_description_comes_from_the_resolved_listener_address` spans
  **`src/state.rs:348-370`** — the spec's citation is exact.
- **Action:** none on the spec. The test edit ships with the `BindMode` change.

**A3-3 · Contrast count 6 → 7 — RESOLVED in the spec, and the underlying failures are gone**

- The spec was corrected to "**7 of 23**" at `:393` and `:1290`.
- **But the premise is now obsolete.** Commit `0cdbbea` ("a11y: clear all 14 contrast
  failures across the theme roster") landed, and the audit was simultaneously tightened —
  `docs/themes/generate_themes.py` `USAGE` now holds all six text tokens to 4.5 against
  **both** `bg` and `surface`. `python3 docs/themes/generate_themes.py --check` returns
  `contrast: all pairs clear across 23 themes`.
- **Action (spec):** rewrite F6 / §3.7 / T17 / §5.4 from "7 of 23 fail" to "zero fail; the
  audit was tightened to 4.5 on both backgrounds by `0cdbbea`". T17 as written asserts a fix
  that already shipped, and the §3.7 "interim vs T17" contradiction (Priority 2 item 7)
  dissolves with it. **This is the only A3 item that needs new work, and it is new work the
  scorecard did not anticipate.**

**A3-4 · Write out the four CI commands verbatim in §5 — VALID, never applied**

- **Verified:** `specs/A3-ops-and-observability.md` contains no occurrence of
  `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, or
  `cargo build --release`. §5 begins at `:866` with no command block; `:869` only says
  "All run under `cargo test --all-targets` in CI."
- Criterion 5D requires the exact commands.
- **Fix (spec):** add to the head of §5, matching `.github/workflows/ci.yml` in order:

  ```sh
  python3 docs/themes/generate_themes.py --check
  cargo fmt --all -- --check
  cargo clippy --all-targets -- -D warnings
  cargo test --all-targets
  cargo build --release
  ```

  Note the theme-contrast step runs **first** in CI — the A3 scorecard's four-command list
  omits it, and it is the step most relevant to A3's own §3.7 contrast work.

**A3-5 · `handlers/releases.rs` already in the README tree — PARTLY APPLIED**

- **Verified:** `README.md:72` does list `releases.rs`. The scorecard is right.
- The spec prose was corrected at `:1326` ("`handlers/releases.rs` **is**…"), but the
  file-edit table at `:1462` still says to add `handlers/releases.rs` to the README tree.
- **Fix (spec):** drop `handlers/releases.rs` from the `:1462` row. `state.rs`,
  `middleware/vitals.rs`, and `handlers/status.rs` are genuinely absent from `README.md` and
  should stay in that row.

**A3-6 · (Priority 2, but now materially wrong) CSS animation citations — SUPERSEDED**

Flagged because the scorecard's own "current numbers" have since moved again:

| Item | Scorecard's corrected value | Current at `b635b99` |
|---|---|---|
| `@media (prefers-reduced-motion: no-preference)` (boot lines) | `:1310` | **`:1389`** |
| shared `animation` shorthand | `:1313` | **`:1392`** |
| stagger delays | `:1315-1319` | **`:1394-1398`** |
| `@keyframes boot-line-in` | `:1322-1325` | **`:1401-1404`** |
| `src/errors.rs:92` → `:90` for the 500 `title()` | `:90` | **`:90` — still correct** |

---

## B3 — portfolio

Source: `scorecards/B3-portfolio-scorecard.md`. Its Priority 1 list is literally "None"; the
items the manifest tracks as outstanding are its Priority 2 and 3 lists.

**B3-1 · `--text-2xs` is a fabricated token — VALID**

- **Verified:** `--text-2xs` and `--measure-narrow` appear **nowhere** in `static/`,
  `templates/`, or `src/`. The real scale is at `static/css/style.css:482-487`:
  `--text-xs: 0.75rem`, `--text-sm: 0.85`, `--text-md: 0.95`, `--text-lg: 1.05`,
  `--text-xl: 1.3`, `--text-2xl: 1.6`; `--measure: 72ch` at `:499`.
- **Where the spec is wrong:** `specs/B3-portfolio.md:249` asserts
  "`--text-2xs: 0.70rem` exists"; `:537-538` lists `--text-2xs` and `--measure-narrow` among
  Layer-2 tokens to reference.
- **The direction is still right:** the two literals the spec wants tokenized are real —
  `static/css/style.css:1085` and `:1289` both hard-code `0.72rem`, which is genuinely
  below the `--text-xs` floor.
- **Fix (spec):** rewrite `:249` to state the real floor. Then either (a) have the A1 sweep
  create `--text-2xs` / `--measure-narrow` before B3 references them, or (b) map the two
  `0.72rem` literals onto `--text-xs`. Do not leave an instruction pointing at a token that
  does not exist.

**B3-2 · Missing designed empty state in `portfolio.html` — VALID**

- **Verified:** `templates/portfolio.html:11-32` is a bare
  `{% for project in projects %} … {% endfor %}` with no `{% else %}`. If `project::all()`
  ever returns empty, the page renders `<h1>Portfolio</h1>`, the intro paragraph, and an
  empty `<ul>`.
- Criterion 3F requires empty states to be designed, not accidental.
- **Fix (source, `templates/portfolio.html`):** add an `{% else %}` branch inside the `for`,
  carrying the honest copy from spec §3.3.

**B3-3 · No external-link cue on the entry link — VALID**

- **Verified:** `templates/portfolio.html:17` is
  `<a href="{{ url }}" target="_blank" rel="noopener noreferrer">{{ project.name }}</a>` —
  no visible affordance and no screen-reader suffix that the link opens a new tab.
- **Fix (source, `templates/portfolio.html:17`):** add a visible `↗` (marked
  `aria-hidden="true"`) and/or an `aria-label` suffix such as
  `"{{ project.name }} (opens in a new tab)"`.

**B3-4 · No rendered-HTML anti-overclaim test — VALID**

- **Verified:** the shipped guard
  `portfolio_only_carries_entries_with_verifiable_status_and_evidence`
  (`src/models/project.rs:92-116`) asserts only on `name + description` strings joined from
  the model (`:103-107`). It never renders the template.
- `src/handlers/pages.rs`'s `#[cfg(test)]` module covers `IndexTemplate` and
  `AboutTemplate` but has **no `PortfolioTemplate` render test at all**.
- **Fix (source, `src/handlers/pages.rs`):** add a `PortfolioTemplate::render()` test in the
  existing `#[cfg(test)] mod tests`, following the shape of
  `home_page_shows_concrete_work_without_strategy_narration`, so the anti-overclaim guard
  covers the rendered HTML (tags, status labels, template chrome) and not just two model
  fields.

**B3-5 · "Portfolio nav link is `base.html:26` not `:25`" — NOT REAL**

- **The scorecard is wrong and the spec was right.** `templates/base.html` is unmodified
  since commit `af6566d`; its nav block reads:

  ```text
  24:  <a href="/about"     …>About</a>
  25:  <a href="/portfolio" …>Portfolio</a>
  26:  <a href="/blog"      …>Writing</a>
  27:  <a href="/learn"     …>Learn</a>
  ```

  `specs/B3-portfolio.md:127` and `:324` cite `base.html:25`, which is correct. The
  scorecard's parenthetical "line 25 is the About link" is off by one — line 24 is About.
- **Action:** do not apply. If anything, note the scorecard's error so a later agent does not
  "fix" a correct citation.

**B3-6 · "Card transition is `style.css:744` not `:745`" — NOT REAL**

- **The scorecard is wrong and the spec was right.** `static/css/style.css:745` is
  `.project-card, .post-item { transition: background-color 0.2s ease, box-shadow 0.2s ease; }`.
  Line 744 is `.nav-link::after { transition: right 0.22s ease; }`.
- This citation sits above line 1112, so the `+31` drift never applied to it, and it has not
  moved.
- **Action:** do not apply.

**B3-7 · Adopt `evidence: Option<&'static str>` and `status_display_and_class_agree`
— VALID, optional**

- Priority 3. `src/models/project.rs:21-30` has no `evidence` field; `ProjectStatus::InProgress`
  and `::Complete` both carry `#[allow(dead_code)]` (`:41`, `:43`) and are never exercised.
- A `status_display_and_class_agree` test would legitimise both variants and let the
  `#[allow(dead_code)]` attributes come off.

---

## Also worth knowing

**B5's outstanding item is RESOLVED.** The manifest lists "stale `network-plus` tag on 11 of
12 learn pages" as outstanding for B5. Commit `9664566` ("content: drop cert tags from the
learn pages") landed: `grep -rn "network-plus" content/pages/` returns **zero** hits. Commit
`ab97562` ("content: attribute every networking page to the source textbook") separately
closed the sourcing half. Only the spec-text half — B5 spec §6.3's false "aligned with the
live cert spine" claim, and extending the A2 retired-claims guard to cover the `tags` field
— may remain; check the B5 spec before re-opening it.

**`gauntlet-output/manifest.md` has unresolved git conflict markers** in both the status
table and the correction-pass table. Whoever implements this brief should resolve them —
the lower (`fc3da33…`) side is current — and update the A2/A3/B3 "Outstanding" cells against
the verdicts above.

---

## Summary

| ID | Item | Verdict | Fix target |
|---|---|---|---|
| A2-1 | U-10 theme order equality | VALID | spec |
| A2-2 | U-6 description length bounds | VALID (numbers corrected) | spec |
| A2-3 | U-7 bans `"enterprise"` | VALID | spec |
| A2-4 | `tests/shell.rs` in a bin-only crate | VALID | spec |
| A2-5 | `og:type` cannot be overridden | VALID | spec |
| A2-6 | `og:url` has no data source | VALID | spec |
| A2-7 | `og-card.png` has no generator | VALID | spec |
| A2-8 | Citation drift | PARTLY VALID, partly superseded | spec |
| A2-9 | Focus-ring contrast request | **RESOLVED** (`0cdbbea`) | delete from spec |
| A2-10 | Hover ≠ active visual cue | VALID (refined) | source |
| A2-11 | Two unowned nav landmarks | VALID | spec + source |
| A2-12 | Chrome onto `--text-*` scale | VALID (one row wrong) | spec + source |
| A3-1 | `set-header` feature gate | **RESOLVED** (`f718f26`) | — |
| A3-2 | `BindMode::description` test | **RESOLVED** (`f718f26`) | — |
| A3-3 | Contrast count 6→7 | RESOLVED, then **obsoleted** by `0cdbbea` | spec (new work) |
| A3-4 | CI commands verbatim in §5 | VALID, never applied | spec |
| A3-5 | `releases.rs` in README tree | PARTLY APPLIED | spec |
| A3-6 | CSS animation citations | SUPERSEDED again | spec |
| B3-1 | `--text-2xs` fabricated | VALID | spec |
| B3-2 | Missing empty state | VALID | source |
| B3-3 | External-link cue | VALID | source |
| B3-4 | Rendered-HTML anti-overclaim test | VALID | source |
| B3-5 | Nav link `:26` | **NOT REAL** | do not apply |
| B3-6 | Card transition `:744` | **NOT REAL** | do not apply |
| B3-7 | `evidence` field + status test | VALID (optional) | source |
