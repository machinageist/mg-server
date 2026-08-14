# Spec: Glossary (terms + commands)

**Feature ID:** `C2` / `glossary` (branch, covering `C2a` glossary-terms and `C2b` glossary-commands)
**Parent feature:** root (New capabilities)
**Spec author agent:** spec-agent-11 (Claude Opus 4.8)
**Date:** 2026-08-09
**Iteration:** 1

---

## 0. Reading notes and scope boundary

This feature is **absent** — no route, handler, model, template, content, or test
for a glossary exists today. `src/router.rs:38-58` registers no `/glossary`;
`src/handlers/mod.rs:3-8` and `src/models/mod.rs:1-3` declare no glossary module;
`content/` has `posts/`, `pages/`, and `drafts/` but no `glossary/`. Everything
below is therefore **target-state design first**, with the gap from current code
enumerated in §7 and grounded in real files read this session. Where I assert
current behaviour I cite `path:line`.

**In scope (C2 owns), one document for parent + both children:**

- `C2` parent: the `/glossary` landing that ties the two sub-glossaries together.
- `C2a` glossary-terms: `/glossary/terms` — an A–Z definitions index of the
  networking and Linux terms the `/learn` corpus introduces.
- `C2b` glossary-commands: `/glossary/commands` — the commands the `/learn`
  practice sections use, each with a synopsis, one-line purpose, when-to-use
  context, a concrete example, an optional caution, and a cross-link back to the
  page that teaches the concept.
- The glossary data model, the flat-file content source, the server-rendered
  templates, the no-JS A–Z jump bar and the no-JS category filter, the drift
  guards, and the glossary→`/learn` cross-link contract.

**Out of scope, referenced only:**

| Concern | Owner | What C2 assumes from it |
|---|---|---|
| Token architecture, type scale, `--text-*`/`--measure`/`--text-2xs` floor, contrast audit, 23-theme roster, `.tag` styling | `A1` design-system | The prose surface (`.post-content`), the audited colour tokens, and the type scale are A1's; C2 adds **zero colour literals and zero font-size literals** and reuses A1 tokens |
| `base.html`, `Section` enum, nav active-state, `head_extra`/`scripts` blocks, `og:*` meta, title contract, skip link, footer, no-JS OS-preference fallback | `A2` site-shell | Glossary pages render through `base.html`; `Section::Glossary`, `og:type`, and the `" — machinageist"` title suffix are A2 cross-feature deps (§7.4) |
| Security headers / CSP, rate limiting, vitals, HTML cache policy | `A3` ops | CSP is `default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; font-src 'self'` (`security_headers.rs:41-45`) — the no-JS floor is a hard constraint, and the glossary honours it by shipping **zero JavaScript** |
| The `/learn` corpus, `SIDEBAR` allowlist, `sidebar_slugs()`, `Page` model, wiki layout CSS | `B5` learn | The glossary is **seeded from** the 12 learn pages and cross-links into them; the `learn` cross-link drift guard consumes B5's proposed `pub fn sidebar_slugs()` (`wiki.rs`); the glossary reuses the `.wiki-layout`/`.wiki-sidebar`/`.post-content` CSS |
| Site-wide `/search?q=` over `content/` | `C1` search | The glossary is a search corpus later; C2 keeps entries crawlable, anchored, and cross-linked, but does not implement search |
| Flashcards / MCQ / PBQ study tools | `C3` | Different feature; the glossary is a **reference** surface, not a study tool, so C3's progressive-enhancement auto-fail gate does not apply here — but 3A (no-JS floor) still does, and C2 meets it natively |

The `content/drafts/` tree (including `content/drafts/geist-wiki/`) is explicitly
out of scope per `feature-tree.md`.

---

## 1. Purpose

### 1.1 One-sentence job

Give a reader one place to look up any term or command the `/learn` pages use —
defined in plain language, tagged to its domain, and linked back to the page that
teaches it — so a concept met mid-article never becomes a dead end.

### 1.2 Why it matters

Three pressures meet on the glossary and it is where they resolve.

1. **The learn corpus introduces far more vocabulary than any one page can
   re-teach.** `linux-abstraction-layers.md` alone defines *kernel space*, *user
   space*, *context switching*, *time slice*, *multitasking*, *memory management
   unit*, *virtual memory*, *system call*, *pseudodevice*, *superuser*, and
   *root access* — a dozen bolded terms in one page (`linux-abstraction-layers.md:24,33,55,59,83-85,97,110,127-129`).
   `network-protocols.md` defines *protocol*, *port number*, *well-known /
   registered / dynamic ports*, *ICMP*, *GRE*, *IPsec*, and *MTU*
   (`network-protocols.md:10-12,22-34,80-93`). A reader who lands on the OSI page
   from a search result and hits "encapsulation" or "MTU" has nowhere to look it
   up without leaving the site. The glossary is that lookup, and because every
   entry links back to the teaching page, it is also a second, alphabetical door
   *into* the corpus (a reviewer path the nav does not provide — criterion 4E).

2. **It is a scannability and cross-linking surface, which the benchmark tier is
   built on.** Criterion 2A names "Arch Wiki and MDN (scannability, cross-linking,
   stable structure)" as a benchmark. A glossary with stable `#anchor` ids, an A–Z
   jump bar, `see also` between related entries, and `<dl>` semantics is exactly
   that pattern. The `/learn` pages can then deep-link a word to
   `/glossary/terms#mtu` instead of re-defining it inline, which keeps the prose
   pages shorter and the definitions single-sourced (criterion 5A).

3. **It is a no-JS reference by nature, so it strengthens the site's identity for
   free.** An A–Z index of definitions is native HTML: a `<dl>` list, a row of
   anchor links, real URLs. It needs no JavaScript to be complete, which is the
   thing the site trades on (criterion 3A, auto-fail rule 3). Building it well is a
   chance to demonstrate the no-JS discipline on a surface where competitors reach
   reflexively for a client-side filter widget.

### 1.3 Success signal

**Primary (observable):** a reader with JavaScript disabled who follows a
`/glossary/terms#mtu` cross-link from `/learn/network-protocols` can read the
definition, use the A–Z jump bar to move to another letter, follow a `see also`
to a related term, and click "Learn more" back to the teaching page — all in a
colour scheme that respects their OS preference, with no control on screen that
does nothing.

