# Spec: Writing

**Feature ID:** B4-writing
**Parent feature:** root
**Spec author agent:** spec-agent-B4 (Claude Opus 4.8)
**Date:** 2026-08-08
**Iteration:** 1

---

## 0. Reading notes and scope boundary

**What B4 owns.** The two writing surfaces and the model behind them:

- The writing index at `/blog` — a pillar-grouped list of posts (`src/handlers/blog.rs:76-120`, `templates/blog_list.html`).
- The single-post reader at `/blog/:slug` (`src/handlers/blog.rs:147-158`, `templates/blog_post.html`).
- `BlogPost`, its frontmatter schema, and the load/parse/render pipeline (`src/models/post.rs`).
- The Markdown source of truth in `content/posts/*.md` (four published posts today).

**What B4 explicitly does not own** (cited so this spec never contradicts a foundation or sibling spec):

| Concern | Owner | Interface B4 relies on |
|---|---|---|
| Design tokens, type scale, `--measure`, group-heading styling policy | `A1` design-system | `--measure` 72ch / `--measure-narrow` 55ch (`A1` §; `style.css:499`), article-heading size restoration `--text-xl` h2 > `--text-lg` h3 (`A1` §7.1; `style.css:1090-1103`), pillar group headings "currently unstyled — §7.1.6" (`A1` screen inventory row 139) |
| `base.html` shell, nav active state, `<head>` meta set, `og:type`, `og:image` | `A2` site-shell | `section()` → `Section::Writing` (`A2:474,494`); `aria-current="page"` on active nav (`A2:154-156,544`); `og:type=article` for `/blog/:slug` (`A2` test I-8 `:748`); `og:image` planned (`A2:656`) |
| Security headers / CSP, rate limiting, vitals strip, `/status` | `A3` ops | CSP `default-src 'self'; script-src 'self'; …` (`README.md:60`, `security_headers.rs`); vitals strip appears on `/blog` (`status.rs:113-123`) |
| Home "Latest writing" teaser that reads `BlogPost` | `B1` home | B1 reads, never writes, `BlogPost::load_all` (`B1` §; `pages.rs:16,55-63`). `POSTS_DIR` is shared via `pub(crate)` (`blog.rs:30`). |
| `/learn` article pages that share `.post-content` prose styling | `B5` learn | Shared prose CSS; a change to `.post-content` ripples to `/learn` — see §5. |

This spec describes the **target** state of the writing feature, then §7 itemizes the **gap** from what ships today. Every capability is tagged **implemented / prototyped / planned / gated / absent**.

---

## 1. Purpose

### 1.1 One-sentence job
Let a reader find and read machinageist's technical writeups — grouped by portfolio pillar, each post a self-contained account of real work with commands, evidence, what broke, and how it was verified.

### 1.2 Why it matters
The site's core asset is that everything on it is true and defensible in an interview (criteria Lens 1). Portfolio entries and `/learn` pages state *that* work happened; the writing surface is where the work is *shown in full* — the network-migration post is an eight-hour outage worked end to end, with the recovery sequence, the quorum/`/etc/pve` circular dependency, and the DNS failure separated out (`content/posts/management-layer-first-network-migration.md`). Criteria 4B names that post the in-repo model for "evidence over enthusiasm," and 4C names original explanation the differentiator against both competitor groups (homelab portfolios that show only green screenshots, and cert-track candidates whose course completion substitutes for operated systems). Writing is the surface that carries that differentiation.

### 1.3 Success signal
A reviewer arriving cold at a deep link (`/blog/management-layer-first-network-migration`) can, without JavaScript, read the whole post, see the pillar it belongs to, and get back to the full index — and a peer engineer subscribing to a feed is notified when the next post lands. Observable: `curl -s https://machinageist.dev/blog/<slug>` returns the full article HTML at 200; `/blog` returns every published post grouped under its pillar; the planned `/feed.xml` validates and lists the newest posts.

---

## 2. User Stories

> As a **hiring manager** skimming from a resume link, I want to land on a single post and immediately see it is real operations work (real commands, a real outage, honest limits), so that I can judge the candidate's judgment in under a minute.

> As a **peer engineer**, I want to browse `/blog` grouped by pillar and follow a feed, so that I can assess writing quality across topics and be notified of new posts without polling the site.

> As a **self-directed learner**, I want a post that starts from the intended change and walks through what broke and how it was verified, so that I can learn the debugging method, not just the fix.

> As a **reader on a keyboard / screen reader**, I want the index and article to have a real heading outline, visible focus, and underlined in-prose links, so that I can navigate and distinguish links without relying on color or a mouse.

> As a **reader with JavaScript disabled**, I want the index, every article, and the feed to work as plain server-rendered HTML with real URLs, so that the writing is never gated behind a script.

> As the **author (admin persona)**, I want to publish by dropping a `.md` file with valid frontmatter into `content/posts/`, and I want a malformed date, missing frontmatter, or mistyped pillar to fail loudly rather than silently mis-file or 500 a page, so that a bad publish is caught at load time, not by a visitor.

> As the **author**, I want work-in-progress (e.g. the GeistScope retrospective in `content/drafts/`) to stay out of the published index until its publication gate clears, so that a draft can never read as a shipped claim (criteria Lens 1C).

