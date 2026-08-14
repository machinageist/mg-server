# Spec: Search

**Feature ID:** `C1` / `search`
**Parent feature:** root (New capabilities)
**Spec author agent:** spec-agent-10 (Claude Opus 4.8)
**Date:** 2026-08-09
**Iteration:** 1

---

## 0. Reading notes and scope boundary

Everything asserted about current state below was read from source, not from docs.
Citations are `path:line` or `path:line-range`. This feature is **absent** — no
route, handler, template, model, index, or CSS for it exists (verified: no
`search` token in `src/`, `templates/`, `docs/`, or `style.css` outside the
gauntlet tree). The spec therefore designs the **target** first, then measures the
gap from a standing start.

**In scope (C1 owns):** a server-rendered `GET /search?q=` route; the search page
and its results view (`templates/search.html`); the query engine that scans the
site's *published* content (`src/search.rs`); the plaintext-extraction the engine
needs from the two existing content models; result ranking, snippet generation,
and safe query/snippet echoing; the search-specific CSS (form, results list,
`<mark>`); and the drift/XSS/no-JS tests that keep all of it honest.

**Out of scope, referenced only:**

| Concern | Owner | What C1 assumes from it |
|---|---|---|
| Token architecture, type scale, `--measure`, `--text-*`, contrast audit, 23-theme roster, **`mark` styling**, form-control styling | `A1` design-system | The prose surface, measure, and focus ring exist and are AA-audited; A1 owns the *values* for the new `input`/`button`/`mark`/`.search-*` rules. C1 files these as a cross-feature request (§7.4) and adds no colour literal. |
| `base.html` shell, `Section` enum, nav active-state, `head_extra`/`scripts` blocks, skip link, footer, `og:*` meta, `title()`/`description()`/`section()` contract | `A2` site-shell | C1's page renders through `base.html`; a new `Section::Search` and an optional header search affordance are A2 cross-feature deps (§7.4). Interim, `section()` returns the string `"search"`. |
| CSP, security headers, rate limiting, HTML cache policy, vitals | `A3` ops | CSP is `default-src 'self'; script-src 'self'; style-src 'self'` (`security_headers.rs:39-50`) — the no-JS floor is a hard constraint. The global rate limiter (`router.rs:72-75`) already fronts every route including this one. |
| The blog corpus (`content/posts/*.md`) and `/blog/:slug` routes | `B4` writing | C1 indexes every post and links to `/blog/{slug}`; every post file is servable there (`blog.rs:147-158`). |
| The learn corpus (`content/pages/*.md`), the `SIDEBAR` allowlist, and `/learn/:slug` routes | `B5` learn | C1 indexes **only** pages in the `SIDEBAR` allowlist and links to `/learn/{slug}`; it consumes the `pub fn sidebar_slugs()` B5 §5.1 T-B5-4 proposes so search and `/learn` agree on what is publishable (§4.1). |
| Glossary terms/commands | `C2` | A future corpus; C1's engine is designed so a third document kind slots in without redesign (§4.2). |

**Explicitly excluded from the index (auto-fail-adjacent, Lens 1C):**
`content/drafts/**` — including `content/drafts/geist-wiki/` (GeistScope project
docs) and every staged post — is **never** indexed. The corpus is defined by
*routable* content only (`POSTS_DIR` + `sidebar_slugs()`), which structurally
excludes drafts because no route reads `content/drafts/`. This is stated as an
invariant and pinned by a test (§5.1 T-C1-8) so search can never leak unpublished
or gated work.

---

## 1. Purpose

### 1.1 One-sentence job

Let a reader type a word or phrase and get back a ranked, snippet-previewed list
of the site's published writing and learn pages that mention it — as a plain
server-rendered page reachable from a shareable URL, with JavaScript disabled.

### 1.2 Why it matters

The site has crossed the size where "read the nav and scroll" stops being a
retrieval strategy. There are four blog posts and thirteen learn pages today
(~21,000 words across seventeen documents), growing as reviewed study and lab
notes move into `/learn` (`content/pages/index.md:66-67`). Three specific pains:

1. **The best evidence is buried in bodies, not titles.** The site's Lens-4
   differentiator is verification-and-recovery writing — the network-migration
   post is "an outage worked end to end" (criterion 4B). But an engineer who
   heard "ask him about the corosync incident" cannot find it from the nav; the
   word `corosync` lives in that post's tags and body
   (`management-layer-first-network-migration.md` frontmatter), not in any menu.
   Search is how a specific reviewer reaches the specific artifact that will
   impress them.

2. **The learn corpus is a reference, and references are searched, not read
   front-to-back.** The Lens-2 benchmark tier (Arch Wiki, MDN) is defined by
   "scannability and cross-linking" — a reader who wants "encapsulation" or
   "twisted pair" should land on the exact page, not skim eleven sidebar entries.
   `/learn` already cross-links (B5 §1.2); search is the other half of
   re-findability (criterion 2D).

3. **The obvious way to build search is the wrong way for this site.** Every
   junior portfolio that adds search reaches for a client-side JS index
   (Lunr/Fuse/Algolia widget) that dies with scripts off and violates the site's
   strict CSP. Doing it as a server-rendered `GET` form is *itself* a craft
   signal to the engineer-peer reader: it is the same "works without JavaScript,
   costs zero third-party bytes" discipline the vitals strip trades on (A2 §1.2).
   The feature-tree names this explicitly: "Server-rendered `/search?q=` over
   `content/posts/` + `content/pages/` fits the no-JS floor" (`feature-tree.md:38`).

### 1.3 Success signal

**Primary (observable):** with JavaScript fully disabled, a reader can type a term
into the search box, submit it, and receive a ranked results page at a shareable
`/search?q=term` URL, where every result links to a page that returns HTTP 200,
the matched term is visibly highlighted in each snippet, and a query with no
matches produces a designed empty state with a way onward — all in a colour scheme
that respects the OS preference, with no control on screen that does nothing.

**Secondary (measurable):** `cargo test --all-targets` passes, including the
no-JS contract test (§5.1 T-C1-6), the query-echo XSS-escape test (T-C1-5), the
every-result-URL-resolves test (T-C1-7), and the drafts-excluded test (T-C1-8),
on a tree where the search corpus is derived from `POSTS_DIR` and
`sidebar_slugs()` and from nothing else.

---

## 2. User Stories

> **Happy path — hiring manager with a lead.** As a hiring manager who was told
> "he wrote up a real outage," I want to search "outage" or "corosync" and land
> directly on the network-migration post, so that I can verify the claim in
> seconds instead of reading the whole blog.

