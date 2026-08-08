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

### 1.3 Success signal

**Primary (measurable):** with JavaScript fully disabled, every route in
`router.rs:38-58` renders complete, readable, navigable HTML, in a colour scheme
that respects the visitor's OS light/dark preference, with no control on screen
that does nothing when pressed. Verified by `cargo test --all-targets` (the shell
contract tests in §5.1–5.2, which parse the *served bytes*, not a browser) plus
one manual pass with JS off.

**Secondary (observable):** a keyboard-only user can go from page load to any nav
destination in ≤ 3 Tab presses (skip link → brand → nav), and the current section
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

> As a **reader on a 360px phone at 200% text zoom**, I want the header to wrap
> rather than clip and the theme menu to stay on screen, so that the site is
> usable on the device most inbound links get opened on.

---

## 3. UX Specification

### 3.1 Screen / view inventory

The shell introduces no standalone screens; it modifies **every** screen. The
enumerable surfaces it owns:

| Surface | Reached by | New / modified | Layout pattern |
|---|---|---|---|
| **Header chrome** | Present on all 13 routes (`router.rs:38-58`) | Modification (exists, `base.html:19-83`) | Full-bleed bar, 1px bottom border, 900px centred inner row |
| **Primary nav** | Inside header | Modification | Horizontal link row, wraps at ≤640px (`style.css:1447-1457`) |
| **Theme menu (popover)** | Header → theme button (`◐`) | Modification | Absolutely-positioned panel, right-aligned, `max-height: min(72vh, 30rem)` scrolling (`style.css:619-636`) |
| **`<main>` content region** | All routes | Modification (`base.html:85-87`) | 900px column, `flex: 1` sticky-footer child |
| **Footer** | All routes | Modification (`base.html:89-97`) | Two rows: name/source row, then vitals strip |
| **Vitals strip** | Inside footer | Modification (`vitals_strip.html`) | Single wrapping line of monospace readouts |
| **404 page** | Any unmatched URL → `errors::fallback_404` (`router.rs:62`) | Modification (`error_404.html`) | Boot-log column, 700px, inside the standard shell |
| **500 page** | Any `SiteError` other than `PostNotFound`/`PageNotFound` (`errors.rs:113-123`) | Modification (`error_500.html`) | Boot-log column, 700px, inside the standard shell |
| **Static-asset 404** | `/static/<missing>` | **New** — currently falls through to `ServeDir`'s bare 404 (see §7.1) | Themed 404 (same as above) |
| **Skip link** | Tab once from page load | Modification (`base.html:17`) | Off-canvas until focused |

No modals, sheets, or drawers. The theme menu is the only overlay and it is
non-modal (does not trap focus, does not block the page — closing on outside
click at `main.js:57`).

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
   accent colour **+** a full-width 1.5px underline (`style.css:697-699`) **+**
   `aria-current="page"` (new, §4.2).
6. Click/Enter navigates. Full page load. No client-side routing, no transition
   choreography, no history manipulation.

**Flow B — change theme (enhancement layer).**

1. Click or Enter/Space on the theme button, or ArrowDown/ArrowUp while it is
   focused (`main.js:68-72`).
2. Menu unhides; focus moves to the **currently checked** item, not the first
   (`main.js:55`) — correct for a `menuitemradio` group per the ARIA APG.
3. Arrows cycle with wraparound; Home/End jump to ends (`main.js:62-65`).
   *New:* printable-character typeahead jumps to the next item whose label starts
   with that character (24 items justifies it; APG recommends it).
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
   1.8s with `steps(1, end)` and `fill-mode: both` (`style.css:1279-1294`) — a
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
`@media (prefers-reduced-motion: no-preference)` (`style.css:710-723`,
`1279-1289`), except the CRT scanline overlay which is instead *removed* under
`reduce` (`style.css:461-470`).

### 3.3 Layout descriptions

**Header** (`base.html:19-83`, `style.css:545-597`)

```
<header class="site-header">                 padding 1.25rem 2rem, border-bottom 1px --border
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

**Theme menu** (`base.html:36-79`, `style.css:619-678`) — 6 `role="group"`
wrappers, 24 `[data-mode]` buttons (verified by count), each
`role="menuitemradio"` with an `aria-hidden` glyph and a text label. The visible
group heading is a `<span aria-hidden="true">`; the group's accessible name comes
from `aria-label` on the wrapper (`base.html:37`, comment at `style.css:639-640`).
Checked item gets `.is-current` → accent colour **plus** a `✓` pseudo-element
(`style.css:677-678`) — that check mark is the non-colour cue and must stay.

**`<main id="content">`** (`base.html:85-87`, `style.css:732-738`) — `flex: 1`,
`max-width: 900px`, `padding: 3.5rem 2rem 5rem`, centred. Prose inside is further
capped at `--measure: 72ch` (`style.css:497-499`); code keeps the full column.
*New:* `tabindex="-1"` so the skip link actually moves keyboard focus.

**Footer** (`base.html:89-97`, `style.css:790-835`)

```
<footer class="site-footer">                  padding 1.5rem 2rem, border-top 1px --border, --text-faint, 0.8rem
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
| `Tab` (first press from load) | Document | Reveals skip link |
| `Enter` | Skip link | Moves focus **and** scroll to `<main>` (requires the new `tabindex="-1"`) |
| `Tab` / `Shift+Tab` | Page | Standard order: skip link → brand → About → Portfolio → Writing → Learn → theme button → main content links → footer source → vitals `/status` |
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

**Responsive.**

