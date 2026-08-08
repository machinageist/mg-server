# Author: Jeff (agent-drafted)
# Date: 2026-07-19
# Description: Single source of truth for the machinageist theme roster.
#              Emits the CSS token blocks, the JS MODES array + ICON map, and the
#              base.html menu buttons, plus a WCAG contrast report. Keeps the four
#              theme registries from drifting out of sync.
# Notes: run `python3 docs/themes/generate_themes.py` to print all artifacts and
#        the contrast audit; `--css`, `--modes`, `--icons`, `--menu` print one part

import sys

# Font role stacks — no webfonts (CSP is script-src 'self'; all system stacks)
MONO = "ui-monospace, SFMono-Regular, Menlo, Consolas, monospace"
SERIF = 'Charter, "Bitstream Charter", "Sitka Text", Georgia, serif'
ROUND = 'ui-rounded, "SF Pro Rounded", "Segoe UI", system-ui, sans-serif'

# Each theme: the 13 tokens (accent_border is derived as accent + "44"), the
# menu icon + label, color-scheme, body font, and an optional "glow" flag that
# opts the theme into the decorative neon text-shadow (see decorative CSS).
# `default` marks the theme served by the bare :root (no data-theme attribute).
THEMES = [
    # ---- Flagship solar / lunar pair -------------------------------------
    dict(slug="lunarcore", label="Lunarcore", icon="⏾", scheme="dark",
         font=MONO, default=True,
         bg="#060a14", surface="#0d1526", text="#e6f0ff",
         muted="#9fb2cc", faint="#6b7f9e", border="#24344d", border_subtle="#141f30",
         accent="#7fe3ff", accent_hover="#a6ecff", code="#7ee787"),
    dict(slug="solarcore", label="Solarcore", icon="✸", scheme="light",
         font=MONO,
         bg="#f4f7ec", surface="#e7efd6", text="#16261a",
         muted="#3c5340", faint="#4f6650", border="#c6d6ac", border_subtle="#dbe6c6",
         accent="#2a6f34", accent_hover="#1f5527", code="#8a4b0a"),
    # ---- Preserved originals (exact values from origin/main) --------------
    dict(slug="dark", label="Dracula", icon="✦", scheme="dark", font=MONO,
         bg="#282a36", surface="#21222c", text="#f8f8f2",
         muted="#a9aec2", faint="#8f96ad", border="#44475a", border_subtle="#383a47",
         accent="#bd93f9", accent_hover="#cba5ff", code="#50fa7b"),
    dict(slug="light", label="Light", icon="☀", scheme="light", font=MONO,
         bg="#fffbeb", surface="#f4f0df", text="#1f1f1f",
         muted="#6c664b", faint="#6c664b", border="#cfcfde", border_subtle="#e4dfcf",
         accent="#644ac9", accent_hover="#4f36ad", code="#14710a"),
    dict(slug="crt", label="CRT", icon="▦", scheme="dark", font=MONO, glow=True,
         bg="#0a0e0a", surface="#0f150f", text="#4afa6e",
         muted="#2fbf50", faint="#279a44", border="#1c4a28", border_subtle="#133619",
         accent="#7dffa0", accent_hover="#b0ffc4", code="#9dff57"),
    dict(slug="amber", label="Amber", icon="◈", scheme="dark", font=MONO, glow=True,
         bg="#100c04", surface="#171106", text="#ffb000",
         muted="#cc8c00", faint="#a8760c", border="#4a3608", border_subtle="#33250a",
         accent="#ffcf5e", accent_hover="#ffe08a", code="#ffe9b0"),
    dict(slug="paper", label="Paper", icon="¶", scheme="light", font=SERIF,
         bg="#f4f1e8", surface="#ebe7da", text="#1c1a15",
         muted="#4f4a3d", faint="#6e6759", border="#ccc5b2", border_subtle="#ddd7c6",
         accent="#8a2f13", accent_hover="#6b2410", code="#2f5d3a"),
    dict(slug="dawn", label="Dawn", icon="◒", scheme="light", font=SERIF,
         bg="#fff7f1", surface="#f5e9e1", text="#3b302f",
         muted="#6f5a56", faint="#75615d", border="#dec9bf", border_subtle="#eadbd3",
         accent="#9a4f72", accent_hover="#7d3b59", code="#486b58"),
    dict(slug="cloud", label="Cloud", icon="☁", scheme="light", font=ROUND,
         bg="#f3f6fb", surface="#e7edf5", text="#273243",
         muted="#56657a", faint="#607087", border="#cad4e2", border_subtle="#dce3ec",
         accent="#566cab", accent_hover="#43568f", code="#3c6e71"),
    # ---- Retro computing --------------------------------------------------
    dict(slug="gameboy", label="Game Boy", icon="▣", scheme="dark", font=MONO,
         bg="#0f2818", surface="#163420", text="#9bbc0f",
         muted="#87a80e", faint="#6e8a0c", border="#306230", border_subtle="#22461f",
         accent="#cfe36a", accent_hover="#e2f08c", code="#b8d24a"),
    dict(slug="c64", label="Commodore", icon="▩", scheme="dark", font=MONO,
         bg="#1f1860", surface="#2b227e", text="#c3b9ff",
         muted="#9d90ee", faint="#8073d6", border="#4a3fa0", border_subtle="#362c85",
         accent="#8be9ff", accent_hover="#b3f1ff", code="#a6ff9c"),
    dict(slug="teletext", label="Teletext", icon="⌗", scheme="dark", font=MONO,
         bg="#000000", surface="#101010", text="#ffffff",
         muted="#cfcfcf", faint="#9a9a9a", border="#333333", border_subtle="#1c1c1c",
         accent="#ffe800", accent_hover="#fff45c", code="#22ff9a"),
    dict(slug="nes", label="8-Bit", icon="✜", scheme="dark", font=MONO,
         bg="#0d0d17", surface="#17172b", text="#e8e8f4",
         muted="#a6a6c8", faint="#7c7cac", border="#38386a", border_subtle="#202040",
         accent="#6ea2ff", accent_hover="#97bcff", code="#84d05a"),
    dict(slug="matrix", label="Matrix", icon="⌁", scheme="dark", font=MONO, glow=True,
         bg="#000500", surface="#040c04", text="#3bff72",
         muted="#26c651", faint="#199a3c", border="#0f4a1e", border_subtle="#082e13",
         accent="#86ffab", accent_hover="#b8ffce", code="#c2ff64"),
    # ---- Retro dev editors -----------------------------------------------
    dict(slug="solarized", label="Solarized", icon="◉", scheme="dark", font=MONO,
         bg="#002b36", surface="#073642", text="#93a1a1",
         muted="#83949a", faint="#6d8087", border="#0a4b5a", border_subtle="#073642",
         accent="#2aa198", accent_hover="#4cc2b9", code="#859900"),
    dict(slug="nord", label="Nord", icon="❄", scheme="dark", font=MONO,
         bg="#2e3440", surface="#3b4252", text="#eceff4",
         muted="#d8dee9", faint="#aab3c5", border="#4c566a", border_subtle="#434c5e",
         accent="#88c0d0", accent_hover="#a6d3df", code="#a3be8c"),
    dict(slug="gruvbox", label="Gruvbox", icon="◆", scheme="dark", font=MONO,
         bg="#282828", surface="#32302f", text="#ebdbb2",
         muted="#c4b393", faint="#a89984", border="#504945", border_subtle="#3c3836",
         accent="#fabd2f", accent_hover="#fed26b", code="#b8bb26"),
    # ---- Future / neon ----------------------------------------------------
    dict(slug="synthwave", label="Synthwave", icon="▹", scheme="dark", font=MONO, glow=True,
         bg="#1a0b2e", surface="#241040", text="#f5e6ff",
         muted="#c79ce4", faint="#a06fc4", border="#4a2a75", border_subtle="#301a52",
         accent="#ff5fa8", accent_hover="#ff8cc2", code="#36f9f6"),
    dict(slug="vaporwave", label="Vaporwave", icon="▧", scheme="dark", font=MONO, glow=True,
         bg="#241734", surface="#2f2044", text="#ffe6f7",
         muted="#dbabe6", faint="#b17fc9", border="#4d3568", border_subtle="#362548",
         accent="#3fd8ff", accent_hover="#77e6ff", code="#ff8ad8"),
    dict(slug="cyberpunk", label="Cyberpunk", icon="⌖", scheme="dark", font=MONO, glow=True,
         bg="#0a0a0f", surface="#13131f", text="#f0f0ff",
         muted="#b2b2d4", faint="#7c7ca6", border="#2a2a45", border_subtle="#18182a",
         accent="#fcee0a", accent_hover="#fff45c", code="#00f0ff"),
    dict(slug="tron", label="Tron", icon="⊞", scheme="dark", font=MONO, glow=True,
         bg="#030a0f", surface="#08161f", text="#d6f4ff",
         muted="#8fc9e0", faint="#5f9bb6", border="#124a5e", border_subtle="#0a2f3d",
         accent="#6ee2ff", accent_hover="#a2eeff", code="#ffa23e"),
    dict(slug="blueprint", label="Blueprint", icon="⊟", scheme="dark", font=MONO,
         bg="#0d3b66", surface="#124a7d", text="#eaf3ff",
         muted="#bcd6ef", faint="#94b8dd", border="#2f6ba0", border_subtle="#1a5286",
         accent="#8bd8ff", accent_hover="#b3e6ff", code="#d3eaff"),
    # ---- Material / vibe --------------------------------------------------
    dict(slug="sepia", label="Steampunk", icon="⚙", scheme="dark", font=SERIF,
         bg="#241a10", surface="#2f2415", text="#ecd9b0",
         muted="#c6ac7c", faint="#a58e60", border="#574227", border_subtle="#3a2c19",
         accent="#e0b154", accent_hover="#eec87e", code="#aeb87e"),
]


