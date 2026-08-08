# Spec: Home

**Feature ID:** `B1` / `home`
**Parent feature:** B — Content surfaces (existing)
**Spec author agent:** spec-agent-4 (Claude Opus 5)
**Date:** 2026-08-08
**Iteration:** 1

---

## 0. Scope boundary and reading notes

Every statement about current state below was read from source at the commit
`a375a14`, on a clean tree, with `cargo test --all-targets` passing 32 tests.
Citations are `path:line`.

**B1 owns:**

| Artifact | Lines |
|---|---|
| The `/` route registration | `src/router.rs:37` |
| `HOME_POST_COUNT`, `IndexTemplate`, its three metadata methods, `home()` | `src/handlers/pages.rs:23-63` |
| The home page template | `templates/index.html` (all 59 lines) |
| Hero CSS — `.hero`, `.hero h1`, `.hero-lede`, `.hero-actions` | `static/css/style.css:922-955`, `:1553` |
| `.section-more` (the "see everything" trailing link) | `static/css/style.css:1205-1209` |
| The three home tests | `src/handlers/pages.rs:146-202` |

**B1 inherits and must not redefine:**

| Concern | Owner | What B1 assumes |
|---|---|---|
| Tokens, type scale, `--measure`, contrast policy, motion policy | `A1` | Layer-2 measurement tokens exist (`style.css:480-507`); all 23 palettes clear their thresholds (CI gate, `.github/workflows/ci.yml:26-30`, landed `0cdbbea`/`7394c53`) |
| `base.html`, `<head>`, nav, footer, vitals strip, `Section`, `title()`/`description()`/`section()` contract, `og:*` | `A2` | `description()` is rendered into `<meta name="description">` (`base.html:6`) **and** `og:description` (`base.html:8`) — the fact that makes §5.1 necessary |
| `.post-list` / `.post-item` / `.post-date` / `.post-summary` | `B4` writing | B1 **reuses** the blog-list row pattern (`blog_list.html:14-22`, `style.css:988-1022`); B1 does not fork it |
| `BlogPost`, `load_all`, `POSTS_DIR`, `PILLARS` | `B4` | `load_all` returns newest-first (`post.rs:135`) and is all-or-nothing (`post.rs:132`) |
| `SIDEBAR` and `/learn` | `B5` | `SIDEBAR` (`wiki.rs:35-98`) is the single definition of what the wiki contains |
| `/portfolio` and its one-entry invariant | `B3` | `project::all()` returns exactly one entry, test-pinned (`project.rs:98`) |

Where B1 needs something outside its territory it is filed as a **cross-feature
request** in §7.4, not specified here.

---

## 1. Purpose

### 1.1 One-sentence job

Answer, in the first screenful and without scrolling, the only question a stranger
arriving at `machinageist.dev` is actually asking — *what does this person
operate, and is any of it real* — and then hand them the one route that suits why
they came.

### 1.2 Why it matters

`/` is the site's least-forgiving surface and its most-linked one. It is the URL
on a résumé, the URL pasted into a Slack DM, and the URL a reader backs out to
after finishing an article. Three specific pains:

1. **It is the only page graded on a thirty-second budget.** Every other surface
   is read by someone who already decided to read. Criterion 4A asks what a
   skimming reviewer concludes; the home page is where that conclusion forms, so
   this spec owns the answer more than any other document in the gauntlet.
2. **Its copy is stale in two directions at once.** `pages.rs:44` says "CompTIA
   study" — the spine re-locked on 2026-08-02 to RHCSA → CCNA → Security+ with
   Network+ dropped, leaving one CompTIA exam that is not first
   (`~/mg-coreforge/bootcamp/CERT_PLAN.md:3-5`). Meanwhile `index.html:8` says
   "Right now I'm building out a three-node cluster" when
   `~/mg-coreforge/bootcamp/PROGRESS.md:11-12` records the cluster as **built and
   running, hardware verified**. One line overclaims a credential; another
   *under*claims the strongest fact on the site. Both are criterion 1D failures.
3. **Its tests do not test what their names say.** `pages.rs:158` asserts
   `html.contains("CompTIA")` inside a test called
   `home_page_shows_concrete_work_without_strategy_narration`. The string appears
   nowhere in `templates/` — verified by grep across the whole tree — so the
   assertion passes only because `base.html:6` and `:8` render
   `IndexTemplate::description()` into two meta tags. A test that reads as a claim
   about the page body is a claim about `<head>`, which means an unrelated SEO
   edit breaks it and a genuine body regression does not. That is criterion 5C's
   named example, and it lives here.

### 1.3 Success signal

**Measurable:** `cargo test --all-targets` passes on a tree where (a) every home
assertion names the surface it inspects and fails loudly if that surface moves,
(b) no home-page string names a certification, and (c) the rendered Learn count
equals `SIDEBAR`'s real entry count. Concretely: deleting the words "Proxmox
cluster" from `index.html` must fail a body test, and rewriting `description()`
for SEO must **not**.

**Observable:** a reviewer who reads only the first screenful, with JavaScript
disabled, can state without scrolling: *he runs a three-node Proxmox cluster at
home, this site runs on it, and the numbers at the bottom of the page came from
the process that served it.*

### 1.4 The thirty-second answer (criterion 4A)

This is the impression the page must produce, written as the sentence a reviewer
would say out loud after thirty seconds:

> "He runs a three-node Proxmox cluster at home, this site is his own Rust
> service running on it, and the last thing he wrote up is an outage he caused
> and recovered from."

Everything on the page either serves that sentence or is cut. The differentiation
against each competitor group in `criteria.md`'s Lens 4 table is specific:

| Field | What they show on their home page | What this page shows instead |
|---|---|---|
| Junior homelab portfolios | A rack photo, a logo wall, "passionate about tech" | A live process readout in the footer that is *read at render time* (`vitals_strip.html:6`), and a lede that tells the reader that is what it is |
| Cert-track candidates | A credential roadmap and a progress bar | No exam names at all on the front door. The visible through-line is operated systems and published failure |

The mechanism matters more than the wording. Every competitor can copy a
sentence; almost none can copy *"the readout in the footer came from the process
answering this request"* and have it be true. The home page's job is to make that
already-shipped asset **legible** — today it is a row of small faint monospace at
the very bottom that a skimming reader never decodes.

### 1.5 Arrival paths (criterion 4E)

| Reader | How they get here | What they need in the first screenful | What they do next | Design consequence |
|---|---|---|---|---|
| **Hiring manager** | A résumé link or a LinkedIn field, often on a phone, often between two other candidates | Identity, scope of the operated system, and one piece of evidence they can click | `/portfolio`, then possibly one post | The lede must state the operated system in its first clause, before any prose. `/portfolio` must be the first hero action. Nothing above the fold may require scrolling on a 360px viewport. |
| **Engineer peer** | A link to a specific post from a feed, then backing out to `/` to see who wrote it | Proof this is self-operated, not a template on a static host: the vitals, the source link, the shape of the writing | `/blog`, `/status`, or View Source | The lede names the vitals strip explicitly. The teaser must surface operations work, not essays. The page must be complete with JS off — this reader is the one most likely to have it off. |
| **Self-directed learner** | Usually **not** here at all — a search for "OSI model" lands them on `/learn/:slug` directly | If they *do* arrive, that a maintained wiki exists and roughly how much of it there is | `/learn` | The Learn section's real audience is the other two readers discovering the wiki exists. Its job is a real count, not a topic list. The learner's actual entry point is `B5`'s problem, not B1's. |

---

## 2. User Stories

> As a **hiring manager with a stack of tabs open**, I want the first two lines of
> the page to tell me what this person actually operates, so that I can decide in
> one glance whether to open `/portfolio` or close the tab.

> As an **engineer peer who arrived from a link to one post**, I want the home page
> to prove the site is self-hosted rather than assert it, so that I can tell within
> seconds that the writing comes from someone running the thing.

> As a **reader with JavaScript disabled**, I want the entire home page — hero,
> current work, the writing teaser, and the Learn pointer — to be present and
> navigable, so that my browser configuration costs me nothing.

> As a **screen reader user**, I want the three sections to be announced with the
> same names I see, the post dates to be read as dates, and the trailing arrows on
> "All writing" and "Education wiki" not to be read aloud as glyphs, so that the
> page is as scannable by ear as by eye.

> As **someone who reads the site regularly**, I want the "In progress" list to be
> work that is genuinely underway right now, so that it tells me something instead
> of decaying into an old to-do list.

> As **a reader arriving during a content outage** (a post file with a broken date,
> a permissions mistake on `content/posts`), I want the front door to still open
> and still route me onward, so that one bad file does not read as a dead site.

> As **the operator**, I want a copy edit to the meta description not to break a
> test whose name says it is about the page body, so that I can change one thing
> without discovering an unrelated coupling.

---

## 3. UX Specification

### 3.1 Screen / view inventory

One screen. No modals, sheets, popovers, or drawers — the only overlay reachable
from this page is the theme menu, which is A2's.

| Surface | Path | New / modified | Layout pattern |
|---|---|---|---|
| Home | `GET /` (`router.rs:37`) | **Modification** of `templates/index.html` | Single 900px column inside A2's shell; hero, then three `<hr>`-separated sections |

Section inventory within the page, top to bottom:

