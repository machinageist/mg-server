# Spec: Site Shell

**Feature ID:** `A2` / `site-shell`
**Parent feature:** root (Foundation)
**Spec author agent:** spec-agent-2 (Claude Opus 5)
**Date:** 2026-08-07
**Iteration:** 1

---

## 0. Reading notes and scope boundary

Everything asserted about current state below was read from source, not from docs.
Citations are `path:line-range`.

**In scope (A2 owns):** `<head>` and metadata contract, skip link, header, brand
mark placement, primary nav and its active-section mechanism, the theme selector
control (behaviour, semantics, no-JS fallback), `<main>`, footer, the vitals strip
*presentation*, the themed 404/500 pages, and the JS surface (`main.js`,
`theme-init.js`).

**Out of scope, referenced only:**

| Concern | Owner | What A2 assumes from it |
|---|---|---|
| Token architecture, palettes, 23-theme roster, contrast audit, brand-mark artwork | `A1` design-system | `--bg/--surface/--text/--text-muted/--text-faint/--border/--border-subtle/--accent/--accent-hover/--code/--shadow/--font-body/--font-mono` exist and are AA-validated per theme; `:root` bare = Lunarcore (`style.css:16-33`) |
| Vitals *data* (middleware counters, `/status`, `/status.json`, security headers, rate limiting, `robots.txt`, `security.txt`) | `A3` ops | `crate::state::Status::current()` returns a null-safe snapshot; CSP is `script-src 'self'` (`security_headers.rs:41-50`) |
| Page bodies and their copy (`hero`, bios, post lists) | `B1`–`B6` | Each supplies `title()` / `description()` / `section()` |

Where A2 needs a change inside another feature's territory, it is filed as a
**cross-feature request** in §7.4 rather than specified here.

---

## 1. Purpose

### 1.1 One-sentence job

Give every page on machinageist.dev the same trustworthy frame — identity,
orientation, a way out, and an honest machine readout — so a reader can land on
any URL and immediately know whose site this is, where they are in it, where else
to go, and that the thing serving them is a real process someone operates.

### 1.2 Why it matters

The shell is the only component every visitor sees on every request, including
the ones who arrive at a 404. Three specific pains:

1. **Orientation cost for a skimming reviewer.** A hiring manager arriving at a
   deep link (`/blog/network-migration`, `/learn/osi-model`) from a resume has no
   idea what the rest of the site is. The shell is the entire answer — the nav is
   the site map. Competitor junior portfolios (Lens 4 table) typically bury this
   behind a hamburger or omit it on article pages.
2. **Proof-of-operation.** The vitals strip is the site's thirty-second
   differentiator (§4A). Uptime, request count, resident memory, version, and
   build timestamp are read from the live process at render time
   (`vitals_strip.html:6`, `state.rs:239-254`) with **no polling and no
   JavaScript**. Every other candidate portfolio in the competitor set shows a
   screenshot of a dashboard; this one *is* the dashboard, and it costs zero
   bytes of JS. That is a claim that cannot be faked by copying a template.
3. **Reachability floor.** The site's stated identity is ~95 lines of JavaScript
   total (`main.js` 80 lines, `theme-init.js` 15 lines), theme selector only. If
   the shell ever depends on JS for navigation, reading, or error recovery, the
   product loses the thing that makes it interesting to the engineer-peer reader.

**What the shell carries, and what it does not.** Criterion 4B asks a feature to
surface *verification, failure, and recovery* rather than green screenshots. The
shell carries exactly two of those three, and it should not be read as claiming
the rest:

| 4B dimension | Shell's share | Owner of the rest |
|---|---|---|
| **Operation** | The whole of it. The vitals strip is a live process readout, not a screenshot (§1.2 point 2). | — |
| **Recovery** | The whole of it. E-01/E-02 keep working navigation on a 404 and a 500, so a failed request is a detour rather than a dead end. | — |
| **Failure** | **None.** The shell renders failure *states* but publishes no account of a failure. | `B4` writing — the network-migration post is the in-repo model |
| **Verification** | **None.** Nothing the shell displays is a claim under test in front of the reader. | `B3` portfolio and `B4` writing |

Stating this matters because the vitals strip is persuasive enough to be mistaken
for the site's whole evidence story. It is not. It is proof that a process is
running; the proof that its operator can diagnose, fix, and verify lives in the
page bodies the shell frames.

### 1.3 Success signal

**Primary (measurable):** with JavaScript fully disabled, each of the **nine
HTML routes** registered in `router.rs:37-57` (`/`, `/about`, `/portfolio`,
`/blog`, `/blog/:slug`, `/learn`, `/learn/:slug`, `/releases`, `/status`) plus
the 404 and the 500 renders complete, readable, navigable HTML, in a colour
scheme that respects the visitor's OS light/dark preference, with no control on
screen that does nothing when pressed. Verified by `cargo test --all-targets`
(the shell contract tests in §5.1–5.2, which parse the *served bytes*, not a
browser) plus one manual pass with JS off.

The other six registered routes are deliberately excluded: `/wiki` and
`/wiki/:slug` are 3xx redirects, `/status.json` is JSON, and `/robots.txt`,
`/security.txt` and `/.well-known/security.txt` are `text/plain`. None of them
render the shell and none of them should.

**Secondary (observable):** a keyboard-only user reaches the **skip link on the
first Tab press** and leaves the header chrome entirely in **two** — that is the
number the skip link exists to make true, and it is the one worth measuring. The
furthest nav destination (Learn) is the **sixth** Tab stop by the §3.4 order
table (skip link → brand → About → Portfolio → Writing → Learn); a four-item nav
cannot be shorter than that and shortening it is not a goal. The current section
is announced by a screen reader without depending on colour.

---

## 2. User Stories

> As a **hiring manager** landing on a deep link from a resume, I want the header
> to tell me whose site this is and what else is on it, so that I can get to
> Portfolio and Writing in one click without going back to the resume.

> As an **engineer peer** skimming for signal, I want the footer to show me real
> process vitals and a source link, so that I can tell in seconds that this site
> is self-operated rather than a template on a static host.

> As a **keyboard-only user**, I want a skip link that visibly lands me in the
> content and a theme menu whose arrow keys, Home/End, Escape and Tab behave like
> every other menu I have used, so that I never have to tab through 24 theme
> buttons to reach the page.

> As a **screen reader user**, I want the current nav section announced as the
> current page and the footer readout announced with expanded labels rather than
> "UP 00:14:32 REQ 1204", so that I get the same orientation a sighted user gets.

> As a **reader with JavaScript disabled** (text browser, corporate lockdown,
> privacy extension, storage blocked), I want the site to render in my OS colour
> preference and to not present a theme button that silently does nothing, so
> that the page is honest about what it can do for me.

> As a **visitor who mistyped a URL or followed a dead link**, I want a 404 that
> tells me what I asked for, keeps the site navigation available, and offers a way
> home, so that a wrong URL is a detour rather than a dead end.

> As **the operator (Jeff)**, I want the footer build stamp and version to come
> from the binary that is actually answering the request, so that I can confirm a
> deploy landed by loading any page instead of SSH'ing to the box.

> As a **self-directed learner** arriving at `/learn/osi-model` from a search
> result, I want to see immediately that this page is one entry in a curriculum
> rather than a stray article — where it sits, what is adjacent to it, and how to
> get back to the index — so that I can keep reading instead of bouncing back to
> the search results. The shell owes this reader three things: the `Learn` nav
> item rendered as the current section (`aria-current="page"`), a
> `contentinfo`-independent route back to `/learn`, and a correctly named
> secondary navigation landmark for the curriculum list. That third one is the
> `/learn` sidebar (`wiki_page.html:4-7`), whose ownership §3.1 settles.

> As a **reader on a 360px phone at 200% text zoom**, I want the header to wrap
> rather than clip and the theme menu to stay on screen, so that the site is
> usable on the device most inbound links get opened on.

---

## 3. UX Specification

### 3.1 Screen / view inventory

The shell introduces no standalone screens; it modifies **every** screen. The
enumerable surfaces it owns:

`router.rs:37-57` registers **15** routes. **Nine** of them render this shell:
`/`, `/about`, `/portfolio`, `/blog`, `/blog/:slug`, `/learn`, `/learn/:slug`,
`/releases`, `/status`. `/wiki` and `/wiki/:slug` are 3xx redirects
(`router.rs:44-45`), `/status.json` is JSON, and `/robots.txt`, `/security.txt`
and `/.well-known/security.txt` are `text/plain`. Every count below is against
those nine plus the 404 and the 500 — eleven shell-rendering responses.

| Surface | Reached by | New / modified | Layout pattern |
|---|---|---|---|
| **Header chrome** | All 9 HTML routes + 404 + 500 | Modification (exists, `base.html:19-83`) | Full-bleed **sticky** bar (`position: sticky; top: 0; z-index: 50`, opaque `--bg`), 1px bottom border, 900px centred inner row (`style.css:566-573`) |
| **Primary nav** | Inside header | Modification | Horizontal link row, wraps at ≤640px (`style.css:1535-1547`) |
| **Theme menu (popover)** | Header → theme button (`◐`) | Modification | Absolutely-positioned panel, right-aligned, `max-height: min(72vh, 30rem)` scrolling (`style.css:644-661`) |
| **`<main>` content region** | All shell routes | Modification (`base.html:85-87`) | 900px column, `flex: 1`; widens to 1200px under `main:has(.wiki-layout)` (`style.css:757-767`) |
| **Footer** | All shell routes | Modification (`base.html:89-97`) | Two rows: name/source row, then vitals strip. **Sticky** to the bottom (`style.css:818-827`), un-pinning under `@media (max-height: 34rem)` (`:832-836`) |
| **Vitals strip** | Inside footer | Modification (`vitals_strip.html`) | Single wrapping line of monospace readouts |
| **404 page** | Any unmatched URL → `errors::fallback_404` (`router.rs:61`) | Modification (`error_404.html`) | Boot-log column, 700px, inside the standard shell |
| **500 page** | Any `SiteError` other than `PostNotFound`/`PageNotFound` (`errors.rs:113-123`) | Modification (`error_500.html`) | Boot-log column, 700px, inside the standard shell |
| **Static-asset 404** | `/static/<missing>` | **New** — currently falls through to `ServeDir`'s bare 404 (see §7.1) | Themed 404 (same as above) |
| **Skip link** | Tab once from page load | Modification (`base.html:17`) | Off-canvas until focused; visible treatment already shipped (`style.css:541-557`) |
| **`/learn` sidebar nav** | 13 `/learn` URLs | **Claimed by A2 as chrome** (`wiki_page.html:4-7`, `style.css:1390-1397`) | Sticky rail measured off `--header-h`/`--footer-h`; collapses to a `<details>` disclosure ≤ 800px (`style.css:1456-1521`) |
| **Home `hero-actions` nav** | `/` only | **Delegated to `B1`** (`index.html:11`) — A2 supplies the landmark-naming rule, B1 supplies the label | Inline link row inside the hero |

**The two navigation surfaces the shell does not render itself.** The site has
four navigation landmarks, not two. The primary nav and the footer nav are inside
`base.html`; the other two are in page templates. A2 claims the `/learn` sidebar
because it is chrome by every test that matters — it is identical on 13 URLs, it
is positioned against the shell's own `--header-h`/`--footer-h` tokens, and it is
a `navigation` landmark subject to the naming contract in §3.7. It delegates
`index.html:11` to `B1` because it appears on exactly one page and is hero
content, but the naming rule travels with the delegation and I-9 enforces it
site-wide either way.

Both currently violate the contract F-10 exists to establish:
`aside aria-label="Education wiki navigation"` wrapping an **unnamed** `<nav>`,
and `nav aria-label="Quick navigation"` — each double-announces exactly as
"Primary navigation" does. Target labels: the `<aside>` loses its label and the
inner `<nav>` gains `aria-label="Curriculum"`; `index.html:11` becomes
`aria-label="Quick links"`.

No modals, sheets, or drawers. The theme menu and the ≤ 800px `/learn` disclosure
are the only overlays and neither is modal (neither traps focus; the theme menu
closes on outside click at `main.js:57`, the disclosure is a native `<details>`).

### 3.2 Interaction flows

**Flow A — arrive, orient, navigate (primary, JS-independent).**

1. Request lands. Server renders the full document; `<html>` carries `lang="en"`.
2. `theme-init.js` runs synchronously in `<head>` *before first paint* and stamps
   `data-theme` on `<html>` (`theme-init.js:5-15`) → no flash of the wrong theme.
   **If it does not run** (JS off, blocked, or the storage read throws), the bare
   `:root` block plus the new OS-preference fallback (§4.2) supplies the palette.
3. First Tab reveals the skip link. Enter → focus and scroll to `<main>`.
4. Second/third Tab reach the brand and then the nav links.
5. The nav link matching `self.section()` renders with the active treatment:
   accent colour **+** a full-width 1.5px underline (`style.css:713-724`) **+**
   `aria-current="page"` (new, §4.2). Hover gets a *thinner, dimmer* underline so
   the two states differ for sighted users too — see §3.3 and F-04.
6. Click/Enter navigates. Full page load. No client-side routing, no transition
   choreography, no history manipulation.

**Flow B — change theme (enhancement layer).**

1. Click or Enter/Space on the theme button, or ArrowDown/ArrowUp while it is
   focused (`main.js:68-72`).
2. Menu unhides; focus moves to the **currently checked** item, not the first
   (`main.js:55`) — correct for a `menuitemradio` group per the ARIA APG.
3. Arrows cycle with wraparound; Home/End jump to ends (`main.js:62-65`).
   *New:* printable-character typeahead jumps to the next item whose label starts
   with that character (24 items justifies it; APG recommends it). **APG defaults,
   stated so the implementer does not invent them:** keystrokes accumulate into a
   search buffer; the buffer resets after **500 ms** of no typing; matching is
   case-insensitive against the item's *text label*, not its `data-mode` slug;
   the search starts from the item *after* the currently focused one and wraps;
   if the buffer is a single character repeated (`g`, `g`, `g`), it cycles
   through the items starting with that character instead of searching for
   `"ggg"`; no match leaves focus where it is.
4. Enter/Space/click applies: sets `data-theme`, writes `localStorage.theme`,
   re-renders the button icon and `aria-checked` state, closes, returns focus to
   the button (`main.js:25-40`, `73-75`).
5. Escape closes and returns focus to the button (`main.js:60`).
6. **Tab closes the menu.** *New behaviour:* focus moves to the theme button
   **before** the menu is hidden, so the browser's default Tab continues from the
   button. (Current code hides first — see §7.1 finding F-06.)
7. `system` mode keeps following the OS live via a `matchMedia` change listener
   (`main.js:77`).

**Flow B′ — theme with JS unavailable (the fallback).** The theme control is not
rendered as an interactive element at all; the page uses the OS preference. See
§3.6 E-05 and §4.2 for the mechanism. No dead control appears.

**Flow C — hit a 404.**

1. Unmatched URL → `fallback_404` (`errors.rs:139-141`) → `render_404(uri.path())`.
2. Response is `404 Not Found` with the full shell: header, nav, footer, vitals.
3. Body is a boot-log sequence ending in the requested path, HTML-escaped by
   Askama (proven by `errors.rs:160-169`), and `(A)bort → return home`.
4. Under `prefers-reduced-motion: no-preference` the five lines stagger in over
   1.8s with `steps(1, end)` and `fill-mode: both` (`style.css:1358-1368`) — a
   discrete type-on, not a fade. Under `reduce`, the whole block is static and
   fully visible immediately, because the "from" state lives only inside the
   animation.