# ---- WCAG contrast helpers ------------------------------------------------
def _lin(c):
    c = c / 255.0
    return c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4


def _lum(hexstr):
    h = hexstr.lstrip("#")
    r, g, b = (int(h[i:i + 2], 16) for i in (0, 2, 4))
    return 0.2126 * _lin(r) + 0.7152 * _lin(g) + 0.0722 * _lin(b)


def contrast(fg, bg):
    a, b = _lum(fg), _lum(bg)
    hi, lo = max(a, b), min(a, b)
    return (hi + 0.05) / (lo + 0.05)


# Where each token is actually rendered, and against what.
#
# The old audit checked five tokens against --bg only, and held faint to the
# AA-large 3.0 threshold. Both were too generous: every one of these tokens also
# renders on --surface (cards, the theme menu, code spans, sidebars), and the
# smallest size each is rendered at is well under the 18.66px that AA-large
# permits. So every text token is held to 4.5 against both backgrounds.
#
# token          backgrounds        smallest rendered size   threshold
USAGE = [
    ("text",         ("bg", "surface"), "0.85rem", 4.5),
    ("muted",        ("bg", "surface"), "0.78rem", 4.5),
    ("faint",        ("bg", "surface"), "0.70rem", 4.5),
    ("accent",       ("bg", "surface"), "0.72rem", 4.5),
    ("accent_hover", ("bg", "surface"), "0.80rem", 4.5),
    ("code",         ("bg", "surface"), "0.90rem", 4.5),
]


