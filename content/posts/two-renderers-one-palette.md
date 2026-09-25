---
title: "Two Renderers, One Palette"
date: 2026-09-20
summary: "My desktop shell and this website share a theme roster. Keeping them in sync meant defining a token contract, generating every copy from one source, and failing the build on low contrast."
category: "Linux / SysAdmin"
tags: [linux, wayland, quickshell, theming, accessibility, automation]
---

This site has a theme switcher. So does my desktop shell, which is a Quickshell
QML application, not a web page. Twenty-three of the palettes are shared between
them. They use the same hex values, declared once.

Getting there was less about color and more about deciding what a theme is, so
that two unrelated rendering stacks could agree on one.

## The problem with having two of anything

The website had a theme switcher first. When I built the desktop shell I wanted
the same palettes, so I copied the hex values across. That lasted about a week.

Then I adjusted a muted text color on the site because it was too faint in a
screenshot. The desktop kept the old value. Now there were two versions of
Solarcore, and the only way to tell which was right was to remember which one I
had edited last. With twenty-three themes and ten color roles each, that was not
going to hold up.

The copying was fine. The problem was that neither copy was the source.

## Deciding what a theme is

What made this manageable was writing down the contract. A theme is exactly ten
named color tokens and nothing else.

```text
bg              surface         text
textMuted       textFaint
border          borderSubtle
accent          accentHover     code
```

Each theme also has a `dark` flag and a few presentation details: a label, a
group, and a glyph.

Keeping it to ten was a choice. Theme systems tend to offer a full scale: nine
grays, five accent shades, and a named color for every state. I have used
systems like that, and no two components end up picking the same gray. There is
no reason to prefer `grey-600` over `grey-700`, so the choice gets made on the
spot forty different times.

With ten roles, a component asks a question that has one answer. Is this body
text, or is it de-emphasized? Is this a border, or a subtle divider? If a new
component needs an eleventh role, that is a change to the design system, not
something one component decides on its own at 1 a.m.

The small contract also makes porting cheap. A palette from somewhere else, such
as Catppuccin, Kanagawa, or Nord, only has to be mapped onto ten slots. The shell
carries twenty of those alongside the twenty-three that came from here.

## One generator for every copy

On the website side, the roster lives in `docs/themes/generate_themes.py`, and
every copy is generated from it:

- the CSS custom property block for each theme
- the JavaScript array the switcher iterates
- the icon map
- the theme menu markup in the base template

Those used to be four hand-maintained lists, and each one was a chance to forget
something. I had already shipped a theme that worked everywhere except the menu,
because I added it to the other three and forgot the menu.

The shell reads the same roster from `palettes.json`, where each palette has a
`source` field. Twenty-three of them say `mg-server`. That field records where the
authoritative values live, so when I change one, I know which direction the
change has to go.

## The contrast check

Generating every copy keeps them from drifting apart. It does not stop them from
all being wrong in the same way.

One palette I liked had `textFaint` at a contrast ratio of about 2.8:1 against
its background. It looked good. On a laptop screen at an angle, in daylight, I
could not read it.

So the generator computes the WCAG contrast ratio for every text token against
every background it can appear on, and `--check` exits non-zero if any pair falls
below 4.5:1:

```text
$ python3 docs/themes/generate_themes.py --check
contrast: all pairs clear across 23 themes
```

That runs in CI before the Rust build. A theme that is hard to read cannot ship
here, however good it looks in a screenshot.

The check works on pairs of colors. A muted gray is only readable or unreadable
against a specific background, so the generator checks every combination that
can occur on the site. That is how it found problems I had missed by eye.

## What I have not done

The shell does not run this check.

Its twenty imported palettes were designed by other people, mostly for code
editors, where contrast expectations are different and a lot of syntax
highlighting is meant to be low-emphasis. I have not held them to 4.5:1, and some
of them would fail.

I have not decided to leave that gap. I just have not closed it yet. For now, the
website's themes are contrast-checked and the desktop's imported ones are not,
and I know which palettes are on which side.

I also have not automated the direction of flow. If I change a token in
`palettes.json` that came from `mg-server`, nothing stops me. The `source` field
records where the change should have been made, but it does not enforce anything.
The website's four lists can no longer drift from each other. The website and the
shell still can.

## What carried over

Most of this ended up not being about color.

- **Name the roles, not the values.** Ten tokens that each mean something work
  better than forty that are positions on a scale.
- **Generate every copy from one source.** Anything maintained by hand next to a
  generated copy will eventually disagree with it, and you will find out from a
  screenshot.
- **Record where the authoritative copy lives.** The `source` field is one word
  per palette, and it answers "which one is right?" without anyone having to
  remember.
- **Let the build check what you cannot see.** I cannot judge a 4.5:1 ratio by
  eye. The generator checks every pair on every build.
- **Say which parts are not checked.** An audit that covers part of the system is
  still useful, as long as nobody describes it as covering all of it.

The desktop and the site match because there is one list of values and a build
that fails when one of them is wrong.