> **Happy path — engineer peer.** As a working engineer, I want to search a
> technical term like "encapsulation" or "CSP" and get the exact learn page or
> post that explains it, with a snippet showing the term in context, so that I can
> judge the depth of the material without opening every result.

> **Happy path — self-directed learner.** As someone studying networking, I want
> to search "subnet" and find the IPv4 addressing page even though "subnet" is not
> in the sidebar label, so that I can navigate by concept rather than by the
> author's page titles.

> **Edge case — no results.** As a visitor who searches for something the site
> does not cover ("kubernetes"), I want a clear "no results" page that does not
> look broken and offers me the blog and learn indexes, so that a miss is a
> redirect, not a dead end.

> **Edge case / security — hostile query.** As a security-minded reviewer (or an
> attacker), I want a query containing `<script>` or an over-long string to be
> safely escaped and bounded, so that the search box is not an XSS or DoS vector —
> and if it were, that would tell me the whole site's security posture is
> theatre.

> **Accessibility — no JavaScript / keyboard / screen reader.** As a reader on a
> text browser, with JS blocked, or using a screen reader, I want the search form
> to be a real labelled `<input>` in a `search` landmark, submittable with Enter,
> and the results to be an ordered list with an announced count, so that search is
> fully usable without a mouse or scripts.

> **Maintainer.** As the person publishing a new post or learn page, I want it to
> become searchable automatically because search reads the same source of truth as
> the routes, and I want a test that fails if a search result could ever point at
> a 404 or at a draft, so that search cannot silently drift from what the site
> actually serves.

---

## 3. UX Specification

### 3.1 Screen / view inventory

| Surface | Path to reach | New / modified | Layout pattern |
|---|---|---|---|
| **Search page — empty** | `/search` (no `q`, or blank/whitespace `q`) | **New** | Standard 900px `<main>` column: `<h1>`, search form, short helper line, links to `/blog` and `/learn`. No results region. |
| **Search page — results** | `/search?q=term` (non-empty match set) | **New** | Same column: `<h1>`, form (input pre-filled with the query), result count line, ordered list of results (title link + meta + highlighted snippet). |
| **Search page — no results** | `/search?q=term` (empty match set) | **New** | Same column: `<h1>`, form (pre-filled), designed empty state, links onward. |
| **Header search affordance** | Present on all routes | **New, A2-owned (cross-feature, §7.4)** | Recommended: a small "Search" nav link (cheapest) or a compact `GET` form in the header. C1 works fully without it via the `/search` page; this is an enhancement to *discoverability*, not to *function*. |
| **404 recovery search** | Any 404 page | **New, A2-owned (cross-feature, §7.4)** | Optional: a search form on the themed 404 so a mistyped URL can be recovered by searching. Deferred to A2; not required for C1 to ship. |

No modals, sheets, drawers, or popovers. Search is a page, not an overlay — which
is precisely what makes it work with no JavaScript and produce shareable URLs.

### 3.2 Interaction flows

**Flow A — search from the search page (primary, JS-independent).**

1. Reader navigates to `/search` (via a nav link, the footer, a bookmark, or a
   typed URL). `search::search` handler renders `search.html` with an empty query
   and no results region.
2. Reader types into `<input type="search" name="q">` and presses Enter or clicks
   the "Search" submit button.
3. The `<form method="get" action="/search">` performs a **full-page GET
   navigation** to `/search?q=<term>`. This is the whole mechanism: no fetch, no
   XHR, no client routing. The URL is now shareable and bookmarkable.
4. The handler parses `q`, builds the corpus, ranks matches, and re-renders
   `search.html` with the query pre-filled and the results list populated.
5. Reader clicks a result title → full-page navigation to `/blog/{slug}` or
   `/learn/{slug}`.

