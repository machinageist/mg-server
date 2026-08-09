# Spec: Learn (education wiki)

**Feature ID:** `B5` / `learn`
**Parent feature:** root (Content surfaces)
**Spec author agent:** spec-agent-8 (Claude Opus 4.8)
**Date:** 2026-08-08
**Iteration:** 1

---

## 0. Reading notes and scope boundary

Everything asserted about current state below was read from source, not from docs.
Citations are `path:line` or `path:line-range`.

**In scope (B5 owns):** the `/learn` overview and `/learn/:slug` topic routes; the
`/wiki` + `/wiki/:slug` legacy redirects; the hardcoded `SIDEBAR` allowlist and
the sidebar navigation UI; `content/pages/*.md` as the education corpus; the
`Page` model that parses them; `templates/wiki_page.html`; the wiki CSS
(`style.css:1379-1531`, `1028-1054`); the `tests/wiki_pages.rs` drift guards; and
— the criterion-2C centre of gravity — the *pedagogical contract* every learn
page must satisfy.

**Out of scope, referenced only:**

| Concern | Owner | What B5 assumes |
|---|---|---|
| Token architecture, type scale, contrast audit, `--text-*`/`--measure`, 23-theme roster | `A1` design-system | The prose surface (`.post-content`) and its measure/contrast are A1's; B5 consumes them and adds no colour literal |
| `base.html`, nav active-state mechanism, `Section` enum, `head_extra`/`scripts` blocks, `og:*` meta, skip link, footer | `A2` site-shell | B5's page renders through `base.html`; the "wiki" → `Section::Learn` fix and `og:type=article` are A2 cross-feature deps (§7.4) |
| Security headers / CSP, rate limiting, vitals | `A3` ops | CSP is `default-src 'self'; script-src 'self'; style-src 'self'` (`security_headers.rs:41-43`) — the no-JS floor is a hard constraint on this feature |
| Site-wide search over `content/pages/` | `C1` search | Learn pages are a search corpus; B5 keeps them crawlable and cross-linked |
| Glossary of terms / commands | `C2` | Adjacent education surface; B5 defers term/command definitions to it |

The `content/drafts/` tree (including `content/drafts/geist-wiki/`, which is
GeistScope project documentation, **not** learn content) is explicitly out of
scope per `feature-tree.md`. It is named in §4.4 only as the upstream of the
publish pipeline.

---

## 1. Purpose

### 1.1 One-sentence job

Give a curious reader a free, FOSS-first path from "I use this technology" to "I
understand, can practice, and can troubleshoot this technology" — one reviewed
topic at a time — while doubling as honest evidence that the author can *explain*
the systems foundations he is studying, not merely list them.

### 1.2 Why it matters

Three pressures meet on `/learn` and this feature is where they resolve.

1. **It is the site's strongest differentiator against both competitor groups.**
   The Lens 4 table says junior homelab portfolios rarely produce "writing that
   survives scrutiny" and cert-track candidates produce "little original
   explanation." `/learn` is the surface that answers both. The OSI page does not
   restate a study guide; it corrects the study guide — "Calling this 'error
   correction' hides the mechanism" (`osi-model.md:116-117`), "SSL is TLS's
   obsolete predecessor, not the name of a current protocol" (`osi-model.md:162-163`),
   "UDP is not always faster and is not limited to real-time traffic"
   (`osi-model.md:126-127`). Explanation that a working engineer would respect is
   the entire point (criterion 4C).

