---
title: "geistos"
date: 2026-09-20
summary: "A local-first Linux workstation: a Quickshell desktop replacing the usual bar and launcher, a generated palette system shared with this site, and Hyprland configured in Lua."
tags: [linux, wayland, hyprland, quickshell, theming, workstation]
---

## What this is

`geistos` is my daily-driver Linux desktop, kept in a repository so it can be
rebuilt rather than remembered. It is a Wayland workstation: Hyprland as the
compositor, and one Quickshell QML codebase providing the bar, launcher,
notifications, and panels that would otherwise be three or four separate
programs.

It exists because I wanted to stop treating my working environment as something
that accumulates. Configuration that only lives on one machine is configuration
you cannot reason about, and a desktop assembled from six tools that each have
their own config language is hard to change without breaking something you
forgot about.

It is not an operating system and not a distribution, despite the name. It is a
configuration repository and a QML application that runs on Arch.

## The shell

The desktop surface is 153 QML files, about 13,000 lines, under one Quickshell
configuration started from Hyprland:

```text
config/quickshell/mgeist/
  shell.qml    entrypoint: screen variants, resident panels, IPC handlers
  Theme/       the only source of colour — palettes.json plus generated themes
  Bar/         the bar surface and one file per module
  Launcher/    application launch and lexical lookup
  Panels/      quick settings, notifications, clipboard, session, theme picker,
               system monitor, keybindings
  Services/    singletons: stats, backlight, notifications, wallpaper, palette,
               rotation, AI usage, spectrum
  Widgets/     Pill, PopupPanel, BarText, AppIcon, Graph, BarMeter
```

The division that makes it maintainable is `Services/` against everything else.
A service is a singleton that owns one piece of state and no appearance — the
battery reading, the current palette, the notification list. Panels and bar
modules read services and own no state of their own. When a bar module and a
panel both need to show the same thing, they read the same singleton instead of
each polling separately, which is what keeps a status bar from becoming a set of
timers that disagree with each other.

`Widgets/` holds the shared visual primitives. A `Pill` is a pill whether it is
showing battery or the clock, so the bar stays visually consistent without every
module reimplementing padding and hover behaviour.

## Hyprland in Lua

The compositor configuration is Lua rather than Hyprland's own config format:

```text
config/hypr/
  hyprland.lua      keybindings.lua   window_rules.lua
  layout.lua        input.lua         monitors.lua
  look_and_feel.lua animations.lua    programs.lua
  autostart.lua     environment.lua   permissions.lua
```

One file per concern, which matters more than the language does. When I want to
change a window rule I open `window_rules.lua`, not a 600-line file where rules
sit between animation curves and monitor layout.

## The palette system, and the link to this site

The shell carries 43 palettes. Twenty-three of them are marked `"source":
"mg-server"` in `palettes.json`, because they *are* the themes this website
ships — same token names, same hex values, rendered by two completely different
stacks. The site resolves them to CSS custom properties; the shell resolves them
to QML properties. Switching the site to Solarcore and switching the desktop to
Solarcore produce the same colours because they are reading the same numbers.

The remaining twenty are ports of themes other people designed — Catppuccin,
Tokyo Night, Rosé Pine, Everforest, Kanagawa, Monokai, Ayu, Nord, Zenburn — plus
five of my own. Each palette declares the same thirteen tokens, so a panel
written against `Theme.accent` works under all forty-three without knowing which
one is active.

On the website side, `docs/themes/generate_themes.py` is the single source of
truth for the roster and emits every artifact from it: the CSS token blocks, the
JavaScript mode list, and the theme menu. It also runs a WCAG contrast audit,
and `--check` fails CI when any text token drops below 4.5:1 against the
backgrounds it can appear on. That check is why a theme cannot ship here looking
good and reading badly.

The shell does not currently run that audit. Its twenty imported palettes were
designed by other people for editors, and I have not held them to the same
contrast floor. That is a real gap rather than a deliberate decision.

## What this replaced

| Was | Now |
|---|---|
| Waybar | `Bar/` |
| wofi | `Launcher/` |
| mako | `Services/Notifications.qml` and the notification panel |
| Three config languages | One QML codebase |

The honest tradeoff: the replaced tools are maintained by other people and mine
is not. When Quickshell changes, I fix this. That is a cost I accepted knowingly
for the thing I wanted, which was one place to change behaviour instead of three.

## What I built, what I directed, and what I still don't understand

This project is AI-assisted, substantially so, and it is more useful to say
where the line falls than to imply there isn't one.

**What I can explain end to end.** The shell's architecture and why it is shaped
this way: services owning state, panels owning appearance, widgets shared. The
palette system completely — the token contract, why thirteen tokens rather than
a full colour scale, how a palette flows from JSON to a rendered panel, and why
the site and the desktop are reading the same values. The Hyprland configuration,
all of it. What each bar module shows and where its data comes from. When
something on the bar is wrong, I can find it.

**What I directed rather than wrote.** Most of the QML itself. I specified what
a panel should do, what state it should read, and how it should behave at the
edges; an agent wrote the implementation and I reviewed and corrected it. The
review is real — I have rejected and rewritten plenty — but reviewing code and
being able to write it unaided from a blank file are different skills, and I
have the first one here more than the second.

**What I do not understand yet.** Quickshell's rendering and event internals: I
use the framework correctly by pattern and by documentation, not because I could
tell you what it does underneath. QML's property binding and garbage collection
behaviour under load — I have hit performance problems and fixed them by
changing what I bind, without a full model of why the fix worked. Wayland
protocol details below what Hyprland and Quickshell expose to me.

The part I would defend under questioning is the shell's structure and the
palette pipeline. The part I would not claim is authorship of thirteen thousand
lines of QML.

## Status

In progress, and honestly so. It runs as my daily desktop, which is the only
real test it gets. There is no installer: the README in the repository describes
one, along with a `bin/geist-db` helper and a `systemd/` directory, and none of
those currently exist in the tree. The repository documentation is ahead of the
repository, which is its own small lesson about writing the README first.

## Related writing

- [Two renderers, one palette](/blog/two-renderers-one-palette) — how the shared
  theme roster works, and the contrast check that gates it.

## Source

[github.com/machinageist/geistos](https://github.com/machinageist/geistos) —
the Hyprland configuration, the Quickshell codebase, and the palette roster.
The application suite it launches is
[documented separately](/portfolio/mg-suite).