**Secondary (measurable):** `cargo test --all-targets` passes, including the new
glossary drift guards (§5.1): every entry parses into a typed record, every
anchor id is unique, entries render in sorted order, every `see_also` resolves to
a real entry in the same glossary, and every `learn` cross-link slug is a valid
`SIDEBAR` slug (via B5's `sidebar_slugs()`).

---

## 2. User Stories

> **Happy path — self-directed learner.** As an adult reading `/learn/osi-model`,
> I want to click "encapsulation" and land on a plain-language definition without
> losing my place in the corpus, so that an unfamiliar word is a two-second detour
> rather than a reason to close the tab.

> **Happy path — engineer peer skimming for signal.** As a working engineer, I
> want the commands glossary to show real synopsis + when-to-use context (e.g.
> `ps -ef --forest` "shows the parent/child process tree `fork()` produces"), not
> a man-page copy-paste, so that in thirty seconds I can tell the author actually
> uses these tools.

> **Happy path — hiring manager.** As a hiring manager, I want the glossary
> entries tagged to the domains the author is studying (networking, Linux) and
> linked to documented learn pages, so that the vocabulary reads as directed
> study with a paper trail, not a scraped word list.

> **Edge case — category filter with no matches / empty glossary.** As a reader
> who filters to a category that has no entries yet, I want a plain "nothing here
> yet" line rather than a blank page or a broken layout, so that an empty state is
> designed, not accidental.

> **Edge case — a term with a synonym or abbreviation.** As a reader who knows
> "MMU" but not "memory management unit", I want to find "MMU" in the A–Z index
> and be sent to the canonical entry, so that I do not have to already know the
> full name to look it up.

> **Accessibility — no JavaScript / keyboard / screen reader.** As a reader on a
> text browser or with JS blocked, I want the A–Z jump bar, every cross-link, and
> the term/definition pairs to work and be announced as term/definition (not
> signalled by colour alone), so that the whole reference is reachable without
> scripts.

> **Maintainer.** As the person moving a reviewed term from my study notes into
> the glossary, I want one obvious file to edit and a test that fails loudly if I
> add a `see also` to a term that does not exist or a "Learn more" link to a page
> that is not published, so that I cannot ship a dangling reference.

---

## 3. UX Specification

### 3.1 Screen / view inventory

| Surface | Path to reach | New / modified | Layout pattern |
|---|---|---|---|
| **Glossary landing** (`C2`) | `/glossary` (new route → `glossary::landing`) | **New** | Standard 900px article column: `<h1>`, one-line intro, two links (Terms, Commands) with counts, one line noting they index `/learn` |
| **Terms glossary** (`C2a`) | `/glossary/terms` (new route → `glossary::terms`) | **New** | Two-column wiki grid (reuses `.wiki-layout`): sidebar with glossary nav + A–Z jump list; article is the `<dl>` A–Z index |
| **Commands glossary** (`C2b`) | `/glossary/commands` (new route → `glossary::commands`) | **New** | Same wiki grid; article is the `<dl>` A–Z command index with synopsis/purpose/context/example/caution per entry |
| **Category-filtered view** | `/glossary/terms?cat=networking`, `?cat=linux` (same routes) | **New** | Same page, server-rendered subset; a visible "Showing: Networking · [All]" line and reset link |
| **Synonym stub** | anchor inside the A–Z index, e.g. `/glossary/terms#mmu` | **New** | A `<dt>` "MMU — see Memory management unit" whose `<dd>` is a single link to the canonical anchor |
| **Glossary 404** | `/glossary/<anything-else>` | Modification (A2-owned) | Themed 404 via the shell fallback; only the three fixed routes exist, everything else falls through |

No modals, sheets, drawers, or popovers of C2's own. The mobile sidebar reuses
B5's native `<details>`/`<summary>` (no JavaScript). The A–Z jump bar and the
category filter are plain links and real URLs.

### 3.2 Interaction flows

**Flow A — arrive from a learn cross-link and look up a term (primary, JS-independent).**

1. Reader clicks `[encapsulation](/glossary/terms#encapsulation)` in a learn page
   body. Browser navigates to `/glossary/terms` and scrolls to the `encapsulation`
   `<dt>`, which clears the sticky header via `scroll-margin-top` (reuse the
   existing pattern at `style.css:1160`).
2. `glossary::terms` (no query) renders the full A–Z index server-side. The
   entry's `<dt id="encapsulation">` and its `<dd>` definition are visible.
3. The `<dd>` ends with, where applicable, a `see also` line and a "Learn more →
   OSI model" link to `/learn/osi-model`. The reader follows either.

**Flow B — browse A–Z (JS-independent).**

1. At the top of `/glossary/terms` a jump bar renders `A B C … Z` as a `<nav
   aria-label="Jump to letter">`. Letters with at least one entry are `<a
   href="#letter-e">E</a>`; letters with none render as a non-link `<span>` styled
   `--text-faint` (colour-independent: the empty ones are simply not links, not
   merely a different hue).
2. Clicking a letter scrolls to `<h2 id="letter-e">E</h2>`. Native anchor
   behaviour; no script.

**Flow C — filter by category (no-JS enhancement, real URL).**

1. Above the jump bar, a filter row renders "Showing: **All** · [Networking] ·
   [Linux]" where the two brackets are links to `?cat=networking` / `?cat=linux`
   and "All" (the current state) is plain text.
2. Selecting `?cat=networking` re-requests the page; `glossary::terms` filters the
   entries to the Networking category **server-side** and re-renders. The jump bar
   recomputes so empty letters reflect the filtered set. A "[Show all]" reset link
   appears. This is the same no-JS filtering philosophy as `C1`'s `/search?q=`.
3. An unknown `?cat=` value is treated as "All" (forgiving; not a 404).

**Flow D — synonym / abbreviation lookup (JS-independent).** The A–Z index
includes stub `<dt>`s for each `aka` synonym, e.g. `<dt id="mmu">MMU</dt><dd>see
<a href="#memory-management-unit">Memory management unit</a></dd>`. The reader who
searches the page for "MMU" (browser Ctrl-F, which needs no feature JS) or jumps
to the M section finds the stub and follows it to the canonical entry.

**Flow E — unknown glossary URL.** `/glossary/nope` matches no route and falls to
`errors::fallback_404` (`router.rs:62`) → the themed 404 with live nav (A2 flow
C). Recovery is not limited to the back button.

**Cues.** No haptics, no sound, **no animation introduced by this feature**. The
only motion on a glossary page is the mobile sidebar disclosure triangle inherited
from B5 (which B5 §3.5 moves under `prefers-reduced-motion: no-preference`). C2
adds nothing to guard.

### 3.3 Layout descriptions

**Glossary landing** (`/glossary`, new `glossary_landing.html`), top → bottom, in
the standard 900px `.post-content` column:

1. `<h1>Glossary</h1>`.
2. One `--measure-narrow` intro paragraph, quiet and non-strategic (copy in §6.3).
3. Two entries rendered as an `.about-list`-style divider list (reuse the existing
   pattern, not cards — criterion 2E forbids card-ification of a two-item list):
   - **Terms** → `/glossary/terms` — "NN definitions across networking and Linux."
   - **Commands** → `/glossary/commands` — "NN commands with usage and context."
   The counts come from `data.len()` at render time, so they cannot go stale.
4. One closing line: "Both index the [Learn](/learn) pages."

**Terms / Commands page** (reuse `.wiki-layout` grid, `style.css:1379-1384`),
left → right:

- **Sidebar** (`<aside class="wiki-sidebar" aria-label="Glossary navigation">`,
  reusing B5's sticky/collapsible CSS): a `<nav>` with two groups — a "Glossary"
  group listing Terms and Commands (the active one carries `aria-current="page"`
  and the non-colour active border, reusing `style.css:1443-1452`), and a "Jump to
  letter" group holding the A–Z bar for narrow screens. On wide screens the A–Z
  bar also renders at the top of the article for immediate reach.
- **Article** (`<article class="glossary-article article-page">`):
  1. `<a class="back-link" href="/glossary">Glossary</a>` (reuse `.back-link`,
     `style.css:1028-1037`).
  2. `<header class="post-header">` — the single page `<h1>` ("Terms" / "Commands")
     and `.post-meta` (a short subtitle; no date pill — a glossary is not dated
     content, so `.post-date` is omitted rather than faked).
  3. The category-filter row (§3.2 Flow C).
  4. The A–Z jump bar (`<nav aria-label="Jump to letter">`).
  5. The index body: for each non-empty letter, `<h2 id="letter-x">X</h2>`
     followed by a `<dl class="glossary-list">` of that letter's entries.

**Term entry markup** (the `<dl>` unit, `C2a`):

```html
<dt id="encapsulation">Encapsulation <span class="glossary-cat">Networking</span></dt>
<dd>
  <p>Each layer wrapping the data from the layer above with its own header … </p>
  <p class="glossary-seealso">See also:
     <a href="#mtu">MTU</a>, <a href="#protocol-data-unit">Protocol data unit</a></p>
  <p class="glossary-learn">Learn more:
     <a href="/learn/osi-model#encapsulation">OSI model</a></p>
</dd>
```

**Command entry markup** (`C2b`) — richer, task-oriented:

```html
<dt id="ps"><code>ps</code> <span class="glossary-cat">Linux</span></dt>
<dd>
  <p class="glossary-synopsis"><code>ps -ef --forest</code></p>
  <p class="glossary-purpose">Lists running processes.</p>
  <p>Reach for it to see the parent/child tree <code>fork()</code> produces and to
     find <code>init</code>/<code>systemd</code> at PID&nbsp;1.</p>
  <p class="glossary-caution"><strong>Note:</strong> read-only; safe on any host.</p>
  <p class="glossary-learn">Learn more:
     <a href="/learn/linux-abstraction-layers#process-management">Linux abstraction layers</a>
     · <a href="https://man7.org/linux/man-pages/man1/ps.1.html" rel="noopener">man&nbsp;ps(1)</a></p>
</dd>
```

**Data sources.** Colour and font role come from the active `[data-theme]` block
(A1). Size, spacing, measure, and the layout grid come from A1's measurement
layer. Entry content comes from `content/glossary/terms.md` and
`content/glossary/commands.md`, parsed into `Vec<GlossaryTerm>` /
`Vec<GlossaryCommand>` (§4.2). Counts come from `data.len()`. No component reads a
literal colour or size.

**Empty states** (mirror A1 §3.3 and B5 §3.3):

- A category filter (or an as-yet-unpopulated glossary) that matches nothing
  renders **one** `--text-muted` paragraph at body size inside `--measure-narrow`:
  "No terms in this category yet." — never a placeholder card, never a spinner.
- A letter with no entries in the current view is **omitted** from the body (no
  empty `<h2>` over nothing) and rendered as a non-link in the jump bar.
- The prose synopsis/example lines that are `Option::None` on a command render
  nothing — no empty label. (Same "omit, don't render blank" rule the vitals strip
  uses at `vitals_strip.html:11-16`.)

### 3.4 Input & gestures

- **Pointer.** Click on: the two landing links, sidebar entries, the A–Z jump
  letters, category-filter links, `see also` links, "Learn more" learn-page links,
  external man-page links, and the mobile `<summary>` toggle. No hover-only
  affordance carries information.
- **Keyboard.** Every interactive element is a native `<a>` or the native
  `<summary>`; all are in the tab order and operable with `Enter`/`Space`, using
  A1's global `:focus-visible` ring (`style.css:685`). No feature-specific
  shortcuts (they would trip WCAG 2.1.4). Browser find-as-you-type (Ctrl-F) works
  over the whole server-rendered index — which is the reason a bespoke filter box
  is unnecessary (§4.6 rejected-JS note).
- **Touch.** Jump-letter and filter links get adequate hit area via padding
  (target the 44px guidance A1 §3.4 sets for the theme button); the `<summary>`
  and `<a>` padding give full-width tap targets on the collapsed sidebar.
- **Responsive.** Reuse B5's single 800px breakpoint (grid → single column,
  sidebar → collapsible floating `<details>`) plus A2's 640px chrome breakpoint.
  The A–Z jump bar is a `flex-wrap` row so it wraps to 2–3 rows on a phone rather
  than scrolling horizontally.
- **Stylus / controller / voice / camera.** N/A — text and links only.

### 3.5 Transitions & animation

**None introduced by C2.** The glossary adds no `transition` or `animation`
declaration. The only motion on the page is the mobile sidebar disclosure triangle
inherited from B5, whose reduced-motion guard is B5's responsibility
(`style.css:1493`, moved under `no-preference` by B5 §3.5). Reduced-motion
alternative is therefore *absence*, achieved by adding nothing — the strongest
possible compliance with criterion 3E.

### 3.6 Error states

| ID | Trigger | Presentation | Why that presentation | Recovery | Data loss |
|---|---|---|---|---|---|
| **E-01** | `/glossary/<unknown>` | **Full-page** themed 404 via the shell fallback (`router.rs:62`) | A wrong URL is a navigation event; the shell keeps nav live (A2 E-01). No toast exists on the site and none is proposed | Header nav + 404 home link | No |
| **E-02** | `?cat=` value not a known category | **Silent** — treated as "All"; the page renders the full index | Forgiving input on a public reference beats a 404 for a mistyped query string; a colour-domain filter is not worth an error page | Automatic | No |
| **E-03** | A glossary content file is missing or malformed (frontmatter parse / bad category / duplicate anchor) | **Full-page** themed 500 (`SiteError::MissingFrontmatter`/`FrontmatterParse`, mirroring `page.rs:52-54`) | A malformed curated data file is an **author** error, not a visitor error; the 500 leaks nothing (A2 E-02). **Caught in CI by the parse + drift guards (§5.1) before it ships** | Fix the file | No |
| **E-04** | A `see_also` or `learn` reference is dangling | **Build/test failure**, never a runtime state | A dangling cross-reference is exactly the drift Lens 5 exists to catch; it must fail loudly in `cargo test`, not 404 for a reader | Fix the reference | No |
| **E-05** | JS disabled | **No degradation.** A–Z bar, filter, entries, and cross-links are native HTML; theme follows OS preference (A1/A2) | The no-JS floor is the feature's baseline, not an afterthought | Read normally | No |

**Presentation justification.** Every visitor-facing error is full-page (404/500)
because the failure is "the resource does not exist / is malformed," not a
transient notice over otherwise-good content. Author errors (E-03, E-04) are
pushed to CI so they cannot reach a visitor at all.

### 3.7 Accessibility

Graded as an auto-fail gate (rules 2 and 3). Written as invariants + the design
that meets each.

**A. Works without JavaScript (auto-fail rule 3).** **Met by construction, and it
is the feature's baseline.** The A–Z bar is `<a href="#letter-x">`; the category
filter is `<a href="?cat=…">`; every entry, `see also`, and "Learn more" is a real
`<a href>`; the term/definition structure is a server-rendered `<dl>`; the mobile
sidebar is native `<details>`. C2 ships **zero JavaScript**. Pinned by §5.1 T-C2-8
(strip `<script>`; assert the jump bar, every entry anchor, and every cross-link
survive).

**B. Contrast & colour independence.**
- All colours are A1 tokens; no literal is introduced. The category label
  (`.glossary-cat`) reuses the audited `.tag` treatment (A1 §3.1) — and crucially
  its **meaning is the word** ("Networking"/"Linux"), never the colour alone.
- The active glossary-nav entry (Terms vs. Commands) carries the non-colour 2px
  accent left border (reuse `style.css:1443-1452`) **plus** `aria-current="page"`.
- Empty jump-letters are distinguished from active ones by **being non-links**
  (not underlined, not focusable), not merely by a lighter hue — a
  colour-independent state.
- No text renders below A1's `--text-2xs` (0.70rem) floor; the category label and
  any metadata use `--text-xs`/`--text-2xs`, audited at 4.5:1 (A1 §3.7A). C2 must
  not reintroduce B5's retired 0.68rem sidebar-heading size.

**C. Focus & keyboard.** All interactive elements are native and keyboard-operable
with A1's visible focus ring. Focus order follows DOM order: back-link → filter
links → jump-bar letters → entry cross-links, with the sidebar preceding the
article in source order.

**D. Semantics & heading outline.**
- The A–Z index uses a real `<dl>`/`<dt>`/`<dd>` — the semantically correct
  element for a glossary. Screen readers announce term/definition pairs, which is a
  materially better experience than a flat list of headings.
- **Heading outline:** the article `<h1>` ("Terms"/"Commands") comes **first** in
  the article DOM; the per-letter `<h2 id="letter-x">` section headings follow it
  in order. The sidebar's group labels are **non-heading** styled elements (a
  `<p>`/`<span>` inside the `<nav>` landmark), so they do not compete for the
  outline — C2 adopts B5 §3.7D's fix from the start rather than repeating B5's
  sidebar-`<h2>`-before-`<h1>` defect (`wiki_page.html:10`).