**Flow B — empty or blank query.** `q` absent, empty, or all-whitespace →
the handler skips the scan entirely and renders the empty-state view (no "0
results" language, because the reader has not searched yet). Helper copy points at
`/blog` and `/learn`.

**Flow C — no matches.** `q` non-empty but no document matches → the results view
renders "No results for {{ q }}" (query auto-escaped by Askama) plus the same
onward links as the empty state, so the page never looks broken.

**Flow D — hostile / degenerate query.** `q` longer than the cap is truncated to
the first 128 characters before scanning (bounds work, §4.7). `q` containing HTML
metacharacters is echoed only through Askama's auto-escaping (`{{ q }}`) and,
inside snippets, through the escape-then-mark pipeline (§4.3) — never with `|safe`
over raw input. A query with no alphanumeric token (e.g. `"!!!"`) yields the
no-results view.

**Cues.** No haptics, no sound, **no animation** (§3.5). Navigation is a normal
document load; the browser's own loading indicator is the only motion.

### 3.3 Layout descriptions

**Search page** (`templates/search.html`, extends `base.html`), top → bottom:

1. `<h1>Search</h1>` — the single page heading (A2 U-4: exactly one `<h1>`).
2. `<form class="search-form" role="search" method="get" action="/search">`
   - `<label class="vh" for="search-q">Search the site</label>` — visually hidden
     (the `.vh` utility A2 §4.2 defines); the field's purpose is also obvious from
     placeholder + button, but the programmatic label is required for AT.
   - `<input id="search-q" name="q" type="search" value="{{ q }}"
     placeholder="Search posts and learn pages" maxlength="128"
     autocomplete="off" spellcheck="false">`
   - `<button type="submit">Search</button>`
   - Data source: `q` is the echoed, escaped query string.
3. **When a search ran** (`q` non-blank): a result-count line — `<p
   class="search-count">{{ n }} result(s) for "{{ q }}"</p>` — sized at body/muted
   and inside `--measure-narrow`.
4. **Results** (`n > 0`): `<ol class="search-results">`, each `<li
   class="search-result">`:
   - `<a class="search-result-title" href="{{ result.url }}">{{ result.title }}</a>`
   - `<p class="search-result-meta">` — a kind label ("Writing" / "Learn"), the
     date, and (for posts) the category pillar. Kind is a **word**, not a colour
     (Lens 2E / 3B).
   - `<p class="search-result-snippet">{{ result.snippet_html|safe }}</p>` — the
     escaped snippet with `<mark>`-wrapped matches (§4.3). `|safe` is sound here
     **only because** the snippet is assembled by escaping every character first
     (§4.3, T-C1-5).
   - Data source: one `SearchResult` per ranked hit from `src/search.rs`.
5. **Empty / no-results state** (§3.6 E-01/E-02): one `--text-muted` paragraph at
   body size inside `--measure-narrow`, then `<a href="/blog">All writing</a>` and
   `<a href="/learn">Learn</a>`. No placeholder cards, no spinner (A1 §3.3 empty-
   state rule).

**Data sources.** Colour/font from the active `[data-theme]` block (A1); size,
spacing, measure from A1's measurement layer; results and snippets from
`src/search.rs::SearchIndex::query()`; the echoed query from the request. No
component reads a colour literal.

### 3.4 Input & gestures

- **Pointer.** Click the input to focus, click "Search" to submit, click a result
  title to navigate. Nothing is hover-only.
- **Keyboard.** The input is a native focusable control; Enter submits the form
  from the input (native form behaviour, no JS). The submit button and every
  result link are in the tab order and operable with Enter/Space. Focus ring is
  A1's global `:focus-visible` (2px accent outline, `style.css:685`). No
  feature-specific keyboard shortcuts (a `/`-to-focus accelerator would need JS
  and would trip WCAG 2.1.4; explicitly declined).
- **Touch.** The input and button are full-size tap targets; `type="search"`
  surfaces the search-optimised mobile keyboard with a "Search" action key that
  submits the form natively.
- **Responsive.** The form is a flex row that wraps below A2's 640px breakpoint so
  the input takes the full width and the button drops beneath it; the results
  list is single-column at every width (it already is). No new breakpoint.
- **Stylus / controller / voice / camera.** N/A — text and links only.

### 3.5 Transitions & animation

**None.** C1 introduces zero motion — no result fade-in, no expand/collapse, no
skeleton. This is deliberate: results appear on a fresh document load, so there is
nothing to animate, and A1 §3.5 rule 3 forbids body-content animation. The only
motion touching the page is A1's chrome transitions (theme swap, focus), which are
already `prefers-reduced-motion`-gated. The existing `input, select, textarea`
colour-transition rule (`style.css:739`) applies to the new input on theme change
and is already inside the `no-preference` block — so the reduced-motion path is
correct with no new work. **Reduced-motion alternative: absence, and it is already
the default.**

### 3.6 Error states

| ID | Trigger | Presentation | Why that presentation | Recovery | Data loss |
|---|---|---|---|---|---|
| **E-01** | `q` absent/blank/whitespace | **Inline** empty state on the search page (not an error — the reader has not searched) | It is the resting state of the page, not a failure; a banner/toast would be wrong | Type a query; links to `/blog` + `/learn` | No |
| **E-02** | `q` non-empty, zero matches | **Inline** "No results for {q}" + onward links | A miss is expected and common; full-page framing keeps the form to retry | Refine the query; onward links | No |
| **E-03** | `q` exceeds 128 chars | Silently truncated to 128, then searched | A length cap is a resource bound, not a user error; interrupting the reader would be worse than quietly bounding the work | Automatic | No |
| **E-04** | A corpus file fails to load/parse at query time (`SiteError::Io`, `MissingFrontmatter`, etc.) | **The search degrades to a partial index; it does not 500.** The offending document is skipped and logged; results from the rest still render | Search is a convenience surface; one malformed post must not take down retrieval for the whole site. Mirrors the home page's degrade-to-empty posture (`pages.rs:52-57`) | The remaining results render; the malformed file is caught in CI (B4/B5 parseable-page guards) before it ships | No |
| **E-05** | JS unavailable / blocked | **No degradation.** The form is a native `GET`; results are server-rendered HTML | The no-JS floor is the feature's entire identity (auto-fail rule 3) | Search normally | No |
| **E-06** | Query is echoed containing HTML/script | Escaped everywhere: `{{ q }}` (Askama auto-escape) in the input value, count line, and no-results copy; snippets via escape-then-mark (§4.3) | An unescaped echo would be reflected XSS on a strict-CSP site — a claim-integrity catastrophe (§6.1). Escaping is the presentation | N/A — never reaches the user as live markup | No |

**Presentation justification.** E-01/E-02 are inline because they are states of the
search page itself, not overlays on other content. No toast exists on the site
(A1 §3.6) and none is proposed — a toast needs JS to appear and dismiss, which is
behind the no-JS floor.

### 3.7 Accessibility

Graded as an auto-fail gate (rules 2 and 3). Written as invariants + the target
that satisfies each (the whole feature is new, so there is no "shipped state").

**A. Works without JavaScript (auto-fail rule 3).** ✅ **Met by construction and it
is the headline property.** The form is `<form method="get" action="/search">`
with a real `<input>` and `<button type="submit">`; results are server-rendered
HTML. No script is added, referenced, or required. Pinned by §5.1 T-C1-6 (strip
`<script>` from the response; assert the form, every result URL, and the snippets
survive).

**B. Contrast & colour independence.**
- Result **kind** is a word ("Writing"/"Learn"), never a colour swatch alone.
- `<mark>` highlighting must not rely on background colour alone (a `<mark>` with
  only a themed background could fail AA in some of the 23 themes and would vanish
  for a reader who overrides background colours). **Target:** style `mark` with a
  colour-independent cue — bold weight **plus** a 2px accent underline — in
  addition to any subtle themed background, and audit the pair at 4.5:1 in all 23
  themes. This is an A1 cross-feature request (§7.4); C1 uses only tokens.
- Input border, submit button, and placeholder must clear AA at their rendered
  size in all 23 themes; placeholder text carries no essential information (the
  `<label>` and button do), so a lower-contrast placeholder does not gate meaning.

**C. Focus & keyboard.** All controls are native and keyboard-operable with A1's
global visible focus ring. Focus order follows DOM order: skip link → header →
`<input>` → submit → first result link → … → footer. No focus trap, no
`tabindex > 0`.

**D. Semantics & assistive technology.**
- The form is wrapped in a `search` landmark via `role="search"` on the `<form>`
  so screen-reader users can jump to it.
- The `<input>` has a programmatic name via `<label for="search-q">` (visually
  hidden) — never placeholder-as-label.
- Results are an `<ol>` (ranked order is meaningful), each result title a real
  `<a>`; the AT user hears "list, N items".
- The result count is plain server-rendered text in a `<p>` before the list; on a
  full page load it is read as part of the document. **No `aria-live` is used or
  needed** — there is no dynamic update to announce (that is the no-JS design
  paying off).
- `<mark>` is semantic: AT may announce "highlight", and the match is *also*
  conveyed by the surrounding snippet, so the highlight is not the only signal.

**E. Heading outline.** Exactly one `<h1>` ("Search"). The result titles are
`<a>` inside `<li>`, **not** headings — they do not compete with the page outline
(a common mistake; a results list is a list, not a heading tree). Pinned by A2 U-4.

**F. Motion & sensory safety.** None introduced (§3.5). No autoplay, no flashing.

**G. Responsive & resilient.** Works 320px → wide (form wraps at 640px, §3.4);
at 200% zoom / large browser font the prose and results reflow (sizes are `rem`,
A1 §3.7F). Empty and no-results states are designed (§3.3, §3.6), not accidental.

---

## 4. Implementation Specification

### 4.1 Architecture placement

```
src/
  search.rs               ← NEW crate module (sibling of state.rs/errors.rs):
                            SearchDoc, DocKind, SearchIndex, SearchResult, ranking,
                            snippet + highlight. Aggregates the two content models;
                            reads the same sources of truth the routes read.
  main.rs                 ← add `mod search;` (main.rs:24-30 module list)
  handlers/
    search.rs             ← NEW: SearchQuery extractor, SearchTemplate, handler
    mod.rs                ← add `pub mod search;` (mod.rs:3-8)
  handlers/blog.rs        ← expose POSTS_DIR (already pub(crate), blog.rs:30) — no change
  handlers/wiki.rs        ← consume `pub fn sidebar_slugs()` (B5 dep) for the page corpus
  models/post.rs          ← add `content_text: String` field, populated in from_file (§4.2)
  models/page.rs          ← add `content_text: String` field, populated in from_file (§4.2)
  router.rs               ← register `.route("/search", get(search::search))` (router.rs:36-61)
templates/
  search.html             ← NEW: form + count + results/empty, extends base.html
  base.html               ← (A2) optional header search affordance; new Section::Search
static/css/style.css      ← NEW `.search-form`, `.search-results`, `.search-result*`,
                            and `mark` rules (values owned by A1, §7.4)
tests/
  search.rs               ← NEW integration test crate: URL-resolves, drafts-excluded,
                            ranking, no-JS, XSS-escape (mirrors tests/wiki_pages.rs)
docs/design/SEARCH.md     ← NEW long-lived doc: corpus definition, ranking contract,
                            snippet/escape invariant, the no-JS decision
```

`src/search.rs` is a **new top-level module**, justified exactly as A2 justified
`src/shell.rs`: it aggregates two existing models plus ranking and snippet logic
that belong to none of them, and it must not live inside a single content model
(it spans both). The handler stays thin (mirrors `blog.rs`/`wiki.rs`): extract
query, call the engine, render the template.

### 4.2 Data model

**No database, no migrations** — the site has no persistence layer; the corpus is
read from disk (§4.4 discusses per-request vs. startup index).

**Two minimal, single-source additions to the content models.** Search needs the
document *body as plaintext* to match on and to snippet from. The models today
expose only `content_html` (`post.rs:61`, `page.rs:36`), which is HTML — matching
against it would match inside tags and produce broken snippets. The clean fix is
one field, populated during the parse the model already does, so there is exactly
one definition of a document's searchable text (Lens 5A):

```rust
// In models/post.rs BlogPost and models/page.rs Page — one added field each:

/// Body rendered to plain text for search matching and snippets.
/// Built from the same pulldown-cmark parse as content_html by folding the
/// Text and Code events into a string; carries no markup, so search never
/// matches inside a tag and snippets never leak HTML.
pub content_text: String,
```

Populated inside each `from_file` alongside `content_html`
(`post.rs:97-99`, `page.rs:58-60`) with a second, text-only pass over the same
`parsed.content`:

```rust
// Verb + noun: extract plain text for the search index
let mut content_text = String::new();
for event in Parser::new_ext(&parsed.content, Options::all()) {
    match event {
        Event::Text(t) | Event::Code(t) => {
            content_text.push_str(&t);
            content_text.push(' ');
        }
        _ => {}
    }
}
```

*Rejected alternative (recorded):* a standalone loader in `src/search.rs` that
re-reads and re-parses each file independently. It would avoid touching the models
but would **duplicate the frontmatter+markdown parse**, which is the exact drift
class Lens 5A warns against — two parsers that can disagree. The one-field
addition keeps a single source of truth and costs a second cheap event-fold over
already-loaded content.

**The search engine types** (`src/search.rs`):

```rust
// Author:      machinageist
// Date:        2026-08-09
// Description: In-memory search over the site's published content. Builds a
//              corpus from the same sources the routes serve (posts + sidebar
//              pages), ranks case-insensitive term matches with field weighting,
//              and returns escaped, highlighted snippets. No external index, no
//              JavaScript, no database.
// Notes:       The corpus is defined by ROUTABLE content only. Drafts are never
//              read, so unpublished/gated work cannot be indexed.

// Which surface a hit came from — decides the URL and the human kind label
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocKind { Post, Page }

// One indexed document, flattened from BlogPost or Page
pub struct SearchDoc {
    pub kind: DocKind,
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub category: Option<String>, // posts only
    pub date: chrono::NaiveDate,
    pub body: String,             // content_text
}

// One ranked result handed to the template
pub struct SearchResult {
    pub url: String,          // "/blog/{slug}" or "/learn/{slug}"
    pub title: String,        // escaped by Askama on render
    pub kind_label: &'static str, // "Writing" | "Learn"
    pub date: chrono::NaiveDate,
    pub category: Option<String>,
    pub snippet_html: String, // PRE-ESCAPED text with <mark> spans (§4.3)
    pub score: i32,           // for tests/ordering; not rendered
}

pub struct SearchIndex { docs: Vec<SearchDoc> }
```

**Corpus construction — the single-source rule (Lens 5A).**

```rust
impl SearchIndex {
    // Build the corpus from ROUTABLE content only — the same sources the routes read
    pub fn build() -> Self {
        let mut docs = Vec::new();
        // Posts: every file under POSTS_DIR is servable at /blog/:slug
        if let Ok(posts) = BlogPost::load_all(Path::new(blog::POSTS_DIR)) {
            docs.extend(posts.into_iter().map(SearchDoc::from_post));
        }
        // Pages: ONLY the SIDEBAR allowlist is servable at /learn/:slug
        for slug in wiki::sidebar_slugs() {         // B5 §5.1 T-C1 dep
            if slug == "index" { continue; }        // overview is /learn, not a topic hit
            if let Ok(page) = Page::find(Path::new(wiki::PAGES_DIR), slug) {
                docs.push(SearchDoc::from_page(page, slug));
            }
        }
        SearchIndex { docs }
    }
}
```

This is why a search result can never 404 and never surface a draft: the post
corpus is exactly `POSTS_DIR` (which `/blog/:slug` serves), and the page corpus is
exactly `sidebar_slugs()` (which `/learn/:slug` allowlists, `wiki.rs:157-166`).
`content/drafts/**` is read by no route and by no branch above.

**Ranking contract** (`SearchIndex::query(&self, raw_q: &str) -> Vec<SearchResult>`):

1. Normalise: trim, truncate to 128 chars, lowercase, split on whitespace into
   terms; drop empty/punctuation-only terms. If no terms remain → return empty.
2. A document is a **candidate** iff **every** term appears (case-insensitively)
   in at least one of its searchable fields (title, summary, tags, body) — AND
   semantics, so multi-word queries narrow rather than widen.
3. Score each candidate = Σ over terms of the **best field weight** the term hit,
   plus a small body term-frequency bonus (capped):
   - title: 6, tags: 4, summary: 3, body: 1 (+1 per extra body occurrence, capped
     at +5 per term). Weights live in named constants, never inline (Jeff's Rust
     conventions).
4. Sort by score descending, then `date` descending as a stable tiebreak
   (newest wins), then slug ascending for determinism.
5. Truncate to `MAX_RESULTS` (20 — larger than the whole corpus today, so
   effectively "all matches," but a defined bound for growth).

Case-insensitive substring matching (not stemming, not fuzzy) is the right
altitude for a seventeen-document corpus: it is predictable, needs no dependency,
and is trivially testable. Stemming/fuzzy matching is recorded as a deferred
option in §8, not built (simplicity-first).

### 4.3 API contracts

**Route.**

| Route | Handler | Query | Returns | Errors | Auth |
|---|---|---|---|---|---|
| `GET /search` | `search::search` | `Query<SearchQuery>` where `SearchQuery { q: Option<String> }` | `SearchTemplate` (200) | None — a malformed/absent `q` is the empty state, not an error; a corpus load failure degrades (E-04), never 500s | none (public) |

`q` is a **search term, not a path** — it never touches the filesystem, so the
path-traversal validation `/blog/:slug` needs (`blog.rs:150-152`) does not apply
here. The only hardening `q` needs is length-bounding (§4.7) and output escaping
(below). Axum's `Query` extractor URL-decodes it; a missing `q` deserialises to
`None` (hence `Option`), so `/search` with no query string is valid.

**Template contract (A2 S-1).** `SearchTemplate` implements:

| Method | Value | Rule |
|---|---|---|
| `title()` | `"Search — machinageist"` when idle; `"Search: {q} — machinageist"` when a query ran (q escaped by Askama in the tag) | A2 U-5 (ends in `" — machinageist"`) |
| `description()` | `"Search the writing and learn pages on machinageist.dev."` | 50–160 chars; static copy, claim-clean (§6.3) |
| `section()` | `"search"` now; `Section::Search` after A2 lands (§7.4) | No nav link matches it today, so nothing highlights — correct until A2 adds a Search affordance |

**The snippet escape-then-mark pipeline (the security-critical contract).** The
snippet is the one place raw text is rendered with `|safe`, so it MUST be
assembled so that no byte of query or content can become live markup:

```rust
// Verb + noun: build a highlighted, fully-escaped snippet around the first match
// Contract: caller renders the result with |safe. Every character of `text` and
// every matched term is HTML-escaped BEFORE any <mark> is introduced, so the only
// live tags in the output are the <mark>/</mark> this function emits.
fn highlight_snippet(text: &str, terms: &[String]) -> String {
    // 1. Choose a window (~30 words) centred on the first term occurrence in `text`.
    // 2. Walk the window splitting it into [plain, match, plain, match, ...] runs
    //    on case-insensitive term boundaries (over the ORIGINAL bytes).
    // 3. HTML-escape each run independently (&, <, >, ", ').
    // 4. Join: escaped-plain + "<mark>" + escaped-match + "</mark>" + ...
    // 5. Prefix/suffix a "…" when the window is not at a text boundary.
}
```

Escaping each run *before* concatenation is the invariant — escaping after
inserting `<mark>` would double-escape the marks; matching on escaped text would
miscount offsets. Pinned by T-C1-5 (query `<script>alert(1)</script>` →
output contains `&lt;script&gt;`, contains no live `<script`, and `<mark>` wraps
only the escaped match). This mirrors the existing 404 path-escape guard
(`errors.rs:160-169`).

**Pagination / rate limiting.** No pagination — `MAX_RESULTS` caps the list and
the corpus is tiny (N/A justified: seventeen documents, twenty-result cap).
Rate limiting is A3's global limiter (`router.rs:72-75`), which already fronts
every route; §4.7 bounds per-request work so search is not a disproportionate DoS
lever behind it.

### 4.4 State management

| State | Owner | Lifetime | Sync |
|---|---|---|---|
| The query `q` | The request URL (`?q=`) | Per request | Server reads it; **never stored**, never a cookie |
| The corpus | `SearchIndex::build()` result | Per request (see decision below) | Server only; read from disk |
| Result list | `SearchTemplate.results` | Per request | Server only |
| Theme | `localStorage.theme` (A1/A2) | Browser | Client only |

**Corpus lifetime — decision (per-request build, recorded with its tradeoff).**
The engine rebuilds the corpus on each `/search` request, reading `POSTS_DIR` and
the `sidebar_slugs()` files fresh. This matches the rest of the site's
"always fresh from disk, no content cache" posture (B5 §4.4, `page.rs:47`,
`post.rs:117`) and means a newly published post is searchable the instant its file
lands — no restart, no stale index on a truth-first site.

*Rejected/deferred alternative:* build the index once at startup into `AppState`
(an `Arc<SearchIndex>`). It would save ~seventeen small file reads + parses per
query, but (a) at this corpus size the saving is microseconds behind a rate
limiter, and (b) it reintroduces staleness — the index would lag disk until a
restart, which on a site whose whole value is currency (Lens 1D) is the wrong
default. **Recommendation:** per-request now; revisit the startup index only if
the corpus reaches the low hundreds of pages (recorded in `docs/design/SEARCH.md`
and §8 Q3). This is the same reasoning B5 §4.4 and A1 §4.7 use to reject caching.

**No new state container, offline/draft persistence:** N/A — search stores no
user content and nothing is authored in the browser.

### 4.5 Dependencies

- **New packages: none.** Matching is case-insensitive substring over
  `std::str`; plaintext extraction reuses `pulldown_cmark` (`Cargo.toml`, already
  vendored); frontmatter reuse is via the existing models. **No search crate**
  (Tantivy, Lunr-server, MeiliSearch) is added — each would be a heavy dependency
  and, for client-side ones, a CSP/no-JS violation. Recorded as a rejected option
  in §8.
- **New assets: none.** No images, no fonts, no icons. The CSP
  `default-src 'self'` (`security_headers.rs:41`) forbids external ones anyway.
- **New docs:** `docs/design/SEARCH.md` (corpus + ranking + escape invariant + the
  no-JS/no-cache decisions).
- **Infrastructure: none.** No CDN, no third-party search service, no new
  environment variable.

### 4.6 Platform-specific considerations

- **Browser support.** `<form method="get">`, `<input type="search">`, and
  `<mark>` are universally supported back to legacy browsers; `type="search"`
  degrades to `type="text"` where unknown. No CSS feature beyond what A1 already
  requires. There is no JS, so no `matchMedia`/`:has()` concern originates here.
- **`content_text|safe` boundary.** The snippet is the only `|safe` in this
  feature and is safe *only* via §4.3's escape-then-mark pipeline. Documented so
  the invariant is explicit, not accidental (mirrors B5 §4.6's `content_html|safe`
  note). The query is untrusted input; the body is trusted content; **both** are
  escaped before rendering.
- **`Query` extractor.** Axum 0.7's `Query<T>` requires `T: Deserialize`; `serde`
  with `derive` is already a dependency (`Cargo.toml`). A malformed query string
  deserialises `q` to `None` — no 400, no panic.
- **Feature flags / rollout:** N/A — single binary, single deploy. The rollout is
  the commit sequence in §7.2.

### 4.7 Performance budget

| Dimension | Estimate | Note |
|---|---|---|
| Per-query work | 4 posts + 12 topic pages read + parsed, then a linear scan of ~21,000 words | Sub-millisecond-to-low-ms; the whole corpus is smaller than a single large page render elsewhere. Bounded by the corpus, not by the query. |
| Query length | Hard cap **128 chars** before tokenising (§4.2 step 1) | Bounds tokenisation and match work regardless of input; the input carries `maxlength="128"` too, but the server cap is the real guard (a client attribute is not a control). |
| Result set | Cap **20** (`MAX_RESULTS`) | Larger than today's whole corpus; a defined ceiling on rendered HTML. |
| CSS added | ~0.6–0.9 KB for `.search-*` + `mark` rules | One file, one request (A1's single `style.css`). |
| JS added | **0 bytes** | The feature is native HTML end to end — the point of it (§1.2). |
| Network requests added | 0 | No images, no fonts, no scripts. |
| Memory | The corpus is built, used for one response, and dropped (per-request build) | No persistent index in `AppState` under the chosen design. |
| Client storage | 0 | Search stores nothing client-side. |

**Caching interaction (handed to A3).** `/search?q=` responses must **not** be
cached at the edge (Caddy/Cloudflare): they are per-query and embed the live
vitals strip (A2 §4.7). The response should carry the same HTML no-cache posture
A2 flagged for A3; C1 asserts the requirement and defers the header mechanism to
A3. A cached search page would serve one visitor's query to the next — a
correctness and mild privacy defect.

---

## 5. Test Specification

All Rust tests run under `cargo test --all-targets` and gate CI
(`.github/workflows`: `fmt → clippy → test → build --release`, criterion 5D).

### 5.1 Unit / integration tests

New (this spec). Unit tests live in `src/search.rs::tests`; router-level tests use
`tower::ServiceExt::oneshot` (the pattern at `errors.rs:171-182`,
`status.rs:84-89`). File-corpus guards live in `tests/search.rs`.

| # | Name | Setup | Assertion | Edge case / lens |
|---|---|---|---|---|
| **T-C1-1** | `all_terms_must_match_and` | Build index; query `"corosync outage"` then `"corosync zzz"` | The two-real-term query returns the migration post; the query with one impossible term returns empty | AND semantics; a term that matches nothing zeroes the result |
| **T-C1-2** | `title_hit_outranks_body_hit` | Two docs, one with the term in title, one only in body | The title-hit doc sorts first | Field weighting (§4.2) |
| **T-C1-3** | `ranking_breaks_ties_by_date_then_slug` | Two docs, equal score, different dates | Newer first; equal dates → slug ascending | Deterministic ordering (no flaky test) |
| **T-C1-4** | `blank_and_punctuation_queries_return_empty` | Query `""`, `"   "`, `"!!!"` | Empty result set; no panic; handler renders the empty state | E-01 / degenerate input |
| **T-C1-5** | `query_and_content_are_escaped_in_output` | `oneshot GET /search?q=<script>alert(1)</script>` (URL-encoded) | Body contains `&lt;script&gt;`, contains **no** live `<script`, and the input `value=` is escaped; any `<mark>` wraps escaped text only | **XSS — reflected query is the sharpest risk (§6.1); mirrors `errors.rs:160-169`** |
| **T-C1-6** | `search_needs_no_javascript` | `oneshot GET /search?q=osi`, strip every `<script>…</script>` from the body | The `<form method="get"`, `role="search"`, every result `href`, and the snippet text all survive | **No-JS floor (auto-fail rule 3)** |
| **T-C1-7** | `every_result_url_resolves_200` | Query terms hitting each corpus kind; for every `result.url`, `oneshot GET` it | Each returns 200 (posts via `/blog/:slug`, pages via `/learn/:slug`) | **A search hit can never 404 (Lens 5B)** |
| **T-C1-8** | `drafts_are_never_indexed` | Place a sentinel term in a `content/drafts/*.md`; build the index; query it | Zero results; assert no `SearchDoc.slug` corresponds to any draft stem | **Lens 1C publication gate — unpublished/gated work must not leak** |
| **T-C1-9** | `page_corpus_equals_sidebar_allowlist` | Compare `SearchIndex` page slugs to `wiki::sidebar_slugs()` minus `"index"` | Equal sets | The corpus tracks the allowlist, not raw disk (a non-sidebar `.md` is not searchable) |
| **T-C1-10** | `snippet_highlights_the_match` | Query `"encapsulation"` against the OSI page | The snippet contains `<mark>encapsulation</mark>` (case-normalised) and surrounding context | Snippet correctness (§4.3) |
| **T-C1-11** | `long_query_is_truncated_not_rejected` | Query of 500 chars whose first 128 contain a real term | Returns the expected hit; no error | E-03 resource bound |
| **T-C1-12** | `search_page_declares_one_h1_and_site_title` | Render `SearchTemplate` idle and with a query | Exactly one `<h1>`; `<title>` ends `" — machinageist"` | A2 U-4 / U-5 |
| **T-C1-13** | `search_ui_copy_carries_no_retired_claims` | Render the template | Static UI copy contains none of A2 U-7's retired terms (`"Network+"`, `"the CompTIA stack"`, `"offensive security"`, `"red-team"`, `"pentest"`, `"production-grade"`, `"enterprise"`, `"SRE"`) | **Lens 1D/1E — search copy is user-visible copy** |

### 5.2 Integration tests

Covered by T-C1-5/6/7/8 (router `oneshot`) plus an addition to A2's
`every_route_renders_the_full_shell` (A2 I-1) so `/search` and `/search?q=osi`
render the full shell (header, nav, `<main>`, footer, vitals) — the shell
contract must include the new route.

### 5.3 UI / E2E tests

**Absent, and deliberately so.** There is no browser-automation harness in the
repo (no Playwright/Selenium, no `package.json`) and this feature has **zero
JavaScript**, so the behaviours E2E would cover (submit, navigate) are native
`<form>`/`<a>` semantics fully verified by the served-bytes tests above (T-C1-6)
and the manual pass in §5.4. Adding a headless browser to test a native GET form
would cost far more than it buys (mirrors A1 §5.3, A2 §5.3, B5 §5.3).

### 5.4 Visual / manual verification

Per A1's tiered matrix (§5.4). Search-specific surfaces on the Tier-1 six themes
(Lunarcore, Solarcore, Paper, Cloud, Solarized, CRT):

- `/search` idle (form + helper + onward links).
- `/search?q=osi` (multiple results, snippets, `<mark>` visible and legible in
  every theme — the A1 cross-feature request's acceptance surface).
- `/search?q=kubernetes` (no-results empty state, not broken-looking).
- 200% zoom / 24px browser font: form wraps, results reflow, no horizontal scroll
  at 320px.
- `prefers-reduced-motion: reduce`: nothing animates (there is nothing to).
- `prefers-color-scheme: light` with **JavaScript disabled**: light palette, the
  form submits and returns results, no dead theme control.
- A query with `<script>` typed literally: the page shows the escaped text, runs
  no script.

### 5.5 Documentation follows behavior (criterion 5E)

`docs/design/SEARCH.md` (new) is updated in the same change as any behavioural
change and records: the corpus definition (routable content only; drafts
excluded), the ranking/weighting contract, the snippet escape-then-mark
invariant, the per-request-build (no-cache) decision and when to revisit it, and
the no-JS decision. The repo `README.md` route list gains the `/search` entry.

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement** for stored/transmitted data — search sets
  no cookies, stores nothing server- or client-side, and transmits no PII. The
  query string is processed and reflected, never persisted.

**But the reflected query is the feature's real risk and is treated as such.** A
search box that echoes user input on a strict-CSP site is the classic reflected-
XSS surface. Protections: (1) the query is echoed only via Askama auto-escaping
(`{{ q }}`) in the input value, count line, and no-results copy; (2) snippets use
the escape-then-mark pipeline (§4.3) and are the only `|safe` in the feature; (3)
the site-wide CSP `script-src 'self'` (`security_headers.rs:42`) is defence in
depth even if an escape were missed; (4) T-C1-5 pins the escaping. The query is
also logged? **No** — `q` is not written to logs to avoid a log-injection/PII
footgun; only aggregate request logging (A3's `TraceLayer`) applies.

### 6.2 Asset provenance

- [x] **No third-party assets.** No models, images, fonts, icons, or datasets.
  Matching is `std` string work; plaintext extraction reuses the already-vendored
  `pulldown_cmark` (MIT/Apache-2.0). The indexed content is the site's own
  author-written flat files.

### 6.3 Language / claims audit

- [ ] Claims not supported by evidence — **no.** Search *surfaces* existing
  content; it asserts nothing new. The only new user-visible copy is the page
  `<h1>`, the input placeholder ("Search posts and learn pages"), the count line,
  and the empty-state helper — all functional, none making a capability or
  identity claim.
- [ ] Capabilities not yet built read as shipped — **no.** The feature is being
  built; §7 states it is currently **absent** and this spec is the plan. No copy
  implies search exists until it ships.
- [ ] Domain-restricted language — **no.** The UI copy carries no cert claim, no
  offensive-security/red-team/pentest/SRE/enterprise/production-grade language;
  T-C1-13 enforces it. Because search indexes the writing and learn corpora, it
  inherits their claim-cleanliness — and, critically, it **cannot index drafts or
  gated work** (§6.4), so it cannot surface an unearned claim from unpublished
  material.

### 6.4 Regulatory alignment (criteria.md)

- **Auto-fail rule 1 (unearned claims) / Lens 1C (publication gate):** the corpus
  is routable content only; `content/drafts/**` and any GeistScope-gated material
  are never read (§4.2, T-C1-8). Work-in-progress cannot enter search results, so
  search cannot promote non-portfolio work to portfolio-adjacent visibility.
- **Auto-fail rule 2 (accessibility floor):** AA contrast for input/button/`mark`
  across all 23 themes (A1 cross-feature, §7.4); `<mark>` uses a colour-
  independent cue (weight + underline), not hue alone; focus states are A1's
  global ring; no motion, so `prefers-reduced-motion` is satisfied trivially
  (§3.5, §3.7).
- **Auto-fail rule 3 (no-JS floor):** the core function is a server-rendered GET
  form with server-rendered results; zero JS is added (§3.7A, T-C1-6). *Met and
  central.*
- **Lens 1D/1E (copy currency/role posture):** UI copy is claim-clean and
  guarded (T-C1-13).
- **Lens 5A/5B (single source / drift guards):** the corpus is derived from
  `POSTS_DIR` + `sidebar_slugs()` (not a hand-maintained list), and T-C1-7/8/9
  fail loudly on any drift between what is searchable and what is servable.

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**Search is absent** — the honest, unambiguous state. Verified: no `/search`
route (`router.rs:36-61` registers every route the site answers, none for search), no search
handler (`handlers/mod.rs:3-8`), no search model (`models/mod.rs:1-3`), no
`search.html` template, no search index, and no search-related CSS
(`style.css` has an `input, select, textarea` transition rule at `:739` but **no
form components render** — nothing uses it yet).

What *exists and is reusable*, so the build is small:

| Reusable asset | State | Evidence |
|---|---|---|
| Post corpus + servable route | implemented | `content/posts/*.md`; `BlogPost::load_all` (`post.rs:117-137`); `/blog/:slug` (`router.rs:47`, `blog.rs:147-158`) |
| Page corpus + allowlist route | implemented | `content/pages/*.md`; `Page::find` (`page.rs:72-78`); `/learn/:slug` gated by `SIDEBAR` (`wiki.rs:35-99`, `157-166`) |
| Frontmatter (title/summary/tags/date/category) | implemented | `post.rs:36-47`, `page.rs:19-25` |
| Markdown parser (for plaintext extraction) | implemented | `pulldown_cmark` `Options::all()` (`post.rs:97`, `page.rs:58`) |
| Output escaping | implemented | Askama auto-escapes `{{ }}`; the pattern proven at `errors.rs:160-169` |
| Global rate limiter fronting every route | implemented | `router.rs:72-75` |
| Themed shell, empty-state pattern, focus ring | implemented | `base.html`, A1 §3.3, `style.css:685` |
| `sidebar_slugs()` for the page corpus | **planned (B5 dep)** | B5 §5.1 T-B5-4 proposes `pub fn sidebar_slugs()` in `wiki.rs`; C1 consumes it (interim fallback in §7.4) |

### 7.2 Delta to spec

**New files:**
- `src/search.rs` — engine (types, corpus build, ranking, snippet/escape).
- `src/handlers/search.rs` — `SearchQuery`, `SearchTemplate`, handler.
- `templates/search.html` — form + count + results/empty-state.
- `tests/search.rs` — corpus/URL/drafts/XSS/no-JS guards.
- `docs/design/SEARCH.md` — the long-lived contract (§5.5).

**Modified files:**
- `src/main.rs` — add `mod search;` (module list `main.rs:24-30`).
- `src/router.rs` — register `.route("/search", get(search::search))` (route block
  `router.rs:36-61`).
- `src/handlers/mod.rs` — add `pub mod search;` (`mod.rs:3-8`).
- `src/models/post.rs` — add `content_text` field + population; update the two
  test-fixture constructors (`blog.rs:166-176`, `pages.rs:134-144`) to include it.
- `src/models/page.rs` — add `content_text` field + population.
- `src/handlers/wiki.rs` — add/consume `pub fn sidebar_slugs()` (B5 dep) and make
  `PAGES_DIR` reachable to the engine (it is currently private, `wiki.rs:19` — make
  it `pub(crate)` like `POSTS_DIR`, or expose an accessor).
- `static/css/style.css` — new `.search-form`, `.search-results`,
  `.search-result-*`, and `mark` rules (values from A1, §7.4).
- `README.md` — add `/search` to the route list.
- (A2) `base.html` — optional header search affordance + `Section::Search`.

**No migrations, no new dependencies, no new runtime assets.**

### 7.3 Estimated scope

**M.** No new content is authored; the corpus already exists. The work is one new
engine module (ranking + the security-sensitive snippet pipeline is the careful
part), one thin handler, one template, a CSS block, and a focused test suite. The
two model-field additions are trivial but touch test fixtures. The single largest
risk item is the escape-then-mark snippet function — small in lines, high in
consequence — which is why it gets its own test (T-C1-5) and its own doc note. Not
S, because of the security surface and the cross-feature coordination (A1 `mark`
styling, B5 `sidebar_slugs()`, optional A2 header affordance); not L, because
nothing here is architecturally novel and no dependency is added.

### 7.4 Blocking dependencies

- **B5 learn — `pub fn sidebar_slugs()`** (B5 §5.1 T-B5-4). C1's page corpus and
  T-C1-9 consume it. *Interim if B5 has not landed:* C1 can call the existing
  private `lookup_sidebar_slug` logic by adding a small `pub(crate) fn
  sidebar_slugs()` in the same commit — it is a pure read over the existing
  `SIDEBAR` const (`wiki.rs:35-99`), so C1 is not hard-blocked, only cleaner after
  B5. `PAGES_DIR` (`wiki.rs:19`) must become `pub(crate)`.
- **A1 design-system — `mark`, input, and button styling** across all 23 themes,
  AA-audited, with a colour-independent highlight cue (§3.7B, §6.4). C1 supplies
  the selectors and the requirement; A1 owns the token values. *Cross-feature
  request.* Until A1 lands the audited values, C1 ships with A1's existing tokens
  and the manual matrix (§5.4) as the check.
- **A2 site-shell — `Section::Search`** and (optional) a header/404 search
  affordance (§3.1). *Interim:* `section()` returns the string `"search"`, which
  matches no current nav link (so nothing mis-highlights). Not hard-blocking.
- **A3 ops — HTML no-cache policy** covering `/search` (§4.7). Search inherits the
  site-wide requirement A2 §4.7 already handed to A3; no C1-specific work beyond
  asserting it.
- No dependency on C2 (glossary) or C3 (study-tools). C2 will *extend* the corpus
  later; the `DocKind` enum (§4.2) is shaped so a `DocKind::Glossary` slots in
  without redesign.

---

## 8. Open Questions

- **Q1 (search box placement):** Should the search affordance live in the header
  (always visible, A2-owned, +markup on every page) or only as a "Search" nav link
  plus the `/search` page? Recommendation: a lightweight "Search" nav link (or a
  compact GET form) in the header, filed to A2 — the `/search` page is the
  complete feature and the header entry is discoverability. — blocks: §3.1, A2
  cross-feature scope.
- **Q2 (match strictness):** AND across terms (a doc must contain *every* term) is
  specified. For a small corpus this can produce empty results on a two-word query
  where each word exists in different docs. Confirm AND (precise, predictable) vs.
  OR-with-ranking (more forgiving, noisier). Recommendation: AND now; revisit if
  real usage shows too many empty results. — blocks: §4.2 ranking.
- **Q3 (per-request build vs. startup index):** §4.4 chooses per-request build for
  freshness and simplicity, with a startup `Arc<SearchIndex>` recorded as the
  scale answer. At what corpus size (page count / total words) should the startup
  index be adopted, and is losing instant-publish freshness acceptable then?
  Recommendation: revisit past ~150 pages. — blocks: nothing now; a future
  performance decision.
- **Q4 (stemming/fuzzy matching):** Case-insensitive substring is specified.
  Should "subnet" match "subnetting", or a typo "encapsulaton" match
  "encapsulation"? Both need either a dependency or hand-rolled logic and add test
  surface. Recommendation: defer; ship substring, measure whether misses are real.
  — blocks: §4.2, §4.5 (would reopen the no-dependency stance).
- **Q5 (indexing scope of body vs. metadata only):** Body-text matching is
  specified (it is what makes "corosync"/"encapsulation" findable). If the
  `content_text` field addition to the models is judged too invasive, the fallback
  is title+summary+tags-only search — weaker but zero model changes. Confirm body
  search is wanted (recommended: yes; it is the difference between a real search
  and a menu). — blocks: §4.2 model additions.
- **Q6 (search analytics):** Should popular/zero-result queries be counted (to
  guide what to write next)? Any counting is a privacy and claim-integrity
  surface and would need A3 review; §6.1 currently specifies **not** logging `q`.
  Recommendation: no analytics for now; if wanted later, aggregate counts only,
  never raw queries. — blocks: nothing; scope decision.
