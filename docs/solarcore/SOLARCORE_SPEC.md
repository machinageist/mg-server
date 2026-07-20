# SOLARCORE — machinageist.dev Theme Spec

Long-lived feature spec for the Solarcore visual identity of machinageist.dev.
Change-level `SPEC.md`/`PLAN.md` files under `AGENTS/changes/` reference this doc;
when shipped behavior diverges, update this file.

Reference art: `docs/solarcore/reference/` —
`solarcore-sample-01.png` (dashboard concept), `solarcore-sample-02.png` (web kit +
palette), `solarcore-sample-03.png` (brand badge + terminal windows),
`golden-gate-cyberpunk.png` / `golden-gate-solarcore.png` (the converging-futures pair).

---

## 1. Philosophy

**Solarcore** is a fork of the solarpunk ethos with the pastoral sanded off and the
sound system turned up. Solarpunk's canonical imagery is daylight: greenhouses,
art-nouveau trellises, communal gardens. Solarcore is the same future at night —
burners and ravers instead of hippies and new agers. Desert-camp infrastructure,
DIY power grids, LED vines on truss steel, mutual aid running over mesh networks.
The two cultures share the load-bearing values: decentralization, reciprocity with
living systems, technology as an instrument of abundance rather than extraction.
Solarcore hints at that resonance constantly — the vine wrapping the gear — while
keeping the aesthetic register nocturnal, electronic, and precise.

The **grit is attitude, not decay.** No rust, no broken glass, no post-apocalyptic
dust layers. Surfaces stay pristine and premium — the grit lives in the voice
(terminal chatter, uppercase system readouts, status lines), in the palette
(night-neon instead of daylight pastel), and in the energy (rave, not retreat).
Think of a festival sound rig at 3 AM: immaculately cabled, glowing, alive, covered
in vines somebody trained up the truss on purpose.

**Symbiotic optimization** is the visual thesis: high-performance systems and
biology running in deliberate harmony. Machine elements (gears, grids, wireframes,
data traces) are geometric and crisp. Organic elements (vines, leaves, growth
curves) are luminous and flowing — and they are *woven into* the machine's
structure, never overgrowing or breaking it. The golden-gate image pair is the
canonical statement: same bridge, two futures — one surveilled and extractive, one
photosynthetic and shared. Solarcore is the second one.

For this site specifically: machinageist.dev is a systems programmer's portfolio,
blog, and tool wiki. The theme must read as *the workstation of someone building
that future* — a premium dark developer environment — not as concept art. Reading
comfort wins every conflict with spectacle.

## 2. Design Principles (invariants)

1. **Reading first.** The site is mostly long-form text (blog posts, ~90 wiki
   pages). Body copy is never neon, never glowing, never uppercase. Spectacle is
   budgeted to chrome: header, hero, brand mark, section labels, footer.
2. **Glow budget.** `text-shadow`/`box-shadow` glow appears ONLY on: the brand
   mark, the hero h1, active/hover nav states, interactive hover/focus states, and
   panel accent borders. Never on body text, never on more than ~3 elements in a
   viewport at rest.
3. **Three neons, three jobs.** Cyan = interactive/data. Magenta = identity/
   structure. Green = organic/growth — the scarcest color on any page (see §5).
4. **Pristine surfaces.** Solid fills, 1px crisp borders, thin vector traces. No
   heavy gradients, lens flares, noise textures, or photographic layers.
5. **Zero new dependencies.** System monospace stack (already in place), one
   hand-written `style.css`, inline/static SVG for all art. No webfonts, no CSS
   frameworks, no JS required for the theme.
6. **Motion is optional.** Any pulse/flicker/grow animation sits behind
   `prefers-reduced-motion: no-preference` and defaults to subtle.
7. **Accessible by construction.** Every text/background pair meets WCAG AA at its
   usage size (ratios documented in §9).

## 3. Design Tokens

All colors become CSS custom properties on `:root`. No raw hex below the token
block — the 575-line stylesheet migrates to token references (see §8 for the
Dracula → Solarcore map).

