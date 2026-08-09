# Spec: Home

**Feature ID:** `B1` / `home`
**Parent feature:** root (Content surfaces)
**Spec author agent:** spec-agent-4 (Claude Opus 4.8)
**Date:** 2026-08-08
**Iteration:** 1

---

## 0. Reading notes and scope boundary

Everything asserted about current state was read from source, not docs. Citations
are `path:line` or `path:line-range`.

**In scope (B1 owns):** the `/` route, the `home()` handler and `IndexTemplate`
(`src/handlers/pages.rs:24-63`), the page body template `templates/index.html`,
the home-specific copy (hero lede, "Lately" list, section pointers, the `<meta
name="description">` value the home page supplies to the shell), and the three
home render tests (`pages.rs:146-202`). The home-only CSS classes `.hero`,
`.hero-lede`, `.hero-actions` (`style.css:926-955`, `1553`).

**Out of scope, inherited:**

| Concern | Owner | What B1 assumes from it |
|---|---|---|
| `<head>`, header, nav, theme selector, footer, vitals strip, skip link, `<main tabindex>` | `A2` site-shell | Home extends `base.html`; supplies `title()`/`description()`/`section()`. Home is **not** a nav item, so no nav link highlights on `/`. |
| Tokens, type scale, 23-theme roster, contrast audit, the `--text-faint`/`--text-muted` remediation | `A1` design-system | `.hero-lede`/`.post-summary`/`.post-date` read `--text-muted`/`--text-faint`; their AA-at-size correctness is A1's matrix, not restated here. |
| The post data the teaser lists (`BlogPost`, `content/posts/`, `/blog`) | `B4` writing | `BlogPost { slug, title, date, summary }` (`post.rs:53-62`), newest-first (`post.rs:135`). B1 reads, never writes, that model. |
| Shared list components `.about-list`, `.post-list`/`.post-item`/`.post-date`/`.post-summary`, `.section-more` | shared with `B2` about / `B4` writing | One CSS definition; changing them ripples — see §5A. |

Where B1 needs a change inside another feature's territory it is filed as a
**cross-feature request** in §7.4, not specified here.

---

## 1. Purpose

### 1.1 One-sentence job

Give a first-time visitor at the site root, in one screen and without scrolling
past the fold, an honest answer to "whose site is this, what do they actually
operate, and where do I go next" — so a thirty-second skim ends with the reader
clicking through to evidence rather than bouncing.

### 1.2 Why it matters

`/` is the most-requested route and the only page whose whole job is orientation.
Three specific pressures land here:

1. **Thirty-second differentiation (Lens 4A).** The competitor set is junior
   homelab portfolios and cert-track candidates (`criteria.md` Lens 4). Both open
   with enthusiasm and a wall of aspiration. The home page's differentiator is the
   opposite: a grounded lede naming a real system Jeff operates ("a Proxmox
   homelab … a three-node cluster … documenting how it actually behaves",
   `index.html:6-10`), a "Lately" list of concrete in-flight work
   (`index.html:22-26`), and immediate routes to the evidence. The reader should
   be able to tell in one screen that this person runs something real.