5. Recovery: the header nav is live on the 404, so recovery is not limited to the
   single home link in the body.

**Flow D — hit a 500.** Identical framing; `errors.rs:113-123` logs the full
internal error and renders a template carrying no path, no version, no stack
(guarded by `errors.rs:184-193`). If even the 500 template fails to render, the
fallback is the plain string `500 internal server error` (`errors.rs:117-121`).

**Cues.** No haptics (web). No sound. Motion is limited to: colour transitions on
theme change, the nav underline sweep, the theme-menu pop (`theme-pop`, 0.16s),
the brand cursor blink, and the 404/500 boot stagger — all inside
`@media (prefers-reduced-motion: no-preference)` (`style.css:735-748`,
`1358-1368`), except the CRT scanline overlay which is instead *removed* under
`reduce` (`style.css:461-470`).

### 3.3 Layout descriptions

**Two pinned bars.** Since commit `a375a14` the header and the footer are both
`position: sticky` with an opaque `var(--bg)` background and `z-index: 50`
(`style.css:566-573`, `:818-827`). Three consequences the rest of this spec is
written against:

- `:root` carries `--header-h: 4.5rem` and `--footer-h: 7.25rem`
  (`style.css:505-506`), declared beside the type scale because they are
  measurements, not palette. The ≤ 640px block overrides them to `6.5rem` /
  `8.75rem` (`:1530-1533`) where the nav wraps to two rows and the vitals strip
  to three. Anything that must clear a pinned bar measures from these tokens
  rather than repeating a literal — the `/learn` sidebar and `#content`'s
  scroll margin both do.
- `#content` carries `scroll-margin-top: calc(var(--header-h) + 1rem)`
  (`style.css:1159-1161`), so the skip link's target does not land underneath the
  nav. That is the *scroll* half of F-14; the *focus* half still needs
  `tabindex="-1"` on `<main>`.
- The footer gives up its pin under `@media (max-height: 34rem)`
  (`style.css:832-836`). Two pinned bars on a short viewport eat the screen, and
  a colophon is worth less than the navigation. The header keeps its pin.

**Header** (`base.html:19-83`, `style.css:559-703`)

```
<header class="site-header">                 padding 1.25rem 2rem, border-bottom 1px --border
                                             position: sticky; top: 0; z-index: 50; background --bg
  ├─ <a class="brand" href="/">              leading; inline SVG mark 26×26 + wordmark
  │    ├─ svg.brand-mark                     aria-hidden focusable=false; .mk-a→--accent, .mk-b→--code
  │    └─ span.brand-word "machinageist"     700 weight, --accent, ::after ▍ blinking cursor
  └─ .nav-right                              trailing; gap 1.5rem
       ├─ <nav aria-label="Primary">         ← NEW placement: nav wraps ONLY the links
       │    └─ .nav-links                    gap 2rem; About · Portfolio · Writing · Learn
       └─ .theme-select                      position:relative anchor for the popover
            ├─ button#theme-btn              2rem square, 1px --border, radius 6px
            └─ div#theme-menu[role=menu]     hidden by default; see below
```

Data source: nav items come from a single `NAV` constant (§4.2); the active item
is decided by comparing each item's section key to `self.section()`.

**Theme menu** (`base.html:36-79`, `style.css:644-703`) — 6 `role="group"`
wrappers, 24 `[data-mode]` buttons (verified by count), each
`role="menuitemradio"` with an `aria-hidden` glyph and a text label. The visible
group heading is a `<span aria-hidden="true">`; the group's accessible name comes
from `aria-label` on the wrapper (`base.html:37`, comment at `style.css:664-665`).
Checked item gets `.is-current` → accent colour **plus** a `✓` pseudo-element
(`style.css:702-703`) — that check mark is the non-colour cue and must stay.

**Active vs hover on a nav link** (`style.css:713-724`) — today
`.nav-link:hover::after` and `.nav-link.is-active::after` set the *identical*
`right: 0`, so a sighted user cannot tell the two apart (F-04). Target: active
keeps the full-width 1.5px underline at `opacity: 0.85`; **hover** drops to
`height: 1px` and `opacity: 0.45`. That is a size and weight change, not a colour
change, so it costs nothing per theme and 2F still holds. `aria-current="page"`
covers the AT half; this covers the sighted half.

**`<main id="content">`** (`base.html:85-87`, `style.css:757-763`) — `flex: 1`,
`max-width: 900px`, `padding: 3.5rem 2rem 5rem`, centred; widening to 1200px on
`/learn` pages via `main:has(.wiki-layout)` (`:765-767`). Prose inside is further
capped at `--measure: 72ch` (`style.css:497-499`); code and Markdown tables keep
the full column and scroll inside themselves (`:1163-1176`).
*New:* `tabindex="-1"` so the skip link actually moves keyboard focus.

**`/learn` sidebar** (`wiki_page.html:4-7`, `style.css:1390-1397`) — a sticky
rail at `top: calc(var(--header-h) + 1rem)` with
`max-height: calc(100vh - var(--header-h) - var(--footer-h) - 3.5rem)`, so it
clears both pinned bars instead of scrolling its last entries under the footer.
At ≤ 800px it goes `position: static` and the inner `<nav>` collapses behind a
native `<details>` disclosure that floats over the page at `z-index: 20`
(`style.css:1456-1521`). Claimed by A2 per §3.1; its landmark naming is fixed in
§3.7 and enforced by I-9.

**Footer** (`base.html:89-97`, `style.css:811-876`)

```
<footer class="site-footer">                  padding 1.5rem 2rem, border-top 1px --border, --text-faint, 0.8rem
                                              position: sticky; bottom: 0; z-index: 50; background --bg
                                              static under @media (max-height: 34rem)
  ├─ .footer-inner                            900px, flex, space-between, wrap
  │    ├─ span  "machinageist.dev — Jeff Cincoski"
  │    └─ <nav aria-label="Footer">           a → github.com/machinageist/mg-server (rel=noopener noreferrer)
  └─ {% include "vitals_strip.html" %}        900px, border-top 1px --border-subtle, 0.75rem, letter-spacing .08em
       UP dd:hh:mm · REQ n · [MEM n MiB] · v0.1.0 · built YYYY-MM-DD HH:MM UTC → /status
```

Data source: `crate::state::Status::current()`, called *from the template*
(`vitals_strip.html:6`). `MEM` is conditional — `rss_mib` is `None` off Linux and
the `{% match %}` at `vitals_strip.html:11-16` omits the item and its separator
entirely rather than printing a blank. That is the correct empty-state behaviour
and must be preserved.

**Empty / degraded states.**

| Condition | Appearance | Copy |
|---|---|---|
| `rss_mib == None` (non-Linux dev box, `/proc` unreadable) | `MEM` item and its `·` separator both absent | — |
| `APP_STATE` unset (unit-test render, pre-`init_global`) | `UP 00:00:00 · REQ 0 · v{crate} · built {…}` | `state.rs:240-253` returns zeros, never panics |
| Build timestamp unparseable | `built unknown` | `state.rs:284-288` |
| Bind address non-standard | classified string only, never a literal address | `state.rs:209-219`; A3 territory |
| Home page posts fail to load | Shell unchanged; the *page* drops its writing section | `pages.rs:52-57`, `index.html:29-46` |

The shell has no "empty state" of its own: it always renders, on every route,
including 500. This is a deliberate resilience property and §5 pins it.

### 3.4 Input & gestures

**Pointer.** Click on brand, nav links, theme button, theme items, footer source
link, vitals `/status` link. Outside-click closes the theme menu (`main.js:57`).
The CRT scanline overlay is `pointer-events: none` (`style.css:449`) so it never
intercepts a click.

**Keyboard.**

| Key | Context | Behaviour |
|---|---|---|
| `Tab` (first press from load) | Document | Reveals skip link (already legible — `style.css:550-557`) |
| `Enter` | Skip link | Moves focus **and** scroll to `<main>`. Scroll clearance under the sticky header already ships (`style.css:1159-1161`); the focus half requires the new `tabindex="-1"` |
| `Tab` / `Shift+Tab` | Page | Standard order, and the source of §1.3's numbers: 1 skip link → 2 brand → 3 About → 4 Portfolio → 5 Writing → 6 Learn → 7 theme button → main content links → footer source → vitals `/status`. On a `/learn` page the sidebar links fall inside `<main>`, after the theme button |
| `Enter` / `Space` / `↓` / `↑` | Theme button (closed) | Opens menu, focus on checked item |
| `↓` / `↑` | Menu open | Cycle with wraparound |
| `Home` / `End` | Menu open | First / last item |
| `a`–`z`, `0`–`9` | Menu open | **New:** typeahead to next matching label |
| `Enter` / `Space` | Menu item | Apply, close, focus returns to button |
| `Esc` | Menu open | Close, focus returns to button |
| `Tab` | Menu open | Focus button, then close; Tab continues from the button |

No global/app-level shortcuts. Introducing single-key page shortcuts would trip
WCAG 2.1.4 (Character Key Shortcuts) and is explicitly declined — the typeahead
above is scoped to an open menu, which is the standard exemption.

**Specialised input:** N/A — no stylus, controller, voice, or camera input. The
shell is text and links.

**Responsive.** The site has **three** breakpoints, not one.

| Breakpoint | Behaviour | Source |
|---|---|---|
| > 800px | Header 1.25rem/2rem padding, nav-links gap 2rem, main 900px (1200px on `/learn`), `/learn` sidebar is a sticky 13rem rail | `style.css:566-573`, `610-613`, `757-767`, `1379-1397` |
| ≤ 800px | `.wiki-layout` collapses from two grid columns to one; `.wiki-sidebar` drops its sticky pin and `max-height`, swaps its right border for a bottom border; the inner `<nav>` collapses behind a `<details>` toggle whose open panel floats over the page (`z-index: 20`, `max-height: 70vh`) rather than pushing it down | `style.css:1456-1521` |
| ≤ 640px | `--header-h` → `6.5rem` and `--footer-h` → `8.75rem` (the nav wraps to two rows and the vitals strip to three, so everything measuring off them follows); header padding 1rem/1.25rem; `.site-nav` and `.nav-links` wrap; main padding 2.5rem/1.25rem; footer 1.25rem | `style.css:1527-1561`, tokens at `:1530-1533` |
| ≤ 34rem **tall** | `.site-footer` gives up its sticky pin; the header keeps its | `style.css:832-836` |
| 320px @ 400% zoom (WCAG 1.4.10 Reflow) | Header wraps to 2–3 rows; theme menu `min-width: 9.5rem` still fits; no horizontal page scroll | Requires verification, §5.4 |

*New requirement:* `body { min-height: 100vh }` (`style.css:517`) becomes
`min-height: 100svh` with a `100vh` fallback declaration first. With the footer
now `position: sticky; bottom: 0`, a `100vh` body on mobile Safari puts the
footer's resting position underneath the collapsing toolbar — the sticky change
makes this fix more load-bearing than it was, not less.

### 3.5 Transitions & animation

| What | Duration / easing | Gate |
|---|---|---|
| Body + chrome colour swap on theme change | 0.3s ease (bg/border), 0.25s ease (colour) | `prefers-reduced-motion: no-preference` (`style.css:735-741`) |
| Link/nav/brand colour | 0.18s ease | same block (`style.css:742`) |
| Nav underline sweep (`right: 100% → 0`) | 0.22s ease | same block (`style.css:744`) |
| Theme menu open (`theme-pop`: 4px rise + fade) | 0.16s ease-out | same block (`style.css:746`), keyframes at `:750` |
| Brand terminal cursor `▍` blink | 1.2s `step-end` **infinite** | same block (`style.css:747`), keyframes at `:751` |
| 404/500 boot lines | 0.2s `steps(1,end)`, staggered 0.15s→1.8s, `fill-mode: both` | `prefers-reduced-motion: no-preference` (`style.css:1358-1368`) |
| CRT scanline overlay | static texture, not motion | removed under `reduce` (`style.css:461-462`) |
| Page navigation | **None.** Full document load. No view transitions. | — |

**Reduced-motion alternative:** every rule above lives inside
`@media (prefers-reduced-motion: no-preference)`, so `reduce` yields: instant
theme swap, static underline, instantly-visible menu, **no blinking cursor**, and
a fully-visible error page with zero delay. Glow text-shadows on the seven neon
themes are also cleared under `reduce` (`style.css:461-470`).

**Constraint carried forward:** the boot-line stagger totals 1.8s, under the 5s
threshold of WCAG 2.2.2, and the blink is ~0.83 Hz, far under the 3 Hz threshold
of WCAG 2.3.1. The blink is nevertheless flagged in §3.7 as the shell's weakest
a11y point.

**Note on the pinned bars.** Neither the header nor the footer animates its pin —
`position: sticky` is layout, not motion, and it produces no transition, no
transform, and no scroll-linked effect. It therefore adds nothing to this table
and nothing new under `prefers-reduced-motion`. It does change the *stacking*
context, which is why the skip link needs `z-index: 60` (`style.css:545`) to sit
above a `z-index: 50` header.

### 3.6 Error states

| ID | Trigger | Presentation | Why that presentation | Recovery | Data loss |
|---|---|---|---|---|---|
| **E-01** | Unmatched route | **Full page**, HTTP 404, themed boot-log (`error_404.html`) | A wrong URL is a navigation event, not a transient notice; a toast would be wrong and a banner would leave a blank page under it. Full page keeps header nav available. | Header nav (4 destinations) + brand + `(A)bort → return home` | No |
| **E-02** | `SiteError::Io`, `MissingFrontmatter`, `FrontmatterParse`, `DateParse`, `InvalidPath` | **Full page**, HTTP 500, themed kernel-panic (`error_500.html`) | Same reasoning; additionally the response must carry **nothing** internal — a banner over a partial page risks leaking half-rendered content | Header nav + `reboot → return home` | No |
| **E-03** | 500 template itself fails to render | Plain-text `500 internal server error` | Last-resort; cannot depend on Askama (`errors.rs:117-121`) | Browser back | No |
| **E-04** | `/static/<missing>` | **Currently** a bare `ServeDir` 404 with an empty body (`router.rs:59`). **Target:** the themed 404. | Consistency — "the shell owns every 404" is only true if it is | Header nav | No |
| **E-05** | JS unavailable / blocked / `localStorage` throws | Theme control **not rendered as an interactive element**; palette follows OS preference | A control that does nothing is worse than no control (see §3.7). Silent, not an error message — losing a colour picker is not an error worth interrupting a reader over | Read the site normally; OS preference is honoured | No |
| **E-06** | `Status::current()` with `APP_STATE` unset | Zeros, not an error | `state.rs:240-253`; the vitals strip must never be the reason a page fails | Page renders normally | No |
| **E-07** | `rss_cache` mutex poisoned | Recovers via `unwrap_or_else(into_inner)` (`state.rs:107-112`) | A metric is not worth a panic | Page renders normally | No |
| **E-08** | A page template omits `title()`/`description()`/`section()` | **Compile error.** Askama validates at build time (`pages.rs:10-12`) | The strongest possible presentation: it never reaches a user | Fix the code | N/A |
| **E-09** | `<meta name="description">` copy goes stale relative to the live cert spine | **No current signal.** Target: a shell contract test fails in CI (§5.1) | Copy drift is invisible by definition — it must fail loudly | Update copy | No |

**Data-loss risk across the whole feature: none.** The shell stores exactly one
thing (`localStorage.theme`, a colour preference) and reads it defensively.

### 3.7 Accessibility

**Landmarks and headings (target).**