# Measure every (token, background) pair in USAGE across every theme
# Returns printable rows plus the list of failures, so callers decide whether a
# failure is advisory or fatal
def audit():
    rows, failures = [], []
    for t in THEMES:
        parts = []
        for token, backgrounds, size, need in USAGE:
            for bg_name in backgrounds:
                ratio = contrast(t[token], t[bg_name])
                ok = ratio >= need
                if not ok:
                    failures.append((t["slug"], token, bg_name, ratio, need, size))
                parts.append(f"{token}/{bg_name} {ratio:4.1f}{'' if ok else ' !!'}")
        rows.append(f"  {t['slug']:11s} " + "  ".join(parts))
    return rows, failures


# Token key -> the CSS custom property it emits as, so failures name the
# variable an author would actually go and edit
CSS_VAR = {
    "bg": "--bg", "surface": "--surface", "text": "--text",
    "muted": "--text-muted", "faint": "--text-faint",
    "accent": "--accent", "accent_hover": "--accent-hover", "code": "--code",
}


# Render the failure list as one line per failure, worst ratio first
def format_failures(failures):
    lines = []
    for slug, token, bg_name, ratio, need, size in sorted(failures, key=lambda f: f[3]):
        lines.append(
            f"  {slug:11s} {CSS_VAR[token]} on {CSS_VAR[bg_name]}: "
            f"{ratio:.2f}:1 (needs {need} at {size})"
        )
    return "\n".join(lines)