---

## 3. UX Specification

### 3.1 Screen / view inventory

| Screen | Path | New/mod | Layout pattern |
|---|---|---|---|
| Writing index | `/blog` | **modification** | Single column inside the shell `<main id="content">`; `h1` + intro + one `<section>` per non-empty pillar, each a divider `<ul>` (`templates/blog_list.html`) |
| Article (single post) | `/blog/:slug` | **modification** | Single reading column; back-link, post header (title + meta), then prose capped at `--measure` with full-width scrolling `pre`/tables (`templates/blog_post.html`; `A1` inventory row 140) |
| Writing feed | `/feed.xml` | **new (planned)** | Non-HTML: server-rendered Atom/RSS XML, no shell |
| 404 for unknown slug | any `/blog/<missing>` | reuses `A2`/`errors` | Themed boot-sequence 404 (`errors.rs:129-135`, `render_404`) |
| Empty index state | `/blog` with zero posts | **new (planned)** | Same column; a single designed empty-state line instead of blank space |

No modal, sheet, popover, or drawer. The feature is entirely page-based — consistent with the site's no-JS identity.

### 3.2 Interaction flows

**Primary — read a post from the index.**
1. Reader opens `/blog`. `list()` calls `BlogPost::load_all(content/posts)` (`blog.rs:79`), which reads every `.md`, parses frontmatter, sorts newest-first (`post.rs:135`).
2. `group_by_pillar` buckets posts into the four pillars in display order, then a trailing "Other writing" group; empty pillars are dropped (`blog.rs:87-120`).
3. Template renders one `<section>` per group, each a `<ul class="post-list">` of items showing date, linked title, and summary (`blog_list.html:11-24`).
4. Reader clicks a title → `GET /blog/:slug`.
5. `post()` validates the slug against traversal (`blog.rs:150`), `BlogPost::find` builds `content/posts/<slug>.md` and parses it (`post.rs:144-151`), template renders the header and the pre-converted `content_html` via `|safe` (`blog_post.html:29`).
6. Reader clicks "Back to writing" (`blog_post.html:4`) → back to `/blog`.

**Primary — subscribe to the feed (planned).**
1. Reader's feed client requests `/feed.xml` (advertised via `<link rel="alternate" type="application/atom+xml">` in `<head>` — see §4.3 / §7.2).
2. Handler reuses `BlogPost::load_all`, emits newest-N entries as Atom XML with absolute URLs and each post's `summary`.
3. Client polls on its own schedule; new post file → next poll shows it. No push, no JS.

**Branch — unknown or malformed slug.** `post()` returns `SiteError::PostNotFound` → HTTP 404 boot-sequence page echoing the requested path (`errors.rs:108-111,129`). No data loss.

