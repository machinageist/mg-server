# Spec: Portfolio

**Feature ID:** B3-portfolio
**Parent feature:** root
**Spec author agent:** spec-agent-6 (portfolio)
**Date:** 2026-08-08
**Iteration:** 1

---

## 0. Reading notes and scope boundary

This spec covers the `/portfolio` route only: the `Project` / `ProjectStatus`
data model (`src/models/project.rs`), its handler (`src/handlers/pages.rs:98-126`),
its template (`templates/portfolio.html`), the CSS component block
(`static/css/style.css:1211-1266`, plus the shared `.tag`, `.section-intro`, and
`.project-card:hover` rules), and the anti-overclaim test that pins the list to
one entry (`src/models/project.rs:92-116`).

Two foundation specs constrain everything below and are treated as fixed inputs,
never re-litigated:

- **A1 design-system** — token architecture (Layer 1 colour/font, Layer 2
  measurement, Layer 3 components with zero literals), the type scale, and the
  ruling that `--accent-border` is decorative-tint-only and its use on
  `.status-active` is compliant because that pill is non-interactive and carries
  its meaning in text (A1 §4.2, lines 477-482).
- **A2 site-shell** — `base.html`, the `{% block content %}` slot, the 900px
  `<main>` column capped at `--measure: 72ch` for prose, the `section()` key that
  drives nav active-state, and the `title()`/`description()` metadata pattern.

The **progress** feature (C4, `src/models/lab.rs`) is explicitly *not* portfolio.
The boundary between the two is a load-bearing claim-integrity decision restated
in §1.2 and §6: work in progress lives on the progress surface; the portfolio
carries only entries with verifiable status and evidence.

The site is server-rendered Rust/Axum + Askama with a strict CSP
(`default-src 'self'`) and an ~80-line JS floor (theme selector only). This page
uses **zero** JavaScript and must stay that way.

---

## 1. Purpose

### 1.1 One-sentence job
Give a reviewer a short, scannable list of the things the owner actually builds
and operates, where every entry can be defended in an interview against the
site's evidence standard.

### 1.2 Why it matters
The site's core asset is that everything on it is true (criteria Lens 1). The
portfolio is the page most likely to be read as a set of *claims*, so it is the
page where overclaiming costs the most. Its answer to that risk is subtraction,
not addition: it lists one entry — `mg-server`, the service you are looking at —
because that is the only project that currently clears the bar of *verifiable
status + real evidence* (`src/models/project.rs:74-86`). The homelab, cert-track,
and GeistScope entries that once lived here were pulled to `content/drafts/portfolio-entries.md`
on 2026-07-25 and wait there for a long-form rewrite; restoring any of them
before it has evidence would trip the anti-overclaim guard
(`src/models/project.rs:92-116`).

This deliberate one-entry minimalism *is* the feature's differentiator. Junior
homelab and cert-track portfolios list fifteen tutorials and lose credibility on
the first one a reviewer probes (criteria Lens 4 competitor table). A one-item
list that is entirely true reads as judgment, and judgment is the thing neither
competitor group demonstrates.

### 1.3 Success signal
`cargo test` keeps
`portfolio_only_carries_entries_with_verifiable_status_and_evidence`
(`src/models/project.rs:92`) green: the list has exactly one entry, it is
`mg-server`, its status is `Active`, it has a real URL, and none of the archived
forbidden strings (`Homelab`, `GeistScope`, `Certification track`, `bug-bounty`,
`red-team`, `offensive security`) reappear. Observationally: a reviewer reaching
`/portfolio` can, within thirty seconds, name what the owner operates
(a self-hosted Rust service on owned infrastructure) and follow the entry to real
evidence.

---

## 2. User Stories

> As a **hiring manager skimming for thirty seconds**, I want one glance at
> `/portfolio` to tell me what this person actually operates, so that I can
> decide whether to read deeper without wading through a list of tutorials.

> As an **engineer peer evaluating rigor**, I want each entry to link out to real
> evidence (a repo and/or a writeup with commands and verification), so that I
> can check the claim rather than take it on faith.