- The jump bar and the sidebar are each a `<nav>` with a distinct `aria-label`
  ("Jump to letter", "Glossary navigation").
- Landmarks: `<aside>` → `<nav>` for the sidebar; the article is inside `<main>`
  from the shell.

**E. Motion & sensory safety.** C2 introduces no motion (§3.5). No autoplay, no
flashing, no body-content animation.

**F. Responsive & resilient.** Works 320px → wide (A2 breakpoints + the 800px grid
collapse). At 200% zoom / large browser font the definitions reflow (sizes are
`rem`, A1 §3.7F) and the jump bar wraps. Empty and filtered-empty states are
designed (§3.3), not accidental.

---

## 4. Implementation Specification

### 4.1 Architecture placement

```
src/
  router.rs                  ← add /glossary, /glossary/terms, /glossary/commands (before ServeDir/fallback)
  handlers/
    mod.rs                   ← add `pub mod glossary;`
    glossary.rs              ← NEW: landing/terms/commands handlers, Query<CatFilter>, template structs, section()
  models/
    mod.rs                   ← add `pub mod glossary;`
    glossary.rs              ← NEW: GlossaryTerm, GlossaryCommand, Category, LearnRef, load + anchor + sort + drift helpers
templates/
  glossary_landing.html      ← NEW: the /glossary parent page
  glossary_index.html        ← NEW: shared A–Z index layout for terms and commands (or two thin templates over one macro)
static/css/style.css         ← NEW `.glossary-*` rules; reuse .wiki-layout/.wiki-sidebar/.post-content/.tag/.back-link
content/glossary/
  terms.md                   ← NEW: the terms data (frontmatter-only; see §4.2)
  commands.md                ← NEW: the commands data
tests/glossary.rs            ← NEW: parse + drift guards (anchor uniqueness, sort, see_also, learn-slug validity, no-JS)
docs/design/GLOSSARY.md      ← NEW long-lived doc: the entry contract, the seeding-from-/learn rule, the drift guards
```