| Breakpoint | Behaviour | Source |
|---|---|---|
| > 640px | Header 1.25rem/2rem padding, nav-links gap 2rem, main 900px | `style.css:545-546`, `585-588`, `732-738` |
| ≤ 640px | Header padding 1rem/1.25rem; `.site-nav` wraps (`gap: 0.75rem 1.25rem`); `.nav-links` wraps; main padding 2.5rem/1.25rem; footer 1.25rem | `style.css:1444-1461` |
| 320px @ 400% zoom (WCAG 1.4.10 Reflow) | Header wraps to 2–3 rows; theme menu `min-width: 9.5rem` still fits; no horizontal page scroll | Requires verification, §5.4 |

*New requirement:* `body { min-height: 100vh }` (`style.css:510`) becomes
`min-height: 100svh` with a `100vh` fallback declaration first, so the sticky
footer does not sit under mobile Safari's collapsing toolbar.

### 3.5 Transitions & animation

| What | Duration / easing | Gate |
|---|---|---|
| Body + chrome colour swap on theme change | 0.3s ease (bg/border), 0.25s ease (colour) | `prefers-reduced-motion: no-preference` (`style.css:710-716`) |
| Link/nav/brand colour | 0.18s ease | same block |
| Nav underline sweep (`right: 100% → 0`) | 0.22s ease | same block (`style.css:719`) |
| Theme menu open (`theme-pop`: 4px rise + fade) | 0.16s ease-out | same block (`style.css:721`, `725`) |
| Brand terminal cursor `▍` blink | 1.2s `step-end` **infinite** | same block (`style.css:722`, `726`) |
| 404/500 boot lines | 0.2s `steps(1,end)`, staggered 0.15s→1.8s, `fill-mode: both` | `prefers-reduced-motion: no-preference` (`style.css:1279-1289`) |
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

### 3.6 Error states

| ID | Trigger | Presentation | Why that presentation | Recovery | Data loss |
|---|---|---|---|---|---|
| **E-01** | Unmatched route | **Full page**, HTTP 404, themed boot-log (`error_404.html`) | A wrong URL is a navigation event, not a transient notice; a toast would be wrong and a banner would leave a blank page under it. Full page keeps header nav available. | Header nav (4 destinations) + brand + `(A)bort → return home` | No |
| **E-02** | `SiteError::Io`, `MissingFrontmatter`, `FrontmatterParse`, `DateParse`, `InvalidPath` | **Full page**, HTTP 500, themed kernel-panic (`error_500.html`) | Same reasoning; additionally the response must carry **nothing** internal — a banner over a partial page risks leaking half-rendered content | Header nav + `reboot → return home` | No |
| **E-03** | 500 template itself fails to render | Plain-text `500 internal server error` | Last-resort; cannot depend on Askama (`errors.rs:117-121`) | Browser back | No |
| **E-04** | `/static/<missing>` | **Currently** a bare `ServeDir` 404 with an empty body (`router.rs:60`). **Target:** the themed 404. | Consistency — "the shell owns every 404" is only true if it is | Header nav | No |
| **E-05** | JS unavailable / blocked / `localStorage` throws | Theme control **not rendered as an interactive element**; palette follows OS preference | A control that does nothing is worse than no control (see §3.7). Silent, not an error message — losing a colour picker is not an error worth interrupting a reader over | Read the site normally; OS preference is honoured | No |
| **E-06** | `Status::current()` with `APP_STATE` unset | Zeros, not an error | `state.rs:240-253`; the vitals strip must never be the reason a page fails | Page renders normally | No |
| **E-07** | `rss_cache` mutex poisoned | Recovers via `unwrap_or_else(into_inner)` (`state.rs:107-112`) | A metric is not worth a panic | Page renders normally | No |
| **E-08** | A page template omits `title()`/`description()`/`section()` | **Compile error.** Askama validates at build time (`pages.rs:10-12`) | The strongest possible presentation: it never reaches a user | Fix the code | N/A |
| **E-09** | `<meta name="description">` copy goes stale relative to the live cert spine | **No current signal.** Target: a shell contract test fails in CI (§5.1) | Copy drift is invisible by definition — it must fail loudly | Update copy | No |

**Data-loss risk across the whole feature: none.** The shell stores exactly one
thing (`localStorage.theme`, a colour preference) and reads it defensively.

### 3.7 Accessibility

**Landmarks and headings (target).**

| Landmark | Element | Accessible name |
|---|---|---|
| `banner` | `<header class="site-header">` | — (implicit, one per page) |
| `navigation` (primary) | `<nav>` wrapping **only** `.nav-links` | `"Primary"` |
| `main` | `<main id="content" tabindex="-1">` | — |
| `contentinfo` | `<footer class="site-footer">` | — |
| `navigation` (footer) | `<nav>` in `.footer-inner` | `"Footer"` |
| `group` | `.vitals-strip` | `"Server vitals"` |

Heading outline: the shell contributes **no** headings; each page supplies
exactly one `<h1>` (verified: all 10 content templates have exactly one, `base.html`
has zero). §5.1 pins this so a future page cannot ship with zero or two.

**Per-element AT contract.**