> As the **site owner adding a second project**, I want the act of publishing an
> entry to *require* that its status is verifiable and its evidence exists, so
> that I cannot accidentally ship an aspirational claim.

> As a **screen-reader user**, I want each entry announced as a titled list item
> with its status read as a word (not inferred from colour), so that I get the
> same information a sighted reader gets.

> As a **visitor with JavaScript disabled**, I want the full portfolio — every
> entry, description, tag, status, and link — to render and function, so that the
> page is not a blank or broken shell.

> As a **visitor on a narrow phone**, I want the entry header (name + status
> pill) to stack instead of overflowing, so that nothing is clipped or forces
> horizontal scroll.

> As a **reviewer arriving from the nav**, I want "Portfolio" marked active in
> the header, so that I know where I am in the site.

---

## 3. UX Specification

### 3.1 Screen / view inventory

| Name | Path | New/Mod | Layout pattern |
|---|---|---|---|
| Portfolio index | `/portfolio` (nav: Portfolio) | Existing (`templates/portfolio.html`) | Single-column list inside the shared 900px `<main>` column (A2 §3.3) |

No modals, sheets, drawers, or popovers. The page introduces no new shell
chrome; it fills `base.html`'s `{% block content %}` (`templates/portfolio.html:3`).
There is no detail view — each entry's "detail" is the external link it carries.

### 3.2 Interaction flows

**Primary flow (target = current):**
1. Visitor clicks "Portfolio" in the header nav (`base.html:25`).
2. Server routes `GET /portfolio` → `pages::portfolio` (`src/router.rs:39`).
3. Handler calls `project::all()` and renders `PortfolioTemplate`
   (`src/handlers/pages.rs:122-126`).
4. Page renders: `<h1>Portfolio</h1>`, a one-paragraph `.section-intro`, then a
   `.project-list` `<ul>` of `.project-card` items.
5. Each card shows name, a status pill, a description, and a tag row.
6. Visitor clicks the entry name → external repo opens in a new tab
   (`target="_blank" rel="noopener noreferrer"`, `templates/portfolio.html:17`).

**Branch — entry with no link** (`url: None`): the name renders as plain,
non-interactive text (`templates/portfolio.html:16-20`). No dead link, no empty
`href`. Meaning of "no link" is carried by absence of link affordance plus, in
target state, the status word (an entry with no URL should not be `Active`).

**Branch — empty list** (`all()` returns `[]`): see §3.3 empty state. Not
reachable today because `all()` is hardcoded non-empty, but the template must
render a designed empty state rather than a heading over a blank `<ul>`
(criterion 3F). This is a **gap** — see §7.

No haptics, no sound, no page-transition animation (full document load, A2 §3.5).

### 3.3 Layout descriptions

Component hierarchy, top → bottom (`templates/portfolio.html:4-33`):

```
<main id="content">                         900px column, centred (A2 §3.3)
  └─ <section>
       ├─ <h1> "Portfolio"                   --text-2xl, --text, 700 (style.css:773-780)
       ├─ <p class="section-intro">          --text-muted, one sentence, ≤ measure
       └─ <ul class="project-list">          list-style:none (style.css:1211-1213)
            └─ <li class="project-card">     for project in projects
                 ├─ .project-header          flex, space-between, baseline, gap 1rem
                 │    ├─ .project-name        700; <a> if url, else plain text
                 │    └─ .project-status      bordered pill, text = status word
                 ├─ .project-description      --text-muted, line-height 1.7, max 72ch
                 └─ .project-tags             flex-wrap row of .tag pills
```

- **Data source:** the entire list is `project::all()` (`src/models/project.rs:76`),
  passed as `PortfolioTemplate.projects: Vec<Project>` (`src/handlers/pages.rs:103-106`).
  Each field maps 1:1 to a `Project` struct field
  (`src/models/project.rs:21-30`): `name`, `description`, `tags`, `url`,
  `status` (rendered via `Display`) and `status_class()` for the CSS class.
