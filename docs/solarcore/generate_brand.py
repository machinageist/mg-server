# Author: Jeff (agent-drafted)
# Date: 2026-07-19
# Description: Generate Solarcore brand SVGs — gear + low-poly brain + vines
# Notes: geometry parametrized so review tweaks are one-line changes

import math

CYAN = "#bfefff"
CYAN_MID = "#46c8f0"
MAGENTA = "#e23a9a"
MAGENTA_HI = "#ff6ec2"
VINE = "#68be20"
VOID = "#010915"

CX, CY = 50.0, 50.0


# Build closed 12-tooth gear path on a 100x100 viewBox
def gear_path(teeth=12, r_outer=46.0, r_root=39.0, tooth_frac=0.42):
    step = 2 * math.pi / teeth
    pts = []
    for i in range(teeth):
        a0 = i * step
        root_end = a0 + step * (1 - tooth_frac) / 2
        tip_start = root_end + step * 0.08
        tip_end = a0 + step - step * (1 - tooth_frac) / 2 - step * 0.08
        fall_end = a0 + step - step * (1 - tooth_frac) / 2 + step * 0.08
        pts.append((r_root, a0))
        pts.append((r_root, root_end))
        pts.append((r_outer, tip_start))
        pts.append((r_outer, tip_end))
        pts.append((r_root, fall_end))
    d = []
    for j, (r, a) in enumerate(pts):
        x = CX + r * math.cos(a)
        y = CY + r * math.sin(a)
        d.append(("M" if j == 0 else "L") + f"{x:.2f},{y:.2f}")
    return " ".join(d) + " Z"


# Left-hemisphere brain outline; right side is a mirror
BRAIN_LEFT = [
    (50, 67), (44, 69), (38, 67), (33, 62), (29, 56), (28, 49),
    (29, 42), (33, 36), (38, 32), (44, 30), (50, 29),
]
BRAIN_NODES_L = [(38, 44), (42, 53), (36, 55)]
BRAIN_EDGES_L = [
    (0, "n1"), (1, "n1"), (2, "n2"), (3, "n2"), (4, "n2"), (5, "n0"),
    (6, "n0"), (7, "n0"), (8, "n0"), (9, "n0"), (10, "n0"),
    ("n0", "n1"), ("n0", "n2"), ("n1", "n2"), (2, "n1"),
]


def mirror(p):
    return (100 - p[0], p[1])


def poly_points(pts):
    return " ".join(f"{x:.1f},{y:.1f}" for x, y in pts)


def brain_group(side_pts, nodes, color, sw):
    def resolve(k):
        if isinstance(k, str):
            return nodes[int(k[1])]
        return side_pts[k]

    lines = []
    for a, b in BRAIN_EDGES_L:
        x1, y1 = resolve(a)
        x2, y2 = resolve(b)
        lines.append(
            f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" '
            f'stroke="{color}" stroke-width="{sw * 0.55:.2f}" opacity="0.75"/>'
        )
    outline = (
        f'<polyline points="{poly_points(side_pts)}" fill="none" '
        f'stroke="{color}" stroke-width="{sw:.2f}" '
        f'stroke-linejoin="round" stroke-linecap="round"/>'
    )
    return outline + "".join(lines)


# One vine: arc-following bezier chain outside the gear with alternating leaves
def vine(a_start_deg, a_end_deg, r, color, sw, leaf_every=3):
    steps = 24
    a0, a1 = math.radians(a_start_deg), math.radians(a_end_deg)
    pts = []
    for i in range(steps + 1):
        t = i / steps
        a = a0 + (a1 - a0) * t
        rr = r + 2.2 * math.sin(t * math.pi * 4)
        pts.append((CX + rr * math.cos(a), CY + rr * math.sin(a)))
    d = f"M{pts[0][0]:.1f},{pts[0][1]:.1f} " + " ".join(
        f"L{x:.1f},{y:.1f}" for x, y in pts[1:]
    )
    out = [
        f'<path d="{d}" fill="none" stroke="{color}" '
        f'stroke-width="{sw:.2f}" stroke-linecap="round"/>'
    ]
    for i in range(2, steps, leaf_every):
        x, y = pts[i]
        a = a0 + (a1 - a0) * (i / steps)
        tilt = math.degrees(a) + (55 if i % 2 else -55)
        out.append(
            f'<ellipse cx="{x:.1f}" cy="{y:.1f}" rx="3.1" ry="1.3" '
            f'fill="{color}" opacity="0.9" '
            f'transform="rotate({tilt:.0f} {x:.1f} {y:.1f})"/>'
        )
    return "".join(out)


