# machinageist theme roster

The site ships a family of **23 themes** spanning tech eras and vibes, selectable
from the header menu. Themes are pure client-side CSS — no server involvement.

## How it works

- Active theme = `data-theme="<slug>"` on `<html>`. The bare `:root` (no attribute)
  is the flagship **Lunarcore** so no-JS and pre-paint both render it.
- `static/js/theme-init.js` (blocking `<head>` script, external because CSP is
  `script-src 'self'`) applies the stored theme before first paint.
- `static/js/main.js` drives the menu, persists the choice to `localStorage["theme"]`,
  and keeps `system` following the OS live.
- `system` resolves to the **Solarcore / Lunarcore** flagship pair:
  `prefers-color-scheme: light → solarcore`, else `lunarcore`.
- Each theme redefines the full **13-token contract** (`--bg --surface --text
  --text-muted --text-faint --border --border-subtle --accent --accent-hover
  --accent-border --code --shadow` + `--font-body --font-mono` + `color-scheme`).

## The roster

Flagship pair: **Lunarcore** (default, dark — night solarpunk), **Solarcore** (light
day). Preserved originals: Dracula, Light, CRT, Amber, Paper, Dawn, Cloud. Retro
computing: Game Boy, Commodore, Teletext, 8-Bit, Matrix. Dev editors: Solarized,
Nord, Gruvbox. Future/neon: Synthwave, Vaporwave, Cyberpunk, Tron, Blueprint.
Material: Steampunk.

Opt-in "vibe" themes (CRT, Amber, Matrix, Synthwave, Vaporwave, Cyberpunk, Tron)
carry a decorative neon/phosphor text-glow; CRT adds a scanline overlay. All of it
is behind `prefers-reduced-motion: reduce`. The default (Lunarcore) keeps body
text crisp — no glow — for long-form reading.

The **brand mark** in the header is inline SVG that recolors per theme: the gear
and brain split between `--accent` and `--code`, so every theme gets a coherent
two-tone emblem.

## Editing / adding a theme

`docs/themes/generate_themes.py` is the single source of truth. It holds every
palette as data, validates WCAG-AA contrast for all themes, and emits the CSS
blocks, the JS `MODES`/`ICON` registries, and the menu HTML — so the four
registries can't drift.

```bash
python3 docs/themes/generate_themes.py            # all artifacts + contrast audit
python3 docs/themes/generate_themes.py --css      # just the CSS token section
python3 docs/themes/generate_themes.py --modes    # MODES array
python3 docs/themes/generate_themes.py --icons    # ICON map
python3 docs/themes/generate_themes.py --menu     # menu <button>s
```

To add a theme: append a dict to `THEMES`, run the audit (must show `failures: 0`),
then splice the four outputs into `static/css/style.css` (the generated block
between the "Theme roster — generated" header and the "Base — font role" section),
`static/js/theme-init.js` + `static/js/main.js` (`MODES`, and `ICON` in main.js),
and the `#theme-menu` in `templates/base.html`. Bump the `?v=` cache-bust on the
three versioned assets in `base.html`.

The brand-mark geometry comes from `docs/solarcore/generate_brand.py`; the
`docs/solarcore/SOLARCORE_SPEC.md` holds the original design philosophy that
seeded the Lunarcore/Solarcore direction.