- **Section intro copy (current):** "Things I build and run, listed once there's
  a verifiable status and evidence behind them." (`templates/portfolio.html:6-9`).
  This copy *states the page's own claim discipline in plain language* — it is
  the one place where naming the standard is appropriate, because it is a promise
  about the list, not strategy narration about the owner (contrast the home/about
  no-meta-copy tests, `src/handlers/pages.rs:159-166, 214-222`).
- **Status pill:** three classes exist —
  `.status-active` (accent text, `--accent-border` tint),
  `.status-in-progress` (`--text`, `--border`),
  `.status-complete` (`--text-faint`, `--border-subtle`)
  (`static/css/style.css:1264-1266`). Only `active` renders today; the other two
  are wired end-to-end (enum → `Display` → CSS) but unused, marked
  `#[allow(dead_code)]` (`src/models/project.rs:41-44`). State: **prototyped**
  (reachable the moment a second entry uses them; not currently rendered).
- **Empty state (target, GAP):** if `projects` is empty, render the `<h1>` and a
  designed line — e.g. "No entries meet the evidence bar yet." — instead of a
  bare `<ul>`. Copy must not imply forthcoming work as if shipped (Lens 1B).

### 3.4 Input & gestures
- **Pointer/keyboard:** two interactive targets per linked entry region — the
  entry name link and any tag (tags are non-interactive today; see §7). The
  status pill is not interactive.
- **Keyboard shortcuts:** none. The page adds no key handlers (consistent with
  A2 §3.4 declining app-level shortcuts).
- **Responsive:** at ≤ 640px the `.project-header` switches from a
  space-between row to a stacked column (`flex-direction: column; align-items:
  flex-start`, `static/css/style.css:1554-1559`) so the name and status pill do
  not collide; the page inherits the shell's narrow-viewport padding (A2 §3.3).
  `.project-tags` already wraps (`flex-wrap: wrap`, `style.css:1254`). The
  `.project-description` is capped at `max-width: 72ch` (`style.css:1248`) for a
  comfortable measure; on narrow screens it is width-constrained by the column.

### 3.5 Transitions & animation
- **Card hover:** background → `--surface` plus a 2px inset accent bar
  (`static/css/style.css:730-733`), transitioned at 0.2s
  (`style.css:745`). This is chrome-level, not body motion (criterion 2E).
- **Reduced motion:** the hover *transition* lives inside
  `@media (prefers-reduced-motion: no-preference)` (`style.css:735-748`); under
  `reduce` the hover state still applies but changes instantly. No autoplay, no
  looping, no body-content animation on this page (criterion 3E).
- **Navigation:** full document load, no view transition (A2 §3.5).

### 3.6 Error states

| Trigger | Presentation | Recovery | Data-loss |
|---|---|---|---|
| Route/handler infallible today (`all()` returns an owned `Vec`, no I/O) | N/A — no runtime error path exists on this page | — | No |
| Empty `all()` (future editing mistake) | Designed empty state (§3.3), not a blank list | Add an entry | No |
| Entry with `url: None` | Name renders as plain text, no broken link (`portfolio.html:16-20`) | N/A (intended) | No |
| Server-level 500 (upstream) | Themed 500 page from the shell (A2), not this template | Retry | No |

The portfolio has **no I/O and no fallible parsing** — unlike the blog/learn
pages, its data is compile-time-embedded `&'static str` (`src/models/project.rs:24-28`,
notes 10-15). This is a deliberate reliability property: the page cannot 500 on
content-loading failure because there is no content to load. The spec preserves
it — do not migrate projects to file-loaded content without a stated reason.

### 3.7 Accessibility
- **Structure:** one `<h1>`, a real `<ul>`/`<li>` list. The status pill is a
  `<span>` whose text *is* the state word ("active" / "in progress" /
  "complete"), so state is communicated by text, not hue — satisfying criterion
  3B and the A1 ruling on `.status-active` (A1 §4.2, lines 480-482). No `aria`
  overrides are needed because the semantics are already correct.
- **Links:** entry-name links carry visible text (the project name) and open in a
  new tab with `rel="noopener noreferrer"` (`portfolio.html:17`). **Gap:** a
  new-tab link should signal that it opens externally to screen-reader and
  sighted users (e.g. an `aria-label` suffix or a visible "↗" cue); today it does
  not. Flagged in §7.
