---
title: "geistos"
date: 2026-09-20
summary: "A local-first Linux workstation: a Quickshell desktop replacing the usual bar and launcher, a generated palette system shared with this site, and Hyprland configured in Lua."
tags: [linux, wayland, hyprland, quickshell, theming, workstation]
---

## What this is

`geistos` is my daily Linux desktop, kept in a repository so I can rebuild it
instead of remembering how I set it up. It is a Wayland workstation. Hyprland is
the compositor, and one Quickshell QML codebase provides the bar, launcher,
notifications, and panels that would otherwise be three or four separate
programs.

I built it because I wanted to stop letting my working environment pile up.
Configuration that only lives on one machine is hard to reason about. A desktop
put together from six tools, each with its own config language, is hard to
change without breaking something I forgot about.

Despite the name, it is not an operating system or a distribution. It is a
configuration repository and a QML application that runs on Arch.

## The shell

The desktop is 153 QML files, about 13,000 lines, in one Quickshell
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

What keeps it maintainable is the split between `Services/` and everything else.
A service is a singleton that owns one piece of state and has no appearance: the
battery reading, the current palette, the notification list. Panels and bar
modules read from services and keep no state of their own. When a bar module and
a panel both show the same thing, they read the same singleton instead of each
polling on its own timer, so they cannot disagree.

`Widgets/` holds the shared visual pieces. A `Pill` looks the same whether it
shows the battery or the clock, so the bar stays consistent without every module
redoing its own padding and hover behavior.

## Hyprland in Lua

The compositor configuration is written in Lua instead of Hyprland's own config
format:

```text
config/hypr/
  hyprland.lua      keybindings.lua   window_rules.lua
  layout.lua        input.lua         monitors.lua
  look_and_feel.lua animations.lua    programs.lua
  autostart.lua     environment.lua   permissions.lua
```

There is one file per concern, and that matters more than the language. When I
want to change a window rule, I open `window_rules.lua`, not a 600-line file
where the rules sit between animation curves and monitor layout.

## The palette system, and the link to this site

The shell has 43 palettes. Twenty-three of them are marked `"source":
"mg-server"` in `palettes.json`, because they are the themes this website ships.
They use the same token names and the same hex values, rendered by two completely
different stacks. The site turns them into CSS custom properties, and the shell
turns them into QML properties. Solarcore on the site and Solarcore on the
desktop are the same colors because both read the same numbers.

The other twenty are ports of themes other people designed, such as Catppuccin,
Tokyo Night, Rosé Pine, Everforest, Kanagawa, Monokai, Ayu, Nord, and Zenburn,
plus five of my own. Every palette declares the same ten color tokens, so a
panel written against `Theme.accent` works with all forty-three without knowing
which one is active.

On the website side, `docs/themes/generate_themes.py` holds the roster and
generates everything from it: the CSS token blocks, the JavaScript mode list, and
the theme menu. It also runs a WCAG contrast check, and `--check` fails CI when
any text token drops below 4.5:1 against a background it can appear on. That is
why a theme cannot ship here if it is hard to read.

The shell does not run that check yet. Its twenty imported palettes were designed
by other people for code editors, and I have not held them to the same contrast
minimum. That is a gap I have not closed, not a decision I made.

## What this replaced

| Was | Now |
|---|---|
| Waybar | `Bar/` |
| wofi | `Launcher/` |
| mako | `Services/Notifications.qml` and the notification panel |
| Three config languages | One QML codebase |

The tradeoff is that other people maintain the tools I replaced, and nobody but
me maintains this. When Quickshell changes, I fix it. I accepted that cost to get
one place to change behavior instead of three.

## What I built, what I directed, and what I still don't understand

This project is AI-assisted, heavily. Here is where the line falls.

**What I can explain end to end.** The shell's architecture and why it is built
this way: services own state, panels own appearance, and widgets are shared. The
palette system in full: the token contract, why it is ten tokens and not a full
color scale, how a palette gets from JSON to a rendered panel, and why the site
and the desktop read the same values. All of the Hyprland configuration. What
each bar module shows and where its data comes from. When something on the bar
is wrong, I can find it.

**What I directed rather than wrote.** Most of the QML itself. I specified what a
panel should do, what state it should read, and how it should handle edge cases.
An agent wrote the implementation, and I reviewed and corrected it. I have
rejected and rewritten plenty of it. But reviewing code and writing it from a
blank file are different skills, and here I have more of the first than the
second.

**What I do not understand yet.** Quickshell's rendering and event internals. I
use the framework correctly by following patterns and the documentation, not
because I could explain what it does underneath. How QML property bindings and
garbage collection behave under load. I have hit performance problems and fixed
them by changing what I bind, without fully understanding why the fix worked.
Wayland protocol details below what Hyprland and Quickshell expose to me.

I can answer questions about the shell's structure and the palette pipeline. I
would not claim to have written thirteen thousand lines of QML.

## Status

In progress. It runs as my daily desktop, which is the only real test it gets.
There is no installer yet. The repository says so, and the packaging notes list
the open questions as open.

The documentation tends to run ahead of the code. A retired helper kept its
README entry for a while after nothing called it anymore, and the theme count in
the README is from several palettes ago. Neither is serious on its own. Together
they are a reminder that the README can be wrong, and the code is what a reader
can check.

## Related writing

- [Two renderers, one palette](/blog/two-renderers-one-palette) — how the shared
  theme roster works, and the contrast check that gates it.

## Source

[github.com/machinageist/geistos](https://github.com/machinageist/geistos) holds
the Hyprland configuration, the Quickshell codebase, and the palette roster. The
application suite it launches is [documented separately](/portfolio/mg-suite).