| # | Section | State today | Change |
|---|---|---|---|
| 1 | Hero — `h1`, lede, three action links | implemented (`index.html:4-16`) | Copy rewritten (§3.3); `name` field removed |
| 2 | "Lately" list | implemented (`index.html:20-27`) | Renamed **"In progress"**; all three bullets rewritten |
| 3 | "Latest writing" teaser | implemented (`index.html:30-46`), guarded by `{% if !posts.is_empty() %}` | Kept; selection changed to pillar-first; `<time>` element; arrow removed from the link's accessible name |
| 4 | "Learn" pointer | implemented (`index.html:50-58`) | Hardcoded topic list replaced with a count derived from `SIDEBAR` |

### 3.2 Interaction flows

**Primary flow — arrive and route onward (fully JS-independent).**

1. `GET /` → `pages::home` (`router.rs:37`, `pages.rs:55`).
2. The handler loads posts from `content/posts` and truncates to
   `HOME_POST_COUNT` (`pages.rs:56-57`). Failure degrades to an empty `Vec`
   (`unwrap_or_default()`, `pages.rs:56`) — the front door answers even when the
   content directory does not, and `/blog` is the route that surfaces the failure
   honestly (`blog.rs:76-79`, which propagates `SiteError`). This is deliberate and
   documented at `pages.rs:52-54`; this spec preserves it and adds the test it
   never had (§5.1 U-6).
3. Askama renders `index.html` inside `base.html`. Server-rendered HTML, no
   hydration, no client routing.
4. The reader scans the hero, then takes one of: a hero action, a teased post
   title, "All writing", "Education wiki", a header nav link, or the footer's
   `/status` link. Every one is a plain `<a href>`; a full document load follows.
5. There is no state to keep, nothing to submit, and nothing that changes on the
   page after paint.

**Branch — no posts loaded.** Step 2 yields an empty `Vec`; `index.html:30` skips
section 3 entirely, including its heading and its `<hr>`. The page does not render
a heading over nothing. Pinned today by `pages.rs:186-202`.

**Branch — JavaScript disabled.** Steps 1–5 are unchanged. B1 introduces no
JavaScript and depends on none; it does not use A2's `{% block scripts %}`.

**Branch — reduced motion.** B1 introduces no animation. The only motion touching
this page is A2/A1's chrome transitions and the `.post-item` hover transition, all
inside `@media (prefers-reduced-motion: no-preference)` (`style.css:735-748`).

### 3.3 Layout descriptions

Component hierarchy, top → bottom, with the exact copy this spec adopts. Data
source is given per component.

```
<main id="content">                       A2; 900px, padding 3.5rem 2rem 5rem (style.css:757-763)

 1  <section class="hero">                B1 (index.html:4)
      <h1>machinageist</h1>               const, not a per-request String (§4.2)
      <p class="hero-lede">               --text-muted, capped at --measure-narrow
      <nav class="hero-actions"           aria-label="Quick navigation"
           aria-label>                    → Portfolio  → Writing  → Learn
                                          arrow is CSS ::before (style.css:955)
    <hr>

 2  <section aria-labelledby="now">       B1 (was aria-label="Lately")
      <h2 id="now">In progress</h2>       renders uppercase small-caps (style.css:783-791)
      <ul class="about-list">             3 <li>, --border-subtle dividers (style.css:967-978)

    <hr>                                  inside the {% if %} guard

 3  <section aria-labelledby="writing">   B1, guarded by {% if !posts.is_empty() %}
      <h2 id="writing">Latest writing</h2>
      <ul class="post-list">              B4's pattern, reused verbatim (style.css:988-1022)
        <li class="post-item"> ×N         N = posts.len(), N ≤ HOME_POST_COUNT = 3
          <time class="post-date">        NEW element; was <span> (index.html:38)
          <a href="/blog/{slug}">         post title
          <p class="post-summary">        frontmatter summary
      <p class="section-more">            → /blog

    <hr>

 4  <section aria-labelledby="learn">     B1
      <h2 id="learn">Learn</h2>
      <p>                                 count derived from SIDEBAR
      <p class="section-more">            → /learn
```

**Adopted copy.** Every string below is justified in §6.3 against a source.