- **Focus:** entry-name links receive the shell's global focus ring
  (A1/A2 focus token, `--focus-width`/`--focus-offset`); focus order is DOM order
  (name before tags, top entry first). No focus is removed anywhere on this page
  (criterion 3C, auto-fail rule 2 respected).
- **Contrast:** all text/background pairs use A1 Layer-1 tokens audited at their
  usage size (A1 §4.2 `USAGE` matrix). The `.project-status` pill text at 0.72rem
  is *small text* and its colour must meet 4.5:1 in every theme; the pill
  *border* (`--accent-border`) is decorative and exempt because meaning is in the
  text (A1 lines 477-482). **Gap:** 0.72rem is below the A1 type-scale floor
  (`--text-2xs: 0.70rem` exists but `.project-status` and `.tag` hardcode
  0.72rem, off-scale) — §7 folds this into the A1 tokenization dependency.
- **Reduced motion / text scaling / reflow:** inherited from the shell (A2
  §3.3-3.5). At 400% zoom / 320px the stacked header (§3.4) keeps the page free of
  horizontal scroll — verify per §5.4.

---

## 4. Implementation Specification

### 4.1 Architecture placement
- **Route:** `src/router.rs:39` — `.route("/portfolio", get(pages::portfolio))`.
  `router.rs` is the single source of truth for routes (its own header comment,
  lines 3-4).
- **Handler + template struct:** `src/handlers/pages.rs:98-126`
  (`PortfolioTemplate`, `title()/description()/section()`, `portfolio()`).
- **Data model:** `src/models/project.rs` (`Project`, `ProjectStatus`, `all()`).
- **Template:** `templates/portfolio.html`, extending `base.html`.
- **Styles:** `static/css/style.css:1211-1266` (component block) plus shared
  `.section-intro` (1198-1203), `.tag` (1080-1086), `.project-card:hover`
  (730-733), and the ≤640px header-stacking rule (1554-1559).

The feature owns no module of its own; it is one route within `pages.rs`
alongside `home` and `about`, matching the shipped structure (README project
tree, `src/handlers/pages.rs:1-8`).

### 4.2 Data model

Shipped types (`src/models/project.rs:21-66`), reproduced with their contract:

```rust
/// One portfolio entry. All fields are compile-time &'static str so the list
/// has zero runtime allocation and every field is compiler-validated at build.
#[derive(Debug, Clone)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],   // zero-allocation fixed slice
    pub url: Option<&'static str>,        // None = renders without a link
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus { Active, InProgress, Complete }
// InProgress / Complete are #[allow(dead_code)]: wired but unused while the
// list carries a single Active entry.
```

`ProjectStatus` implements both `class_name()` (CSS hook: `active` /
`in-progress` / `complete`) and `Display` (rendered word: `active` /
`in progress` / `complete`), so the template writes `{{ project.status }}` and
`status-{{ project.status_class() }}` directly (`portfolio.html:22`). Adding a
variant without updating both matches is a compile error (model notes, lines
13-14) — a real drift guard (criterion 5B).

**No database, no migration.** Entries are edited in `all()`
(`src/models/project.rs:76-86`); the compiler is the schema.