# ---- Emitters -------------------------------------------------------------
def _block(t, selector):
    return f"""{selector} {{
    --bg:            {t['bg']};
    --surface:       {t['surface']};
    --text:          {t['text']};
    --text-muted:    {t['muted']};
    --text-faint:    {t['faint']};
    --border:        {t['border']};
    --border-subtle: {t['border_subtle']};
    --accent:        {t['accent']};
    --accent-hover:  {t['accent_hover']};
    --accent-border: {t['accent']}44;
    --code:          {t['code']};
    --shadow:        0 8px 24px rgba(0, 0, 0, {'0.5' if t['scheme'] == 'dark' else '0.12'});
    --font-body:     {t['font']};
    --font-mono:     {MONO};
    color-scheme:    {t['scheme']};
}}"""


def emit_css():
    out = []
    default = next(t for t in THEMES if t.get("default"))
    out.append("/* Default palette (bare :root) — flagship Lunarcore. */")
    out.append(_block(default, ":root, :root[data-theme=\"lunarcore\"]"))
    out.append("")
    out.append("/* All other themes redefine every token so no page is half-themed. */")
    for t in THEMES:
        if t.get("default"):
            continue
        out.append(_block(t, f':root[data-theme="{t["slug"]}"]'))
        out.append("")
    # decorative neon/phosphor glow for opted-in dark themes
    glow = [t["slug"] for t in THEMES if t.get("glow")]
    sel = ",\n".join(f':root[data-theme="{s}"] body' for s in glow)
    out.append("/* Neon/phosphor text-glow — opt-in vibe themes only; the default")
    out.append("   (Lunarcore) keeps body text crisp for long-form reading. */")
    out.append(f"{sel} {{\n    text-shadow: 0 0 6px var(--accent-border);\n}}")
    out.append("")
    out.append("/* CRT scanline overlay — fixed pure-CSS, clicks pass through. */")
    out.append(""":root[data-theme="crt"] body::after {
    content:        "";
    position:       fixed;
    inset:          0;
    pointer-events: none;
    z-index:        9999;
    background:     repeating-linear-gradient(
        to bottom,
        rgba(0, 0, 0, 0.16) 0px,
        rgba(0, 0, 0, 0.16) 1px,
        transparent 1px,
        transparent 3px
    );
}""")
    out.append("")
    out.append("/* Reduced motion: drop the scanline texture and every glow. */")
    reduced = ",\n".join(f':root[data-theme="{s}"] body' for s in glow)
    out.append(f"""@media (prefers-reduced-motion: reduce) {{
    :root[data-theme="crt"] body::after {{ display: none; }}
{reduced} {{ text-shadow: none; }}
}}""")
    return "\n".join(out)


