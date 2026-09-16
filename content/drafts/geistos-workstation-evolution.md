---
title: "Geistos: a workstation that can explain itself"
date: 2026-09-15
summary: "An in-progress engineering note about building a local-first Arch workstation around Hyprland, Quickshell, and a small suite of inspectable tools."
tags: [geistos, arch-linux, quickshell, hyprland, local-first, systems]
---

Geistos is becoming the name for the workstation layer around the Geist application suite. It is not a finished distribution and it is not an attempt to hide an entire home directory behind a theme. The active workstation source is currently the dotfiles repository's `quickshell-shell` branch: a working Hyprland and Quickshell configuration with service units, shell adapters, theme and wallpaper synchronization, and inspectable desktop surfaces. The older package-oriented Geistos tree is divergent and is not a mirror of that source.

The problem is friction. A desktop should make daily state visible without turning every pixel into a control panel. It should also make the next level of detail easy to reach, and keep a terminal path when the preferred interface is unavailable.

## Quiet surfaces, deep controls

The shell follows a three-level model:

1. pills for persistent, high-value state;
2. cards for inspection and common actions;
3. CLI/TUI tools for deep workflows and recovery.

Quickshell is therefore a client, not the owner of calendar, reminder, vault, or database truth. The desktop consumes stable command output and IPC contracts. The applications remain separate Rust repositories with their own storage and tests.

## Why Hyprland and Quickshell

Hyprland provides a composable Wayland window manager with a declarative configuration boundary. Quickshell provides a resident QML surface that can replace several small desktop utilities without forcing their domain logic into QML. The bar, launcher, notifications, cards, theme selector, and session menu can share interaction primitives while still calling normal commands underneath.

That choice creates obligations. A shell that looks coherent but cannot be restarted, inspected, or operated from a terminal is only a screenshot. The repository keeps lifecycle actions explicit, gives bindings descriptions so they can be browsed, and keeps the database helper separate from the visual layer.

## The wallpaper as an event

The wallpaper is more than decoration in the current design. One service owns the active path. Applying a wallpaper updates Hyprpaper, persists the choice, feeds the automatic palette, and updates the stable link Hyprlock reads when the lock screen starts. Theme-client synchronization then exports the resolved colors to other tools.

This is deliberately narrower than a universal theme framework. The useful boundary is a single state transition with several consumers, not a second watcher for each application.

## What is working and what is not

The active dotfiles source now has a shared launcher result boundary, application and action search, calculator and unit-conversion providers, and a Rust-backed `mg-calcr` JSON contract used by the launcher and dedicated calculator. The calculator's standard/scientific keypads, bounded graph sampler, range controls, history, and shared keyboard-action primitive are implemented. Its request lifecycle rejects cancelled or unsuccessfully completed subprocess output, while whole-shell keyboard coverage and richer calculator interactions remain unfinished. A dictionary/thesaurus provider exists as an optional network path and reports failure safely when the environment cannot reach its API; no offline dictionary is being claimed.

The weather surface now keeps the compact bar reading while expanding into current conditions. Its primary Open-Meteo path supplies twelve hourly entries and seven daily entries; the wttr.in fallback and approximate-location path may provide a smaller, explicitly bounded forecast instead of inventing missing data. Provider units, ordering, required fields, response size, and process completion are validated before new data is published; a failed refresh retains the previous successful reading and marks it stale rather than replacing it with partial output. The successful live card has been inspected, but every scroll, settings, accessibility, theme, and multi-monitor interaction has not yet been signed off.

The same source also contains the bar, launcher, notifications, calendar and reminder cards, operations surfaces, local AI panel, wallpaper/theme services, PostgreSQL resolution, persistent system HUD, and an end-to-end suite projection test. These are verified slices of the workstation, not evidence that Geistos is a finished distribution. The link-only installer has been smoke-tested in a disposable home, but the distribution installer, PKGBUILD, clean-machine setup, package pinning, and reproducible setup documentation are unfinished. The package-oriented tree remains divergent and must not be described as synchronized with the active workstation source.

## Lessons so far

- A visual card is not evidence that its data source is correct.
- A projection needs freshness semantics separate from the source record's timestamp.
- A shared service boundary is more valuable than a clever UI abstraction.
- Recovery actions deserve first-class keyboard paths.
- Public documentation should follow repository state, not the other way around.