**Target enhancement (proposed, not shipped — see §7 / §8 Q1):** add
`pub evidence: Option<&'static str>` so an entry can point at its on-site
writeup (a `/blog/:slug` report meeting the §6.4 evidence standard) *in addition*
to `url` (the repo). This mirrors the existing `Lab.writeup_url: Option<&'static str>`
pattern already in the repo (`src/models/lab.rs:33-34`, "None until Completed —
set once a real writeup is published"), so it is not a new idea, and it keeps the
deep evidence in one place (the blog post) while the card stays a pointer
(criterion 5A). Adopting it is a small, additive change; it is proposed, not
assumed.

### 4.3 API contracts

| Item | Signature | Errors | Auth | Notes |
|---|---|---|---|---|
| Route | `GET /portfolio` → `200 text/html` | none (infallible) | public | `src/router.rs:39` |
| Handler | `pub async fn portfolio() -> impl IntoResponse` | none | — | `src/handlers/pages.rs:122-126`; askama_axum renders the struct to a 200 |
| Data fn | `pub fn all() -> Vec<Project>` | none | — | `src/models/project.rs:76`; returns an owned `Vec` cloned from static data on each request |
| Metadata | `title()="Portfolio — machinageist"`, `description()="What machinageist builds and runs — verified, evidenced work only."`, `section()="portfolio"` | — | — | `src/handlers/pages.rs:110-118`; `section()` drives nav active-state (`base.html:25`) |

No pagination, no rate-specific behaviour beyond the global limiter (A3). No
query parameters. The page is fully deterministic given the compiled binary.

### 4.4 State management
- **Owner:** none beyond the request. `all()` returns freshly-cloned static data
  per request; there is no shared mutable state, no store, no session
  (`src/models/project.rs:24-28` notes). `AppState` (vitals/rate-limit) is not
  read by this handler.
- **Local vs. server-synced:** N/A — server-rendered, stateless. There is no
  client state to manage (no JS on this page).
- **Offline / draft persistence:** N/A — no user-authored data. (Owner-side
  "drafts" of *entries* live as prose in `content/drafts/portfolio-entries.md`,
  outside the app; they are not loaded, routed, or scanned — see
  `content/drafts/README.md`.)

### 4.5 Dependencies
- **Packages:** none new. Uses the already-present `askama` / `askama_axum`
  (`src/handlers/pages.rs:19-20`) and `axum` routing. No serde, no I/O crate.
- **Assets:** none. No images, no fonts beyond the shell.
- **Infrastructure:** none. No database, CDN, or third-party service.
- **Design tokens:** depends on A1's Layer-1/Layer-2 token set (colours + type
  scale + spacing). Any new visual rule must resolve to A1 tokens, not literals
  (criterion 2F, 5A). This is the one real cross-feature dependency.

### 4.6 Platform-specific considerations
- **No-JS / CSP:** the page has zero JavaScript and needs none; it is unaffected
  by `default-src 'self'`. This is a hard constraint, not a nice-to-have (auto-
  fail rule 3). Any future enhancement (e.g. tag filtering) must be a progressive
  layer over a working server-rendered baseline (real URLs / form GET), never the
  only path.
- **Browser support:** flexbox + CSS custom properties only; no modern-only APIs.
  Works back to any evergreen browser and degrades gracefully in older ones (list
  still readable without the flex header layout).
- **Feature flags / rollout:** N/A — one static page.

### 4.7 Performance budget
- **Payload:** one small HTML document (currently a single `<li>`) plus the
  shared `style.css` (cached, versioned query string, `base.html:13`). No page-
  specific CSS file, no JS, no images. Well under any reasonable budget.
- **Memory / CPU:** `all()` allocates one `Vec` of one `Project` per request;
  the `&'static str` fields are not copied (model notes, lines 10-12). Render is a
  compile-time-checked Askama template — microseconds.
- **Storage:** zero client storage; zero server storage (data is in the binary).
- **Startup:** no startup cost — no content directory scan for this feature
  (contrast blog/learn which read `content/` at request time).

---

## 5. Test Specification

### 5.1 Unit tests

**Shipped (must stay green, criterion 1F):**
`portfolio_only_carries_entries_with_verifiable_status_and_evidence`
(`src/models/project.rs:92-116`):
- Setup: call `all()`.
- Asserts: `len() == 1`; `[0].name == "mg-server"`; `[0].status == Active`;
  `[0].url.is_some()`; and the joined `name + description` of all entries
  contains **none** of `Homelab`, `GeistScope`, `Certification track`,
  `bug-bounty`, `red-team`, `offensive security`.
- Edge covered: silent regrowth of the list or reintroduction of an archived,
  unevidenced claim. This is the encoded claim boundary; the spec **satisfies it,
  never weakens it**. Relaxing `len() == 1` requires an explicit recorded
  decision (§7.4, §8 Q1), not a quiet edit.