```css
:root {
    /* Canvas + surfaces */
    --sc-void:          #010915;   /* page background — deep space navy      */
    --sc-surface:       #021226;   /* panels, code blocks, terminal windows  */
    --sc-surface-hi:    #062038;   /* hover fill, raised chrome              */
    --sc-line:          rgba(191, 239, 255, 0.14);  /* default 1px borders   */
    --sc-line-faint:    rgba(191, 239, 255, 0.07);  /* dividers, hr          */
    --sc-grid:          rgba(191, 239, 255, 0.025); /* background grid       */

    /* Neon triad */
    --sc-cyan:          #bfefff;   /* Geist Cyan — links, data, active nav   */
    --sc-cyan-mid:      #46c8f0;   /* traces, chart lines, hover accents     */
    --sc-magenta:       #e23a9a;   /* Machina Magenta — headings, identity   */
    --sc-magenta-hi:    #ff6ec2;   /* magenta at small sizes, link hover     */
    --sc-vine:          #68be20;   /* Vine Green — organic accents, code,    */
                                   /* success states ONLY                    */
    --sc-vine-hi:       #8ff53f;   /* vine glow cores, sparingly             */

    /* Text ramp */
    --sc-text:          #e8f4ff;   /* h1, strong, high-emphasis              */
    --sc-text-body:     #8fa8c8;   /* body copy                              */
    --sc-text-faint:    #56719a;   /* metadata, dates, footer                */

    /* Glow (the entire glow budget lives in these three) */
    --sc-glow-cyan:     0 0 12px rgba(70, 200, 240, 0.35);
    --sc-glow-magenta:  0 0 12px rgba(226, 58, 154, 0.35);
    --sc-glow-vine:     0 0 10px rgba(104, 190, 32, 0.35);
}
```

Note: `solarcore-sample-02.png` renders Vine Green's hex as `#B88E20` — that is an
AI-raster typo. Canonical Vine Green is `#68be20`.

## 4. Typography

- **Stack (unchanged):** `ui-monospace, SFMono-Regular, Menlo, Consolas, monospace`.
  Monospace-everywhere is already the site's voice and matches the concept art.
- **h1:** `--sc-text`, 1.6–1.75rem, weight 700. On the index hero only, uppercase
  with `letter-spacing: 0.06em` and `--sc-glow-cyan` — the one glowing heading.
- **h2 (section labels):** `--sc-magenta`, uppercase, `letter-spacing: 0.1em`,
  0.85rem — the existing pattern, recolored. This is the primary "magenta as
  structure" surface.
- **h2 inside `.post-content`:** stays sentence-case and larger (existing rule),
  color `--sc-magenta-hi` (small-size-safe magenta), no glow.
- **Body:** `--sc-text-body`, 15px, line-height 1.7. Never uppercase.
- **System-readout microcopy** (footer status line, wiki breadcrumbs, meta rows):
  uppercase, 0.72–0.8rem, `letter-spacing: 0.08em`, `--sc-text-faint` or
  `--sc-cyan` for live values. This is where the terminal-chatter grit lives.

## 5. Color Semantics

| Color | Meaning | Used for | Never used for |
|---|---|---|---|
| Cyan | interactive / data / signal | links, active nav, hover, chart traces, live values, focus rings | headings, body text |
| Magenta | identity / structure / neural | brand, h2 labels, post-content h2, active wiki border, primary buttons | success/error states |
| Green | organic / growth / alive | inline code, vine flourishes, "active" project status, success readouts | links, borders at rest, large fills |
| Text ramp | prose | everything readable | — |

Green scarcity rule: on any given page, green appears in at most inline code +
one status/flourish element. If a page has lots of code (wiki pages), the vine
flourishes drop out — code green *is* the organic presence there.

## 6. Motifs

### 6.1 Grid canvas
`body` gets a fixed 48px vector grid via two `repeating-linear-gradient`s in
`--sc-grid` (≈2.5% opacity cyan). It reads as workspace, disappears under text.
No masks, no parallax.

### 6.2 Panels
The site keeps its current divider-list architecture — no card-ifying the blog or
wiki lists. Panelization applies to exactly two surfaces:
- **Hero + "Currently building"** on the index: 1px `--sc-line` border,
  `--sc-surface` fill, and a 2px top accent border in `--sc-cyan-mid`.
