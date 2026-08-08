# Spec: Design System

**Feature ID:** `design-system` (A1)
**Parent feature:** root
**Spec author agent:** Spec Gauntlet agent 1 (Claude Opus 5)
**Date:** 2026-08-07
**Iteration:** 1

---

## 1. Purpose

### 1.1 One-sentence job

Give every surface on machinageist.dev one set of colour, type, spacing, and
motion decisions — expressible as CSS custom properties, generated from a single
audited source, and correct across all 23 themes with JavaScript disabled — so
that adding a page never means inventing design.

### 1.2 Why it matters

Three pressures collide on this site and the design system is where they resolve.

1. **The site is mostly long-form reading.** Four blog posts and twelve `/learn`
   pages today, growing. The reader's job is to read for twenty minutes without
   fatigue. That makes measure, line-height, contrast, and quiet body copy
   load-bearing, not decorative.
2. **The site is itself the portfolio entry.** `src/models/project.rs:89-116`
   pins the portfolio at exactly one entry — `mg-server`, this codebase. An
   engineer who opens devtools *is* reviewing the work. The stylesheet, the
   generator, and the no-JS fallback are the artifact.
3. **The design doc and the code have already drifted.** `docs/solarcore/SOLARCORE_SPEC.md`
   describes a site that does not exist: a single dark theme with no toggle, a
   `--sc-*` token namespace, and a split-colour `MACHINA`/`GEIST` wordmark. The
   shipped site has 23 themes, `--bg`/`--surface` tokens, and a theme-driven
   two-tone mark. Every downstream feature spec in this gauntlet cites the design
   system; if it cites a fiction, thirteen specs inherit the fiction.

The pain this feature addresses is therefore not "the site looks bad." It is:
*a new page cannot be built without either guessing or reading a document that is
wrong*, and *a visual change cannot be verified because "works in all 23 themes"
has no definition and no test*.

### 1.3 Success signal

**Measurable:** `cargo test --all-targets` and
`python3 docs/themes/generate_themes.py --check` both pass on a tree where the
committed CSS/JS/HTML theme registries were regenerated from
`docs/themes/generate_themes.py`, with zero contrast failures across the full
matrix of **(token × background × usage size)** for all 23 themes — a matrix that
today is not audited and, when computed, produces **19 failures** (§7.1.4).

**Observable:** a reader with JavaScript disabled and
`prefers-color-scheme: light` set gets the Solarcore light palette rather than the
dark default, and sees no theme control at all instead of a button that does
nothing.

### 1.4 Reconciliation ledger — criterion 2A (blocking)

**Resolved direction (Jeff, 2026-08-07): the shipped site wins.** The job of this
spec is to define the rewrite of `docs/solarcore/SOLARCORE_SPEC.md` so it
describes what shipped, then to layer best practice on top. Every divergence found
is enumerated below with its reconciliation and the *design reason* the shipped
version is the better one. Nothing in this spec re-asserts the stale document.