| Element | Role | Name | State / properties |
|---|---|---|---|
| Skip link | `link` | `"Skip to content"` | Visible only on focus; **must gain a solid `--surface` background, padding, border, and `z-index`** — see F-02 |
| Brand SVG | — | — | `aria-hidden="true" focusable="false"` (correct today, `base.html:21`) |
| Brand link | `link` | `"machinageist"` | Decorative `▍` is a CSS `::after` (`style.css:702`) → never in the a11y tree |
| Nav link (inactive) | `link` | `"About"` etc. | — |
| Nav link (active) | `link` | same | **`aria-current="page"`** (new) + accent colour + underline |
| Theme button | `button` | **`"Theme: Lunarcore"`** (new — currently the static `"Theme"`, `base.html:30`) | `aria-haspopup="menu"`, `aria-expanded` toggled by `main.js:55-56` |
| Theme menu | `menu` | `"Theme"` | `hidden` toggled; not modal |
| Theme group | `group` | `"Core"` / `"Editor"` / … | Visible label is `aria-hidden` (`base.html:38`) — correct, avoids double-announcing |
| Theme item | `menuitemradio` | `"Lunarcore"` etc. | `aria-checked`, `tabindex="-1"` (roving, `main.js:43`) |
| Vitals items | — | — | **New:** each carries a visually-hidden expansion (`"Uptime"`, `"Requests served"`, `"Resident memory"`) so `"UP 00:14:32"` is not read as two opaque tokens |
| Vitals link | `link` | **New:** `"Full status — version 0.1.0, built 2026-08-07 14:22 UTC"` | Currently the name is the bare version string, which does not say where the link goes |

**Custom actions:** N/A — no composite widget beyond the theme menu, which is
covered by the APG button-menu pattern.

**Text scaling / dynamic type.** Root type is `15px` on `body` (`style.css:513`)
with a 1.125-ratio `rem` scale (`style.css:480-500`) — sizes are `rem`, so browser
font-size settings scale them. Two carried requirements: at 200% zoom the header
must wrap rather than clip (already true via `style.css:1447-1450`), and at 400%
zoom on a 320px viewport the page must not scroll horizontally (§5.4).

**Colour-independent state.**

| State | Colour cue | Non-colour cue |
|---|---|---|
| Active nav section | `--accent` (`style.css:597`) | Full-width 1.5px underline (`style.css:699`) **+ `aria-current="page"`** (new) |
| Hover nav | `--text` (`style.css:596`) | Same underline — **currently indistinguishable from active** (F-04) |
| Checked theme | `--accent` (`style.css:677`) | `✓` pseudo-element (`style.css:678`) + `aria-checked` |
| Menu open | — | `aria-expanded` |
| Focus | `--accent` outline | 2px outline + 2px offset (`style.css:685`) |

**Focus order and keyboard navigability.** DOM order is already correct: skip
link (`:17`) → brand (`:21`) → nav links (`:24-27`) → theme button (`:30`) →
main (`:85`) → footer (`:89`). No `tabindex > 0` anywhere. No focus trap. Three
target fixes: `<main tabindex="-1">`, the skip-link background, and the Tab-close
focus-restore ordering (F-06).

**Known residual risk (stated, not hidden):** `.brand::after` blinks forever
(`style.css:722`, `726`). WCAG 2.2.2 asks for a mechanism to pause/stop/hide
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
  base.html               ← head contract, nav loop, landmarks, main tabindex
  vitals_strip.html       ← group role + visually-hidden labels
  error_404.html          ← unchanged
  error_500.html          ← unchanged
static/
  css/style.css           ← skip-link visibility, OS-preference fallback, .vh utility
  js/main.js              ← typeahead, Tab focus order, aria-label sync
  js/theme-init.js        ← narrow the try/catch, set a JS-present flag
build.rs                  ← already stamps BUILD_TS (state.rs:34)
docs/themes/generate_themes.py
                          ← emit_menu() already group-aware (lines 244-282); add a test that
                            compares its output to the shipped base.html