| Landmark | Element | Accessible name | Where |
|---|---|---|---|
| `banner` | `<header class="site-header">` | — (implicit, one per page) | `base.html:19` |
| `navigation` (primary) | `<nav>` wrapping **only** `.nav-links` | `"Primary"` | `base.html:20` |
| `main` | `<main id="content" tabindex="-1">` | — | `base.html:85` |
| `contentinfo` | `<footer class="site-footer">` | — | `base.html:89` |
| `navigation` (footer) | `<nav>` in `.footer-inner` | `"Footer"` | `base.html:92` |
| `group` | `.vitals-strip` | `"Server vitals"` | `vitals_strip.html:7` |
| `complementary` | `<aside class="wiki-sidebar">` | — (**drop** the current `"Education wiki navigation"`; an `aside` is not a nav) | `wiki_page.html:4` |
| `navigation` (curriculum) | the `<nav>` inside `.wiki-nav` | `"Curriculum"` (**new** — currently unnamed) | `wiki_page.html:7` |
| `navigation` (hero) | `nav.hero-actions` | `"Quick links"` (**delegated to `B1`**; currently `"Quick navigation"`) | `index.html:11` |

**The naming rule this table establishes.** A `navigation` landmark's accessible
name must not end in the word "navigation" — a screen reader already announces
the role, so `aria-label="Primary navigation"` reads as "Primary navigation,
navigation". That is F-10. Three surfaces violate it today and only one of them
is inside `base.html`, which is why the rule is enforced by a test (**I-9**)
rather than by a one-time edit. Second half of the rule: at most one *unnamed*
`navigation` landmark per page, so a page with two navs cannot leave both
anonymous.

Heading outline: the shell contributes **no** headings; each page supplies
exactly one `<h1>` (verified: all **10 non-shell templates** — 8 content plus
`error_404.html` and `error_500.html` — have exactly one; `base.html` and
`vitals_strip.html` have zero). §5.1 pins this so a future page cannot ship with
zero or two. Note that the `/learn` sidebar's per-section `<h2>`s
(`wiki_page.html:10`) sit inside `<main>` and are part of the page outline, not
the shell's.

**Per-element AT contract.**

| Element | Role | Name | State / properties |
|---|---|---|---|
| Skip link | `link` | `"Skip to content"` | Visible only on focus, with a solid `--surface` background, padding, border and `z-index: 60` — **already shipped** in `a375a14` (`style.css:541-557`). F-02 is resolved; **preserve it**, and keep `z-index` above the header's `50` |
| Brand SVG | — | — | `aria-hidden="true" focusable="false"` (correct today, `base.html:21`) |
| Brand link | `link` | `"machinageist"` | Decorative `▍` is a CSS `::after` (`style.css:727`) → never in the a11y tree |
| Nav link (inactive) | `link` | `"About"` etc. | — |
| Nav link (active) | `link` | same | **`aria-current="page"`** (new) + accent colour + full-weight underline (hover drops to 1px / 0.45 opacity, §3.3) |
| Theme button | `button` | **`"Theme: Lunarcore"`** (new — currently the static `"Theme"`, `base.html:30`) | `aria-haspopup="menu"`, `aria-expanded` toggled by `main.js:55-56` |
| Theme menu | `menu` | `"Theme"` | `hidden` toggled; not modal |
| Theme group | `group` | `"Core"` / `"Editor"` / … | Visible label is `aria-hidden` (`base.html:38`) — correct, avoids double-announcing |
| Theme item | `menuitemradio` | `"Lunarcore"` etc. | `aria-checked`, `tabindex="-1"` (roving, `main.js:43`) |
| Vitals items | — | — | **New:** each carries a visually-hidden expansion (`"Uptime"`, `"Requests served"`, `"Resident memory"`) so `"UP 00:14:32"` is not read as two opaque tokens |
| Vitals link | `link` | **New:** `"Full status — version 0.1.0, built 2026-08-07 14:22 UTC"` | Currently the name is the bare version string, which does not say where the link goes |

**Custom actions:** N/A — no composite widget beyond the theme menu, which is
covered by the APG button-menu pattern.

**Text scaling / dynamic type.** Root type is `15px` on `body` (`style.css:520`)
with a 1.125-ratio `rem` scale, `--text-xs` … `--text-2xl` (`style.css:480-507`)
— sizes are `rem`, so browser font-size settings scale them. Two carried
requirements: at 200% zoom the header must wrap rather than clip (already true
via `style.css:1535-1540`), and at 400% zoom on a 320px viewport the page must
not scroll horizontally (§5.4).

**The shell's chrome does not yet use that scale.** Every text size A2 owns is a
hard-coded literal that predates the scale commit: `.nav-link` `0.875rem`
(`:617`), `.brand` `1rem` (`:590`), `.theme-menu button` `0.8rem` (`:696`),
`.theme-group-label` `0.65rem` (`:681`), `.site-footer` `0.8rem` (`:822`),
`.vitals-strip` `0.75rem` (`:863`). A type scale that the site's most-repeated
component ignores is not a system, it is a suggestion. §7.2 carries the migration
task. `--text-xs` is `0.75rem`, so `.vitals-strip` maps cleanly and
`.theme-group-label`'s `0.65rem` has **no** step — that one is an A1 request for
a new bottom step, not a licence to keep the literal.

**Colour-independent state.**

| State | Colour cue | Non-colour cue |
|---|---|---|
| Active nav section | `--accent` (`style.css:622`) | Full-width **1.5px / 0.85 opacity** underline (`style.css:713-724`) **+ `aria-current="page"`** (new) |
| Hover nav | `--text` (`style.css:621`) | Full-width **1px / 0.45 opacity** underline (new — today it is byte-identical to active, F-04) |
| Checked theme | `--accent` (`style.css:702`) | `✓` pseudo-element (`style.css:703`) + `aria-checked` |
| Menu open | — | `aria-expanded` |
| Focus | `--accent` outline | 2px outline + 2px offset (`style.css:710`) |

**Focus order and keyboard navigability.** DOM order is already correct: skip
link (`:17`) → brand (`:21`) → nav links (`:24-27`) → theme button (`:30`) →
main (`:85`) → footer (`:89`). No `tabindex > 0` anywhere. No focus trap. Two
target fixes remain: `<main tabindex="-1">` (F-14) and the Tab-close
focus-restore ordering (F-06). The third — skip-link legibility (F-02) — shipped
in `a375a14` and is now a preservation requirement rather than a change.

**Known residual risk (stated, not hidden):** `.brand::after` blinks forever
(`style.css:747`, `751`). WCAG 2.2.2 asks for a mechanism to pause/stop/hide
blinking that lasts more than five seconds. The current mechanism is
`prefers-reduced-motion`, which is a legitimate and widely-accepted user-agent
mechanism, and the blink is a 1-character `▍` at 1rem, not body content. This
spec **keeps** the blink (it is the wordmark's whole personality and Lens 2E
budgets spectacle to chrome) and records the reasoning here rather than
pretending the question does not exist. Open question Q3 offers the alternative.

---

## 4. Implementation Specification

### 4.1 Architecture placement

```
src/
  router.rs               ← ServeDir not_found_service wiring (E-04)
  errors.rs               ← Error404Template / Error500Template already here
  shell.rs                ← NEW: Section enum, NavItem, NAV, asset_version()
  state.rs                ← unchanged (A3 owns); read from templates
  handlers/{pages,blog,wiki,releases,status}.rs
                          ← section() return type changes &str → Section
templates/
  base.html               ← head contract, nav loop, landmarks, main tabindex, og_type block
  vitals_strip.html       ← group role + visually-hidden labels
  wiki_page.html          ← landmark naming; og:type = article
  error_404.html          ← unchanged
  error_500.html          ← unchanged
static/
  css/style.css           ← OS-preference fallback, .vh utility, hover≠active underline,
                            --text-* migration for the shell chrome
  js/main.js              ← typeahead, Tab focus order, aria-label sync
  js/theme-init.js        ← narrow the try/catch, set a JS-present flag
build.rs                  ← already stamps BUILD_TS (state.rs:34)
docs/themes/generate_themes.py
                          ← emit_menu() already group-aware (MENU_GROUPS at 281-289,
                            emit_menu at 292-318, internal drift guard at 296-304);
                            add a test that compares its output to the shipped base.html
```

`src/shell.rs` is a new module and is the *only* new file. It is justified because
four separate concerns (nav definition, section identity, asset versioning, and
the metadata contract) are currently hand-duplicated across `base.html` and six
handler files with nothing keeping them honest.

**Where the shell's tests live, and why it is not `tests/`.** `mg-server` is a
**binary-only** crate. There is no `src/lib.rs`, and `src/main.rs:16-21` declares
`errors`, `handlers`, `middleware`, `models`, `router` and `state` as private
modules. Nothing under `tests/` can name `crate::router::build` or
`crate::shell::NAV` — an integration test compiles as a separate crate against a
library target this project does not have. That is not an inconvenience to work
around; it is the reason `tests/wiki_pages.rs` re-declares `WIKI_SLUGS` with the
comment *"duplicated here on purpose so the test crate stays decoupled from the
bin"* (`tests/wiki_pages.rs:12-14`), and it is why every router-level test in the
repo already lives in a `#[cfg(test)]` module inside `src/` (`errors.rs:143-193`,
`handlers/status.rs:60-135`).

So **`Section`, `NAV` and every shell test follow the bin-internal convention,
not the `WIKI_SLUGS` decoupling convention** — deliberately, and the distinction
is worth naming because criterion 5A asks for it. `WIKI_SLUGS` is duplicated
because its test asserts a property of *files on disk* (every `content/pages/*.md`
has a sidebar entry) and gains nothing from the bin. The shell's tests assert
properties of *the built router* and gain nothing without it. Different tests,
different homes, one rule: a test lives wherever it can reach its subject without
a second copy of the truth. §5.1 and §5.2 place every test accordingly and
§7.2 lists no `tests/` file at all.

### 4.2 Data model

```rust
// Author:      machinageist
// Date:        2026-08-07
// Description: The site shell's contract — the section identity every page
//              declares, the primary nav built from it, and the asset version
//              stamped onto every static URL. One definition each, so the nav,
//              the active-state logic, and the cache-buster cannot drift.
// Notes:       Section is an enum rather than a &str because base.html compares
//              it to the nav table; a string typo silently disabled highlighting
//              (that is exactly how /learn ended up matching "wiki").

use std::fmt;

// -----------------------------------------------------------------------
// Section identity — what every page template declares about where it lives
// -----------------------------------------------------------------------

// Every top-level area of the site; pages return one of these from section()
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Home,
    About,
    Portfolio,
    Writing,
    Learn,
    Releases,
    Status,
    Error,
}

impl fmt::Display for Section {
    // Render the stable slug used in markup and tests
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let slug = match self {
            Section::Home => "home",
            Section::About => "about",
            Section::Portfolio => "portfolio",
            Section::Writing => "writing",
            Section::Learn => "learn",
            Section::Releases => "releases",
            Section::Status => "status",
            Section::Error => "error",
        };
        f.write_str(slug)
    }
}

// -----------------------------------------------------------------------
// Primary navigation — the single definition the header renders from
// -----------------------------------------------------------------------

// One entry in the header nav
pub struct NavItem {
    pub href: &'static str,
    pub label: &'static str,
    pub section: Section,
}

// The primary nav, in display order — adding a top-level area means adding a row
pub const NAV: &[NavItem] = &[
    NavItem { href: "/about",     label: "About",     section: Section::About },
    NavItem { href: "/portfolio", label: "Portfolio", section: Section::Portfolio },
    NavItem { href: "/blog",      label: "Writing",   section: Section::Writing },
    NavItem { href: "/learn",     label: "Learn",     section: Section::Learn },
];

// -----------------------------------------------------------------------
// Canonical URLs — the origin every absolute URL in <head> is built from
// -----------------------------------------------------------------------

// Public origin, no trailing slash; joined to canonical_path() for og:url
pub const SITE_ORIGIN: &str = "https://machinageist.dev";

// -----------------------------------------------------------------------
// Asset versioning — one cache-buster, derived, never hand-edited
// -----------------------------------------------------------------------

// Return the query-string value appended to every versioned static URL
// Reads state.rs's single BUILD_TS_EPOCH rather than calling env!() a second
// time — see the note below on why a second env! site would be wrong here
pub fn asset_version() -> &'static str {
    crate::state::BUILD_TS_EPOCH
}
```

**One `env!("BUILD_TS")`, not two.** `state.rs:34` already declares
`const BUILD_TS_EPOCH: &str = env!("BUILD_TS");`. A second declaration inside a
module whose stated purpose is removing hand-duplication would be an own goal, so
`state.rs` widens that constant to `pub(crate)` and `shell.rs` reads it. The two
sites could not in fact drift — `env!` resolves the same compile-time variable in
both — but "cannot drift" is not the test 5A applies. The test is whether a
reader can find the one definition, and two `env!` sites make the answer no.
`state.rs` keeps ownership because `build.rs` stamps the variable for the vitals
strip; `shell.rs` is the consumer.

`base.html` then renders the nav as a loop instead of four hand-written
conditionals:

```jinja
<nav aria-label="Primary">
  <div class="nav-links">
    {% for item in crate::shell::NAV %}
    {% if item.section == self.section() %}
    <a href="{{ item.href }}" class="nav-link is-active" aria-current="page">{{ item.label }}</a>
    {% else %}
    <a href="{{ item.href }}" class="nav-link">{{ item.label }}</a>
    {% endif %}
    {% endfor %}
  </div>
</nav>
```

**No database. No migrations.** The site has no persistence layer; the only client
storage is `localStorage.theme` (a string from a 24-value allowlist,
`theme-init.js:6`, `main.js:12`).

**CSS additions** (values owned by A1; A2 specifies only the mechanism):

```css
/* No-JS / storage-blocked fallback: honour the OS preference when theme-init.js
   never got to stamp data-theme. :not([data-theme]) stops matching the moment it
   does, so this never fights the selector. */
@media (prefers-color-scheme: light) {
    :root:not([data-theme]) { /* Solarcore token block — A1 owns the values */ }
}

/* The theme control is an enhancement; hide it until theme-init.js proves JS ran.
   Set before first paint, so there is no flash of a control that then vanishes. */
:root:not([data-js]) .theme-select { display: none; }

/* Visually-hidden utility for the vitals expansions and link labels */
.vh {
    position: absolute; width: 1px; height: 1px;
    margin: -1px; padding: 0; overflow: hidden;
    clip-path: inset(50%); white-space: nowrap;
}

/* F-04: hover and active currently draw the identical underline. Split them by
   weight and opacity, never by colour, so all 23 themes are unaffected.
   Replaces the single combined rule at style.css:724. */
.nav-link:hover::after     { right: 0; height: 1px;   opacity: 0.45; }
.nav-link.is-active::after { right: 0; height: 1.5px; opacity: 0.85; }
```

**Not in this list, because it already shipped.** Commit `a375a14` pinned the
header and footer and rewrote `.skip-link:focus` with a `--surface` background,
padding, a 1px `--accent` border and `z-index: 60` (`style.css:541-557`). The
first iteration of this spec asked for that work under F-02; it is done. What A2
now owes the skip link is a *preservation* requirement (§3.7) and one carried
constraint: the skip link's `z-index` must stay above the header's `50`, so any
future change to either number is a change to both.

### 4.3 API contracts

The shell exposes no HTTP endpoints of its own. Its contracts are template-level.

**Contract S-1 — the page metadata contract (compile-enforced).** Every Askama
struct rendered through `base.html` must implement:

