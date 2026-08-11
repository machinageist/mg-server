# Spec: About

**Feature ID:** `B2` / `about`
<<<<<<< HEAD
**Parent feature:** `B` content-surfaces (root)
**Spec author agent:** spec-agent-5 (Claude Opus 5)
=======
**Parent feature:** root (Content surfaces, existing)
**Spec author agent:** Spec Gauntlet agent 5 (Claude Opus 4.8)
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d
**Date:** 2026-08-08
**Iteration:** 1

---

<<<<<<< HEAD
## 0. Reading notes, scope boundary, and the headline finding

Everything asserted below was read from source. Citations are `path:line` or
`path:line-range`. Commands run: `cargo test --all-targets` (30 unit + 2
integration tests, all passing) and `python3 docs/themes/generate_themes.py
--check` (exit 0, "contrast: all pairs clear across 23 themes").

**In scope (B2 owns):** the `/about` route (`src/router.rs:38`), `pages::about()`
and `AboutTemplate` (`src/handlers/pages.rs:69-95`), `templates/about.html`, the
about-page copy in every form it reaches a reader (body, `<title>`,
`<meta name="description">`, `og:*`), the about-page tests
(`src/handlers/pages.rs:204-223`), and the about-specific CSS block
(`static/css/style.css:957-982`).
=======
## 0. Reading notes and scope boundary

Everything asserted about current state was read from source, not docs. Citations
are `path:line`.

**In scope (B2 owns):** the `/about` page body and its copy — the bio paragraph,
the "What I work with" capability list, the "Further out" interests paragraph, the
location line — plus the `AboutTemplate` struct and `about()` handler that supply
them (`src/handlers/pages.rs:69-95`), the template (`templates/about.html`), the
page-body CSS (`static/css/style.css:957-982`), and the tests that pin about copy
(`src/handlers/pages.rs:204-223`). B2 also owns the page's `title()`,
`description()`, and `section()` values *as content* — the wiring of those into
`<head>` and the nav is A2's.
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

**Out of scope, referenced only:**

| Concern | Owner | What B2 assumes |
|---|---|---|
<<<<<<< HEAD
| Tokens, type scale, `--measure`, 23-theme contrast | `A1` | `.about-list`'s `font-size: 0.875rem` literal (`style.css:973`) and `.bio`'s `max-width: 65ch` (`style.css:963`) are A1's to normalise onto the scale; `.bio-loc` is A1 T8's orphan-class finding |
| `<head>`, nav, footer, skip link, sticky chrome, `Section` enum, `description()` guard (U-7) | `A2` | `base.html:6` and `:8` render `self.description()` into `<meta name="description">` and `og:description`; header is `position: sticky` (`style.css:566-573`) and footer likewise (`:818-827`) with the short-viewport unpin at `:829-836` |
| Home-page copy and its `CompTIA` test coupling | `B1` | `index.html:7-9` and `pages.rs:44`, `:158` are B1's strings, not B2's |
| Blog post bodies and their heading anchors | `B4` | `/blog/hosting-machinageist-dev` and `/blog/security-headers-on-machinageist-dev` are the evidence B2 links to |

### 0.1 The headline finding

Criterion 1D sent me here for `"working through the CompTIA stack"`
(`pages.rs:92`). That defect is real and §6.3 fixes it. **It is not the worst
defect on this page.**

`templates/about.html:9-14` is a four-bullet list titled "What I work with."
Read against the site's own published posts, **three of its four bullets present
planned capabilities as operated ones**, and the contradiction is one click away
on the same site:

| about.html says | The site's own post says | Verdict |
|---|---|---|
| `:10` "…backups, monitoring." | `hosting-machinageist-dev.md:103-104`: "**No automated monitoring or alerting.** If the service goes down, I find out by looking. A homelab monitoring stack is planned…" | **Contradicted** |
| `:10` "…backups…" | `hosting-machinageist-dev.md:105-106`: "**No tested backup/restore of the VM.** …one of my planned homelab projects, **not something I have validated yet**." | **Contradicted** |
| `:11` "subnetting and VLANs" | `management-layer-first-network-migration.md:15`: "The services are restored now, but **segmentation has not started**." | **Contradicted** |
| `:13` "SSH and TLS hardening, and auth-log detection" | `security-headers-on-machinageist-dev.md:96-98`: "…SSH hardening, TLS configuration, and auth-log detection **are planned next**, on owned scope only." | **Contradicted** |
| `:12` "health checks, and reports" | No published evidence. `PUBLIC_FACE.md:246-247` gates `mg-health` and `mg-netnotes` as "Later, after real checks and sample output exist" | **Unevidenced** |

This is the exact wording of auto-fail rule 1 — "presents a capability as built
when it is planned" — and of criterion 1B. It is worse than the CompTIA phrase
in three ways: it is five claims rather than one; a reviewer can falsify it
**without leaving the site**, using the artifact the site is proudest of; and it
is the single screen a hiring manager reads first.

The root cause is a genre error, not carelessness: **"What I work with" was
written as an aspiration list and is read as a capability list.** The fix is not
to add disclaimers — the existing test forbids that shape on purpose
(`pages.rs:216`, and §4.4) — it is to delete the aspirations and let one link
carry the honest edge.
=======
| `<head>`, `<meta>`, nav active-state, skip link, footer, theme control | `A2` site-shell | About supplies `title()`/`description()`/`section()`; A2 renders them and owns the `&str → Section` change and `aria-current` |
| Colour/font tokens, type-scale tokens, contrast audit, measurement layer | `A1` design-system | `--text`, `--text-muted`, `--border-subtle` exist and are AA-audited per theme; size literals in about's CSS are A1's to tokenise |
| The identical stale `CompTIA study` string on the home page (`pages.rs:44`) and its test coupling (`pages.rs:158`) | `B1` home | B2 fixes only the two about occurrences; the home one is filed as a cross-feature request (§7.4) |
| Portfolio entry list and Writing post list that about will link to | `B3` portfolio / `B4` writing | About links to their public routes (`/portfolio`, `/blog/:slug`), which are stable |

Where about needs a change inside another feature's territory it is filed as a
**cross-feature request** in §7.4, not specified here.
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

---

## 1. Purpose

### 1.1 One-sentence job

<<<<<<< HEAD
Let a reviewer who has thirty seconds and a resume tab open learn who operates
this site, what hardware and services that claim is actually backed by, and where
the evidence for each claim is — with nothing on the page that the site's own
published posts contradict.

### 1.2 Why it matters

`/about` is the highest-risk page on machinageist.dev for claim integrity, for
three structural reasons.

1. **It is the only page whose entire content is a claim.** `/blog` shows
   artifacts, `/portfolio` shows one verified entry pinned by a test
   (`project.rs:98-101`), `/status` shows measured process data
   (`vitals_strip.html`). `/about` is prose about a person, with no `curl`
   output to back it. Its truth is not self-evidencing; it has to be *made* true
   by discipline and pinned by tests.