def emit_modes():
    slugs = ["system"] + [t["slug"] for t in THEMES]
    return "var MODES = [" + ", ".join(f'"{s}"' for s in slugs) + "];"


def emit_icons():
    pairs = ['system: "\\u25d0"'] + [f'{t["slug"]}: "{t["icon"]}"' for t in THEMES]
    # render icons literally (not escaped) for readability in the JS file
    pairs = ['system: "◐"'] + [f'{t["slug"]}: "{t["icon"]}"' for t in THEMES]
    return "var ICON = { " + ", ".join(pairs) + " };"


# Menu grouping — display order of the theme picker, which is deliberately not
# THEMES order. "system" is not a theme (it follows the OS) so it is listed here
# rather than in THEMES. Every theme slug must appear in exactly one group;
# emit_menu raises if the roster and the grouping drift apart.
MENU_GROUPS = [
    ("Core", "Core", ["system", "lunarcore", "solarcore"]),
    ("Editor", "Editor", ["dark", "solarized", "nord", "gruvbox"]),
    ("Terminal", "Terminal", ["crt", "amber", "matrix", "teletext"]),
    ("Retro", "Retro", ["gameboy", "c64", "nes"]),
    ("Neon", "Neon", ["synthwave", "vaporwave", "cyberpunk", "tron"]),
    ("Light and print", "Light &amp; print",
     ["light", "paper", "dawn", "cloud", "blueprint", "sepia"]),
]


def emit_menu():
    by_slug = {t["slug"]: t for t in THEMES}
    by_slug["system"] = dict(slug="system", label="System", icon="◐")

    # Drift guard — a theme added to THEMES but not grouped would silently vanish
    # from the menu, and a stale slug here would emit a button with no palette
    grouped = [slug for _, _, slugs in MENU_GROUPS for slug in slugs]
    missing = [s for s in by_slug if s not in grouped]
    unknown = [s for s in grouped if s not in by_slug]
    if missing or unknown:
        raise SystemExit(
            f"MENU_GROUPS out of sync with THEMES — ungrouped: {missing}, unknown: {unknown}"
        )

    rows = []
    for aria_label, visible_label, slugs in MENU_GROUPS:
        rows.append(f'<div class="theme-group" role="group" aria-label="{aria_label}">')
        rows.append(f'  <span class="theme-group-label" aria-hidden="true">{visible_label}</span>')
        for slug in slugs:
            t = by_slug[slug]
            rows.append(
                f'  <button type="button" role="menuitemradio" data-mode="{t["slug"]}" '
                f'aria-checked="false"><span aria-hidden="true">{t["icon"]}</span> '
                f'{t["label"]}</button>'
            )
        rows.append('</div>')
    return "\n".join(rows)


if __name__ == "__main__":
    arg = sys.argv[1] if len(sys.argv) > 1 else "all"
    if arg in ("--css", "all"):
        print(emit_css() if arg == "--css" else "===== CSS =====\n" + emit_css())
    if arg in ("--modes", "all"):
        print(("\n===== MODES =====\n" if arg == "all" else "") + emit_modes())
    if arg in ("--icons", "all"):
        print(("\n===== ICON =====\n" if arg == "all" else "") + emit_icons())
    if arg in ("--menu", "all"):
        print(("\n===== MENU =====\n" if arg == "all" else "") + emit_menu())
    if arg == "all":
        rows, failures = audit()
        print("\n===== CONTRAST AUDIT (>= threshold) =====")
        print("\n".join(rows))
        print(f"\nfailures: {len(failures)}  (themes: {len(THEMES)})")
    # --check is the CI-facing form: failures only, and a non-zero exit so the
    # gate can fail on them. Plain `audit` stays advisory and always exits 0.
    if arg == "--check":
        _, failures = audit()
        if not failures:
            print(f"contrast: all pairs clear across {len(THEMES)} themes")
            sys.exit(0)
        print(f"contrast: {len(failures)} failing pairs across {len(THEMES)} themes\n")
        print(format_failures(failures))
        sys.exit(1)