| Slot | Current | Adopted |
|---|---|---|
| `description()` (`pages.rs:44`) | "Homelab, networking, and Linux notes from machinageist — a Proxmox lab, CompTIA study, and the projects that come out of it." | **"Homelab, networking, and Linux notes from machinageist — a three-node Proxmox cluster, the site it serves, and what broke along the way."** (136 chars, inside A2's 50–160 window) |
| `<h1>` (`index.html:5`) | `{{ name }}` → "machinageist" | **"machinageist"** — unchanged text, but sourced from a const rather than a per-request `String` (§4.2) |
| Hero lede (`index.html:6-10`) | "I run a Proxmox homelab and write about the networking, Linux, and operations work that comes out of it. Right now I'm building out a three-node cluster and documenting how it actually behaves." | **"I run a three-node Proxmox cluster at home. This site is a small Rust service on it — the readout in the footer is read from the process answering this request. I write up the networking, Linux, and operations work the lab produces, including what breaks."** |
| Section 2 heading (`index.html:21`) | "Lately" | **"In progress"** |
| Section 2 bullets (`index.html:23-25`) | "Standing up a three-node Proxmox cluster on a shared managed switch." / "Internal DNS and a subnet/VLAN map of the lab." / "Hosting this site on the homelab, behind Caddy and a Cloudflare Tunnel." | **"Mapping the cluster network — node and VM inventory, internal DNS, and a subnet/VLAN plan."** / **"Studying Linux systems administration — SELinux, LVM, and storage first, since those tested weakest."** / **"Re-checking older claims on this site. Anything I can't explain from the mechanism gets rewritten or pulled."** |
| Section 4 body (`index.html:52-56`) | "I keep a public education wiki … — the OSI model, topologies, transmission media, protocols and ports, and cloud concepts." | **"A public education wiki of the networking and Linux material I'm working through — {{ learn_page_count }} pages so far, written as I go."** |
| `.section-more` links (`index.html:44`, `:57`) | `All writing &rarr;` / `Education wiki &rarr;` | `All writing<span aria-hidden="true"> &rarr;</span>` / `Education wiki<span aria-hidden="true"> &rarr;</span>` |

**Three copy rules this spec establishes**, so future edits do not have to
re-derive them:

- **R1 — the hero lede leads with the operated system.** Its first clause names
  hardware Jeff owns and runs. Not an ambition, not a role, not a credential.
- **R2 — "In progress" contains only work underway right now.** Finished work
  leaves: it becomes a `/portfolio` entry or a post. The heading *is* the state
  label, which is why it replaced "Lately" — it satisfies criterion 1B without a
  disclaimer sentence, and disclaimers are exactly what `pages.rs:159-162`
  already forbids on this page.
- **R3 — the home page names no exam.** See §6.3 for the full reasoning and §8 Q1
  for the decision Jeff may reverse.

**Empty states.**

| Condition | Appearance |
|---|---|
| `posts` empty (load failed, or `content/posts` is empty) | Section 3 and its preceding `<hr>` are absent entirely. Sections 1, 2, 4 unaffected. No heading over an empty list, no placeholder, no spinner. (`index.html:30`, pinned by `pages.rs:186-202`) |
| Fewer than 3 posts exist | Section 3 renders with however many exist. `truncate` is a cap, not a requirement (`pages.rs:57`). |
| `SIDEBAR` somehow empty | Section 4's count renders `0`. This cannot happen — `SIDEBAR` is a `const` (`wiki.rs:35`) — and the derived-count test (§5.1 U-7) makes it a compile-and-test-time fact rather than a runtime concern. |

### 3.4 Input & gestures

- **Pointer.** Eleven links in the page body at full population: 3 hero actions,
  3 post titles, "All writing", "Education wiki", plus A2's chrome. `.post-item`
  gains a `--surface` fill and a 2px inset left edge on hover
  (`style.css:730`, `:745`); `.hero-actions a` and `.section-more a` change colour
  only. No hover-only affordance carries information.
- **Touch.** All targets are inline text links. `.post-item` rows have
  `1.25rem 0` padding (`style.css:996-999`) so the title link sits in a tall row;
  the hero actions are a `flex-wrap: wrap` row with a `1.5rem` gap
  (`style.css:942-948`), which keeps them from colliding at 360px. B1 adds no
  control smaller than its text.
- **Keyboard.** Tab order in the body is DOM order: hero actions (3) → post
  titles (3) → All writing → Education wiki. No `tabindex` above 0, no traps, no
  shortcuts. B1 introduces no keyboard handler of any kind.
- **Specialised input.** N/A — the page is text and links.
- **Responsive.** Two behaviours, both inherited: `main` drops to `2.5rem 1.25rem`
  padding below 640px, and `.hero` margin drops from `4rem` to `2.5rem`
  (`style.css:1553`). `.hero-lede` is capped at a narrow measure so it never runs
  the full 900px on desktop. **Requirement:** at 360px × 200% text, the hero
  (`h1` + lede + actions) must still fit above the fold with the sticky header
  (`--header-h: 6.5rem` below 640px, `style.css:1531`) accounted for — checked in
  §5.4, because the hiring-manager path depends on it.

### 3.5 Transitions & animation

**B1 introduces none.** Complete inventory of motion touching this page, all of it
inherited and all of it already guarded:

| Motion | Source | Guarded |
|---|---|---|
| `a` colour transition (hero actions, section-more, post titles) | `style.css:742` | ✅ inside `prefers-reduced-motion: no-preference` (`:735`) |
| `.post-item` background + shadow on hover | `style.css:745` | ✅ same block |
| Chrome/theme cross-fade, brand cursor blink | A2 | ✅ same block |

**Reduced-motion alternative is absence**, per A1's rule. Nothing on this page
communicates state through motion, so removing all of it costs no information. No
scroll-triggered reveal, no counter animation, no parallax — a "live" number that
animates on arrival would be exactly the dashboard cosplay criterion 2E forbids,
and the vitals strip deliberately does not do it (`vitals_strip.html:1-5`: "A
status stamp, not a live feed: no polling, no JavaScript").

### 3.6 Error states

| ID | Trigger | Presentation | Why that presentation | Recovery | Data loss |
|---|---|---|---|---|---|
| **E-1** | `content/posts` unreadable (permissions, missing directory) | Section 3 silently absent; rest of page normal | The reader came for orientation, not for a post. An error banner about a teaser would make a working page look broken, and `/blog` reports the same failure honestly with a themed 500 (`blog.rs:76-79`). Presentation is *omission*, deliberately. | Header nav, hero actions, `/learn` | No |
| **E-2** | **One** post has malformed frontmatter or an unparseable date | Identical to E-1: **the whole teaser disappears**, not just the bad post | Not a design choice — a consequence. `load_all` uses `collect::<Result<Vec<_>, _>>()?` (`post.rs:132`), so one bad file fails the entire load. The degradation is all-or-nothing and coarser than it looks. Named here rather than hidden; the fix belongs to `B4` (§7.4 CFR-3). | Same as E-1 | No |
| **E-3** | A teased post's file is deleted between render and click | 404 from `blog::post` → A2's themed 404 with working nav | The link was valid when served; a stale link is a navigation event | Header nav | No |
| **E-4** | `index.html` fails to render | `home()` returns `impl IntoResponse`, not `Result<_, SiteError>` (`pages.rs:55`), so this does **not** route through `errors.rs`'s themed 500 — it surfaces as askama_axum's own error response | Stated, not hidden. Askama validates field and method references at compile time (`pages.rs:10-12`), so the realistic trigger set is empty; converting `home()` to `Result` to buy a themed page for an impossible case would be speculative complexity | Browser back | No |
| **E-5** | The Learn count drifts from `SIDEBAR` | **Cannot occur** — the count is derived, not written (§4.2), and U-7 pins the derivation | — | — | No |

**No toast, banner, or inline alert exists on this page**, consistent with A1
§3.6: a transient message class would require JavaScript to appear and to dismiss,
which puts it behind the no-JS floor.

### 3.7 Accessibility

**Headings.** Exactly one `<h1>` (`index.html:5`), then three `<h2>`s at the same
level. No level is skipped. Note the site's deliberate inversion — `h2` renders
*smaller* than `h3` outside article content and carries hierarchy through
uppercase, letter-spacing, and weight (`style.css:782-791`) — which is A1's
documented decision and is legible without colour perception.

**Landmarks.** The page contributes one `navigation` landmark of its own
(`.hero-actions`, named "Quick navigation", `index.html:11`) and three `region`s.

**Change: `aria-label` → `aria-labelledby`.** Today each section carries an
`aria-label` whose text duplicates its visible `<h2>` — `aria-label="Lately"` sitting over
`<h2>Lately</h2>` (`index.html:20-21`, `:33-34`, `:50-51`). Two problems: the
label can silently drift from the heading (two strings, one meaning — a criterion
5A duplication), and a screen reader announces the name twice. Fix: give each
`<h2>` an `id` and point `aria-labelledby` at it. The visible heading becomes the
single source of the accessible name.

*Consequence to handle in the same change:* heading `id`s make those headings
anchor targets, and the header is `position: sticky` (`style.css:569-575`). The
`scroll-margin-top` rule currently applies only to `#content`, with a comment
saying so explicitly — *"Headings carry no ids today, so nothing else on the page
is an anchor target yet"* (`style.css:1156-1161`). That comment stops being true,
so the rule must extend to `[id]` targets or a deep link to `#writing` lands under
the nav. This is the kind of coupling criterion 5C exists for, so it is specified
rather than discovered later.

**Per-element AT contract.**

| Element | Role | Accessible name | Notes |
|---|---|---|---|
| `<h1>` | heading level 1 | "machinageist" | Duplicates the brand link text (`base.html:21`). Accepted: this is the site's front door and the site's name, the minimal-personal-site convention. §8 Q2 records the alternative. |
| `.hero-actions` | navigation | "Quick navigation" | Correct today (`index.html:11`) |
| `.hero-actions a` | link | "Portfolio" / "Writing" / "Learn" | The `→` is a CSS `::before` (`style.css:955`), so it is presentational; the visible arrow is also the non-colour affordance that keeps these underline-less links distinguishable (§3.7 colour table) |
| `<time class="post-date">` | — | — | **New.** `<time datetime="2026-07-31">2026-07-31</time>`. There is currently no `<time>` element anywhere in `templates/` (verified) — every date on the site is a bare `<span>` (`index.html:38`, `blog_list.html:17`). A date is the one datum on this page with a standard machine-readable element. |
| Post title | link | The post title | See the colour table below — this link's treatment is the page's weakest a11y point |
| `.section-more a` | link | "All writing" / "Education wiki" — **without** the arrow | Today the `&rarr;` is literal text inside the anchor (`index.html:44`, `:57`), so it lands in the accessible name and is announced ("right arrow"). Wrapping it in `<span aria-hidden="true">` removes it from the name while keeping it visible. This also makes the two arrow implementations on one page consistent in *effect* (both presentational) even though the mechanisms differ. |

**Colour independence.** No state on this page is signalled by hue.

| Element | Colour | Non-colour cue | Status |
|---|---|---|---|
| Hero action links | `--accent` (`style.css:951`) | `→` prefix (`style.css:955`), inside a `<nav>` with no surrounding prose | ✅ |
| "All writing" / "Education wiki" | `--accent` (global `a`, `style.css:534-537`) | Underline retained — no rule removes it — plus the trailing arrow | ✅ |
| Post title links | `--text`, i.e. **body-text colour**, with `text-decoration: none` (`style.css:1003-1007`) | **None.** Nothing but position distinguishes the title from non-link text until hover. | ❌ **Finding H-06** |
| `.post-item` hover | `--surface` fill | 2px inset left edge (`style.css:730`, `:745`) | ✅ |
| Focus | `--accent` ring | 2px outline, 2px offset, never removed (A1) | ✅ |

H-06 is a shared defect: the rule lives in `B4`'s `.post-item a`, and the home
teaser inherits it. B1's binding requirement is stated as an invariant — *a post
title on the home page must be identifiable as a link without hover, without
colour perception, and with JavaScript disabled* — and the fix is filed to `B4`
(§7.4 CFR-2) because forking the rule for one page would violate the reuse
decision this spec is built on.

**Text scaling.** All B1 sizes must become scale tokens (§7.2): `.hero h1` is a
literal `1.75rem` (`style.css:930-933`), `.hero-lede` a literal `0.9rem` with a
literal `55ch` measure (`style.css:935-940`), `.hero-actions` a literal `0.875rem`
(`style.css:942-948`), `.section-more` a literal `0.875rem`
(`style.css:1205-1209`). These are four of the 34 literals A1's T4 enumerates, and
they are B1's to convert. `rem` already scales with the browser font setting; the
conversion is about the token contract, not about scaling.

**Focus order.** DOM order, no interruptions, no traps: hero actions → post titles
→ All writing → Education wiki. Nothing on this page can receive focus and do
nothing.

**Reduced motion.** §3.5 — the page contributes no motion.

---

## 4. Implementation Specification

### 4.1 Architecture placement

```
src/router.rs                 :37   — "/" → pages::home                  (unchanged)
src/handlers/pages.rs         :23-63 — HOME_POST_COUNT, IndexTemplate,
                                       home(), and the new home_view()
                              :128-224 — the #[cfg(test)] module, restructured (§5.1)
templates/index.html                 — copy, <time>, aria-labelledby, arrow spans
static/css/style.css          :922-955, :1205-1209, :1553 — literals → scale tokens
src/handlers/wiki.rs          :35   — SIDEBAR visibility widened to pub(crate)
src/handlers/blog.rs          :35-40 — PILLARS visibility widened to pub(crate)
```

**No new module.** B1 is one handler, one template, and one CSS block. Adding a
`src/home/` for four functions would be exactly the single-use abstraction Jeff's
conventions forbid. The one structural change is splitting `home()` into a thin
async wrapper and a testable pure builder — see §4.2.

### 4.2 Data model

```rust
// -----------------------------------------------------------------------
// Home page — index.html
// -----------------------------------------------------------------------

// The site wordmark, rendered as the home page's h1 and its <title>
// Shared with base.html's brand once crate::shell lands (A2); until then this
// const is the single definition inside this module
const SITE_NAME: &str = "machinageist";

// How many recent posts the home page teases before sending readers to /blog
const HOME_POST_COUNT: usize = 3;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    // Newest-first, pillar-anchored posts first, capped at HOME_POST_COUNT.
    // Empty renders no writing section at all — see templates/index.html
    pub posts: Vec<BlogPost>,
    // How many pages the education wiki actually holds, counted from
    // wiki::SIDEBAR at request time so the copy cannot drift from the wiki
    pub learn_page_count: usize,
}

impl IndexTemplate {
    // Supply page title to base.html <title> slot
    pub fn title(&self) -> &str {
        SITE_NAME
    }
    // Supply the site name to the hero heading
    pub fn name(&self) -> &str {
        SITE_NAME
    }
    // Rendered into <meta name="description"> and og:description by base.html.
    // This is user-visible copy and is bound by the same claim discipline as
    // the page body — see gauntlet-output/specs/B1-home.md §6.3
    pub fn description(&self) -> &str {
        "Homelab, networking, and Linux notes from machinageist — a three-node \
         Proxmox cluster, the site it serves, and what broke along the way."
    }
    pub fn section(&self) -> &str {
        "home"
    }
}
```

**Three changes and their reasons:**

1. **`name: String` is deleted.** Today `home()` allocates
   `"machinageist".to_string()` on every request (`pages.rs:60`) to render a
   compile-time constant that `title()` already returns (`pages.rs:41`). That is
   the same literal in three places — `pages.rs:41`, `pages.rs:60`, and
   `base.html:21`'s `<span class="brand-word">` — with nothing keeping them
   honest (criterion 5A), and it violates Jeff's own rule that constants are
   `ALL_CAPS_SNAKE_CASE` and never inlined. `SITE_NAME` collapses the first two;
   the third is A2's, filed as CFR-1.
2. **`learn_page_count: usize` is added.** Section 4's copy currently hardcodes
   five topic names (`index.html:54-55`) that were accurate when written and are
   now incomplete — `SIDEBAR` has grown to include IPv4 addressing and a whole
   Linux Foundations section (`wiki.rs:79-97`) that the copy does not mention.
   This is silent drift with no guard (criterion 5B). Deriving a count from the
   same `const` `/learn` renders from makes drift structurally impossible and
   replaces a decaying list with a real number, which is also better 4A signal.
3. **Selection becomes pillar-first** (below).

**Handler:**

```rust
// Render home page with the most recent posts and the live wiki page count
// Post loading degrades to an empty list rather than propagating SiteError — the
// front door should still answer if content/posts is unreadable, and /blog is
// the route that surfaces that failure honestly
pub async fn home() -> impl IntoResponse {
    home_view(Path::new(POSTS_DIR))
}

// Build the home page from a posts directory — split out from home() so the
// degraded path (unreadable directory) is reachable from a test
fn home_view(posts_dir: &Path) -> IndexTemplate {
    let mut posts = BlogPost::load_all(posts_dir).unwrap_or_default();

    // Posts carrying a portfolio pillar lead; undated essays keep their place
    // behind them. Both halves stay newest-first — sort_by_key is stable.
    posts.sort_by_key(|post| post.category.is_none());
    posts.truncate(HOME_POST_COUNT);

    IndexTemplate {
        posts,
        learn_page_count: wiki::learn_page_count(),
    }
}
```

**Import delta in `pages.rs`:** add `use crate::handlers::wiki;` and change
`use std::path::PathBuf;` (`pages.rs:21`) to `use std::path::Path;` — `PathBuf`
becomes unused once `home()` stops constructing one per request, and clippy's
`-D warnings` (`ci.yml:35-36`) will say so. `wiki` is already a sibling module in
`crate::handlers` (`router.rs:18`), so the dependency direction is flat, not
circular: `pages` reads from `blog` (`pages.rs:16`) and now from `wiki`; neither
reads from `pages`.

**Why pillar-first, empirically.** `content/posts/` holds four files. Strict date
order (`post.rs:135`) puts them in this sequence:

| # | Post | Date | `category` |
|---|---|---|---|
| 1 | Moving My Homelab Management Network First | 2026-07-31 | `Networking` |
| 2 | Solarpunk Is an Operations Question | 2026-07-12 | **none** |
| 3 | Security Headers on machinageist.dev | 2026-07-09 | `Security` |
| 4 | How machinageist.dev Is Hosted | 2026-07-08 | `Linux / SysAdmin` |

So the middle of the site's three most valuable slots currently goes to a
philosophical essay with no command output, no evidence, and no pillar — while a
post that traces the full request path with real `dig` and `curl` output falls off
the page. `category` is not a new signal invented for this: it already exists in
frontmatter (`post.rs:45-46`), already drives the `/blog` grouping against
`PILLARS` (`blog.rs:35-40`), and already sorts unpillared posts into a trailing
"Other writing" group (`blog.rs:43`). Making the home teaser agree with that
existing judgement is consistency, not curation.

**This is not hiding anything.** Nothing is removed from `/blog`; the essay keeps
its place there under "Other writing". The stable partition also means the change
is a one-line revert if it reads wrong.

**Wiki count accessor** (`src/handlers/wiki.rs`, B5's file, one function):

```rust
// Count the real education pages behind the sidebar, excluding the overview
// entry. The home page renders this so its copy cannot drift from SIDEBAR.
pub(crate) fn learn_page_count() -> usize {
    SIDEBAR
        .iter()
        .flat_map(|section| section.entries)
        .filter(|entry| entry.slug != OVERVIEW_SLUG)
        .count()
}
```

**No database, no migration, no persistence.** The site has none.

### 4.3 API contracts

| Contract | Signature | Notes |
|---|---|---|
| Route | `GET /` → `200 text/html` (`router.rs:37`) | No parameters, no query string, no auth, no pagination. Rate limiting is uniform across all routes (`router.rs:72-75`, A3's) |
| `pages::home` | `async fn home() -> impl IntoResponse` | Signature unchanged. Deliberately not `Result<_, SiteError>` — see §3.6 E-4 |
| `pages::home_view` | `fn home_view(posts_dir: &Path) -> IndexTemplate` | **New**, private. Exists so the degraded path is testable without touching the filesystem layout the handler hardcodes |
| `wiki::learn_page_count` | `pub(crate) fn learn_page_count() -> usize` | **New**. `pub(crate)`, matching the precedent of `blog::POSTS_DIR` (`blog.rs:29`), which is `pub(crate)` for exactly this reason |
| Metadata contract | `title()`, `description()`, `section()` | A2's; Askama enforces it at compile time (`pages.rs:10-12`). B1 changes only `description()`'s text and `title()`'s source |

**Error cases:** none at the HTTP layer. `/` returns 200 or, in the impossible
render-failure case, askama_axum's 500 (§3.6 E-4). No 4xx path exists —
there is nothing to request wrongly.

### 4.4 State management

| State | Owner | Lifetime | Sync |
|---|---|---|---|
| `posts` | `IndexTemplate`, built per request | One render | Server only. Never cached, never memoised — the filesystem is the source and it is read on every request |
| `learn_page_count` | Derived from `wiki::SIDEBAR`, a `const` | Compile-time value, counted per request | Server only |
| Theme | A2 / `localStorage` | Browser | Client only; B1 never touches it |

**No new state container, no cache, no store.** Re-reading four Markdown files per
request is measured in microseconds on this hardware, and a cache would introduce
an invalidation problem where none exists — the operator edits files on the box
and expects the next request to show them.

**Offline / draft persistence:** N/A — nothing is authored in the browser.

### 4.5 Dependencies

- **New packages:** none. `askama 0.12`, `axum 0.7`, `chrono`, `gray_matter`,
  `pulldown-cmark` are all already present.
- **New assets:** none. B1 requests no image, no font, no script. The home page's
  only subresources are A2's shared `style.css`, `theme-init.js`, `main.js`, and
  `favicon.svg`.
- **Infrastructure:** none.
- **New content:** none required. The three "In progress" bullets are sourced from
  existing records (§6.3), not from work that must first be done.

### 4.6 Platform-specific considerations

- **`<time datetime="…">`** — universally supported, and inert where it is not: an
  unknown element still renders its text content. No fallback needed.
- **`aria-labelledby`** — WAI-ARIA 1.0, supported by every screen reader in the
  test matrix. Where unsupported, the section is simply an unnamed region, which
  is what it would be without the attribute anyway. Strictly non-regressive.
- **CSS** — B1 introduces no new CSS feature. It converts literals to
  `var(--text-*)`, which is Custom Properties Level 1 and is already the site's
  baseline (`style.css:480-507`).
- **No JavaScript**, therefore no browser-JS floor, no transpilation question, and
  no interaction with A2's 150-line JS ceiling.
- **Feature flags / rollout:** N/A — one binary, one deploy. The rollout mechanism
  is the commit sequence in §7.2.

### 4.7 Performance budget

| Dimension | Today | After | Note |
|---|---|---|---|
| Rendered HTML | Shell ≈ 11 KB + ≈ 1.6 KB of body | +≈ 0.2 KB (`<time>` attributes, three `id`s, two arrow spans) | Body copy is roughly unchanged in length |
| Per-request I/O | One `read_dir` + **four** `read_to_string` on `content/posts` (`post.rs:119-132`) | Unchanged | Grows linearly with the archive |
| Per-request CPU | **Four full Markdown → HTML conversions**, of which zero are used | Unchanged, and flagged | See below |
| Allocations | One needless `String::from("machinageist")` per request (`pages.rs:60`) | Removed | Trivial, but it was pure waste |
| Client JS | 0 bytes from B1 | 0 bytes | Hard requirement |
| Network requests | 0 additional | 0 additional | No image, no font, no fetch |

**Named inefficiency, not fixed here.** `BlogPost::from_file` always builds
`content_html` (`post.rs:96-99`), including when the caller only wants list
fields. The home page therefore renders every post's full Markdown body to HTML
and then discards all of it — the teaser uses only `slug`, `title`, `date`, and
`summary` (`index.html:37-41`). The model's own doc comment says the opposite:
`content_html` is documented as *"empty in list view"* (`post.rs:61`). That
comment is false, and a false comment about performance is worse than no comment
(criterion 5E).

Two reasons B1 does not fix it: `load_all` is shared with `/blog` (`blog.rs:76-79`),
so the fix belongs to `B4`; and at four posts the cost is invisible. What B1 owns
is **naming the threshold**: once `content/posts/` passes roughly 30 files, `/`
and `/blog` are converting ~30 Markdown documents per request to render a list,
and a `load_all_meta` variant becomes worth writing. Filed as CFR-3.

---

## 5. Test Specification

All tests run under `cargo test --all-targets`, which gates CI
(`.github/workflows/ci.yml:32-42`).

### 5.1 Unit tests — `src/handlers/pages.rs::tests`

**The criterion 5C problem, stated precisely.** `IndexTemplate::render()` returns
the *whole document* — A2's `<head>`, header, 24 theme names, footer, and vitals
strip, plus B1's body. The three current tests assert `contains` against that
whole string, so:

- `assert!(html.contains("CompTIA"))` (`pages.rs:158`) is satisfied **only** by
  `<meta name="description">` and `og:description` (`base.html:6`, `:8`), because
  the string exists nowhere in `templates/`. A test named
  `home_page_shows_concrete_work_without_strategy_narration` is pinned to
  `<head>`.
- `assert!(html.contains("Proxmox"))` (`:157`) and `contains("homelab")` (`:156`)
  are satisfied by **both** the body (`index.html:7`) and the meta tag — so they
  would keep passing if the body copy were deleted. They are not testing what
  their names say either; they are just accidentally right.
- Every negative assertion (`:160-166`) is silently a guard over the **entire
  shell**, including all 24 theme labels. That breadth is desirable — a forbidden
  claim anywhere in the served bytes is a forbidden claim — but it is currently
  accidental rather than stated.

**Resolution: every assertion names the surface it inspects, and the surfaces are
extracted rather than conflated.** One helper, three test groups:

```rust
// Slice out the page body so a test about the home page's copy cannot be
// satisfied by chrome. Panics loudly if base.html renames the element — that
// panic is the drift guard, and it must never degrade to whole-document matching
fn body_of(html: &str) -> &str {
    let open = html.find("<main id=\"content\">").expect("base.html renders <main id=\"content\">");
    let close = html.find("</main>").expect("base.html closes <main>");
    &html[open..close]
}
```

| # | Name | Surface inspected | Assertion | Replaces / covers |
|---|---|---|---|---|
| **U-1** | `home_body_names_the_system_it_runs_on` | `body_of(&html)` | Contains `"Proxmox"`, `"cluster"`, `"In progress"`, `"Learn"` | Fixes `pages.rs:156-158` — body claims tested against the body |
| **U-2** | `home_metadata_stays_within_the_meta_description_window` | `IndexTemplate::description()`, the `&str` itself | `50 <= len <= 160`; non-empty; no leading/trailing whitespace | The metadata test that `:158` was pretending to be. Tests the accessor, not the rendered tag — the *tag* is A2's I-7 |
| **U-3** | `home_never_serves_a_retired_or_forbidden_claim` | **The whole rendered document, deliberately and stated in the name** | Contains none of: `"CompTIA"`, `"Network+"`, `"Security+"`, `"RHCSA"`, `"CCNA"`, `"certified"`, `"in training"`, `"evidence-first"`, `"infrastructure-support"`, `"security engineer"`, `"offensive security"`, `"red-team"`, `"pentest"`, `"production"`, `"high availability"`, `"enterprise"`, `"SRE"` | **Strengthens** `pages.rs:159-166`. Every existing negative assertion is kept and the list is extended with the 1D and 1E terms. Criterion 1F: no guard is weakened |
| **U-4** | `home_teaser_leads_with_pillar_anchored_posts` | `home_view` output | Given a mixed `Vec` (two with `category: Some(..)`, one with `None`, newest being the `None`), the pillared posts occupy the leading slots and date order holds within each half | The selection rule in §4.2; regression guard on `sort_by_key` stability |
| **U-5** | `home_teaser_caps_at_three_and_links_to_the_full_list` | `body_of(&html)` | With four posts: exactly three `post-item` rows; contains `"/blog/"` for each; contains `"All writing"` and `href="/blog"` | Keeps `pages.rs:169-184`, moved onto the body |
| **U-6** | `home_still_answers_when_the_posts_directory_is_unreadable` | `home_view(Path::new("content/definitely-not-here"))` | Returns a template with `posts.is_empty()`; rendering it succeeds; body contains `"In progress"` and `"/learn"` and does **not** contain `"Latest writing"` | **New.** Turns the comment at `pages.rs:52-54` into a tested behaviour. Covers §3.6 E-1 and E-2 |
| **U-7** | `home_learn_count_matches_the_real_sidebar` | `IndexTemplate.learn_page_count` vs `wiki::SIDEBAR` | Equals the count of `SIDEBAR` entries excluding `OVERVIEW_SLUG`, and the rendered body contains that number | **New.** The 5B drift guard for §4.2 change 2 |
| **U-8** | `home_dates_are_machine_readable` | `body_of(&html)` | Every `post-date` is a `<time datetime="YYYY-MM-DD">` and the attribute value equals the visible text | **New.** §3.7 |
| **U-9** | `home_section_names_come_from_their_headings` | `body_of(&html)` | Every `aria-labelledby="X"` has a matching `id="X"` in the same body, and no `<section>` in the body carries a literal `aria-label` | **New.** Pins the §3.7 change and prevents a relapse to the duplicated-string form |

**Why U-3 keeps whole-document scope.** Criterion 1F forbids weakening an
anti-overclaim guard. Narrowing the negative assertions to the body would let a
forbidden claim ship inside a meta tag — precisely the hole that produced the 5C
defect in the first place. So the *positive* claims narrow to the body and the
*negative* guards widen to the document, and both facts are now in the test names
instead of being implicit.

**Ordering trap.** U-3 fails the moment it lands, because `description()` still
contains "CompTIA" until the copy change ships. U-3 and the copy rewrite must be
in the **same commit**. The same trap applies to A2's U-7, which B1's copy fix is
a precondition for (§7.4).

### 5.2 Integration tests

Router-level, via `tower::ServiceExt::oneshot` — the pattern already used at
`status.rs:70-90` and `errors.rs:171-182`. Today **no test requests `/` and checks
its status code**; the only integration touch is
`status.rs:113-123`, which asserts the string `"vitals-strip"` on `/` and `/blog`.

| # | Name | Assertion |
|---|---|---|
| **I-1** | `home_route_answers_200_with_the_full_shell` | `GET /` → `200`; body contains the skip link, `<main id="content"`, and `vitals-strip` |
| **I-2** | `home_body_needs_no_javascript` | `GET /`; strip every `<script …></script>` from the response; the remainder still contains all three hero action hrefs, `href="/blog"`, `href="/learn"`, and every teased post href. **The machine-checkable form of the no-JS floor for this page** |
| **I-3** | `every_link_the_home_page_offers_resolves` | Extract every `href` from `body_of(&body)`; request each through the same router; each returns `200` (or `3xx` for the legacy `/wiki` prefix). Guards against a teaser linking to a post that no longer exists, and against a hero action outliving its route |
| **I-4** | `home_emits_no_inline_script_or_style` | No `<script>` with a body, no `on[a-z]+=` attribute, no `style=` attribute in `body_of(&body)`. Keeps A3's CSP (`security_headers.rs`) from being quietly violated by a template edit |

I-3 is the strongest of the four: it is the only test in the repo that would catch
a home page pointing at a dead route, and it costs about fifteen lines.

### 5.3 UI / E2E tests

**Absent, and deliberately not proposed.** There is no browser-automation harness
in this repo — no Playwright, no Selenium, no `package.json` — and B1 ships zero
JavaScript, so an E2E harness here would test the browser, not the feature. The
behaviours E2E would cover are covered instead by I-2 (served bytes with scripts
removed), U-8/U-9 (semantics, statically), and §5.4's manual pass.

The one scenario worth recording for the day a harness lands (it is a prerequisite
for `C3` study-tools, not for B1): with `setJavaScriptEnabled(false)` and a 360px
viewport at 200% text, the hero heading, the lede, and at least one hero action
are visible without scrolling.

### 5.4 Visual / manual verification

`cargo run`, then `/` in each configuration. Tiered so the checklist is actually
runnable, using A1's six-theme Tier 1 set.

**Tier 1 — every change to this page.**

| Configuration | What to look for |
|---|---|
| Lunarcore, Solarcore, Paper, Cloud, Solarized, CRT | `h1` in `--accent` reads cleanly; the `→` prefixes render in the theme's font (they are text, not icons — Teletext and Amber are the ones to distrust); `.post-item` hover fill is visible; `<hr>` rules do not disappear |
| **JavaScript disabled** | Every section present; every link works; no theme control (A2's fix); OS light preference honoured |
| `prefers-reduced-motion: reduce` | No transition on hover, no cursor blink, page fully visible at t=0 |
| **360px × 200% text** | Hero fits above the fold under the 6.5rem sticky header (`style.css:1531`); hero actions wrap rather than clip; the lede does not overflow — **this is the hiring-manager path and it is the one that fails first** |
| 320px, 768px, 1440px, 2560px | Column stays centred at 900px; `.hero-lede` stays at its narrow measure and does not stretch |
| Browser default font 24px | Everything scales; nothing clips; the `In progress` list dividers stay aligned |
| **Empty state** — move `content/posts` aside | Writing section *and* its `<hr>` both gone; no stray rule, no gap, no dangling heading |
| **One malformed post** — corrupt a `date:` field | Same as empty state (E-2), and `/blog` returns the themed 500. Confirms the two routes disagree *on purpose* |
| Screen reader (Orca / VoiceOver) | Heading list reads: machinageist / In progress / Latest writing / Learn. Regions announce once each. Dates read as dates. "All writing" and "Education wiki" read **without** "right arrow" |
| Print preview | Once A1's print block lands: the hero and all three sections print; the arrows do not become artefacts |

**Tier 2 — token-contract changes only.** All 23 themes on `/`, using the same
checks as Tier 1 row 1.

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement.**

The home page reads four Markdown files from a directory in the repository and
counts entries in a `const`. It accepts no input — no query string, no form, no
path parameter, no header it reads. It stores nothing, sets no cookie, and writes
no client storage. There is no user, no session, no auth, and no PII anywhere in
the request path. The only data leaving the process that B1 authored is the copy
in this document.

The page inherits A2's vitals strip, which publishes uptime, a request count, and
RSS — a deliberate, recorded disclosure judgement documented in A2 §6.1 and not
re-litigated here. B1's only relationship to it is the lede sentence that tells the
reader what it is.

### 6.2 Asset provenance

- [x] **No third-party assets.**

B1 introduces no image, no font file, no icon set, no data file, and no library.
The only non-alphabetic characters it renders are `→` (U+2192), `—` (U+2014),
`’` (U+2019), and `·` — Unicode code points drawn by the system font, not
copyrightable as characters and not shipped as files. The four teased post titles
and summaries come from `content/posts/*.md`, which are Jeff's own writing.

### 6.3 Language / claims audit

- [ ] Make claims not supported by evidence — **no.** Every adopted string is
      sourced below.
- [ ] Promise capabilities not yet built — **no.** Section 2 is explicitly the
      *in-progress* section and is named so; nothing in it is written as done.
- [ ] Use language restricted by domain regulations — **no.**

**Source table for every adopted claim** (§3.3):

| Claim | Source | Verdict |
|---|---|---|
| "a three-node Proxmox cluster" | `~/mg-coreforge/bootcamp/PROGRESS.md:11-12` — *"Three-node Proxmox cluster built and running; hardware verified (3× M720q…), managed switch and off-cluster Pi DNS in place. (Done)"* | ✅ Defensible. Note this **raises** the current copy, which understates a shipped fact |
| "I run … at home" | `PROGRESS.md:9` — *"self-hosted on my own three-node Proxmox cluster — no cloud host, no managed platform"* | ✅ "Run", not "administer". Deliberately avoids the HA/cluster-administration framing `PUBLIC_FACE.md:319` forbids |
| "This site is a small Rust service on it" | `src/models/project.rs:78-85` (the one portfolio entry), `README.md:11-22`, and the running process | ✅ Verifiable by clicking Source |
| "the readout in the footer is read from the process answering this request" | `templates/vitals_strip.html:6` calls `crate::state::Status::current()` at render time; `vitals_strip.html:1-5` documents it as *"A status stamp, not a live feed: no polling, no JavaScript"* | ✅ **Measured, not asserted** — the strongest class of claim available on the site |
| "including what breaks" | `content/posts/management-layer-first-network-migration.md` — an outage worked end to end, the in-repo model criterion 4B names | ✅ |
| "Mapping the cluster network — node and VM inventory, internal DNS, and a subnet/VLAN plan." | `PROGRESS.md:29-30` — *"Project 1 — cluster network + infrastructure foundation. Topology diagrams, node/VM inventory, internal DNS, quorum and migration evidence"*, listed under **In progress** | ✅ Correct section, correct tense |
| "Studying Linux systems administration — SELinux, LVM, and storage first, since those tested weakest." | `PROGRESS.md:23-27` — *"Working SELinux and LVM first, since those were weakest on the 2026-07-25 diagnostic"* | ✅ Names the domain, not the exam (R3). The "tested weakest" clause is a deliberate 4B move: it admits a measurement and a weakness, which no competitor home page does |
| "Re-checking older claims on this site. Anything I can't explain from the mechanism gets rewritten or pulled." | `PROGRESS.md:31-35`, `DEBT_REGISTER.md:71-85` (D-02, four published claims, **Open**) | ✅ True and current. Deliberately does **not** quantify ("four") or name them — see §8 Q3 |
| "{{ learn_page_count }} pages so far" | Counted from `wiki::SIDEBAR` (`wiki.rs:35-98`) at render time | ✅ Cannot be wrong; U-7 pins it |

**Claims removed, and why (criterion 1D):**

| Removed | Where | Why |
|---|---|---|
| `"CompTIA study"` | `pages.rs:44` | The spine re-locked 2026-08-02 to **RHCSA → CCNA → Security+**, Network+ dropped, RHCSA the only pre-employment exam (`CERT_PLAN.md:3-5`, `:80-81`). One CompTIA exam remains and it is third. "CompTIA study" as the site's headline credential claim was true once and is now misleading, which criterion 1D scores 0 regardless of former accuracy |
| `"Right now I'm building out a three-node cluster"` | `index.html:8` | The cluster is built and running (`PROGRESS.md:11-12`). A stale *under*claim is still stale copy, and it happens to discard the single best fact the page has |
| The five-topic Learn list | `index.html:54-55` | Incomplete since `linux-abstraction-layers` and `ipv4-addressing` landed (`wiki.rs:79-97`). Replaced with a derived count that cannot go stale |

**Why the home page names no exam at all (R3).** Three independent reasons:

1. **The auto-fail rule is absolute.** `criteria.md` rule 1 fails any spec that
   *"introduces a certification claim without a booked exam voucher."*
   `CERT_PLAN.md:86` records that no voucher was purchased, and `PROGRESS.md:23-25`
   states RHCSA is *"Not earned, no date booked."* This spec does not test the
   edge of an absolute rule on the site's most-linked page.
2. **`PUBLIC_FACE.md` permits it only with the status attached, and the status
   does not fit here.** The 2026-08-03 loosening
   (`~/mg-coreforge/bootcamp/career/PUBLIC_FACE.md:15-23`) makes RHCSA nameable
   *"provided the copy also says it is not earned and no date is booked."* That is
   a full clause of hedging. In a hero lede it reads as apology, and this page's
   own tests already forbid defensive meta-copy (`pages.rs:159-162`). There is
   room for the honest version on `/about`, which is `B2`'s to write.
3. **It is the wrong thirty seconds.** A cert in progress is the *competitor set's*
   signal, not the differentiator (`criteria.md` Lens 4 table: cert-track
   candidates' gap is *"course completion substitutes for operated systems"*).
   Spending the front door's scarcest asset on the one thing the field already has
   is a strategic error even where it is permitted.

The tension between rule 1 and the 2026-08-03 loosening is real and is Jeff's to
resolve — recorded as §8 Q1, not silently decided.

**Claims the home page must never make** — the invariant list, enforced by U-3:
`CompTIA`, `Network+`, `Security+`, `RHCSA`, `CCNA`, `certified`, `in training`,
`evidence-first`, `infrastructure-support`, `security engineer`,
`offensive security`, `red-team`, `pentest`, `production`, `high availability`,
`enterprise`, `SRE`. The last four are new: `PUBLIC_FACE.md:37` forbids senior
DevOps / SRE / production framing, and `:319` specifically forbids claiming
HA/cluster administration — which is exactly the phrase a reader might reach for
when writing about a three-node cluster.

**One adjacency the home page must keep clear of.** `DEBT_REGISTER.md:71-85`
records four published claims that cannot currently be defended, of which two live
on this site: the systemd `203/EXEC` explanation
(`content/posts/hosting-machinageist-dev.md:86`) and the "without opening inbound
ports" mechanism (`README.md:131`). Neither string appears on the home page today
— verified — and the date-ordered teaser does not surface the hosting post
(it is 4th of 4). **Requirement:** if a future change adds a curated "start here"
list to this page, it may not feature a post carrying an open D-02 claim until
that claim closes. Recorded here so the constraint outlives this spec.

### 6.4 Regulatory alignment — `criteria.md` Lens 3

| Criterion | How B1 addresses it |
|---|---|
| **3A Works without JavaScript** *(auto-fail)* | B1 ships **zero** JavaScript, uses no `{% block scripts %}`, and depends on no client behaviour. Every section, every link, and the empty state are server-rendered by Askama. Machine-checked by **I-2**, which strips `<script>` elements from the served bytes and asserts every destination is still reachable. JS on this page is not an enhancement layer — it is absent |
| **3B Contrast and colour independence** | Token contrast is A1's, now CI-gated across all 23 themes (`ci.yml:26-30`). B1's contribution is the colour-independence table in §3.7: five of six states already carry a non-hue cue; the sixth (post-title links, H-06) is named as a defect with a stated invariant and filed to `B4` rather than being papered over |
| **3C Keyboard and focus** | Eleven body links, DOM focus order, no `tabindex`, no widget, no trap, no shortcut, nothing focusable that does nothing. The global `:focus-visible` ring is A1's and is never overridden by B1's CSS |
| **3D Semantics and AT** | One `h1`, three `h2`, no skipped level (§3.7). `aria-labelledby` replaces duplicated `aria-label` strings so the accessible name has one source. `<time datetime>` replaces bare `<span>` dates. Decorative arrows leave the accessible name via `aria-hidden` (markup) and `::before` (CSS). Pinned by **U-8** and **U-9** |
| **3E Motion and sensory safety** *(auto-fail)* | B1 introduces no animation, no transition, no autoplay, no flashing. The only motion touching the page is inherited and already inside `@media (prefers-reduced-motion: no-preference)` (`style.css:735-748`). Reduced-motion alternative is absence, and no information is carried by motion (§3.5) |
| **3F Responsive and resilient** | Two inherited breakpoints (§3.4); the 360px × 200% hero check is a named Tier-1 gate because it is the hiring-manager path. Resilience: the empty state is designed and tested (E-1, E-2, U-6), the all-or-nothing loader failure is documented rather than hidden, and the page renders correctly with zero posts, one post, or four |

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**Overall state: implemented, and structurally sound.** The findings below are
copy, semantics, and test-hygiene corrections to a page that works. Two recent
commits landed real improvements that this spec preserves rather than revisits:
`7ca53fd` added the writing teaser and the Learn pointer with the empty-section
guard, and `a375a14` made the header and footer sticky.

**Implemented and correct — keep, do not touch:**

- The empty-section guard `{% if !posts.is_empty() %}` (`index.html:30`), which
  omits the heading *and* its `<hr>` rather than rendering a heading over nothing,
  with a test (`pages.rs:186-202`). This is the empty-state pattern A1 §3.3 cites
  as the site-wide reference.
- The deliberate degradation asymmetry: `/` swallows the load error
  (`pages.rs:56`) while `/blog` propagates it (`blog.rs:76-79`), documented in the
  handler comment (`pages.rs:52-54`). Correct, and this spec pins it (U-6).
- `HOME_POST_COUNT` as a named constant with a comment (`pages.rs:27-28`) rather
  than an inline `3` — Jeff's convention, followed.
- Reuse of `.post-list` / `.post-item` from `blog_list.html:14-22` instead of a
  home-specific list. One pattern, one stylesheet block.
- The `<hr>`-separated section rhythm and the absence of cards. `criteria.md` 2E
  forbids card-ification of list architecture; this page never fell for it.
- `.hero-actions` as a real `<nav>` with an `aria-label` (`index.html:11`).

**Findings, in severity order:**

| ID | Severity | Finding | Evidence |
|---|---|---|---|
| **H-01** | **High (1D)** | `description()` says "CompTIA study". The spine re-locked 2026-08-02 to RHCSA → CCNA → Security+ with Network+ dropped and RHCSA the only pre-employment exam; one CompTIA exam remains and it is third. The string reaches users through `<meta name="description">` *and* `og:description`, i.e. it is what Slack, LinkedIn, and Google display | `pages.rs:44`; `CERT_PLAN.md:3-5`, `:80-81`; `base.html:6`, `:8` |
| **H-02** | **High (5C)** | `assert!(html.contains("CompTIA"))` in `home_page_shows_concrete_work_without_strategy_narration` passes **only** via the meta description — the string is absent from every file in `templates/`. The neighbouring `"Proxmox"` and `"homelab"` assertions are satisfied by body *and* meta, so they would survive deletion of the body copy. The negative assertions silently guard the entire shell | `pages.rs:146-167`; grep: zero `CompTIA` matches under `templates/` |
| **H-03** | **Medium (1D/4A)** | The hero says *"Right now I'm building out a three-node cluster"*; the cluster is recorded as built, running, and hardware-verified. The page **understates** its own strongest fact, and the "Lately" list repeats the stale framing in bullet 1 | `index.html:8`, `:23`; `PROGRESS.md:11-12` |
| **H-04** | **Medium (5B)** | The Learn section hardcodes five topic names. `SIDEBAR` now carries 12 topic pages across two sections including a Linux Foundations section the copy never mentions. Nothing fails when they diverge | `index.html:54-55`; `wiki.rs:35-98` |
| **H-05** | **Medium (4A/4B)** | Teaser selection is strict date order, so the middle of three slots currently goes to an essay with no `category`, no command output, and no evidence, while a post with real `dig`/`curl` output falls off the page. The signal that would fix it — `category` — already exists and already drives `/blog` | `post.rs:135`; `blog.rs:35-43`; the four frontmatter blocks in `content/posts/` |
| **H-06** | **Medium (3B)** | Teased post titles are `--text` coloured with `text-decoration: none` — the same colour as body text, with no underline and no marker. Nothing but position identifies them as links before hover. Shared with `/blog`; the rule is `B4`'s | `style.css:1003-1007` |
| **H-07** | **Medium (3D/5A)** | Each `<section>` carries an `aria-label` duplicating its visible `<h2>` — two strings, one meaning, free to drift, announced twice | `index.html:20-21`, `:33-34`, `:50-51` |
| **H-08** | **Low (3D)** | Post dates are bare `<span>`s. There is no `<time>` element anywhere in `templates/` | `index.html:38`; `blog_list.html:17` |
| **H-09** | **Low (3D)** | `&rarr;` is literal text inside the `.section-more` anchors, so it enters the accessible name and is announced as a glyph. Thirty lines away the same visual device is a CSS `::before` and is not | `index.html:44`, `:57`; `style.css:955` |
| **H-10** | **Low (5A)** | `"machinageist"` is written three times — `title()`, the per-request `name` field, and `base.html`'s wordmark — with no constant and no guard, against Jeff's own "never inline magic strings" convention | `pages.rs:41`, `:60`; `base.html:21` |
| **H-11** | **Low (2F/A1)** | Four `font-size` literals and one `55ch` measure literal in B1's CSS block, outside the type scale: `1.75rem`, `0.9rem`, `0.875rem` ×2, `max-width: 55ch`. These are B1's share of A1's 34-literal cleanup | `style.css:930-948`, `:1205-1209` |
| **H-12** | **Low (5E)** | `BlogPost.content_html` is documented as *"empty in list view"*; `from_file` always populates it, so `/` converts four full Markdown documents to HTML per request and discards every one | `post.rs:61` vs `post.rs:96-99` |
| **H-13** | **Low (5D)** | **No test requests `/`.** The three home tests render the template directly; the only integration touch is `status.rs:113-123` asserting the string `"vitals-strip"`. Nothing asserts `GET /` returns 200, and nothing checks that the links the page offers resolve | `pages.rs:146-202`; `status.rs:113-123` |

### 7.2 Delta to spec

**New files:** none.

**Modified files (5):**

| File | Change | Fixes |
|---|---|---|
| `src/handlers/pages.rs` | `SITE_NAME` const; drop the `name` field and add `name()`; add `learn_page_count` field; rewrite `description()`; split `home()` → `home_view(&Path)`; pillar-first sort; rewrite the test module per §5.1 | H-01, H-02, H-05, H-10, H-13 |
| `templates/index.html` | Hero lede copy; "Lately" → "In progress" with three new bullets; `aria-label` → `aria-labelledby` + heading `id`s; `<span class="post-date">` → `<time class="post-date" datetime="…">`; arrows wrapped in `aria-hidden` spans; Learn copy with the derived count | H-03, H-04, H-07, H-08, H-09 |
| `src/handlers/wiki.rs` | Add `pub(crate) fn learn_page_count()` | H-04 |
| `static/css/style.css` | `.hero h1`, `.hero-lede`, `.hero-actions`, `.section-more` literals → scale tokens; `55ch` → `var(--measure-narrow)`; extend `scroll-margin-top` from `#content` to `[id]` targets and correct the comment at `:1156-1158` that says headings carry no ids | H-11, §3.7 |
| `tests/home.rs` *(new integration file)* | I-1 … I-4 | H-13 |

**Migrations / schema changes:** none — no database.
**New dependencies:** none.

**Commit sequence** — each independently shippable and verifiable, per the
atomic-task rule in `~/.claude/CLAUDE.md`:

1. **`fix: correct the home page's certification and cluster copy`** — H-01, H-03.
   `description()`, hero lede, and the three "In progress" bullets, **together
   with** U-3's forbidden-term list, because U-3 fails on the old copy.
   *Verify:* `cargo test --all-targets`; read the rendered `<meta>` by hand.
2. **`test: make the home tests name the surface they inspect`** — H-02, H-13.
   `body_of()`, U-1 … U-3, U-5, and `tests/home.rs` I-1 … I-4.
   *Verify:* delete a hero sentence locally → U-1 fails; edit `description()`
   whitespace → nothing fails. That two-step check **is** the acceptance test for
   criterion 5C and should be run manually once.
3. **`refactor: one definition for the site name, and a testable home builder`** —
   H-10, and `home_view` + U-6.
   *Verify:* U-6; `cargo clippy --all-targets -- -D warnings`.
4. **`feat: count the education wiki instead of listing it`** — H-04, U-7.
   *Verify:* add a scratch `SIDEBAR` entry → the rendered number moves and U-7
   still passes; remove it.
5. **`feat: lead the home teaser with pillar-anchored posts`** — H-05, U-4.
   *Verify:* U-4; load `/` and confirm the three slots are the migration, security
   headers, and hosting posts.
6. **`a11y: real dates, real section names, and quiet arrows on the home page`** —
   H-07, H-08, H-09, U-8, U-9, plus the `scroll-margin-top` extension.
   *Verify:* U-8/U-9; one screen-reader pass over the heading list.
7. **`style: put the hero on the type scale`** — H-11.
   *Verify:* A1's T4 if it has landed; otherwise Tier-1 visual pass.

Steps 1 and 2 are the ones criterion 1D and 5C are graded on; 3–7 can follow
independently. H-06 and H-12 are **not** in this sequence — they are `B4`'s
(§7.4).

### 7.3 Estimated scope

**S**, at the top of S.

Justification: no new route, no new module, no new dependency, no new data
structure beyond one `usize` field, and no migration. The whole delta is one
template, one handler, one CSS block, one small function in `wiki.rs`, and a test
rewrite. Every change is local to files this feature already owns.

The volume sits in tests (nine unit + four integration, against three today),
which is the right ratio: the defect this spec is graded on is a *test* defect,
and the copy fix is worthless if nothing keeps it fixed.

It is not XS because seven of thirteen findings touch user-visible copy that must
each be traced to a source record before shipping (§6.3), and copy review is not
mechanical. It is not M because nothing is redesigned and the page's structure —
hero, in-progress list, teaser, pointer — is already right.

### 7.4 Blocking dependencies

**Blocking B1:**

| Dependency | Feature | What B1 needs | Blocking? |
|---|---|---|---|
| `crate::shell::SITE_NAME` (or equivalent) | `A2` | So the wordmark has one definition across `base.html` and `pages.rs` | **No.** B1 ships a module-local `SITE_NAME` and collapses it into A2's const when that lands. Recorded so the intermediate state is deliberate |
| `--measure-narrow`, `--text-*` scale tokens | `A1` | For H-11 | **Only step 7.** `--measure-narrow` is A1's proposal; if it has not landed, step 7 waits. Steps 1–6 do not depend on it |
| `<main id="content">` staying stable | `A2` | `body_of()` slices on it | **No** — and by design the helper panics with a named message if it moves, so the coupling fails loudly instead of silently reverting to whole-document matching |
| U-7 (retired-claim guard over every `description()`) | `A2` | A2's U-7 fails CI until B1's step 1 lands | **B1 blocks A2**, not the reverse. A2 §7.4 already records the ordering trap |

**Blocked by B1:** nothing. No other feature imports from the home page.

**Cross-feature requests B1 files:**

1. **CFR-1 → `A2`:** collapse the three copies of `"machinageist"`
   (`pages.rs:41`, `pages.rs:60`, `base.html:21`) into one shell constant. B1
   removes one copy and stages the second behind a module-local const; the
   wordmark in the brand link is A2's to point at the same definition.
2. **CFR-2 → `B4`:** `.post-item a` (`style.css:1003-1007`) renders a link in body
   text colour with no underline and no marker (H-06). B1's invariant is that a
   teased post title must be identifiable as a link without hover, without colour
   perception, and with JS off. The fix belongs in the shared rule, not a home-page
   override.
3. **CFR-3 → `B4`:** two loader properties B1 depends on and cannot change.
   (a) `load_all` is all-or-nothing (`post.rs:132`), so one malformed post empties
   the home teaser entirely and 500s `/blog` — B1 documents this as E-2 and asks
   whether per-file degradation is wanted. (b) `content_html` is always built
   (`post.rs:96-99`) despite the doc comment at `post.rs:61` claiming otherwise;
   at ~30 posts a metadata-only load path becomes worth writing. The false comment
   should be corrected regardless of whether the behaviour changes.
4. **CFR-4 → `B2` (about):** `pages.rs:81` and `pages.rs:92` carry the same
   "CompTIA study" / "working through the CompTIA stack" copy. B1 fixes only its
   own two strings and does not reach into `AboutTemplate`. `/about` is also where
   the PUBLIC_FACE-compliant, status-attached exam sentence belongs if Jeff wants
   one anywhere (§6.3 reason 2).
5. **CFR-5 → `B5` (learn):** `learn_page_count()` is added to `wiki.rs` by this
   change. If `B5` restructures `SIDEBAR`, the function and U-7 move with it.

**External gates:** none. B1 publishes no artifact, claims no credential, and
touches nothing behind the GeistScope publication gate.

---

## 8. Open Questions

- **Q1 — Should the home page name RHCSA at all?**
  *Blocks:* §3.3 copy, §6.3 R3, U-3's forbidden-term list.
  `criteria.md` auto-fail rule 1 forbids a certification claim without a booked
  voucher, and none is booked (`CERT_PLAN.md:86`). `PUBLIC_FACE.md:15-23`,
  loosened 2026-08-03, permits naming RHCSA *as intent with its real status
  attached*. These do not obviously agree. **B1's decision is no** — the
  status-attached form is a full hedging clause that does not survive a hero, the
  front door's thirty seconds are better spent on the operated system, and a spec
  should not test the edge of an absolute rule. If Jeff wants the exam named, the
  place is `/about` (`B2`), in the full form the loosening requires.

- **Q2 — Should the `<h1>` stay the wordmark?**
  *Blocks:* §3.3, §3.7.
  `<h1>machinageist</h1>` duplicates the brand link three inches above it and the
  `<title>`, spending the page's strongest heading on a string the reader has
  already seen twice. The alternative — an `h1` that states what the site is —
  is better for search results and for a screen reader's heading list, but it is
  longer, and `.hero h1` is `--accent` at 1.75rem, a size and colour tuned for one
  word. **B1 keeps the wordmark**, because it is the minimal-personal-site
  convention the whole design descends from and because a long accent-coloured
  headline would be the loudest thing on a deliberately quiet page. Recorded
  because it is a genuine trade, not an oversight.

- **Q3 — How openly should "In progress" bullet 3 admit the D-02 claim review?**
  *Blocks:* §3.3 copy.
  The adopted wording — *"Re-checking older claims on this site. Anything I can't
  explain from the mechanism gets rewritten or pulled."* — states the standard
  without quantifying the problem. The stronger version names the number ("four
  claims I couldn't explain cold"), which is what `PROGRESS.md:31-35` does
  internally and is arguably the single most differentiating sentence available to
  this page — no competitor in either group in Lens 4's table would write it.
  The weaker version drops the bullet entirely. **B1 recommends the adopted
  middle**: it earns the credibility without inviting a reviewer to go hunting for
  which four. Jeff's call, and it is a voice question as much as a claims one.

- **Q4 — Should the home page link to `/status`?**
  *Blocks:* §3.3 hero actions.
  The adopted lede tells the reader the footer readout is live but offers no link;
  `/status` is reachable only through the small faint version link in the vitals
  strip (`vitals_strip.html:18`). A fourth hero action would make the site's best
  proof-of-operation one click from the front door, and it would resolve A2's Q2
  without touching the global nav. **B1 leans no** — three hero actions map
  exactly to the three arrival paths in §1.5, a fourth dilutes that, and the lede
  already tells a curious reader where to look. Cheap to reverse either way.

- **Q5 — Is pillar-first teaser selection the right rule, or should the essay
  simply get a `category`?**
  *Blocks:* §4.2, U-4.
  The alternative to sorting is editorial: give
  `solarpunk-is-an-operations-question.md` a `category` and let strict date order
  stand. That is one frontmatter line instead of one sort line — but it also puts
  an essay into a portfolio pillar, which is a claim-shaped decision about what the
  pillars mean, and it does not generalise to the next uncategorised post.
  **B1 recommends the sort**, and notes that the two are not exclusive.

**Sub-feature needs (per dispatch rules): none.** B1 is a leaf. One adjacent
observation is recorded rather than acted on: `src/models/lab.rs:245-246` asserts
`combined.contains("Network+")` and `contains("Security+")` on lab copy, and
Network+ was dropped from the spine on 2026-08-02. That is a criterion 1D issue
for **C4 progress**, already flagged by A1 §8, and it is not B1's to touch.

---

**Verification commands for this feature** (all must pass; all run in CI per
`.github/workflows/ci.yml:26-42`):

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
python3 docs/themes/generate_themes.py --check
```

Plus the §5.4 Tier-1 manual pass, minimally: Lunarcore and Solarcore, JavaScript
on and off, `prefers-reduced-motion: reduce`, 360px at 200% text, the empty-posts
state, and one screen-reader run over the heading list.

Plus the one manual check that no command can make for you (§7.2 step 2): delete a
hero sentence and confirm a **body** test fails; then change `description()`'s
punctuation and confirm **nothing** fails. That is criterion 5C's acceptance test.

**Documents that must be updated in the same change (criterion 5E):**

- `docs/public-portfolio-structure.md` — its own banner at `:9` admits the cert
  spine it documents is two revisions stale, and `:32` still reads *"Certification
  journey — Network+ → Security+ → Linux+ → Server+ by January 2027."* This
  document is a **governing standard for Lens 1** in `criteria.md`. Shipping a copy
  fix while the document that governs the copy still says the old thing is exactly
  the drift criterion 5E exists to prevent. Correct §"Certification journey" and
  the banner to the 2026-08-02 spine in the same change.
- `IMPROVEMENT_PLAN.md:15` — describes *"the four-CompTIA-cert spine (Network+ →
  Security+ → Linux+ → Server+, targeted January 2027)"* as this repo's *"visible
  through-line."* Directly contradicted by the change. Correct or mark superseded.
- `docs/agent-context/README.md` — **does not exist** (verified), despite the
  global `~/.claude/CLAUDE.md` index pointing at it. Creating it is outside B1's
  scope, but three durable constraints established here belong in it when it is
  written: the home page names no exam (R3), "In progress" holds only work
  underway (R2), and home tests must name the surface they inspect.
- `src/models/post.rs:61` — the `content_html` doc comment is false (H-12). It is
  `B4`'s line, but it is wrong today and the correction is one word.