| # | Stale spec says | Shipped reality | Reconciliation | Why shipped wins |
|---|---|---|---|---|
| **D1** | Solarcore is a **night** theme; `--sc-void: #010915` (§3) | `:root[data-theme="solarcore"]` is **light**: `--bg: #f4f7ec`, `color-scheme: light` (`style.css:36-52`). The dark flagship shipped as **Lunarcore** (`style.css:17-33`) | Shipped wins. Solarcore = light/day, Lunarcore = dark/night. The pair is the `system` resolution (`theme-init.js:10-12`) | A `system` mode needs one light and one dark flagship or it cannot honour `prefers-color-scheme`. Naming the light one "Solarcore" (sun) and the dark one "Lunarcore" (moon) is the only assignment that reads correctly. The stale spec had no light theme at all, so `system` was unimplementable. |
| **D2** | §10 anti-goals forbid "scanlines, CRT filters" and "**No lightening/theming toggle** for now" | CRT theme ships **with** a scanline overlay (`style.css:444-458`), alongside cyberpunk/synthwave/vaporwave/matrix, behind a **24-entry** grouped toggle (`base.html:36-79`) | Shipped wins. The anti-goal is **rewritten**, not deleted: spectacle is opt-in per theme, never the default, and never the `system` resolution | The anti-goal was aimed at the right target (don't make the reader endure spectacle) but chose the wrong instrument (ban it globally). Opt-in is strictly better: Lunarcore stays crisp for reading (`style.css:432-442` excludes it from glow), and the reader who wants a CRT gets one. The ban would also have forbidden Paper and Dawn — the two serif themes that are the *best* long-reading surfaces on the site. |
| **D3** | §7 wordmark: uppercase split-colour `MACHINA` (magenta) / `GEIST` (cyan), with vines and a 12-tooth gear | Lowercase `machinageist` in `--accent` plus a gear/brain inline SVG whose halves stroke `--accent` and `--code` (`base.html:21`, `style.css:571-577`), and a blinking `▍` cursor (`style.css:702`) | Shipped wins. Brand is **theme-parametric**: two tones drawn from the active palette | A hardcoded magenta/cyan wordmark is legible in exactly one palette. Across 23 it would clash in 21 of them (magenta on Gruvbox, cyan on Paper). Binding the mark to `--accent`/`--code` means the emblem is coherent in every theme with zero per-theme art. This is the single strongest design decision on the site and the stale spec would have destroyed it. |
| **D4** | §3 token namespace `--sc-void`, `--sc-cyan`, `--sc-magenta`, `--sc-text-body` … | `--bg`, `--surface`, `--text`, `--text-muted`, `--text-faint`, `--border`, `--border-subtle`, `--accent`, `--accent-hover`, `--accent-border`, `--code`, `--shadow`, `--font-body`, `--font-mono` (`style.css:17-33`) | Shipped wins. Token names are **semantic roles**, not colour names | `--sc-cyan` is a lie the moment a second theme exists — in Gruvbox it is yellow. Role names (`--accent`) survive re-palettes; colour names do not. The `sc-` prefix also pinned the whole system to one theme's name, which is exactly the coupling that caused D1. |
| **D5** | §2 principle 3 "three neons, three jobs": cyan = interactive, magenta = identity/structure, green = organic/growth | Two signal colours per theme: `--accent` (interactive) and `--code` (code/second tone). Lunarcore is cyan + green; **no magenta structural role exists** | Shipped wins. The contract is **two signal roles**, not three | Three mandated hues cannot be satisfied by 23 palettes — Teletext is white/yellow/green, Amber is a single hue by definition, Gruvbox has no magenta. Two roles is the largest number every palette can carry. The "structure" job that magenta held is re-assigned to *typography* (case, weight, letter-spacing, size), which works in every theme including monochrome ones. |
| **D6** | §7 artifacts: `mark.svg`, `mark-sm.svg`, `favicon.svg`, `vine-trace.svg`, `og-card.png` | `favicon.svg` **ships but is stale** — it hardcodes `#010915` / `#46c8f0` / `#e23a9a`, the abandoned night palette, matching **no shipped theme**. `mark.svg`, `mark-sm.svg`, `vine-trace.svg` (14,588 B) are referenced by **no** template or stylesheet. `og-card.png` does not exist and `base.html:7-9` has no `og:image` | Shipped mark wins; the **assets are wrong and must be fixed**. Regenerate `favicon.svg` from the flagship pair; delete the three orphans; `og-card` stays **planned** | This divergence is not in the brief's list of five and is the one with a live user-visible defect: every visitor's browser tab currently shows an emblem in colours the site abandoned. The orphans are worse than dead code — they are a *second, contradicting* brand definition sitting in `static/`. |
| **D7** | §1: "machinageist.dev is a **systems programmer's** portfolio, blog, and tool wiki" | The site's own claim discipline permits systems/NOC-in-training, homelab operations, owned scope — and `src/handlers/pages.rs:163-166`, `src/models/project.rs:109-115`, `src/models/lab.rs:258-269` encode that as tests | Stale spec's **role assertion is dropped**. The rewrite describes the artifact ("a reading-first personal site"), never the author's title | A design document has no business asserting a job title. This is the one divergence where the *stale spec*, not the code, violates policy (criteria 1E). Removing it costs nothing and closes a claim-integrity hole. |
| **D8** | §6.4 footer: uppercase readout `SYS: … │ OPERATOR: … │ STATUS: ONLINE │ UPDATED: 2026-05-22` — a hand-written status line | Vitals strip renders **real process data** at request time: `UP`/`REQ`/`MEM`/version/build from `crate::state::Status::current()` (`templates/vitals_strip.html`), no JS, no polling | Shipped wins, emphatically | `STATUS: ONLINE` hardcoded into a template is dashboard cosplay — it says "online" on a page that by definition rendered. The shipped strip reads `/proc/self/status` and the `build.rs` compile stamp. It is the difference between a fake metric and a real one, which is the exact line criteria 2E and 4B draw. |

**Disposition of the stale document.** `docs/solarcore/SOLARCORE_SPEC.md` is
retired to a five-line "superseded" stub pointing at a new
`docs/design/DESIGN_SYSTEM.md`. Its §1 philosophy prose and the reference art
under `docs/solarcore/reference/` survive as a non-normative **Origins** section
inside the new document, clearly labelled as mood that seeded the Lunarcore /
Solarcore pair rather than as rules. The rename is not cosmetic: a document named
for one theme slug governing 23 themes is the structural cause of D1 and D4.

---

## 2. User Stories

> **Happy path — reader.** As someone who found a `/learn` page from a search
> result, I want the page to be comfortable to read for twenty minutes at my own
> browser font size, so that I finish the page instead of skimming it.

> **Happy path — theme chooser.** As a returning reader, I want the theme I picked
> last week to be the one that paints on first load, with no flash of the wrong
> palette, so that the site feels like mine.

> **Edge case — JavaScript disabled.** As someone browsing with JS off (uMatrix,
> Tor Browser, a corporate proxy that strips scripts), I want the site to honour my
> operating system's light/dark preference and to *not* show me a theme button that
> does nothing, so that I get a usable page rather than a broken control.

> **Accessibility — low vision.** As a reader who runs their browser at a 24px
> default font and 200% zoom, I want every size on the page to scale with that
> setting and every text/background pair to clear WCAG AA at the size it is
> actually rendered, in whichever theme I chose, so that I never have to
> screenshot-and-invert to read metadata.

> **Accessibility — keyboard and screen reader.** As a keyboard-only reader, I want
> to reach the theme menu with Tab, drive it with arrows, close it with Escape, and
> see a visible focus ring at every stop, so that the one interactive widget on the
> site is not the one thing I cannot use.

> **Accessibility — vestibular / photosensitive.** As a reader with
> `prefers-reduced-motion: reduce` set, I want no transition, no blinking cursor,
> no staged boot-log reveal, and no scanline texture, so that the site does not
> make me ill.

> **Maintainer.** As the person adding the 24th theme or the next `/learn` page, I
> want one file to edit and one command that fails loudly if I got it wrong, so
> that I cannot ship a half-themed page or an unreadable colour pair by accident.

---

## 3. UX Specification

### 3.1 Screen / view inventory

The design system introduces **no new screens**. It governs every existing one.
Each row below is a *modification of existing*; the "Layout pattern" column is the
structure the system must keep working across 23 themes.

| Surface | Path to reach | Layout pattern | Design-system surface area |
|---|---|---|---|
| Site chrome (header, nav, footer, vitals strip) | every page, `templates/base.html` | Fixed-width 900px centred bar, flex nav | Brand mark, nav states, theme control, footer readout |
| Theme menu | header → theme button | Absolutely positioned popover, `role="menu"`, scrolls at `max-height: min(72vh, 30rem)` (`style.css:619-637`) | The only interactive widget on the site |
| Home | `/` | Hero + `<hr>`-separated sections | Hero type scale, list rows |
| About | `/about` | Prose + definition-style list | Measure, `.about-list` dividers |
| Portfolio | `/portfolio` | Divider list, status pill per row | Status vocabulary, pills |
| Writing index | `/blog` | Grouped divider lists (`post-group`) | Group headings (currently unstyled — §7.1.6) |
| Article | `/blog/:slug`, `/learn/:slug` | Prose column capped at `--measure`; `pre` uncapped and scrolling | The core reading surface |
| Learn shell | `/learn`, `/learn/:slug` | 13rem sidebar + article grid, collapses to a `<details>` popover below 800px (`style.css:1300-1438`) | Sidebar hierarchy, active-page marker |
| Releases | `/releases` | Divider list with download affordance | Monospace SHA wrapping |
| Status | `/status` | 700px `<dl>` readout on `--surface` | The one panel surface; `--accent` on `--surface` contrast |
| Errors | any 404/500 | Boot-log column, staged reveal | The one place spectacle is free |

### 3.2 Interaction flows

**Primary flow — page load with a stored theme.**

1. Browser requests the document. Axum serves server-rendered HTML with security
   headers including `script-src 'self'` (`security_headers.rs:38-50`).
2. `<head>` loads `/static/js/theme-init.js` **render-blocking** (`base.html:12`).
   It is an external file precisely because the CSP forbids inline script.
3. `theme-init.js` reads `localStorage["theme"]`, validates it against `MODES`,
   resolves `system` → `solarcore | lunarcore` by `prefers-color-scheme`, and sets
   `data-theme` on `<html>` (`theme-init.js:5-15`). This happens before first
   paint, so there is no flash.
4. CSS applies the matching `:root[data-theme="…"]` block. **Target change:** the
   same step also reveals the theme control, which is `display: none` until
   `data-theme` exists (§3.7, no-JS gate).
5. `main.js` loads at end of body, wires the menu, and calls `render(getMode())`
   to stamp `aria-checked` and the `is-current` class (`main.js:79`).

**Branch — no JavaScript.** Steps 2–3 and 5 do not happen. No `data-theme`
attribute exists.
- *Current behaviour:* the bare `:root` block applies → Lunarcore dark, regardless
  of the reader's OS preference; the theme button renders, takes focus, and does
  nothing.
- *Target behaviour:* the bare `:root` still carries Lunarcore, and a new
  `@media (prefers-color-scheme: light) { :root { …Solarcore tokens… } }` block
  gives light-preference readers the light flagship. The theme control is hidden.
  Specificity holds: `:root[data-theme="lunarcore"]` is (0,2,0) and beats the
  media-query `:root` at (0,1,0), so the JS path is unaffected.

**Branch — JavaScript enabled but `localStorage` throws** (Safari private mode,
storage disabled by policy). `theme-init.js` wraps the read *and* the
`setAttribute` in one `try` (`theme-init.js:7-14`), so a throwing `getItem`
skips the attribute entirely. Harmless today; under the `[data-theme]` gate it
would hide the menu from users whose JS works fine. **Target:** narrow the `try`
to the storage read only, so `setAttribute` always runs.

**Flow — choosing a theme.**

1. Click, `Enter`/`Space`, or `ArrowDown`/`ArrowUp` on the closed button opens the
   menu (`main.js:68-72`) and moves focus to the currently checked item.
2. Arrows cycle with wraparound; `Home`/`End` jump; `Escape` closes and returns
   focus to the button; `Tab` closes (`main.js:59-66`).
3. Selecting applies `data-theme`, writes `localStorage["theme"]`, re-renders
   `aria-checked` + `is-current`, closes, and returns focus to the button
   (`main.js:73-75`).
4. Under `prefers-reduced-motion: no-preference` the menu plays a 0.16s
   `theme-pop` and the page cross-fades background/colour over 0.3s
   (`style.css:710-726`). Under `reduce`, both are absent — the swap is instant.
5. `system` keeps following the OS live via an `mql` change listener
   (`main.js:77`).

No haptics, no sound. Animation cues are listed in §3.5.

### 3.3 Layout descriptions

The system defines four layout constants, all in the measurement layer (§4.2):

| Constant | Value | Applies to |
|---|---|---|
| `--layout-column` | `900px` | `main`, `.site-nav`, `.footer-inner`, `.vitals-strip` |
| `--layout-wide` | `1200px` | `main:has(.wiki-layout)` — the sidebar layout needs more room |
| `--layout-narrow` | `700px` | `.status-page`, `.error-page` — single-purpose readouts |
| `--measure` / `--measure-narrow` | `72ch` / `55ch` | Prose blocks / lede and intro copy |

`ch` is deliberate rather than `rem`: `ch` is the advance width of `0` **in the
current font**, so a 72ch measure stays 72 characters in Paper's serif, Cloud's
sans, and the mono default. A `rem` measure would be too wide in serif and too
narrow in mono. This is why measure is theme-invariant *and* still visually
correct per theme.

**Component hierarchy, top → bottom, for every page:**

1. `.skip-link` — first focusable element, off-canvas until focused
   (`style.css:534-539`).
2. `header.site-header` — 1px `--border` bottom rule. Leading: brand (26px inline
   SVG + lowercase wordmark + blinking cursor). Trailing: `.nav-links` (About /
   Portfolio / Writing / Learn), then the theme control.
3. `main#content` — `--layout-column`, `3.5rem 2rem 5rem` padding.
4. `footer.site-footer` — 1px `--border` top rule; name + source link on one row,
   then `.vitals-strip` separated by a `--border-subtle` rule.

**Data sources.** Colour and font role come from the active `[data-theme]` block,
generated from `THEMES` in `docs/themes/generate_themes.py:21-122`. Size, spacing,
measure, and layout come from the bare `:root` measurement block
(`style.css:472-500`, to be extended). Vitals values come from
`crate::state::Status::current()` at request time. No component reads a literal.

**Empty states.** The design system supplies the *shape*; each feature supplies
the copy. The invariants are: an empty section is omitted entirely rather than
rendered as a heading over nothing (the home page already does this —
`templates/index.html:30`, asserted at `pages.rs:186-202`); an empty list that
*must* stay renders one `--text-muted` paragraph at body size inside
`--measure-narrow`, never a placeholder card, never a spinner. `.releases-empty`
(`style.css:1193-1196`) is the reference implementation.

### 3.4 Input & gestures

- **Pointer.** Hover states exist on links, nav links, list rows (`.post-item`,
  `.project-card` gain `--surface` fill plus a 2px inset `--accent` left edge —
  `style.css:705-708`), theme button, and menu items. All are additive; none
  changes layout, so no reflow on hover.
- **Touch.** No hover-only affordance carries information. The 640px breakpoint
  (`style.css:1444-1471`) wraps the nav and stacks `.project-header`. **Target
  addition:** the theme button is `2rem × 2rem` (32px) — below the 44×44 CSS-pixel
  target the WCAG 2.2 AAA / mobile-usability guidance recommends. Spec raises the
  *hit area* to 44px via padding while keeping the 32px visual box, which needs no
  layout change.
- **Keyboard.** Full model in §3.2 and §3.7. There are **no site-wide keyboard
  shortcuts** and the spec adds none — single-letter accelerators collide with
  screen-reader browse modes and with the browser's own find-as-you-type.
- **Stylus / controller / voice / camera.** N/A — a text site with one popover
  menu; nothing here has a specialised input mode.
- **Responsive.** Two breakpoints total: 800px (wiki grid → stacked with a
  `<details>` popover) and 640px (chrome padding, nav wrap, header stacking).
  **Target:** both become named tokens (`--bp-wide: 800px`, `--bp-narrow: 640px`)
  documented in the system doc; CSS cannot use custom properties inside media
  queries, so the tokens are documentation plus a comment at each `@media`, not a
  `var()`. State the limitation rather than pretending otherwise.

### 3.5 Transitions & animation

Complete inventory of motion on the site, all of it currently in `style.css`:

| Motion | Where | Duration | Guarded? |
|---|---|---|---|
| Background/colour cross-fade on theme swap | `body` | 0.3s | ✅ `no-preference` (`:710-712`) |
| Chrome colour/border transitions | header, footer, buttons, rows, tags | 0.18–0.3s | ✅ (`:713-720`) |
| Nav underline sweep (`right: 100% → 0`) | `.nav-link::after` | 0.22s | ✅ (`:719`) |
| Menu pop-in (`theme-pop`) | `.theme-menu` | 0.16s | ✅ (`:721`) |
| Blinking cursor (`cursor-blink`, infinite) | `.brand::after` | 1.2s | ✅ (`:722`) |
| Staged boot-log reveal (`boot-line-in`) | error pages | 0.2s × 5 stagger | ✅ (`:1279-1289`) |
| Wiki `<details>` marker rotation | `.wiki-nav > summary::before` | 0.15s | ❌ **unguarded** (`:1407-1411`) |
| CRT scanline overlay (static, not animated) | `crt` body::after | — | ✅ removed under `reduce` (`:461-462`) |
| Neon text-shadow on 7 opt-in themes | `body` | — | ✅ removed under `reduce` (`:463-469`) |

**Rules the system enforces.**
1. Every declaration of `transition` or `animation` lives inside
   `@media (prefers-reduced-motion: no-preference)`. The one violation above is
   fixed by moving it.
2. Animations use `fill-mode: both` with the hidden state *inside* the keyframe,
   never as a static `opacity: 0` — so if animations never run the content is
   simply visible. The error page already documents and does this
   (`style.css:1275-1282`). This is the pattern that keeps a reduced-motion or
   JS-free reader from getting a blank page, and it is now a written rule.
3. No autoplay, no looping motion in body content. The one infinite animation
   (`cursor-blink`) is chrome, is 1.2s step-end (not a flash — it is a two-state
   toggle well under the 3Hz photosensitive threshold), and is guarded.
4. **Reduced-motion alternative** is always *absence*, never a substitute
   animation. State is still communicated because every animated state also has a
   non-animated signal (§3.7).

### 3.6 Error states

The design system has three failure modes of its own, plus the site's HTTP errors
which it styles.

| # | Trigger | Presentation | Recovery | Data loss |
|---|---|---|---|---|
| E1 | `localStorage` unavailable or throws | Silent. Theme falls back to `system` resolution; menu still operable for the session | Reader re-picks each visit | No — a theme slug is the only stored value |
| E2 | Stored theme slug not in `MODES` (roster shrank, storage tampered) | Silent fallback to `system` (`theme-init.js:9`, `main.js:18`) | Automatic | No |
| E3 | `style.css` fails to load | Browser default styling; document is semantic HTML with a real heading outline, real links, real landmarks, so it remains fully usable | Reload | No |
| E4 | 404 / 500 | Full-page boot-log column (`templates/error_404.html`), `SECTOR NOT FOUND`, the failed path in `--code` colour, and `(A)bort → return home` | The home link | No |

**Justification of presentation choice.** E1–E3 are *inline/silent* rather than
banner or toast because the failure is invisible to the reader's task — a
preference did not persist. Interrupting a reader with a toast about a colour
preference is worse than the failure. E4 is *full page* because the requested
resource does not exist; there is nothing to overlay.

**No toast/snackbar component exists or is proposed.** Toasts require JS to
appear and JS to dismiss, which puts a message class behind the no-JS floor. Any
future transient message must be a server-rendered region on the next page load.

### 3.7 Accessibility

This section is graded as an auto-fail gate. It is written as invariants the
system must hold, followed by the shipped state of each.

**A. Contrast — WCAG 2.1 AA at usage size, all 23 themes.**

The contract is a matrix, not a list: **every (foreground token × background
token × smallest rendered size)** pair is audited.

- Backgrounds in play: `--bg` and `--surface`. `--surface` is a real text
  background — `.tag`, `.status-readout`, `.theme-menu`, `.post-content pre/code`,
  and the floating wiki nav all sit on it.
- Thresholds by **rendered size**, not by token name: 4.5:1 for anything below
  18.66px bold / 24px regular; 3:1 above; 3:1 for the boundary of any interactive
  control (WCAG 1.4.11).
- `--text-faint` is rendered at 0.65rem–0.8rem everywhere it appears
  (`.theme-group-label` 0.65rem `:656`, `.wiki-sidebar h2` 0.68rem `:1342`,
  `.vitals-strip` 0.75rem `:822`, `.post-date` 0.78rem `:973`, `.site-footer`
  0.8rem `:794`). That is small text. It must clear **4.5:1**, not the 3.0
  the generator currently asks for (`generate_themes.py:146-148`).

Shipped state and the 19 failures this exposes are tabulated in §7.1.4.

**B. Colour independence.** No state is signalled by hue alone. Enumerated:

| State | Hue signal | Non-hue signal | Status |
|---|---|---|---|
| Active nav item | `--accent` text | Full-width 1.5px underline via `::after` (`:688-699`) | ✅ shipped |
| Current theme in menu | `--accent` text | `✓` glyph via `::after` + `aria-checked="true"` (`:678`, `main.js:37`) | ✅ shipped |
| Project status | `--accent` / `--text` / `--text-faint` border+text | The status **word** is the content (`portfolio.html:24`) | ✅ shipped |
| Active wiki page | `--accent` text | 2px left border appears where others are `transparent` (`:1355-1369`) | ✅ shipped |
| Links in prose | `--accent` | Underlined — the reset never removes `text-decoration`, so the UA default stands; chrome links opt out explicitly | ✅ shipped, **but accidental** — pin it as an invariant with a test |
| Hover on list rows | `--surface` fill | 2px inset left edge (`:705-708`) | ✅ shipped |
| Keyboard focus | `--accent` | 2px outline + 2px offset (`:685`) | ✅ shipped |

**Target addition:** `aria-current="page"` on the active nav link and the active
wiki sidebar link. Today the state is carried only by a CSS class, so it reaches
sighted readers and no one else.

**C. Focus.** `:focus-visible { outline: 2px solid var(--accent); outline-offset:
2px }` (`style.css:685`) — never removed, applied globally. `--accent` clears 3:1
against `--bg` in all 23 themes (min: cloud 4.7:1) and against `--surface` in all
23 (min: solarized 4.12:1), so the ring is visible on both. The skip link uses
`:focus` rather than `:focus-visible`, which is correct — it must appear for a
`Tab` press regardless of heuristics.

**D. Focus order.** `skip-link → brand → 4 nav links → theme button → [menu items
when open] → main content → footer links`. Menu items carry `tabindex="-1"`
(`main.js:43`) so they leave the tab order and are reached by arrows — the ARIA
APG roving-focus model, and the in-repo reference for any future widget.

**E. Semantics.** `<html lang="en">`; landmarks are `header` / `nav[aria-label]` /
`main#content` / `footer`; every decorative glyph is `aria-hidden` (the brand SVG
`base.html:21`, menu icons, vitals separators, theme group labels
`base.html:38`); the theme group's accessible name comes from
`role="group"` + `aria-label` rather than the visually-hidden-from-AT span.
`role="menu"` + `role="menuitemradio"` + `aria-checked` + `aria-haspopup` +
`aria-expanded` are all present and driven by `main.js`.

**Heading outline.** One `h1` per page. `h2` is a *section label* rendered smaller
than `h3` outside article content (`style.css:757-766`) — deliberate and
documented in the CSS, and defensible because the hierarchy is carried by case,
letter-spacing, and weight rather than size alone. Inside `.post-content`, real
article headings restore size order (`--text-xl` h2 > `--text-lg` h3,
`:1049-1062`). **Invariant:** no page may skip a level, and `.post-content` is the
only place `h2` is a full-size heading.

**F. Text scaling.** `body { font-size: 15px }` (`style.css:513`) is a **fixed
pixel size and therefore overrides the reader's browser font-size preference** —
the one WCAG 1.4.4 problem in the current stylesheet. Target: `body { font-size:
var(--text-md) }` with `--text-md: 0.95rem`, which renders 15.2px at a default
16px root and *scales* when the reader raises it. Zoom already works; browser
font-size preference does not, and will.

**G. Motion.** §3.5. One unguarded transition to fix.

**H. No-JS.** §3.2 branch, §6.4.

---

## 4. Implementation Specification

### 4.1 Architecture placement

The design system is not a Rust module. It spans four artifacts:

```
docs/themes/generate_themes.py    # SOURCE OF TRUTH: palettes, audit, emitters
docs/design/DESIGN_SYSTEM.md      # NEW: the long-lived spec (replaces SOLARCORE_SPEC.md)
docs/solarcore/SOLARCORE_SPEC.md  # becomes a 5-line superseded stub
docs/themes/README.md             # operational how-to; stops restating design rules
static/css/style.css              # the four CSS layers (§4.2)
static/js/theme-init.js           # pre-paint applier (generated MODES)
static/js/main.js                 # menu driver (generated MODES + ICON)
templates/base.html               # generated menu markup; asset versioning
static/img/favicon.svg            # regenerated from the flagship pair
docs/solarcore/generate_brand.py  # brand geometry source; gains favicon emission
tests/design_system.rs            # NEW: drift guards runnable without Python
```

Rust code touches the system in exactly one place: a new
`crate::state::asset_version()` so `base.html` can stamp cache-busting query
strings from `BUILD_TS` instead of a hand-typed literal (§4.3).

### 4.2 Data model

**The four CSS layers.** Order in the file is the contract; each layer may only
reference layers above it.

```
Layer 0  Reset                    style.css:1-9        no tokens
Layer 1  Theme layer              style.css:11-470     COLOUR + FONT ROLE only
Layer 2  Measurement layer        style.css:472-500    SIZE + SPACE + LAYOUT only
Layer 3  Component layer          style.css:502-1471   references L1 + L2, zero literals
```

**The governing rule (criterion 2F):** *themes own colour and font role; they
never own size or spacing.* Its corollary is equally binding: *the measurement
layer never contains a colour.* One bare `:root` means 23 themes inherit one set
of measurements instead of repeating them 23 times — which is why the type scale
landed outside the roster in commit `3f96165` and why it must stay there.

**Layer 1 — theme token contract.** 14 colour/font tokens + `color-scheme`,
redefined in full by every theme so no page is half-themed. Two changes to the
shipped contract:

```css
/* Per theme, generated by generate_themes.py::_block */
--bg              /* page background                                    */
--surface         /* panels, code, menu, pills, floating nav            */
--text            /* headings, strong, high emphasis                    */
--text-muted      /* body copy                                          */
--text-faint      /* metadata — audited at 4.5:1, it is SMALL text      */
--border          /* decorative dividers and rules                      */
--border-subtle   /* list dividers                                      */
--border-strong   /* NEW: boundary of interactive controls; ≥3:1 vs --bg */
--accent          /* the interactive signal colour                      */
--accent-hover    /* accent, one step                                   */
--accent-border   /* accent at 0x44 — decorative tint ONLY (§4.2 note)  */
--code            /* the second signal colour: code, brand tone B       */
--shadow-color    /* CHANGED: colour half of the elevation shadow       */
--font-body       /* THE ONE font exception — role, not size            */
--font-mono
color-scheme      /* light | dark — drives UA form controls and scrollbars */
```

*Why `--border-strong` is legitimately theme-owned:* it is a colour, it must be
tuned per palette, and it goes through the generator so adding it is **one edit
producing 23 blocks** — not 23 edits. That is precisely the "genuinely a palette
concern" carve-out.

*Why `--shadow` splits:* today it is `0 8px 24px rgba(0,0,0,0.5|0.12)` — geometry
and colour fused, duplicated 23 times, and the geometry half is a *measurement*
sitting in the theme layer, a direct violation of the governing rule. Split:
`--shadow-color` stays in Layer 1; `--shadow: 0 8px 24px var(--shadow-color)`
moves to Layer 2.

*Note on `--accent-border`:* it is `--accent` at `0x44` (26.7% alpha), which
computes to **1.41:1–2.08:1 against `--bg` in every one of the 23 themes**. It is
therefore forbidden as the sole boundary of anything interactive, and is permitted
only as decorative tint (its shipped use on `.status-active`, a non-interactive
pill whose meaning is carried by its text, is compliant). This constraint is
written into the token's doc comment so it cannot be misused later.

**Layer 2 — measurement contract.** Extends the block that landed in `3f96165`:

```css
:root {
    /* Type scale — 1.125 ratio anchored on the 15px body size */
    --text-2xs: 0.70rem;  /* NEW floor — nothing on the site renders smaller */
    --text-xs:  0.75rem;
    --text-sm:  0.85rem;
    --text-md:  0.95rem;  /* body */
    --text-lg:  1.05rem;
    --text-xl:  1.30rem;
    --text-2xl: 1.60rem;
    --text-3xl: 2.20rem;  /* NEW — error-page boot title, the only user */

    /* Line height */
    --leading-tight:  1.25;   /* h1 */
    --leading-normal: 1.5;
    --leading-prose:  1.7;    /* body */

    /* Vertical rhythm */
    --space-1: 0.35rem;  --space-2: 0.75rem;  --space-3: 1.1rem;
    --space-4: 1.5rem;   --space-5: 2rem;     --space-6: 2.5rem;

    /* Measure and layout */
    --measure:        72ch;
    --measure-narrow: 55ch;
    --layout-column:  900px;
    --layout-wide:    1200px;
    --layout-narrow:  700px;

    /* Geometry */
    --radius-sm: 3px;  --radius-md: 6px;  --radius-lg: 9px;
    --rule: 1px;
    --focus-width: 2px;  --focus-offset: 2px;
    --shadow: 0 8px 24px var(--shadow-color);
}
```

**Floor rule:** no rendered text below `--text-2xs` (0.70rem ≈ 11.2px at default
root). This retires the shipped 0.65rem theme-group label and the 0.68rem wiki
sidebar heading.

**Layer 3 — component rules.** Every `font-size`, `color`, `background`,
`border-color`, `margin`, and `padding` resolves to a Layer-1 or Layer-2 token.
Two enforced prohibitions: **zero colour literals** and **zero `font-size`
literals** below the layer boundary. Both are tested (§5.1).

**Python source of truth.** `THEMES` in `generate_themes.py:21-122` is a list of
dicts, one per theme, carrying palette + `icon` + `label` + `scheme` + `font` +
optional `glow` + optional `default`. `MENU_GROUPS` (`:245-253`) is the display
ordering. Additions:

```python
# What each token is actually rendered on and at what size — drives the audit.
# Keep in step with static/css/style.css; the CSS lint test in tests/design_system.rs
# fails if a font-size literal appears outside the scale, which is what makes the
# "smallest size" column below trustworthy.
USAGE = [
    # token,        backgrounds,          smallest rendered size, threshold
    ("text",        ("bg", "surface"),    "0.85rem", 4.5),
    ("muted",       ("bg", "surface"),    "0.78rem", 4.5),
    ("faint",       ("bg", "surface"),    "0.70rem", 4.5),  # was 3.0 — wrong
    ("accent",      ("bg", "surface"),    "0.72rem", 4.5),
    ("accent_hover",("bg", "surface"),    "0.80rem", 4.5),
    ("code",        ("bg", "surface"),    "0.90rem", 4.5),
    ("border_strong",("bg",),             None,      3.0),  # WCAG 1.4.11
]
```

No database, no migrations — the site has no database.

### 4.3 API contracts

**Generator CLI** (`python3 docs/themes/generate_themes.py`):

| Invocation | Returns | Exit code | Change |
|---|---|---|---|
| *(no args)* | all four artifacts + contrast report | 0 always | unchanged |
| `--css` / `--modes` / `--icons` / `--menu` | that artifact on stdout | 0 | unchanged |
| `--check` | contrast report over the full USAGE matrix | **non-zero on any failure** | **NEW** — the audit is currently advisory (`:296-299` prints and returns 0) |
| `--write` | rewrites the generated regions of `style.css`, both JS files, and `base.html` in place | 0, or non-zero on a missing region marker | **NEW** — replaces the hand-splice ritual in `docs/themes/README.md` |

`--write` operates on explicit region markers so the hand-written parts of each
file are never touched:

```
/* >>> generated: theme-roster — do not edit by hand */    …    /* <<< generated */
// >>> generated: modes                                    …    // <<< generated
<!-- >>> generated: theme-menu -->                         …    <!-- <<< generated -->
```

The existing `emit_menu` drift guard (`:262-268`, raises `SystemExit` when
`MENU_GROUPS` and `THEMES` disagree) is kept and joined by a second: `--write`
refuses to run if a marker pair is missing or unbalanced.

**Rust function** — one addition, in the existing `crate::state` module beside the
`Status::current()` that `vitals_strip.html` already calls:

```rust
/// Cache-busting token for versioned static assets.
///
/// Returns the compile-time `BUILD_TS` stamped by `build.rs`, so the query string
/// changes on every rebuild and can never go stale the way a hand-typed literal
/// can. `templates/base.html` renders it into the CSS, JS, and favicon URLs.
pub fn asset_version() -> &'static str {
    env!("BUILD_TS")
}
```

Auth, pagination, and rate limiting are N/A — nothing here is an HTTP endpoint.
Static assets are served by `ServeDir` (`router.rs:59`) with `Last-Modified`
revalidation and no explicit `Cache-Control`, so the version query is belt-and-
braces rather than the sole cache key; that is why generating it is preferable to
either keeping the manual bump or dropping it.

### 4.4 State management

| State | Owner | Lifetime | Sync |
|---|---|---|---|
| Chosen mode (one of 24 slugs) | `localStorage["theme"]` | Until cleared | Local only, never sent to the server |
| Resolved theme | `document.documentElement[data-theme]` | Per document | Derived from mode + `prefers-color-scheme` |
| Menu open/closed | `.theme-menu[hidden]` + `aria-expanded` | Per interaction | None |
| Checked item | `aria-checked` + `.is-current` | Per interaction | Derived from mode |

**No new state container.** The site has no store, no framework, and no
server-side session; adding one for a colour preference would be the single worst
change available. Server-synced state boundary: **none crosses it** — the server
never learns the theme, which is also why the theme cannot be server-rendered and
why the pre-paint script exists at all.

**Offline / draft persistence:** N/A — nothing is authored in the browser.

### 4.5 Dependencies

- **New packages:** none. No CSS framework, no build step, no PostCSS, no
  webfonts. The CSP is `default-src 'self'; script-src 'self'; style-src 'self';
  font-src 'self'` (`security_headers.rs:38-50`) — a CDN dependency is not merely
  discouraged, it is blocked.
- **New assets:** a regenerated `static/img/favicon.svg`. **Deletions:**
  `static/img/mark.svg`, `mark-sm.svg`, `vine-trace.svg` (14,588 B, referenced
  nowhere).
- **Fonts:** system stacks only — `ui-monospace, SFMono-Regular, Menlo, Consolas,
  monospace`; `Charter, "Bitstream Charter", "Sitka Text", Georgia, serif`;
  `ui-rounded, "SF Pro Rounded", "Segoe UI", system-ui, sans-serif`
  (`generate_themes.py:13-15`). No files shipped, no licence obligations.
- **Infrastructure:** none. CI already runs on `ubuntu-latest`
  (`.github/workflows/ci.yml`), which provides `python3`, so the `--write` +
  `git diff --exit-code` guard needs no new tooling.

### 4.6 Platform-specific considerations

- **Browser support.** The stylesheet uses `:has()` (`style.css:740`),
  `::details-content` (`:1330`), `:focus-visible`, `color-scheme`, and
  `overscroll-behavior`. `:has()` is the binding floor: Chrome/Edge 105+, Safari
  15.4+, Firefox 121+ (Dec 2023). Degradation is graceful in every case — without
  `:has()` the wiki page renders at 900px instead of 1200px, which is narrower but
  entirely usable. **Rule:** no CSS feature may be adopted whose absence breaks
  reading; layout-enhancing features are fine, layout-critical ones are not.
- **`color-scheme`.** Every theme declares it, which is what makes native form
  controls, scrollbars, and the browser's own UI match the palette. It must stay
  in the token contract for that reason, even though nothing in `style.css` reads
  it.
- **Print.** There is **no `@media print` block** (verified: the only `@media`
  rules are `prefers-reduced-motion`, `max-width: 800px`, `max-width: 640px`).
  Printing a dark theme relies on the browser's default background-suppression,
  which is inconsistent. **Target:** a small print block that forces the Solarcore
  light tokens, hides chrome (header nav, theme control, vitals strip), removes
  `--measure` (paper has its own measure), and expands `pre` rather than scrolling
  it. This is one block and one review pass, and it is what the "Light & print"
  menu group name currently implies but does not deliver.
- **Feature flags / rollout.** N/A — a static stylesheet deployed as one unit.
  There is no mechanism, and inventing one for a personal site would be
  speculative complexity.

### 4.7 Performance budget

Measured today:

| Asset | Bytes | Notes |
|---|---|---|
| `static/css/style.css` | 41,272 (8,248 gzipped) | one file, 1,471 lines; the theme roster is lines 11–470 ≈ 30% |
| `static/js/main.js` | 4,664 (80 lines) | end of body, non-blocking |
| `static/js/theme-init.js` | 960 (15 lines) | **render-blocking** in `<head>` |
| `static/img/favicon.svg` | 2,387 | the only image any page requests |
| `static/img/{mark,mark-sm,vine-trace}.svg` | 14,588 | requested by nothing |

Budgets:

- **CSS ≤ 48 KB uncompressed / ≤ 10 KB gzipped.** Current headroom is ~6.7 KB
  uncompressed. Adding a 24th theme costs ~620 B uncompressed / ~90 B gzipped
  (palettes compress extremely well — 23 near-identical blocks).
- **Render-blocking JS ≤ 1.5 KB.** `theme-init.js` is 960 B and must stay tiny:
  it sits between the request and first paint on every page.
- **Total JS ≤ 8 KB, and the count is 95 lines across two files** (80 + 15).
  Where the site's identity is described as "80 lines of JavaScript", that is
  `main.js` alone; the honest total is 95. The doc says 95.
- **Zero additional network requests.** No font fetch, no CDN, no sprite sheet,
  no second stylesheet.
- **Memory / CPU:** one `matchMedia` listener and one `document` click listener,
  both attached only while the menu is open (`main.js:55-56`). The CRT scanline is
  a single fixed `repeating-linear-gradient` pseudo-element with
  `pointer-events: none` — composited, not repainted on scroll.
- **Storage:** one `localStorage` key holding a slug ≤ 10 bytes. No cookies.
- **Startup:** unchanged. No build step for CSS; `cargo build --release` is
  unaffected by any of this.

**Rejected optimisation, recorded with its reason.** Splitting the 23 palettes
into per-theme stylesheets loaded on demand would cut ~12 KB uncompressed but
requires JS to select and inject the correct sheet, which reintroduces a
flash-of-wrong-theme, adds a request on the critical path, and puts the *palette*
behind the no-JS floor. The 8.2 KB gzipped whole-roster file is cheaper than the
correctness it would cost.

---

## 5. Test Specification

Verification runs in CI today via `.github/workflows/ci.yml`: `cargo fmt --all --
--check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`,
`cargo build --release`. New tests must land inside those commands, plus one new
Python step.

### 5.1 Unit tests — `tests/design_system.rs` (new integration-test crate)

These parse the shipped asset files as text. That is deliberate: the design system
*is* text files, and a test that reads them catches exactly the drift class that
has already occurred once (`fbc6c2e`).

| # | Name | Setup | Assertion | Edge case covered |
|---|---|---|---|---|
| T1 | `every_theme_defines_the_full_token_contract` | Read `style.css`, extract each `:root[data-theme="…"]` block | Each block declares all 15 contract entries (14 tokens + `color-scheme`) | A theme added by hand with a token forgotten → half-themed page |
| T2 | `theme_registries_agree` | Collect slugs from: CSS `[data-theme=]` selectors, `theme-init.js` `MODES`, `main.js` `MODES`, `main.js` `ICON` keys, `base.html` `data-mode` attributes | All five sets equal, modulo the `system` pseudo-mode which must be present in the three JS/HTML registries and absent from CSS | A theme spliced into three of four registries — the failure the README's hand-splice invites |
| T3 | `no_colour_literals_outside_the_theme_layer` | Read `style.css` from the `>>> generated` end-marker to EOF | Zero `#rgb`/`#rrggbb`/`rgb(`/`hsl(` matches | **Currently fails:** `#e0a458` at `:1005` |
| T4 | `no_font_size_literals_outside_the_scale` | Same region | Every `font-size:` value is `var(--text-*)`, `inherit`, or an `em` value on `code` | **Currently fails: 34 occurrences** |
| T5 | `no_theme_owns_a_length` | Read the generated region | No theme block declares `px`/`rem`/`ch`/`%` outside `--font-*` | Guards the governing rule from the other direction |
| T6 | `default_theme_is_not_a_glow_theme` | Parse the glow selector list and the bare `:root` | The default palette is not in the glow set, and neither `system` resolution target is | Someone makes CRT the default |
| T7 | `theme_control_is_hidden_without_a_resolved_theme` | Read `style.css` | `.theme-select` has `display: none` at base and is revealed only under `:root[data-theme]` | The no-JS dead-control regression |
| T8 | `every_class_in_markup_has_a_rule_or_an_exemption` | Extract `class="…"` from `templates/*.html`, compare to selectors in `style.css` | Difference is empty, or listed in an explicit `UNSTYLED_HOOKS` const with a reason | **Currently fails:** `article-page`, `bio-loc`, `brand-word`, `post-group`, `post-group-heading`, `vitals-item` |
| T9 | `every_rule_has_a_consumer` | Reverse of T8 across templates **and** `content/**/*.md` | No selector matches nothing | **Currently fails:** `.wiki-disclaimer` (`:998-1013`), `.pm-status` and `.box` (`:714`) |
| T10 | `reduced_motion_covers_every_transition` | Scan for `transition:`/`animation:` declarations | Each is inside a `prefers-reduced-motion: no-preference` block, or in the `reduce` override | **Currently fails:** `:1410` |

### 5.2 Integration tests — `python3 docs/themes/generate_themes.py`

| # | Name | Assertion |
|---|---|---|
| P1 | `--check` contrast matrix | Every (token, background, size) pair in `USAGE` clears its threshold for all 23 themes. Exit non-zero otherwise. **Currently 19 failures** |
| P2 | contrast-function self-test | `contrast("#000","#fff") == 21.0` and `contrast("#777","#fff") ≈ 4.48` — the two canonical WCAG reference pairs, so a refactor of `_lin`/`_lum` cannot silently skew every result |
| P3 | `MENU_GROUPS` drift guard | Existing (`:262-268`); every slug grouped exactly once |
| P4 | codegen idempotence | `--write` then `git diff --exit-code` is clean. This is the guard that would have caught `fbc6c2e` before it happened |

CI gains one step after `cargo test`:

```yaml
- name: theme roster
  run: |
    python3 docs/themes/generate_themes.py --check
    python3 docs/themes/generate_themes.py --write
    git diff --exit-code
```

### 5.3 UI / E2E tests

**State: absent, and deliberately not proposed.** There is no browser-automation
harness in this repo and adding Playwright to a site with 95 lines of JavaScript
would be a heavier dependency than the thing it tests. The behaviours an E2E suite
would cover are instead covered by: T2/T7 (registry and no-JS gating, statically),
the existing `pages.rs` and `wiki_pages.rs` render tests (server output), and the
manual matrix in §5.4 (visual). This is stated as a decision, not an omission.

Rendered-HTML assertions that *do* belong in `cargo test --all-targets`, added to
the existing `#[cfg(test)] mod tests` blocks in `src/handlers/`:

- The skip link is the first focusable element in the body.
- The active nav link carries `aria-current="page"`.
- No template emits an inline `style=` attribute (the design system owns
  presentation; an inline style is by definition outside the token contract).

### 5.4 Visual / manual verification

23 themes × 11 surfaces = 253 combinations. An unrunnable checklist is worse than
none, so verification is tiered.

**Tier 1 — every visual change. Six themes, chosen to cover every structural axis:**

| Theme | Axis it covers |
|---|---|
| Lunarcore | the default; dark; mono; what most visitors see |
| Solarcore | light; the `system` light resolution |
| Paper | serif `--font-body` — the axis that breaks `ch` measures and cap-heights |
| Cloud | sans `--font-body`; also the *lowest* accent-on-surface ratio (4.32:1) |
| Solarized | the tightest palette in the roster — if it passes here it passes everywhere |
| CRT | the scanline overlay and the neon glow path |

**Tier 2 — token-contract changes only.** All 23, using a scratch page that renders
every component once.

**Surfaces checked in both tiers:** home, an article (`/blog/network-migration`), a
`/learn` page with sidebar, portfolio (status pills), `/status` (the `--surface`
panel), and a 404.

**Configuration extremes, checked in Tier 1:**
- Browser default font 24px **and** 200% zoom (WCAG 1.4.4 / 1.4.10).
- Viewport 320px, 800px (the wiki breakpoint boundary), 1280px, 2560px.
- `prefers-reduced-motion: reduce` forced.
- `prefers-color-scheme: light` **with JavaScript disabled** — the specific
  regression this spec's no-JS work exists to fix.
- Empty states: a blog list with zero posts, a releases page with no tarballs.
- Print preview (once the print block lands).

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement.**

The design system stores exactly one value: `localStorage["theme"]`, a slug from a
24-item allowlist, validated on read (`theme-init.js:9`, `main.js:18`). It is a
functional preference, never transmitted — the server has no knowledge of the
reader's theme, which is *why* the pre-paint script exists. No cookies, no
fingerprinting surface beyond what `prefers-color-scheme` already exposes to every
site, no consent obligation. The `/status` readout it styles carries process-wide
counters only and explicitly retains no per-visitor data
(`templates/status.html:43-46`).

### 6.2 Asset provenance

- [x] **Uses third-party assets** — palettes and names, no files.

| Asset | Source | Licence | Rights status |
|---|---|---|---|
| Font stacks (mono / Charter serif / rounded sans) | OS-provided | N/A — no files shipped | Clear. CSP `font-src 'self'` and zero webfonts means no font is ever distributed by this site |
| Dracula palette (`dark`) | dracula/dracula-theme | MIT | Clear; attribution to be added to `DESIGN_SYSTEM.md` |
| Solarized palette | Ethan Schoonover | MIT | Clear; attribution to be added |
| Nord palette | Arctic Ice Studio | MIT | Clear; attribution to be added |
| Gruvbox palette | Pavel Pertsev | MIT | Clear; attribution to be added |
| Hardware-era palettes (`gameboy`, `c64`, `nes`, `teletext`, `amber`, `crt`) | Hand-approximated from historical display characteristics | N/A — colour values are not copyrightable | Clear |
| Brand mark + favicon | Generated by `docs/solarcore/generate_brand.py` in this repo | Original work | Clear |
| Reference art `docs/solarcore/reference/*.png` | AI-generated mood boards | Repo-internal | **Never shipped to users** — `docs/` only. The stale spec explicitly warns these are mood references, not artwork to trace; that warning carries into the rewrite |

**Trademark note (raised, not hidden).** Four theme *labels* reference marks owned
by others: "Game Boy" (Nintendo), "Commodore", "Tron" (Disney), "Dracula"/"Nord"/
"Gruvbox"/"Solarized" (project names, MIT-licensed). The roster already softened
one — the `nes` slug is labelled "8-Bit" — which shows the concern was live.
Recommendation: **keep the remaining labels.** They are nominative references
identifying the visual style being evoked, use no logos or wordmarks, imply no
endorsement, and are the ordinary convention across editor-theme ecosystems. The
mitigation is an attribution paragraph in `DESIGN_SYSTEM.md` stating exactly that.
Flagged as Q3 in §8 because it is a judgment call, not a technical one.

### 6.3 Language / claims audit

- [ ] Make claims not supported by evidence — **no.** The only user-visible text
      this feature owns is 24 theme labels, six group labels, and ARIA labels.
      None asserts anything about the author.
- [ ] Promise capabilities not yet built — **no**, and the spec is explicit about
      state throughout: the print stylesheet, `--border-strong`, `og-card.png`,
      and the write-up in §8 are all labelled **planned/absent**, never described
      as shipped.
- [ ] Use language restricted by domain regulations — **no.**

**Claim the rewrite removes:** the stale spec's §1 assertion that
"machinageist.dev is a **systems programmer's** portfolio" (D7). The site's own
tests forbid unearned identity claims in page copy (`pages.rs:163-166`,
`project.rs:109-115`, `lab.rs:258-269`); a design document sitting in the same repo
should not be the one place the discipline lapses. The rewrite describes the
artifact, not the author.

**Claim currency (criterion 1D).** The design system introduces no certification
copy. It also does not *fix* the stale "working through the CompTIA stack" copy at
`pages.rs:92` — that is B2's job, and this spec must not silently absorb it. What
this spec does own is a related trap: `.status-*` classes are the visual vocabulary
for *claim state*, and §7.2 requires the missing `queued`/`completed` variants
before C4 can render `/labs` — so that "queued" reads visibly as *not started*
rather than inheriting a style that suggests otherwise.

### 6.4 Regulatory alignment — criteria Lens 3

| Criterion | How this spec addresses it |
|---|---|
| **3A Works without JavaScript** *(auto-fail)* | Core function — reading every page in a correct, complete theme — is already reachable with JS off, because the bare `:root` carries the full Lunarcore palette (`style.css:17-33`) and all content is server-rendered by Axum + Askama. This spec closes the two remaining gaps: a `prefers-color-scheme: light` CSS fallback so the OS preference is honoured without JS (§3.2), and gating `.theme-select` on `[data-theme]` so the JS-only control does not appear as a dead, focusable button (§3.7, T7). JS remains a pure enhancement: it adds *choice*, never *access*. |
| **3B Contrast and colour independence** *(auto-fail)* | §3.7A defines the audit as a (token × background × usage-size) matrix rather than the current token-vs-`--bg` list, raises `--text-faint` to the 4.5:1 small-text threshold it actually needs, adds `--border-strong` for the 3:1 interactive-boundary requirement, and makes the audit exit non-zero (P1). §3.7B enumerates every state on the site and its non-hue signal; all seven already have one, and two gain `aria-current`. |
| **3C Keyboard and focus** | Global `:focus-visible` ring verified ≥3:1 against both `--bg` and `--surface` in all 23 themes; APG roving-focus menu documented as the in-repo reference pattern (§3.2, §3.7C–D); theme-button hit area raised to 44px (§3.4). |
| **3D Semantics and AT** | §3.7E: landmarks, one `h1` per page, decorative glyphs `aria-hidden`, group naming via `aria-label`, heading-outline invariant including the deliberate `h2`-as-label inversion. |
| **3E Motion and sensory safety** *(auto-fail)* | §3.5 inventories all nine motion sources, identifies the one unguarded transition (`:1410`), and states the rules: every `transition`/`animation` behind `no-preference`; hidden state inside the keyframe with `fill-mode: both`; no autoplay; no body-content animation; reduced-motion alternative is absence. T10 enforces it. |
| **3F Responsive and resilient** | §3.4 two breakpoints, §5.4 extremes from 320px to 2560px at 24px default font and 200% zoom, §3.3 empty-state rules, §3.6 E3 (stylesheet fails to load → semantic HTML still reads). |

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**7.1.1 Theme system — `implemented`, and good.** 23 palettes, each redefining a
14-token contract, generated from `docs/themes/generate_themes.py:21-122`; a
pre-paint applier (`theme-init.js`); an APG-conformant roving-focus menu
(`main.js:42-75`) grouped into six labelled sections (`base.html:36-79`,
`generate_themes.py:245-282`); a WCAG audit that reports **0 failures** at its own
thresholds. The `MENU_GROUPS` drift guard (`:262-268`) was added in `fbc6c2e`
after exactly the failure criterion 5B cites.

**7.1.2 Type scale and rhythm — `implemented`, partially adopted.** Landed in
`3f96165` as a bare `:root` block outside the roster (`style.css:472-500`) with
`--text-xs..2xl`, `--space-1..6`, `--measure: 72ch`. The governing rule — themes
own colour and font role, never size or spacing — is written into the file's own
comment (`:474-477`) and is correct. Adoption is partial: `h1`, `h2`, `h3`, `p`,
`hr`, and the `.post-content` block use the tokens; **34 `font-size` declarations
elsewhere remain literals**, along with most `margin`/`padding`. Prose caps at
`--measure`; `pre` deliberately does not, keeping the full column and scrolling
(`:1081-1089`). Both decisions are correct and this spec preserves them.

**7.1.3 Brand — `implemented` in markup, `stale` in assets.** The header mark is
inline SVG whose two halves stroke `--accent` and `--code` (`base.html:21`,
`style.css:571-577`), so it recolours per theme — the best decision in the system.
But `static/img/favicon.svg` hardcodes `#010915`/`#46c8f0`/`#e23a9a`, the
abandoned night palette, matching **no shipped theme**; and `mark.svg`,
`mark-sm.svg`, `vine-trace.svg` (14,588 B) are referenced by no template or
stylesheet, preserving a second contradicting brand definition in `static/`.

**7.1.4 Accessibility — `implemented` with real, quantified gaps.**

*Contrast.* The generator audits five tokens against `--bg` only, with `--text-faint`
held to the AA-**large** 3.0 threshold (`generate_themes.py:146-148`). Re-running
the same maths against the backgrounds and sizes the tokens are *actually*
rendered at produces **19 failures**:

| Failure class | Themes affected | Worst ratio |
|---|---|---|
| `--text-faint` vs `--bg` at small size (needs 4.5, audited at 3.0) | `gameboy` 3.97, `c64` 3.93, `solarized` 3.64 | 3.64:1 |
| `--text-faint` vs `--surface` (background never audited) | `lunarcore` 4.47, `nes` 4.47, `blueprint` 4.41, `cloud` 4.28, `gameboy` 3.43, `c64` 3.27, `solarized` 3.15 | 3.15:1 |
| `--text-muted` vs `--surface` | `solarized` 4.13 | 4.13:1 |
| `--accent` vs `--surface` | `cloud` 4.32, `solarized` 4.12 | 4.12:1 |
| `--code` vs `--surface` — this is `.post-content code`, i.e. **inline code in every article** | `solarized` 4.06 | 4.06:1 |

`solarized` accounts for five of the nineteen and is the palette that most needs
either a `--surface` lightening or removal from the roster.

*Non-text contrast.* `--border` sits at **1.38:1–2.18:1** against `--bg` in every
theme. As a decorative divider that is fine. As the boundary of `.theme-btn`
(`style.css:609`) — the site's only interactive control — it fails WCAG 1.4.11's
3:1 requirement in all 23 themes. `--accent-border` (accent at 26.7% alpha) is
worse, 1.41:1–2.08:1, and is correctly used only on non-interactive pills.

*Text scaling.* `body { font-size: 15px }` (`:513`) is a fixed pixel size and
overrides the reader's browser font-size preference.

*Motion.* Nine motion sources; eight guarded, one not (`:1407-1411`).

*No-JS.* Site fully readable; the OS light preference is honoured only via JS, and
the theme button renders as a focusable dead control when JS is off.

**7.1.5 Drift surface — `implemented` for one of four vectors.**

| Vector | Guard today | Verdict |
|---|---|---|
| `MENU_GROUPS` vs `THEMES` | `SystemExit` in `emit_menu` | ✅ guarded (`fbc6c2e`) |
| Generator output vs the four committed files | **none** — `docs/themes/README.md` documents a hand-splice into three files | ❌ the live risk |
| Contrast audit | prints, always exits 0 (`:296-299`) | ❌ advisory only |
| `?v=` cache-bust in `base.html:11-13` | hand-typed `20260719-spectrum`, documented as a manual step | ❌ **already stale** — `3f96165` changed `style.css` and touched no other file, so the marker still reads July 19 |

**7.1.6 Dead and orphaned CSS — `absent` consumers.**
- `.wiki-disclaimer` (`:998-1013`) has **no consumer** in `templates/` or
  `content/`, and carries the file's only colour literal, `#e0a458` (`:1005`).
- `.pm-status` and `.box` appear only inside the transition selector list
  (`:714`) — dead selectors.
- Six classes are in markup with no rule: `article-page`, `bio-loc`, `brand-word`,
  `post-group`, `post-group-heading`, `vitals-item`.

**7.1.7 Status vocabulary — `implemented` for projects, `absent` for labs.**
`.status-active` / `.status-in-progress` / `.status-complete` exist
(`style.css:1185-1187`) and match `ProjectStatus::class_name`
(`project.rs:48-54`). `LabStatus::class_name` (`lab.rs:50-58`) emits `queued` and
`completed` — **neither has a rule.** `src/models/lab.rs` is untracked, has 12
entries all `Queued` with three tests (`:212-270`), and no handler, route, or
template. C4 cannot render without this.

**7.1.8 The governing document — `stale`.**
`docs/solarcore/SOLARCORE_SPEC.md` contradicts the shipped site in the eight ways
tabulated in §1.4. `docs/themes/README.md` is accurate but restates design rules
that will now live in one place, and documents the hand-splice ritual that §4.3
replaces.

### 7.2 Delta to spec

**New files**
- `docs/design/DESIGN_SYSTEM.md` — the long-lived spec: token contract, the four
  layers, the governing rule, contrast policy, motion policy, no-JS policy,
  brand, palette attributions, and a non-normative Origins section carrying the
  stale spec's philosophy prose and reference art.
- `tests/design_system.rs` — T1–T10.

**Modified files**
- `docs/solarcore/SOLARCORE_SPEC.md` → five-line superseded stub.
- `docs/themes/README.md` → operational how-to only; the hand-splice section is
  replaced by `--write`; design rules become a link.
- `docs/themes/generate_themes.py` → add `--border-strong` and `--shadow-color` to
  the token contract and drop `--shadow`'s geometry; add the `USAGE` matrix;
  `--check` exits non-zero; add `--write` with region markers; emit the
  `@media (prefers-color-scheme: light)` no-JS fallback block; delete the dead
  double assignment in `emit_icons` (`:235-237`); add the P2 self-test.
- `static/css/style.css` → regenerate Layer 1; extend Layer 2 (§4.2); Layer 3
  cleanup: 34 `font-size` literals → scale tokens, remove `#e0a458` and the dead
  `.wiki-disclaimer`/`.pm-status`/`.box` rules, resolve the six orphan classes,
  `body` font-size → `var(--text-md)`, `.theme-btn` boundary → `--border-strong`
  and 44px hit area, `.theme-select` gated on `[data-theme]`, `.status-queued` and
  `.status-completed` added, `--measure-narrow`/layout tokens replace the four
  hardcoded `ch`/`px` measures, the `:1410` transition moved behind
  `no-preference`, and a `@media print` block.
- `static/js/theme-init.js` → narrow the `try` to the storage read so
  `setAttribute` always runs; regenerated `MODES`; region markers.
- `static/js/main.js` → regenerated `MODES`/`ICON`; region markers.
- `templates/base.html` → region markers around the menu; `?v={{ … }}` from
  `crate::state::asset_version()`; `aria-current="page"` on the active nav link.
- `templates/wiki_page.html` → `aria-current="page"` on the active sidebar link.
- `src/state.rs` → `asset_version()`.
- `static/img/favicon.svg` → regenerated from the flagship pair, with an internal
  `prefers-color-scheme` stylesheet so the tab icon matches the `system`
  resolution.
- `docs/solarcore/generate_brand.py` → emit the favicon from the default theme's
  tokens so mark and palette cannot drift again.
- `.github/workflows/ci.yml` → the three-line theme-roster step (§5.2).

**Deleted**
- `static/img/mark.svg`, `mark-sm.svg`, `vine-trace.svg` (14,588 B, zero
  references).

**Migrations / schema changes:** none — no database.
**New dependencies:** none.

### 7.3 Estimated scope

**M**, with one L-shaped risk.

The mechanical work is large but shallow and highly parallel: 34 `font-size`
substitutions, one generator extension, one new test file. It is M rather than L
because there is no new architecture — the four-layer model already exists in the
file, the generator already exists, and the CI already runs the right four
commands.

The L-shaped risk is the **19 contrast failures**. Fixing them means adjusting
`--text-faint` and `--surface` in roughly seven palettes, and every adjustment is a
visual judgment that must be re-eyeballed, not just re-computed. `solarized`
specifically may not be salvageable while remaining recognisably Solarized (its
canonical `base01`/`base00` relationship is *why* it fails), which turns a colour
tweak into a roster decision — hence Q2 in §8.

Sequencing, each step independently shippable and verifiable:

1. **Guards first** — `--check`, `--write`, region markers, `tests/design_system.rs`
   with T1/T2/T5/T6 (the ones that pass today). *Verify:* CI green; the new tests
   pass on an unmodified tree. This step alone stops future drift.
2. **Contrast** — `USAGE` matrix, `--border-strong`, palette fixes, `--check` in
   CI. *Verify:* `--check` exits 0; Tier-1 visual pass.
3. **No-JS and motion** — light-preference fallback, `.theme-select` gating,
   `theme-init.js` try-scope, `:1410`. *Verify:* T7/T10; manual pass with JS off
   under both OS preferences.
4. **Type and measurement** — Layer-2 extension, 34 literals, `body` rem, layout
   tokens. *Verify:* T3/T4; 24px-default-font pass.
5. **Cleanup and brand** — dead CSS, orphan classes, favicon, asset deletions,
   `asset_version()`, print block, `.status-queued`/`.status-completed`.
   *Verify:* T8/T9; print preview.
6. **Documentation** — write `DESIGN_SYSTEM.md`, stub the old spec, retarget
   `docs/themes/README.md`. *Verify:* every §1.4 divergence appears in the new
   document with its reconciliation.

### 7.4 Blocking dependencies

**Upstream: none.** A1 is the root of the foundation tier and depends on no other
feature.

**Downstream — A1 blocks:**
- **A2 site-shell** — consumes every token; owns `base.html`, so the
  `aria-current`, `asset_version()`, and `og:image` wiring land there against this
  contract.
- **A3 ops-and-observability** — styles `/status` and the vitals strip on
  `--surface`; two of the 19 contrast failures are on that surface.
- **B1–B6** — every page inherits the type scale, measure, and status vocabulary.
- **C1 search** — a results page needs list-row and empty-state patterns that
  exist here and nowhere else.
- **C2 glossary, C3 study-tools** — the no-JS floor and the focus/keyboard model
  defined here are the constraints those specs must design within; C3c
  (PBQ simulations) is the hardest case in the tree and needs the ruling in §3.6
  that no toast/JS-only message class exists.
- **C4 progress** — cannot render `/labs` until `.status-queued` and
  `.status-completed` exist (§7.1.7).

**External gates:** none. No exam voucher, no engagement authorisation, no
GeistScope publication gate is implicated — this feature publishes no artifact and
makes no claim.

---

## 8. Open Questions

- **Q1 — Document location and name.** This spec moves the long-lived design doc to
  `docs/design/DESIGN_SYSTEM.md` and reduces `docs/solarcore/SOLARCORE_SPEC.md` to
  a stub, on the reasoning that a document named for one theme slug while
  governing 23 themes is the structural cause of divergences D1 and D4. The
  alternative is to rewrite in place and accept the name. — *Blocks:* §4.1, §7.2,
  and the wording of every downstream spec's "see the design system" reference.

- **Q2 — `solarized` fails five of the nineteen contrast checks.** Its
  `--surface: #073642` against `--text-faint: #6d8087` is 3.15:1, and inline code
  on that surface is 4.06:1. Options: (a) lighten `--surface` toward `base02`,
  which drifts from canonical Solarized; (b) darken the whole palette to the
  Solarized *dark* baseline properly; (c) drop `solarized` from the roster; (d)
  accept a documented exception, which criteria rule 2 forbids. Recommendation:
  (b) — the palette is currently a half-conversion. — *Blocks:* §7.3 step 2.

- **Q3 — Theme labels referencing third-party marks.** "Game Boy", "Commodore",
  and "Tron" are marks owned by others; the roster already softened `nes` to
  "8-Bit". §6.2 recommends keeping them with an attribution paragraph, as
  nominative use. The alternative is generic renames ("Handheld", "Home Computer",
  "Grid") at the cost of the joke. — *Blocks:* §6.2, and the `label` field for
  three entries in `THEMES`.

- **Q4 — Is `--text-faint` worth keeping at all?** It is the source of ten of the
  nineteen failures because it is a deliberately low-emphasis colour used at
  deliberately small sizes — the two failure modes multiply. Options: (a) raise
  every failing palette's `faint` until it clears 4.5:1 on both backgrounds, which
  compresses it toward `--text-muted` and may make the ramp pointless; (b) retire
  the token and use `--text-muted` at a smaller size for metadata; (c) keep it but
  forbid it below `--text-sm` (0.85rem). Recommendation: (c) plus (a) for the
  worst offenders. — *Blocks:* §4.2 token contract, §7.3 step 2.

- **Q5 — `prefers-contrast: more`.** A `@media (prefers-contrast: more)` block
  mapping `--text-muted` → `--text` and `--border` → `--border-strong`, and
  dropping the CRT scanline, costs about eight lines and no per-theme work.
  In scope now, or a later change? Recommendation: include it — it is nearly free
  once `--border-strong` exists. — *Blocks:* §4.2.

- **Q6 — Print stylesheet scope.** §4.6 proposes forcing the Solarcore light
  tokens on print. Should chrome be dropped entirely, or should the vitals strip
  print as a provenance stamp (it is the only place the build version appears)?
  Recommendation: print the vitals strip, drop the rest. — *Blocks:* §4.6, §5.4.

- **Q7 — Write-up as a blog post.** This reconciliation — a design spec and a
  codebase that disagreed in eight places, with the resolution reasoned out — is
  the shape criteria 1A and 4B describe: starting state, target state, what broke,
  verification, what is still unknown. `docs/blog-outlines/` already holds one such
  outline. Worth writing? It would be **planned**, not shipped, and nothing in the
  site's copy may imply otherwise until it exists. — *Blocks:* nothing; it is
  additive.

**Sub-feature needs (per dispatch rules): none.** A1 is a leaf. Two adjacent
observations are recorded here rather than acted on, because they belong to other
specs:

- `src/models/lab.rs:245` asserts `combined.contains("Network+")`, but the
  certification spine was re-locked on 2026-08-02 to **RHCSA → CCNA → Security+
  with Network+ dropped**. That is a criterion 1D copy-currency issue for **C4**,
  not for the design system. Flagged, not touched.
- `src/handlers/pages.rs:158` asserts `html.contains("CompTIA")` on the home page,
  but the string appears only in the `<meta name="description">` rendered from
  `pages.rs:44` — `templates/index.html` contains no such text. The test reads as
  a claim about the page body and is actually a claim about a meta tag, so
  unrelated copy edits will break it (criterion 5C's named example). Belongs to
  **B1**. Flagged, not touched.