The glossary follows the exact shape of the learn feature: a curated allowlist +
flat-file content + a typed model + a server-rendered template + drift guards. It
adds two new modules (`handlers/glossary.rs`, `models/glossary.rs`) — justified
because it is a genuinely new top-level surface with its own routes, data model,
and content, not an extension of an existing handler.

### 4.2 Data model

**New types** in `src/models/glossary.rs`, in the project's Rust style (author
banner, `// Verb + noun` above each fn, section-divider comments above blocks,
constants in `ALL_CAPS`):

```rust
// The domain a glossary entry belongs to; drives the text tag and the ?cat= filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Networking,
    Linux,
}

// A cross-link back into the /learn corpus: /learn/<slug>#<anchor>
#[derive(Debug, Clone, Deserialize)]
pub struct LearnRef {
    pub slug: String,             // must be a valid SIDEBAR slug (drift-guarded)
    pub anchor: Option<String>,   // optional in-page anchor
    pub label: String,            // link text, e.g. "OSI model"
}

// One term definition (C2a). Prose-first: definition leads in ordinary language
#[derive(Debug, Clone, Deserialize)]
pub struct GlossaryTerm {
    pub term: String,             // canonical display term, e.g. "Memory management unit"
    #[serde(default)]
    pub aka: Vec<String>,         // synonyms/abbreviations, each gets a stub anchor, e.g. ["MMU"]
    pub category: Category,
    pub definition: String,       // plain-language definition (Markdown-inline allowed)
    #[serde(default)]
    pub see_also: Vec<String>,    // other term anchors in THIS glossary (drift-guarded)
    #[serde(default)]
    pub learn: Vec<LearnRef>,     // teaching-page cross-links
}

// One command (C2b): synopsis + purpose + context + optional example/caution
#[derive(Debug, Clone, Deserialize)]
pub struct GlossaryCommand {
    pub name: String,             // "ps"
    pub synopsis: String,         // "ps -ef --forest"
    pub category: Category,
    pub purpose: String,          // one line: what it does
    pub context: String,          // when/why to reach for it
    #[serde(default)]
    pub example: Option<String>,  // a concrete invocation from a learn practice section
    #[serde(default)]
    pub caution: Option<String>,  // destructive/root notes, text-labeled (not colour-only)
    #[serde(default)]
    pub see_also: Vec<String>,
    #[serde(default)]
    pub learn: Vec<LearnRef>,
    #[serde(default)]
    pub man: Option<String>,      // man-page URL (man7.org), rendered as an external link
}
```