**Branch — a post file is malformed** (bad `date:`, missing frontmatter, missing required key). Caught at load time: `from_file` returns a typed error (`post.rs:88-94`) → `/blog` returns 500 (generic, leaks nothing — `errors.rs:113-123`). This is deliberate: a broken publish fails the whole index loudly rather than silently dropping one post. (Contrast with the home teaser, which degrades to empty — that is `B1`'s decision, `pages.rs:56`.)

No haptics, no sound. Animation is limited to the `.post-item` hover transition, which sits behind `prefers-reduced-motion` (`style.css:738-745`).

### 3.3 Layout descriptions

**Writing index (`/blog`) — top → bottom:**
- `<h1>Writing</h1>` (`blog_list.html:5`) — the only `h1`; `--text-2xl`, capped at `--measure` (`style.css:773-780`).
- Intro `<p class="section-intro">` (`blog_list.html:6-9`): "Notes on the homelab, networking, Linux, and a bit of defensive security — mostly what broke and how I worked through it." **Target:** intro copy uses `--measure-narrow` (55ch) per `A1`; today `.section-intro` is `65ch` (`style.css:1202`) — reconcile in the `A1` sweep, not per-page.
- For each non-empty group, in `PILLARS` order (`blog.rs:35-40`) then "Other writing":
  - `<section class="post-group">` with `<h2 class="post-group-heading">{label}</h2>`.
  - **Data source:** `group.label` and `group.posts` from `PostGroup` (`blog.rs:50-53`).
  - Group heading styling: `.post-group`/`.post-group-heading` carry **no CSS rule of their own**; they inherit the base `h2` — small-caps uppercase accent label (`style.css:784-791`), which is the intended "uppercase section labels used on listing pages" (`style.css:1088-1089`). `A1` §7.1.6 owns whether to formalize a rule; B4 does not introduce one.
  - `<ul class="post-list">` (`style.css:988-994`), each `<li class="post-item">` (`style.css:996-1001`): `.post-date` (block, faint — `style.css:1011-1016`), linked title `.post-item a` (`style.css:1003-1009`), `.post-summary` (faint — `style.css:1018-1022`).

**Empty state (planned).** When `groups` is empty (readable but zero posts), render one line — "No posts published yet." — inside `--measure-narrow`, in place of the group loop. Today the loop simply produces nothing, leaving `h1` + intro over blank space (`A1` §7 lists "a blog list with zero posts" as a state to check).

**Article (`/blog/:slug`) — top → bottom:**
- `<a class="back-link" href="/blog">Back to writing</a>` — block link, `←` prefix via CSS `::before` (`blog_post.html:4`; `style.css:1028-1037`).
- `<header class="post-header">`: `h1` = `post.title`; `.post-meta` row = `.post-date` + `.post-tags` (`blog_post.html:5-15`; `style.css:1056-1086`). Tags are non-interactive `<span class="tag">` pills bordered on `--surface` (`style.css:1080-1086`) — deliberately not links today (no tag pages exist; see §7).
- `<div class="post-content">`: `post.content_html` rendered `|safe` (`blog_post.html:29`). Prose (`p`, `ul`, `ol`, `blockquote`) capped at `--measure`; `pre` and `table` uncapped, taking the full column and scrolling inside `overflow-x:auto` (`style.css:1105-1192`). Article headings restore true size order (`--text-xl` h2 > `--text-lg` h3), reversing the listing-page small-caps convention (`style.css:1090-1103`).
- **Data source:** the whole view is one `BlogPost` (`blog_post.html` → `BlogPostTemplate.post`).

### 3.4 Input & gestures
- Pointer/touch: tap a title or the back-link. `.post-item` and its link are the click targets; full title line is clickable.
- Keyboard: every interactive element is a native `<a>` — Tab to reach, Enter to activate; focus ring is the global `:focus-visible` 2px accent outline (`style.css:710`). No custom key handling, no shortcuts (none are warranted here; the theme menu's roving-focus model is the site's only widget pattern and lives in `A2`).
- Specialized input: N/A — no stylus/controller/voice/camera surface.
- Responsive: single fluid column; `pre`/tables scroll rather than force page-level horizontal scroll (`style.css:1124-1130,1167-1176`), so the page body never scrolls sideways from a wide code block on mobile.

### 3.5 Transitions & animation
- Only the `.post-item` hover transition (`background-color`/`box-shadow`, 0.2s — `style.css:730,745`). Wrapped by the global `@media (prefers-reduced-motion: reduce)` block that disables it (`style.css:738-744`).
- Navigation is full-page loads; no SPA transitions, no view-transition API, no scroll-triggered motion in body content (criteria 2E, 3E).

### 3.6 Error states

| Trigger | Presentation | Justify | Recovery | Data loss |
|---|---|---|---|---|
| Requested slug has no file | Full-page themed 404 (boot-sequence), echoes the path escaped (`errors.rs:129-135`, `Error404Template`) | A missing article is a navigation dead-end, not an inline field error; a full page with the shell keeps the reader oriented | Nav / back to `/blog` | No |
| Slug contains `/`, `\`, or `..` | Same 404 via early `PostNotFound` before any filesystem touch (`blog.rs:150`) | Traversal probes must never reach disk; identical 404 avoids leaking that the guard exists | Nav | No |
| A post file malformed (bad date / missing frontmatter / missing key) | `/blog` returns generic 500; internals logged, not shown (`errors.rs:113-123`, `post.rs:88-94`) | A broken publish should fail loudly at the author, not silently drop a post or expose a parse error | Author fixes the `.md`, redeploys | No (source file intact) |
| `content/posts/` unreadable | `/blog` 500 (I/O error propagates via `?`, `blog.rs:79`) | Same reasoning; the index is the honest failure surface (the home teaser degrades instead — `B1` owns that) | Restore directory | No |
| Zero posts (readable, empty) | Designed empty-state line (planned; §3.3) | An empty index with a dangling `h1` reads as broken; a sentence reads as intentional | Publish a post | No |
| Feed requested, load fails (planned) | 500 with `Content-Type: text/plain`, no partial XML | A truncated feed can poison clients; fail whole | Retry after fix | No |

### 3.7 Accessibility
- **Heading outline:** `/blog` = one `h1` ("Writing") → `h2` per pillar; post titles are links inside `<li>`, not headings (correct — they are navigation, not sections). Article = `h1` (title) → `h2`/`h3` from Markdown. Outline is real and color-independent (weight + case + size carry hierarchy, `style.css:773-798,1090-1103`) (criteria 3D).
- **Landmark caution (refinement, §7.2):** each pillar `<section>` carries `aria-label="{group.label}"` (`blog_list.html:12`), which promotes every group to a labeled region landmark and duplicates the visible `h2`. Target: reference the heading with `aria-labelledby` (give the `h2` an `id`) or drop the region role, so a screen reader's landmark list is not flooded with one region per pillar. Headings already provide the structure.
- **Links & color independence:** in-article links keep the browser-default underline (global `a` sets color + underline-offset but never `text-decoration:none` — `style.css:534-537`; `.post-content a` overrides only color — `style.css:1112`), so prose links are not distinguished by hue alone (WCAG 1.4.1). Index title links use `text-decoration:none` (`style.css:1005`) but are the dominant line of each item and the full click target; acceptable, though it leans on position rather than an affordance — noted for review.
- **Focus:** all links reachable and operable by keyboard; visible 2px `--accent` focus ring, 2px offset (`style.css:710`). Focus order follows DOM: back-link → (article) or group order (index). The `A2` focus-ring 3:1 contrast audit (`A2` §3B) covers this ring across all 23 themes; B4 defers to it.
- **Contrast:** all text/background pairs are theme tokens audited by `A1`'s `generate_themes.py --check` (criteria 3B); `.post-date`/`.post-summary` use `--text-faint`, which must clear AA at their usage sizes (0.78rem / 0.85rem) in every theme — this is inside `A1`'s 14-failure remediation scope, not B4's to re-derive.
- **Text scaling:** all sizes are `rem`/token-based (`style.css:481-487`), so browser zoom and OS text scaling reflow; nothing is pinned in `px` except the 15px body root (`style.css:520`).
- **Motion:** see §3.5 — only hover motion, behind `prefers-reduced-motion` (criteria 3E).
- **No-JS:** index, article, and feed are fully server-rendered; no script participates in reading (criteria 3A / auto-fail rule 3). The page's only scripts are the shell's theme init/menu (`base.html:12,99`), which are enhancement-only and irrelevant to reading.

---

## 4. Implementation Specification

### 4.1 Architecture placement
- Handlers: `src/handlers/blog.rs` — `list()` (`:76-83`), `post()` (`:147-158`), `group_by_pillar()` (`:87-120`), the `PILLARS`/`OTHER_GROUP` constants (`:35-43`), `POSTS_DIR` (`:30`, `pub(crate)` so `B1` home shares it).
- Model: `src/models/post.rs` — `BlogPost` (`:53-62`), `Frontmatter` (`:36-47`), `from_file`/`load_all`/`find` (`:70-151`). Declared in `src/models/mod.rs:2`.
- Templates: `templates/blog_list.html`, `templates/blog_post.html`, both `{% extends "base.html" %}`.
- Routes: `src/router.rs:40` (`/blog` → `blog::list`) and `:47` (`/blog/:slug` → `blog::post`). `:slug` matches exactly one path segment (`router.rs:12`).
- Content: `content/posts/*.md`. **Planned:** a feed handler (`blog::feed` or `src/handlers/feed.rs`) + route `/feed.xml`.

### 4.2 Data model

Shipped model (unchanged for target unless noted):

```rust
// src/models/post.rs:36-47 — frontmatter schema; serde field names must match YAML keys
#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    date: String,            // parsed to NaiveDate at load time (post.rs:93)
    summary: String,
    tags: Vec<String>,
    #[serde(default)]
    category: Option<String>, // optional pillar label; missing => None (post.rs:45-46)
}

// src/models/post.rs:53-62 — public model passed to templates
#[derive(Debug, Clone)]
pub struct BlogPost {
    pub slug: String,          // filename stem; URL == filename, intentionally
    pub title: String,
    pub date: NaiveDate,       // parsed, so malformed dates fail at load
    pub summary: String,       // list view + <meta description> + feed entry
    pub tags: Vec<String>,
    pub category: Option<String>, // matched against PILLARS to group the list
    pub content_html: String,  // pulldown-cmark output; empty in list view intent
}
```

**Frontmatter contract (authoring convention, planned as documented — §7.2).** Required keys: `title`, `date` (`YYYY-MM-DD`), `summary`, `tags`. Optional: `category` (must be one of `PILLARS` to group; anything else falls to "Other writing"). Two authoring rules the code cannot yet enforce but the target adds guards for:
- `summary` ≤ **160 characters** so it is safe as a `<meta name="description">` (see §5, and `A2` U-6 `:722`). Today the longest shipped summary (`management-layer-first-network-migration.md:4`, ~220 chars) exceeds this.
- `category` typos silently mis-file into "Other writing" (`blog.rs:107-109`) — the target adds a load-time guard (§4.4 / §5).

**No database, no migration.** Content is flat files read at request time (`README.md:93-99`). N/A for schema migrations.

### 4.3 API contracts

| Method / path | Handler | Returns | Errors | Auth |
|---|---|---|---|---|
| `GET /blog` | `blog::list` (`blog.rs:76`) | 200 HTML (grouped index) | 500 on I/O or parse failure | none (public) |
| `GET /blog/:slug` | `blog::post` (`blog.rs:147`) | 200 HTML (article) | 404 `PostNotFound` (missing / traversal); 500 on parse failure | none |
| `GET /feed.xml` **(planned)** | `blog::feed` | 200 `application/atom+xml; charset=utf-8`, newest-N entries | 500 on load failure | none |

- Params: `:slug` is one path segment (`router.rs:12`); validated against `/`, `\`, `..` before disk access (`blog.rs:150`). No query params today. Feed takes none (or an optional `?n=` cap — decide in §8).
- Pagination / rate limiting: no pagination (four posts; revisit past ~50 — §7.3). Rate limiting is global middleware (`A3`, `router.rs:72-75`), not per-route here.
- **Head/meta contract with `A2`:** `BlogPostTemplate` supplies `title()` = `post.title`, `description()` = `post.summary`, `section()` = "writing" (`blog.rs:132-143`). Target: it also signals `og:type=article` so `A2`'s `base.html` emits it (`A2` I-8 `:748`) — today `base.html:9` hardcodes `og:type=website` and B4 supplies no override (see §7.2). `BlogListTemplate` supplies `title()`/`description()`/`section()` (`blog.rs:62-73`). `<head>` also gains `<link rel="alternate" type="application/atom+xml" href="/feed.xml">` when the feed lands (mechanism owned by `A2`'s `head_extra`).

### 4.4 State management
- No client state, no store, no view model. State is the filesystem; each request re-reads and re-parses (`post.rs:117-137`). Ownership: the handler owns the per-request `Vec<BlogPost>`; nothing is cached between requests today.
- Local vs. server: entirely server-side. There is no offline or draft persistence in the app — "drafts" are files in `content/drafts/` outside the routed directory (`README.md:57-62`, feature-tree "out of scope").
- **Planned guard (single source of truth for pillar names).** A load-time check (or a test) asserting every post's `category` is `None` or a member of `PILLARS`, so a typo fails loudly instead of silently landing in "Other writing" (§5B). `PILLARS` (`blog.rs:35-40`) stays the one definition; the guard closes the soft coupling between it and the free-text `category:` strings in each `.md`.
- **Planned optimization (metadata-only load).** `list()` and the feed only need `{slug,title,date,summary,category,tags}`, yet `from_file` always runs `pulldown-cmark` over the full body and builds `content_html` that the list discards (`post.rs:96-99`). A `load_all_meta()` (or a `load_recent(n)` — `B1` flagged the same territory) that skips body conversion cuts index/feed cost. Negligible at four posts; noted in the performance budget as it scales.

### 4.5 Dependencies
- Existing only for shipped scope: `pulldown-cmark` 0.10 with `Options::all()` (tables, footnotes, strikethrough — `post.rs:96-97`; `Cargo.toml:26`), `gray_matter` 0.2 YAML (`post.rs:24-25,82`; `Cargo.toml:27`), `chrono` (`NaiveDate` parse — `post.rs:93`), `askama`/`askama_axum`, `serde`.
- **Feed (planned):** prefer hand-rolled Atom via `askama` (a `feed.xml` template) to avoid a new crate and keep output auditable; or a small feed crate if hand-rolling proves error-prone (decide in §8). No new runtime service, no CDN (CSP `default-src 'self'`).
- Assets: none new. No third-party fonts/images enter via B4 (`A2` owns the planned `og-card.png`).
- Infrastructure: none. Files on disk, served by the same Axum process.

### 4.6 Platform-specific considerations
- Browser support: plain HTML + CSS; the reading path needs no JS on any browser (criteria 3A). CSP `script-src 'self'` forbids inline script, which the feature never uses (`README.md:60`).
- Askama compile-time templates: a missing `title()`/`description()`/`section()` is a **build error**, not a runtime fault (`A2` E-08 `:348`; `pages.rs:10-12`) — the strongest presentation of a metadata bug.
- `section()` migration: when `A2` lands the `Section` enum (`A2:474,494`), `BlogListTemplate::section()` and `BlogPostTemplate::section()` return `Section::Writing` instead of the `&str` `"writing"` (`blog.rs:70,140`). B4 must move in lockstep with that change (blocking dependency, §7.4).
- No feature flags; the feed can ship dark by simply not linking it until validated.

### 4.7 Performance budget
- Memory: one `Vec<BlogPost>` per request; at four posts, kilobytes. The wasted `content_html` on the list path is the only notable overhead (§4.4).
- CPU / render: Markdown conversion per request. List view converts N bodies it discards — O(total post bytes) per `/blog` hit; the metadata-only load (§4.4) removes this. Article view converts one body. No template caching beyond Askama's compiled structs.
- Network payload: index HTML scales with post count × (~one line + summary). Article payload ≈ rendered post size; the network-migration post is ~13 KB of Markdown → comparable HTML, gzipped at Caddy. Feed ≈ newest-N summaries, small.
- Storage: content is the `.md` files already on disk; no growth from the app.
- Startup: none — content is read per request, not at boot (`README.md:108-109` notes content is read from `content/` at startup only for the wiki drift test, not the blog).

---

## 5. Test Specification

### 5.1 Unit tests

| Name | Setup | Assertion | Edge covered |
|---|---|---|---|
| `grouping_orders_pillars_and_collects_the_rest_under_other` **(exists, `blog.rs:178-196`)** | Posts across Networking, Security, a nonsense category, and `None` | Labels == `["Networking","Security","Other writing"]`; empty pillars dropped; both uncategorized posts land in Other | Empty pillar drop + Other fallback |
| `from_file_parses_frontmatter_and_renders_markdown` **(new)** | Temp `.md` with valid frontmatter + a heading + a table | `BlogPost` has expected title/date/tags/category; `content_html` contains `<h2` and `<table` | Happy parse + `Options::all()` table rendering (`post.rs:96-99`) |
| `from_file_rejects_malformed_date` **(new)** | Frontmatter with `date: 2026-13-40` | `Err(SiteError::DateParse(_))` | Load-time date validation (`post.rs:93-94`) |
| `from_file_rejects_missing_frontmatter` **(new)** | `.md` with no `---` block | `Err(SiteError::MissingFrontmatter(_))` | `post.rs:88` |
| `load_all_sorts_newest_first` **(new)** | Dir with three dated posts | Result dates descending | `post.rs:135` |
| `find_returns_post_not_found_for_missing_slug` **(new)** | Empty dir, slug "nope" | `Err(SiteError::PostNotFound("nope"))` | `post.rs:147-149` |
| `every_post_category_is_a_known_pillar_or_none` **(new — drift guard)** | `load_all(content/posts)` | Each `category` is `None` or ∈ `PILLARS` | Catches a mistyped `category:` before it silently mis-files (§4.4, criteria 5B) |
| `every_post_summary_fits_meta_description` **(new — drift guard)** | `load_all(content/posts)` | `50 <= summary.len() <= 160` | The `<meta description>`/feed length contract (`A2` U-6 `:722`); today's longest summary fails, forcing the fix |

### 5.2 Integration tests

| Name | Request | Assertion |
|---|---|---|
| `blog_index_lists_present_pillars` | `GET /blog` | 200; body contains "Writing", each present pillar heading, and every published post's title + `/blog/<slug>` link |
| `blog_post_renders_full_article` | `GET /blog/management-layer-first-network-migration` | 200; body contains the post title and rendered body markup (`<h2`, and a `<table` from the post's tables) |
| `unknown_slug_returns_404` | `GET /blog/does-not-exist` | 404; boot-sequence page echoes the path (extends `errors.rs:171-182`) |
| `traversal_slug_never_reads_disk` | `GET /blog/..%2f..%2fetc%2fpasswd` and `/blog/foo..bar` | 404; no file content leaked (guard at `blog.rs:150`) |
| `blog_index_needs_no_javascript` | `GET /blog`, strip all `<script>…</script>` | Remaining body still contains every post link and the `/blog` structure (the machine-checkable no-JS floor; mirrors `A2` I-2 `:742`) |
| `article_declares_og_type_article` **(after §7.2 lands)** | `GET /blog/:slug` | `<head>` has `og:type` = `article` (`A2` I-8 `:748`) |
| `vitals_strip_appears_on_pages` **(exists, `status.rs:113-123`)** | `GET /blog` | Body contains `vitals-strip` |
| `feed_is_valid_and_lists_newest` **(planned)** | `GET /feed.xml` | 200; `Content-Type` atom/rss; parses as XML; contains newest posts with absolute `/blog/<slug>` URLs |

### 5.3 UI / E2E tests
No JS test harness exists or is warranted (no client behavior to drive). "E2E" here = the `tower::ServiceExt::oneshot` router round-trips in §5.2, which exercise routing → handler → template → HTML exactly as a browser navigation would (pattern established in `errors.rs:171-193`, `status.rs:113-123`). Manual browser navigation is covered in §5.4.

### 5.4 Visual / manual verification
- **Themes:** render `/blog` and `/blog/management-layer-first-network-migration` in at least Lunarcore, Solarcore, `solarized` (its 5 known contrast failures — `A1` Q2), CRT (scanlines over `pre`), and Paper (serif measure). Confirm `pre`/tables scroll rather than overflow the page; confirm `.post-date`/`.post-summary` faint text stays legible (`A1` contrast scope).
- **Text size:** browser zoom to 200%; confirm the prose column reflows at `--measure` and code still scrolls.
- **Screen size:** 320px mobile to wide desktop; the body must never scroll horizontally from a wide code block or table (`style.css:1124-1130,1167-1176`).
- **Empty vs. populated:** verify the designed empty state (planned) with `content/posts` temporarily emptied, and the populated index with all four posts.
- **Feed (planned):** validate `/feed.xml` with `xmllint --noout` and the W3C Feed Validator; subscribe in one reader.

**Verification commands (criteria 5D — run in CI):**
```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
```

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification
- [x] **No sensitive data involvement.** The feature reads author-written Markdown and serves it publicly. No user input is stored or reflected except the requested slug, which is validated and HTML-escaped in the 404 (`errors.rs:161-169`). No auth, no PII, no secrets.

### 6.2 Asset provenance
- [x] **No third-party assets** introduced by B4. Fonts are OS-provided via theme stacks (`A1`); no images ship with a post today. If a post embeds an image later, it must be author-produced or correctly licensed and served from `/static` under CSP `img-src 'self' data:` — noted as an authoring rule, not shipped scope. The planned `og:image` is `A2`'s asset.

### 6.3 Language / claims audit
- [ ] Makes claims not supported by evidence? **No.** Every published post is first-person operations work with reproducible evidence (real `dig`/`curl` output in `hosting-machinageist-dev.md:37-67`; the outage narrative with verification method in `management-layer-first-network-migration.md`).
- [ ] Promises capabilities not yet built? **No** — the feed, empty state, anchor links, `og:type`, tag pages, and metadata-only load are all labeled **planned** in this spec and must not read as shipped in any user-visible copy.
- [ ] Uses domain-restricted language? **No.** B4 copy carries no certification claim, no offensive-security identity, no "secured/production-grade/SRE/enterprise" framing. The index intro (`blog_list.html:6-9`) and `description()` (`blog.rs:67-69`) say "defensive security" and "what broke and how it got fixed" — the permitted posture (criteria 1E). The Security-pillar post is scoped explicitly as "header hardening on a personal site, not a claim that the application is 'secured'" (`security-headers-on-machinageist-dev.md:12-13,92-94`).

### 6.4 Regulatory alignment — criteria Lens 3
- **3A no-JS floor:** index, article, and feed are server-rendered plain HTML with real URLs; JS never participates in reading (§3.7, §5.2 no-JS test). *Auto-fail rule 3 satisfied.*
- **3B contrast & color independence:** all colors are theme tokens under `A1`'s `--check` audit; state/hierarchy is never hue-only (weight/case/size, underlined prose links) (§3.7). *Auto-fail rule 2 addressed.*
- **3C keyboard & focus:** native links, visible focus ring, DOM focus order (§3.7).
- **3D semantics:** real heading outline; the pillar-region landmark over-labeling is flagged as a refinement (§3.7).
- **3E motion:** only hover motion, behind `prefers-reduced-motion` (§3.5).
- **3F responsive & resilient:** fluid column, scrolling `pre`/tables, designed empty state (planned), honest 404/500 (§3.6).

**Claim-integrity gate (Lens 1C) — GeistScope publication gate.** *Binding, not auto-fail.* Publishing = moving a draft into `content/posts/`. `content/drafts/geistscope-retrospective.md` (and the other drafts) must **not** be promoted into the writing index until the GeistScope gate clears (full pipeline + human and AI operation + sanitized evidence from an authorized engagement). Work-in-progress may live on a progress surface (`C4`), never in `/blog`. The feature-tree already scopes `content/drafts/` out (feature-tree "Explicitly out of scope"); this spec restates it as B4's publish discipline.

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**Implemented (shipped, tested where noted):**
- `/blog` grouped-by-pillar index and `/blog/:slug` article (`router.rs:40,47`; `blog.rs`; both templates).
- Pillar grouping with fixed order + "Other writing" fallback, empty pillars dropped, newest-first within each (`blog.rs:87-120`, `post.rs:135`). Unit-tested (`blog.rs:178-196`).
- Frontmatter schema + typed load errors (`post.rs:36-110`); path-traversal defense before disk access (`blog.rs:150`); `|safe` used only on trusted `pulldown-cmark` output (`blog_post.html:18-29`, `post.rs:17-18`).
- Markdown with tables/footnotes/strikethrough (`Options::all()`, `post.rs:96-97`); article prose capped at `--measure`, `pre`/tables full-width scrolling, article-heading size restored (`style.css:1090-1192`).
- Four published posts (`content/posts/*.md`): Networking (network-migration — the criteria-4B model), Linux/SysAdmin (hosting), Security (headers), and one uncategorized orientation piece (solarpunk → "Other writing").
- Shared coupling: `POSTS_DIR` `pub(crate)` and the `BlogPost` model reused by `B1`'s home teaser (`pages.rs:16,55-63`); `/blog` covered by the shell vitals test (`status.rs:113-123`).

**Absent / planned (target adds):**
- **RSS/Atom feed** — *absent.* No `/feed.xml`, no `<link rel="alternate">`. Highest-value gap for the peer-engineer reviewer path (criteria 4E) and the standard writing-site expectation (benchmark: Julia Evans, Simon Willison, Brandur — criteria Lens 2).
- **`og:type=article` for posts** — *absent.* `base.html:9` hardcodes `website`; `BlogPostTemplate` supplies no override (`A2` I-8 wants `article`).
- **Designed empty state** — *absent.* Zero posts renders `h1` + intro over blank space.
- **Heading anchor links / ids** — *absent.* `pulldown-cmark` emits headings without `id`s, so no deep-linking within a post; CSS already anticipates this ("Headings carry no ids today, so nothing else on the page is an anchor target yet." — `style.css:1157-1158`). Scannability gap vs. the Arch Wiki/MDN benchmark (criteria 2D).
- **Group-heading styling** — *unstyled hooks.* `.post-group`/`.post-group-heading` classes exist with no rule (inherit base `h2`); `A1` §7.1.6 owns the decision.
- **Summary length + category-typo guards** — *absent.* Both fail silently today (§4.4, §5.1).
- **Metadata-only / `load_recent(n)` load path** — *absent.* List converts bodies it discards (`post.rs:96-99`).
- **Tag pages / category filter routes** — *absent.* Tags render as inert pills (`blog_post.html:9-13`); no `/blog/tag/:tag`. (Open question — adds routes; may violate restraint.)

**Gated (must not ship as writing):**
- `content/drafts/geistscope-retrospective.md` and siblings — GeistScope publication gate (Lens 1C, §6.4).

**Copy currency (criteria 1D/1E):** B4's own copy is clean — no cert claim, no senior/offensive framing (`blog_list.html:6-9`, `blog.rs:67-69`, `security-headers…md:12-13`). B4 does **not** carry the stale "working through the CompTIA stack" line — that lives in `pages.rs` (owned by `B1`/`B2`).

### 7.2 Delta to spec

**New files:**
- `src/handlers/feed.rs` (or `blog::feed`) + `templates/feed.xml` — the Atom/RSS feed (planned).
- New tests: `from_file`/`load_all`/`find` unit tests + the two drift guards (§5.1), and the `/blog` integration tests (§5.2) — either in `blog.rs`/`post.rs` `#[cfg(test)]` modules or a `tests/blog.rs` integration file.

**Modified files:**
- `src/models/post.rs` — add `load_all_meta()`/`load_recent(n)` (metadata-only load); no schema change.
- `src/handlers/blog.rs` — `list()` uses the metadata path; add the category/summary load-time guards (or as tests); add `blog::feed`; add `og_type()` on `BlogPostTemplate` (article signal); adopt `Section::Writing` when `A2` lands.
- `templates/blog_list.html` — designed empty-state branch; intro measure via `--measure-narrow` (token change lands in `A1`); `aria-labelledby` on pillar sections instead of `aria-label`.
- `templates/blog_post.html` — supply the `og:type=article` signal (mechanism owned by `A2`'s `base.html`/`head_extra`); optionally add heading-anchor rendering (needs a `pulldown-cmark` heading-id pass in `post.rs`).
- `src/router.rs` — add `/feed.xml`.
- `static/css/style.css` — empty-state class; anchor-link affordance if headings become anchor targets (heading `id` + a `#` link on hover); no group-heading rule unless `A1` §7.1.6 decides one.
- `content/posts/management-layer-first-network-migration.md:4` — shorten `summary` to ≤160 chars (meta-description contract).

**Migrations / schema:** none (flat files).
**New dependencies:** none required (hand-roll Atom in Askama); a feed crate is optional (§8).

**Docs that must follow behavior (criteria 5E):**
- `README.md:50-62` — the project-structure block lists only three posts and omits the network-migration post; update the count/listing (it is already stale). Advertise the feed once it lands.
- `SOLARCORE_SPEC.md` / `A1` — if `A1` §7.1.6 formalizes group-heading styling, record it there, not per-page.

### 7.3 Estimated scope
**Shipped surface: S** (already implemented; the delta is guards + refinements). **Full target including feed + empty state + guards + `og:type` + anchor links: M.** Justification: the feed is a self-contained handler + template + route + one test; the guards are small; the metadata load is a refactor of one function; anchor links require a `pulldown-cmark` heading-id pass (the largest single piece). No new dependency, no migration, no client JS. It does not reach L because there is no data-model change, no auth, and no cross-cutting infrastructure.

### 7.4 Blocking dependencies
- **`A1` design-system** — `--measure-narrow` token and the group-heading decision (§7.1.6) must land for the intro-measure and heading-styling deltas. *Passed (2.62).*
- **`A2` site-shell** — the `Section` enum (`section()` return-type change), `aria-current="page"`, and the `og:type=article` / `og:image` / `<link rel="alternate">` `<head>` mechanism all live in `base.html`. B4's `og:type` and feed-advertisement deltas depend on `A2`. *Passed (2.53).* B4 must not edit `base.html` directly (surgical-change discipline).
- **No external gate** for the shipped reading path. The GeistScope gate (§6.4) blocks *promoting a specific draft*, not the feature.

---

## 8. Open Questions

- **Q1: Feed format and location** — Atom or RSS 2.0? `/feed.xml` at root, or `/blog/feed.xml`? Full-content entries or summary-only? Hand-rolled Askama template (no new crate) or a feed crate? — blocks §4.3/§4.5 and the feed tests. *Recommendation: Atom at `/feed.xml`, summary-only entries with absolute links, hand-rolled to keep output auditable under CSP and avoid a dependency.*
- **Q2: Heading anchor links** — worth the `pulldown-cmark` heading-id pass (deep-linking, criteria 2D scannability) for posts of this length, or over-engineering at four posts? If yes, do headings also get a visible `#` affordance on hover (must degrade without JS — a plain `<a href="#slug">`)? — blocks the anchor-link delta in §7.2.
- **Q3: Tag pages / category filtering** — make tags clickable to `/blog/tag/:tag` and add a per-pillar filter, or keep tags as inert metadata for restraint (criteria 2E)? Adds routes and an index-by-tag load. — blocks whether §7.1's "tag pages" moves from absent to planned. *Lean: defer until post count justifies it.*
- **Q4: Summary as `<meta description>` vs. a separate field** — cap `summary` at 160 chars (one field serves list + meta + feed), or add an optional `meta_description` frontmatter key so the list summary can stay long? — blocks the §5.1 length guard and the `management-layer` summary edit. *Lean: single 160-char summary; simplicity over a second field.*
- **Q5: Sub-feature — does the feed belong under B4 or as its own leaf?** Per `GAUNTLET.md` dispatch, the feed could be a small child of B4 rather than a sibling capability. It is spec'd here as B4's own planned surface; if the gauntlet prefers it as a tracked leaf, assign it an ID (e.g. `B4a-feed`). — reported per the "no sub-agents; report sub-feature needs in Open Questions" rule.
- **Q6: Pagination threshold** — at what post count does the single flat index need pagination or year-grouping? (Four today; the pillar grouping absorbs growth for a while.) — informs the §7.3 scaling note; not blocking now.