- **Code blocks / `pre`:** become terminal windows — `--sc-surface` fill, 1px
  `--sc-line` border, and a slim title strip carrying three 6px dots in the neon
  triad (the sample-03 window chrome, minimized). Phase-5 flourish; plain
  bordered blocks are the Phase-3 baseline.

List rows (`.post-item`, `.project-card`, `.release-item`) keep border-bottom
dividers in `--sc-line-faint`; on hover they gain a 2px cyan left edge — a data
trace lighting up, not a card.

### 6.3 Vine micro-traces
One hand-drawn SVG asset (`static/img/vine-trace.svg`): a thin 1.5px-stroke vine
with 3–4 leaves, `--sc-vine` at 50% opacity. Usage: creeping along the header's
bottom border from the left corner (behind the nav, ~200px long), and optionally
one corner sprig on the index hero panel. That is the entire site-chrome vine
budget — restraint is what keeps it premium instead of decorative.

### 6.4 Footer status line
The footer becomes a system readout (§4 microcopy style):

```
SYS: MACHINAGEIST.DEV │ OPERATOR: JEFF CINCOSKI │ STATUS: ONLINE │ UPDATED: 2026-05-22 │ SRC ↗
```

Faint at rest; `STATUS: ONLINE` value in `--sc-vine` (its one green allowance).

## 7. Brand Mark

Hand-built SVG (never the AI rasters — they are mood references only), three
nested layers per sample-01/03:

1. **Gear** — 12-tooth industrial cog, 2px stroke, alternating cyan/magenta arc
   segments (cyan left hemisphere, magenta right, hard split at vertical center).
2. **Brain** — low-poly wireframe brain inside the gear's inner ring, 1px strokes,
   left lobe cyan / right lobe magenta (machine-mind duality).