**Anchor derivation.** A single `fn anchor(display: &str) -> String` lowercases,
trims, replaces runs of non-alphanumerics with a single `-`, and strips leading/
trailing `-` (e.g. "Memory management unit" → `memory-management-unit`, `ps` →
`ps`, `/proc` → `proc`). Applied to `term`/`name` and to each `aka`. Anchors must
be unique within a glossary — a drift guard (§5.1 T-C2-2), because a duplicate `id`
silently breaks in-page links.

**Sorting.** The handler sorts entries case-insensitively by the canonical
`term`/`name` before grouping by first letter, so the on-disk order is irrelevant
and the A–Z index is always correct regardless of author discipline. `aka` stubs
are interleaved at their own first letter.

**Content format (recommended: reuse the in-tree parser, zero new dependencies).**
The two files are authored as **frontmatter-only Markdown**, parsed by the exact
`gray_matter::Matter::<YAML>` + `parsed.data.deserialize()` idiom already used in
`page.rs:48-54`. `content/glossary/terms.md` looks like:

```markdown
---
entries:
  - term: "Encapsulation"
    category: networking
    definition: >
      Each network layer wrapping the data it receives from the layer above
      with its own header (and sometimes trailer) before handing it down.
    see_also: ["mtu", "protocol-data-unit"]
    learn:
      - { slug: "osi-model", anchor: "encapsulation", label: "OSI model" }
  - term: "Memory management unit"
    aka: ["MMU"]
    category: linux
    definition: >
      Hardware in modern CPUs that gives each process a private, contiguous-
      looking virtual address space regardless of the physical layout.
    learn:
      - { slug: "linux-abstraction-layers", anchor: "memory-management", label: "Linux abstraction layers" }
---
```

`Glossary::load(path)` reads the file, runs `Matter::<YAML>::new().parse(&raw)`,
`.data.ok_or(MissingFrontmatter)?.deserialize::<GlossaryFile>()`, and returns the
`Vec`. **This adds no dependency** — `gray_matter`, `serde`, and its YAML engine
are already vendored (`Cargo.toml:27,35`), and the idiom is identical to the one
`Page::from_file` uses. The one ergonomic wart — the ceremonial `---` fences
around what is really a data file, with an unused Markdown body — is documented in
`GLOSSARY.md`. The alternative (adding a maintained standalone data-format crate)
is flagged as Q1 in §8; I recommend the zero-dependency reuse because `serde_yaml`
is deprecated and pulling `toml` in for a nested list is heavier than the wart it
removes.

**No database, no migrations.** The site has no persistence layer; content is read
from disk per request, exactly as `Page::find` does (`page.rs:72-78`).

### 4.3 API contracts

| Route | Handler | Returns | Errors | Auth |
|---|---|---|---|---|
| `GET /glossary` | `glossary::landing` | `GlossaryLandingTemplate` (counts from both files) | 500 if either file malformed | none |
| `GET /glossary/terms` | `glossary::terms(Query<CatFilter>)` | `GlossaryIndexTemplate` (terms, optionally filtered) | 500 if `terms.md` malformed | none |
| `GET /glossary/commands` | `glossary::commands(Query<CatFilter>)` | `GlossaryIndexTemplate` (commands, optionally filtered) | 500 if `commands.md` malformed | none |

```rust
// Optional ?cat= filter; unknown/absent values render the full index
#[derive(Debug, Deserialize)]
pub struct CatFilter {
    #[serde(default)]
    pub cat: Option<String>,  // "networking" | "linux" | anything else → ignored
}
```

- **Handlers** return `Result<impl IntoResponse, SiteError>` so `?` propagates I/O
  and parse errors to the shell's themed 500, matching `wiki.rs:124-133`.
- **Template contract (A2 S-1).** Each template implements `title()` /
  `description()` / `section()`. `title()` ends in `" — machinageist"` (A2 U-5);
  `description()` is 50–160 chars and passes A2 U-7's retired-claims guard;
  `section()` returns `Section::Glossary` (A2 cross-feature request, §7.4). Until
  A2's enum lands, the interim `section()` returns the magic string `"glossary"`,
  which matches no nav item — so, like `/releases` and `/status` today, no nav
  entry highlights on glossary pages (A2 U-3 behaviour).
- **`head_extra` (A2 S-2).** The two index templates fill `{% block head_extra %}`
  with `<meta property="og:type" content="article">` so a shared glossary link
  previews as an article rather than the default `website` (`base.html:9`).
- **No pagination** (a personal-scale glossary is one page each; the A–Z bar is the
  in-page navigation). **No rate-limit specifics** (A3's limiter applies uniformly,
  `router.rs:71-75`). **No auth** — the corpus is public by design.
- **Route ordering.** The three routes are registered with the other `get(...)`
  routes **before** `.nest_service("/static", …)` and `.fallback(...)`
  (`router.rs:59-62`), following the file's "more specific routes first" rule
  (`router.rs:11`). `:slug`-style capture is not used — the three paths are fixed.

**External links and CSP.** Man-page (`man7.org`) and any RFC links are plain
`<a href>` navigations. CSP restricts *resource loading* (script/style/img/font),
not hyperlink navigations — the corpus already links `man7.org` and `rfc-editor.org`
(`linux-abstraction-layers.md:156-160`, `network-protocols.md:122-129`), so this is
established, in-policy behaviour. External links carry `rel="noopener"` per the
footer's convention (`base.html` footer uses `rel="noopener noreferrer"`).

### 4.4 State management

| State | Owner | Lifetime | Sync |
|---|---|---|---|
| Glossary entries | `content/glossary/{terms,commands}.md`, read + parsed per request | Per request | Server only; no cache |
| Active category filter | `?cat=` query param, parsed into `CatFilter` | Per request | Server only; a real URL, shareable, back-button-safe |
| Sidebar open/closed (mobile) | native `<details>` `open` attribute | Ephemeral | Client only, no JS |
| Theme | `localStorage.theme` (A1/A2) | Browser | Client only |

**No new state container.** Content is read from disk per request — consistent
with `Page::find` (`page.rs:47`) and B5's explicit no-cache decision (B5 §4.4).
Two small files parsed per request is trivial at this traffic. A `LazyLock`/
`OnceLock` parse-once cache is a **rejected optimisation** (recorded, §4.7):
premature for two files, and it would fossilise content on a truth-first site the
same way B5 declined a page cache.

**Offline / draft persistence:** N/A — nothing is authored in the browser; the
`?cat=` state lives entirely in the URL.

### 4.5 Dependencies