**Target additions:**
1. `status_display_and_class_agree` — for each `ProjectStatus` variant, assert
   `Display` string and `class_name()` are the documented pair
   (`active`/`active`, `in progress`/`in-progress`, `complete`/`complete`).
   Guards the two-match drift the model comment warns about
   (`src/models/project.rs:13-14`, criterion 5B). Setup: iterate the three
   variants. This test also legitimises the `#[allow(dead_code)]` variants by
   exercising them, so the allow can eventually be reconsidered.
2. `entry_with_no_url_is_not_active` (invariant test) — assert every entry with
   `url == None` has `status != Active`, encoding "active means there's something
   to point at." Only add if the owner confirms this invariant (§8 Q2).

### 5.2 Integration tests
`portfolio_page_renders_and_carries_no_forbidden_claims` (new, in
`src/handlers/pages.rs` `#[cfg(test)]`, matching the home/about render tests at
`pages.rs:146-223`):
- Setup: `PortfolioTemplate { projects: project::all() }.render()`.
- Assert: `Ok`; HTML contains `"Portfolio"`, `"mg-server"`, and the repo URL;
  HTML does **not** contain the forbidden strings from §5.1.
- Why it matters (criterion 5C — no hidden coupling): the shipped anti-overclaim
  test asserts against the *model* (`name + description`), so an overclaim
  introduced only in the *template* (e.g. a hardcoded blurb) would slip past it.
  Asserting on rendered HTML closes that gap and keeps the test's name honest
  about what it checks.

### 5.3 UI / E2E tests
No browser E2E harness exists in-repo (consistent with A1/A2, which rely on
`render()` + manual verification). Target E2E, if a harness is added later:
navigate to `/portfolio`, assert the nav "Portfolio" item has `is-active`, assert
the entry link opens the repo, and re-run with JS disabled to confirm identical
output (auto-fail rule 3). Until then this is **manual** (§5.4).

### 5.4 Visual / manual verification
- **Themes:** render `/portfolio` in all 23 themes; confirm the `.status-active`
  pill text meets 4.5:1 and the pill is legible on both `--bg` and, on hover,
  `--surface` (criterion 2F). Spot-check the light themes (Light, Paper, Dawn,
  Cloud, Blueprint, Steampunk) where accent contrast is tightest.
- **JS disabled:** load `/portfolio` with scripting off; confirm the full list,
  link, and hover-less layout all work (auto-fail rule 3).
- **Reduced motion:** enable `prefers-reduced-motion: reduce`; confirm the card
  hover applies instantly with no transition.
- **Reflow:** 320px width at 400% zoom; confirm the header stacks (§3.4) and
  there is no horizontal page scroll.
- **Empty vs populated:** temporarily return `vec![]` from `all()` in a scratch
  build and confirm the empty state (once built, §7) is designed, not a bare
  heading. Revert before commit.
- **Verification commands (criterion 5D, run in CI):**
  `cargo fmt --all -- --check`,
  `cargo clippy --all-targets -- -D warnings`,
  `cargo test --all-targets` (includes the anti-overclaim test),
  `cargo build --release`.

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification
- [x] **No sensitive data involvement.** The page displays only owner-authored,
  public project descriptions embedded in the binary. No user input, no PII, no
  secrets, no request-derived data. The repo URL is intentionally public.

### 6.2 Asset provenance
- [x] **No third-party assets.** No images, models, data files, or fonts beyond
  the shell's system font stack (A1). Tags and text are original owner content.

### 6.3 Language / claims audit
- [x] Makes no claim unsupported by evidence — the single entry is the running
  service itself, and its status (`Active`) and URL are both verifiable.
- [x] Promises no unbuilt capability — `InProgress`/`Complete` styling exists but
  is not asserted as describing any current project; the section-intro copy
  ("listed once there's a verifiable status and evidence") *is the discipline*,
  not a promise of forthcoming entries.
- [x] Uses no domain-restricted language — no cert claim (none is booked; auto-
  fail rule 1), no offensive-security/red-team identity, no
  senior/SRE/production-grade framing (Lens 1E). The tags on the live entry
  (`rust`, `axum`, `linux-service`, `self-hosting`, `headers`,
  `src/models/project.rs:82`) describe capabilities, not credentials — exactly
  what `content/drafts/portfolio-entries.md` (lines 15-17) instructs.