2. **It is the page a hiring manager opens second.** The nav order is About →
   Portfolio → Writing → Learn (`base.html:24-27`), and About is the first link
   after the brand. `PUBLIC_FACE.md:50` states the job of `machinageist.dev` as
   answering "Can he explain real systems, failures, and verification?" — the
   about page currently answers a *different* question ("what does he know
   about?") and answers it optimistically.
3. **It is the page with no test coverage of its shipped copy.** See §7.1
   B2-03: no test calls `pages::about()`, no integration test requests `/about`,
   and the one about test injects a *synthetic* bio
   (`pages.rs:206-208`). That is precisely why the 1D defect and the five
   capability defects all survived a green CI.

The pain this addresses: **a reviewer who reads the about page and then reads a
blog post catches the site lying to itself.** For a portfolio whose whole
differentiator is that everything on it is defensible, that is the most expensive
failure available.

### 1.3 Success signal

**Measurable:** `cargo test --all-targets` passes with a new test that asserts
the **shipped** about copy — the same `&'static str` the handler serves —
contains none of the five capability nouns the published posts call planned, and
none of the retired certification strings. That test fails today if written
against the current copy, and cannot pass again by accident.

**Observable:** every noun in "What I work with" can be traced, in one click from
that page, to a published post containing real command output that supports it.
=======
Tell a reviewer, in one quiet screen, who Jeff is, what he actually operates, and
where the evidence for each capability lives — without asserting a title, a
credential, or a scope he has not earned.

### 1.2 Why it matters

`/about` is the page a hiring manager opens second, right after the home page and
right before deciding whether to keep reading. It is the site's identity claim in
prose, and on this site the identity claim is load-bearing in two directions:

1. **It is where overclaim is most tempting and most expensive.** An about page is
   the natural home for "aspiring DevOps engineer," "cybersecurity professional,"
   or "working toward my CompTIA certs." The site's entire thesis (Lens 1) is that
   every word is defensible in an interview, so about is precisely where a single
   loose phrase does the most damage. The page has already been through one
   correction — the Certifications section was removed on 2026-07-25
   (`templates/about.html:16-18`) — and it still carries a stale credential phrase
   in two places (§1.3, §7.1). About is therefore the feature where criterion **1D
   (copy currency)** is decided.

2. **It is the bridge from assertion to evidence.** The home page teases; the
   portfolio and writing prove. About is the connective tissue: it says "I do
   networking" and — in its target state — links that claim to the migration
   writeup that proves it. Competitor junior portfolios (Lens 4 table) list skills
   as an unlinked word cloud; this page's differentiation is that every capability
   named points at operated evidence.

The pain B2 addresses: today the page **asserts capabilities without linking to
their evidence**, and it **carries a credential phrase that the 2026-08-02 cert
re-lock made false**.

### 1.3 Success signal

**Primary (measurable):** no user-visible about copy — the bio (`pages.rs:92`),
the meta description (`pages.rs:81`), the template body — contains a retired or
unearned claim. Concretely, a new test (`§5.1 T-B2-1`) renders the *real*
`AboutTemplate` (not an injected fixture) and asserts the rendered HTML **and** the
`description()` string contain none of `CompTIA`, `Network+`, `A+`,
`offensive security`, `red-team`, `pentest`, `production-grade`, `enterprise`,
`SRE`, and name no certification exam. That test passes in
`cargo test --all-targets`.

**Secondary (observable):** a reviewer reading about can reach the evidence for
each named capability in one click — every item in "What I work with" that has a
published writeup links to it, verified by `§5.1 T-B2-3`.
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

---

## 2. User Stories

<<<<<<< HEAD
> **Hiring manager (primary — criterion 4E).** As a hiring manager who followed a
> resume link and clicked About, I want the first paragraph to tell me what this
> person operates and the list beneath it to name only things they can defend in
> an interview, so that I can decide in thirty seconds whether to read a post.

> **Hiring manager, adversarial.** As the same reader after I have read one blog
> post, I want the about page and the post to agree, so that I trust the rest of
> the site instead of discounting it.

> **Engineer peer (criterion 4E).** As an engineer skimming for signal, I want
> the about page to be specific enough to be falsifiable — "three-node Proxmox
> cluster", "Cloudflare Tunnel → Caddy → mg-server", `dig`/`curl` — and to admit
> what is not proved, so that I read the author as calibrated rather than as
> another homelab portfolio.

> **Self-directed learner (criterion 4E).** As someone who arrived from a
> `/learn` page and wondered who wrote it, I want a short honest answer and a
> route back to the material, so that I know whether to trust the notes.

> **Screen reader user.** As someone navigating by heading and landmark, I want
> `/about` to be one `h1` and two `h2`s in a real outline, the list to be a real
> list, and the one link on the page to say where it goes, so that I get the same
> structure a sighted reader gets.

> **Reader with JavaScript disabled.** As someone browsing with JS off, I want
> `/about` to be complete — it is static server-rendered prose with one internal
> link and no JS at all — so that nothing about this page depends on scripting.

> **The operator (Jeff).** As the person who has to defend every word of this
> page in an interview, I want a test that fails the build if a capability the
> posts call planned reappears in the copy, so that a future edit cannot quietly
> reintroduce the claim I just removed.
=======
> As a **hiring manager**, I want the about page to tell me what Jeff operates and
> point me straight at the proof, so that I can judge the claims instead of taking
> them on faith.

> As an **engineer peer**, I want the "What I work with" list to name real tools
> and a real request path rather than buzzwords, so that I can tell in ten seconds
> whether this person has touched the systems or only read about them.

> As a **self-directed learner** who arrived from a `/learn` page, I want about to
> tell me who wrote the material and where the hands-on writeups are, so that I can
> decide whether to trust and follow the rest of the site.

> As **Jeff (the operator)**, I want the about copy to be impossible to ship with a
> credential I have not earned or a title I cannot defend, so that a careless edit
> six months from now cannot reintroduce the exact overclaim I already removed
> once.

> As a **screen-reader user**, I want the capability list read as a clean sequence
> of term-and-description pairs with a correct heading outline, so that I get the
> same structured overview a sighted reader gets.

> As a **reader with JavaScript disabled**, I want the entire about page — bio,
> capabilities, interests, links — to render and be navigable, so that nothing
> about my identity claim depends on a script.
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

---

## 3. UX Specification

### 3.1 Screen / view inventory

| Surface | Path to reach | New / modified | Layout pattern |
|---|---|---|---|
<<<<<<< HEAD
| **About page** | `/about` — nav link `base.html:24`, route `router.rs:38` | **Modification** of `templates/about.html` (30 lines) | Single 900px column inside `main#content`; prose lede, one divider list, one prose paragraph, one location line |

No modals, sheets, popovers, drawers, or panels. No new screens. The only
overlay reachable from this page is the theme menu, which is A2's.

### 3.2 Interaction flows

**Primary flow — read the page.**

1. Reader clicks "About" in the header nav (`base.html:24`) or lands directly on
   `/about`.
2. Axum matches `router.rs:38` → `pages::about()` (`pages.rs:89-95`). The handler
   performs **no I/O**: it constructs one struct and returns it. It cannot fail,
   cannot 404, and cannot 500 — unlike `home()`, which reads `content/posts/` and
   degrades to an empty list (`pages.rs:55-63`).
3. `base.html` renders the shell; the About nav link carries `is-active` because
   `section()` returns `"about"` (`pages.rs:83-85`, compared at `base.html:24`).
4. Reader scans: name and what is operated → what that is made of → where the
   evidence is → where the ambition sits → location.
5. Reader either clicks the evidence link into `/blog/hosting-machinageist-dev`
   (§3.3, new) or returns to the nav.

**Branch — the evidence link's target is missing.** If the post slug is renamed
or the file removed, `blog::post` returns `SiteError::PostNotFound` → the themed
404 (`errors.rs`). The reader is not stranded (header nav is live on the 404, A2
Flow C), but the about page has silently become a dead end. §5.2 pins this with
an integration test rather than trusting it.

**No animation, sound, or haptic cue is introduced.** The page has none today and
gains none.

### 3.3 Layout descriptions

Component hierarchy, top → bottom, inside `main#content`:

```
<section>                                     templates/about.html:4
 ├─ <h1>About</h1>                            :5   — the page's only h1
 ├─ <p class="bio">{{ bio }}</p>              :6   — lede; --text-muted, capped measure
 ├─ <h2>What I work with</h2>                 :8   — section label
 ├─ <ul class="about-list">                   :9-14 — 4 <li>, each <strong>Label:</strong> + prose
 │    Systems · Networking · Automation · Security
 ├─ <p class="about-note">…</p>               NEW  — one sentence + the evidence link
 ├─ {# claim-boundary comment #}              :16-18 — do not remove; see §4.4
 ├─ <h2>Further out</h2>                      :20
 ├─ <p>…interests…</p>                        :21-26
 └─ <p class="bio-loc">Portland, OR.</p>      :28
```

**Data sources.** `bio` is the single template field
(`AboutTemplate`, `pages.rs:71-73`), supplied by the handler at `pages.rs:91-93`.
Every other string is literal template text. `title()`, `description()`, and
`section()` (`pages.rs:77-85`) are read by `base.html:6-10` and `:24`. There is no
model, no store, no filesystem read.

**Empty states.** N/A in the data sense — there is no collection to be empty and
no source that can fail. The one empty-state-adjacent rule this page must hold:
`bio` is never rendered as an empty `<p class="bio">`. Since `bio` comes from a
compile-time constant (§4.2), emptiness is structurally impossible, which is
strictly better than a runtime guard.

**Copy budget.** The page must stay readable in one screen at 1280×800 in the
default theme. The change in §6.3 makes it shorter, not longer: seven aspirational
nouns removed, one sentence and one link added.

### 3.4 Input & gestures

- **Pointer.** One new interactive target: the evidence link in `.about-note`.
  It is an ordinary in-flow `<a>`, so its hit area is its text box at body size —
  above the 24×24 CSS-px minimum of WCAG 2.5.8 for inline text links, and inline
  links are exempt from 2.5.5 sizing in any case.
- **Keyboard.** The link is reachable by Tab in DOM order, after the header's
  theme button and before the footer links (A2 §3.4). No new shortcut, no
  `tabindex`, no `accesskey`.
- **Touch.** Nothing hover-only carries meaning on this page.
- **Specialised input** (stylus, controller, voice, camera): **N/A — static prose
  with one link.**
- **Responsive.** Inherits the shell's two breakpoints (A2 §3.4). The page-local
  concern is that `.bio` caps at `65ch` (`style.css:963`) while A1 defines
  `--measure-narrow: 55ch` for lede copy — B2 requests the token, does not set the
  value (§7.4). At `max-height: 34rem` the footer un-pins itself
  (`style.css:829-836`), which is already the correct behaviour for a page this
  short; B2 changes nothing about the sticky chrome.

### 3.5 Transitions & animation

**None introduced.** The page has no `transition` or `animation` of its own; the
only motion visible while on it is shell chrome (theme swap, nav underline sweep,
brand cursor blink), all already inside
`@media (prefers-reduced-motion: no-preference)` (`style.css:735-748`, A2 §3.5).

**Reduced-motion alternative:** absence, inherited. Nothing on this page needs a
substitute animation because nothing on it animates.

**Navigation transition:** full document load. No view transitions, no
client-side routing.

### 3.6 Error states

| ID | Trigger | Presentation | Why that presentation | Recovery | Data loss |
|---|---|---|---|---|---|
| **A-01** | None reachable from the handler | — | `pages::about()` (`pages.rs:89-95`) performs no I/O, parses nothing, and returns an infallible struct. There is no error path to present. | — | No |
| **A-02** | The evidence-link target post is renamed or deleted | Themed 404 on the *next* page, not on `/about` | The about page cannot know its link is dead at render time without a filesystem check per request, which would trade a real cost for a hypothetical one. The correct place to catch it is CI (§5.2 I-B2-2), where it is free. | Header nav on the 404 | No |
| **A-03** | `about.html` references a field `AboutTemplate` does not have | **Compile error** — Askama validates template field references at build time (`pages.rs:10-12`) | Strongest available presentation: it never reaches a reader | Fix the code | N/A |
| **A-04** | Copy drifts out of step with the posts it depends on | **CI failure** (§5.1 U-B2-4) | Copy drift is invisible by definition. It is the failure mode that produced every defect in §0.1 and it must fail loudly rather than wait for a reviewer to notice. | Update the copy, or update the post and the test together in one commit | No |
| **A-05** | `style.css` fails to load | Unstyled but fully readable: `h1`, two `h2`, a real `<ul>`, real `<p>`, one real link | The page is semantic HTML; the stylesheet is presentation only | Reload | No |

**No toast, banner, or modal is proposed.** A2 §3.6 rules that no toast component
exists on this site because toasts require JS to appear and dismiss, which would
put a message class behind the no-JS floor. B2 inherits that ruling and needs
nothing from it.

### 3.7 Accessibility

**Heading outline.** Exactly one `h1` ("About", `about.html:5`) and two `h2`s
("What I work with" `:8`, "Further out" `:20`). No level is skipped. Per A1
§3.7E, `h2` renders *smaller* than `h3` outside `.post-content` by design
(`h2` is `--text-sm` at `style.css:784-791`, `h3` is `--text-md` at `:793`), with
hierarchy carried by case, weight, and
letter-spacing rather than size alone — so the outline is legible without colour
and without size ordering. B2 preserves this and adds no heading.

**Landmarks.** Contributed by the shell (A2 §3.7). The page adds none; a
`<section>` without an accessible name (`about.html:4`) is correctly *not* exposed
as a region, which is the desired outcome — there is only one section of content
and naming it would add noise.

**Per-element AT contract.**

| Element | Role | Accessible name | Notes |
|---|---|---|---|
| `<h1>About</h1>` | `heading` level 1 | "About" | One per page |
| `<p class="bio">` | — | — | Plain prose |
| `<ul class="about-list">` | `list` | — | Four `listitem`s; `<strong>` inside each is styled emphasis, announced as ordinary text by every mainstream screen reader — acceptable because the label is also visually a label and the meaning does not depend on the emphasis |
| `<code>dig</code>`, `<code>curl</code>`, `<code>ss</code>` | — | literal text | Announced as their own characters; no `aria-label` needed and none should be added (it would double-announce) |
| **New** evidence link | `link` | "the hosting post" **in the sentence context** | The link text must be meaningful out of context per WCAG 2.4.4/2.4.9. §6.3 specifies the text as **"what this setup still doesn't have"**, which names its destination without needing the surrounding sentence |
| `<p class="bio-loc">Portland, OR.</p>` | — | — | City only. Deliberate: commit `a2c096a` "fix: keep location copy city-only". Do not restore a street, ZIP, or employer |

**Colour independence.** The one link is `--accent` **and** underlined — the UA
default `text-decoration` is never removed for in-prose links, which A1 §3.7B
records as a shipped-but-accidental invariant to be pinned. B2 depends on it and
names the dependency (§7.4). No other state on this page is signalled at all, by
hue or otherwise.

**Focus.** The link inherits the global, never-removed
`:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px }`
(`style.css:710`). Focus order is DOM order; nothing on this page reorders
it.

**Text scaling.** `.about-list li` is `font-size: 0.875rem` (`style.css:973`) —
a literal that is not on A1's scale and renders the list *smaller than body copy*
on the page whose job is to be read. This is A1's normalisation to make
(§7.4), but B2 records the consequence: the list carrying every capability claim
is currently the smallest prose on the page, which is a scannability defect
(criterion 2D) as well as a scaling one.

**No-JS.** The page is 100% server-rendered static prose. Zero scripts, zero
JS-dependent behaviour, zero enhancement layer. There is nothing here to
degrade — which makes `/about` the cleanest possible instance of the no-JS floor
and a fine page to point a skeptical engineer-peer at with scripts blocked.
=======
| About page | `/about` (`router.rs:38`) → `pages::about` | **Modification** (exists, `templates/about.html`) | Single prose column inside the standard shell `<main>` (900px, capped at `--measure`), no sidebar, no cards |

About introduces **no** new screens, modals, popovers, or drawers. It is one
server-rendered page inside the A2 shell. The only interactive elements on it are
links (nav in the shell, plus the in-body evidence links this spec adds).

### 3.2 Interaction flows

**Primary flow — read and branch to evidence (JS-independent).**

1. Reader requests `/about`. `about()` (`pages.rs:89-95`) builds `AboutTemplate`,
   Askama renders it, `askama_axum` returns `200 OK` with the full shell.
2. Reader sees `<h1>About</h1>`, the bio paragraph, the "What I work with" list,
   the "Further out" paragraph, and the location line — top to bottom, one column.
3. **Target addition:** capability list items that have published evidence carry an
   inline link to it (e.g. Networking → `/blog/management-layer-first-network-migration`).
   Click / Enter → full page load to that writeup. No client-side routing.
4. Reader returns via browser back or the shell nav. No state is held.

There are no branches in the about page's own logic: it takes no input, has no
form, no query parameter, no conditional server state. The only conditional
rendering is the target evidence links, which are static in the template.

**Cues.** No haptics, no sound, no page-body animation. The only motion on the
page is inherited shell chrome (nav underline sweep, theme cross-fade), all of it
already gated by `prefers-reduced-motion: no-preference` in A1/A2's territory.
About itself adds **zero** animation.

### 3.3 Layout descriptions

Component hierarchy, top → bottom (`templates/about.html:4-29`):

```
<section>
  ├─ <h1>About</h1>                         one h1; the page's only top heading
  ├─ <p class="bio">{{ bio }}</p>           lede paragraph; --text-muted, capped measure
  ├─ <h2>What I work with</h2>              section label
  ├─ <ul class="about-list">               4 items, each <strong>label</strong> + description
  │    ├─ Systems      (Linux, systemd, journald, Proxmox homelab)
  │    ├─ Networking   (DNS, subnetting/VLANs, the Tunnel→Caddy→mg-server path, dig/ss/curl)
  │    ├─ Automation   (Rust/Python operations tools, health checks, reports)
  │    └─ Security      (HTTP headers, SSH/TLS hardening, auth-log detection, owned scope)
  ├─ {# comment: certifications section intentionally removed #}
  ├─ <h2>Further out</h2>                   section label
  ├─ <p>…interests, framed as aspiration…</p>
  └─ <p class="bio-loc">Portland, OR.</p>   location line
```

**Data sources.**

| Component | Source | Note |
|---|---|---|
| Bio text | `bio` field, set in `about()` (`pages.rs:91-93`) | Runtime `String`; the only field on the struct |
| `<title>` | `AboutTemplate::title()` (`pages.rs:77-79`) → `"About — machinageist"` | A2 renders into `<head>` |
| `<meta description>` | `AboutTemplate::description()` (`pages.rs:80-82`) | User-visible copy — carries the stale claim (§7.1) |
| Nav active state | `AboutTemplate::section()` (`pages.rs:83-85`) → `"about"` | Matches `base.html:24`; highlights correctly today |
| Capability list, interests, location | Hard-coded in `templates/about.html:9-28` | Static prose |

**Measure.** The prose column inherits the shell's `--measure` cap; `.bio`
additionally sets `max-width: 65ch` (`style.css:963`). That literal is an A1
measurement-layer concern (§7.4 CFR-1), not an about behaviour.

**Empty state.** About has **no dynamic collection** and therefore no empty
state — every element is static prose. This is deliberate and correct: an identity
page should never be empty. The one runtime input, `bio`, is a constant set in the
handler and can never be absent. Marked intentionally-not-applicable rather than
omitted.

### 3.4 Input & gestures

- **Pointer / touch.** Click or tap on the in-body evidence links (target
  addition) and on inherited shell chrome. No hover-only affordance carries
  meaning; links are underlined (§3.7).
- **Keyboard.** Tab reaches each in-body link in DOM order, after the shell nav
  and before the footer. Enter activates. No custom key handling — about adds no
  JavaScript, so there are no widget keys.
- **Keyboard shortcuts.** None, and none proposed. About is prose; single-key
  accelerators would trip WCAG 2.1.4 and collide with find-as-you-type.
- **Specialised input (stylus/controller/voice/camera).** N/A — a text page.
- **Responsive.** About inherits the shell's two breakpoints (800px, 640px). Its
  own content is a single flowing column that reflows naturally at any width; the
  `.about-list` items wrap within the measure. No about-specific media query
  exists or is needed. Verified: `style.css:957-982` contains no `@media` rule.

### 3.5 Transitions & animation

About contributes **no** motion. Full inventory of motion visible on `/about` is
inherited shell chrome, already specified and gated in A1 §3.5 and A2 §3.5:
theme cross-fade, nav underline sweep, brand cursor blink — all inside
`@media (prefers-reduced-motion: no-preference)`.

- **In-view state changes:** none. About has no state.
- **Navigation transitions:** none. Full document load, like every route.
- **Reduced-motion alternative:** N/A for about's own content (it has no motion);
  the inherited chrome's reduced-motion behaviour is A1/A2 territory and is
  *absence*, never a substitute animation.

### 3.6 Error states

About has no input, no I/O in its handler, and no fallible rendering path, so its
error surface is narrow.

| ID | Trigger | Presentation | Why | Recovery | Data loss |
|---|---|---|---|---|---|
| B2-E1 | `about.html` references a field that does not exist on `AboutTemplate` | **Compile error** — Askama validates field references at build time (`pages.rs:10-12`) | Strongest possible: never reaches a user | Fix the code | N/A |
| B2-E2 | Reader requests `/about/anything` or a mistyped path | Falls through to the themed 404 (`router.rs:61`, A2 E-01) | A wrong URL is a navigation event; the 404 keeps nav available | Header nav / home | No |
| B2-E3 | `style.css` fails to load | Semantic HTML remains fully readable and navigable — real `<h1>`/`<h2>`, a real list, real links, real landmarks (A1 E3) | The page is meaningful without CSS | Reload | No |

There is **no** runtime error case in `about()`: it constructs a struct from a
string literal and returns it. No file read, no parse, no unwrap that can fail.
This is stated so a verifier does not expect (and dock for a missing) toast/banner
that would have nothing to report.

**No toast/banner/modal is proposed** — consistent with the site-wide decision
(A1 §3.6) that transient JS-driven messages sit behind the no-JS floor.

### 3.7 Accessibility

**Heading outline.** One `<h1>` ("About", `about.html:5`) and two `<h2>` section
labels ("What I work with" `:8`, "Further out" `:20`). No level is skipped. This
matches A2 U-4 (exactly one `<h1>` per content template). **Invariant:** about
must keep exactly one `<h1>`.

**The capability list — semantics decision.** Today the list is a `<ul>` whose
items are `<strong>Label:</strong> description` (`about.html:9-14`). It is
semantically a set of term/description pairs. Two defensible options:

- **Option A (keep `<ul>`):** acceptable. A screen reader announces "list, 4 items"
  and reads each `<strong>` label inline with its description. The colon after the
  label carries the term/definition relationship visually and audibly.
- **Option B (upgrade to `<dl>`):** `<dl>` with `<dt>` labels and `<dd>`
  descriptions is the *precise* semantic for term/definition pairs and lets AT
  users navigate term-by-term.

**Resolved direction:** keep Option A (`<ul>`). Rationale: the `<dl>` upgrade
touches the template and the `.about-list` CSS for a marginal AT gain, it is not
required by WCAG (a `<ul>` of labelled items is conformant), and A1 §3.1 already
names this the site's "definition-style list" pattern. Changing it here would fork
that pattern for one page. The `<ul>` **is** pinned as the invariant so it cannot
silently become a `<div>` soup. (Logged as Q1 in case Jeff prefers the `<dl>`.)

**Colour independence.** About signals nothing by hue. Labels are distinguished by
weight (`.about-list strong { color: var(--text) }`, `style.css:980-982`) against
`--text-muted` body — a weight-and-colour difference, not colour alone. In-body
evidence links (target) are underlined (the UA default `text-decoration` is never
removed in prose, A1 §3.7 B), so they are distinguishable from body text without
relying on `--accent` hue. **Invariant:** in-body links stay underlined.

**Contrast.** About's text/background pairs are `--text` and `--text-muted` on
`--bg`. Both are in A1's audited USAGE matrix. Note the `.about-list li` size is
`0.875rem` ≈ 14px (`style.css:973`) — small text requiring **4.5:1**. That literal
is one of the font-size literals A1 T4 flags and A1's audit covers `--muted` at
4.5:1. B2 defers the contrast guarantee to A1 and files the literal as CFR-1.

**Focus and keyboard.** About adds only links, which are natively focusable and
sit in DOM order. The shell's global `:focus-visible` ring (`style.css:685`)
applies. No `tabindex`, no focus trap, no custom widget.

**Screen-reader labels.** About needs no ARIA of its own — its content is native
semantic HTML (`<h1>`, `<h2>`, `<ul>`, `<p>`, `<a>`). The one place to be careful:
in-body evidence links must have self-describing link text ("the network
migration writeup"), never "here" or "this post," so the link makes sense read out
of context. **Invariant:** evidence-link text names its destination.

**Text scaling.** All about sizes are `rem`-based (`.about-list li` `0.875rem`,
`.bio` inherits) and scale with the reader's browser font-size, once A1's
body-size fix lands (A1 §3.7 F). No fixed `px` in about's own CSS.

**Decorative content.** About has none — every element is meaningful. The
`.bio-loc` "Portland, OR." line is meaningful (it is a real fact a reviewer wants)
and is correctly a plain `<p>`; it must not be `aria-hidden`.
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

---

## 4. Implementation Specification

### 4.1 Architecture placement

<<<<<<< HEAD
```
src/router.rs                    :38   — GET /about → pages::about   (unchanged)
src/handlers/pages.rs            :65-95 — the About block            (modified)
                                 :204-223 — the About tests          (modified + extended)
templates/about.html                    — page body                  (modified)
static/css/style.css             :957-982 — .bio / .about-list       (one addition; values are A1's)
tests/about_page.rs                     — NEW integration test file
```

**No new module.** B2 introduces one file — an integration test — and touches no
architecture. `pages.rs` already carries home, about, and portfolio together
(`pages.rs:3`); splitting it would be an unrequested refactor.

### 4.2 Data model

The only data change is turning the shipped copy into a named constant so it can
be tested, which also brings the file into line with Jeff's Rust conventions
(`~/.claude/CLAUDE.md`: "`ALL_CAPS_SNAKE_CASE` for all constants — never inline
magic strings"). `HOME_POST_COUNT` (`pages.rs:28`) is the in-file precedent.

```rust
// -----------------------------------------------------------------------
// About page — about.html
// -----------------------------------------------------------------------

// The bio, as one constant so the shipped copy is the copy the tests read.
// Held to what the published posts can back: the cluster and the outage are in
// /blog/management-layer-first-network-migration, the request path is in
// /blog/hosting-machinageist-dev. Nothing here may name a capability those
// posts call planned -- see the tests below.
const ABOUT_BIO: &str = "I'm Jeff. I run a homelab — a three-node Proxmox cluster on hardware \
                         I own — and I write up the networking and Linux work that comes out of \
                         it, including the parts that broke. Most of what's on this site starts \
                         from hardware I operate myself.";

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    // Borrowed rather than owned — the bio is a compile-time constant, so there
    // is nothing to allocate per request
    pub bio: &'static str,
}
```

`bio` changes from `String` (`pages.rs:72`) to `&'static str`. Askama renders
either; the borrow removes one allocation per request and makes it structurally
impossible for the handler to serve a bio other than `ABOUT_BIO`. That
impossibility is the point — it is what closes B2-03 (§7.1).

```rust
// Render about page with the constant bio
pub async fn about() -> impl IntoResponse {
    AboutTemplate { bio: ABOUT_BIO }
}
```

**No database, no migration, no schema.** The site has no persistence layer.

### 4.3 API contracts

| Item | Signature | Contract |
|---|---|---|
| Route | `GET /about` → `pages::about` (`router.rs:38`) | Always `200`. No params, no query string, no body. Unchanged. |
| `AboutTemplate::title` | `fn title(&self) -> &str` | `"About — machinageist"` (`pages.rs:77-79`). Already conforms to A2 Contract S-1's `" — machinageist"` suffix rule; **unchanged**. |
| `AboutTemplate::description` | `fn description(&self) -> &str` | 50–160 chars (A2 U-6). Current is 110 chars and carries a retired claim; **corrected in §6.3** to 149 chars. |
| `AboutTemplate::section` | `fn section(&self) -> &str` → `Section` | Returns `"about"` (`pages.rs:83-85`), matched at `base.html:24`. **B2 changes nothing**; A2 owns the `&str` → `Section` type change and B2 must not pre-empt it. |

**Auth, rate limiting, pagination:** N/A. The route is public and unauthenticated
like every other; the global limiter (`router.rs:72-75`) applies uniformly and is
A3's.

### 4.4 State management

**No state.** The page has no client state, no server state, no session, no
storage, no query parameters, and no request-varying output. Every byte of the
response is a pure function of the binary — the same request produces the same
bytes until a redeploy, except for the shell's vitals strip
(`vitals_strip.html`), which is A3's.

There is one piece of *durable state* that matters and it is not machine state:
**the claim boundary encoded in `about.html:16-18`.**

```
{# Certifications section intentionally removed 2026-07-25. Do not restore a cert
   claim here until an exam voucher is actually booked, and then state only that one
   exam with its scheduled date. See mg-coreforge/PUBLIC_FACE.md. #}
```

This comment is a recorded decision, and commit `9b5b1a1` ("Remove Certifications
section assertion from about page test") is its matching test change. **It must
not be deleted, weakened, or moved.** §6.3 amends it — with a dated line, in the
same style, citing the same governing document — because the underlying policy
was itself amended on 2026-08-03. That is the repo's own convention for changing
a claim boundary and B2 follows it rather than inventing one.

**Offline / draft persistence:** N/A — nothing on this page is authored in the
browser.

### 4.5 Dependencies

- **New packages:** none.
- **New assets:** none. No images, no fonts, no icons. The page requests exactly
  the shell's CSS, favicon, and two JS files, and nothing of its own.
- **Infrastructure:** none.
- **Content dependency (new, and deliberate):** the page will link to
  `/blog/hosting-machinageist-dev`. That is a coupling between a template literal
  and a Markdown filename, and it is exactly the kind of thing criterion 5B says
  must fail loudly — hence I-B2-2 (§5.2), which requests the link target through
  the real router and asserts `200`.

### 4.6 Platform-specific considerations

- **Browser support:** the page uses `<h1>`, `<h2>`, `<p>`, `<ul>`, `<li>`,
  `<strong>`, `<code>`, `<a>`. There is no CSS or HTML feature here with a support
  floor worth naming.
- **Markdown/anchor limitation (verified):** the site parses posts with
  pulldown-cmark 0.10 (`Cargo.toml:26`, `src/models/post.rs:26`, `:97-99`) using
  `Options::all()`, which enables *explicit* `{#id}` heading attributes but does
  **not** auto-generate heading `id`s. `grep -c '{#' content/posts/*.md` returns
  `0` for all four posts. **Therefore a deep link to
  `/blog/hosting-machinageist-dev#what-is-honestly-not-here-yet` would not work**
  and must not be written. The link target is the post URL. Filed as a
  cross-feature request to `B4` (§7.4) — heading anchors would make this link
  land on the exact section, and the "What is honestly not here yet" heading
  (`hosting-machinageist-dev.md:101`) is the best argument for adding them.
- **Feature flags / rollout:** N/A — single binary, single deploy.
- **Print:** the page is prose and prints correctly under A1's proposed print
  block. No page-specific rule needed.

### 4.7 Performance budget

| Dimension | Current | After | Note |
|---|---|---|---|
| Rendered HTML | `about.html` is 30 lines of template producing ~1.4 KB of body inside the ~10 KB shell | **Smaller.** Seven aspirational nouns removed; one sentence and one anchor added | The page is already the second-smallest on the site |
| Handler allocations | One `String::from` per request (`pages.rs:93` `.to_string()`) | **Zero** — `&'static str` field (§4.2) | Trivial in absolute terms; taken because it is free and it makes the copy unforgeable |
| Filesystem I/O | None | None | Unlike `home()` (`pages.rs:56`) |
| Network requests | Shell assets only | Unchanged | No image, no font, no script of its own |
| CSS added | — | 3–5 declarations for `.about-note` (values from A1 tokens) | Within A1's ≤48 KB budget with ~6.7 KB headroom |
| Server CPU | One struct construction + Askama render | Unchanged | |
| Client storage | None | None | |
=======
About spans three artifacts, all pre-existing:

```
src/handlers/pages.rs      # AboutTemplate struct + title/description/section + about() handler (:69-95)
                           # and the about tests (:204-223)
templates/about.html       # the page body (30 lines)
static/css/style.css       # .bio / .about-list / .about-list li/strong (:957-982); .bio-loc is an unstyled hook
```

No new module. No new file. `about()` is registered at `router.rs:38`. This is a
content-and-copy feature; its "architecture" is one handler, one template, one CSS
block, and the tests that guard the copy.

### 4.2 Data model

The only type is the existing template struct. Two changes, both minimal:

```rust
// Author:      machinageist
// Date:        2026-08-08
// Description: About page — bio, capabilities, interests, location. Copy leads
//              with owned, operated capability; it names no unbooked certification
//              and asserts no title the claim discipline forbids.
// Notes:       description() is user-visible copy (rendered into <meta> by base.html)
//              and is guarded by the same claim tests as the body. section() returns
//              Section::About once A2's Section enum lands; &str until then.
#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    // Lede paragraph — set in about(); the only runtime field
    pub bio: String,
}
```

**Copy changes (the heart of B2 — criterion 1D).** Two strings carry the stale
`CompTIA` claim and both change:

| Location | Current (stale) | Target (capability-led, cert-agnostic) |
|---|---|---|
| `pages.rs:81` `description()` | `"About Jeff Cincoski — a Proxmox homelab, networking and Linux operations, small automation, and CompTIA study."` | `"About Jeff Cincoski — a Proxmox homelab, networking and Linux operations, small automation, and defensive security on owned scope."` (128 chars, in the 50–160 window A2 S-1 requires) |
| `pages.rs:92` `bio` | `"…and I'm working through the CompTIA stack. Most of what's here comes out of hardware I own and operate."` | `"…and I'm steadily working through Linux and networking fundamentals. Most of what's here comes out of hardware I own and operate."` |

These are **proposed** strings — the exact wording is Jeff's (copy is his to own;
logged Q2). What is **not** negotiable is the constraint they satisfy: no cert
vendor or exam name until a voucher is booked (§6.3), capability-led framing
(1E), and no retired claim (1D).

**Why not name the live spine (RHCSA → CCNA → Security+)?** Criterion 1D says copy
must match "current reality — including the live certification spine." The current
reality has two parts that a naive reading collides: the spine *is* RHCSA → CCNA →
Security+ (re-locked 2026-08-02), **and** the site's own claim discipline forbids
publishing any cert name before its voucher is booked
(`templates/about.html:16-18`, `README.md:14-16`,
`docs/public-portfolio-structure.md:9-11`, and **auto-fail rule 1**). Naming
RHCSA/CCNA/Security+ in about copy today would be an **auto-fail** (a certification
claim without a booked voucher). The resolution that satisfies *both* halves of
"current reality": describe the **capability being studied** (Linux
administration, networking) without naming an exam; keep the spine internal to
this spec and to `mg-coreforge/PUBLIC_FACE.md`; and, the moment a voucher is
booked, replace the phrase with that **one** exam and its scheduled date, per the
standing instruction at `about.html:16-18`. That path keeps 1D (not stale), 1E
(capability-led), and rule 1 (no unearned cert) simultaneously true.

**Target evidence links (criterion 4B/4E).** The capability list items gain inline
links to the writeups that evidence them, added in `templates/about.html`:

| Capability | Evidence link | Present in repo? |
|---|---|---|
| Networking / request path | `/blog/hosting-machinageist-dev` (Tunnel → Caddy → mg-server, real `dig`/`curl`) | Yes — `content/posts/hosting-machinageist-dev.md`, `README.md:133` |
| Networking / migration | `/blog/…network-migration…` (an outage worked end to end — the Lens 4B in-repo model) | **Verify slug** at implementation time (Q3) |
| Security | `/blog/security-headers-on-machinageist-dev` | Yes — `README.md:155` |
| Systems / this server | `/portfolio` (the `mg-server` entry — the site is itself the artifact) | Yes — `B3` |

Only links whose target actually exists ship; a link to an unpublished writeup
would be a broken promise (1A) and is gated by `§5.1 T-B2-4`.

**No database, no migration.** The site has no persistence layer.

### 4.3 API contracts

About exposes one route, unchanged in signature:

| Method | Path | Handler | Returns | Auth | Rate limit |
|---|---|---|---|---|---|
| `GET` | `/about` | `pages::about` (`router.rs:38`, `pages.rs:89`) | `impl IntoResponse` → `200 OK`, `text/html`; the rendered `AboutTemplate` | None (public) | The global limiter (`router.rs`), A3's, applies uniformly |

No query params, no path params, no body, no pagination. No error status other
than the shell's 404/500 for non-`/about` URLs. The three template methods
(`title`/`description`/`section`) are the template-level contract A2 S-1 enforces
at compile time; about's obligation is to keep `description()` in the 50–160 char
window and free of retired claims.

**`section()` return type.** Adopts A2's `&str → Section` migration: about returns
`Section::About` once `src/shell.rs` lands. Until then it returns `"about"`
(`pages.rs:83-85`), which matches `base.html:24` and highlights correctly today.
This is a cross-feature dependency on A2, not a B2 decision (§7.4 CFR-2).

### 4.4 State management

| State | Owner | Lifetime | Sync |
|---|---|---|---|
| `bio` string | `AboutTemplate.bio`, set per request in `about()` | Per-request | Server only; a constant, never user-influenced |
| Section identity | `AboutTemplate::section()` | Per-request | Server only |

**No store, no view model, no client state.** About holds nothing across requests
and writes nothing to the client. The `bio` "state" is a compile-time constant
expressed as a runtime `String` — see the note below. Offline/draft persistence:
N/A, nothing is authored in the browser.

**Observation (not a required change): `bio` could be a `const`/template literal.**
It is single-use and never varies, so passing it as a `String` field is slightly
more indirection than needed. Keeping it a handler field is *also* defensible — it
holds copy in Rust (unit-testable, out of the template) and matches how
`description()` lives beside it. Per surgical-change discipline B2 does **not**
mandate a refactor; it only notes that if the field is kept, the bio and
`description()` are two copies of the same "what Jeff studies" claim and must be
edited together and guarded together (§5.1 T-B2-1 covers both). Logged as Q4.

### 4.5 Dependencies

- **New packages:** none. About uses `askama 0.12` / `axum 0.7` /
  `askama_axum`, already present.
- **New assets:** none. About renders no image; the in-body links are text.
- **Fonts / infrastructure:** none. About inherits the shell's system font stacks
  and the CSP (`default-src 'self'`), which it does not touch.

### 4.6 Platform-specific considerations

- **Browser support:** about is plain HTML + inherited shell CSS. It has no CSS
  feature of its own beyond `max-width`/`list-style`/`border` — universally
  supported. It degrades to fully-readable unstyled HTML (B2-E3).
- **No-JS:** about uses **zero** JavaScript. Every element — bio, list, interests,
  links — is server-rendered and works with scripts disabled. This is the strongest
  possible position against auto-fail rule 3.
- **Feature flags / rollout:** N/A — one template, one handler, deployed as one
  unit with the binary.

### 4.7 Performance budget

| Dimension | Figure | Note |
|---|---|---|
| Rendered HTML | The about body is ~1.4 KB of the ~11 KB document; the rest is shared shell | The evidence links add a few hundred bytes |
| JS | 0 bytes from about | It ships none |
| CSS | About's rules are `style.css:957-982` (~26 lines), already in the single shared stylesheet — no new request | Adding evidence links adds no CSS |
| Server CPU per render | One struct construction + one Askama render; no I/O, no syscall, no lock | `about()` reads no file and holds no state |
| Memory | One short `String` per request, dropped after render | Negligible |
| Network requests | Zero beyond the shared document + shell CSS/JS the whole site already loads | About requests nothing of its own |
| Startup | Unaffected — the template compiles into the binary | |

About is the cheapest content page on the site: no content-file read (unlike blog
and learn), no collection load (unlike portfolio), no process-state read of its
own.
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

---

## 5. Test Specification

<<<<<<< HEAD
All tests run under `cargo test --all-targets`, which CI already executes
(`.github/workflows/ci.yml`). Baseline measured for this spec: **30 unit + 2
integration tests, all passing.**

### 5.1 Unit tests — `src/handlers/pages.rs` `#[cfg(test)] mod tests`

**Existing test, kept and repaired.** `about_page_describes_work_plainly_without_disclaimers`
(`pages.rs:204-223`) keeps **every one of its eight assertions unchanged**. Two
things about it change, neither of which weakens a guard:

1. Its fixture stops inventing a bio. `pages.rs:206-208` currently constructs
   `AboutTemplate { bio: "I run a homelab and work through the CompTIA stack." }`
   — a string that appears nowhere in the product and that hardcodes the very
   claim §6.3 removes. It becomes `AboutTemplate { bio: ABOUT_BIO }`, so the test
   finally reads the shipped copy.
2. It gains a comment stating what it actually covers: these assertions run over
   the **whole rendered document including `base.html`**, so the negatives are
   also an implicit guard on shell copy — the 24 theme labels, the nav, the
   footer. That coupling is real and worth keeping (a theme named "Red Team"
   *should* fail something), but it is currently invisible, which is criterion
   5C's complaint. Naming it in a comment converts hidden coupling into declared
   coupling without deleting a guard.

| # | Name | Setup | Assertion | Edge case covered |
|---|---|---|---|---|
| **U-B2-1** | `about_bio_names_only_work_the_posts_can_back` | `ABOUT_BIO` directly — no rendering, no shell | Contains `"homelab"`, `"Proxmox"`, `"broke"`; does **not** contain `"in training"`, `"evidence-first"`, `"security engineer"`, `"red-team"`, `"offensive security"`, `"production-grade"`, `"enterprise"`, `"SRE"` | **Asserts against the string it names** (criterion 5C). The existing test's positives are satisfiable by three different sources — see B2-05 (§7.1) — so this one removes the ambiguity for the bio specifically |
| **U-B2-2** | `about_copy_carries_no_retired_certification_claim` | `ABOUT_BIO` **and** `AboutTemplate::description()` | Neither contains `"CompTIA"`, `"Network+"`, `"Linux+"`, `"Server+"` | **Criterion 1D.** Network+ was dropped 2026-08-02 (`CERT_PLAN.md:84-88`, `:116`); Linux+ and Server+ were dropped earlier (`PUBLIC_FACE.md:22`). These are unconditional — a dropped exam is never nameable |
| **U-B2-3** | `a_named_exam_must_carry_its_real_status` | Same two strings | *If* the copy contains `"RHCSA"`, `"CCNA"`, or `"Security+"`, it **must also** contain `"not earned"` and `"no date booked"` | **Criterion 1F, policy encoded as a test.** `PUBLIC_FACE.md:19-20` permits naming an exam as intent *"provided the copy also says it is not earned and no date is booked."* Under the §6.3 default copy this passes vacuously; the moment someone names an exam it enforces the clause instead of silently allowing a possession claim. It is a *guard*, not a permission — see Q1 |
| **U-B2-4** | `about_page_does_not_claim_capabilities_the_posts_call_planned` | `page_body(&rendered)` — see the helper below | Does not contain `"monitoring"`, `"backup"`, `"VLAN"`, `"auth-log"`, `"health check"` | **Criterion 1B / auto-fail rule 1.** Each banned word is justified in the test's own comment by file and line: `hosting-machinageist-dev.md:103-104` (monitoring), `:105-106` (backup), `management-layer-first-network-migration.md:15` (VLAN/segmentation), `security-headers-on-machinageist-dev.md:96-98` (auth-log). **This test fails against the copy shipping today**, which is the proof it is worth writing. When one of these becomes real, the commit that makes it real updates the post, the copy, and this test together — a conscious decision with a paper trail, which is exactly the shape `project.rs:109-115` and `lab.rs:258-269` already use |
| **U-B2-5** | `about_description_is_within_meta_length` | `AboutTemplate::description()` | `50 <= len <= 160` | A2 U-6's rule applied locally so B2 is verifiable before A2 lands. Current 110; §6.3's replacement is 149 |
| **U-B2-6** | `about_page_links_to_the_evidence` | `page_body(&rendered)` | Contains `href="/blog/hosting-machinageist-dev"` exactly once | The evidence link is the mechanism carrying the honest limits (§6.3). If a future edit removes it, the page silently reverts to unqualified claims |

**The `page_body` helper — B2's answer to criterion 5C.** Both new
rendered-HTML tests assert against the page body, not the whole document:

```rust
// Return just what <main> contains, so a page-body assertion cannot be
// satisfied -- or broken -- by shell copy it never mentions. base.html renders
// description() into <meta name="description"> and og:description, which is how
// the home page's contains("CompTIA") assertion has been passing all along.
fn page_body(html: &str) -> &str {
    let start = html.find("<main id=\"content\">").expect("main opens");
    let end = html.find("</main>").expect("main closes");
    &html[start..end]
}
```

This is nine lines and it *resolves* the coupling class rather than inheriting
it: a test named "about page" now reads only the about page. It does not remove
the existing document-scoped assertions (`pages.rs:213-222`), which stay as a
deliberately broader guard — see B2-06 and the comment change described above.
The same helper is the natural fix for the home page's `contains("CompTIA")`
problem, and is offered to `B1` as a cross-feature request (§7.4).

**Guards explicitly NOT relaxed (criterion 1F).** No assertion is deleted or
loosened anywhere in this spec. For the record, the guards this page and its
neighbours carry, all of which continue to pass unchanged:
`pages.rs:216-218` (`"What I am not claiming yet"`, `"in training"`,
`"evidence-first"`), `pages.rs:220-222` (`"security engineer"`, `"red-team"`,
`"offensive security"`), `project.rs:109-115`, `lab.rs:258-269`.

### 5.2 Integration tests — `tests/about_page.rs` (new)

Router-level, using `tower::ServiceExt::oneshot`, the pattern already established
at `errors.rs:175` and `status.rs:71`. **This file exists because no test in the
repo currently requests `/about` at all** (verified: the only matches for
`"/about"` in `src/` are the route at `router.rs:38` and two counter fixtures at
`state.rs:319`, `:322`).

| # | Name | Assertion |
|---|---|---|
| **I-B2-1** | `about_route_answers_200_with_the_shipped_bio` | `GET /about` → `200`, and the body contains `ABOUT_BIO`'s first clause. **Closes B2-03**: the handler itself, not a fixture, is finally under test |
| **I-B2-2** | `the_about_evidence_link_resolves` | Extract every `href="/blog/…"` from the `/about` body, request each through the same router, assert `200`. **Closes A-02** and is the drift guard criterion 5B asks for: renaming `content/posts/hosting-machinageist-dev.md` now breaks CI instead of breaking the page |
| **I-B2-3** | `about_page_needs_no_javascript` | Strip `<script …></script>` elements from the `/about` body; assert the remainder still contains the `h1`, all four list labels, and the evidence `href`. The machine-checkable form of the no-JS floor for this page (A2 I-2's pattern, applied locally) |
| **I-B2-4** | `about_highlights_its_own_nav_entry` | `/about` body contains `class="nav-link is-active"` on the About link and on no other. Guards the `section()` string against the `/learn` → `"wiki"` class of silent mismatch (`base.html:27`, A2 F-08) |

### 5.3 UI / E2E tests

**Absent, and deliberately not proposed.** There is no browser-automation harness
in this repo — no Playwright, no Selenium, no `package.json` — and A1 §5.3 and
A2 §5.3 both rule that introducing one would cost more than it buys on a site
with ~95 lines of JavaScript. `/about` is the weakest possible case for a browser
harness: it is static prose with one link and **no JavaScript-dependent behaviour
whatsoever**, so a headless browser would assert nothing that I-B2-1…4 do not
already assert against the served bytes. Recorded as a decision, not an omission.

### 5.4 Visual / manual verification

Run `cargo run` and load `/about`.

| Configuration | What to look for |
|---|---|
| **Lunarcore** (dark default) and **Solarcore** (light) | Full pass: the `.about-list` dividers (`--border-subtle`) are visible but quiet; `<strong>` labels (`--text`) separate from list prose (`--text-muted`); the evidence link is both accent-coloured **and** underlined |
| **Paper** and **Cloud** (serif and sans `--font-body`) | `.bio`'s `ch`-based measure holds; the `<code>` spans (`dig`, `ss`, `curl`) stay legible against the surrounding proportional type |
| **Solarized** | The tightest palette in the roster (A1 §5.4); confirm list prose on `--bg` is comfortable |
| **All 23 themes**, spot check | `python3 docs/themes/generate_themes.py --check` covers the token pairs mechanically (currently exit 0); the eye check is for the divider rhythm only |
| **JS disabled** | Page is complete and identical apart from the shell's theme control. This is the page to screenshot when someone asks whether the site really works without JS |
| **`prefers-reduced-motion: reduce`** | Nothing on the page moves; confirm the brand cursor blink is gone (shell) |
| **Browser default font 24px, and 200% zoom** | The `.about-list` at `0.875rem` is the first thing to become uncomfortable — this is the visual proof of the A1 dependency in §7.4 |
| **Viewport 320px @ 400% zoom** | No horizontal scroll; the request-path arrows in the Networking bullet wrap rather than overflow |
| **Viewport height ≤ 34rem** (e.g. landscape phone) | The footer un-pins (`style.css:829-836`) and the short page still scrolls to its own end. Header stays pinned, correctly |
| **Screen reader** (Orca) | Heading list reads "About / What I work with / Further out"; the list announces "list, 4 items"; the evidence link announces its own text without needing the sentence |
| **Read `/about`, then read `/blog/hosting-machinageist-dev`** | **The acceptance test for this whole spec:** no sentence in either contradicts the other |
=======
All tests run under `cargo test --all-targets` and gate CI (`fmt → clippy → test →
build --release`, per criteria 5D).

### 5.1 Unit tests — `src/handlers/pages.rs::tests`

The existing about test (`about_page_describes_work_plainly_without_disclaimers`,
`pages.rs:204-223`) has a **latent defect this spec fixes**: it renders an
*injected* bio (`"I run a homelab and work through the CompTIA stack."`,
`pages.rs:207`), so it exercises template structure but **never touches the real
bio or the real `description()`** — the two places the stale claim actually lives.
That is exactly the hidden-coupling failure criterion 5C exists to catch. The new
tests assert against the **real** handler output.

| # | Name | Setup | Assertion | Edge case covered |
|---|---|---|---|---|
| T-B2-1 | `about_copy_carries_no_retired_or_unearned_claim` | Call `about().await` (real bio) **and** read `AboutTemplate::description()`; render | Neither the rendered HTML nor `description()` contains any of: `CompTIA`, `Network+`, `A+`, `Server+`, `Linux+`, `offensive security`, `red-team`, `pentest`, `production-grade`, `enterprise`, ` SRE` | **The live stale claim** at `pages.rs:81` and `:92`; and future reintroduction of any retired claim |
| T-B2-2 | `about_description_fits_meta_length` | `AboutTemplate::description()` | `50 <= len <= 160` | A description silently truncated in search results / social cards |
| T-B2-3 | `about_leads_with_owned_capability_not_forbidden_identity` | Render the real about page | Contains `homelab` and `What I work with`; does **not** contain `security engineer`, `DevOps engineer`, `SRE`, `in training` as a self-label | Criterion 1E — keeps the existing anti-overclaim negatives and extends them; preserves the show-don't-tell voice (memory: mg-server copy voice) |
| T-B2-4 | `about_evidence_links_point_at_real_routes` | Extract every `/blog/…` and `/portfolio` href from the rendered about body | Each matches a route registered in `router::build` (drive `oneshot` per href and assert `200`) | Criterion 1A/4B — an evidence link to an unpublished writeup is a broken promise; catches a slug typo or a link added before its post exists |
| T-B2-5 | `about_further_out_stays_aspirational` | Render the real about page | If the body contains `AI infrastructure`, it also contains an aspiration qualifier in the same section (`Interests` / `drawn to` / `once the foundation`) | Criterion 1E — pins the "Further out" framing so the AI-infra interest can never drift into a present-tense capability claim |

The existing `pages.rs:204-223` test is **kept** (its structural and
show-don't-tell assertions are still valuable) but its injected bio fixture is
updated to drop `CompTIA`, so the fixture stops modelling the very copy the site
is retiring. T-B2-1 is what actually guards the shipped strings.

### 5.2 Integration tests

Router-level, `tower::ServiceExt::oneshot` (the pattern at `status.rs:84-89`).

| # | Name | Assertion |
|---|---|---|
| I-B2-1 | `about_route_returns_200_and_full_shell` | `GET /about` → `200`; body contains the skip link, `<header class="site-header"`, `<main id="content"`, `<footer`, `vitals-strip`, and `class="nav-link is-active"` on the About link with `aria-current="page"` (A2). Extends A2 I-1 to name About specifically |
| I-B2-2 | `about_needs_no_javascript` | `GET /about`; strip all `<script>` elements; assert the remainder still contains the bio text, all four capability labels, and every evidence-link href | The no-JS floor for this page, machine-checked (rule 3) |

### 5.3 UI / E2E tests

**Absent, and deliberately not proposed.** About has no interactive widget of its
own — only links — and the repo has no browser-automation harness (A1 §5.3, A2
§5.3 record this decision site-wide). The behaviours an E2E suite would cover
(links reachable by keyboard, focus ring visible) are covered by the served-bytes
tests above plus the manual pass in §5.4. Adding Playwright to guard a prose page
would cost more than it buys.

### 5.4 Visual / manual verification

Following A1's tiered model. About is a Tier-1 surface (checked on every visual
change) because it is a primary reviewer path.

- **Themes:** the A1 Tier-1 six — Lunarcore (default/dark/mono), Solarcore
  (light/`system`), Paper (serif — the `65ch` measure and `<strong>` weight
  contrast are most stressed here), Cloud (sans; lowest accent-on-surface ratio),
  Solarized (tightest palette), CRT (glow path). Confirm `--text-muted` body and
  `--text` labels clear AA in each, and that in-body links are distinguishable
  from body text by underline in every theme (not by hue).
- **Text size / zoom:** browser default 24px and 200% zoom — the `.about-list li`
  `0.875rem` items must scale and stay ≥ AA (after A1's body-size fix).
- **Viewport:** 320px, 800px, 1280px — the capability list wraps cleanly; no
  horizontal scroll at 320px @ 400% zoom.
- **Reduced motion:** `prefers-reduced-motion: reduce` — about has no motion of
  its own; confirm inherited chrome is static.
- **JS disabled:** load `/about` with scripts off — full page renders, all links
  work, palette follows OS preference (A2 fallback).
- **Empty/degraded:** N/A — about has no dynamic collection; confirm the page is
  identical on every load (it takes no input).
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement.**

<<<<<<< HEAD
The page stores nothing, transmits nothing, and reads no request data. It
contains one piece of personal information: **"Portland, OR."**
(`about.html:28`), city-only by a recorded decision — commit `a2c096a`, *"fix:
keep location copy city-only."* That granularity is appropriate for a job-seeking
portfolio (recruiters filter on metro area) and is preserved unchanged. No street
address, no employer, no phone, no personal email; contact is via
`security.txt` and the GitHub link in the shell footer (`base.html:93`), both
A2/A3 territory.

No cookies, no forms, no analytics, no third-party requests — CSP is
`default-src 'self'` (A2 §6.1).
=======
About stores nothing, reads no request data, sets no cookie, and takes no input.
Its only "data" is static biographical copy the operator chose to publish
(name, city, capabilities). "Portland, OR." is city-level only — a2c096a
("fix: keep location copy city-only") already pinned that; the spec preserves it
and does not add a street address, employer, or contact detail.
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

### 6.2 Asset provenance

- [x] **No third-party assets.**

<<<<<<< HEAD
The page ships no image, font, icon, script, or data file. Its only non-text
elements are the shell's, and A1 §6.2 accounts for those. The four `<code>` spans
name standard POSIX/iproute2/curl utilities, which is nominative use of a command
name and carries no licence obligation.

### 6.3 Language / claims audit — **the substance of this spec**

- [x] Makes claims not supported by evidence? **Yes, today — five of them. This
      section removes them.**
- [x] Promises capabilities not yet built? **Yes, today. Removed.**
- [x] Uses language restricted by domain regulations? No.

#### 6.3.1 Full copy inventory with state classification

Every user-visible string this feature owns, classified
**implemented / prototyped / planned / gated / absent** (criterion 1B), with the
evidence that decides the classification.

| # | String | Location | State | Evidence | Verdict |
|---|---|---|---|---|---|
| 1 | `"About — machinageist"` | `pages.rs:78` | — | Functional | ✅ Keep |
| 2 | `"about"` | `pages.rs:84` | — | Functional | ✅ Keep |
| 3 | `"…a Proxmox homelab, networking and Linux operations, small automation, and CompTIA study."` | `pages.rs:81` | **stale** | Spine re-locked 2026-08-02 (`CERT_PLAN.md:3-5`) | ❌ **Replace** |
| 4 | `"…I'm working through the CompTIA stack."` | `pages.rs:91-93` | **stale** | Same | ❌ **Replace** |
| 5 | `"I run a homelab"` | `pages.rs:91` | implemented | `management-layer-first-network-migration.md:19` | ✅ Keep, sharpen |
| 6 | `"write about what breaks and how I fix it"` | `pages.rs:91-92` | implemented | The migration post is exactly this (`:13`, `:106-135`) | ✅ Keep, sharpen |
| 7 | `"Linux, systemd, journald"` | `about.html:10` | implemented | `hosting-machinageist-dev.md:74-88`; `journalctl -u mg-server.service` at `:82` | ✅ Keep |
| 8 | `"a Proxmox homelab — VMs, a small cluster"` | `about.html:10` | implemented | "three-node Proxmox cluster on hardware I own, with a router VM, a managed switch, and several guests" (`migration:19`); quorum broken and recovered (`:106-135`) | ✅ Keep, **sharpen to "three-node"** — the evidence is stronger than the claim |
| 9 | `"backups"` | `about.html:10` | **planned** | `hosting:105-106` "**No tested backup/restore of the VM.** …not something I have validated yet" | ❌ **Remove** |
| 10 | `"monitoring"` | `about.html:10` | **absent** | `hosting:103-104` "**No automated monitoring or alerting.** If the service goes down, I find out by looking" | ❌ **Remove** |
| 11 | `"DNS, subnetting"` | `about.html:11` | implemented | `hosting:33-50`; the whole migration post | ✅ Keep |
| 12 | `"and VLANs"` | `about.html:11` | **planned** | `migration:15` "segmentation has not started"; `:236-243` plans it | ❌ **Remove** |
| 13 | `"the Cloudflare Tunnel → Caddy → mg-server request path"` | `about.html:11` | implemented | `hosting:15-32` with real `dig`/`curl` output | ✅ Keep — the strongest claim on the page |
| 14 | `"CLI diagnostics (dig, ss, curl)"` | `about.html:11` | implemented (`dig`, `curl`); unevidenced (`ss`) | `dig`/`curl` output published in `hosting` and `security-headers`; `ss` appears in no post | ⚠️ **Keep all three** — see 6.3.3 |
| 15 | `"Rust and Python for small operations tools"` | `about.html:12` | implemented | `mg-server` itself (`project.rs:77-85`); "backup and audit scripts" (`migration:49`); read-only state collection across all three nodes (`migration:94-97`, `:99-103`) | ✅ Keep, retarget the tail — see 6.3.3 decision 4 |
| 16 | `"health checks, and reports"` | `about.html:12` | **planned** | No published evidence; `PUBLIC_FACE.md:246-247` gates `mg-health`/`mg-netnotes` as "Later, after real checks and sample output exist" | ❌ **Remove** |
| 17 | `"HTTP headers"` | `about.html:13` | implemented | `security-headers-on-machinageist-dev.md` entire, with reproducible `curl` evidence and its own safe-claim line at `:92-94` | ✅ Keep |
| 18 | `"SSH and TLS hardening, and auth-log detection"` | `about.html:13` | **planned** | `security-headers:96-98` "…are planned next, on owned scope only" | ❌ **Remove** |
| 19 | `"on things I own"` | `about.html:13` | — | The owned-scope qualifier `public-portfolio-structure.md:104` requires | ✅ Keep |
| 20 | `"Further out … AI infrastructure and platform work … Interests, mostly"` | `about.html:21-26` | **explicitly labelled as interest** | `PUBLIC_FACE.md:326` permits AI infrastructure as "Learning notes only"; `:37`, `:39` forbid the *identity* | ✅ **Keep unchanged**, and pin its position — see 6.3.4 |
| 21 | `"Portland, OR."` | `about.html:28` | — | Commit `a2c096a` | ✅ Keep |
| 22 | The claim-boundary comment | `about.html:16-18` | — | Recorded decision + commit `9b5b1a1` | ✅ Keep, **amend with a dated line** |

#### 6.3.2 Criterion 1D — the corrected certification copy, stated explicitly

**What changed and when.** `~/mg-coreforge/bootcamp/CERT_PLAN.md:3-5`:
*"Re-locked 2026-08-02 (Network+ dropped; RHCSA promoted to first and made the
only pre-employment exam). … Study/exam order: **RHCSA → CCNA → Security+.**"*
The comparison table at `:80` confirms: exams `RHCSA, CCNA, Security+`,
pre-employment `RHCSA only`. Exam codes and status at `:113-116`: RHCSA EX200
(pre-employment), CCNA 200-301 (employed), Security+ SY0-701 (employed),
`~~Network+ N10-009~~` **dropped 2026-08-02**.

**Why the current copy is wrong.** `"working through the CompTIA stack"`
(`pages.rs:92`) and `"CompTIA study"` (`pages.rs:81`) describe the *original*
four-exam CompTIA sequence — Network+ → Security+ → Linux+ → Server+, still
visible in `IMPROVEMENT_PLAN.md:15` and `content/drafts/portfolio-entries.md:49-51`.
Under the live spine **exactly one CompTIA exam remains and it is last**, behind
a Red Hat exam and a Cisco exam. "The CompTIA stack" therefore overstates both the
*quantity* and the *ordering* of the plan. Criterion 1D: a claim that was true once
still scores 0.

**What replaces it — the default, and it names no exam.**

`pages.rs` description (110 → 149 chars, both inside A2 U-6's 50–160 band):

```rust
pub fn description(&self) -> &str {
    "About Jeff Cincoski — a three-node Proxmox homelab, networking and Linux operations, \
     small automation tools, and the write-ups that come out of them."
}
```

`ABOUT_BIO`:

```
I'm Jeff. I run a homelab — a three-node Proxmox cluster on hardware I own — and I
write up the networking and Linux work that comes out of it, including the parts
that broke. Most of what's on this site starts from hardware I operate myself.
```

Note what the bio preserves from the original: the hedge **"Most of what's…"**
(`pages.rs:93`). The site's `/learn` pages are study material, not operated
systems, and the original copy was already honest about that. Removing the hedge
to sound stronger would be a new overclaim, so it stays.

**Why no exam is named at all in the default.** Two governing documents point in
slightly different directions and the spec resolves the tension rather than
picking one silently:

- `criteria.md` auto-fail rule 1 bans "a certification claim without a booked
  exam voucher," and `CERT_PLAN.md:143-145` confirms **no date is booked** —
  RHCSA's is gated behind a measured baseline that does not exist yet.
- `PUBLIC_FACE.md:15-20`, amended **2026-08-03** (one day *after* the re-lock
  that criterion 1D cites), loosens the rule: certs are *"nameable as intent,
  never as possession,"* and RHCSA specifically is *"nameable as actively
  studying, provided the copy also says it is not earned and no date is booked."*

The strict reading and the loosened reading agree on everything except whether
"RHCSA" may appear at all. **The default copy names no exam, which satisfies both
readings and cannot auto-fail.** The loosened variant is offered as **Q1** (§8) —
it needs Jeff's explicit decision, because a spec agent adopting the more
permissive of two conflicting policies without one is exactly the move criterion
1F scores 0. Whichever way Q1 goes, **U-B2-3 enforces the required status clause**,
so the permission can never be exercised half-way.

**Amendment to the in-template comment** (`about.html:16-18`), following the
repo's own convention of a dated line citing the governing doc:

```
{# Certifications section intentionally removed 2026-07-25. Do not restore a cert
   claim here until an exam voucher is actually booked, and then state only that one
   exam with its scheduled date. See mg-coreforge/PUBLIC_FACE.md.
   2026-08-08: the spine re-locked 2026-08-02 to RHCSA -> CCNA -> Security+; Network+
   is dropped. "The CompTIA stack" left the bio in the same change. PUBLIC_FACE.md
   (2026-08-03) now allows naming an exam as intent if the copy also says it is not
   earned and no date is booked -- the test below enforces that clause if anyone
   takes the option. #}
```

#### 6.3.3 Criterion 1B — the corrected "What I work with"

Replaces `about.html:9-14`. Every clause traces to row 7, 8, 11, 13, 14, 15, 17,
or 19 of the inventory above.

```html
<ul class="about-list">
  <li><strong>Systems:</strong> Linux, systemd, and journald on a three-node
    Proxmox cluster I own &mdash; VMs, a managed switch, and the recovery work
    after I broke quorum during a subnet migration.</li>
  <li><strong>Networking:</strong> DNS, subnetting, and the Cloudflare Tunnel
    &rarr; Caddy &rarr; mg-server request path, traced from the CLI with
    <code>dig</code>, <code>ss</code>, and <code>curl</code>.</li>
  <li><strong>Automation:</strong> Rust and Python for small operations tools
    &mdash; this site&rsquo;s server, and the scripts that collect read-only
    state from the lab nodes before I change anything.</li>
  <li><strong>Security:</strong> the HTTP response headers on this site,
    reviewed and documented with reproducible <code>curl</code> evidence, on
    things I own.</li>
</ul>

<p class="about-note">
  Where something isn&rsquo;t proved yet, the write-up says so &mdash; the hosting
  post ends with <a href="/blog/hosting-machinageist-dev">what this setup still
  doesn&rsquo;t have</a>.
</p>
```

Four design decisions worth defending:

1. **Omission, not disclaimer.** Seven aspirational nouns are deleted rather than
   qualified. `pages.rs:216` asserts the page contains no `"What I am not
   claiming yet"` section and the test is literally named
   `…without_disclaimers`; that boundary is satisfied, not weakened. A per-bullet
   "not yet" clause would technically pass the substring check while violating
   the decision the check encodes.
2. **One link carries every honest limit.** `hosting-machinageist-dev.md:101-110`
   is already titled *"What is honestly not here yet"* and already enumerates
   monitoring, backups, CI/CD, and the TLS boundary — with more precision than an
   about page could. Pointing at it is strictly better than paraphrasing it, it
   is *shorter*, and it turns `/about` from a dead end into a route to evidence.
   `about.html` currently contains **zero links** (verified), which for the
   hiring-manager path is a defect in its own right (§7.1 B2-08).
3. **The "Security" bullet copies the post's own safe-claim wording.**
   `security-headers-on-machinageist-dev.md:92-94` already states the exact
   defensible form: *"reviewed and documented the HTTP security headers for an
   owned web service, with reproducible `curl` evidence. **Not** a claim to have
   'secured the application.'"* Reusing that phrasing means the about page and
   the post cannot drift, and it is the in-repo model for a bounded claim.

4. **The word "backup" does not appear, even though backup scripts do.**
   `management-layer-first-network-migration.md:49` lists "backup and audit
   scripts with hardcoded addresses" — they exist, and naming them would be
   defensible in isolation. But `hosting-machinageist-dev.md:105-106` says "**No
   tested backup/restore of the VM**," and a reader who meets the word "backup"
   on an about page infers the second thing, not the first. The replacement
   claims the *discipline* instead, which is both stronger and unambiguous:
   "the scripts that collect read-only state from the lab nodes before I change
   anything" is `migration:94-97` and the three-step recovery pattern at
   `:99-103`, and it is the behaviour that actually distinguishes this candidate
   from the competitor set (criterion 4B). It also keeps U-B2-4's banned-word
   list clean rather than carving an exception into it — a test with an
   exception for the word that caused the defect is not a test.

**On `ss` (inventory row 14).** `dig` and `curl` have published output; `ss` does
not. Kept, deliberately: a tools line is a "can use," not a published artifact,
and the evidence standard (`public-portfolio-structure.md:76-92`) governs
*artifacts*. Deleting a command Jeff can demonstrate in an interview would be
miscalibrated pruning in the opposite direction, and the calibration is the point.
Recorded here so the decision is visible rather than accidental.

#### 6.3.4 Criterion 1E — role posture

**What the page must lead with, and does under this spec:** owned hardware, a
named cluster topology, service operations, a traced request path, an incident
worked end to end. **What it must not lead with, and never does:** the copy
contains no job title of any kind — not "Systems Administrator", not "NOC
Technician", not "SRE", not "DevOps", not "engineer".

That is a stronger position than the one `public-portfolio-structure.md:98`
recommends (*"Systems Administrator / NOC Technician (in training)"*), and it
resolves a direct contradiction inside this repository:

> `docs/public-portfolio-structure.md:98` says to **say** "(in training)".
> `src/handlers/pages.rs:217` asserts the page does **not** contain
> `"in training"`.

Criterion 1F is unambiguous: **the test wins.** The resolution is not a
compromise but a better answer than either input — *a page that claims no title
at all needs no qualifier on the title.* "(in training)" exists to bound a claim;
delete the claim and the bound becomes unnecessary. This also matches
`PUBLIC_FACE.md:53-55` ("Show evidence before identity… let titles and
aspirations stay conservative") and `:39` ("Describe what you have built and
operated; let the trajectory be implied by the evidence").

**"Further out" (`about.html:20-26`) stays exactly as written.** It names AI
infrastructure, FPGA/RISC-V/photonics, off-grid compute, and auditable AI —
then closes *"Interests, mostly — the kind of thing I'd chase once the foundation
is solid."* `PUBLIC_FACE.md:326` permits AI infrastructure as "Learning notes
only" and forbids the identity claim "AI infrastructure engineer"; `:39` warns
specifically that eventual ambition "must not leak into present-tense public
copy." This paragraph is present-tense about *interest* and explicitly
future-tense about *work*, which is the compliant form. Two invariants B2 pins:
it must remain the **last** content section (an aspiration section above the
evidence section would invert the posture 1E requires), and it must retain an
explicit not-yet marker in its final sentence.

#### 6.3.5 Criterion 1C — the GeistScope publication gate

**Not implicated.** `templates/about.html` and the `pages.rs` About block contain
no occurrence of "GeistScope" (verified). The site's only GeistScope exposure is
`ReleasesTemplate::description()`, which A2 §6.3 routed to `B6`. B2 adds an
invariant: **`/about` must never name a GeistScope tool**, since an about page is
the easiest place for a gated capability to reappear as a personal-identity
claim. Covered by the existing `!html.contains("offensive security")` /
`"red-team"` guards (`pages.rs:220-222`) and by U-B2-1.

### 6.4 Regulatory alignment — `criteria.md` Lens 3

| Criterion | How B2 addresses it |
|---|---|
| **3A — works without JavaScript** *(auto-fail)* | `/about` is 100% server-rendered static prose with **zero** page-level JavaScript and no enhancement layer to fall back from. The one interactive element added is a plain `<a href>`. Machine-checked by **I-B2-3**, which strips `<script>` elements from the served bytes and asserts the page is still complete. This is the site's least-conditional satisfaction of the floor. |
| **3B — contrast and colour independence** *(auto-fail)* | The page introduces no new colour. Every pair it uses (`--text` on `--bg`, `--text-muted` on `--bg`, `--border-subtle` dividers, `--accent` link) is in A1's audited set; `python3 docs/themes/generate_themes.py --check` exits 0 today across all 23 themes. The one new element, the evidence link, is `--accent` **plus** the UA underline, so it is not colour-only. The page communicates **no state at all**, by hue or otherwise, which removes the failure mode. |
| **3C — keyboard and focus** | One new focusable element in DOM order, no `tabindex`, no shortcut, no focus trap, no widget. It inherits the global never-removed `:focus-visible` ring. There is nothing on this page a keyboard user can reach but not operate. |
| **3D — semantics and AT** | §3.7: one `h1`, two `h2`, no skipped level; a real `<ul>`/`<li>`; `<code>` for command names; link text meaningful out of context per WCAG 2.4.4/2.4.9; no `aria-*` added because none is warranted — the correct elements make it unnecessary, which is the preferred outcome. |
| **3E — motion and sensory safety** *(auto-fail)* | The page defines no `transition`, `animation`, autoplay, or flashing content. Shell motion is already behind `prefers-reduced-motion: no-preference` (A2 §3.5). Reduced-motion alternative is absence, inherited. |
| **3F — responsive and resilient** | §3.4 and §5.4: 320px @ 400% zoom, 24px browser default font, ≤34rem viewport height (where `style.css:829-836` un-pins the footer — already correct, unchanged). Resilience: the handler has no failure path (§3.6 A-01), and the page reads correctly with the stylesheet absent (A-05). |
=======
About ships no image, font, model, or data file. Its text is original copy written
by the operator; the fonts are the shell's system stacks (A1); the CSP forbids any
external asset. Nothing to license.

### 6.3 Language / claims audit

- [ ] Makes claims not supported by evidence — **must not.** The target state
  *strengthens* evidence support by linking each named capability to a published
  writeup (§4.2). Capabilities without a published writeup (e.g. "Automation:
  Rust and Python for small operations tools") are stated as **plainly true
  activities**, not as portfolio artifacts, and carry no link implying one — which
  is correct, because the portfolio is pinned at one entry (`project.rs`, A1 D7)
  and a claim is not the same as an artifact.

- [ ] Promises capabilities not yet built — **must not.** The "Further out"
  paragraph (`about.html:20-26`) names AI infrastructure, FPGA/RISC-V/photonics,
  off-grid compute, and auditable AI as **interests**, explicitly framed as
  aspiration ("drawn to," "Interests, mostly," "the kind of thing I'd chase once
  the foundation is solid"). This framing is what keeps it inside claim discipline;
  T-B2-5 pins it so it cannot drift into a present-tense claim. This is the one
  place on the page where 1E is at live risk, and it is handled by framing plus a
  test rather than by deleting a legitimate statement of interest.

- [ ] Uses language restricted by domain regulations — **must not.** No cert name
  appears until its voucher is booked (§4.2, auto-fail rule 1). No "secured,"
  "production-grade," "enterprise," "SRE," "DevOps engineer," "pentest," or
  "red-team" identity (1E). The retired `CompTIA` phrase is removed from both
  occurrences (`pages.rs:81`, `:92`).

### 6.4 Regulatory alignment

Mapping to `criteria.md` and confirming each is addressed:

| Criterion | How B2 satisfies it |
|---|---|
| **1A evidence standard** | Each named capability links to a writeup that can state why-it-matters / start / target / tools / evidence / verification (the `hosting` and `security-headers` posts already do); capabilities without such a writeup are stated as activities, not artifacts (§6.3) |
| **1B state honesty** | Implemented vs. target is distinguished throughout §7; the aspirational "Further out" content is labelled aspiration, never shipped capability |
| **1C GeistScope gate** | About names no GeistScope tool and implies no offensive-security identity; verified — no `geistscope`/`offensive`/`red-team` token in about copy |
| **1D copy currency** | **The feature's primary job.** The stale `CompTIA` phrase is removed from `description()` and `bio`; replacement is capability-led and cert-agnostic; T-B2-1 guards it (§4.2, §5.1) |
| **1E role posture** | Copy leads with owned/operated capability (homelab, request path, CLI diagnostics, defensive on owned scope); forbidden identities excluded and tested (T-B2-3) |
| **1F test-encoded policy** | The existing anti-overclaim test (`pages.rs:204-223`) is kept and strengthened, not weakened; T-B2-1..5 add guards. No guard is relaxed |
| **2B typographic craft** | Prose capped at measure; heading hierarchy legible without colour (case/weight); the `65ch` and `0.875rem` literals are handed to A1 to tokenise (CFR-1) |
| **2E restraint** | About stays quiet prose — no cards, no fake metrics, no motion; the show-don't-tell voice (memory) is pinned by T-B2-3 |
| **2F theme integrity** | About uses only role tokens (`--text`, `--text-muted`, `--border-subtle`); works across all 23 themes with no per-theme edit |
| **3A no-JS** | About ships zero JS; I-B2-2 machine-checks it |
| **3B contrast / colour independence** | Pairs are in A1's audited matrix; state (labels, links) uses weight + underline, never hue alone |
| **3C keyboard / focus** | Only links; native focus, global `:focus-visible` ring, DOM order |
| **3D semantics** | One `<h1>`, two `<h2>`, a real list, self-describing link text; no decorative content to hide |
| **3E motion** | None of its own |
| **3F responsive / resilient** | Single reflowing column; readable unstyled (B2-E3); no empty-state accident (it has no collection) |
| **4A 30-second differentiation** | Names a real operated system and a real request path, not a skills word cloud (§4B below) |
| **4B evidence over enthusiasm** | Capabilities link to writeups incl. the migration outage — the in-repo 4B model (§4.2) |
| **4C original explanation** | N/A for about (an identity page, not an education surface) — the learning material is `B5`; about *points* to it |
| **4D depth of a real system** | Copy references the Proxmox lab, DNS/VLANs, the Tunnel→Caddy→mg-server path, and this server itself |
| **4E reviewer paths** | Serves hiring manager (role + evidence links), engineer peer (concrete tools/path), learner (`/learn` pointer via shell + writing links) |
| **5A single source of truth** | The "what Jeff studies" claim is deduplicated in intent and co-guarded across `description()` and `bio` (T-B2-1); links reference canonical routes |
| **5C no hidden coupling** | Fixes the existing test's injected-fixture blind spot by asserting against the *real* handler output (T-B2-1) |
| **5D verification stated** | The four CI commands; the exact new test names |
| **5E docs follow behavior** | §7.4 names the docs to update when the copy changes |
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

<<<<<<< HEAD
**Overall state: implemented, and materially wrong in its copy.** The route,
handler, template, and CSS all work. The defects are claims and coverage.

**Implemented and correct — keep:**

- Route (`router.rs:38`), handler (`pages.rs:89-95`), compile-time-validated
  template binding (`pages.rs:70`).
- `title()` already carries the `" — machinageist"` suffix A2 Contract S-1
  requires (`pages.rs:78`) — one of only five pages that do.
- `section()` returns `"about"` and matches `base.html:24` correctly.
- Heading outline, list semantics, city-only location (`a2c096a`).
- The claim-boundary comment (`about.html:16-18`) and its matching test change
  (`9b5b1a1`) — a genuinely good in-repo practice this spec extends.
- "Further out" is correctly hedged and correctly last (§6.3.4).

**Findings, in severity order:**

| ID | Severity | Finding | Evidence |
|---|---|---|---|
| **B2-01** | **Critical (1B / auto-fail rule 1)** | Five capability claims present planned or absent work as operated: `"backups"` and `"monitoring"` (`about.html:10`), `"VLANs"` (`:11`), `"health checks, and reports"` (`:12`), `"SSH and TLS hardening, and auth-log detection"` (`:13`). Four of five are contradicted **by the site's own published posts**, one click away. | `hosting-machinageist-dev.md:103-106`; `management-layer-first-network-migration.md:15`; `security-headers-on-machinageist-dev.md:96-98`; `PUBLIC_FACE.md:246-247` |
| **B2-02** | **High (1D)** | `"working through the CompTIA stack"` and `"CompTIA study"` describe a four-exam CompTIA sequence retired 2026-08-02. One CompTIA exam remains and it is **last**, behind RHCSA and CCNA. | `pages.rs:81`, `:91-93`; `CERT_PLAN.md:3-5`, `:80`, `:113-116` |
| **B2-03** | **High (5C)** | **The shipped about copy has no test coverage.** No test calls `pages::about()`; no integration test requests `/about` (`status.rs:114` covers only `/` and `/blog`); the sole about test constructs the template with a **synthetic** bio. B2-01 and B2-02 both survived a green CI *because of this*. | `pages.rs:206-208`; `grep '"/about"' src/` → `router.rs:38`, `state.rs:319`, `:322` only |
| **B2-04** | **High (5C)** | The test fixture itself hardcodes the retired claim: `"I run a homelab and work through the CompTIA stack."` Fixing the handler leaves the stale phrase sitting in the test file, where a future grep for the retired wording still hits and a future reader is misled about what the site says. | `pages.rs:207` |
| **B2-05** | Medium (5C) | `assert!(html.contains("homelab"))` is satisfied **three independent ways** — the injected fixture bio, `about.html:10`, and `<meta name="description">` rendered from `pages.rs:81` via `base.html:6`/`:8`. It therefore cannot fail even if the page body loses the word entirely. Same class as A2 F-17, overdetermined rather than misdirected. | `pages.rs:214`; `about.html:10`; `base.html:6`, `:8`; `pages.rs:81` |
| **B2-06** | Medium (5C) | Every negative assertion is evaluated over the **whole rendered document**, including `base.html`'s 24 theme labels, nav, and footer. An about-page test is therefore an undeclared guard on shell copy: adding a theme named "Red Team" would fail a test whose name mentions neither themes nor the shell. Worth keeping as a guard; not worth leaving invisible. | `pages.rs:216-222` evaluated over `base.html:36-79` |
| **B2-07** | Medium (1E/1F conflict) | `docs/public-portfolio-structure.md:98` instructs the site to **say** "Systems Administrator / NOC Technician (in training)". `pages.rs:217` asserts the page does **not** contain `"in training"`. A governing doc and a test-encoded policy in the same repository directly contradict each other, and nothing records which wins. | `public-portfolio-structure.md:98`; `pages.rs:217` |
| **B2-08** | Medium (2D / 4E) | `templates/about.html` contains **zero links**. The page a hiring manager opens first offers no route to any evidence; the header nav is the only way onward. For the primary arrival path this is the difference between "interesting claims" and "interesting claims I can check." | `templates/about.html:1-30` |
| **B2-09** | Low (2B / 2F) | Three A1-owned defects land on this page: `.about-list li` is `font-size: 0.875rem` (`style.css:973`), a literal off the type scale that renders the *claim list* smaller than body copy; `.bio` caps at `max-width: 65ch` (`:963`), a hardcoded measure rather than `--measure-narrow`; `.bio-loc` (`about.html:28`) has **no CSS rule at all** — one of A1 T8's six orphan classes. | `style.css:961-982`; `about.html:28` |
| **B2-10** | Low (1A) | `ss` (`about.html:11`) has no published command output, while `dig` and `curl` do. A tools line is not an artifact, so this is not an overclaim — recorded so the judgment is visible. | `about.html:11`; `grep '\bss\b' content/posts/` → no match |
| **B2-11** | Informational | `docs/public-portfolio-structure.md` is itself stale in two ways relevant here: its amendment header (`:9`) names a **Network+ then RHCSA** spine that is two revisions old, and `:128` asserts *"A dedicated `/start-here` route exists"* — **no such route exists** in `router.rs:37-58`. Not B2's document to rewrite, but B2 depends on it and must not inherit its errors. | `public-portfolio-structure.md:9`, `:128`; `router.rs:37-58` |
| **B2-12** | Informational | Cross-page inconsistency for `B1`: `index.html:8-9` says *"Right now I'm building out a three-node cluster"* while `management-layer-first-network-migration.md:19` says *"The homelab **is** a three-node Proxmox cluster on hardware I own"* and describes an eight-hour outage recovering it. The home page **under**claims relative to published evidence. B2's copy uses the evidenced form; B1 should reconcile. | `index.html:8-9`; `migration:19`, `:13` |

### 7.2 Delta to spec

**New files (1)**

- `tests/about_page.rs` — I-B2-1 … I-B2-4.

**Modified files (3)**

| File | Change | Fixes |
|---|---|---|
| `src/handlers/pages.rs` | Add `const ABOUT_BIO: &str` with the corrected copy; `AboutTemplate.bio` becomes `&'static str`; `about()` returns `AboutTemplate { bio: ABOUT_BIO }`; `description()` replaced; existing test's fixture switched to `ABOUT_BIO` and given a coupling comment; add U-B2-1 … U-B2-6 | B2-02, B2-03, B2-04, B2-05, B2-06 |
| `templates/about.html` | Replace the four `<li>` in `.about-list`; add `<p class="about-note">` with the evidence link; amend the claim-boundary comment with the dated 2026-08-08 line | B2-01, B2-08 |
| `static/css/style.css` | Add a `.about-note` rule (3–5 declarations, using A1 tokens — `--text-muted`, `--space-*`, `--measure-narrow`) | B2-08 |

**Not modified, deliberately:** `src/router.rs` (the route is correct);
`templates/base.html` (A2's); the `.about-list` / `.bio` values (A1's — B2 files
a request, it does not set them); `.about-list`'s `<ul>` markup, because the class
is **shared with `index.html:22`** and converting it to a `<dl>` on `/about`
would either fork the class or silently restyle the home page. A `<dl>` would be
marginally more semantic for a label→description list; it is not worth a
cross-feature regression on a list four items long whose labels are already
visually and textually labels. Recorded as a decision.

**Migrations / schema changes:** none — no database.
**New dependencies:** none.

**Suggested commit sequence** (each independently shippable and verifiable):

1. `refactor: make the about bio a constant so tests read the shipped copy` —
   `ABOUT_BIO`, `&'static str` field, existing test's fixture switched. Nothing
   user-visible changes; CI must stay green. *This lands first so every
   subsequent copy change is under test as it happens.*
2. `test: pin the about page's claims to the posts that back them` — U-B2-1 …
   U-B2-6 and `tests/about_page.rs`. **U-B2-4 fails here by design**, which is
   the demonstration that the defect is real.
3. `fix: stop the about page claiming work the posts call planned` — the
   `about.html` list rewrite and the evidence link. U-B2-4 and U-B2-6 go green.
4. `fix: retire the CompTIA copy from the about page` — `ABOUT_BIO` and
   `description()` replaced, comment amended. U-B2-2 goes green.
5. `style: give the about note a rule` — the `.about-note` CSS.

Commits 2 and 3 may be squashed if a red intermediate commit on `main` is
unwanted; the ordering trap A2 §7.4 names applies here in the same way — a guard
that lands before the copy it guards turns CI red.

### 7.3 Estimated scope

**S.**

One template rewritten in place (30 lines), one handler block edited, one new
integration test file, one CSS rule. No new route, no new model, no new
dependency, no migration, no architecture. The volume is in judgment, not in
code: the hard work was cross-checking twenty-two copy strings against four
published posts and two governing documents, and that work is done and recorded
in §6.3.1 so the implementer does not repeat it.

Not XS: it touches user-visible claims on the site's highest-risk page, and it
adds ten tests including one that fails on purpose until the copy is fixed.

### 7.4 Blocking dependencies

**Blocking B2 (must land first, or B2 ships and inherits a known defect):**

| Dependency | Feature | What B2 needs |
|---|---|---|
| `--measure-narrow` token and the removal of `font-size` literals | `A1` | `.bio`'s `65ch` and `.about-list li`'s `0.875rem` are A1's to normalise. B2 can ship its copy without this; the page is simply typographically off-scale until A1 lands. **Not hard-blocking.** |
| `.about-note` styling tokens | `A1` | The new paragraph needs `--text-muted` at body size within `--measure-narrow`. All exist or are specified in A1 §4.2. |
| The "in-prose links stay underlined" invariant | `A1` §3.7B | B2's colour-independence argument depends on it. A1 records it as shipped-but-accidental and pins it with a test; B2 needs that pin. |
| `Section` enum | `A2` | `section()` changes `&str` → `Section`. B2 must **not** pre-empt it; when A2 lands, `pages.rs:83-85` changes mechanically. |
| U-7 `descriptions_do_not_carry_retired_claims` | `A2` | A2's global guard and B2's U-B2-2 overlap deliberately. **Ordering trap:** U-7 fails CI the moment it lands unless B2 commit 4 has already landed. B2 commit 4 should precede A2's U-7, or they ship together. |

**Blocked by B2:** nothing. `/about` is a leaf content surface.

**Cross-feature requests B2 files (not implemented here):**

1. → **`B1` home.** Reconcile `index.html:8-9` ("building out a three-node
   cluster") with `migration:19` ("The homelab **is** a three-node Proxmox
   cluster"). The home page under-claims relative to its own evidence (B2-12).
   B2 uses the evidenced form; two pages should not disagree about whether the
   cluster exists. **Also take the `page_body` helper** (§5.1): it is the direct
   fix for `pages.rs:158`'s `contains("CompTIA")`, which passes only through the
   `<meta>` tag and is criterion 5C's named example. B2 wrote the helper for its
   own tests and B1 should reuse it rather than reinvent it — and per A2's §7.4
   note, the relocated assertion must pin the *new* spine wording, not the
   retired one.
2. → **`B4` writing.** Two items. (a) Heading anchors: pulldown-cmark 0.10 with
   `Options::all()` generates no heading `id`s and no post uses explicit `{#id}`
   syntax (verified), so `/about` must link to the post rather than to
   *"What is honestly not here yet"* (`hosting:101`) directly. Adding slug
   anchors would materially improve this and every future cross-link. (b)
   `hosting:104` says a monitoring stack is "planned for a later **cert phase**",
   which is 1D-stale against the re-locked spine. B4's string, flagged not
   touched.
3. → **`A1` design system.** B2-09's three items are already inside A1's §7.1.6
   orphan-class list and §7.2 font-size-literal work; this is confirmation from a
   consuming page, with the added note that the off-scale literal lands on the
   *claim list*, where scannability matters most.
4. → **Documentation owner (see §5E below).** `docs/public-portfolio-structure.md`
   needs two corrections independent of B2: its amendment header (`:9`) names a
   two-revision-stale spine, and `:128` asserts a `/start-here` route that does
   not exist in `router.rs` (B2-11).

**External gates:** none. B2 publishes no artifact, names no exam under the
default copy, and does not touch the GeistScope gate.
=======
**State: implemented, with one live claim defect.**

- **Route + handler:** implemented. `/about` → `pages::about` (`router.rs:38`);
  handler builds `AboutTemplate` and returns it (`pages.rs:89-95`).
- **Template:** implemented. `templates/about.html` (30 lines): `<h1>`, bio, "What
  I work with" list (4 items), the cert-removal comment (`:16-18`), "Further out"
  interests paragraph, location line.
- **Copy:** implemented **but stale.** The `CompTIA` phrase appears in **two**
  user-visible places: `description()` (`pages.rs:81`, rendered into `<meta
  description>`/`og:description` via `base.html:6,8`) and the `bio`
  (`pages.rs:92`, rendered into the body). The 2026-08-02 cert re-lock (Network+
  dropped; spine now RHCSA → CCNA → Security+) makes "CompTIA study" / "working
  through the CompTIA stack" misleading — the exact defect criterion 1D calls out.
- **Tests:** implemented **but blind to the defect.** `pages.rs:204-223` asserts
  structure and anti-overclaim negatives against an *injected* bio, so it never
  reads the real bio or `description()` where the stale claim lives (5C).
- **CSS:** implemented. `.bio`, `.about-list`, `.about-list li`, `.about-list
  strong` (`style.css:957-982`). `.bio-loc` is a **class hook with no CSS rule**
  (confirmed — A1 T8 lists it among unstyled hooks). `.about-list li` uses a
  `0.875rem` font-size literal and `.bio` a `65ch` measure literal (A1 T4 flags
  both).
- **Evidence links:** **absent.** No capability in "What I work with" links to the
  writeup that evidences it; the list is unlinked assertion.
- **Nav highlight:** implemented and correct — `section()` returns `"about"`
  (`pages.rs:83`), matched at `base.html:24`.

### 7.2 Delta to spec

**Modified files:**

- `src/handlers/pages.rs`
  - `description()` `:81` — remove `CompTIA study`; replace with the cert-agnostic
    capability string (§4.2).
  - `about()` `:91-93` — remove `working through the CompTIA stack`; replace with
    the capability-led bio (§4.2).
  - `about()` tests `:204-223` — update the injected-bio fixture to drop `CompTIA`;
    add T-B2-1..5 and I-B2-1..2.
  - `section()` `:83` — change return type to `Section::About` **when A2 lands**
    (cross-feature, not standalone in B2).
- `templates/about.html`
  - Add inline evidence links to the "What I work with" items whose writeups exist
    (Networking → hosting/migration posts; Security → security-headers post;
    Systems → `/portfolio`), with self-describing link text (§4.2, §3.7).
  - (Optional, Q1) consider `<ul>`→`<dl>`; default is keep `<ul>`.

**New files / modules:** none.
**Migrations / schema:** none.
**New dependencies:** none.

### 7.3 Estimated scope

**S (small).** The copy fix is two string edits plus five new tests. The evidence
links are a handful of template edits gated by one test. No new file, no
dependency, no migration, no CSS beyond what A1 already owns. The care required is
in *wording* (claim discipline) and in *not weakening* the existing guard — the
diff is small; the review is where the value is.

### 7.4 Blocking dependencies

- **A1 (design-system)** — non-blocking for the copy fix. Cross-feature request
  **CFR-1:** move `.about-list li`'s `0.875rem` and `.bio`'s `65ch` into A1's
  measurement-layer tokens (`--text-sm`, `--measure-narrow`); decide `.bio-loc`
  (give it a muted small rule, or drop the class) under A1's unstyled-hook cleanup
  (A1 T8/T9). B2 can ship the copy fix before these land.
- **A2 (site-shell)** — non-blocking for the copy fix. **CFR-2:** when A2's
  `Section` enum lands, about's `section()` returns `Section::About` and picks up
  `aria-current="page"` from the shell's nav loop. Until then the current `&str`
  path works.
- **B1 (home)** — **coupled.** The identical stale `CompTIA study` string lives at
  `pages.rs:44` (home `description()`), and the home test `assert!(html.contains
  ("CompTIA"))` (`pages.rs:158`) passes **only** via that meta tag (the 5C
  coupling criterion documents). **CFR-3:** B1 must fix its own `CompTIA`
  occurrence and repair that test so it either asserts against real body copy or
  drops the retired claim. B2 does **not** touch `pages.rs:44` or `:158` — but a
  site-wide `grep CompTIA` should return **zero** hits only after both B1 and B2
  land, so the two changes should be sequenced together to avoid a half-corrected
  site.
- **External docs (5E):** `docs/public-portfolio-structure.md` still carries the
  five-pillar cert-journey framing and a two-revisions-stale spine (`:9`, `:32`,
  `:59`); `mg-coreforge/PUBLIC_FACE.md` is the wording authority the about comment
  points to. When the about copy changes, `public-portfolio-structure.md` should
  be updated in the same change to stop describing a certification pillar the site
  no longer has. (README `:9-16` is already correct — "infrastructure technician,"
  cert claims removed — and needs no change.)
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d

---

## 8. Open Questions

<<<<<<< HEAD
- **Q1 — Should the about page name RHCSA? (Blocks: §6.3.2, the wording of
  `ABOUT_BIO`.)** Two governing documents conflict. `criteria.md` auto-fail rule
  1 bans a certification claim without a booked voucher, and none is booked
  (`CERT_PLAN.md:143-145` gates the booking behind a measured baseline that does
  not yet exist). `PUBLIC_FACE.md:15-20`, amended 2026-08-03, permits naming
  RHCSA *"as actively studying, provided the copy also says it is not earned and
  no date is booked."*
  **B2's default is to name no exam** — it satisfies both readings and cannot
  auto-fail. The alternative, if Jeff signs off explicitly, is one added sentence:
  *"I'm studying for the RHCSA — not earned, and no date booked yet."* That
  wording is deliberately the minimum that satisfies `PUBLIC_FACE.md:19` verbatim,
  and **U-B2-3 enforces it mechanically**, so the option cannot be taken
  half-way. *The trade:* naming it converts a silence into a concrete, checkable
  signal of direction for a hiring manager, at the cost of putting an exam noun
  on a page whose whole value is that it never reaches. **A spec agent should not
  make this call; it is a claim-policy decision with Jeff's name on it.**

- **Q2 — Should `/about` state the target role at all? (Blocks: §6.3.4.)**
  §6.3.4 resolves the `public-portfolio-structure.md:98` vs `pages.rs:217`
  conflict by claiming no title, which is the strongest position available and
  is what `PUBLIC_FACE.md:53-55` recommends. But `PUBLIC_FACE.md:39` also offers
  a headline it calls honest — *"Infrastructure technician — Linux, networking,
  and virtualization, documented at machinageist.dev"* — and a hiring manager
  scanning for a role match currently has to infer one. Options: (a) no title,
  B2's default; (b) add that exact sentence, which introduces a title but a
  conservative one blessed by the governing doc; (c) leave `/about` title-free
  and put role targeting in a future `/start-here` page, which
  `public-portfolio-structure.md:128` already believes exists (B2-11).
  *B2 leans (a),* on the grounds that the corrected copy makes the role obvious
  from the work.

- **Q3 — Does removing seven nouns make the page too thin? (Blocks: §6.3.3.)**
  The corrected "What I work with" is materially shorter and names four
  capabilities where it previously named eleven. That is the honest count, and
  §1.3's success signal treats it as a feature. But an about page can be *so*
  spare that it reads as thin rather than as disciplined. The additive fix, if
  Jeff wants one, is not to restore claims but to make each surviving bullet
  carry its own evidence link — `/blog/hosting-machinageist-dev` for Systems and
  Networking, `/blog/security-headers-on-machinageist-dev` for Security,
  `github.com/machinageist/mg-server` for Automation. That trades §6.3.3's
  single-link restraint for four inline links. *B2's default is the single link*
  (Lens 2E budgets spectacle to chrome and keeps body copy quiet); four links is
  a legitimate alternative if the thirty-second reviewer test says otherwise.

- **Q4 — Should U-B2-4's banned-word list live in one shared place? (Blocks:
  §5.1.)** U-B2-4 bans `"monitoring"`, `"backup"`, `"VLAN"`, `"auth-log"`,
  `"health check"` on the about page. A2's U-7 bans retired *claim* strings across
  every page's `description()`. `project.rs:109-115` and `lab.rs:258-269` each
  carry their own list. Four lists now encode one policy, which is a criterion 5A
  single-source-of-truth question. Consolidating into one `claims.rs` module would
  be cleaner, but it would also make every one of those tests depend on a shared
  file — and the current duplication is *deliberate* in at least one case (the
  `SIDEBAR`/`WIKI_SLUGS` split precedent, criteria 5A). *B2 keeps the list local*
  and flags the consolidation as a decision for whoever owns the claim-policy
  layer, because a per-page list is readable at the point of failure and a shared
  one is not.

- **Q5 — When one of the five banned capabilities becomes real, what is the
  ritual? (Blocks: nothing; it is the maintenance contract.)** B2's answer, which
  should be written into whichever long-lived doc §5E identifies: the same commit
  must (a) publish or update the post carrying the evidence, (b) update the about
  copy, and (c) remove the word from U-B2-4's list with a comment naming the post
  and line that now backs it. Any one of the three alone is a regression. This is
  the same shape as the `9b5b1a1` + `about.html:16-18` precedent, generalised.

---

**Verification commands for this feature** (all five run in CI today —
`.github/workflows/ci.yml`; the theme check runs *first*, before the Rust steps):

```sh
python3 docs/themes/generate_themes.py --check    # currently: exit 0, 23 themes clear
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets                          # baseline: 30 unit + 2 integration, all passing
cargo build --release
```

Plus the manual pass in §5.4, minimally: Lunarcore and Solarcore, JavaScript
disabled, `prefers-reduced-motion: reduce`, 320px at 400% zoom, browser default
font at 24px — and the acceptance read: **`/about` followed by
`/blog/hosting-machinageist-dev`, checking that no sentence in either contradicts
the other.**

**Documents that must be updated in the same change (criterion 5E):**

- `docs/public-portfolio-structure.md` — §"Public claim discipline" (`:94-113`)
  is the standard the corrected copy is written against, and its
  `:98` "(in training)" line is now contradicted by a test in the same repo
  (B2-07). It must record that `/about` claims no title at all and why. Its
  amendment header (`:9`) must also be re-dated to the 2026-08-02 spine, and its
  `:128` claim that a `/start-here` route exists must be corrected — that route
  is absent from `router.rs:37-58` (B2-11).
- `templates/about.html:16-18` — the in-template claim-boundary comment, amended
  with the dated 2026-08-08 line specified in §6.3.2. This is the repo's own
  convention for recording a claim-boundary change and is not optional.
- `docs/agent-context/README.md` — **does not exist**, despite being referenced by
  the global `CLAUDE.md` index. A2 §5E made the same observation. If it is
  created, the durable constraint from this spec belongs in it: *the about page
  names only capabilities a published post can back, and U-B2-4 is what enforces
  that.*
=======
- **Q1 (semantics):** Keep the capability list as `<ul>` with `<strong>` labels
  (spec default), or upgrade to a `<dl>`/`<dt>`/`<dd>` for term-by-term AT
  navigation? — blocks: §3.7, §4.2. Default: keep `<ul>` (matches A1's
  site-wide "definition-style list" pattern; the upgrade is a marginal, optional
  a11y gain).
- **Q2 (copy wording):** The exact replacement strings for `description()` and
  `bio` are Jeff's to set (copy is his). The spec fixes the *constraints* (no cert
  name, capability-led, ≤160 chars, no retired claim); the proposed strings in
  §4.2 are candidates. — blocks: §4.2 final wording.
- **Q3 (link target):** Confirm the exact slug of the network-migration writeup
  (`content/posts/…`) before adding its evidence link — criteria 4B name it as the
  in-repo model but the slug must be verified so T-B2-4 does not fail on a typo. —
  blocks: §4.2 evidence links.
- **Q4 (architecture, minor):** Keep `bio` as a handler-set `String` field, or
  inline it as a template literal / `const`? Spec default: keep the field (copy in
  Rust, unit-testable), with T-B2-1 guarding both it and `description()` together.
  — blocks: nothing; recorded for tidiness.
- **Q5 (sequencing):** B2's `CompTIA` fix and B1's (`pages.rs:44`/`:158`, CFR-3)
  should land together so the site is never half-corrected. Confirm they are
  scheduled as one change or adjacent changes. — blocks: §7.4.

**Sub-feature needs:** none. About is a leaf feature with no children; nothing here
requires spawning additional spec work beyond the cross-feature requests filed in
§7.4.
>>>>>>> fc3da33a5d4c8cbf00d88e1525dfb5b22075f68d