```

`src/shell.rs` is a new module and is the *only* new file. It is justified because
four separate concerns (nav definition, section identity, asset versioning, and
the metadata contract) are currently hand-duplicated across `base.html` and six
handler files with nothing keeping them honest.

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
// Asset versioning — one cache-buster, derived, never hand-edited
// -----------------------------------------------------------------------

// Build timestamp as epoch seconds, stamped by build.rs
const BUILD_TS_EPOCH: &str = env!("BUILD_TS");

// Return the query-string value appended to every versioned static URL
pub fn asset_version() -> &'static str {
    BUILD_TS_EPOCH
}
```

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
```

### 4.3 API contracts

The shell exposes no HTTP endpoints of its own. Its contracts are template-level.

**Contract S-1 — the page metadata contract (compile-enforced).** Every Askama
struct rendered through `base.html` must implement:

| Method | Signature | Rendered into | Rule |
|---|---|---|---|
| `title` | `fn title(&self) -> &str` | `<title>` (`base.html:10`) and `og:title` (`:7`) | Must end in `" — machinageist"` for every page except the home page (whose title is the bare wordmark). Currently violated by `blog.rs:134-136` and `wiki.rs:106-108`. |
| `description` | `fn description(&self) -> &str` | `<meta name="description">` (`:6`) and `og:description` (`:8`) | 50–160 characters. **Must be treated as user-visible copy** and therefore falls under Lens 1 claim discipline — see §6.3. |
| `section` | `fn section(&self) -> Section` | Nav active-state comparison | Return type changes from `&str` to `Section` |

Askama resolves these at **compile time** — a missing or misspelled method is a
build error (`pages.rs:10-12` documents this), so there is no runtime error case
and no auth dimension. This is the strongest guard in the shell and the spec
leans on it rather than adding a runtime check.

**Contract S-2 — the `head_extra` block.** `base.html:14` defines
`{% block head_extra %}` and **no template currently uses it**. It becomes the
per-page override point for:

- `<meta property="og:type" content="article">` on `blog_post.html` and
  `wiki_page.html` (base defaults to `website`, `base.html:9`)
- `<link rel="canonical">` where a page is reachable at more than one URL — the
  `/wiki/*` legacy redirects (`router.rs:44-46`) are 3xx so they need none, but
  this is the hook if that ever changes

`{% block scripts %}` (`base.html:100`) is likewise unused. It stays as the
extension point for `C3` study-tools; A2 asserts only that anything landing there
must satisfy the no-JS floor on its own.

**Contract S-3 — static asset 404.** `router.rs:60` becomes:

```rust
.nest_service(
    "/static",
    ServeDir::new("static").not_found_service(get(errors::fallback_404).into_service()),
)
```

so a missing static file returns the themed 404 rather than `ServeDir`'s
empty-bodied one. (tower-http 0.5, `Cargo.toml:14` — API confirmed available at
that version; verify at implementation time.)

**Rate limiting / pagination / auth:** N/A — the shell is stateless chrome. The
rate limiter (`router.rs:71-75`) is A3's and applies to every route uniformly.

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
every handler solely to render a footer would put an ops concern into thirteen
signatures. The cost is that **every template render implicitly touches process
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
| `static/img/og-card.png` (1200×630) | `og:image` — currently **absent**, so every link shared into Slack/LinkedIn/Discord previews as text only. `SOLARCORE_SPEC.md:190-191` claimed it would be "wired into the existing `og:` meta in base.html"; it never shipped. | **Planned.** Generated from the existing `mark.svg` by `docs/solarcore/generate_brand.py` — no third-party art, no licence question. |

**Infrastructure:** none. No CDN (CSP is `default-src 'self'`,
`security_headers.rs:41-50`), no webfonts (all font stacks are system stacks —
`generate_themes.py:11-15`), no third-party services.

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
- **`ServeDir::not_found_service`** — confirm the exact method name against
  tower-http 0.5 at implementation time before writing the code.
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
| CSS payload | `style.css` 41 KB (all 23 themes inline, one file, `style.css:1-460`) | +~0.6 KB for the OS-preference fallback block | One file, one request, no critical-CSS split |
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

### 5.1 Unit tests

New module `src/shell.rs::tests` plus additions to `src/errors.rs::tests`.

| # | Name | Setup | Assertion | Edge case covered |
|---|---|---|---|---|
| U-1 | `every_nav_section_is_reachable` | Iterate `shell::NAV` | Each `item.href` matches a route registered in `router::build` (drive `oneshot` per href) | A nav link to a route that was renamed or removed |
| U-2 | `every_nav_item_highlights_its_own_page` | For each `NAV` item, request its `href` | Body contains `class="nav-link is-active" aria-current="page"` on exactly one link, and it is that item's label | **The `/learn` → `section "wiki"` class of bug: a section string that matches nothing** |
| U-3 | `pages_outside_the_nav_highlight_nothing` | Request `/`, `/releases`, `/status`, `/no-such-page` | Body contains zero `is-active` and zero `aria-current` | Silent mis-highlighting on off-nav routes |
| U-4 | `every_page_declares_exactly_one_h1` | Render each of the 10 content templates + both error templates | `matches("<h1").count() == 1` | A page shipping with no heading or a duplicated one |
| U-5 | `page_titles_carry_the_site_name` | Render every template | `<title>` ends with `" — machinageist"`, except home which equals `"machinageist"` | **Current violation: `blog.rs:134-136` and `wiki.rs:106-108` return a bare title** |
| U-6 | `descriptions_are_within_meta_length` | Every `description()` | `50 <= len <= 160` | A description silently truncated by search engines |
| U-7 | `descriptions_do_not_carry_retired_claims` | Every `description()` | Does not contain `"Network+"`, `"A+"`, `"the CompTIA stack"`, `"offensive security"`, `"red-team"`, `"pentest"`, `"production-grade"`, `"enterprise"`, `"SRE"` | **Criterion 1D/1E — the `<meta>` tag is user-visible copy that no page-body test currently guards** |
| U-8 | `asset_version_is_derived_not_literal` | `shell::asset_version()` | Non-empty, parses as an integer, and `base.html` contains no literal `?v=2026` string | **Criterion 5B — the current `?v=20260719-spectrum` string is hand-typed and already stale** |
| U-9 | `theme_menu_matches_the_generator` | Run `python3 docs/themes/generate_themes.py --menu`, normalise whitespace | Output equals the `.theme-group` block in `base.html` | **Criterion 5B — `generate_themes.py:255-282` has an internal drift guard but nothing compares its output to the shipped file** |
| U-10 | `theme_modes_match_the_menu_buttons` | Parse `[data-mode]` values from `base.html`; parse `MODES` from `main.js` and `theme-init.js` | All three lists are equal, in order, length 24 | A theme added to the menu but not the JS allowlist becomes a no-op button |
| U-11 | `vitals_strip_renders_with_no_global_state` | `Status::current()` with `APP_STATE` unset | Returns zeros; strip renders; no panic | `state.rs:240-253` — the 500 page depends on this |
| U-12 | `requested_path_is_html_escaped` | **exists** (`errors.rs:160-169`) | Keep unchanged | XSS via the 404 path echo |

*Marked `#[ignore]` if the CI image lacks Python:* U-9 additionally gets a plain
`cargo test` variant that asserts the group/label/slug **structure** without
shelling out, so the guard is never fully absent.

### 5.2 Integration tests

Router-level, `tower::ServiceExt::oneshot` (the pattern already used at
`errors.rs:171-182` and `status.rs:84-89`).

| # | Name | Assertion |
|---|---|---|
| I-1 | `every_route_renders_the_full_shell` | For all 11 HTML routes **plus** an unmatched URL **plus** a forced 500: body contains the skip link, `<header class="site-header"`, `aria-label="Primary"`, `<main id="content"`, `<footer class="site-footer"`, and `vitals-strip`. Extends `status.rs:113-123`, which today covers only `/` and `/blog`. |
| I-2 | `shell_needs_no_javascript_to_be_complete` | For every HTML route: strip all `<script …></script>` elements from the body, then assert the remainder still contains every nav `href`, the skip link target `id="content"`, the footer source link, and the `/status` link. **This is the machine-checkable form of the no-JS floor.** |
| I-3 | `no_inline_script_or_style_survives_csp` | No served HTML contains `<script>` with a body, `on[a-z]+=` handler attributes, or a `style=` attribute | Guards `security_headers.rs:41-50` from being quietly violated by a template edit |
| I-4 | `missing_static_asset_returns_the_themed_404` | `GET /static/nope.css` → 404 **and** body contains `SECTOR NOT FOUND` | **E-04; currently fails** |
| I-5 | `error_pages_carry_working_navigation` | 404 and 500 bodies contain every `NAV` href | A 404 that stranded the visitor |
| I-6 | `internal_error_page_leaks_nothing` | **exists** (`errors.rs:184-193`) — extend the allowlist to also reject `"panicked"`, `"askama"`, `"axum"`, `"tower"` | Framework fingerprinting via an error page |
| I-7 | `head_carries_the_required_meta_set` | Every route: exactly one `<title>`, one `<meta name="description">`, `og:title`, `og:description`, `og:type`, `og:url`, `og:image`; `<html lang="en">` | Silent loss of a meta tag during a template edit |
| I-8 | `article_pages_declare_og_type_article` | `/blog/:slug` and `/learn/:slug` → `og:type` is `article`; all others `website` | The `head_extra` override actually wired up |

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
| **All 23 themes** (`style.css:16-460`) | Header border, nav active underline, theme-menu surface/shadow, footer border, vitals text, boot-log text, and the **focus ring** are all legible. The focus ring is `--accent` on `--bg` (`style.css:685`) and must clear **3:1** per WCAG 1.4.11 — add a focus-ring row to `generate_themes.py`'s contrast audit (A1 cross-request, §7.4) |
| **Lunarcore + Solarcore** at minimum | The two flagship themes get the full pass every time; the other 21 get a spot check |
| **JS disabled** | Theme control absent (not present-and-dead); OS light preference → Solarcore, dark → Lunarcore; all nav works; 404 works |
| **`localStorage` blocked** (Firefox `dom.storage.enabled=false`) | Same as above — this is the case current `theme-init.js:7-14` mishandles (F-05) |
| **`prefers-reduced-motion: reduce`** | No cursor blink, no menu pop, no underline sweep, no CRT scanlines, error page fully visible at t=0 |
| **Text zoom 200%** | Header wraps; nav does not clip; theme menu still on screen |
| **Viewport 320px @ 400% zoom** | No horizontal page scroll (WCAG 1.4.10) |
| **Viewport 360 / 768 / 1440 / 2560px** | Header and footer wrap sensibly; `main` stays centred at 900px |
| **Non-Linux host** (`rss_mib() == None`, `state.rs:262-272`) | `MEM` item **and** its separator both absent; no double `·` |
| **Fresh process** (`UP 00:00:00`, `REQ 1`) vs **long-running** (`UP 07:13:42`, `REQ 40381`) | Strip does not reflow the footer or wrap awkwardly at either extreme |
| **Screen reader** (Orca on Linux, VoiceOver on macOS) | Landmark list reads banner / navigation "Primary" / main / contentinfo / navigation "Footer"; active nav announces "current page"; vitals read as expanded words |
| **Social preview** | Paste a URL into Slack and Discord after `og:image` lands; card shows the mark, not a bare link |

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
| Font stacks (`ui-monospace`, `SFMono-Regular`, `Menlo`, `Consolas`, `Charter`, `Georgia`, `ui-rounded`, `Segoe UI`, `system-ui`) | OS-provided; `generate_themes.py:11-15` | Referenced by name only — **no font file is bundled or served** | ✅ Clear; also why `font-src 'self'` is safe |
| Menu and vitals glyphs (`◐ ⏾ ✸ ✦ ☀ ▦ ◈ ¶ ◒ ☁ ▣ ▩ ⌗ ✜ ⌁ ◉ ❄ ◆ ▹ ▧ ⌖ ⊞ ⊟ ⚙ ▍ ✓ ·`) | Unicode code points, rendered by the system font | Not copyrightable as characters | ✅ Clear |
| Theme *names* — Dracula, Solarized, Nord, Gruvbox, Game Boy, Commodore, Tron | Third-party colour schemes and trademarks referenced descriptively | Colour values are not copyrightable; names are nominative reference in a personal, non-commercial UI. Note the menu already renames some (Game Boy → label "Game Boy", `nes` → "8-Bit", `sepia` → "Steampunk") | ⚠️ **A1's call, not A2's** — flagged, not resolved here |
| `og:card.png` (planned) | To be generated from the owner's own `mark.svg` | Owner's own work | ✅ Clear on arrival |

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
| **3B — contrast and colour independence** | Token contrast is A1's, audited by `generate_themes.py`. A2 adds: (a) the **focus-ring** 3:1 requirement (WCAG 1.4.11) as a new audit row — currently unaudited; (b) `aria-current="page"` so the active nav state is not colour-plus-underline only; (c) preservation of the theme `✓` mark as the non-colour checked cue. |
| **3C — keyboard and focus** | Full key table in §3.4. Fixes: `<main tabindex="-1">`, skip-link legibility, Tab-close focus restore (F-06), typeahead. The theme menu already implements the APG roving-focus model (`main.js:43-72`) and remains the in-repo reference. Visible focus indicator is global and never removed (`style.css:685`). |
| **3D — semantics and AT** | Landmark table in §3.7. Fixes: `<nav>` narrowed to wrap only the links (the theme control is not navigation); `aria-label="Primary"`/`"Footer"` de-duplicated against the role announcement; `.vitals-strip` given `role="group"` so its `aria-label` is actually honoured (an `aria-label` on a bare `<div>` is ignored — `vitals_strip.html:7`); decorative SVG and glyphs stay `aria-hidden` (already correct); one `<h1>` per page pinned by **U-4**. |
| **3E — motion and sensory safety** | Every animation already sits inside `@media (prefers-reduced-motion: no-preference)` (`style.css:710-723`, `1279-1289`), and the CRT texture is removed under `reduce` (`style.css:461-470`). No autoplay, no body-content animation. Boot stagger totals 1.8s (< 5s, WCAG 2.2.2); cursor blink is ~0.83 Hz (< 3 Hz, WCAG 2.3.1). The blink's residual 2.2.2 exposure is stated openly in §3.7 with Q3 as the alternative. |
| **3F — responsive and resilient** | Breakpoint table in §3.4; `100svh` fix; 320px@400% reflow check in §5.4. Resilience: the shell renders on 500 and with no global state (**U-11**), and the `MEM` empty state omits the item *and its separator* rather than printing a gap (`vitals_strip.html:11-16`). |

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
- Global focus ring, never removed (`style.css:685`).
- All motion behind `prefers-reduced-motion` (`style.css:710-723`, `1279-1289`).
- Vitals strip: server-rendered, zero JS, zero polling, null-safe, with a
  designed empty state for `MEM` (`vitals_strip.html`, `state.rs:239-254`).
- Themed 404 and 500 with path escaping and no internal disclosure, both
  test-covered (`errors.rs:160-193`).
- Footer restored at commit `f8553d5` — the markup had been commented out with a
  malformed `--!>` terminator (`git show f8553d5`), leaving `.footer-inner` CSS
  (`style.css:797-810`) orphaned. It now carries the name and source link above
  the vitals strip (`base.html:89-97`). The hardcoded `updated on: 2026-08-02`
  span was correctly dropped in favour of the real build stamp.
- `generate_themes.py` is now group-aware and has its own internal drift guard
  (`generate_themes.py:255-282`), fixing the `criteria.md` 5B reference case.

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
| **F-02** | **High (WCAG 2.4.7 / 1.4.3)** | The skip link on focus sets only `left: 1rem`. It has **no background, no padding, no z-index** — it paints as an underlined accent-coloured link directly over the header's brand text, because it is the only positioned element in that stacking context. Frequently illegible. | `style.css:534-539`; header is `position: static` (`:545-548`) |
| **F-03** | **High (no-JS)** | **`prefers-color-scheme` is honoured only in JavaScript.** There is no `@media (prefers-color-scheme: …)` rule anywhere in the 1,471-line stylesheet; theme resolution lives entirely in `theme-init.js:10-12` / `main.js:22`. With JS off, every visitor gets bare `:root` = Lunarcore dark, including someone whose OS is set to light. | `grep prefers-color-scheme static/css/style.css` → zero matches; `style.css:16-33` |
| **F-04** | Medium (WCAG 1.3.1 / 4.1.2) | The active nav item is conveyed by `--accent` colour plus an underline — and the **hover** state uses the *same* underline (`.nav-link:hover::after, .nav-link.is-active::after { right: 0 }`), so hover and active are visually identical. There is **no `aria-current`** anywhere, so assistive tech gets nothing at all. | `style.css:597`, `:699`; `base.html:24-27` |
| **F-05** | Medium | `theme-init.js` wraps the **`setAttribute` call inside the same `try` block** as the `localStorage.getItem`. If storage access throws (Safari private mode, Firefox with `dom.storage.enabled=false`, lockdown profiles), the catch swallows everything and `data-theme` is never set — so a light-preference user is forced into dark even though JS *is* running and `matchMedia` would have worked fine. | `theme-init.js:7-14` |
| **F-06** | Medium (WCAG 2.4.3) | Pressing **Tab** with the menu open calls `close()`, which sets `menu.hidden = true` **while focus is still on a menu item**. Focus is dropped to `<body>`, so the browser's default Tab restarts from the top of the document instead of continuing from the theme button. | `main.js:61`, `:56` |
| **F-07** | Medium (5B drift) | The asset cache-buster `?v=20260719-spectrum` is a **hand-typed literal repeated four times** in `base.html:11-13, 99`. `style.css` was last modified 2026-08-07; the query string still says `20260719`. It is **already stale** — returning visitors can be served a stale stylesheet against a changed template. Nothing fails when it drifts. | `base.html:11-13`, `:99`; `stat static/css/style.css` |
| **F-08** | Medium (5B drift) | `/learn` compares `self.section() == "wiki"` — a leftover from the `/wiki` → `/learn` rename (`router.rs:41-46` shows both routes). It works today, but nothing enforces the mapping: any new page returning `"learn"` would silently fail to highlight, and there is no test that would catch it. The comparison is a **stringly-typed match with no compiler help**. | `base.html:27`, `wiki.rs:114-116` |
| **F-09** | Medium (3D) | `.vitals-strip` is a bare `<div>` carrying `aria-label="Server vitals"`. ARIA prohibits naming `role="generic"`; most screen readers **ignore this label entirely**. The readout is then announced as the raw token stream `"UP 00:14:32 REQ 1204 MEM 14 MiB v0.1.0 built …"` with no expansion. | `vitals_strip.html:7-18` |
| **F-10** | Medium (3D) | The primary `<nav>` wraps the brand **and the theme selector**, so the navigation landmark contains a colour-scheme control that is not navigation. Its name `"Primary navigation"` also double-announces ("Primary navigation, navigation"). Same duplication in the footer's `"Footer navigation"`. | `base.html:20-82`, `:92` |
| **F-11** | Medium (2D consistency) | `<title>` policy is inconsistent: `"About — machinageist"`, `"Portfolio — machinageist"`, `"Writing — machinageist"`, `"Releases — machinageist"`, `"Status — machinageist"` — but blog posts and learn pages return the **bare** title with no site name, so a browser tab or a bookmark reads "Moving My Homelab" with no attribution. | `pages.rs:77`, `blog.rs:64`; vs `blog.rs:134-136`, `wiki.rs:106-108` |
| **F-12** | Medium (4A/4E) | **No `og:image`, `og:url`, `og:site_name`, `twitter:card`, or canonical link.** A link to this site pasted into Slack, LinkedIn, or Discord — the exact channel a hiring manager receives it through — renders as a bare text row. `og:type` is also hardcoded `website` even for blog posts and learn pages. | `base.html:4-14` |
| **F-13** | Low | `/static/<missing>` returns `ServeDir`'s bare 404 with an empty body, not the themed page. | `router.rs:60` |
| **F-14** | Low (3C) | The skip link's target `<main id="content">` is not focusable, so activating the link moves scroll but not keyboard focus in several browsers — the next Tab restarts from the top. | `base.html:85` |
| **F-15** | Low | `focusedIndex()` returns `-1` when focus has left the menu while it is open; `ArrowUp` then computes `(-2 + 24) % 24 = 22`, landing on the second-to-last item instead of the last. | `main.js:45-48`, `:63` |
| **F-16** | Low (5B) | `generate_themes.py` emits the menu markup and the `MODES`/`ICON` arrays, and has an internal guard against `MENU_GROUPS`/`THEMES` drift (`:255-262`) — but **nothing compares its output to the shipped `base.html`, `main.js`, or `theme-init.js`**. The generated artifacts are copy-pasted by hand. | `generate_themes.py:255-282`; no consumer in `src/` or `tests/` |
| **F-17** | Low (5C) | **The hidden-coupling case.** `pages.rs:158` asserts `html.contains("CompTIA")` inside a test named `home_page_shows_concrete_work_without_strategy_narration`. `index.html` contains no such string — the assertion passes **only** because `base.html:6` and `:8` render `IndexTemplate::description()` (`pages.rs:44`) into `<meta name="description">` and `og:description`. A test that appears to be about the home page **body** is in fact pinned to a metadata string, routed through the shell. Editing `description()` for entirely unrelated SEO reasons breaks a test whose name mentions neither meta tags nor descriptions. The same shell coupling silently affects `html.contains("Proxmox")` (`:157`) and `html.contains("homelab")` (`:156`, `:214`), each satisfied by *both* body and meta, and it makes every negative assertion (`!html.contains("in training")`, `:161`) an implicit guard over all shell copy including 24 theme names. | `pages.rs:146-167`, `:204-223`; `base.html:6`, `:8`; `index.html` (no "CompTIA") |
| **F-18** | Low | `body { min-height: 100vh }` uses the legacy viewport unit; on mobile Safari the footer sits under the collapsing toolbar. | `style.css:510` |
| **F-19** | Informational | HTML documents carry **no** `Cache-Control`. `/status` and `/status.json` correctly carry `no-store` and are test-pinned (`status.rs:125-135`), but every ordinary page embeds a live counter with no cache policy. Currently harmless (Cloudflare does not cache `text/html` by default), but nothing enforces it. | `grep -rn cache-control src/` → one hit, in `status.rs` |

### 7.2 Delta to spec

**New files (1)**

- `src/shell.rs` — `Section` enum, `NavItem`, `NAV`, `asset_version()`, and the
  `#[cfg(test)]` module carrying U-1…U-10.

**Modified files (9)**

| File | Change | Fixes |
|---|---|---|
| `src/shell.rs` (new) | as above | F-08, F-07 |
| `src/main.rs` | `mod shell;` | — |
| `templates/base.html` | `<nav>` narrowed to the links only; nav rendered from `crate::shell::NAV`; `aria-current="page"`; `aria-label` `"Primary"`/`"Footer"`; `<main tabindex="-1">`; `?v={{ crate::shell::asset_version() }}` ×4; `og:url`, `og:site_name`, `og:image`, `twitter:card`; `aria-haspopup="menu"` | F-04, F-07, F-10, F-12, F-14 |
| `templates/vitals_strip.html` | `role="group"`; `<span class="vh">` expansions per item; descriptive link label | F-09 |
| `templates/blog_post.html`, `templates/wiki_page.html` | `{% block head_extra %}` → `og:type = article` | F-12 |
| `src/handlers/{pages,blog,wiki,releases,status}.rs`, `src/errors.rs` | `section()` return type `&str` → `Section`; blog and wiki `title()` gain the `" — machinageist"` suffix (needs an owned `String` field or a `format!` helper) | F-08, F-11 |
| `src/router.rs` | `ServeDir::new("static").not_found_service(…)` | F-13 |
| `static/css/style.css` | skip-link background/padding/border/z-index; `@media (prefers-color-scheme: light) { :root:not([data-theme]) { … } }`; `:root:not([data-js]) .theme-select { display: none }`; `.vh` utility; `min-height: 100svh` | F-01, F-02, F-03, F-18 |
| `static/js/theme-init.js` | narrow the `try` to the storage read only; set `data-js` on `<html>` | F-01, F-05 |
| `static/js/main.js` | Tab → focus button **then** close; typeahead; keep `aria-label` in sync with the current theme; clamp `focusedIndex() == -1` | F-06, F-15, §3.7 |
| `tests/shell.rs` (new integration file) | I-1…I-8 | all |
| `docs/themes/generate_themes.py` | add a focus-ring (`--accent` on `--bg`, 3:1) row to the contrast audit | 3B / A1 request |

**Migrations / schema changes:** none — no database.

**New dependencies:** none.

**Suggested commit sequence** (each independently shippable and verifiable, per
the AI-engineering workflow's atomic-task rule):

1. `fix: make the shell honest with JavaScript disabled` — F-01, F-03, F-05
   (CSS fallback + `data-js` flag + narrowed try). *The auto-fail gate; ship first.*
2. `fix: skip link, main focus target, and menu Tab order` — F-02, F-06, F-14, F-15.
3. `refactor: one definition for sections and the primary nav` — `src/shell.rs`,
   F-08, F-04 (`aria-current`), with U-1…U-3.
4. `fix: derive the asset cache-buster from the build stamp` — F-07, U-8.
5. `fix: landmarks and labels in the header and footer` — F-09, F-10.
6. `feat: complete the head metadata contract` — F-12, I-7, I-8, `og-card.png`.
7. `fix: serve the themed 404 for missing static assets` — F-13, I-4.
8. `test: pin the shell contract` — remaining tests, including U-7 and U-9.
9. `fix: title suffix on blog and learn pages` — F-11, U-5.

### 7.3 Estimated scope

**M**, trending to the top of M.

Justification: no new dependency, no new route, no data model, no migration, and
one new source file. The work is nine surgical commits across nine existing files
plus a test module. The volume is in the **tests** (~18 new cases) rather than the
features, which is the correct ratio for a foundation component that thirteen
other features inherit — a shell regression breaks every page at once.

Two items carry above-average risk and account for the "top of M": the
`section() -> Section` type change touches seven handler files and both error
templates in a single commit (compiler-guided, so mechanical but wide), and the
blog/learn `title()` suffix requires those structs to own a `String` rather than
return a borrow of `post.title` — a small ownership change with a clippy
implication.

Not S: it edits every page's rendered output. Not L: nothing is redesigned, no
new surface is introduced, and the existing behaviour is largely correct.

### 7.4 Blocking dependencies

**Blocking A2:**

| Dependency | Feature | What A2 needs |
|---|---|---|
| Token values for the `prefers-color-scheme: light` no-JS fallback block | `A1` | The Solarcore token set as a reusable declaration (a `@media`-scoped `:root:not([data-theme])` copy, or A1 restructures so a theme's tokens can be applied under two selectors without duplication). **Commit 1 is blocked on this.** |
| Focus-ring contrast audit across all 23 themes | `A1` | `--accent` vs `--bg` at ≥ 3:1 (WCAG 1.4.11). If any theme fails, the ring needs a theme-owned override token. A2 cannot verify 3B without it. |
| Theme-name trademark posture | `A1` | §6.2 flags Dracula / Nord / Gruvbox / Game Boy / Tron. A1's call. |
| Explicit `Cache-Control` policy for HTML documents | `A3` | §4.7 — the vitals strip is only truthful if HTML is not edge-cached. |

**Blocked by A2 (A2 must land first):** every page feature `B1`–`B6` and `C1`–`C4`
inherits `Section`, the nav table, the title/description contract, and the
`head_extra` override point. `C3` study-tools additionally inherits the
`{% block scripts %}` extension point and the 150-line JS ceiling.

**Cross-feature requests A2 files (not implemented here):**

1. → `B1` **home** and `B2` **about**: rewrite `description()` and the about bio
   so `"CompTIA study"` / `"the CompTIA stack"` match the 2026-08-02 spine
   (RHCSA → CCNA → Security+, Network+ dropped). Criterion 1D. **A2 supplies the
   guard (U-7); B1/B2 supply the words.** Note the ordering trap: U-7 will fail
   CI the moment it lands, so it must be committed *with* or *after* the copy fix,
   not before.
2. → `B1`: rewrite the `home_page_shows_concrete_work_without_strategy_narration`
   assertions so each one names what it is actually testing (F-17). The
   `"CompTIA"` assertion belongs in a metadata test, not a page-body test.
3. → `B6` **releases**: `ReleasesTemplate::description()` publishes GeistScope
   naming through `<meta>` and `og:description`. Confirm it clears the §1C gate.
4. → `A1`: the four items in the blocking table above.

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
