---
title: "Two Renderers, One Palette"
date: 2026-09-20
summary: "My desktop shell and this website share a theme roster. Keeping them honest meant defining a token contract, generating every artifact from one source, and failing the build on contrast."
category: "Linux / SysAdmin"
tags: [linux, wayland, quickshell, theming, accessibility, automation]
---

This site has a theme switcher. So does my desktop shell, which is a Quickshell
QML application rather than a web page. Twenty-three of the palettes are the
same palettes — not "similar," the same hex values, declared once.

Getting there was less about colour than about deciding what a theme *is* so
that two unrelated rendering stacks could agree on one.

## The problem with having two of anything

I had a theme switcher on the website first. When I built the desktop shell I
wanted the same palettes, and the obvious approach — copy the hex values across
— lasted about a week.

The failure is boring and total. I adjusted a muted-text colour on the site
because it was too faint in a screenshot. The desktop kept the old value. Now I
had two Solarcores, and no way to tell which was correct except by remembering
which one I had edited last. Multiply by twenty-three themes and thirteen
colour roles each.

Copying is not the problem. Copying without a source is.

## Deciding what a theme is

The thing that made this tractable was writing down the contract: a theme is
exactly thirteen named tokens, and nothing else.

```text
bg              surface         text
textMuted       textFaint
border          borderSubtle
accent          accentHover     code
```

Plus a `dark` boolean and presentation metadata — label, group, glyph.

Thirteen is a deliberate constraint. The temptation with a theme system is to
offer a full scale: nine greys, five accent shades, a semantic colour for every
state. I have used systems like that and the result is that no two components
pick the same grey, because there is no reason to prefer `grey-600` over
`grey-700` and the choice gets made ad hoc forty times.

With thirteen roles, a component asks a question that has one answer. Is this
body text, or is it de-emphasised? Is this a border, or a subtle divider? If a
new component genuinely needs a fourteenth role, that is a conversation about
the design system, not a decision a component makes alone at 1 a.m.

The constraint also makes porting cheap. A palette from somewhere else —
Catppuccin, Kanagawa, Nord — becomes a mapping exercise onto thirteen slots. The
shell carries twenty of those alongside the twenty-three that came from here.

## One generator, every artifact

On the website side the roster lives in `docs/themes/generate_themes.py`, and
every artifact is emitted from it:

- the CSS custom property block for each theme;
- the JavaScript array the switcher iterates;
- the icon map;
- the theme menu markup in the base template.

Four registries that used to be four opportunities to forget one. I had already
shipped a theme that worked everywhere except the menu, because the menu was
hand-maintained and I added the theme to the other three.

The shell reads the same roster from `palettes.json`, where each palette carries
a `source` field. Twenty-three say `mg-server`. That field is not decoration —
it is the record of where the authoritative values live, so that when I change
one, I know which direction the change has to flow.

## The check that actually earns its place

Generating the artifacts stops them from drifting apart. It does not stop them
from being wrong together.

A palette I liked had `textFaint` at a contrast ratio of about 2.8:1 against its
background. It looked good. On a laptop screen at an angle, in daylight, it was
unreadable — and "unreadable in some conditions" is not a matter of taste, it is
a matter of whether the text is text.

So the generator computes WCAG contrast for every text token against every
background that token can appear on, and `--check` exits non-zero if any pair
falls below 4.5:1:

```text
$ python3 docs/themes/generate_themes.py --check
contrast: all pairs clear across 23 themes
```

That runs in CI ahead of the Rust build. A theme that reads badly cannot ship
here, regardless of how good the screenshot looks.

The important property is that it checks *pairs*, not colours. A muted grey is
not accessible or inaccessible on its own; it is one or the other against a
specific background. Checking every combination that can actually occur is the
only version of this check that means anything, and it is why the audit found
problems that eyeballing had not.

## What I have not done

The shell does not run this audit.

Its twenty imported palettes were designed by other people, mostly for code
editors, where the contrast expectations are different and a lot of syntax
highlighting is deliberately low-emphasis. I have not held them to 4.5:1, and
some of them would fail.

That is a gap, not a decision. The honest statement is that the website's themes
are contrast-audited and the desktop's imported ones are not, and I know which
side of that line each palette sits on.

I also have not automated the direction of flow. If I change a token in
`palettes.json` that came from `mg-server`, nothing stops me — the `source`
field records where it should have changed, but it is documentation, not
enforcement. The website's four registries can no longer drift from each other.
The website and the shell still can.

## What transferred

Most of this was not really about colour.

- **Name the roles, not the values.** Thirteen tokens that mean something beat
  forty that mean a position on a ramp.
- **Generate every copy from one source.** Anything hand-maintained alongside a
  generated thing will eventually disagree with it, and you will find out from a
  screenshot.
- **Record where authority lives.** The `source` field is one word per palette
  and it answers "which one is right?" without anyone having to remember.
- **Make the machine check what taste cannot.** I cannot eyeball a 4.5:1 ratio.
  The generator can, on every pair, every build.
- **Say which parts are not checked.** An audit that covers some of the system
  is useful. An audit described as covering all of it is worse than none.

The desktop and the site look the same not because I am careful, but because
there is one list of numbers and a build that fails when it is wrong. Those are
the only two mechanisms in this that do any work.