3. **Vines** — two thin green vines emerging from gear notches (~10 and ~4
   o'clock), wrapping a short arc along the outer ring. Omitted below 32px.

Artifacts:
- `static/img/mark.svg` — full mark. Used in the header at 28px (vines omitted at
  this size via a `<g>` toggle or a `mark-sm.svg` variant) and inline in the index
  hero at ~160px with vines, beside the hero copy (Phase 4).
- `favicon.svg` — gear + brain only.
- `static/img/og-card.png` — 1200×630 social card: mark + wordmark on `--sc-void`
  with grid; wired into the existing `og:` meta in `base.html`.

**Wordmark:** `MACHINAGEIST` — uppercase, `letter-spacing: 0.08em`, split-color
per sample-02: `MACHINA` in `--sc-magenta-hi`, `GEIST` in `--sc-cyan`. Replaces
the current lowercase purple `.brand` text, sits right of the mark. `SOLARCORE`
itself is the design language's name, not a site wordmark — it may appear in the
footer readout (`THEME: SOLARCORE`) but nowhere else in chrome.

## 8. Component Migration Map (Dracula → Solarcore)

The current stylesheet uses seven Dracula values. Mechanical mapping:

| Current hex | Where | Becomes |
|---|---|---|
| `#282a36` | body bg | `--sc-void` |
| `#21222c` | code bg, tags | `--sc-surface` |
| `#44475a` | borders, faint text | `--sc-line` (borders) / `--sc-text-faint` (text) |
| `#383a47` | list dividers | `--sc-line-faint` |
| `#6272a4` | body/muted text | `--sc-text-body` |
| `#f8f8f2` | bright text | `--sc-text` |
| `#bd93f9` / `#cba5ff` | links, h2, brand, active states | links → `--sc-cyan` (hover `--sc-cyan-mid` + glow); h2 → `--sc-magenta`; brand → wordmark (§7) |
| `#50fa7b` | inline code | `--sc-vine` |

Per-component decisions that aren't mechanical:

- **Nav:** `.nav-link` rest `--sc-text-faint`, hover `--sc-text`; `.is-active`
  `--sc-cyan` with a 1px glowing underline (`--sc-glow-cyan`).
- **Hero actions:** arrow links in `--sc-cyan`; hover shifts to `--sc-cyan-mid` +
  glow. No button-ification — the site's link idiom stays.
- **Status pills (`.project-status`):** active → `--sc-vine` text/border (alive);
  in-progress → `--sc-cyan`; complete → `--sc-text-faint`. Pills get
  `--sc-surface` fill.
- **Tags (`.tag`):** `--sc-surface` fill, `--sc-line` border, `--sc-text-faint`
  text — quiet.
- **Wiki sidebar:** section h2s stay faint; `li.active a` gets `--sc-magenta`
  text + 2px magenta left border (structure, not signal — the reader's position
  in the architecture). Hover stays cyan-less: `--sc-text`.
- **Blockquote:** left border `--sc-magenta` at 40% opacity, text `--sc-text-faint`.
- **Error pages:** giant status code in `--sc-magenta` with glow — the one place
  spectacle is free — plus a readout line: `SYS_FAULT: ROUTE_NOT_FOUND`.
- **Inline style purge:** `index.html:22` has `style="color: #c8c8c8;"` — replace
  with a class on migration.

## 9. Accessibility & Performance Invariants

Contrast vs `--sc-void`/`--sc-surface` (both L ≈ 0.003–0.006, ratios effectively
identical):

| Token | Ratio | Cleared for |
|---|---|---|
| `--sc-text` `#e8f4ff` | ≈ 17:1 | everything |
| `--sc-text-body` `#8fa8c8` | ≈ 8.2:1 | body copy (AAA) |
| `--sc-text-faint` `#56719a` | ≈ 4.0:1 | metadata/large only — never body copy |
| `--sc-cyan` `#bfefff` | ≈ 15:1 | links at any size |
| `--sc-magenta` `#e23a9a` | ≈ 5.0:1 | headings/bold ≥0.85rem uppercase; not small prose |
| `--sc-magenta-hi` `#ff6ec2` | ≈ 7.9:1 | magenta at small sizes |
| `--sc-vine` `#68be20` | ≈ 8.5:1 | inline code, status text |

- Focus states: 2px `--sc-cyan-mid` outline with offset — never removed, glow
  optional on top.
- Don't rely on hue alone: status pills keep text labels; links keep underlines
  in prose contexts.
- Perf budget unchanged: one CSS file, no JS, SVGs < 5KB each, grid/glow are pure
  CSS. Target: no measurable change to current page weight beyond the SVGs.

## 10. Anti-Goals

- No dashboard cosplay: no fake metrics, uptime widgets, toggle switches, or
  chart chrome on a content site. The concept art's data-viz panels apply only if
  a page ever has real data to show.
- No decay: rust, glitch-text, scanlines, CRT filters are cyberpunk's half of the
  golden-gate pair — not ours.
- No animation of body content; no autoplaying motion anywhere.
- No lightening/theming toggle for now — Solarcore is a night theme; a "daylight
  solarpunk" variant is a future idea, not scope.

## 11. Resolved Decisions (Jeff, 2026-07-19)

1. **Wordmark** — uppercase split-color `MACHINAGEIST` (MACHINA magenta / GEIST
   cyan), per §7.
2. **Hero art** — the full badge mark (with vines) ships in the index hero as an
   inline SVG beside the hero copy as soon as Phase 4 lands. Phases 1–3 run with
   the typographic hero in the interim.
3. **Code block chrome** — terminal-window title strip with three neon dots ships
   in Phase 5; plain bordered blocks are the Phase-3 baseline.
4. **SVG authorship** — agent drafts all SVG assets (vine trace, gear-brain mark,
   favicon) against the reference art; Jeff reviews like any other diff.

## 12. Rollout Phases

Each phase is one change under `AGENTS/changes/solarcore-<n>-<slug>/` with its own
SPEC/PLAN, committed and visually verified (`cargo run` + browser pass across
index, a blog post, a wiki page, portfolio, releases, 404) before the next begins.

1. **tokens-shell** — `:root` token block; migrate body, header, footer, links per
   §8 map. Site fully readable in Solarcore palette. No art.
2. **typography-nav** — heading recolors, nav states, footer status line, focus
   states, hero uppercase treatment.
3. **components** — lists, pills, tags, code blocks (plain bordered), wiki
   sidebar, blockquotes, error pages, inline-style purge.
4. **brand** — mark SVG, favicon, wordmark, hero badge placement, og-card +
   meta wiring.
5. **flourishes** — grid canvas, vine traces, terminal window chrome, hover
   glow polish, reduced-motion guards.

Phases 1–3 are pure CSS/template edits and independently shippable; the site never
half-renders between phases.