| Method | Signature | Rendered into | Rule |
|---|---|---|---|
| `title` | `fn title(&self) -> &str` | `<title>` (`base.html:10`) and `og:title` (`:7`) | Must end in `" — machinageist"` for every page except the home page (whose title is the bare wordmark). Currently violated by `blog.rs:134-136` and `wiki.rs:110-112`. |
| `description` | `fn description(&self) -> &str` | `<meta name="description">` (`:6`) and `og:description` (`:8`) | Length bounded per U-6a/U-6b below. **Must be treated as user-visible copy** and therefore falls under Lens 1 claim discipline — see §6.3. |
| `section` | `fn section(&self) -> Section` | Nav active-state comparison | Return type changes from `&str` to `Section` |
| `canonical_path` | `fn canonical_path(&self) -> impl Display` — in practice `&'static str` on the six static pages, `String` on `BlogPostTemplate` and `WikiPageTemplate` | `og:url` (**new**, `base.html:9a`), joined to `crate::shell::SITE_ORIGIN` | **New, and the reason `og:url` is implementable.** Absolute-path form with a leading slash, no origin, no query, no trailing slash except on `/`. |

**Why `canonical_path` and not a shared field.** These are inherent impls on ten
unrelated structs, not a trait, so each one may return whatever type Askama can
`Display` — there is no common signature to satisfy and no ownership problem.
`BlogPostTemplate` returns `format!("/blog/{}", self.post.slug)` and
`WikiPageTemplate` returns `format!("/learn/{}", self.page.slug)`; both structs
already own the slug (`models/post.rs:55`, `models/page.rs:30`), so nothing new
has to be threaded through a handler. The six static pages return a literal.

**Error pages return `"/"`, deliberately.** `Error404Template` already owns the
requested path, and echoing it into `og:url` would be wrong twice over: it would
publish a canonical URL for a resource that does not exist, and it would reflect
attacker-chosen input into the one tag whose whole job is to be quoted by Slack
and LinkedIn. Askama escapes the value, so this is a correctness choice rather
than an injection fix — but a 404 has no canonical URL, and the site root is the
honest answer. I-7 asserts this explicitly.

**Cost of the fourth method, stated.** Contract S-1 goes from three required
methods to four, so all ten templates gain one. That is the price of a compile-
enforced contract, and it is the right price: the alternative — a `head_extra`
override per page — is the mechanism that made `og:type` unimplementable below.

Askama resolves these at **compile time** — a missing or misspelled method is a
build error (`pages.rs:10-14` documents this), so there is no runtime error case
and no auth dimension. This is the strongest guard in the shell and the spec
leans on it rather than adding a runtime check.

**Contract S-2 — per-page `<head>` overrides.**