2. **It is a claim-integrity minefield if written carelessly.** Education copy is
   the easiest place on the site to accidentally assert unearned expertise. The
   corpus already handles this well — `index.md:70-79` states the authorship rule
   in plain language ("AI... is not used to manufacture unlearned expertise or
   unperformed evidence") and the pages lead with concepts, mark practice as
   *suggested*, and connect evidence only to owned work (`osi-model.md:257-260`
   links a real hosting walkthrough with captured `dig`/`curl` output). A spec
   that let a page claim operated experience it does not have would trip the Lens 1
   auto-fail (rule 1).

3. **It is a long-form reading surface with a no-JS floor.** Twelve topic pages
   today, 500–2,400 words each, growing. The reader's job is to read for twenty
   minutes and re-find material later. The whole feature — including the
   collapsible sidebar — works with JavaScript disabled because it is native HTML
   (`<details>`/`<summary>`, real `<a>` links, server-rendered Markdown). That is
   not incidental; it is the identity the site trades on (criterion 3A, auto-fail
   rule 3).

### 1.3 Success signal

**Primary (observable):** a reader who lands on any `/learn/:slug` from a search
result can, with JavaScript disabled, read the full page, see where they are in
the sidebar, open the collapsed sidebar on mobile, and follow a "Related pages"
link to a neighbouring topic — in a colour scheme that respects their OS
preference, with no control on screen that does nothing.

**Secondary (measurable):** `cargo test --all-targets` passes, including the two
`tests/wiki_pages.rs` drift guards **plus** the new SIDEBAR↔WIKI_SLUGS agreement
guard (§5.1 T-B5-4) and the new pedagogical-structure lint (§5.1 T-B5-5), on a
tree where every page in `SIDEBAR` resolves to a parseable file that carries the
required page structure.

---

## 2. User Stories

> **Happy path — self-directed learner.** As an adult who uses computers daily
> but wants to *understand* networking, I want each topic to start in ordinary
> language and build up to the jargon, so that I can follow it without a
> textbook and know what to practise next.

> **Happy path — engineer peer skimming for signal.** As a working engineer
> reading a resume link, I want the OSI page to show me the author knows where
> the model *breaks* (TLS crossing layers, "packet" used loosely), so that in
> thirty seconds I can tell this is understanding, not a memorised acronym.

> **Happy path — hiring manager.** As a hiring manager, I want the education
> pages to be tagged to the certifications the author is actually pursuing
> (RHCSA, CCNA, Security+) and to link to real documented work where a concept
> was applied, so that the study is legible as directed effort with evidence.

> **Edge case — unknown or retired slug.** As a visitor following a stale or
> mistyped `/learn/:slug`, I want a themed 404 that keeps the site navigation
> available, so that a dead link is a detour, not a dead end.

> **Edge case — legacy `/wiki` bookmark.** As someone who bookmarked `/wiki/osi-model`
> before the rename, I want it to permanently redirect to `/learn/osi-model`, so
> that my old link never breaks.

> **Accessibility — no JavaScript / keyboard / screen reader.** As a reader on a
> text browser or with JS blocked, I want the sidebar to open, every topic link
> to work, and the current page to be announced (not signalled by colour alone),
> so that the education material is fully reachable without scripts.

> **Maintainer.** As the person moving a reviewed note from my study workspace
> into `/learn`, I want one obvious place to register a new page and a test that
> fails loudly if I register it in some places but not others, so that I cannot
> ship a sidebar link to a missing file or a file with no sidebar link.

---

## 3. UX Specification

### 3.1 Screen / view inventory

| Surface | Path to reach | New / modified | Layout pattern |
|---|---|---|---|
| **Learn overview** | `/learn` (`router.rs:41` → `wiki::index`) | Modification | Two-column wiki grid: 13rem sidebar + article; article body is `content/pages/index.md` rendered |
| **Learn topic page** | `/learn/:slug` (`router.rs:42` → `wiki::page`) | Modification | Same wiki grid; article body is `content/pages/<slug>.md` |
| **Sidebar (wide)** | Inside both, left column | Modification | Sticky `<aside>`, always-expanded `<nav>`, section headings + link lists (`wiki_page.html:4-26`, `style.css:1390-1452`) |
| **Sidebar (≤800px)** | Inside both, above article | Modification | Collapsible `<details>` popover that floats over the page when open (`style.css:1456-1531`) |
| **Legacy `/wiki`** | `/wiki` (`router.rs:44`) | Modification | 308 redirect to `/learn`, no body |
| **Legacy `/wiki/:slug`** | `/wiki/:slug` (`router.rs:45`) | Modification | 308 redirect to `/learn/:slug`, no body |
| **Topic 404** | `/learn/<unknown>` | Modification (A2-owned) | Themed 404 (slug not in `SIDEBAR` → `PageNotFound` → `errors::fallback` chain) |

No modals, sheets, or drawers of B5's own. The mobile sidebar `<details>` is the
only overlay, it is non-modal, and it is native HTML — it needs no JavaScript to
open or close.

### 3.2 Interaction flows

**Flow A — read a topic (primary, JS-independent).**

1. Request `/learn/osi-model`. `wiki::page` (`wiki.rs:129-133`) calls
   `lookup_sidebar_slug` (`wiki.rs:157-166`) — a linear scan of `SIDEBAR`.
2. If the slug is in the allowlist it returns the `&'static str`; otherwise
   `SiteError::PageNotFound(slug)` → themed 404 (branch F below).
3. `render_for_slug` (`wiki.rs:136-144`) calls `Page::find` (`page.rs:72-78`),
   which parses the Markdown + YAML frontmatter and converts the body to HTML with
   `pulldown_cmark` `Options::all()` (`page.rs:58-60`).
4. `WikiPageTemplate` renders through `base.html`: full shell (header, nav,
   footer, vitals), sidebar with the active entry marked, article with back-link,
   title, date, tags, and the rendered body.
5. The reader scrolls; anchor targets clear the sticky header via
   `scroll-margin-top` (`style.css:1160`). At the bottom, "Related pages" and
   "Sources and further reading" (page-content convention, e.g.
   `osi-model.md:262-289`) carry them onward.

**Flow B — orient in the sidebar (wide, ≥800px).** The `<nav>` is always
expanded; the `<summary>` toggle is `display:none` (`style.css:1400-1402`). The
active entry shows accent text **and** a 2px accent left border where the others
are transparent (`style.css:1449-1452`) — a non-colour cue.

**Flow C — orient in the sidebar (narrow, ≤800px).** The sidebar collapses to a
`<details class="wiki-nav">` labelled "Wiki navigation" (`wiki_page.html:5-6`).
Tapping (or `Enter`/`Space` on) the `<summary>` opens a floating menu that
overlays the article rather than pushing it down (`style.css:1505-1531`). **This
is native `<details>` and requires no JavaScript.** The disclosure triangle
rotates on open (`style.css:1490-1498`).

**Flow D — legacy redirect.** `GET /wiki` → `wiki::redirect_index` →
`Redirect::permanent("/learn")` = **308** (`wiki.rs:147-149`, asserted
`wiki.rs:242`). `GET /wiki/osi-model` → `wiki::redirect_page` →
`Redirect::permanent("/learn/osi-model")` = 308 (`wiki.rs:152-154`, asserted
`wiki.rs:254`). Browsers follow transparently; the reader lands on the `/learn`
equivalent.

**Flow F — unknown slug.** `lookup_sidebar_slug` returns `None` →
`SiteError::PageNotFound` → the shell's themed 404 (A2 flow C). The header nav
(including the Learn link) is live on the 404, so recovery is not limited to the
back button.

**Cues.** No haptics, no sound. The only motion is the disclosure-triangle
rotation (§3.5), which must be reduced-motion-gated (currently is not — §7.1).

### 3.3 Layout descriptions

**Wiki grid** (`wiki_page.html:3`, `style.css:1379-1384`): CSS grid,
`grid-template-columns: 13rem minmax(0, 1fr)`, `gap: 3rem`, `align-items: start`.
The `main:has(.wiki-layout)` selector widens the column to `--layout-wide` (1200px,
A1 §3.3) so the sidebar has room; without `:has()` support it degrades to 900px —
narrower but fully usable (A1 §4.6 graceful-degradation rule).

**Sidebar** (`wiki_page.html:4-26`), top → bottom:

1. `<aside class="wiki-sidebar" aria-label="Education wiki navigation">` — sticky
   at `top: calc(var(--header-h) + 1rem)`, `max-height` bounded so it clears the
   footer, `overflow-y: auto`, 1px `--border` right rule (`style.css:1390-1397`).
2. `<details class="wiki-nav">` → `<summary>` (hidden ≥800px) → `<nav>` →
   per section: `<h2>` heading + `<ul>` of `<li><a>`.
3. Data source: the `SIDEBAR` const (`wiki.rs:35-99`) — three
   `SidebarSection`s: **Overview** (1 entry, links to `/learn`), **Networking
   Foundations** (11 entries), **Linux Foundations** (1 entry). The active entry
   is the `<li>` whose `entry.slug == active_slug` (`wiki_page.html:13`).

**Article** (`wiki_page.html:28-45`), top → bottom:

1. `<a class="back-link" href="/learn">` — reads "Education wiki", `::before`
   renders "← " (`style.css:1028-1037`).
2. `<header class="post-header">` — the single page `<h1>` (`page.title`), then
   `.post-meta`: `.post-date` and `.post-tags` (`page.tags` as `.tag` pills).
3. `<div class="post-content">` — `page.content_html|safe`, capped at
   `--measure` (72ch) for prose; `pre`/tables keep the full column and scroll
   (A1 §2B, `style.css:1124-1195`).

**Data sources.** Colour/font from the active `[data-theme]` block (A1); size,
spacing, measure, layout constants (`--header-h: 4.5rem`, `--footer-h: 7.25rem`,
`style.css:505-506`) from A1's measurement layer; page content from
`content/pages/<slug>.md`. No component reads a literal — **except** the orphaned
`.wiki-disclaimer` which hardcodes `#e0a458` and is fixed in §7.

**Empty states.** The overview `index.md` *is* the empty-adjacent state — it names
the small first release explicitly ("This is an intentionally small first
release", `index.md:66-67`), so the corpus never presents as thin by accident. A
section with zero entries would render a heading over an empty `<ul>`; the target
rule (mirroring A1 §3.3) is that a `SidebarSection` with no entries is omitted, not
rendered empty. Not currently possible (every section has ≥1 entry), pinned by
§5.1 so it stays impossible.

### 3.4 Input & gestures

- **Pointer.** Click on: sidebar entries, the mobile `<summary>` toggle, the
  back-link, in-body links (Related pages, Sources, cross-links), and tag pills
  (non-interactive today — see §7.4 / C2). The mobile floating menu closes by
  toggling `<summary>` again (native).
- **Keyboard.** Every interactive element is a real `<a>` or the native
  `<summary>`, so all are in the tab order and operable with `Enter`/`Space` with
  zero custom JS. Focus ring is A1's global `:focus-visible` (2px accent outline).
  No feature-specific shortcuts (they would trip WCAG 2.1.4).
- **Touch.** The mobile toggle and links are full-width tap targets; the
  `<summary>` and `<a>` padding give adequate hit area. No hover-only affordance
  carries information.
- **Responsive.** One breakpoint, 800px (`style.css:1456`): grid → single column,
  sidebar → static full-width block with a bottom rule, nav → collapsible
  floating `<details>`. Below that A2's 640px chrome breakpoint also applies.
- **Stylus / controller / voice / camera.** N/A — text and links only.

### 3.5 Transitions & animation

| Motion | Where | Duration | Guarded? |
|---|---|---|---|
| Disclosure-triangle rotation on sidebar open (≤800px) | `.wiki-nav > summary::before` | 0.15s | ❌ **unguarded** (`style.css:1493`) — **must move under `prefers-reduced-motion: no-preference`** (A1 test T10, §7.1) |
| Back-link / sidebar-link colour on hover | `.back-link`, `.wiki-sidebar li a` | inherits A2 chrome transitions | ✅ (A2 §3.5) |
| `<details>` open/close | native | UA default (none in most engines) | N/A |

**Rule (inherited from A1 §3.5).** Every `transition`/`animation` B5 introduces
lives inside `@media (prefers-reduced-motion: no-preference)`. There is exactly
one violation today (the triangle rotation) and fixing it is the whole of B5's
motion work. Reduced-motion alternative is *absence* — the triangle simply snaps;
the open/closed state is still carried by the native marker and the visible
expanded/collapsed nav.

### 3.6 Error states

| ID | Trigger | Presentation | Why | Recovery | Data loss |
|---|---|---|---|---|---|
| **E-01** | Slug not in `SIDEBAR` allowlist | **Full-page** themed 404 (`wiki.rs:131` → `PageNotFound`) | A wrong URL is a navigation event; the shell keeps nav live (A2 E-01). Allowlist-first means an un-curated `.md` on disk is *not* servable — 404, not a raw dump | Header nav (Learn included) + 404 home link | No |
| **E-02** | Slug in `SIDEBAR` but file missing on disk | **Full-page** themed 404 via `Page::find` (`page.rs:74-75` → `PageNotFound`) | Same; but this state means SIDEBAR and disk drifted — it must be caught in CI, not in prod (§5.1 T-B5-4) | Header nav | No |
| **E-03** | File present but frontmatter missing/malformed, or date unparseable | **Full-page** themed 500 (`page.rs:52,54,56` → `MissingFrontmatter`/`FrontmatterParse`/`DateParse`) | A malformed curated page is an author error, not a visitor error; the 500 leaks nothing (A2 E-02). Caught in CI by the parseable-page guard (`wiki_pages.rs:31-49`) before it ships | Fix the file | No |
| **E-04** | Legacy `/wiki*` URL | 308 permanent redirect, empty body | Correct HTTP semantics for a rename; search engines transfer link equity | Automatic | No |
| **E-05** | JS disabled | **No degradation.** Sidebar (`<details>`), links, and content are native HTML; theme follows OS preference (A1/A2) | The no-JS floor is the feature's identity | Read normally | No |

**Presentation justification.** Every error here is full-page (404/500) rather
than inline/banner/toast because the failure is "the resource does not exist / is
malformed," not a transient notice over otherwise-good content. No toast exists on
the site (A1 §3.6) and none is proposed — a toast needs JS to appear and dismiss,
which is behind the no-JS floor.

### 3.7 Accessibility

Graded as an auto-fail gate (rules 2 and 3). Written as invariants + shipped
state.

**A. Works without JavaScript (auto-fail rule 3).** ✅ **Met, and it is the
feature's headline property.** The sidebar is a native `<details>`/`<summary>`
(`wiki_page.html:5-6`); links are real `<a href>`; content is server-rendered
Markdown. Nothing in B5 requires or references JavaScript. Pinned by §5.1
T-B5-6 (strip `<script>`; assert every sidebar href and the content survive).

**B. Contrast & colour independence.**
- Sidebar section headings use `--text-faint` at **0.68rem** (`style.css:1424-1425`).
  This is small text and **violates A1's `--text-2xs` 0.70rem floor** and must
  clear 4.5:1 (A1 §3.7A). **Target:** restyle to `var(--text-2xs)` and inherit
  A1's audited `--text-faint`-at-4.5:1 contract.
- Sidebar links use `--text-muted` at 0.85rem (`style.css:1438-1441`) — inherits
  A1's audited `--text-muted` pair.
- **Active entry** carries a non-colour cue: the 2px accent left border where
  siblings are `transparent` (`style.css:1443,1449-1452`). ✅ colour-independent
  for sighted users.
- **Gap:** the active state is *not* exposed to assistive tech — it is carried
  only by the `li.active` CSS class (`wiki_page.html:13`). **Target:**
  `aria-current="page"` on the active `<a>` (A1 §3.7B target addition, A2 §3.7).

**C. Focus & keyboard.** All interactive elements are native and keyboard-operable
with A1's global visible focus ring. ✅. Focus order follows DOM order:
back-link → article links, and (in source order) the sidebar precedes the article.

**D. Semantics & heading outline.**
- Landmark: `<aside aria-label="Education wiki navigation">` wrapping a `<nav>`.
  ✅ but the aria-label says "Education wiki" while the nav elsewhere says "Learn"
  — a naming split (§7.1 G2).
- **Heading-order defect:** the sidebar section `<h2>`s (`wiki_page.html:10`)
  appear in DOM order **before** the article `<h1>` (`wiki_page.html:31`). A page
  should open with its `<h1>`; three `<h2>`s preceding it break the outline and a
  screen-reader "next heading" jump lands in navigation, not content. **Target:**
  demote the sidebar section labels from `<h2>` to non-heading styled elements
  (they live inside a `<nav>` landmark, so a styled `<p>`/`<span>` preserves the
  visual grouping without competing for the heading outline), OR keep them as
  headings only if the article `<h1>` is guaranteed to precede them in DOM order.
  Recommended: non-heading labels. Pinned by §5.1 T-B5-7.
- Inside `.post-content`, article headings are real, size-ordered `<h2>`/`<h3>`
  (A1 §3.7E, `style.css:1090-1104`). Author rule: one `<h1>` per page (the
  template supplies it from `page.title`), body uses `##`/`###` only — never a
  second `#`.

**E. Motion & sensory safety.** One unguarded transition (§3.5), fixed by moving
it under `no-preference`. No autoplay, no flashing, no body-content animation.

**F. Responsive & resilient.** Works 320px → wide (A2 breakpoints + the 800px
grid collapse). At 200% zoom / large browser font, the article prose reflows
(sizes are `rem`, A1 §3.7F); the sidebar stacks. Empty/degraded states are
designed (§3.3), not accidental.

---

## 4. Implementation Specification

### 4.1 Architecture placement

```
src/
  handlers/wiki.rs        ← SIDEBAR allowlist, WikiPageTemplate, index/page/redirect handlers,
                            lookup_sidebar_slug; section() returns "wiki" (→ A2 Section::Learn)
  models/page.rs          ← Page: frontmatter parse + Markdown→HTML
  router.rs               ← /learn, /learn/:slug, /wiki, /wiki/:slug (lines 41-45)
templates/
  wiki_page.html          ← wiki-layout grid, sidebar, article; adds aria-current, og:type via head_extra
static/css/style.css      ← .wiki-* rules (1379-1531), .back-link (1028-1037);
                            DELETE orphaned .wiki-disclaimer (1039-1054); guard the summary transition
content/pages/*.md        ← the corpus: index.md + 12 topic pages
tests/wiki_pages.rs       ← drift guards; gains SIDEBAR↔WIKI_SLUGS agreement + structure lint
docs/design/LEARN.md      ← NEW long-lived doc: the pedagogical page contract (§2C), publish pipeline
```

`src/handlers/wiki.rs` and the `wiki-*` CSS/module names are the retired "wiki"
naming. Renaming the module and CSS to `learn-*` is high-churn, low-value and
**not** proposed; instead the retired name is documented as legacy in `LEARN.md`
so it does not read as a live product name. The one name that *does* leak to a
user-visible drift hazard — `section() -> "wiki"` — is fixed via A2's `Section`
enum (§7.4).

### 4.2 Data model

**No new persistent types.** The feature is flat files + two existing structs.

`Frontmatter` (`page.rs:19-25`) — the per-page metadata contract:

```rust
#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,     // page <h1> and <title>
    date: String,      // "%Y-%m-%d", parsed to NaiveDate (page.rs:55)
    summary: String,   // <meta name="description">; 50–160 chars (A2 U-6)
    tags: Vec<String>, // .tag pills; cert-aligned (network-plus, rhcsa, ...)
}
```

`Page` (`page.rs:27-37`) — the parsed page. Two accuracy fixes:

- `pub summary` carries `#[allow(dead_code)]` (`page.rs:33`) but **is used** via
  `WikiPageTemplate::description()` (`wiki.rs:114-116`). The stale `allow` is
  misleading (Lens 5C — an annotation that lies about usage). **Target:** remove
  the `allow` on `summary`.
- `pub slug` is genuinely unused at read-time (`page.rs:29`); its `allow` is
  honest. Keep, or consume it in the SIDEBAR-agreement guard (§5.1).

**The SIDEBAR contract** (`wiki.rs:22-99`) — the single definition of what `/learn`
publishes, in display order:

```rust
pub struct SidebarEntry  { pub slug: &'static str, pub label: &'static str }
pub struct SidebarSection { pub heading: &'static str, pub entries: &'static [SidebarEntry] }
const SIDEBAR: &[SidebarSection] = &[ /* Overview(1), Networking(11), Linux(1) */ ];
```

`SIDEBAR` is both the **navigation source** and the **allowlist**:
`lookup_sidebar_slug` (`wiki.rs:157-166`) means only a slug present here is
servable at `/learn/:slug`. This is a deliberate curation + safety property — an
un-reviewed `.md` dropped into `content/pages/` is not reachable until it is added
to `SIDEBAR`. Document it as such.

**No database, no migrations.** The site has no persistence layer; content is read
from disk per request (`page.rs:47`).

### 4.3 API contracts

| Route | Handler | Returns | Errors | Auth |
|---|---|---|---|---|
| `GET /learn` | `wiki::index` (`wiki.rs:124-126`) | `WikiPageTemplate` (overview) | 500 if `index.md` malformed | none |
| `GET /learn/:slug` | `wiki::page` (`wiki.rs:129-133`) | `WikiPageTemplate` (topic) | 404 if slug ∉ SIDEBAR; 404 if file missing; 500 if malformed | none |
| `GET /wiki` | `wiki::redirect_index` (`wiki.rs:147-149`) | 308 → `/learn` | none | none |
| `GET /wiki/:slug` | `wiki::redirect_page` (`wiki.rs:152-154`) | 308 → `/learn/:slug` | none | none |

`:slug` matches exactly one path segment (`router.rs:42`, comment `router.rs:11`);
`ServeDir` traversal concerns do not apply because `Page::find` joins a
SIDEBAR-validated `&'static str`, never raw user input (`wiki.rs:138`,
`page.rs:73`). No pagination, no rate-limit specifics (A3's limiter applies
uniformly), no auth — the corpus is public by design.

**Template contract (A2 S-1).** `WikiPageTemplate` implements `title()` /
`description()` / `section()` (`wiki.rs:109-121`). Two changes:
- `section()` returns `&str` `"wiki"` today; **target:** `Section::Learn` (A2 dep).
- `title()` returns the bare page title (`wiki.rs:110-112`); A2 U-5 requires it to
  end in `" — machinageist"`. **Target:** align with A2's title contract.

**`head_extra` (A2 S-2).** `wiki_page.html` should fill `{% block head_extra %}`
with `<meta property="og:type" content="article">` so a shared `/learn` link
previews as an article, not the default `website` (`base.html:9`). Cross-feature
dep on A2 (§7.4).

### 4.4 State management

| State | Owner | Lifetime | Sync |
|---|---|---|---|
| Active slug | `WikiPageTemplate.active_slug` (`wiki.rs:106`) | Per request | Server only |
| Sidebar structure | `SIDEBAR` const (`wiki.rs:35`) | Compile-time static | Server only |
| Page content | `content/pages/<slug>.md`, read per request (`page.rs:47`) | Per request | Server only; no cache |
| Theme | `localStorage.theme` (A1/A2) | Browser | Client only |

**No new state container.** Content is read from disk on each request — acceptable
for a low-traffic personal site with ~13 small files; no cache is proposed (adding
one would be speculative complexity, and stale-content risk on a truth-first site
outweighs the microseconds saved).

**Publish pipeline (offline authoring).** New pages originate in Jeff's private
study workspace, are reviewed, then land as `content/pages/<slug>.md` + a
`SIDEBAR` entry + a `WIKI_SLUGS` entry, in one commit. `index.md:66-67` states
this ("It will expand as finished notes and completed labs move from my private
study workspace into reviewed public editions"). `content/drafts/` is the staging
area and is **not routed** (no handler reads it) — out of scope per feature-tree.

### 4.5 Dependencies

- **New packages:** none. `gray_matter` (YAML frontmatter) and `pulldown_cmark`
  (Markdown→HTML) are already in use (`page.rs:12-14`).
- **New assets:** none. No images shipped by learn pages; the corpus is text.
- **New doc:** `docs/design/LEARN.md` (the pedagogical contract, §5.5).
- **Infrastructure:** none. CSP `default-src 'self'` (`security_headers.rs:41`)
  forbids external images/scripts/fonts in page bodies — author rule: no CDN
  images, no embeds; diagrams are ASCII/`text` code blocks (as `osi-model.md:188-194`
  already does) or, later, self-hosted inline SVG (no JS).

### 4.6 Platform-specific considerations

- **`<details>::details-content`** override (`style.css:1413-1415`) forces the
  always-open wide layout in modern Chromium; without it the wide sidebar would
  render collapsed. Degradation elsewhere is graceful — an engine without
  `::details-content` still shows the nav because `.wiki-nav > nav { display:
  block }` (`style.css:1406-1408`).
- **`main:has(.wiki-layout)`** widening (A1 §4.6) degrades to 900px without
  `:has()` — narrower, still usable. No layout-critical feature is adopted.
- **`pulldown_cmark` `Options::all()`** (`page.rs:58`) enables tables, footnotes,
  strikethrough, task lists, etc. The corpus relies on tables heavily
  (`osi-model.md:33-41`). Keep.
- **`content_html|safe`** (`wiki_page.html:43`) renders unescaped HTML from
  Markdown. **Trust boundary:** the source is single-author, version-controlled
  flat files — not user input. This is safe *because* the corpus is trusted; the
  moment content becomes user-submitted (it will not, per this spec) it would need
  sanitization. Documented so the invariant is explicit, not accidental.
- **Feature flags / rollout:** N/A — single binary, single deploy.

### 4.7 Performance budget

| Dimension | Current | Target | Note |
|---|---|---|---|
| Largest page HTML | `transmission-media.md` 14.9 KB source (2,417 words) → ~30–40 KB rendered | ≤ ~60 KB rendered per page | Text; gzip at Caddy makes this trivial on the wire |
| Per-request work | 1 file read + gray_matter parse + pulldown-cmark render | Unchanged | ~13 small files; no measurable cost |
| CSS added by B5 | `.wiki-*` already in the one `style.css` | **−~350 B** (deleting `.wiki-disclaimer`) | Net reduction |
| JS added by B5 | **0 bytes** | **0 bytes** | The feature is native HTML end to end |
| Network requests added | 0 | 0 | No images, no fonts, no scripts |
| Client storage | 0 (theme is A1's) | 0 | |

**No caching layer proposed** (§4.4). The vitals strip on every page is a live
counter (A2 §4.7); learn pages inherit A3's HTML no-cache requirement — do not
introduce a per-page cache that would fossilise content or the strip.

---

## 5. Test Specification

All Rust tests run under `cargo test --all-targets` and gate CI (`fmt → clippy →
test → build --release`, criterion 5D).

### 5.1 Unit / integration tests

Existing (keep, cite as coverage):

| # | Name | Location | Guards |
|---|---|---|---|
| E-1 | `every_wiki_slug_has_a_parseable_page` | `wiki_pages.rs:31-49` | Every `WIKI_SLUGS` entry has a file starting with `---` and a `title:` |
| E-2 | `no_orphaned_wiki_pages_on_disk` | `wiki_pages.rs:53-71` | Every `.md` in `content/pages/` is in `WIKI_SLUGS` (no unlinked file) |
| E-3 | `rendering_overview_template_includes_sidebar_and_content` | `wiki.rs:173-202` | Overview renders sidebar, active class, "Understand → Practice → Evidence" framing |
| E-4 | `rendering_education_page_marks_correct_active_entry` | `wiki.rs:204-230` | Exactly one `class="active"`, wrapping the requested slug |
| E-5 | `unknown_slug_returns_none` | `wiki.rs:232-235` | Allowlist rejects unknown slugs |
| E-6 | `legacy_wiki_root_redirects_to_learn` / `legacy_wiki_slug_redirects_to_matching_learn_slug` | `wiki.rs:237-260` | 308 to `/learn` and `/learn/:slug` |

New (this spec):

| # | Name | Setup | Assertion | Edge case |
|---|---|---|---|---|
| **T-B5-1** | `every_sidebar_slug_resolves` | Iterate `SIDEBAR`, `Page::find` each | All parse; no `PageNotFound`/`Frontmatter*` | A SIDEBAR entry pointing at a missing/broken file (E-02/E-03 caught in CI, not prod) |
| **T-B5-2** | `active_entry_carries_aria_current` | Render a topic page | The active `<a>` has `aria-current="page"`; non-active links do not | The AT-invisible active state (§3.7B) |
| **T-B5-3** | `unknown_learn_slug_is_404` | `oneshot GET /learn/does-not-exist` | Status 404, body contains the themed 404 marker | Allowlist enforcement end-to-end |
| **T-B5-4** | `sidebar_and_wiki_slugs_agree` | Expose `SIDEBAR` slugs via a `pub fn sidebar_slugs() -> Vec<&'static str>` in `wiki.rs`; compare to a golden list | The set of SIDEBAR slugs equals the on-disk `.md` stems equals `WIKI_SLUGS` (the cross-crate copy) modulo none | **The direct SIDEBAR↔WIKI_SLUGS drift the existing pair does not catch** (§7.1 G4) |
| **T-B5-5** | `every_topic_page_has_the_required_structure` | For each non-index slug, read the raw Markdown | Contains an `## Overview` (or equivalent lede), a `## Suggested practice`/practice section, a `## Related pages` section, and a `## Sources` section | The 2C floor: a page that regresses to a bullet-dump of a source note |
| **T-B5-6** | `learn_pages_need_no_javascript` | `oneshot` each learn route, strip `<script>` | Every sidebar `href` and the article body survive | No-JS floor (auto-fail rule 3) |
| **T-B5-7** | `article_h1_precedes_any_h2` | Render a topic page | The first heading in the body is the article `<h1>` (no `<h2>` before it) | The sidebar-`<h2>`-before-`<h1>` outline defect (§3.7D) |

**On T-B5-4 and the deliberate duplication (criterion 5A).** `WIKI_SLUGS`
(`wiki_pages.rs:14-28`) intentionally duplicates `SIDEBAR` so the *test crate*
does not depend on the *bin* (`wiki_pages.rs:11-13`). That decoupling is
legitimate, but today **nothing asserts the two copies agree** — the existing
guards only tie each copy to disk independently. A slug added to `SIDEBAR` and to
disk but forgotten in `WIKI_SLUGS` (or vice-versa) drifts silently. T-B5-4 closes
it by having the bin emit its slug list through a `pub fn` the test can call,
turning the duplication into a *checked* duplication — this is the named guard
criterion 5A asks for.

### 5.2 Integration tests

Covered by T-B5-3 and T-B5-6 (router `oneshot`), plus A2's `every_route_renders_the_full_shell`
(A2 I-1) which already includes `/learn` and `/blog`.

### 5.3 UI / E2E tests

**Absent, deliberately.** There is no browser harness in the repo and the feature
has zero JavaScript, so the behaviours E2E would cover (sidebar open/close,
keyboard) are native HTML verified by the server-byte tests above (T-B5-6) and the
manual pass in §5.4. Adding Playwright to test native `<details>` would cost more
than it buys (mirrors A1 §5.3 / A2 §5.3).

### 5.4 Visual / manual verification

Per A1's tiered matrix (§5.4). Learn-specific surfaces on the Tier-1 six themes
(Lunarcore, Solarcore, Paper, Cloud, Solarized, CRT):

- `/learn` overview and `/learn/osi-model` (long page with tables + code block).
- Sidebar wide (≥800px, sticky, active border) and narrow (≤800px, `<details>`
  floating menu open/closed).
- 200% zoom / 24px browser font: prose reflows, sidebar stacks, no horizontal
  scroll at 320px.
- `prefers-reduced-motion: reduce`: the disclosure triangle snaps (no rotation).
- `prefers-color-scheme: light` with **JS disabled**: light palette, no dead
  theme control, sidebar and links fully operable.
- A `/learn/<unknown>` slug → themed 404 with live nav.

### 5.5 Documentation follows behavior (criterion 5E)

`docs/design/LEARN.md` (new) is updated in the same change as any structural
change and records: the Understand → Practice → Evidence page contract (§6.3/§2C),
the SIDEBAR/WIKI_SLUGS/disk three-way guard, the retired "wiki" naming, the
`content_html|safe` trust boundary, and the publish pipeline. README's
one-line `wiki.rs` description (`README.md:71`) is corrected from "archive index"
to the education-wiki role.

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement.** Learn stores nothing, reads no user
  input beyond a path segment validated against a static allowlist
  (`wiki.rs:130-131`), transmits no PII, sets no cookies. Content is public,
  author-written flat files.

### 6.2 Asset provenance

- [x] **Uses third-party assets — text references only, no files.**

| Asset | Source | Licence | Status |
|---|---|---|---|
| Standards cited in "Sources" (ISO/IEC 7498-1, RFC 1122/9293/768, etc., `osi-model.md:279-286`) | Standards bodies / IETF | Cited by hyperlink, not reproduced | Clear — reference, not redistribution |
| Markdown/HTML pipeline (`gray_matter`, `pulldown_cmark`) | crates.io | MIT/Apache-2.0 | Already vendored |
| Page prose | Original, author-written | Jeff's | Clear |

No fonts, images, or media are shipped by this feature (CSP forbids external ones
anyway, `security_headers.rs:41`).

### 6.3 Language / claims audit

- [ ] Claims not supported by evidence — **no.** Pages teach concepts; practice is
  marked *suggested* ("Use a site you own or have permission to inspect",
  `osi-model.md:244`); evidence links only to real documented work (the hosting
  walkthrough with captured commands, `osi-model.md:257-260`). The overview states
  the authorship rule outright (`index.md:70-79`).
- [ ] Capabilities not yet built read as shipped — **no.** The overview frames the
  corpus as "an intentionally small first release" (`index.md:66`) and marks the
  three-part model as flexible ("Not every topic needs all three parts",
  `index.md:31`). The Evidence part is present on ~5 of 12 topic pages and absent
  where no owned work yet applies — which is honest, not an overclaim.
- [ ] Domain-restricted language — **no offensive-security / red-team / pentest /
  production-grade / SRE / enterprise identity** appears. Tags reference the live
  cert spine (`network-plus`, `rhcsa`, `osi-model.md:5`, `linux-abstraction-layers.md:5`),
  aligned with criterion 1D (RHCSA → CCNA → Security+). Role posture leads with
  "an early career in Linux systems administration and network operations"
  (`index.md:12-13`) — permitted by 1E.

**One currency fix (1D).** The `description()`/`<meta>` for learn pages comes from
frontmatter `summary` (`wiki.rs:114-116`) and is user-visible copy. It must pass
A2's U-7 retired-claims guard (no "Network+", "the CompTIA stack", "offensive
security", etc.). Current summaries are clean; the guard makes it stay so.

### 6.4 Regulatory alignment (criteria.md Lens 3)

- **3A no-JS floor:** met and central (§3.7A, T-B5-6). *Auto-fail avoided.*
- **3B contrast/colour independence:** sidebar heading font size raised to A1's
  `--text-2xs` floor and audited at 4.5:1; active state has a non-colour border
  (§3.7B). *Auto-fail avoided.*
- **3C keyboard/focus:** all-native controls, A1 focus ring (§3.7C).
- **3D semantics:** heading-order defect fixed (§3.7D, T-B5-7); `aria-current`
  added (T-B5-2).
- **3E motion:** the one unguarded transition moved under `no-preference` (§3.5).
  *Auto-fail avoided.*
- **3F responsive/resilient:** 320px→wide, empty states designed (§3.3, §5.4).

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

The learn feature is **implemented and shipping** — routes, redirects, sidebar,
model, template, 12 topic pages + overview, and drift guards all live. It is one
of the more mature surfaces on the site and its pedagogy already reaches the
criterion-2C bar on the flagship pages. The gaps are refinements and honesty/
maintainability fixes, not a build-out.

| ID | Gap | State | Evidence | Lens |
|---|---|---|---|---|
| **G1** | `section()` returns `"wiki"` (retired name), matched by `base.html:27`'s `== "wiki"` | **implemented but fragile** — highlighting *works today* only because both sides use the stale magic string; the identity is the old name and any natural edit to `"learn"` on one side silently breaks it | `wiki.rs:118-120`, `base.html:27` | 5A/5C; fixed by A2 `Section::Learn` |
| **G2** | Naming split: route `/learn` + nav "Learn" vs page copy/back-link/aria-label "Education wiki" vs module `wiki.rs` + `wiki-*` CSS + `section "wiki"` | **implemented, inconsistent** | `wiki_page.html:4,29`, `index.md:2`, `wiki.rs`, `style.css:1379` | 1D/2D |
| **G3** | `.wiki-disclaimer` CSS is orphaned (no template renders it) and hardcodes `#e0a458` | **absent from render, dead code** — the "honesty banner" moved to `index.md`'s quiet "Authorship and scope" (correct per copy-voice: no disclaimer sections) | `style.css:1039-1054`; no ref in `templates/`/`content/` | 5A (A1 T3/T9), claim-integrity |
| **G4** | No direct SIDEBAR↔WIKI_SLUGS agreement guard | **gap** — each is tied to disk independently; cross-copy drift is silent | `wiki_pages.rs:14-71` vs `wiki.rs:35-99` | 5A/5B |
| **G5** | `og:type` is `website` for learn pages (should be `article`) | **implemented, wrong default** | `base.html:9`, `head_extra` unused | 2D (A2 S-2/I-8) |
| **G6** | Active sidebar entry lacks `aria-current="page"` | **gap** — state is CSS-class-only | `wiki_page.html:13` | 3D |
| **G7** | Sidebar `<h2>`s precede the article `<h1>` in DOM | **outline defect** | `wiki_page.html:10` before `:31` | 3D |
| **G8** | Disclosure-triangle transition is unguarded by reduced-motion | **motion gap** | `style.css:1493` | 3E (A1 T10) |
| **G9** | Sidebar heading at 0.68rem violates A1's 0.70rem `--text-2xs` floor | **contrast/size** | `style.css:1424-1425` | 3B/2F |
| **G10** | Stale `#[allow(dead_code)]` on `summary` (a used field) | **misleading annotation** | `page.rs:33` vs `wiki.rs:114-116` | 5C |
| **G11** | Pedagogical page contract is convention, not codified/tested | **works, ungoverned** — flagship pages hit it; nothing prevents a future bullet-dump | corpus vs no lint | 2C/5E |
| **G12** | Title lacks `" — machinageist"` suffix | **implemented, off-contract** | `wiki.rs:110-112` | A2 U-5 |

**Pedagogical depth today (criterion 2C — the central one).** The corpus **meets
the bar on its strongest pages.** The OSI page builds Physical→Application in
ordinary language, states where the model breaks, walks encapsulation with an
ASCII diagram, contrasts OSI with TCP/IP, turns the model into a troubleshooting
procedure, ends with a FOSS practice (`curl -v` on an owned site) and links real
captured evidence, then cites the ISO standard and RFCs (`osi-model.md` throughout).
The Linux page opens "Linux looks arcane from the outside, but most of it becomes
tractable once you split the system into three layers" (`linux-abstraction-layers.md:10-11`)
— concept before jargon. An audit of all 12 topic pages confirms every one carries
a Practice section and a Sources section, 11/12 carry Related pages, and 5/12 reach
the third Evidence part (blog links) — the others honestly stand on Understand +
Practice, exactly as `index.md:31` permits. This is **not** a bullet-dump of a
source note; it clears the 2C floor. G11 is about *keeping* it there as the corpus
grows, not about a current deficiency.

### 7.2 Delta to spec

**New files:**
- `docs/design/LEARN.md` — the pedagogical contract + pipeline + naming ledger.

**Modified files:**
- `templates/wiki_page.html` — `aria-current="page"` on active link (G6);
  demote sidebar `<h2>` labels to non-heading elements (G7); fill `head_extra`
  with `og:type=article` (G5).
- `src/handlers/wiki.rs` — `section()` → `Section::Learn` (G1, A2 dep); `title()`
  suffix (G12); add `pub fn sidebar_slugs()` for the agreement guard (G4).
- `src/models/page.rs` — remove stale `allow(dead_code)` on `summary` (G10).
- `static/css/style.css` — delete `.wiki-disclaimer` block (G3); move the
  `summary::before` transition under `no-preference` (G8); raise sidebar heading
  size to `--text-2xs` (G9).
- `tests/wiki_pages.rs` — add T-B5-4 agreement guard and T-B5-5 structure lint;
  the `wiki.rs` test module gains T-B5-1/2/3/6/7.
- `README.md:71` — correct the `wiki.rs` one-liner (5E).

**No migrations, no new dependencies, no new assets.**

### 7.3 Estimated scope

**S–M.** The feature is already built; this is targeted refinement. The largest
single item is the pedagogical-structure lint + `LEARN.md` (M on its own because
it must be careful not to over-constrain author voice); everything else is a
handful of small, surgical edits, several of which (G1, G5, G12) are actually
*A2's* changes that B5 consumes. No new page content is in scope — new topics
arrive through the publish pipeline (§4.4) as reviewed work lands.

### 7.4 Blocking dependencies

- **A1 design-system** — the `--text-2xs` floor (G9), the `--text-faint`-at-4.5:1
  audit (§3.7B), and the deletion of the `#e0a458` literal (G3) are A1's contract;
  B5 conforms to it.
- **A2 site-shell** — the `Section::Learn` enum (G1/G12) and the `head_extra`
  `og:type` mechanism (G5) land in A2; B5's template/handler edits depend on them.
  Until A2 ships, the interim honest fix for G1 is to change the magic string to
  `"learn"` on **both** `wiki.rs:119` and `base.html:27` in the same commit.
- No blocking dependency on C1 (search) or C2 (glossary) — those consume the
  learn corpus, not the reverse.

---

## 8. Open Questions

- **Q1 (naming, G2):** Settle the public name. Recommendation: "Learn" is the
  product name (nav + route + tab); "Education wiki" stays only as the overview's
  descriptive subtitle; the back-link and `aria-label` change to "Learn" for
  consistency; internal `wiki.rs`/`wiki-*` names stay as documented legacy.
  Confirm? — blocks: §3.7D copy, `wiki_page.html:4,29`.
- **Q2 (disclaimer, G3):** Confirm deleting `.wiki-disclaimer` outright (the
  copy-voice memory forbids disclaimer sections; the honesty framing already lives
  quietly in `index.md`). If instead a per-page one-line provenance note is
  wanted, it must be quiet chrome, not a banner — and would reintroduce a rule the
  copy voice discourages. — blocks: §7.2 CSS delete.
- **Q3 (structure lint strictness, G11/2C):** How prescriptive should T-B5-5 be?
  Requiring exact section headings ("## Suggested practice") risks flattening
  author voice; a looser check (a practice section + a sources section + at least
  one cross-link exist, by heuristic) preserves voice. Recommendation: the loose
  form, with `LEARN.md` carrying the full contract as guidance rather than a hard
  gate. — blocks: §5.1 T-B5-5.
- **Q4 (sub-feature — new topic pipeline):** New Linux/RHCSA and networking/CCNA
  pages are the corpus's growth path but are *content*, produced by reviewing real
  study/lab work, not a code deliverable of B5. Should the gauntlet track a
  content-production sub-feature (e.g. "learn-linux-track") separately, or is that
  out of scope as ongoing authoring? Flagged here per the no-sub-agents rule. —
  blocks: nothing in this spec; scope decision only.
- **Q5 (diagrams):** Several pedagogy benchmarks (Ciechanowski, Nicky Case) lean
  on interactive diagrams, which the no-JS floor forbids. Confirm that self-hosted
  inline **static** SVG (no script) is the ceiling for visual explanation here,
  with ASCII/`text` blocks as the current baseline. — blocks: §4.5 author rules.