- **New packages: none** (recommended path). `gray_matter` (`Cargo.toml:27`),
  `serde` with `derive` (`:35`), `pulldown-cmark` (`:26`, if inline-Markdown
  definitions are rendered), and `serde` `Deserialize` for the `Query` extractor
  (axum re-exports `serde`) are all present. The `toml`/`serde_yml` alternative
  (Q1) would add exactly one dependency; the recommendation avoids it.
- **New assets: none.** The glossary is text; no images, fonts, or scripts. (CSP
  forbids external ones anyway, `security_headers.rs:41-45`.)
- **New content:** `content/glossary/terms.md`, `content/glossary/commands.md`.
- **New doc:** `docs/design/GLOSSARY.md`.
- **Infrastructure:** none.

### 4.6 Platform-specific considerations

- **`<dl>` inside `.post-content`.** The prose surface already styles `<p>`, `<ul>`,
  `<pre>`, tables, and headings (`style.css:1049-1195` region). `<dl>`/`<dt>`/`<dd>`
  need a small style block (`.glossary-list`) but reuse the same tokens; nothing
  layout-critical depends on a bleeding-edge CSS feature.
- **`:has()` for the wide grid.** The glossary reuses `main:has(.wiki-layout)` to
  widen to `--layout-wide` (A1 §4.6); without `:has()` it degrades to 900px —
  narrower, fully usable. No layout-critical feature is adopted.
- **`scroll-margin-top` on anchors.** Required so jump/deep-link targets clear the
  sticky header; reuse the existing declaration pattern (`style.css:1160`). Applied
  to `dt[id]` and `h2[id]`.
- **`pulldown-cmark` for inline definition Markdown.** If definitions carry inline
  Markdown (`` `code` ``, links, emphasis), render each `definition` string through
  the existing `pulldown_cmark` pipeline (`page.rs:58-60`) and emit with `|safe`.
  **Trust boundary:** the source is single-author, version-controlled flat files,
  never user input — the same invariant B5 §4.6 documents for `content_html|safe`.
  The `?cat=` value is **never** reflected into HTML unescaped; it only selects a
  filter branch, so it carries no XSS surface. If inline Markdown is judged
  unnecessary, definitions render as plain escaped text and pulldown-cmark is not
  invoked here — a scope simplification flagged in Q2.
- **Rejected JavaScript (recorded, criterion 3A / 2E).** A client-side instant
  filter/typeahead box was considered and **declined**. The server-rendered A–Z
  index is fully searchable with the browser's native Ctrl-F, the `?cat=` filter
  covers the only two categories server-side, and the site's identity is ~95 lines
  of JS total (A2 §1.2). Adding a filter widget would spend the JS budget to
  duplicate Ctrl-F and would create a no-JS-floor liability for no real gain.
  Glossary ships zero JavaScript, matching B5's 0-byte posture.
- **Feature flags / rollout:** N/A — single binary, single deploy.

### 4.7 Performance budget

