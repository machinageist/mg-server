---
title: "Building a Quiet, Local-First Arch Workstation"
date: 2026-08-23
summary: "A working draft on restructuring my Arch desktop around quiet status pills, deeper interactive cards, stable CLI boundaries, and reversible configuration."
tags: [arch-linux, quickshell, hyprland, local-first, rust, workstation]
---

I have been rebuilding my Arch workstation around a simple rule: persistent
interface elements should show only information worth seeing all day, while the
detail and controls should remain one action away.

I call the pattern **quiet bar, deep cards**.

The bar is a dashboard, not a control panel. A battery pill can show charge. An
AI pill can show a compact current-day summary. A health pill can indicate that
maintenance needs attention. Clicking or using a keybinding opens a larger card
with the explanation, evidence, and common actions.

This draft records the architecture and the first working desktop slice. It is
not yet a release announcement for the larger workstation kit.

## Why I changed the desktop model

A status bar can become noisy very quickly. CPU, memory, disk, temperature,
network, battery, package updates, notifications, timers, music, and application
state all compete for permanent space. Giving each metric another number or
sparkline does not necessarily improve awareness. It can make the bar harder to
scan and teach me to ignore it.

I wanted three layers instead:

1. **Pills** for persistent, high-value state.
2. **Cards** for inspection and common actions.
3. **CLI/TUI tools** for deep workflows and recovery.

That last layer is important. Quickshell is a client of the system, not the
owner of it. If the shell crashes, I should still be able to inspect power,
perform maintenance, search notes, or work with a calendar from the terminal.

## The first implemented slice

The current desktop work adds an Operations card to my Quickshell environment.
It groups five related views:

- Overview
- AI usage
- Power
- Maintenance
- Security posture

A read-only telemetry script collects and normalizes system state. Quickshell
consumes that output through a QML service rather than spreading shell commands
through many visual components.

The same work introduced a shared command catalog. The launcher can search both
installed applications and workstation actions, so opening a maintenance view
or power view is discoverable rather than dependent on remembering one exact
binding.

The operations surfaces are also reachable through Quickshell IPC. Examples
from the development configuration include:

```sh
qs -c mgeist ipc call operations tab overview
qs -c mgeist ipc call operations tab power
qs -c mgeist ipc call operations tab maintenance
qs -c mgeist ipc call operations status
```

Those calls made the panel testable without relying only on pointer interaction.

## Keeping the battery pill quiet

Battery telemetry was a useful design test.

The persistent pill initially risked becoming a sentence: charge percentage,
time remaining, power rate, and health. I reduced it back to percentage-only.
The card carries the explanation:

- charging or discharging state;
- estimated time to full or empty;
- current energy rate; and
- physical battery health.

UPower also exposed a practical integration problem. Its display device did not
reliably provide the battery-health value I needed, so the implementation had
to enumerate devices and select the physical laptop battery. Health values also
appeared in more than one numerical scale. Normalizing both `0..1` and `0..100`
representations fixed an incorrect `9130%` display.

That defect reinforced an important rule: a polished card is not evidence that
the data source is correct. Visual verification and source-level verification
both matter.

## Verification performed

The desktop slice was exercised rather than accepted from compilation alone:

- the telemetry collector compiled;
- its JSON schema assertion passed;
- Quickshell restarted successfully;
- Operations IPC calls opened the expected views;
- launcher searches returned the relevant system actions;
- four direct Operations keybindings were registered by Hyprland;
- screenshots of the Overview, AI, Power, battery-health, and command-palette
  states were reviewed;
- scoped diff hygiene passed.

The shell remains the machine-specific layer, while the reusable application
contracts are designed as separate products with their own repositories.

## From dotfiles to a product family

The larger direction is not to publish a snapshot of my home directory. It is
to separate reusable products from machine-specific configuration:

```text
geist-shell   Quickshell pills, cards, command palette, and adapters
mg-calr       calendar, reminders, and local todos
mg-vault      Markdown knowledge system and terminal editor
geist-config  packages, Hyprland, terminal tools, install, and rollback
```

Each application owns its domain. The shell consumes stable JSON or IPC and
launches public commands. The configuration repository joins compatible
versions through manifests.

This avoids two failure modes:

- hiding application logic inside QML; and
- turning one personal dotfiles tree into an inseparable pseudo-distribution.

The target remains an upstream Arch workstation layer. Normal Arch packages and
configuration stay visible. Installation, upgrade, rollback, uninstall, and
local overlays need explicit behavior.

## The shared application rule

Every daily-driver application added to this environment should provide:

1. a minimal pill, or deliberate grouping under a related pill;
2. an interactive card;
3. a direct card keybinding;
4. command-palette actions;
5. stable machine-readable status and quick-action contracts;
6. a path into the full CLI/TUI workflow; and
7. a complete terminal recovery path.

The card is not a miniature replacement for the application. It is the bridge
between ambient awareness and deliberate work.

## What remains

The shell code still needs clean extraction from the mixed dotfiles tree,
independent review, packaging, install/rollback tooling, and clean-machine
verification. The calendar and notes applications are now separate Rust
repositories with initial foundation slices; richer interfaces remain planned.

The larger lesson so far is that a minimalist desktop is not created by hiding
complexity. It is created by deciding where complexity belongs, giving it stable
interfaces, and preserving a way to operate the system when the preferred UI is
not available.