**Copy-currency note (criterion 1D):** the portfolio page carries **no cert
copy**, so it is not affected by the stale "working through the CompTIA stack"
line that lives in `about`/`home` (`src/handlers/pages.rs:44, 91-94`). B3 must
not *introduce* cert copy; the live spine (RHCSA → CCNA → Security+, re-locked
2026-08-02) is internal and unpublished (`docs/public-portfolio-structure.md`
amendment, lines 3-13). No auto-fail rule 1 exposure here.

### 6.4 Regulatory alignment — criteria Lens 3
- **1A Evidence standard:** the live entry can answer every field of the standard
  (`docs/public-portfolio-structure.md` §76-92): *why it matters* (a self-hosted
  Rust service is the target-role artifact), *starting/target state* and *tools*
  (Rust/Axum/Askama/Caddy/Cloudflare Tunnel on a Proxmox Debian VM, the entry
  description itself), *real evidence, what broke, verification* (the two
  published reports it maps to — "How machinageist.dev Is Hosted" and "Security
  Headers on machinageist.dev", README lines 130-156), and *what's still unknown*
  (the README's explicit "not production-grade / not 'secured'" boundary, lines
  27-30). Any *future* entry must have this same path before it appears — the
  proposed `evidence` field (§4.2) makes the mapping first-class. A card without a
  path to these fields must not ship (criterion 1A scores ≤1 otherwise).
- **1B State honesty:** implemented (mg-server card) vs. prototyped (unused status
  styling) vs. archived/absent (drafts entries) are kept distinct in this spec and
  never let a planned entry read as shipped.
- **1C Publication gate:** GeistScope is barred from this page by name in the
  anti-overclaim test (`src/models/project.rs:113`); it may appear only on a
  progress surface (C4) as *progress*, never as a portfolio claim, and only enters
  the portfolio after the full pipeline + human/AI operation + sanitized
  authorized-engagement evidence gate. This spec upholds that.
- **3A No-JS floor:** satisfied — the page is pure server-rendered HTML (§4.6).
- **3B/3C/3E:** contrast via A1 tokens, focus never removed, motion behind
  `prefers-reduced-motion` (§3.7, §3.5).

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

| Piece | State | Evidence |
|---|---|---|
| `/portfolio` route + handler + metadata | **implemented** | `src/router.rs:39`, `src/handlers/pages.rs:98-126` |
| `Project` / `ProjectStatus` model + `all()` | **implemented** | `src/models/project.rs:21-86` |
| Template (list, status pill, tags, optional link) | **implemented** | `templates/portfolio.html:1-34` |
| Component CSS + hover + responsive header | **implemented** | `static/css/style.css:730-733, 1211-1266, 1554-1559` |
| Anti-overclaim test pinning `len()==1` | **implemented** | `src/models/project.rs:92-116` |
| `InProgress` / `Complete` status styling | **prototyped** (wired, unused) | `src/models/project.rs:41-44`, `style.css:1265-1266` |
| Archived homelab/cert/GeistScope entries | **absent** from site (parked) | `content/drafts/portfolio-entries.md` |
| On-site evidence link per entry (`evidence` field) | **absent** | proposed §4.2 |
| Designed empty state | **absent** | template has no `{% else %}` on the loop |
| Template-level render / anti-overclaim test | **absent** | `pages.rs` tests cover home/about only (`pages.rs:146-223`) |
| External-link screen-reader cue | **absent** | `portfolio.html:17` |

### 7.2 Delta to spec

Target state is *not* "more entries." Target state is: the current one-entry list,
plus the guards and affordances that make growth safe and each entry fully
evidence-mappable. Itemized:

- **Modify `templates/portfolio.html`:** add a designed empty state
  (`{% else %}` branch on the `for` loop) with honest copy; add an external-link
  cue (visible "↗" and/or `aria-label`) to the entry-name link.
- **Modify `src/handlers/pages.rs`:** add the template render test (§5.2).
- **Modify `src/models/project.rs`:** add `status_display_and_class_agree` test
  (§5.1); *optionally* add the `evidence: Option<&'static str>` field (§4.2, §8
  Q1) and, if adopted, the `entry_with_no_url_is_not_active` invariant (§8 Q2).
- **CSS (folds into A1 tokenization):** replace off-scale literals
  (`.project-status`/`.tag` `0.72rem`; `.section-intro` `0.875rem`/`65ch`;
  `.project-name` `0.95rem`; `.project-description` `0.85rem`/`72ch`) with A1
  Layer-2 tokens (`--text-xs`/`--text-2xs`, `--text-md`, `--text-sm`,
  `--measure`, `--measure-narrow`). This is required by A1's zero-literal lint
  (A1 §4.2 Layer 3, §5.1) and is best done in the A1 sweep, not independently, so
  the values stay single-sourced (criterion 5A/2F).
- **New files:** none required. (If the render test grows large it stays in
  `pages.rs`'s existing `#[cfg(test)]` module.)
- **Migrations / new dependencies:** none.
- **Documentation to update on behaviour change (criterion 5E):**
  `docs/public-portfolio-structure.md` "Still open" section (lines 133-136) —
  when an entry flips from archived to published, record it there; and
  `content/drafts/portfolio-entries.md` guidance if the growth workflow changes.

### 7.3 Estimated scope
**S.** The route, model, handler, template, and CSS all exist and pass. The
target work is: two small tests, an empty-state branch, an a11y link cue, and
participation in the A1 CSS-token sweep. The only *M*-sized item is the optional
`evidence` field, and only if the owner also wants a matching writeup published
for the live entry — which is content work, not code (§8 Q1).

### 7.4 Blocking dependencies
- **A1 (design-system)** — the token set and the zero-literal lint. B3's CSS
  cleanup must land inside or after A1's tokenization to avoid inventing values.
  Not blocking for the *behavioural* work (tests, empty state, a11y cue).
- **A2 (site-shell)** — the `<main>` column, `section()`/nav active-state, and
  metadata pattern. Already shipped; no wait.
- **Recorded-decision gate:** any change to `all()`'s length (adding a second
  entry) requires updating `portfolio_only_carries_entries_with_verifiable_status_and_evidence`
  in the *same* commit, with the decision noted (which project, what evidence
  cleared it). Growing the list without that is a criterion 1F zero. This is a
  process gate, not a code dependency.
- **B4 (writing)** — if the `evidence` field is adopted, its target is a
  `/blog/:slug` report; the strongest live targets already exist (the hosting and
  security-headers posts). No new writing is strictly required for the live entry.

---

## 8. Open Questions

- **Q1:** Adopt the `evidence: Option<&'static str>` field (mirroring
  `Lab.writeup_url`) so each portfolio card points at its on-site writeup in
  addition to its repo? — blocks: §4.2 data model, §5.1 tests, §7.2 delta.
  *Recommendation: yes, additively; it makes the criterion 1A evidence path
  first-class without adding entries.*
- **Q2:** Is "an `Active` entry must have a URL/evidence to point at" a hard
  invariant worth encoding as a test (`entry_with_no_url_is_not_active`)? — blocks:
  §5.1. *The shipped test already asserts the live entry's URL is `Some`, so the
  intent seems present; confirm it should generalise.*
- **Q3:** When (if ever) does a second entry land, and which archived project
  earns it first? The drafts note (lines 15-17) says anchor each to a *capability*
  and drop all cert wording — but the trigger (what evidence is "enough") is a
  judgment call the owner must make per project. — blocks: §7.4 recorded-decision
  gate; not blocking for shipping the current spec.
- **Q4:** Should tags become interactive (filter/link to a tag view) later? Today
  they are decorative `<span>`s. Any such feature must keep a no-JS, real-URL
  baseline (§4.6). — blocks: nothing now; noted so a future change doesn't violate
  auto-fail rule 3.

---

**Sub-feature note (per dispatch rules):** B3 is a leaf. No sub-features were
discovered that need separate agents. The `evidence`-field proposal touches B4
(writing) only as a link target, not as new scope for B4.
