# Spec: About

**Feature ID:** `B2` / `about`
**Parent feature:** root (Content surfaces, existing)
**Spec author agent:** Spec Gauntlet agent 5 (Claude Opus 4.8)
**Date:** 2026-08-08
**Iteration:** 1

---

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

**Out of scope, referenced only:**

| Concern | Owner | What B2 assumes |
|---|---|---|
| `<head>`, `<meta>`, nav active-state, skip link, footer, theme control | `A2` site-shell | About supplies `title()`/`description()`/`section()`; A2 renders them and owns the `&str → Section` change and `aria-current` |
| Colour/font tokens, type-scale tokens, contrast audit, measurement layer | `A1` design-system | `--text`, `--text-muted`, `--border-subtle` exist and are AA-audited per theme; size literals in about's CSS are A1's to tokenise |
| The identical stale `CompTIA study` string on the home page (`pages.rs:44`) and its test coupling (`pages.rs:158`) | `B1` home | B2 fixes only the two about occurrences; the home one is filed as a cross-feature request (§7.4) |
| Portfolio entry list and Writing post list that about will link to | `B3` portfolio / `B4` writing | About links to their public routes (`/portfolio`, `/blog/:slug`), which are stable |

Where about needs a change inside another feature's territory it is filed as a
**cross-feature request** in §7.4, not specified here.

---

## 1. Purpose

### 1.1 One-sentence job

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

---

## 2. User Stories

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

---

## 3. UX Specification

### 3.1 Screen / view inventory

| Surface | Path to reach | New / modified | Layout pattern |
|---|---|---|---|
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

---

## 4. Implementation Specification

### 4.1 Architecture placement

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

---

## 5. Test Specification

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

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement.**

About stores nothing, reads no request data, sets no cookie, and takes no input.
Its only "data" is static biographical copy the operator chose to publish
(name, city, capabilities). "Portland, OR." is city-level only — a2c096a
("fix: keep location copy city-only") already pinned that; the spec preserves it
and does not add a street address, employer, or contact detail.

### 6.2 Asset provenance

- [x] **No third-party assets.**

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

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

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

---

## 8. Open Questions

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