2. **Reviewer routing (Lens 4E).** Three readers arrive at `/`: a hiring manager
   (needs Portfolio and a one-line "what"), an engineer peer (needs writing that
   survives scrutiny, plus the shell's live vitals strip as proof-of-operation),
   and a self-directed learner (needs the Learn wiki). The page is the switchboard
   — its `.hero-actions` nav (`index.html:11-15`) and two section pointers
   (`index.html:44`, `:57`) must cover all three paths without forcing a reader
   back to the header.

3. **Claim discipline at the front door (Lens 1).** The home page's copy — hero
   lede, "Lately" bullets, and the `<meta name="description">` it feeds the shell
   — is the first place an overclaim would cost. It must lead with what the claim
   discipline permits (owned homelab, real operations) and never with a title or
   credential it cannot defend (`criteria.md` 1E, and the copy-voice rule: quiet,
   show-don't-tell, no strategy narration). The existing anti-overclaim test
   (`pages.rs:160-166`) already encodes this and must hold.

### 1.3 Success signal

**Observable:** with JavaScript disabled, `GET /` renders a complete page whose
first screen names the real system, offers three working links to Portfolio /
Writing / Learn, and shows the newest real posts (or gracefully omits that
section when content fails to load). No copy on the page asserts a certification,
a role, or a capability that is not true today.

**Measurable:** `cargo test --all-targets` passes, including the three home tests
(`pages.rs:146-202`), **each of which asserts against the surface it names** — the
current `assert!(html.contains("CompTIA"))` at `pages.rs:158` does not (§5C, §7),
and closing that gap is part of this spec's success condition.

---

## 2. User Stories

> **Happy path — hiring manager.** As a hiring manager landing on `/` from a
> resume link, I want the first paragraph to tell me what this person operates and
> a visible "Portfolio" link right under it, so that I can reach the evidence in
> one click without reading the whole page.

> **Happy path — engineer peer.** As an engineer skimming for signal, I want to
> see the titles and one-line summaries of the most recent real writeups on the
> landing page, so that I can judge the writing quality before deciding to invest
> more time.

> **Happy path — learner.** As someone who found the site looking for study
> material, I want a clear pointer from the home page to the education wiki, so
> that I land in `/learn` without hunting through the nav.

> **Edge case — content fails to load.** As any visitor arriving while
> `content/posts/` is unreadable (mid-deploy, permissions, a bad file), I want the
> home page to still render its identity and its links, with the "Latest writing"
> section simply absent rather than showing an empty heading or a 500, so that a
> content glitch is invisible to me.

> **Accessibility — screen reader.** As a screen-reader user, I want one clear
> page heading, labelled navigation for the quick links, and section structure I
> can jump between, without the same label being announced twice, so that I can
> orient and move as fast as a sighted reader.

> **Accessibility — no JavaScript.** As someone browsing with JS off, I want the
> entire home page — identity, links, recent posts — to work as plain
> server-rendered HTML with real `href`s, so that nothing on the front door
> depends on a script.

> **Maintainer.** As the person editing the home copy, I want the tests that guard
> the home page to break only when the thing they claim to check actually changes,
> so that editing the `<meta>` description does not fail a test that reads as if it
> is about the page body (the current `CompTIA` coupling, §5C).

---

## 3. UX Specification

### 3.1 Screen / view inventory

The home feature introduces exactly **one** surface. It adds no modal, sheet,
popover, or drawer.

| Surface | Path to reach | New / modified | Layout pattern |
|---|---|---|---|
| **Home page** | `/` → `pages::home` (`router.rs:37`) | Modification (exists, `index.html`) | Single 900px column inside the shell `<main>`; stacked sections separated by `<hr>` rules |

The theme menu, header, footer, and vitals strip on this page are the shell's
(A2). The home body is four stacked sections in the shell's content column.

### 3.2 Interaction flows

**Primary flow — land and route (JS-independent).**

1. `GET /` → `home()` (`pages.rs:55-63`). Handler loads posts via
   `BlogPost::load_all`, `unwrap_or_default()` on error, truncates to
   `HOME_POST_COUNT = 3` (`pages.rs:28`, `56-57`), and renders `IndexTemplate`.
2. Server returns full HTML. The shell paints (A2); the home body renders in
   `<main>`.
3. Reader reads the hero lede (`index.html:6-10`), scans the "Lately" list
   (`:22-26`), the "Latest writing" teaser (`:33-45`), and the "Learn" pointer
   (`:50-58`).
4. Reader clicks a `.hero-actions` link (`:12-14`) or a section pointer (`:44`,
   `:57`) or a post title (`:39`). Full page load to the destination. No
   client-side routing, no transition.

**Branch — posts fail to load.** `load_all` returns `Err` → `unwrap_or_default()`
yields an empty `Vec` → the template guard `{% if !posts.is_empty() %}`
(`index.html:30`) omits the entire "Latest writing" section **and its leading
`<hr>`**. The page renders hero + Lately + Learn. Verified by
`home_page_omits_the_writing_section_when_no_posts_load` (`pages.rs:186-202`).
This is deliberate: `/blog` is the route that surfaces a content failure honestly
(`pages.rs:52-54`); the front door stays up.

**Branch — fewer than three posts.** `truncate(3)` on a shorter list is a no-op;
the teaser renders whatever exists. With four posts today (`content/posts/`) the
teaser shows the newest three and "All writing →" carries the reader to the rest.

**Cues.** No haptics, no sound, no home-specific animation. The only motion in
view is the shell's (theme swap, nav underline, brand cursor), all guarded by
`prefers-reduced-motion` per A1/A2.

### 3.3 Layout descriptions

Component hierarchy, top → bottom (`index.html`):

```
<section class="hero">                         (index.html:4-16)
  ├─ <h1>{{ name }}</h1>                        the ONLY h1; renders "machinageist"
  ├─ <p class="hero-lede">…</p>                 --text-muted, max-width 55ch (style.css:935-940)
  └─ <nav class="hero-actions"                  aria-label="Quick navigation"
        aria-label="Quick navigation">          3 links, each ::before "→ " (style.css:955)
        Portfolio · Writing · Learn             → /portfolio, /blog, /learn
<hr>
<section aria-label="Lately">                   (index.html:20-27)
  ├─ <h2>Lately</h2>
  └─ <ul class="about-list">                    3 STATIC <li> bullets (shared class w/ B2)
<hr>   {# only when posts exist #}
<section aria-label="Latest writing">           (index.html:33-45) — guarded by :30
  ├─ <h2>Latest writing</h2>
  ├─ <ul class="post-list">                      iterates posts:
  │    └─ <li class="post-item">                   .post-date, <a href="/blog/{slug}">title</a>, .post-summary
  └─ <p class="section-more"><a href="/blog">All writing →</a></p>
<hr>
<section aria-label="Learn">                    (index.html:50-58)
  ├─ <h2>Learn</h2>
  ├─ <p>…education wiki pointer…</p>
  └─ <p class="section-more"><a href="/learn">Education wiki →</a></p>
```

**Data sources.**

| Component | Source |
|---|---|
| `<h1>` text | `IndexTemplate.name` (`pages.rs:33`), set to `"machinageist"` in the handler (`pages.rs:60`) |
| Hero lede, Lately bullets, Learn copy | Static text in `index.html` — not data-driven |
| Post rows | `IndexTemplate.posts: Vec<BlogPost>` (`pages.rs:35`), each `{ date, slug, title, summary }` (`post.rs:55-62`) |
| `<title>`, `<meta description>` | `IndexTemplate::title()` / `description()` (`pages.rs:40-45`) via `base.html:6,10` |

**Empty states.** The home page **is** the reference implementation of the
design-system empty-state invariant (A1 §3.3 cites `index.html:30`): an empty
section is omitted entirely rather than rendered as a heading over nothing. No
placeholder card, no spinner, no "no posts yet" copy — the section simply is not
there. This behaviour is load-bearing and pinned by test (`pages.rs:186-202`); it
must be preserved.

### 3.4 Input & gestures

- **Pointer.** Click on any of: 3 hero-action links, up to 3 post titles, "All
  writing →", "Education wiki →". All are plain `<a href>`. `.post-item` gains a
  `--surface` hover fill and inset accent edge from the shared component rule
  (`style.css:730`); additive, no reflow.
- **Keyboard.** Every link is a native anchor, in DOM order, reachable by Tab
  after the shell chrome. No custom key handling on this page. No page-level
  shortcuts (correct — see A2's WCAG 2.1.4 reasoning).
- **Touch.** Link tap targets are text links at body size; they sit in the flowed
  column with comfortable spacing (`.hero-actions` gap 1.5rem, `.post-item`
  padding 1.25rem vertical). No hover-only affordance carries meaning.
- **Specialised input.** N/A — text and links only.
- **Responsive.** Inherits the shell's 640px breakpoint; the only home-specific
  responsive rule shrinks `.hero` bottom margin to 2.5rem (`style.css:1553`).
  `.hero-actions` is `flex-wrap: wrap` (`style.css:948`) so the three links wrap
  on narrow screens instead of clipping. The single 900px column reflows naturally
  from 320px to wide desktop.

### 3.5 Transitions & animation

The home page introduces **no motion of its own**. All motion visible on `/` is
the shell's (A2 §3.5) and is already guarded by
`@media (prefers-reduced-motion: no-preference)`. Reduced-motion alternative:
absence, inherited from the shell. There is nothing for B1 to add or guard here,
and the spec deliberately adds none — a landing page that animates its own body
copy would violate Lens 2E (restraint) and 3E (motion safety).

### 3.6 Error states

| ID | Trigger | Presentation | Why | Recovery | Data loss |
|---|---|---|---|---|---|
| **HE-1** | `content/posts/` unreadable / a post fails to parse | "Latest writing" section (heading + list + its `<hr>`) omitted; rest of page renders | A content glitch must not take down the front door; `/blog` surfaces the failure honestly (`pages.rs:52-54`, `56`) | Reader still has hero, Lately, Learn, and full shell nav | No |
| **HE-2** | Zero posts exist at all | Identical to HE-1 — section omitted | Same guard (`index.html:30`); no "no posts" placeholder | Navigation intact | No |
| **HE-3** | Template fails to render (missing `title()`/`description()`/`section()`) | **Compile error** — Askama validates at build time (`pages.rs:10-12`) | Strongest possible: never reaches a user | Fix the code | N/A |
| **HE-4** | A post has an empty `summary` | `<p class="post-summary">` renders empty; benign | `summary` is a required frontmatter field (`post.rs:39,46-47`) so this is unlikely, but a blank paragraph is harmless | — | No |

There is **no toast, banner, or inline error** on the home page and none is
proposed. Every failure mode here is either compiled away (HE-3) or degrades to a
quietly smaller page (HE-1/HE-2). Interrupting a reader with a notice about
missing posts would be worse than the absence.

### 3.7 Accessibility

Graded as an auto-fail gate (Lens 3). Written as invariants + shipped state.

**A. No-JS (auto-fail rule 3).** ✅ Shipped. The entire home page is
server-rendered. Every interactive element is a native `<a href>` to a real URL
(`index.html:12-14,39,44,57`). Nothing on `/` requires JavaScript; the only JS on
the page is the shell's theme selector, and its absence leaves the home content
fully usable. This is the core function and it is reachable with JS off.

**B. Heading outline.** ✅ One `<h1>` (`index.html:5`), three `<h2>` section
labels (`:21,34,51`). No skipped levels. The `<h1>` is the site name
("machinageist"); the "what I do" is carried by the lede immediately under it.
Invariant: exactly one `<h1>` on this page (A2 test U-4 covers it).

**C. Landmarks and the region-label redundancy (target fix).** The hero's quick
links are a labelled `<nav aria-label="Quick navigation">` (`index.html:11`) —
correct; it distinguishes them from the shell's Primary nav. **However**, the
three content `<section aria-label="…">` elements (`:20,33,50`) each *also*
contain an `<h2>` with the **same text** ("Lately", "Latest writing", "Learn").
`aria-label` on a `<section>` promotes it to a `region` landmark whose accessible
name then duplicates the visible heading, so a screen reader announces the name
twice ("Lately, region" → "Lately, heading level 2"). **Target:** either (a) drop
the `aria-label` and let the `<h2>` provide structure (a plain `<section>` is not
a landmark and needs no name), or (b) switch to
`aria-labelledby="<h2-id>"` so the region's name **is** the heading — one name,
one announcement, and the section becomes a properly named landmark. Option (b) is
preferred if the region landmarks are wanted for navigation; option (a) is simpler
if they are not. This is a real craft nit (Lens 3D), not a blocker.

**D. Colour independence.** ✅ Links are underlined-by-default or accent-coloured
with the shell's global focus ring; no state on this page is signalled by hue
alone. The `.hero-actions` "→ " prefix (`style.css:955`) is decorative and read as
literal text by AT — acceptable, but note it is content, not `aria-hidden`.

**E. Contrast at usage size.** Deferred to A1's matrix, but B1 is a **direct
consumer** of the flagged tokens: `.hero-lede` is `--text-muted` at 0.9rem
(`style.css:935-940`), `.post-summary` is `--text-faint` at 0.85rem
(`style.css:1018-1022`), `.post-date` is `--text-faint` at 0.78rem
(`style.css:1011-1016`). A1 §3.7A raises `--text-faint` to a 4.5:1 requirement at
these sizes (14 current failures). B1 inherits whatever A1 lands; it must not
introduce a new small-faint pairing beyond these.

**F. Text scaling.** The home-specific sizes are `rem`/literal (`h1` 1.75rem,
lede 0.9rem, actions 0.875rem — `style.css:930,939,945`). They scale with zoom.
The one caveat is A1's known `body { font-size: 15px }` pixel floor (A1 §3.7F),
which affects every page including this one; B1 defers the fix to A1 and adds no
new pixel font-size.

**G. Focus order.** After the shell chrome, Tab reaches: 3 hero-action links →
(post titles when present) → "All writing →" → "Learn" pointer, in DOM order.
Visible focus ring is the shell's global `:focus-visible` (`style.css:685`). No
`tabindex` on this page; no trap.

---

## 4. Implementation Specification

### 4.1 Architecture placement

```
src/
  router.rs                 route "/" → pages::home (router.rs:37) — unchanged
  handlers/pages.rs         home() + IndexTemplate (pages.rs:24-63) + tests (146-202)
  models/post.rs            BlogPost, load_all — READ ONLY from B1's view (B4 owns)
templates/
  index.html                the page body (extends base.html)
static/css/style.css        .hero/.hero-lede/.hero-actions (926-955, 1553); shared list classes
```

B1 adds **no new module and no new file**. All changes are edits to the existing
handler, template, tests, and copy. The A2 migration of `section()` from `&str`
to a `Section` enum (A2 §4.2) touches `pages.rs:46-48`; B1 defers to A2 for that
signature and only requires that home continues to highlight no nav item.

### 4.2 Data model

`IndexTemplate` — the home page's only type (`pages.rs:30-36`), reproduced with
the two changes this spec asks for:

```rust
// Author:      machinageist
// Date:        2026-04
// Description: Home page model — the owner name for the <h1> and the newest posts
//              for the teaser. posts is capped at HOME_POST_COUNT and may be empty;
//              an empty vec renders no writing section at all (index.html:30).
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: String,
    // Newest-first, capped at HOME_POST_COUNT — empty renders no section at all
    pub posts: Vec<BlogPost>,
}
```

**Change 1 — `name` is a constant, not a variable (§5A, optional).** `name` is
always `"machinageist"` (`pages.rs:60`) and duplicates the same string in
`title()` (`pages.rs:41`), the header brand-word (`base.html:21`), and the footer
(`base.html:91`). It is a field that carries no variability. The minimal-surface
option is to keep it (it is harmless), but the single-source-of-truth-correct
option is to drop the field and render a shared site-name constant (naturally
folded into A2's shell constants). Flagged as Q1; not a blocker.

**Change 2 — `description()` copy currency (§6.3, Lens 1D — required).** Current:

```rust
pub fn description(&self) -> &str {
    "Homelab, networking, and Linux notes from machinageist — a Proxmox lab, CompTIA study, and the projects that come out of it."
}   // pages.rs:43-45
```

The phrase **"CompTIA study"** is stale. The live certification spine (re-locked
2026-08-02, `criteria.md` 1D) is **RHCSA → CCNA → Security+**, with Network+
dropped and RHCSA (Red Hat, not CompTIA) the lead and only pre-employment exam.
Framing the study as "CompTIA" now names the last of three certs as if it were the
program, and the README records that public cert claims were removed pending a
booked voucher (`README.md:14-16`). The `<meta name="description">` is
user-visible copy (rendered into `base.html:6`), so it falls under claim
discipline. **Target:** remove the cert reference and describe the real work,
staying within the 50–160 char meta window (A2 U-6). Proposed:

```rust
pub fn description(&self) -> &str {
    "Homelab, networking, and Linux notes from machinageist — a Proxmox cluster, the operations work it generates, and the writeups that document it."
}   // ~145 chars; no cert claim, no role claim
```

Dropping the cert reference (rather than swapping in "RHCSA study") is the
claim-safe choice: auto-fail rule 1 forbids introducing a certification claim
without a booked voucher, and the README's own rule is "no public cert claims
until an exam voucher is booked." Whether a booked voucher exists that would
permit naming a single exam is Q2. **No database, no migration** — the site has
none.

### 4.3 API contracts

One HTTP route, unchanged:

| Method | Path | Handler | Returns | Auth | Rate limit |
|---|---|---|---|---|---|
| `GET` | `/` | `pages::home` (`router.rs:37`) | `200 OK`, `text/html`, the rendered `IndexTemplate` | None | The global limiter (A3, `router.rs:72-75`) applies uniformly |

There is no error status path: `home()` is infallible by construction — it
`unwrap_or_default()`s the post load (`pages.rs:56`) and returns `impl
IntoResponse`, so it can only produce `200`. Pagination is N/A (the teaser is a
fixed `HOME_POST_COUNT = 3`, `pages.rs:28`). The template contract with the shell
(`title`/`description`/`section`) is A2 Contract S-1; home is the one page whose
`title()` is the bare wordmark rather than `"… — machinageist"` (A2 U-5 exempts
it).

### 4.4 State management

| State | Owner | Lifetime | Sync |
|---|---|---|---|
| `name` | `IndexTemplate.name`, set in `home()` | Per-request | Server only; a constant in practice |
| `posts` | `IndexTemplate.posts`, loaded per request from `content/posts/` | Per-request | Server only; read from disk, never cached, never client state |

**No new state container.** Posts are read from the filesystem on every request
(`pages.rs:56`) — acceptable for a four-post directory on a personal site, and it
means a newly published post appears without a restart. No store, no session, no
client state. Offline/draft persistence: N/A — nothing is authored in the browser.

### 4.5 Dependencies

**New packages: none.** The page uses `askama` (compile-time templating) and the
existing `BlogPost` model. No new assets — the home page requests no image of its
own (the shell's favicon is the only image any page loads, A1 §4.7). No
infrastructure change. CSP unaffected (no inline script/style on this page; the
`.hero-actions` "→ " is CSS `content`, `style.css:955`).

### 4.6 Platform-specific considerations

- **Browser support:** the home body uses only flexbox and `::before` — supported
  everywhere. No `:has()`, no `::details-content`, none of the shell's newer CSS
  features are used by home-specific rules.
- **No feature flags / rollout:** single binary, single deploy. N/A.
- **CSP:** no inline `<script>` or `style=` on this page; nothing to reconcile.
- **Migration concern:** the only forward-looking change is A2's `section() ->
  Section` enum migration, which will touch `pages.rs:46-48`. B1 requires only
  that `Section::Home` continues to match no nav item so `/` highlights nothing.

### 4.7 Performance budget

| Dimension | Current | Note |
|---|---|---|
| Handler cost | One `read_dir` of `content/posts/` + parse of up to N `.md` files per request (`post.rs:117-137`), then `truncate(3)` | Four small files today; negligible. If the corpus grows large this becomes a per-request full scan (parses **all** posts, then discards all but 3) — see Q3 |
| Rendered HTML | Home body is small: hero + 3 bullets + ≤3 post rows + 1 pointer. Dominated by the shell (A2 §4.7), not the body | No home-specific budget concern |
| Network | Zero additional requests beyond the shared `style.css` + shell JS | No home-only asset |
| Client storage | None | Home stores nothing |

**Noted inefficiency (not urgent):** `home()` loads and parses *every* post via
`load_all` then throws away all but three (`pages.rs:56-57`). At four posts this
is free; it is recorded here so a future large corpus does not silently make the
front door the slowest route. A `load_recent(n)` on `BlogPost` (B4 territory)
would fix it. Flagged Q3.

---

## 5. Test Specification

### 5.1 Unit tests

Home has three existing render tests in `pages.rs::tests` (`146-202`). They render
`IndexTemplate` directly and assert on the HTML string. Target state keeps all
three and fixes the one that lies about what it checks.

| # | Name | State | Assertion | Edge case |
|---|---|---|---|---|
| HT-1 | `home_page_shows_concrete_work_without_strategy_narration` (`pages.rs:146-167`) | **exists; one assertion is mis-coupled** | Body contains "homelab", "Proxmox"; and does **not** contain "infrastructure-support", "in training", "evidence-first", "security engineer", "offensive security", "red-team" | The anti-overclaim guard (`:160-166`) — **must be preserved** (Lens 1F) |
| HT-2 | `home_page_teases_recent_posts_and_links_to_the_full_list` (`pages.rs:169-184`) | exists; correct | With one post, body contains "Latest writing", `/blog/<slug>`, the title, "All writing", `/learn` | Teaser renders and offers a way through |
| HT-3 | `home_page_omits_the_writing_section_when_no_posts_load` (`pages.rs:186-202`) | exists; correct | With zero posts, body **lacks** "Latest writing" and "All writing" but **keeps** "Lately" and `/learn` | The empty-state omission (§3.6 HE-1) |

**Required fix — HT-1's `CompTIA` assertion (Lens 5C, auto-fail-adjacent for claim
integrity of the test suite).** `pages.rs:158` asserts `html.contains("CompTIA")`.
"CompTIA" appears in the home page **only** through the `<meta name="description">`
tag (rendered from `description()` into `base.html:6`) — it is **not** in the
visible body of `index.html`. So this assertion:

1. **Is hidden coupling** — it reads as a claim about the page body but passes
   only via a meta tag; editing unrelated body copy cannot break it, and editing
   the meta description (as §4.2 Change 2 requires) breaks it for a reason the test
   name does not explain. This is the exact 5C failure `criteria.md` cites by name.
2. **Pins stale copy** — it actively requires the word "CompTIA" to stay, working
   against the 1D copy-currency fix.

**Target:** remove the `contains("CompTIA")` assertion and replace the intent with
two honest, well-named tests:

- `home_description_carries_no_retired_claims` — assert `IndexTemplate::description()`
  (the string method, named directly) does **not** contain `"CompTIA"`, `"Network+"`,
  `"A+"`, `"the CompTIA stack"`, `"offensive security"`, `"red-team"`, `"pentest"`,
  `"production-grade"`, `"enterprise"`, `"SRE"`. This mirrors A2 U-7 and targets the
  surface it names.
- Keep HT-1's remaining body assertions (concrete work + anti-overclaim), which
  correctly test the body.

New home-body assertions worth adding to HT-1 or a sibling test:

- The hero declares exactly one `<h1>` (also covered globally by A2 U-4).
- The `.hero-actions` nav contains `/portfolio`, `/blog`, and `/learn` — the
  reviewer-routing contract (§1.2 Lens 4E) made machine-checkable.

### 5.2 Integration tests

Router-level, `tower::ServiceExt::oneshot` (the pattern at `status.rs:113-123`,
`errors.rs:171-182`). These belong to A2's shell suite (I-1, I-2) but home is one
of the routes they iterate:

| # | Name | Assertion for `/` |
|---|---|---|
| HI-1 | `every_route_renders_the_full_shell` (A2 I-1) | `GET /` body contains skip link, header, Primary nav, `<main id="content"`, footer, vitals strip |
| HI-2 | `shell_needs_no_javascript_to_be_complete` (A2 I-2) | With `<script>` elements stripped, `GET /` still contains `/portfolio`, `/blog`, `/learn`, and (when posts exist) `/blog/<slug>` — **the machine-checkable no-JS floor for the front door** |
| HI-3 | `pages_outside_the_nav_highlight_nothing` (A2 U-3) | `GET /` body contains zero `is-active` / zero `aria-current` — home is not a nav item |

B1 does not own these but depends on `/` being included in each. If A2's iteration
omits `/`, B1 requires it be added.

### 5.3 UI / E2E tests

**Absent, and deliberately not proposed.** There is no browser-automation harness
in the repo (A2 §5.3), and the home page has no client-side behaviour of its own
to drive — every interaction is a native link resolvable by the integration tests
above (served-bytes assertions). Adding a headless browser to verify a static
landing page would cost far more than it buys. Stated as a decision, not an
omission.

### 5.4 Visual / manual verification

Home is one surface in A2/A1's tiered matrix. The home-specific checks:

- **Themes:** render `/` in Tier-1 themes (Lunarcore, Solarcore, Paper, Cloud,
  Solarized, CRT — A1 §5.4) and confirm the hero lede (`--text-muted`) and
  `.post-summary`/`.post-date` (`--text-faint`) clear AA at their sizes.
- **Empty state:** run with `content/posts/` emptied/renamed and confirm the page
  renders hero + Lately + Learn with no "Latest writing" heading, no dangling
  `<hr>`, and no gap (§3.6 HE-1).
- **Populated state:** with four posts, confirm exactly three teaser rows and the
  "All writing →" pointer.
- **Sizes:** 320px, 640px (nav/hero wrap boundary), 1280px; 200% zoom and 24px
  default font — confirm `.hero-actions` wraps instead of clipping.
- **No-JS:** load `/` with JS disabled; confirm all links work and the page is
  complete (this is the auto-fail regression the whole suite exists to prevent).
- **Reduced motion:** confirm no home-body motion appears (there is none to
  suppress; this verifies none was added).

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement.** The home page reads public post metadata
  from disk and renders static copy. It stores nothing, sets no cookie, collects
  no input, and transmits nothing about the visitor. The only client-side state on
  the page is the shell's `localStorage["theme"]` (A2), not B1's.

### 6.2 Asset provenance

- [x] **No third-party assets.** The home page ships no image, font, or data file
  of its own. The favicon and font stacks are the shell/design-system's (A1/A2)
  and are OS-provided or repo-generated. The "→ " and "↗" glyphs are Unicode
  characters in CSS `content` / template text, not assets.

### 6.3 Language / claims audit

- [x] Makes claims not supported by evidence — **one stale item, flagged and
  fixed here.** The `<meta name="description">` "CompTIA study" (`pages.rs:44`) is
  stale relative to the 2026-08-02 cert spine (Lens 1D) and is corrected in §4.2
  Change 2 by removing the cert reference. Every other claim on the page is
  defensible: "I run a Proxmox homelab" (owned hardware, `README.md:27`), the
  three "Lately" bullets (real in-flight work, and the cluster/DNS/hosting are the
  subjects of shipped posts), and the "Latest writing" list (real posts from
  `content/posts/`).
- [x] Promises capabilities not yet built — **no.** The page points only to
  surfaces that exist (`/portfolio`, `/blog`, `/learn`).
- [x] Uses language restricted by domain regulations — **no.** The anti-overclaim
  test (`pages.rs:160-166`) already forbids "security engineer", "offensive
  security", "red-team" in the body, and §5.1 extends the same discipline to the
  meta description.

**One copy-rot risk to name (Lens 1D/5E).** The hero lede is time-anchored: "Right
now I'm building out a three-node cluster and documenting how it actually behaves"
(`index.html:8-10`), and the "Lately" list is three **static** bullets baked into
the template (`index.html:23-25`) with no date and no drift guard. "Lately" and
"Right now" *assert recency*; when the cluster work is finished, this copy silently
becomes false. This is not an overclaim today, but it is the kind of statement that
rots. Recommendation: keep the list template-driven (simplicity-first — a personal
site does not need a CMS for three bullets), but phrase it so it stays true as work
completes (state what the lab *is* rather than what is *in progress this week*), or
accept that "Lately" is a manually-maintained surface and add it to the operator's
copy-review checklist (§8 Q4). Do **not** build a dashboard/activity feed for it —
`docs/plans/deferred-dashboard-notes.md:7` explicitly defers any homepage activity
section until Jeff owns the concept.

### 6.4 Regulatory alignment

Referencing `criteria.md` Lens 3:

- **3A Works without JavaScript** — ✅ §3.7A. Home is fully SSR; auto-fail rule 3
  satisfied.
- **3B Contrast / colour independence** — deferred to A1's matrix; B1 introduces
  no new small-faint pairing (§3.7E) and signals no state by hue (§3.7D).
- **3C Keyboard / focus** — ✅ native links, shell focus ring, DOM order (§3.7G).
- **3D Semantics** — one `<h1>`, labelled quick-nav; the region-label redundancy
  is the one target fix (§3.7C).
- **3E Motion** — ✅ no home-body motion (§3.5); auto-fail rule 2 (motion) N/A.
- **3F Responsive / resilient** — ✅ single column reflows; designed empty state
  (§3.6 HE-1) is the resilience property (§3.3).

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**State: implemented.** The home page ships and works.

- Route `/` → `pages::home` (`router.rs:37`). **implemented.**
- `IndexTemplate { name, posts }` + `title()`/`description()`/`section()`
  (`pages.rs:30-49`). **implemented.**
- `home()` handler with graceful post-load degradation and `truncate(3)`
  (`pages.rs:55-63`). **implemented.**
- `templates/index.html`: hero (h1 + lede + quick-nav), Lately, guarded Latest
  writing, Learn pointer (`index.html:1-59`). **implemented.**
- CSS `.hero`/`.hero-lede`/`.hero-actions` (`style.css:926-955`, `1553`);
  shared `.about-list`/`.post-*`/`.section-more`. **implemented.**
- Three render tests (`pages.rs:146-202`), including the empty-state guard.
  **implemented** — but HT-1 carries the mis-coupled `CompTIA` assertion.

### 7.2 Delta to spec

**Modified files (no new files, no migrations, no new dependencies):**

1. `src/handlers/pages.rs`
   - `description()` (`:43-45`): remove "CompTIA study"; use claim-current copy
     (§4.2 Change 2). *(Lens 1D — required.)*
   - Tests (`:146-202`): remove `assert!(html.contains("CompTIA"))` (`:158`); add
     `home_description_carries_no_retired_claims` targeting `description()`
     directly; optionally add hero-nav routing assertions. *(Lens 5C — required.)*
   - Optionally drop the `name` field in favour of a shared site-name constant.
     *(Lens 5A — optional, Q1.)*
2. `templates/index.html`
   - Resolve the region-label redundancy: switch the three content `<section
     aria-label>` to `aria-labelledby` on their `<h2>`, or drop the labels.
     *(Lens 3D — recommended.)*
   - Optionally re-phrase the time-anchored lede/Lately copy to resist rot.
     *(Lens 1D/5E — recommended, Q4.)*
3. `static/css/style.css`
   - No B1-owned change required. The home-class font-size literals (`h1` 1.75rem,
     lede 0.9rem, actions 0.875rem) are folded into **A1's** Layer-3 tokenization
     sweep (A1 T4), not B1's.

### 7.3 Estimated scope

**S (small).** The required work is: one copy edit, one test correction plus one
new test, and one template accessibility tweak. All within existing files. No new
module, no dependency, no migration, no data-model change of substance. The
optional items (drop `name`, re-phrase Lately) are also small. The largest lever —
the `--text-faint` contrast remediation and the font-size tokenization — belongs to
A1, not here.

### 7.4 Blocking dependencies

- **A1 design-system** — must land the `--text-faint`/`--text-muted` contrast
  remediation and the font-size tokenization that the home classes consume. B1's
  contrast correctness (§3.7E) is contingent on A1, but B1's own required changes
  (copy, test, aria) are **not** blocked by A1.
- **A2 site-shell** — owns the `section() -> Section` enum migration (`pages.rs:46-48`),
  the `<meta>`/`og:` contract, and the integration tests that iterate `/`. B1's
  `description()` edit must satisfy A2 Contract S-1 (50–160 chars, claim-safe) and
  A2 U-7's retired-claims check. B1's copy edit is **not** blocked by A2 landing
  first; they are compatible.
- **B4 writing** — owns `BlogPost` and `content/posts/`. A `load_recent(n)` to fix
  the "parse all, keep three" inefficiency (§4.7, Q3) is B4's to add; not required
  for B1 to ship.

**Cross-feature requests filed:**

- **To B4:** add `BlogPost::load_recent(n)` so `home()` need not parse the whole
  corpus (Q3).
- **To A2:** ensure `/` is included in the shell integration iterations (I-1/I-2/
  U-3).

---

## 8. Open Questions

- **Q1 (Lens 5A) — blocks: §4.2 Change 1.** Should `IndexTemplate.name` be dropped
  in favour of a single shared site-name constant (also used by `title()`, the
  header brand, and the footer), or kept as-is? "machinageist" currently appears as
  a literal in at least four places. Consolidating is the SoT-correct move but is
  cross-cutting with A2's shell constants — decide whether B1 does it or defers to
  A2.
- **Q2 (Lens 1D) — blocks: §4.2 Change 2 final wording.** Is there a booked exam
  voucher that would permit naming a single certification in the meta description?
  If **yes**, the description may name that one exam with its scheduled date
  (per `README.md:14-16`); if **no** (the safe default), the description drops the
  cert reference entirely as proposed. The spec assumes **no** unless told
  otherwise.
- **Q3 (Lens 5 efficiency) — blocks: nothing today; §4.7.** At what corpus size
  does "parse every post, keep three" stop being acceptable? Confirm whether to
  add `BlogPost::load_recent(3)` now (cheap, forward-looking) or defer until the
  post count grows.
- **Q4 (Lens 1D/5E) — blocks: §6.3.** How is the time-anchored home copy ("Right
  now I'm building…", the static "Lately" bullets) kept from going stale? Options:
  (a) re-phrase to timeless statements of what the lab *is*; (b) accept manual
  maintenance and add it to a copy-review checklist; (c) defer any activity-feed
  idea per `docs/plans/deferred-dashboard-notes.md`. The spec recommends (a) or (b)
  and explicitly rejects building a feed.
- **Q5 (Lens 3D) — blocks: §3.7C.** For the region-label redundancy, prefer named
  landmarks (`aria-labelledby` → three navigable regions) or plain sections
  (`aria-label` dropped)? Depends on whether the sections are meant to be landmark
  navigation targets or just visual/structural grouping.