*Iteration 1 of this spec routed `og:type` through `head_extra`. That does not
work and the mechanism is replaced here.* `base.html` emits
`<meta property="og:type" content="website">` at **line 9** and opens
`{% block head_extra %}` at **line 14**. A child template filling `head_extra`
can only **append** a second `og:type` tag five lines further down; it can never
replace the first. Consumers take the first occurrence, so blog and learn pages
would still advertise `website` — and I-7 ("exactly one of each required meta
tag") and I-8 ("`/blog/:slug` and `/learn/:slug` declare `article`") would be
mutually unsatisfiable. Appending is not overriding.

**The mechanism, corrected.** `base.html:9` becomes a block with a default:

```jinja
<meta property="og:type" content="{% block og_type %}website{% endblock %}">
```

`blog_post.html` and `wiki_page.html` each add one line,
`{% block og_type %}article{% endblock %}`. The other eight templates add
nothing and inherit `website`. Askama block overrides *replace* the parent's
content, so exactly one `og:type` tag is emitted per document and I-7 and I-8
become simultaneously satisfiable.

Chosen over the alternative — a fifth Contract S-1 method `og_type()` — because
`og:type` is a two-value fact about a template, not a per-request fact about a
struct, and a block costs two templates one line each where a method costs ten
templates one method each.

`{% block head_extra %}` survives with a narrower job: `<link rel="canonical">`
where a page is reachable at more than one URL. The `/wiki/*` legacy routes
(`router.rs:44-45`) are 3xx redirects so they need none today; this is the hook
if that ever changes. It has **no consumer on landing** and §6.3's planned-vs-
shipped rule applies — it produces no user-visible output until something fills it.

`{% block scripts %}` (`base.html:100`) is likewise unused. It stays as the
extension point for `C3` study-tools; A2 asserts only that anything landing there
must satisfy the no-JS floor on its own.

**Contract S-3 — static asset 404.** `router.rs:59` becomes:

```rust
use axum::handler::HandlerWithoutStateExt;

.nest_service(
    "/static",
    ServeDir::new("static").not_found_service(errors::fallback_404.into_service()),
)
```

so a missing static file returns the themed 404 rather than `ServeDir`'s
empty-bodied one. (tower-http 0.5, `Cargo.toml:14`.)

Note the exact form: `into_service()` lives on `HandlerWithoutStateExt` and takes
the **handler function itself**, not a `MethodRouter`. `get(errors::fallback_404)
.into_service()` does not compile on axum 0.7 — `MethodRouter` has no such
method. `fallback_404` qualifies as a `HandlerWithoutStateExt` because its only
extractor is `axum::http::Uri` (`errors.rs:139-141`).

**Rate limiting / pagination / auth:** N/A — the shell is stateless chrome. The
rate limiter (`router.rs:72-75`) is A3's and applies to every route uniformly.

### 4.4 State management

| State | Owner | Lifetime | Sync boundary |
|---|---|---|---|
| Current section | The page's Askama struct, via `section()` | Per-request | Server only |
| Page title/description | The page's Askama struct | Per-request | Server only |
| Vitals snapshot | `crate::state::APP_STATE` (`OnceLock<AppState>`, `state.rs:137`), read via `Status::current()` | Process | Server only; **read directly from the template**, not passed through handlers |
| Theme preference | `localStorage["theme"]` | Browser, persistent | Client only. Never sent to the server, never a cookie — no server-side theme state, no `Vary` header, no cache fragmentation |
| Menu open/closed | DOM `hidden` attribute + `aria-expanded` | Ephemeral | Client only |

**No new state container.** The vitals-strip-reads-a-global pattern is
pre-existing and deliberate (`state.rs:8-14`): threading `State<AppState>` into
every handler solely to render a footer would put an ops concern into all nine
shell-rendering handlers plus both error paths. The cost is that **every template render implicitly touches process
state**, which is why unit tests that render templates see zeros rather than
failing. This spec keeps the pattern and names the cost (§5.1 pins the zero-state
behaviour so it cannot silently become a panic).

**Offline / draft persistence:** N/A — the shell stores no user content.

### 4.5 Dependencies

**New packages: none.** Everything specified uses `askama 0.12`, `axum 0.7`,
`tower-http 0.5` already in `Cargo.toml:10-21`.

**New assets:**

| Asset | Purpose | Status |
|---|---|---|
| `static/img/og-card.png` (1200×630) | `og:image` — currently **absent**, so every link shared into Slack/LinkedIn/Discord previews as text only. `SOLARCORE_SPEC.md:190-191` claimed it would be "wired into the existing `og:` meta in base.html"; it never shipped. | **Planned**, with a two-step pipeline — see below. Owner's own artwork throughout; no third-party art, no licence question. |

**The generator, corrected.** *Iteration 1 said this file was "generated from the
existing `mark.svg` by `generate_brand.py`" and that infrastructure was "none".
Both were wrong, and commit 6 cannot land on them.* What the script actually does
(`docs/solarcore/generate_brand.py:225-233`): it builds `svg_mark(with_vines=True)`
once, writes it to `mark.svg`, strips the outer `<svg>` wrapper from that same
string (`:230`), and embeds the inner geometry into `svg_og_card()`, which writes
**`og-card.svg`** at `:231`. So the card really is derived from the mark — but the
output is **SVG**, and the script imports only `math`, `re`, `sys`, `os`
(`:6`, `:222`). There is no rasteriser anywhere in the repo. `static/img/` today
holds `favicon.svg`, `mark.svg`, `mark-sm.svg`, `vine-trace.svg` and nothing else.

Shipping the SVG as `og:image` is not an option: Slack, LinkedIn and Discord do
not reliably render SVG cards, and those three are the exact channel F-12 exists
to fix. So the deliverable is a **checked-in PNG**, produced by a documented
dev-time step:

```sh
# One-time, on the operator's workstation. Not run in CI, not run at build time.
python3 docs/solarcore/generate_brand.py static/img     # writes og-card.svg
rsvg-convert -w 1200 -h 630 static/img/og-card.svg -o static/img/og-card.png
git add static/img/og-card.png                          # the PNG is the artifact
```

- **Tool:** `rsvg-convert` (librsvg). Confirmed present on the workstation.
  `cairosvg`, Inkscape and ImageMagick are not installed; `chromium` is, and is
  the documented fallback (`chromium --headless --screenshot`) if librsvg ever
  goes away.
- **CI does not regenerate it.** No Python, no rasteriser, and no image step
  enters `.github/workflows/ci.yml`. The PNG is committed like any other asset and
  its provenance is the committed SVG beside it, which anyone can re-derive.
- **Keep `og-card.svg` committed too**, so the PNG is reproducible rather than a
  binary blob with no source.
- **Drift risk, named:** nothing makes the PNG follow the SVG. This is the one
  place A2 accepts an unguarded copy, because the alternative — a rasteriser in
  CI — costs more than a stale social card. The guard is procedural: the two
  commands above live in `README.md` beside the brand-regeneration step, and
  `og-card.svg` and `og-card.png` change in the same commit or neither changes.

**Infrastructure:** **none at runtime and none in CI** — no CDN (CSP is
`default-src 'self'`, `security_headers.rs:41-50`), no webfonts (all font stacks
are system stacks — `generate_themes.py:12-15`), no third-party services, no
image processing in the request path. **One dev-time tool**, `rsvg-convert`, run
by hand for the `og:image` card and declared above. Correcting the earlier
"Infrastructure: none" line: a tool the operator must have installed to produce a
shipped asset is a dependency even when it never runs on the server.

### 4.6 Platform-specific considerations

- **Browser support:** the shell targets evergreen browsers plus graceful
  degradation to *anything*. `main.js` is deliberately ES5-flavoured (`var`,
  `function`, no arrow functions, no optional chaining) — keep it that way; it is
  served unbundled and untranspiled with no build step.
- **`matchMedia().addEventListener`** (`main.js:77`) requires Safari ≥ 14 (2020).
  Acceptable; older Safari simply stops live-following the OS in `system` mode,
  which is a graceful degradation, not a break.
- **`clip-path: inset(50%)`** for `.vh` is universally supported; no fallback
  needed.
- **`100svh`** requires a `100vh` declaration first as the fallback for older
  engines.
- **`ServeDir::not_found_service`** exists on tower-http 0.5; the axum-side form
  is pinned in §4.3 Contract S-3 (`HandlerWithoutStateExt::into_service` on the
  handler, not on a `MethodRouter`).
- **`position: sticky`** on the header and footer is universally supported and
  needs no fallback; a browser that ignored it would simply render two static
  bars, which is the pre-`a375a14` layout.
- **CSP interaction:** `script-src 'self'` is why `theme-init.js` is an external
  file rather than an inline anti-FOUC snippet (`theme-init.js:2-4`). Any future
  shell work must not reach for an inline `<script>` or `style` attribute.
- **Feature flags / gradual rollout:** N/A — single binary, single deploy, one
  operator. The rollout mechanism is the commit sequence in §7.2.

### 4.7 Performance budget

| Dimension | Current | Target | Note |
|---|---|---|---|
| Shell HTML weight | `base.html` is 9,845 B of template source + `vitals_strip.html` 881 B. Two blocks dominate: the inline brand SVG is 2,625 B (`base.html:21`, a single line) and the 24-button theme menu is 4,973 B (`:33-79`) — together 77% of the shell. | ≤ 11 KB rendered, uncompressed, with **no growth** from this spec beyond the `.vh` spans, `aria-current`, and the new `<meta>` tags (a few hundred bytes) | Gzip/brotli at the Caddy layer makes the highly repetitive theme-button markup nearly free on the wire; the uncompressed figure is what matters for parse cost |
| JS payload | `theme-init.js` 960 B + `main.js` 4,664 B = **5.6 KB, 95 lines** | ≤ 6.5 KB after typeahead + aria-label sync. **A hard ceiling: if the shell's JS crosses 150 lines, the identity claim in §1.2 is no longer true and the copy must change.** | This ceiling is the spec's teeth for Lens 3A |
| CSS payload | `style.css` **44 KB / 1,561 lines** (all 23 themes inline, one file; the roster occupies `style.css:1-470`). It grew ~3 KB after this spec's first iteration: Markdown table rules, a contrast-driven palette pass, and the sticky header/footer commit. | +~0.6 KB for the OS-preference fallback block, and **no net growth** from the `--text-*` migration (literals are replaced, not added) | One file, one request, no critical-CSS split. The figure is worth re-reading before quoting it: three commits moved it under this spec |
| `og:image` asset | absent | one 1200×630 PNG, expected well under 100 KB from flat vector art | Served from `/static`, requested only by link-preview crawlers, never on a page render |
| Render-blocking | 1 script (`theme-init.js`, `base.html:12`) + 1 stylesheet (`:13`) | Unchanged | The blocking script is the anti-FOUC trade and is worth 960 B |
| Server CPU per render | `Status::current()` → one atomic load, one `Instant::elapsed`, one mutex acquire on `rss_cache`, `/proc` read at most once per 5s (`state.rs:42`, `169-186`) | Unchanged | The RSS cache is what keeps the strip from being a per-request syscall |
| Memory | Process RSS in the low tens of MiB (rendered live in the footer) | Unchanged | `NAV` is a `const` slice; `Section` is a fieldless enum |
| Startup | `build.rs` stamps `BUILD_TS` at compile time (`state.rs:34`); no runtime asset scan | Unchanged | |
| Client storage | One `localStorage` key | Unchanged | |

**Caching requirement handed to A3:** the vitals strip embeds a live counter into
**every** HTML document. If any layer (Caddy, Cloudflare) ever caches HTML, the
strip will display stale numbers — which on a site whose asset is truthfulness is
not a perf bug but a **claim-integrity bug**. `/status` and `/status.json`
already carry `Cache-Control: no-store` and are test-pinned
(`status.rs:125-135`); HTML documents carry **no** cache header today. A3 must
add an explicit HTML cache policy and a test in the same family.

---

## 5. Test Specification

All tests run under `cargo test --all-targets` and gate CI (`.github/workflows`,
`fmt → clippy → test → build --release`).

**Placement rule (see §4.1).** `mg-server` is a bin-only crate, so *every* test
below — unit and router-level alike — lives in a `#[cfg(test)] mod tests` inside
`src/`. Nothing goes in `tests/`. `src/shell.rs::tests` is the home for the shell
contract; `src/errors.rs::tests` gains the error-page additions. This follows the
existing convention at `errors.rs:143-193` and `handlers/status.rs:60-135`.

**Landing rule (criterion 5B).** Every guard below is labelled **green today** or
**red today**. A guard that is red on arrival is not a defect — U-7 is
deliberately red because the copy it guards is wrong — but a guard whose colour
is *unstated* is, because an implementer will read a red test as a broken test
and weaken it. Where a guard is red, the row names the commit that turns it
green and what else must land first.

### 5.1 Unit tests

New module `src/shell.rs::tests` plus additions to `src/errors.rs::tests`.

| # | Name | Setup | Assertion | Edge case covered |
|---|---|---|---|---|
| U-1 | `every_nav_section_is_reachable` | Iterate `shell::NAV` | Each `item.href` matches a route registered in `router::build` (drive `oneshot` per href) | A nav link to a route that was renamed or removed. **Green today** |
| U-2 | `every_nav_item_highlights_its_own_page` | For each `NAV` item, request its `href` | Body contains `class="nav-link is-active" aria-current="page"` on exactly one link, and it is that item's label | **The `/learn` → `section "wiki"` class of bug: a section string that matches nothing.** **Red today** (no `aria-current` exists); green at commit 3 |
| U-3 | `pages_outside_the_nav_highlight_nothing` | Request `/`, `/releases`, `/status`, `/no-such-page` | Body contains zero `is-active` and zero `aria-current` | Silent mis-highlighting on off-nav routes. **Green today** |
| U-4 | `every_page_declares_exactly_one_h1` | Render each of the **10 non-shell templates** — `about`, `blog_list`, `blog_post`, `index`, `portfolio`, `releases`, `status`, `wiki_page`, `error_404`, `error_500` | `matches("<h1").count() == 1` | A page shipping with no heading or a duplicated one. **Green today** — verified: all 10 have exactly one, `base.html` and `vitals_strip.html` have zero. (Iteration 1 said "10 content templates *plus* both error templates"; the error templates are two of the ten, not two more) |
| U-5 | `page_titles_carry_the_site_name` | Render every template | `<title>` ends with `" — machinageist"`, except home which equals `"machinageist"` | **Red today** on `blog.rs:134-136` and `wiki.rs:110-112`, which return a bare title; green at commit 9 |
| U-6a | `template_descriptions_are_within_meta_length` | The **six static, indexable** `description()` sites: `pages.rs:44`, `:81`, `:114`, `blog.rs:68`, `releases.rs:40`, `status.rs:38` | `50 <= len <= 160` | **Red today by one character** — `releases.rs:40` is 49. Every other site clears: 124 / 110 / 66 / 121 / 133. Green when `B6` rewrites that string, which it must do anyway for the §1C gate (§7.4 request 3); lands with commit 8 and **must not be committed before** the B6 edit |
| U-6b | `error_descriptions_are_capped_but_not_floored` | `errors.rs:76`, `:93` | `len <= 160` only | **Green today** (38 and 42). The 50-character floor is deliberately **not** applied: it exists so a search snippet is at least one line long, and a 404 is not an indexable surface. Padding *"The requested page could not be found."* to reach a test threshold would be the test writing the copy |
| U-7 | `descriptions_do_not_carry_retired_claims` | Every `description()`, including the frontmatter-backed ones | Rejects the substrings `"Network+"`, `"CompTIA A+"`, `"CompTIA study"`, `"CompTIA stack"`, `"offensive security"`, `"red-team"`, `"red team"`, `"pentest"`, `"penetration test"`, `"production-grade"`, `"enterprise-grade"`, `"enterprise-ready"`, `"SRE"`. Plus one scoped rule: reject `\b(I|we|my|our)\b[^.]{0,80}\benterprise\b` (case-insensitive) | **Criterion 1D/1E — the `<meta>` tag is user-visible copy that no page-body test currently guards.** **Red today** on `pages.rs:44` and `:81`, both *"CompTIA study"*; green when `B1`/`B2` land (§7.4 request 1). See the narrowing note below |
| U-8 | `asset_version_is_derived_not_literal` | `shell::asset_version()` | Non-empty, parses as an integer, and `base.html` contains no literal `?v=2026` string | **Criterion 5B — the current `?v=20260719-spectrum` string is hand-typed and already stale.** **Red today**; green at commit 4 |
| U-9 | `theme_menu_matches_the_generator` | Run `python3 docs/themes/generate_themes.py --menu`, normalise whitespace | Output equals the `.theme-group` block in `base.html` | **Criterion 5B — `generate_themes.py:292-318` emits the menu and `:296-304` guards `MENU_GROUPS` against `THEMES`, but nothing compares its output to the shipped file.** **Green today** (verified by eye; the shipped block is the generator's output). Q7 is unresolved and blocks writing it |
| U-10 | `theme_modes_agree_as_sets_and_the_two_js_arrays_agree_in_order` | Parse `[data-mode]` values from `base.html`; parse `MODES` from `main.js:12` and `theme-init.js:6` | (a) all three have length 24; (b) as **sets**, all three are equal; (c) `main.js` `MODES` and `theme-init.js` `MODES` are equal **in order**. **Document order of the menu is explicitly NOT compared to `MODES` order** | A theme added to the menu but not the JS allowlist becomes a no-op button; a theme added to one JS file but not the other breaks anti-FOUC. **Green today.** See the ordering note below |
| U-11 | `vitals_strip_renders_with_no_global_state` | `Status::current()` with `APP_STATE` unset | Returns zeros; strip renders; no panic | `state.rs:240-253` — the 500 page depends on this. **Green today** |
| U-12 | `requested_path_is_html_escaped` | **exists** (`errors.rs:160-169`) | Keep unchanged | XSS via the 404 path echo. **Green today** |

*Marked `#[ignore]` if the CI image lacks Python:* U-9 additionally gets a plain
`cargo test` variant that asserts the group/label/slug **structure** without
shelling out, so the guard is never fully absent.

---

**U-10, and why order equality would have been a bug.** *Iteration 1 of this spec
asserted the three lists were "equal, in order, length 24". That is false today,
and shipping it would have forced a deliberate design decision to be reverted.*

The two orders, read from source:

```
base.html [data-mode], document order (grouped):
  system, lunarcore, solarcore | dark, solarized, nord, gruvbox |
  crt, amber, matrix, teletext | gameboy, c64, nes |
  synthwave, vaporwave, cyberpunk, tron | light, paper, dawn, cloud, blueprint, sepia

main.js MODES == theme-init.js MODES:
  system, lunarcore, solarcore, dark, light, crt, amber, paper, dawn, cloud,
  gameboy, c64, teletext, nes, matrix, solarized, nord, gruvbox,
  synthwave, vaporwave, cyberpunk, tron, blueprint, sepia
```

They agree on the first four entries and diverge at the fifth — `solarized` in
the menu, `light` in the JS. **The divergence is correct and must be preserved.**
Commit `af6566d` grouped the menu into six `role="group"` wrappers for
scannability, and `generate_themes.py` says so at `:277-280`: *"display order of
the theme picker, which is deliberately not THEMES order"*. `emit_modes()`
(`:265-267`) emits `THEMES` order; `emit_menu()` (`:292-318`) emits
`MENU_GROUPS` order. Two orders, one roster, on purpose.

Nothing depends on the two orders matching. `MODES` is used only as an
**allowlist** — `MODES.indexOf(m) >= 0` at `main.js:18` and `theme-init.js:8` —
never as a sequence. Roving focus reads `[data-mode]` from the DOM in document
order (`main.js:14`), which is why the grouping commit needed no JS change at
all. So the property worth guarding is *membership*, and the property worth
guarding **in order** is only that the two JS arrays match each other, because
`theme-init.js` runs before paint and `main.js` runs after: if they disagreed,
a stored theme could pass one allowlist and fail the other.

**U-7, and why bare `"enterprise"` had to come out.** *Iteration 1 banned the
bare substring `"enterprise"` across every `description()`. That fires on shipped
educational copy.* `content/pages/network-topologies.md:4` reads: *"How nodes and
links are arranged — mesh, star, ring, spine-and-leaf, and the tiered designs
used in **enterprise** and data center networks."* That reaches `<meta>` through
`WikiPageTemplate::description()` (`wiki.rs:114-116`). It is topic vocabulary — a
network topology page describing enterprise networks — not a role claim, and
banning it would force a page to misdescribe its own subject to satisfy a guard.

Criterion 1E is about what the author claims to *be*, not about what a page is
*about*. So the rule splits: role vocabulary (`"production-grade"`,
`"enterprise-grade"`, `"SRE"`, `"red-team"`, `"pentest"`, `"offensive security"`)
stays an unconditional ban, and bare `"enterprise"` is caught only when a
first-person pronoun attributes it to the author within the same sentence. Bare
`"A+"` comes out for the same reason in miniature — it is two characters that
appear inside ordinary text — and is replaced by `"CompTIA A+"`.

`"CompTIA"` is **not** banned outright, because Security+ is a CompTIA
certification and is on the live spine. What is banned is the stale framing:
`"CompTIA study"` and `"CompTIA stack"`, which read as the multi-cert sequence the
2026-08-02 re-lock dropped, and `"Network+"`, which was dropped by name.

Verified against every shipped string: the narrowed list is clean on all six
static template descriptions, both error descriptions, and all **16 served**
frontmatter summaries (13 in `content/pages/`, 3 in `content/posts/`;
`content/drafts/` is not loaded by any handler — `POSTS_DIR` is `content/posts`
at `blog.rs:30` and `PAGES_DIR` is `content/pages` at `wiki.rs:19`). The only
failures are the two intended ones in `pages.rs`.

**The frontmatter length rule is filed, not solved here.** U-6a and U-6b cover
the eight static `description()` strings. The two frontmatter-backed ones
(`blog.rs:137-139`, `wiki.rs:114-116`) are deliberately excluded, because four
served summaries exceed 160 characters — `management-layer-first-network-migration.md`
at 227, `hosting-machinageist-dev.md` at 220,
`security-headers-on-machinageist-dev.md` at 217, and `ipv4-addressing.md` at
168 — and shortening them would be the wrong fix. `summary:` serves two
consumers with different length appetites: the `<meta name="description">` tag,
where anything past ~160 is truncated, and the blog-list card
(`blog_list.html`), where 220 characters is a good blurb. One field cannot be
bounded for both.

A2 files this to `B4` (writing) and `B5` (learn) with the measurement above and
the two candidate resolutions: (a) add an optional `meta_description:`
frontmatter field that `description()` prefers and falls back to `summary`, or
(b) accept the truncation and stop pretending the tag is authored. A2 declines
to pick, because the answer is a content-model decision in someone else's
territory, and declines to write a guard it knows would be red for reasons it
cannot fix. §7.4 carries the request.

### 5.2 Router-level tests

*Iteration 1 placed these in a new integration file, `tests/shell.rs`. That file
cannot exist.* `mg-server` has no `src/lib.rs` and `src/main.rs:16-21` declares
every module privately, so a `tests/` crate cannot name `router::build`,
`shell::NAV`, or any template struct. The two options were: (a) add a
`src/lib.rs` and split the crate so `tests/` can reach in, or (b) put these tests
where every other router-level test in the repo already lives.

**(b), for three reasons.** Splitting the crate would be a real architectural
change — a new public surface, a `lib` + `bin` target pair, and re-exports for
things that exist only to be tested — proposed by a shell spec, for the
convenience of a test directory. The repo has already answered the question
twice: `errors.rs:143-193` and `handlers/status.rs:60-135` drive `oneshot`
against `router::build(AppState::new())` from inside `src/`, and
`tests/wiki_pages.rs:12-14` documents the one thing the external crate is *for*
— assertions about files on disk, kept deliberately decoupled from the bin. And
these tests assert on private types (`shell::Section`, `shell::NAV`), which is
exactly the case a `#[cfg(test)]` module handles and an integration test cannot.

So **I-1 … I-9 are router-level tests in `src/shell.rs::tests`**, using
`tower::ServiceExt::oneshot` (the pattern at `errors.rs:171-182` and
`status.rs:84-89`), with the `get_body` helper lifted from `status.rs:84-89` into
a shared test helper. `tests/` gains no new file. §7.2 reflects this.

The nine HTML routes under test — `/`, `/about`, `/portfolio`, `/blog`,
`/blog/:slug`, `/learn`, `/learn/:slug`, `/releases`, `/status` — plus the 404
and a forced 500 make **eleven shell-rendering responses**. (`/wiki`,
`/wiki/:slug`, `/status.json`, `/robots.txt`, `/security.txt` and
`/.well-known/security.txt` are excluded and I-10 pins that they *stay*
excluded.)

| # | Name | Assertion |
|---|---|---|
| I-1 | `every_route_renders_the_full_shell` | For the 11 shell responses: body contains the skip link, `<header class="site-header"`, `aria-label="Primary"`, `<main id="content"`, `<footer class="site-footer"`, and `vitals-strip`. Extends `status.rs:113-123`, which today covers only `/` and `/blog`. **Green today** except the `aria-label="Primary"` narrowing (commit 5) |
| I-2 | `shell_needs_no_javascript_to_be_complete` | For every shell response: strip all `<script …></script>` elements from the body, then assert the remainder still contains every nav `href`, the skip link target `id="content"`, the footer source link, and the `/status` link. **This is the machine-checkable form of the no-JS floor.** **Green today** |
| I-3 | `no_inline_script_or_style_survives_csp` | No served HTML contains `<script>` with a body, `on[a-z]+=` handler attributes, or a `style=` attribute. Guards `security_headers.rs:41-50` from being quietly violated by a template edit. **Green today** |
| I-4 | `missing_static_asset_returns_the_themed_404` | `GET /static/nope.css` → 404 **and** body contains `SECTOR NOT FOUND`. **Red today** (E-04); green at commit 7 |
| I-5 | `error_pages_carry_working_navigation` | 404 and 500 bodies contain every `NAV` href. **Green today** |
| I-6 | `internal_error_page_leaks_nothing` | **exists** (`errors.rs:184-193`) — extend the allowlist to also reject `"panicked"`, `"askama"`, `"axum"`, `"tower"`. Framework fingerprinting via an error page. **Green today**, and stays green with the extension |
| I-7 | `head_carries_the_required_meta_set` | Every shell response: exactly one `<title>`, one `<meta name="description">`, and exactly one each of `og:title`, `og:description`, `og:type`, `og:url`, `og:image`; `<html lang="en">`. `og:url` starts with `crate::shell::SITE_ORIGIN` and its path component equals the requested path — **except** on the 404 and 500, where it must equal exactly `SITE_ORIGIN` + `"/"` and must **not** contain the requested path (§4.3). **Red today**; green at commit 6, which is blocked on `og-card.png` (§4.5) |
| I-8 | `article_pages_declare_og_type_article` | `/blog/:slug` and `/learn/:slug` → the single `og:type` is `article`; the other nine responses → `website`. Proves the `{% block og_type %}` override replaces rather than appends — the defect that made this test unsatisfiable in iteration 1. **Red today**; green at commit 6 |
| I-9 | `landmark_names_follow_the_shell_contract` | For every shell response: no `aria-label` on a `<nav>` or `<aside>` matches `/navigation"?$/i`, and each page renders **at most one unnamed `<nav>`**. **Red today** on three surfaces — `base.html:20` `"Primary navigation"`, `base.html:92` `"Footer navigation"`, `index.html:11` `"Quick navigation"` — plus `wiki_page.html:4-7`, which both mislabels an `<aside>` and leaves its `<nav>` unnamed. Green when commit 5 lands the `base.html` and `wiki_page.html` halves and `B1` lands `index.html` (§7.4 request 5). **This is the test F-10 needed and iteration 1 did not have:** without it the contract is a one-time edit that the fourth nav surface will violate |
| I-10 | `non_html_routes_do_not_render_the_shell` | `/status.json`, `/robots.txt`, `/security.txt`, `/.well-known/security.txt` contain no `site-header` and no `vitals-strip`; `/wiki` and `/wiki/:slug` return 3xx. **Green today.** Cheap, and it keeps I-1's route list honest — a future route added to the wrong list fails here instead of silently widening the shell's claimed surface |

### 5.3 UI / E2E tests

**Status: absent, and deliberately so.** There is no browser-automation harness
in the repo (no Playwright, no Selenium, no `package.json`) and this spec does
**not** introduce one — a headless-browser dependency on a site with 95 lines of
JS would cost more maintenance than it buys, and it would put a Node toolchain
into a Rust-only CI pipeline.

The keyboard behaviours that E2E would normally cover are instead specified as
**a written manual checklist** (§5.4) and, where possible, pushed down into I-2/
I-3, which verify the *served bytes* rather than a rendered browser.

If a browser harness ever lands (it is a prerequisite for `C3` study-tools,
which cannot be verified any other way), these are the first six scenarios:

1. Tab once from load → skip link is visible, legible against the header, and
   `document.activeElement` is the skip link; Enter → `activeElement` is `<main>`.
2. Theme button → ArrowDown → focus is on the *checked* item, not the first.
3. Menu open → `End` → last item; `ArrowDown` → wraps to first.
4. Menu open → `Tab` → menu closes **and** `activeElement` is the theme button
   (regression guard for F-06).
5. Menu open → Escape → closed, `activeElement` is the theme button.
6. With `page.setJavaScriptEnabled(false)`: `.theme-select` is not visible, and
   with `colorScheme: 'light'` emulated, `getComputedStyle(body).backgroundColor`
   is the Solarcore background.

### 5.4 Visual / manual verification

Run `cargo run` and check, per `docs/solarcore/SOLARCORE_SPEC.md:279-283`'s
per-phase visual pass, on: `/`, `/about`, `/portfolio`, `/blog`, a blog post,
`/learn`, a learn page, `/releases`, `/status`, `/no-such-page`.

| Configuration | What to look for |
|---|---|
| **All 23 themes** (`style.css:16-470`) | Header border, nav active underline (and that **hover ≠ active** after the F-04 fix), theme-menu surface/shadow, footer border, vitals text, boot-log text, and the **focus ring** are all legible. On the focus ring, see the correction below — it is machine-audited, so this pass is a sanity check rather than the measurement |
| **Sticky bars** (`style.css:566-573`, `818-836`) | Header and footer stay opaque over scrolling content in every theme — a translucent or missing `--bg` shows text through the bar. Check one dark, one light and one neon theme at minimum, plus a short viewport (< 34rem tall) where the footer un-pins |
| **Skip link over the sticky header** | Tab once and confirm the link paints **above** the header, not under it — `z-index: 60` vs the header's `50`. This is a regression check on `a375a14`, not a change |
| **`/learn` sidebar against the pinned bars** | At > 800px the sticky rail's last entry clears the footer at scroll 0 and at full scroll; at ≤ 800px it goes static and the `<details>` panel floats over the page rather than pushing it down |
| **Lunarcore + Solarcore** at minimum | The two flagship themes get the full pass every time; the other 21 get a spot check |
| **JS disabled** | Theme control absent (not present-and-dead); OS light preference → Solarcore, dark → Lunarcore; all nav works; 404 works |
| **`localStorage` blocked** (Firefox `dom.storage.enabled=false`) | Same as above — this is the case current `theme-init.js:7-14` mishandles (F-05) |
| **`prefers-reduced-motion: reduce`** | No cursor blink, no menu pop, no underline sweep, no CRT scanlines, error page fully visible at t=0 |
| **Text zoom 200%** | Header wraps; nav does not clip; theme menu still on screen |
| **Viewport 320px @ 400% zoom** | No horizontal page scroll (WCAG 1.4.10) |
| **Viewport 360 / 768 / 1440 / 2560px** | Header and footer wrap sensibly; `main` stays centred at 900px |
| **Non-Linux host** (`rss_mib() == None`, `state.rs:262-272`) | `MEM` item **and** its separator both absent; no double `·` |
| **Markdown table on a `/learn` page** | The table scrolls inside its own container (`style.css:1163-1176`) rather than widening the page — the ≤ 640px case is the one that breaks |
| **Fresh process** (`UP 00:00:00`, `REQ 1`) vs **long-running** (`UP 07:13:42`, `REQ 40381`) | Strip does not reflow the footer or wrap awkwardly at either extreme |
| **Screen reader** (Orca on Linux, VoiceOver on macOS) | Landmark list reads banner / navigation "Primary" / main / contentinfo / navigation "Footer"; active nav announces "current page"; vitals read as expanded words |
| **Social preview** | Paste a URL into Slack and Discord after `og:image` lands; card shows the mark, not a bare link |

**The focus-ring audit, corrected — and then retired.** *Iteration 1 asked A1 to
add a contrast row for `--accent` on `--bg` at 3:1, calling that pair
"currently unaudited". Both halves were wrong.*

Wrong pair, first. `:focus-visible` sets `outline-offset: 2px`
(`style.css:710`), which draws the ring two pixels *outside* the element's border
box — so the colour behind it is whatever the element sits **on**, not the page
background. On a theme-menu button that is the menu's `--surface`
(`style.css:657`); on `.project-card:hover` it is `--surface` again
(`style.css:731`). `--bg` is the right backdrop only for the nav links and the
brand. Any ring audit that checks `--bg` alone misses the two surfaces where the
ring is most likely to disappear.

Wrong status, second, and this is the part that changes the deliverable. Commits
`c2f403e` and `0cdbbea` rewrote `generate_themes.py`'s `USAGE` table
(`:143-159`) for exactly this reason — its own comment says the old audit
"checked five tokens against `--bg` only" and that "every one of these tokens
also renders on `--surface` (cards, the theme menu, code spans, sidebars)". It
now holds `accent` against **both** `bg` and `surface` at **4.5**
(`:156`), which is stricter than WCAG 1.4.11's 3:1 for a non-text UI component.
Verified green: `python3 docs/themes/generate_themes.py --check` reports
*"contrast: all pairs clear across 23 themes"* and exits 0.

So there is **no audit row to add**. What A2 owes A1 is not a request but a
**dependency to declare**: the focus ring's WCAG 1.4.11 compliance is carried
entirely by the `("accent", ("bg", "surface"), …, 4.5)` row in `USAGE`. If A1
ever relaxes that row below 3:1, narrows it back to `bg` alone, or drops
`surface` from any token's backdrop list, the ring stops being audited and 3B
regresses silently across 23 themes. §7.4 records it as a constraint on A1's
territory rather than as work A2 is waiting on — and commit 1 is no longer
blocked on it.

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement.**

The shell handles: a colour preference in `localStorage` (24-value allowlist,
never transmitted), and read-only process metrics. Positive controls confirmed in
source:

- No cookies, no session, no auth, no forms, no PII, no analytics, no third-party
  requests (CSP `default-src 'self'`, `security_headers.rs:41-50`).
- The vitals strip exposes uptime, a request count, RSS in MiB, crate version and
  build timestamp — **no hostname, no IP, no filesystem path**
  (`state.rs:16-18`); the bind address is deliberately reduced to a
  classification string (`state.rs:209-219`) and pinned by
  `state.rs:332-338` and `status.rs:97-100`.
- The 404 echoes the requested path but Askama auto-escapes it, proven by
  `errors.rs:160-169`.
- The 500 page discloses nothing internal, proven by `errors.rs:184-193`.
- `Server` header removed (`security_headers.rs`, final block).

**One judgement call recorded:** publishing a live request counter and uptime is a
small operational-intelligence disclosure (it tells an observer roughly how much
traffic the site gets and when it last restarted). For a personal portfolio this
is the *point* — the number is evidence — and it exposes nothing exploitable.
Accepted deliberately.

### 6.2 Asset provenance

- [x] **Uses third-party assets — listed below.**

| Asset | Source | Licence | Rights status |
|---|---|---|---|
| Brand mark SVG (inline, `base.html:21`; files `static/img/mark.svg`, `mark-sm.svg`, `favicon.svg`, `vine-trace.svg`) | Hand-authored / agent-drafted for this project per `SOLARCORE_SPEC.md:173-176`, `:271-273` | Owner's own work | ✅ Clear |
| Font stacks (`ui-monospace`, `SFMono-Regular`, `Menlo`, `Consolas`, `Charter`, `Georgia`, `ui-rounded`, `Segoe UI`, `system-ui`) | OS-provided; `generate_themes.py:12-15` | Referenced by name only — **no font file is bundled or served** | ✅ Clear; also why `font-src 'self'` is safe |
| Menu and vitals glyphs (`◐ ⏾ ✸ ✦ ☀ ▦ ◈ ¶ ◒ ☁ ▣ ▩ ⌗ ✜ ⌁ ◉ ❄ ◆ ▹ ▧ ⌖ ⊞ ⊟ ⚙ ▍ ✓ ·`) | Unicode code points, rendered by the system font | Not copyrightable as characters | ✅ Clear |
| Theme *names* — Dracula, Solarized, Nord, Gruvbox, Game Boy, Commodore, Tron | Third-party colour schemes and trademarks referenced descriptively | Colour values are not copyrightable; names are nominative reference in a personal, non-commercial UI. Note the menu already renames some (Game Boy → label "Game Boy", `nes` → "8-Bit", `sepia` → "Steampunk") | ⚠️ **A1's call, not A2's** — flagged, not resolved here |
| `static/img/og-card.png` (planned) | Rasterised by `rsvg-convert` from `og-card.svg`, which `generate_brand.py:230-231` builds from the same `svg_mark()` geometry it writes to `mark.svg` | Owner's own work end to end; `librsvg` is LGPL-2.1 and is a **build-time tool**, not a bundled or linked component, so nothing of it ships | ✅ Clear on arrival |

**No AI-generated raster art ships.** `SOLARCORE_SPEC.md:173-174` is explicit that
the AI reference images are mood boards only.

### 6.3 Language / claims audit

- [x] Makes claims not supported by evidence? **No** — with one inherited
  violation A2 must guard against, below.
- [x] Promises capabilities not yet built? **No** — every user-visible string
  this spec introduces describes shipped behaviour. Where something is planned
  (`og:image`, typeahead, themed static 404), this document labels it *planned*
  and it produces **no user-visible copy** until it lands.
- [x] Uses language restricted by domain regulations? **No.**

**Shell-authored copy inventory** (everything the shell itself says):

| String | Location | Verdict |
|---|---|---|
| `"Skip to content"` | `base.html:17` | Functional |
| `"machinageist"` (wordmark) | `base.html:21` | Identity, not a claim |
| `"About" "Portfolio" "Writing" "Learn"` | `base.html:24-27` | Functional |
| `"Theme"` + 24 theme labels + 6 group labels | `base.html:30-78` | Functional |
| `"machinageist.dev — Jeff Cincoski"` | `base.html:91` | Fact |
| `"Source"` → the real repo | `base.html:93` | Verifiable by clicking |
| `UP` / `REQ` / `MEM` / `v{version}` / `built {ts}` | `vitals_strip.html:8-18` | **Measured, not asserted** — read from the live process. The strongest kind of claim on the site. |
| `"MG-BIOS v0.1 — DISK CHECK"`, `"SECTOR NOT FOUND"`, `"0 pages read at …"`, `"(A)bort → return home"` | `error_404.html:5-9` | Costume, not a capability claim. `MG-BIOS` is not asserted to exist; it is an error page in period dress. No reader could mistake it for a shipped BIOS product. |
| `"*** STOP ***"`, `"KERNEL PANIC — REQUEST HALTED"`, `"registers dumped to the operator log — nothing useful to see here"` | `error_500.html:5-8` | Same. "registers dumped to the operator log" is a stylised but **true** statement — `errors.rs:114` really does `error!()` the full internal error to the tracing log. |

**The inherited violation A2 must guard.** `base.html:6` renders
`self.description()` into `<meta name="description">`. `IndexTemplate::description()`
(`pages.rs:44`) and `AboutTemplate::description()` (`pages.rs:81`) both say
`"CompTIA study"`, and `AboutTemplate`'s bio (`pages.rs:92-93`) says *"working
through the CompTIA stack"*. Per criterion **1D**, the spine re-locked on
2026-08-02 to **RHCSA → CCNA → Security+ with Network+ dropped**; "the CompTIA
stack" reads as the multi-cert CompTIA sequence and is now misleading.

**A2 does not rewrite that copy** — the strings belong to `B1` (home) and `B2`
(about) and rewriting them here would duplicate ownership. What A2 owns is the
fact that this copy reaches the user *through the shell, invisibly*, with no test
watching it. Test **U-7** is A2's contribution: it makes any retired-claim string
in **any** page's `description()` fail CI, regardless of which feature authored
it. Filed as a cross-feature request in §7.4.

**Releases copy note (criterion 1C).** `ReleasesTemplate::description()`
(`releases.rs:40`) says *"GeistScope source tarballs and compiled binaries"* and
reaches `<meta>` and `og:description` through this shell. GeistScope is under the
publication gate. A2 records the exposure and hands it to `B6`; it is not A2's
string to change.

### 6.4 Regulatory alignment

`criteria.md` Lens 3 is the governing standard.

| Criterion | How A2 addresses it |
|---|---|
| **3A — works without JavaScript** | Every route is fully server-rendered (Askama → HTML, no hydration, no client routing). The only JS is theme selection. **Fallback defined and specified:** OS preference honoured in pure CSS via `:root:not([data-theme])` under `prefers-color-scheme` (§4.2), and the dead control removed via `:root:not([data-js])`. Machine-checked by **I-2**, which strips scripts from the served bytes and asserts the shell is still complete. A hard 150-line JS ceiling is written into §4.7. |
| **3B — contrast and colour independence** | Token contrast is A1's, audited by `generate_themes.py`. A2 adds: (a) a **visual** hover/active distinction on nav links by weight and opacity, not hue (§3.3, F-04) — today they are byte-identical; (b) `aria-current="page"` so the active nav state is not colour-plus-underline only; (c) preservation of the theme `✓` mark as the non-colour checked cue. On the **focus ring**: it is already covered at 4.5 against both `--bg` and `--surface` by `generate_themes.py:152-159`, stricter than 1.4.11's 3:1, and A2 declares that row as a standing constraint rather than requesting a new one (§5.4). |
| **3C — keyboard and focus** | Full key table in §3.4. Fixes: `<main tabindex="-1">` (F-14), Tab-close focus restore (F-06), the `focusedIndex() == -1` clamp (F-15), typeahead with APG defaults stated (§3.2). Skip-link legibility (F-02) shipped in `a375a14` and is now a preservation requirement. The theme menu already implements the APG roving-focus model (`main.js:43-72`) and remains the in-repo reference. Visible focus indicator is global and never removed (`style.css:710`). |
| **3D — semantics and AT** | Landmark table in §3.7, now covering all four of the site's navigation surfaces. Fixes: `<nav>` narrowed to wrap only the links (the theme control is not navigation); `aria-label="Primary"`/`"Footer"`/`"Curriculum"` de-duplicated against the role announcement; the `/learn` `<aside>` loses its nav-shaped label; `.vitals-strip` given `role="group"` so its `aria-label` is actually honoured (an `aria-label` on a bare `<div>` is ignored — `vitals_strip.html:7`); decorative SVG and glyphs stay `aria-hidden` (already correct); one `<h1>` per page pinned by **U-4**; the naming contract itself pinned site-wide by **I-9**. |
| **3E — motion and sensory safety** | Every animation already sits inside `@media (prefers-reduced-motion: no-preference)` (`style.css:735-748`, `1358-1368`), and the CRT texture is removed under `reduce` (`style.css:461-470`). The sticky bars add no motion — `position: sticky` is layout. No autoplay, no body-content animation. Boot stagger totals 1.8s (< 5s, WCAG 2.2.2); cursor blink is ~0.83 Hz (< 3 Hz, WCAG 2.3.1). The blink's residual 2.2.2 exposure is stated openly in §3.7 with Q3 as the alternative. |
| **3F — responsive and resilient** | Three-breakpoint table in §3.4 (≤ 800px, ≤ 640px, and the ≤ 34rem-tall footer un-pin); `100svh` fix; 320px@400% reflow check in §5.4. Resilience: the shell renders on 500 and with no global state (**U-11**), and the `MEM` empty state omits the item *and its separator* rather than printing a gap (`vitals_strip.html:11-16`). |

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**Overall state: implemented.** The shell ships, works, and is the site's best
existing asset. The findings below are refinements to a working system, not a
rebuild.

**Implemented and correct — keep, do not touch:**

- Full server-rendered shell on every route; `<html lang="en">` (`base.html:2`).
- Skip link present and first in DOM order (`base.html:17`).
- Compile-time metadata contract — Askama validates `title()`/`description()`/
  `section()` at build time (`pages.rs:10-12`), so a missing method is a build
  error, never a runtime failure.
- Anti-FOUC theme init as an external file, in `<head>`, before paint, because CSP
  forbids inline script (`theme-init.js:1-4`, `base.html:12`).
- ARIA APG button-menu with roving focus: arrows cycle, Home/End jump, Escape and
  Tab close, opening focuses the checked item (`main.js:43-72`). 24 `[data-mode]`
  buttons in 6 `role="group"` wrappers whose visible labels are `aria-hidden`
  (`base.html:36-79`) — correct.
- Global focus ring, never removed (`style.css:710`).
- All motion behind `prefers-reduced-motion` (`style.css:735-748`, `1358-1368`).
- **Skip link legibility (`a375a14`).** `.skip-link` carries `z-index: 60` and
  `.skip-link:focus` a `--surface` background, padding and a 1px `--accent`
  border (`style.css:541-557`). Iteration 1 of this spec filed this as F-02; it
  is fixed and is now a preservation requirement (§3.7).
- **Sticky header and footer (`a375a14`).** Both are `position: sticky` with an
  opaque `--bg` and `z-index: 50` (`style.css:566-573`, `818-827`), measured by
  `--header-h` / `--footer-h` (`:505-506`, overridden at `:1530-1533`), with
  `#content` given `scroll-margin-top` so the skip target clears the header
  (`:1159-1161`) and the footer un-pinning on short viewports (`:832-836`).
- **A contrast audit that measures where tokens actually render**
  (`c2f403e`, `0cdbbea`). `generate_themes.py`'s `USAGE` table (`:143-159`) holds
  every text token to 4.5 against **both** `--bg` and `--surface`, and
  `--check` (`:336-345`) exits non-zero on failure. All 23 themes pass today.
- Vitals strip: server-rendered, zero JS, zero polling, null-safe, with a
  designed empty state for `MEM` (`vitals_strip.html`, `state.rs:239-254`).
- Themed 404 and 500 with path escaping and no internal disclosure, both
  test-covered (`errors.rs:160-193`).
- Footer restored at commit `f8553d5` — the markup had been commented out with a
  malformed `--!>` terminator (`git show f8553d5`), leaving `.footer-inner` CSS
  (`style.css:838-845`) orphaned. It now carries the name and source link above
  the vitals strip (`base.html:89-97`). The hardcoded `updated on: 2026-08-02`
  span was correctly dropped in favour of the real build stamp.
- `generate_themes.py` is now group-aware (`MENU_GROUPS` at `:281-289`,
  `emit_menu()` at `:292-318`) and has its own internal drift guard
  (`:296-304`), fixing the `criteria.md` 5B reference case.
- A type scale and vertical rhythm exist (`style.css:480-507`) and the reading
  measure is capped at `--measure: 72ch` (`:499`), with code and Markdown tables
  keeping the full column and scrolling inside themselves (`:1163-1176`).

**Prototyped / partial:**

- `{% block head_extra %}` (`base.html:14`) and `{% block scripts %}` (`:100`) are
  defined but used by **zero** templates — extension points with no consumer.
- Sections `releases` (`releases.rs:42-44`), `status` (`status.rs:40-42`) and
  `error` (`errors.rs:78-80`, `:95-97`) exist but have no nav entry, so those
  pages highlight nothing. Acceptable behaviour, but nothing states it and
  nothing tests it.

**Planned / absent:**

- `og:image` / `og:url` / `og:site_name` / `twitter:card` / `<link rel="canonical">`
  — none present (`base.html:4-14`). `SOLARCORE_SPEC.md:190-191` describes an
  `og-card.png` "wired into the existing `og:` meta"; no such file exists in
  `static/img/` (only `favicon.svg`, `mark.svg`, `mark-sm.svg`, `vine-trace.svg`).
- Any test asserting anything about the shell. `status.rs:113-123` checks the
  string `"vitals-strip"` on `/` and `/blog` — that is the entire shell test
  surface across 35 tests in the repo.

**Findings — defects and gaps, in severity order:**

| ID | Severity | Finding | Evidence |
|---|---|---|---|
| **F-01** | **High (no-JS floor)** | With JS off the theme button is a **dead control**: focusable, named "Theme", `aria-haspopup="true"`, `aria-expanded="false"` — and pressing it does nothing, because the menu's `hidden` attribute is server-rendered and only `main.js` removes it. Nothing in the markup or CSS hides it when JS is absent. | `base.html:30-36` (`hidden` in markup), `main.js:55` (only remover) |
| **F-02** | ~~High (WCAG 2.4.7 / 1.4.3)~~ **RESOLVED in `a375a14`** | *Was:* the skip link on focus set only `left: 1rem`, with no background, no padding and no `z-index`, so it painted over the header's brand text. The sticky-header commit rewrote it: `z-index: 60` on `.skip-link`, and `--surface` background + padding + 1px `--accent` border on `.skip-link:focus`. **No longer a defect.** Carried forward as a preservation requirement, plus one new invariant: the skip link's `z-index` must stay above the header's `50`. | `style.css:541-557`; header is now `position: sticky; z-index: 50` (`:566-573`) |
| **F-03** | **High (no-JS)** | **`prefers-color-scheme` is honoured only in JavaScript.** There is no `@media (prefers-color-scheme: …)` rule anywhere in the **1,561-line** stylesheet; theme resolution lives entirely in `theme-init.js:10-12` / `main.js:22`. With JS off, every visitor gets bare `:root` = Lunarcore dark, including someone whose OS is set to light. | `grep -c prefers-color-scheme static/css/style.css` → **0**, re-verified against the current file; `style.css:16-33` |
| **F-04** | Medium (WCAG 1.3.1 / 4.1.2) | The active nav item is conveyed by `--accent` colour plus an underline — and the **hover** state uses the *same* underline (`.nav-link:hover::after, .nav-link.is-active::after { right: 0 }` — one selector list, one declaration), so hover and active are visually identical. There is **no `aria-current`** anywhere, so assistive tech gets nothing at all. Fixed on both axes: `aria-current="page"` for AT, and a weight/opacity split for sighted users (§3.3, §4.2). | `style.css:622`, `:724`; `base.html:24-27` |
| **F-05** | Medium | `theme-init.js` wraps the **`setAttribute` call inside the same `try` block** as the `localStorage.getItem`. If storage access throws (Safari private mode, Firefox with `dom.storage.enabled=false`, lockdown profiles), the catch swallows everything and `data-theme` is never set — so a light-preference user is forced into dark even though JS *is* running and `matchMedia` would have worked fine. | `theme-init.js:7-14` |
| **F-06** | Medium (WCAG 2.4.3) | Pressing **Tab** with the menu open calls `close()`, which sets `menu.hidden = true` **while focus is still on a menu item**. Focus is dropped to `<body>`, so the browser's default Tab restarts from the top of the document instead of continuing from the theme button. | `main.js:61`, `:56` |
| **F-07** | Medium (5B drift) | The asset cache-buster `?v=20260719-spectrum` is a **hand-typed literal repeated four times** in `base.html:11-13, 99`. `style.css` was last modified 2026-08-07; the query string still says `20260719`. It is **already stale** — returning visitors can be served a stale stylesheet against a changed template. Nothing fails when it drifts. | `base.html:11-13`, `:99`; `stat static/css/style.css` |
| **F-08** | Medium (5B drift) | `/learn` compares `self.section() == "wiki"` — a leftover from the `/wiki` → `/learn` rename (`router.rs:41-45` shows both the new routes and the legacy redirects). It works today, but nothing enforces the mapping: any new page returning `"learn"` would silently fail to highlight, and there is no test that would catch it. The comparison is a **stringly-typed match with no compiler help**. | `base.html:27`; `WikiPageTemplate::section()` at `wiki.rs:118-120` |
| **F-09** | Medium (3D) | `.vitals-strip` is a bare `<div>` carrying `aria-label="Server vitals"`. ARIA prohibits naming `role="generic"`; most screen readers **ignore this label entirely**. The readout is then announced as the raw token stream `"UP 00:14:32 REQ 1204 MEM 14 MiB v0.1.0 built …"` with no expansion. | `vitals_strip.html:7-18` |
| **F-10** | Medium (3D) | The primary `<nav>` wraps the brand **and the theme selector**, so the navigation landmark contains a colour-scheme control that is not navigation. Its name `"Primary navigation"` also double-announces ("Primary navigation, navigation"). Same duplication in the footer's `"Footer navigation"`. | `base.html:20-82`, `:92` |
| **F-10b** | Medium (3D, 2D) | **The same defect, twice more, outside `base.html`** — which is why F-10 needs a test and not an edit. `index.html:11` is `<nav aria-label="Quick navigation">`. `wiki_page.html:4` is `<aside class="wiki-sidebar" aria-label="Education wiki navigation">` — a nav-shaped name on a `complementary` landmark — wrapping a `<nav>` at `:7` that has **no** accessible name at all. Neither appears in iteration 1's surface inventory. Targets and ownership in §3.1; enforcement in **I-9**. | `index.html:11`, `wiki_page.html:4`, `:7` |
| **F-11** | Medium (2D consistency) | `<title>` policy is inconsistent: `"About — machinageist"`, `"Portfolio — machinageist"`, `"Writing — machinageist"`, `"Releases — machinageist"`, `"Status — machinageist"` — but blog posts and learn pages return the **bare** title with no site name, so a browser tab or a bookmark reads "Moving My Homelab" with no attribution. | `pages.rs:77`, `blog.rs:64`; vs `blog.rs:134-136`, `wiki.rs:110-112` |
| **F-12** | Medium (4A/4E) | **No `og:image`, `og:url`, `og:site_name`, `twitter:card`, or canonical link.** A link to this site pasted into Slack, LinkedIn, or Discord — the exact channel a hiring manager receives it through — renders as a bare text row. `og:type` is also hardcoded `website` even for blog posts and learn pages. | `base.html:4-14` |
| **F-13** | Low | `/static/<missing>` returns `ServeDir`'s bare 404 with an empty body, not the themed page. | `router.rs:59` |
| **F-14** | Low (3C) | The skip link's target `<main id="content">` is not focusable, so activating the link moves scroll but not keyboard focus in several browsers — the next Tab restarts from the top. Note this is now the *only* half left: `a375a14` added `#content { scroll-margin-top: calc(var(--header-h) + 1rem) }` (`style.css:1159-1161`), which fixes the scroll half against the sticky header but does nothing for focus. | `base.html:85` |
| **F-15** | Low | `focusedIndex()` returns `-1` when focus has left the menu while it is open; `ArrowUp` then computes `(-2 + 24) % 24 = 22`, landing on the second-to-last item instead of the last. | `main.js:45-48`, `:63` |
| **F-16** | Low (5B) | `generate_themes.py` emits the menu markup (`emit_menu()`, `:292-318`) and the `MODES`/`ICON` arrays (`emit_modes()` `:265-267`, `emit_icons()` `:270-274`), and has an internal guard against `MENU_GROUPS`/`THEMES` drift (`:296-304`) — but **nothing compares its output to the shipped `base.html`, `main.js`, or `theme-init.js`**. The generated artifacts are copy-pasted by hand. U-9 and U-10 close the two halves of this; note that U-10 must compare *sets*, because the generator emits two different orders on purpose (`:277-280`). | `generate_themes.py:292-318`; no consumer in `src/` or `tests/` |
| **F-17** | Low (5C) | **The hidden-coupling case.** `pages.rs:158` asserts `html.contains("CompTIA")` inside a test named `home_page_shows_concrete_work_without_strategy_narration`. `index.html` contains no such string — the assertion passes **only** because `base.html:6` and `:8` render `IndexTemplate::description()` (`pages.rs:44`) into `<meta name="description">` and `og:description`. A test that appears to be about the home page **body** is in fact pinned to a metadata string, routed through the shell. Editing `description()` for entirely unrelated SEO reasons breaks a test whose name mentions neither meta tags nor descriptions. The same shell coupling silently affects `html.contains("Proxmox")` (`:157`) and `html.contains("homelab")` (`:156`, `:214`), each satisfied by *both* body and meta, and it makes every negative assertion (`!html.contains("in training")`, `:161`) an implicit guard over all shell copy including 24 theme names. | `pages.rs:146-167`, `:204-223`; `base.html:6`, `:8`; `index.html` (no "CompTIA") |
| **F-18** | Low | `body { min-height: 100vh }` uses the legacy viewport unit; on mobile Safari the footer sits under the collapsing toolbar. More load-bearing since `a375a14` made the footer `position: sticky; bottom: 0`. | `style.css:517` |
| **F-20** | Low (2B) | **The shell chrome ignores the type scale it sits beside.** `3f96165` introduced `--text-xs` … `--text-2xl` (`style.css:480-507`) and the body copy adopted them, but every text size A2 owns is still a literal: `.nav-link` `0.875rem` (`:617`), `.brand` `1rem` (`:590`), `.theme-menu button` `0.8rem` (`:696`), `.theme-group-label` `0.65rem` (`:681`), `.site-footer` `0.8rem` (`:822`), `.vitals-strip` `0.75rem` (`:863`). The most-repeated component on the site is the one place the scale is not a system. `0.65rem` has no step and needs an A1 decision, not a local literal. | `style.css:480-507` vs `:590`, `:617`, `:681`, `:696`, `:822`, `:863` |
| **F-19** | Informational | HTML documents carry **no** `Cache-Control`. `/status` and `/status.json` correctly carry `no-store` and are test-pinned (`status.rs:125-135`), but every ordinary page embeds a live counter with no cache policy. Currently harmless (Cloudflare does not cache `text/html` by default), but nothing enforces it. | `grep -rn cache-control src/` → one hit, in `status.rs` |

### 7.2 Delta to spec

**New files (1)**

- `src/shell.rs` — `Section` enum, `NavItem`, `NAV`, `SITE_ORIGIN`,
  `asset_version()`, and the `#[cfg(test)]` module carrying U-1…U-11 **and**
  I-1…I-10. There is no new file under `tests/`: the crate is bin-only and an
  integration test cannot reach `router::build` or `shell::NAV` (§4.1).

**Modified files (10)**

| File | Change | Fixes |
|---|---|---|
| `src/shell.rs` (new) | as above | F-08, F-07 |
| `src/main.rs` | `mod shell;` (joining the six private module declarations at `:16-21`) | — |
| `src/state.rs` | widen `BUILD_TS_EPOCH` (`:34`) to `pub(crate)` so `shell::asset_version()` reads the one definition rather than calling `env!` twice | 5A |
| `templates/base.html` | `<nav>` narrowed to the links only; nav rendered from `crate::shell::NAV`; `aria-current="page"`; `aria-label` `"Primary"`/`"Footer"`; `<main tabindex="-1">`; `?v={{ crate::shell::asset_version() }}` ×4; **`og:type` becomes `{% block og_type %}website{% endblock %}` at `:9`**; `og:url` from `SITE_ORIGIN` + `canonical_path()`; `og:site_name`, `og:image`, `twitter:card`; `aria-haspopup="menu"` | F-04, F-07, F-10, F-12, F-14 |
| `templates/vitals_strip.html` | `role="group"`; `<span class="vh">` expansions per item; descriptive link label | F-09 |
| `templates/blog_post.html` | `{% block og_type %}article{% endblock %}` | F-12 |
| `templates/wiki_page.html` | `{% block og_type %}article{% endblock %}`; drop the `aria-label` from the `<aside>` (`:4`); name the inner `<nav>` `"Curriculum"` (`:7`) | F-12, F-10b |
| `src/handlers/{pages,blog,wiki,releases,status}.rs`, `src/errors.rs` | `section()` return type `&str` → `Section`; **new `canonical_path()` on all ten templates**; blog and wiki `title()` gain the `" — machinageist"` suffix (needs an owned `String` field or a `format!` helper); shorten nothing — the description edits belong to B1/B2/B6 | F-08, F-11, F-12 |
| `src/router.rs` | `ServeDir::new("static").not_found_service(errors::fallback_404.into_service())` at `:59` | F-13 |
| `static/css/style.css` | `@media (prefers-color-scheme: light) { :root:not([data-theme]) { … } }`; `:root:not([data-js]) .theme-select { display: none }`; `.vh` utility; `min-height: 100svh`; split `.nav-link:hover::after` from `.nav-link.is-active::after` by height and opacity (`:724`); migrate the shell chrome's six literal sizes onto `--text-*` (`:590`, `:617`, `:681`, `:696`, `:822`, `:863`) — **`.theme-group-label`'s `0.65rem` has no step and is an A1 request, not a retained literal**. **Not** the skip link: `a375a14` already fixed it | F-01, F-03, F-04, F-18, F-20 |
| `static/js/theme-init.js` | narrow the `try` to the storage read only; set `data-js` on `<html>` | F-01, F-05 |
| `static/js/main.js` | Tab → focus button **then** close; typeahead (APG defaults per §3.2); keep `aria-label` in sync with the current theme; clamp `focusedIndex() == -1` | F-06, F-15, §3.7 |
| `static/img/og-card.png` (new binary) + `og-card.svg` | checked in, produced by the two-command dev-time step in §4.5 | F-12 |
| `README.md` | record the `generate_brand.py` → `rsvg-convert` step so the card is reproducible | 5E |

**Not modified:** `docs/themes/generate_themes.py`. Iteration 1 asked it for a
focus-ring audit row; `c2f403e`/`0cdbbea` already audit `accent` against both
`--bg` and `--surface` at 4.5 (`:143-159`), which covers the ring at better than
WCAG 1.4.11's 3:1. See §5.4. A2's interest in that file is now a standing
constraint on A1, not an edit.

**Migrations / schema changes:** none — no database.

**New dependencies:** none.

**Suggested commit sequence** (each independently shippable and verifiable, per
the AI-engineering workflow's atomic-task rule):

1. `fix: make the shell honest with JavaScript disabled` — F-01, F-03, F-05
   (CSS fallback + `data-js` flag + narrowed try). *The auto-fail gate; ship
   first.* No longer blocked on A1: see §7.4.
2. `fix: main focus target and menu Tab order` — F-06, F-14, F-15. (F-02 dropped
   from this commit; `a375a14` shipped it.)
3. `refactor: one definition for sections and the primary nav` — `src/shell.rs`,
   the `state.rs` `pub(crate)` widening, F-08, F-04 on both axes
   (`aria-current` + the hover/active weight split), with U-1…U-3.
4. `fix: derive the asset cache-buster from the build stamp` — F-07, U-8.
5. `fix: landmark names across all four navigation surfaces` — F-09, F-10, F-10b
   (`base.html` and `wiki_page.html` halves), with **I-9**. `index.html` is B1's
   and lands separately; I-9 stays red until it does, which is stated in §5.2 and
   is the same ordering rule U-7 follows.
6. `feat: complete the head metadata contract` — F-12, I-7, I-8, the
   `{% block og_type %}` mechanism, `canonical_path()` on all ten templates, and
   `og-card.png`. **Blocked on the §4.5 rasteriser step**, which is the operator's
   two commands, not another feature.
7. `fix: serve the themed 404 for missing static assets` — F-13, I-4.
8. `test: pin the shell contract` — remaining tests. **Ordering trap:** this
   commit carries U-6a and U-7, both of which are red until B6 and B1/B2 land
   their copy (§7.4 requests 1 and 3). Either sequence commit 8 after those, or
   split the two guards out into a commit 8b that ships with them. Do not weaken
   either guard to make commit 8 green on its own — that is the failure mode
   criterion 1F names.
9. `fix: title suffix on blog and learn pages` — F-11, U-5.
10. `style: move the shell chrome onto the type scale` — F-20, pending A1's
    ruling on a step below `--text-xs`.

### 7.3 Estimated scope

**M**, trending to the top of M.

Justification: no new runtime dependency, no new route, no data model, no
migration, and one new source file. The work is ten surgical commits across
existing files plus one test module. The volume is in the **tests** (~22 cases)
rather than the features, which is the correct ratio for a foundation component
that every other feature inherits — a shell regression breaks every page at once.

Three items carry above-average risk and account for the "top of M": the
`section() -> Section` type change touches five handler files and both error
templates in a single commit (compiler-guided, so mechanical but wide); the
blog/learn `title()` suffix requires those structs to own a `String` rather than
return a borrow of `post.title` — a small ownership change with a clippy
implication; and `canonical_path()` adds a fourth required method to all ten
templates, which is wide but trivial per site.

Not counted in the estimate, because iteration 1 double-counted them: F-02 and
the skip-link CSS shipped in `a375a14`, and the `generate_themes.py` audit row
turned out to be already present.

Not S: it edits every page's rendered output. Not L: nothing is redesigned, no
new surface is introduced, and the existing behaviour is largely correct.

### 7.4 Blocking dependencies

**Blocking A2:**

| Dependency | Feature | What A2 needs |
|---|---|---|
| Token values for the `prefers-color-scheme: light` no-JS fallback block | `A1` | The Solarcore token set as a reusable declaration (a `@media`-scoped `:root:not([data-theme])` copy, or A1 restructures so a theme's tokens can be applied under two selectors without duplication). **Commit 1 is blocked on this.** |
| A type-scale step below `--text-xs` (`0.75rem`), or a ruling that `.theme-group-label` should move up to it | `A1` | Blocks commit 10 only (F-20). The scale is A1's; A2 will not invent a seventh step or keep the `0.65rem` literal. |
| Theme-name trademark posture | `A1` | §6.2 flags Dracula / Nord / Gruvbox / Game Boy / Tron. A1's call. |
| Explicit `Cache-Control` policy for HTML documents | `A3` | §4.7 — the vitals strip is only truthful if HTML is not edge-cached. |

**No longer blocking — the focus-ring audit.** Iteration 1 listed
"`--accent` vs `--bg` at ≥ 3:1" here as work A2 was waiting on. It is done and it
was done better: `generate_themes.py:143-159` holds `accent` to **4.5** against
both `--bg` and `--surface`, and `--check` exits non-zero (`:336-345`). This moves
from the blocking table to a **standing constraint on A1**: do not relax that row
below 3:1, do not narrow it back to `bg` alone, and do not drop `surface` from any
token's backdrop list — the focus ring's 1.4.11 compliance across 23 themes rests
entirely on it, and nothing else would notice if it went.

**Blocked by A2 (A2 must land first):** every page feature `B1`–`B6` and `C1`–`C4`
inherits `Section`, the nav table, the title/description contract, and the
`head_extra` override point. `C3` study-tools additionally inherits the
`{% block scripts %}` extension point and the 150-line JS ceiling.

**Cross-feature requests A2 files (not implemented here):**

1. → `B1` **home** and `B2` **about**: rewrite `description()` (`pages.rs:44`,
   `:81`) and the about bio (`pages.rs:91-93`) so `"CompTIA study"` /
   `"the CompTIA stack"` match the 2026-08-02 spine (RHCSA → CCNA → Security+,
   Network+ dropped). Criterion 1D. **A2 supplies the guard (U-7); B1/B2 supply
   the words.** Ordering trap: U-7 fails on exactly these two strings the moment
   it lands, so it must be committed *with* or *after* the copy fix, not before.
   Note that `"Security+"` is deliberately **not** banned — it is CompTIA and it
   is on the live spine.
2. → `B1`: rewrite the `home_page_shows_concrete_work_without_strategy_narration`
   assertions so each one names what it is actually testing (F-17). The
   `"CompTIA"` assertion (`pages.rs:158`) belongs in a metadata test, not a
   page-body test — and it must be re-pointed at the **new** spine wording, not
   the retired one. Sequence matters: after request 1 lands there is no
   `"CompTIA"` string in `IndexTemplate::description()` for it to find, so
   relocating the assertion unchanged would simply move a failure. The relocated
   test asserts that the home page's `<meta name="description">` names the
   current spine; the page-body test keeps `"Proxmox"` and `"homelab"`, which
   really are in `index.html`.
3. → `B6` **releases**: `ReleasesTemplate::description()` (`releases.rs:40`)
   publishes GeistScope naming through `<meta>` and `og:description`. Confirm it
   clears the §1C gate — **and while rewriting it, clear 50 characters.** It is
   49 today, which is the sole reason U-6a is red on arrival. One edit closes
   both.
4. → `B4` **writing** and `B5` **learn**: decide the frontmatter `summary:`
   length policy. Four served summaries exceed the 160-character `<meta>` budget
   (227 / 220 / 217 / 168 — measurements and filenames in §5.1), and the field
   serves both the `<meta>` tag and the list card, which want different lengths.
   Two candidate resolutions are stated in §5.1; A2 declines to pick one because
   it is a content-model decision, and declines to write a guard it cannot turn
   green. Whichever is chosen, the guard belongs with the content feature.
5. → `B1`: relabel `index.html:11` from `aria-label="Quick navigation"` to
   `"Quick links"`. A2 establishes the rule and ships **I-9** to enforce it
   site-wide; B1 owns the one instance inside a page body. I-9 stays red until
   this lands — same ordering rule as U-7.
6. → `A1`: the items in the blocking table above, plus the standing constraint
   on the `USAGE` contrast row.

---

## 8. Open Questions

- **Q1 — Should theme selection get a real no-JS path (a `GET /theme?mode=…`
  route setting a cookie, with the server stamping `data-theme` on `<html>`)?**
  Blocks: §3.2 Flow B′, §4.2, §4.4.
  *Trade:* it would make the theme genuinely universal and would be the purest
  reading of "JS is an enhancement layer." Against it: a cookie makes every HTML
  response `Vary`-dependent, fragments any future edge cache, adds a route and a
  form control to a header that is currently four links and a button, and puts
  server state behind something that is purely a display preference.
  **A2 recommends no.** The spec'd fallback — honour the OS preference in CSS,
  hide the control that cannot work — satisfies 3A without inventing state. If
  Jeff wants the cookie, it is a small addition to `A3`, not a redesign here.

- **Q2 — Should `/releases` and `/status` get primary-nav entries?**
  Blocks: §4.2 `NAV`, U-3.
  Both are real, shipped, and interesting to the engineer-peer reader; `/status`
  in particular is the site's best thirty-second differentiator and is currently
  reachable **only** through the small faint version link in the footer
  (`vitals_strip.html:18`). Against: five nav items dilute a nav whose current
  restraint is a design asset, and `/releases` is GeistScope-adjacent (§1C).
  *A2's lean:* leave the nav at four, but give the vitals link a clearer
  accessible name (already in §3.7) so `/status` is discoverable to AT users too.
  Jeff's call.

- **Q3 — Keep the infinite brand cursor blink?**
  Blocks: §3.5, §3.7.
  It is the wordmark's whole character and Lens 2E budgets spectacle to chrome.
  It is also the only unbounded animation on the site, and WCAG 2.2.2 asks for a
  pause/stop/hide mechanism for blinking over five seconds —
  `prefers-reduced-motion` is the accepted mechanism, but it is a *user-agent*
  control, not an in-page one. Options: (a) keep as-is (A2's default);
  (b) cap it at ~8 blinks with `animation-iteration-count`, so it reads as a
  terminal waking up and then settles; (c) drop it. (b) is a one-token change
  that keeps the effect and removes the exposure entirely.

- **Q4 — Should the `og:image` card be per-page or one site-wide card?**
  Blocks: §4.5, F-12, I-7.
  Site-wide (one static `og-card.png` with mark + wordmark) is ~30 minutes and
  covers every URL. Per-page (title rendered onto the card) would make shared
  blog and learn links substantially more clickable but needs either a build-time
  generator or a runtime image endpoint — the latter would be the first
  non-HTML dynamic route on the site. *A2 recommends site-wide now*, and filing
  per-page as a separate feature if link-sharing turns out to matter.

- **Q5 — What is the correct blog/learn `<title>` format?**
  Blocks: §4.3 Contract S-1, U-5, F-11.
  Candidates: `"Moving My Homelab — machinageist"` (matches the existing pattern),
  or `"Moving My Homelab — Writing — machinageist"` (section-aware, better for a
  tab strip with several of the site's pages open, worse for narrow tabs).
  *A2 recommends the two-part form* for consistency with the five pages that
  already use it.

- **Q6 — Is a `<noscript>` message wanted anywhere?**
  Blocks: §3.6 E-05.
  A2's position is **no**: the site works without JS, so there is nothing to
  apologise for, and a `<noscript>` banner would be the only thing on the page
  drawing attention to an absence the reader already chose. The theme control
  simply not being there is the more confident answer.

- **Q7 — Should U-9 (theme menu matches `generate_themes.py --menu`) shell out to
  Python in CI?**
  Blocks: §5.1.
  It is the only guard that would have caught the original 5B failure case at the
  source, but it puts a Python dependency into a Rust CI job. Alternative: port
  `MENU_GROUPS` into `src/shell.rs` as the single source of truth and have the
  Python script read *it*, inverting the direction of trust so the drift becomes
  structurally impossible. That is cleaner but expands A1's territory into A2's,
  so it needs Jeff's ruling on ownership.

---

**Verification commands for this feature** (must all pass, and all run in CI per
`.github/workflows`):

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
python3 docs/themes/generate_themes.py --menu   # must match templates/base.html
```

Plus the manual pass in §5.4, minimally: Lunarcore and Solarcore, JS on and off,
`prefers-reduced-motion: reduce`, 360px and 1440px, and one screen-reader run over
the landmark list.

**Documents that must be updated in the same change** (criterion 5E):

- `docs/solarcore/SOLARCORE_SPEC.md` — §6.4 "Footer status line" describes a
  `SYS: … │ OPERATOR: … │ STATUS: ONLINE │ UPDATED: …` readout that never shipped;
  what shipped is the real-data vitals strip. §7 describes a split-colour
  `MACHINA`/`GEIST` wordmark that never shipped. Per criterion 2A the **shipped
  site wins** — `A1` owns the rewrite, and A2's shipped shell is the reference it
  must be rewritten against.
- `README.md` — if the local-run instructions mention the shell's JS surface.
- `docs/agent-context/README.md` — **does not exist** in this repo despite being
  referenced by the global `CLAUDE.md` index (`~/mg-server/docs/agent-context/README.md`).
  Creating it is out of A2's scope, but the shell's decisions (no-JS floor, JS
  line ceiling, the `Section`/`NAV` single source of truth) are exactly the kind
  of durable constraint it should carry.