| Dimension | Target | Note |
|---|---|---|
| Largest glossary HTML | ≤ ~80 KB rendered per page at ~150 entries | Text; gzip/brotli at Caddy makes this trivial on the wire |
| Per-request work | 1 file read + gray_matter parse + sort + group + render | Two small files; sub-millisecond; matches `Page` cost profile |
| CSS added | ≤ ~0.8 KB for `.glossary-*` (most styling reused from `.wiki-*`/`.post-content`/`.tag`) | Stays inside A1's CSS budget headroom (A1 §4.7) |
| JS added | **0 bytes** | The feature is native HTML end to end |
| Network requests added | 0 | No images, fonts, or scripts |
| Client storage | 0 (theme is A1's) | The `?cat=` state lives in the URL |
| Memory | Two `Vec`s of small structs, dropped after each render (no cache) | `NAV`/`SIDEBAR`-scale data |

**Caching:** none proposed. Glossary pages inherit A3's HTML no-cache requirement
(A2 §4.7) so the live vitals strip stays honest; do not introduce a per-page cache
that would fossilise content or the strip.

---

## 5. Test Specification

All Rust tests run under `cargo test --all-targets` and gate CI (`fmt → clippy →
test → build --release`, criterion 5D). Tests parse the shipped content and the
served bytes — the drift class this feature is most exposed to.

### 5.1 Unit / integration tests

New `tests/glossary.rs` plus a `#[cfg(test)] mod tests` in
`src/handlers/glossary.rs` (router `oneshot`, the pattern at `wiki.rs:237-260` and
`errors.rs:171-182`).

| # | Name | Setup | Assertion | Edge case covered |
|---|---|---|---|---|
| **T-C2-1** | `both_glossary_files_parse` | `Glossary::load` each file | Both deserialize into the typed `Vec`; every `category` is a known variant | A malformed data file (E-03) caught in CI, not prod |
| **T-C2-2** | `anchors_are_unique_within_each_glossary` | Compute `anchor()` over every `term`/`name` **and** every `aka` | No duplicate id in a glossary | A duplicate `id` silently breaking in-page links |
| **T-C2-3** | `entries_render_in_sorted_order` | Render each index | Canonical terms appear case-insensitively sorted; letter `<h2>`s are ascending | On-disk order drifting from displayed order |
| **T-C2-4** | `every_see_also_resolves` | For each entry, check each `see_also` against the set of anchors in the **same** glossary | No dangling `see also` | E-04: a reference to a removed/renamed entry |
| **T-C2-5** | `every_learn_slug_is_a_published_learn_page` | Collect all `LearnRef.slug`; compare to `wiki::sidebar_slugs()` (B5) | Every cross-link points at a real, published `/learn` page | A "Learn more" link to an unpublished/renamed slug |
| **T-C2-6** | `commands_carry_purpose_and_context` | For each `GlossaryCommand` | `purpose` and `context` are non-empty | C2b regressing to a man-page-only stub (criterion 2C) |
| **T-C2-7** | `terms_have_substantive_definitions` | For each `GlossaryTerm` | `definition` length ≥ a small floor (e.g. 40 chars) | A one-word placeholder definition (criterion 2C) |
| **T-C2-8** | `glossary_pages_need_no_javascript` | `oneshot` all three routes, strip `<script>` | The jump bar, every entry `id` anchor, every `see_also`/`learn` href, and the category-filter links survive | No-JS floor (auto-fail rule 3) |
| **T-C2-9** | `category_filter_narrows_and_reset_shows_all` | `oneshot GET /glossary/terms?cat=linux` vs no query | The filtered body contains only Linux entries and a reset link; the unfiltered body contains both categories | Server-side filter correctness (Flow C) |
| **T-C2-10** | `unknown_category_renders_full_index` | `oneshot GET /glossary/terms?cat=bogus` | Status 200; body equals the unfiltered index | E-02 forgiving-input behaviour |
| **T-C2-11** | `article_h1_precedes_any_h2` | Render each index | The first heading in the article is the `<h1>`; no `<h2>` before it | The B5 sidebar-`<h2>`-before-`<h1>` outline defect (§3.7D) |
| **T-C2-12** | `active_glossary_nav_entry_carries_aria_current` | Render `/glossary/terms` | The Terms sidebar link has `aria-current="page"`; Commands does not | AT-invisible active state (§3.7D) |
| **T-C2-13** | `landing_counts_match_data` | Render `/glossary` | The Terms/Commands counts equal `data.len()` for each file | A hand-typed count going stale (criterion 5B) |
| **T-C2-14** | `glossary_descriptions_do_not_carry_retired_claims` | Every `description()` | Contains none of `"Network+"`, `"the CompTIA stack"`, `"offensive security"`, `"red-team"`, `"pentest"`, `"production-grade"`, `"enterprise"`, `"SRE"` | Criterion 1D/1E on user-visible `<meta>` copy (A2 U-7 family) |

**On T-C2-5 and the cross-feature guard (criterion 5A).** The `learn`-slug guard
is the glossary's equivalent of B5's SIDEBAR↔WIKI_SLUGS agreement guard. It
consumes B5's proposed `pub fn sidebar_slugs() -> Vec<&'static str>` so the two
features share **one** definition of "what `/learn` publishes." Until that `pub
fn` lands, the interim form is a golden list in `tests/glossary.rs` that duplicates
the SIDEBAR slugs with a comment pointing at `wiki.rs::SIDEBAR` — the same honest,
documented duplication B5 uses for `WIKI_SLUGS` (`wiki_pages.rs:11-13`), replaced
by the checked form once B5 exposes the function.

### 5.2 Integration tests

Covered by T-C2-8/9/10/11 (router `oneshot`) plus A2's
`every_route_renders_the_full_shell` (A2 I-1), which is extended to include the
three glossary routes so they inherit the shell contract (skip link, header, nav,
footer, vitals, one `<h1>`, meta set).

### 5.3 UI / E2E tests

**Absent, deliberately.** There is no browser-automation harness in the repo and
the feature has zero JavaScript, so the behaviours E2E would cover (jump-bar
navigation, filter links, sidebar open/close) are native HTML verified by the
server-byte tests above and the manual pass in §5.4. Adding Playwright to test
native `<details>` and `<a href="#…">` would cost more than it buys — the same
decision A1 §5.3, A2 §5.3, and B5 §5.3 reach.

### 5.4 Visual / manual verification

Per A1's tiered matrix (§5.4). Glossary-specific surfaces on the Tier-1 six themes
(Lunarcore, Solarcore, Paper, Cloud, Solarized, CRT):

- `/glossary` landing, `/glossary/terms` (long `<dl>` with the A–Z bar), and
  `/glossary/commands` (synopsis/example lines).
- A deep link (`/glossary/terms#mtu`) scrolling with the target clear of the
  sticky header.
- `/glossary/terms?cat=networking` and a `?cat=` value with no matches (empty
  state) and `?cat=bogus` (full index).
- Sidebar wide (sticky, active border) and narrow (≤800px, `<details>` floating
  menu open/closed); A–Z bar wrapping at 320px with no horizontal scroll.
- 200% zoom / 24px browser font: definitions reflow, jump bar wraps.
- `prefers-color-scheme: light` with **JS disabled**: light palette, no dead
  control, all links and the filter operable.
- A `/glossary/<unknown>` URL → themed 404 with live nav.

### 5.5 Documentation follows behavior (criterion 5E)

`docs/design/GLOSSARY.md` (new) is updated in the same change as any structural
change and records: the entry contract (term/definition/category/see_also/learn;
command synopsis/purpose/context/example/caution/man), the **seeding-from-/learn**
rule (every term/command originates in a learn page or its practice section), the
anchor-derivation and sort rules, the four drift guards, and the frontmatter-reuse
content-format decision (with its wart). `README.md` gains a one-line glossary
description in the routes/handlers list.

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement.** The glossary stores nothing, reads no
  user input beyond an optional `?cat=` query value (matched against a two-variant
  allowlist and never reflected into HTML), transmits no PII, sets no cookies.
  Content is public, author-written flat files.

### 6.2 Asset provenance

- [x] **Uses third-party assets — text references only, no files.**

| Asset | Source | Licence | Status |
|---|---|---|---|
| Man-page / RFC / IANA links in `learn`/`man` fields | man7.org, IETF, IANA | Cited by hyperlink, not reproduced | Clear — reference, not redistribution (same standard the corpus already uses, `network-protocols.md:122-129`) |
| Markdown/HTML + frontmatter pipeline (`gray_matter`, `pulldown_cmark`) | crates.io | MIT/Apache-2.0 | Already vendored (`Cargo.toml:26-27`) |
| Definitions and command context | Original, edited from Jeff's study notes / the learn corpus | Jeff's | Clear |

No fonts, images, or media are shipped by this feature (CSP forbids external ones
anyway, `security_headers.rs:41-45`).

### 6.3 Language / claims audit

- [ ] Claims not supported by evidence — **no.** Entries are factual definitions
  and general command usage. Every entry is **seeded from the learn corpus** (a
  bolded term or a `Suggested practice` command), so nothing is asserted that the
  site does not already teach. The landing copy is quiet and non-strategic per the
  copy-voice memory: "Definitions and commands from the Learn pages, gathered in
  one place." — **no** "evidence-first", "in training", "aimed at … roles", or
  disclaimer section.
- [ ] Capabilities not yet built read as shipped — **no.** The whole feature is
  marked **absent/planned** in this spec (§7); nothing in user-visible copy implies
  it exists yet. Command `context` describes general usage ("reach for it to see
  …"), never "I run this in production" — it must not imply operated experience
  Jeff has not documented (criterion 1E).
- [ ] Domain-restricted language — **no offensive-security / red-team / pentest /
  production-grade / SRE / enterprise identity** appears. Categories are
  `networking` and `linux`; command examples come from the corpus's owned-lab
  framing ("On a network you own or are authorized to inspect",
  `network-protocols.md:97`). The `description()`/`<meta>` copy aligns with the
  live cert spine (RHCSA → CCNA → Security+, criterion 1D) and is pinned by
  T-C2-14.

### 6.4 Regulatory alignment (criteria.md Lens 3)

- **3A no-JS floor:** met by construction — zero JavaScript, native anchors,
  server-side filter (§3.7A, T-C2-8). *Auto-fail avoided.*
- **3B contrast / colour independence:** all A1 tokens, category shown as a word,
  active state has a non-colour border, empty jump-letters are non-links, no
  sub-0.70rem text (§3.7B). *Auto-fail avoided.*
- **3C keyboard / focus:** all-native `<a>`/`<summary>`, A1 focus ring (§3.7C).
- **3D semantics:** real `<dl>` term/definition pairs; `<h1>`-before-`<h2>`
  outline (T-C2-11); `aria-current` on the active nav entry (T-C2-12); distinct
  `<nav>` landmarks.
- **3E motion:** none introduced (§3.5). *Auto-fail avoided.*
- **3F responsive / resilient:** 320px → wide; filtered-empty and empty states
  designed (§3.3, §5.4).

**Auto-fail rule 1 (unearned claims):** avoided — a factual reference seeded from
existing corpus, cert spine correct, no offensive-security identity, and every
"Learn more" link points only at published work (T-C2-5).

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**Absent.** No glossary exists in any form. Verified this session:

| Area | Current state | Evidence |
|---|---|---|
| Routes | No `/glossary*` route registered | `router.rs:38-58` lists home/about/portfolio/blog/learn/wiki/blog-slug/releases/status/well-known only |
| Handler | No glossary handler or module | `src/handlers/mod.rs:3-8` declares blog/pages/releases/status/well_known/wiki |
| Model | No glossary model | `src/models/mod.rs:1-3` declares page/post/project |
| Templates | No glossary template | `templates/` has no `glossary*.html` |
| Content | No glossary content | `content/` has `posts/`, `pages/`, `drafts/` — no `glossary/` |
| CSS | No `.glossary-*` rules | reusable `.wiki-*`/`.post-content`/`.tag`/`.back-link` exist and will be reused |
| Tests | No glossary tests | `tests/` has `wiki_pages.rs` only |

The **inputs**, however, already exist and are strong: the 12 learn pages define
the terms and commands the glossary will index (e.g.
`linux-abstraction-layers.md:24-129` for Linux terms, `network-protocols.md:10-93`
for networking terms, and the `Suggested practice` sections across the corpus for
commands — `ps -ef --forest`, `free -h`, `vmstat 1 5`, `strace -f`, `dig`,
`curl -v`, `tcpdump`, `ip route`, `ss -tln`, `nmcli dev wifi list`, `ipcalc`, etc.,
gathered from `content/pages/*.md`). The glossary is therefore an **index of
existing, reviewed material**, not new content invention — which is what keeps it
inside the claim-integrity boundary (§6.3).

### 7.2 Delta to spec

**New files:**
- `src/models/glossary.rs`, `src/handlers/glossary.rs`
- `templates/glossary_landing.html`, `templates/glossary_index.html`
- `content/glossary/terms.md`, `content/glossary/commands.md`
- `tests/glossary.rs`
- `docs/design/GLOSSARY.md`

**Modified files:**
- `src/router.rs` — three `get(...)` routes before `ServeDir`/`fallback`
- `src/handlers/mod.rs` — `pub mod glossary;`
- `src/models/mod.rs` — `pub mod glossary;`
- `static/css/style.css` — new `.glossary-*` block (reusing existing tokens/patterns)
- `README.md` — one-line glossary description
- **Cross-feature (A2):** add `Section::Glossary` to the shell enum + (optionally)
  the primary `NAV`; extend A2 I-1 to cover the three routes
- **Cross-feature (B5):** consume `wiki::sidebar_slugs()` for T-C2-5
- **`/learn` corpus (optional):** replace some inline re-definitions with
  `/glossary/terms#…` deep links once the glossary ships (single-sourcing, 5A)

**No migrations, no new dependencies** (recommended path), **no new assets.**

### 7.3 Estimated scope

**M.** Two new modules, two templates, one small CSS block, two curated content
files, and a test module — each individually small and modelled closely on the
existing learn feature. The largest single item is **authoring the two content
files** to the pedagogical bar (criterion 2C): definitions that teach in plain
language and command context that reads as real usage, not a man-page paste. The
code scaffolding is S; the content-plus-drift-guards is what pushes the whole to M.
It is deliberately **not L** because it reuses the learn feature's proven shape and
adds no new dependency or infrastructure.

### 7.4 Blocking dependencies

- **A2 site-shell** — `Section::Glossary` (or the interim magic-string
  `section()`), the `head_extra` `og:type` hook, and the `" — machinageist"` title
  contract. Until A2's `Section` enum lands, the interim path (§4.3) keeps the
  glossary shippable and honest (off-nav, nothing highlights).
- **B5 learn** — `wiki::sidebar_slugs()` for the T-C2-5 cross-link guard (interim:
  a documented golden list), and the learn corpus itself as the cross-link targets
  and the seed for entries. B5 must ship before the corpus is stable enough to
  single-source against.
- **A1 design-system** — the `--text-2xs` floor, the audited `.tag`/`--text-*`
  tokens, and the measurement layer. C2 conforms to A1's contract and adds no
  literal.
- **Not blocked by** `C1` (search) or `C3` (study tools) — those consume the
  glossary, not the reverse.

---

## 8. Open Questions

- **Q1 (content format / dependency):** Recommended path reuses the in-tree
  `gray_matter` YAML parser via frontmatter-only files (zero new dependency) at the
  cost of ceremonial `---` fences around a data file. The alternative is one new
  maintained data-format crate (`toml`, or `serde_yml` — not deprecated
  `serde_yaml`). Confirm the zero-dependency reuse, or approve adding a crate? —
  blocks: §4.2/§4.5 and the shape of `content/glossary/*`.
- **Q2 (inline Markdown in definitions):** Should definitions support inline
  Markdown (`` `code` ``, links, emphasis) via the existing pulldown-cmark pipeline
  (richer, but a `|safe` trust surface to document), or render as plain escaped
  text (simpler, no render step)? Recommendation: allow inline Markdown — the
  corpus leans on inline `code` and links heavily — with the trust boundary
  documented as B5 §4.6 does. — blocks: §4.6.
- **Q3 (nav placement):** Should the glossary get a **5th primary nav item**
  ("Glossary"), or stay off-nav and be reached from `/learn` (sidebar + overview)
  and from in-body cross-links? Recommendation: **off-nav** to protect nav
  restraint (criterion 2E), surfaced prominently from the education cluster; revisit
  if analytics show readers cannot find it. This is the parent-feature (`C2`)
  discoverability decision. — blocks: A2 `NAV`, `base.html:24-27`.
- **Q4 (commands: A–Z vs. task grouping):** Commands are specified as A–Z with a
  category tag (consistent with terms, predictable). An alternative is grouping by
  task ("Processes & memory", "Networking & DNS", "Files"). Recommendation: keep
  A–Z for launch — it is the more predictable lookup and shares the terms layout —
  and reconsider task grouping only if the command count grows large. — blocks:
  §3.3 command layout.
- **Q5 (sub-feature — glossary growth pipeline):** New entries arrive by reviewing
  learn pages as they are added (the same publish discipline as B5 §4.4). Should
  the gauntlet track glossary-content production as a separate content sub-feature,
  or treat it as ongoing authoring outside the spec set? Flagged here per the
  no-sub-agents rule. — blocks: nothing in this spec; scope decision only.
- **Q6 (bidirectional `see_also`):** Should the drift guard require `see_also` to
  be symmetric (if A → B then B → A)? Recommendation: **no** — asymmetric
  references are often correct (a specific term points at its general parent but not
  vice-versa); enforce only that every reference *resolves* (T-C2-4), not that it is
  mirrored. — blocks: §5.1 T-C2-4 strictness.