def svg_mark(with_vines, sw_gear=2.4, sw_brain=1.5, pad_vines=True):
    g = gear_path()
    vb = "-8 -8 116 116" if (with_vines and pad_vines) else "0 0 100 100"
    parts = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{vb}">']
    parts.append(
        "<defs>"
        '<clipPath id="left"><rect x="-10" y="-10" width="60" height="120"/></clipPath>'
        '<clipPath id="right"><rect x="50" y="-10" width="60" height="120"/></clipPath>'
        "</defs>"
    )
    for clip, color in (("left", CYAN_MID), ("right", MAGENTA)):
        parts.append(
            f'<g clip-path="url(#{clip})">'
            f'<path d="{g}" fill="none" stroke="{color}" '
            f'stroke-width="{sw_gear}" stroke-linejoin="round"/>'
            f'<circle cx="50" cy="50" r="33" fill="none" '
            f'stroke="{color}" stroke-width="{sw_gear * 0.7:.2f}"/>'
            "</g>"
        )
    parts.append(brain_group(BRAIN_LEFT, BRAIN_NODES_L, CYAN, sw_brain))
    right_pts = [mirror(p) for p in BRAIN_LEFT]
    right_nodes = [mirror(p) for p in BRAIN_NODES_L]
    parts.append(brain_group(right_pts, right_nodes, MAGENTA_HI, sw_brain))
    if with_vines:
        # weave along the gear edge (r≈r_outer) so leaves cross the teeth
        parts.append(vine(118, 210, 45.5, VINE, 1.4))
        parts.append(vine(305, 378, 45.5, VINE, 1.4))
    parts.append("</svg>")
    return "".join(parts)


def svg_favicon():
    g = gear_path(teeth=12, r_outer=48.0, r_root=38.0)
    parts = [
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100">',
        f'<rect width="100" height="100" fill="{VOID}"/>',
        "<defs>"
        '<clipPath id="l"><rect x="0" y="0" width="50" height="100"/></clipPath>'
        '<clipPath id="r"><rect x="50" y="0" width="50" height="100"/></clipPath>'
        "</defs>",
    ]
    for clip, color in (("l", CYAN_MID), ("r", MAGENTA)):
        parts.append(
            f'<g clip-path="url(#{clip})">'
            f'<path d="{g}" fill="none" stroke="{color}" stroke-width="5"/>'
            "</g>"
        )
    parts.append(
        f'<polyline points="{poly_points(BRAIN_LEFT)}" fill="none" '
        f'stroke="{CYAN}" stroke-width="4" stroke-linejoin="round"/>'
    )
    parts.append(
        f'<polyline points="{poly_points([mirror(p) for p in BRAIN_LEFT])}" '
        f'fill="none" stroke="{MAGENTA_HI}" stroke-width="4" stroke-linejoin="round"/>'
    )
    parts.append("</svg>")
    return "".join(parts)


def svg_og_card(mark_svg_inner):
    grid = []
    for x in range(0, 1201, 48):
        grid.append(
            f'<line x1="{x}" y1="0" x2="{x}" y2="630" '
            'stroke="rgba(191,239,255,0.035)" stroke-width="1"/>'
        )
    for y in range(0, 631, 48):
        grid.append(
            f'<line x1="0" y1="{y}" x2="1200" y2="{y}" '
            'stroke="rgba(191,239,255,0.035)" stroke-width="1"/>'
        )
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 630">'
        f'<rect width="1200" height="630" fill="{VOID}"/>'
        + "".join(grid)
        + f'<g transform="translate(105,155) scale(3.2)">{mark_svg_inner}</g>'
        + '<text x="500" y="300" font-family="monospace" font-size="76" '
        f'font-weight="bold" letter-spacing="6"><tspan fill="{MAGENTA_HI}">MACHINA</tspan>'
        f'<tspan fill="{CYAN}">GEIST</tspan></text>'
        + f'<text x="503" y="360" font-family="monospace" font-size="28" '
        f'letter-spacing="8" fill="{CYAN_MID}">MACHINAGEIST.DEV</text>'
        + f'<text x="503" y="410" font-family="monospace" font-size="22" '
        f'letter-spacing="4" fill="#56719a">SYSTEMS · SECURITY · RUST</text>'
        + "</svg>"
    )


# Horizontal vine trace for the header baseline — spec §6.3
def svg_vine_trace(w=220, h=26, steps=60):
    pts = []
    for i in range(steps + 1):
        t = i / steps
        pts.append((t * w, 15 + 6 * math.sin(t * math.pi * 3)))
    d = "M%.1f,%.1f " % pts[0] + " ".join("L%.1f,%.1f" % p for p in pts[1:])
    leaves = []
    for i in (10, 22, 34, 46, 54):
        x, y = pts[i]
        tilt = -38 if i % 2 else 38
        leaves.append(
            f'<ellipse cx="{x:.1f}" cy="{y:.1f}" rx="4.6" ry="1.8" '
            f'fill="{VINE}" opacity="0.92" transform="rotate({tilt} {x:.1f} {y:.1f})"/>'
        )
    return (
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}">'
        f'<path d="{d}" fill="none" stroke="{VINE}" stroke-width="1.5" '
        f'stroke-linecap="round"/>' + "".join(leaves) + "</svg>"
    )


import re, sys, os

out_dir = sys.argv[1]
mark = svg_mark(with_vines=True)
mark_sm = svg_mark(with_vines=False)
open(f"{out_dir}/mark.svg", "w").write(mark)
open(f"{out_dir}/mark-sm.svg", "w").write(mark_sm)
open(f"{out_dir}/favicon.svg", "w").write(svg_favicon())
inner = re.sub(r"^<svg[^>]*>|</svg>$", "", mark)
open(f"{out_dir}/og-card.svg", "w").write(svg_og_card(inner))
open(f"{out_dir}/vine-trace.svg", "w").write(svg_vine_trace())
for f in ("mark", "mark-sm", "favicon", "og-card", "vine-trace"):
    print(f, os.path.getsize(f"{out_dir}/{f}.svg"), "bytes")
