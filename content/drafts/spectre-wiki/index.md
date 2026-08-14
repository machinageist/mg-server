---
title: "Spectre-Seq"
date: 2026-08-06
summary: "An honest introduction to Spectre's electronic-music mission, linked-lens design, present boundary, and staged roadmap."
tags: [draft, spectre, daw, electronic-music, open-source]
---

> **DRAFT / UNPUBLISHED.** This page is not routed or listed on the public site.
>
> **Last synchronized:** 2026-08-06. Current implementation statements reflect the
> dated Spectre status and traceability records; R2 is active and still in progress.

## What Spectre is

Spectre is an open-source digital audio workstation being designed for electronic
musicians who compose, shape sounds, and perform their own material. Its first concrete
contexts are hypnotic techno, forest psytrance, deep dubstep, and modern
synthesis-driven arrangement. Studio production and live performance both matter;
band-recording and scoring workflows are respected later-stage concerns rather than
the first target.

The product direction starts with a loop-first creative cycle: sketch a loop, branch
variations, audition quickly, and grow an arrangement without repeatedly losing
selection, zoom, or transport context. It also treats first-party synthesis and effects,
performance playback, trustworthy audio/MIDI capture, and project recovery as parts of
one instrument-building environment.

That is the intended product. It is not a statement that all of those capabilities are
available today.

## One project, linked lenses

Spectre's central identity is **one project viewed through linked lenses**:

- a timeline for arrangement;
- a performance grid for launching clips and scenes;
- a mixer for routing and level decisions; and
- a modular sound-flow view for synthesis and signal design.

These are intended to share object identity, selection, and context rather than behave
like separate applications joined by import/export steps. Playback authority between
timeline and performance views is planned to be explicit per track. Modulation is also
intended to be visible as its own contribution alongside a parameter's base value and
automation.

The linked-lens product model is accepted direction. The complete timeline,
performance grid, mixer, and modular surface are later roadmap outcomes, not current
implementation claims.

## Current capability boundary

**Verified state as synchronized on 2026-08-06:** R0 foundation and R1 musical-kernel
milestones have exited. The repository contains explicit musical time and identity
types, tempo and meter maps, deterministic transport and event ordering, parameter
descriptors, a versioned project envelope, a command/undo seed, native DSP devices,
and a deterministic offline harness.

R2, the offline-graph milestone, is **in progress**. Its dated evidence includes an
app-thread editable graph, validated compilation into an immutable render plan, an
offline Pulse → Gain → Saturator fixture, and targeted silence, impulse, allocation,
and deterministic-hash checks. App parameter snapshots can influence offline plan
construction using exact backend device and parameter identities. The full workspace
gate for the latest R2 slice was still awaiting rerun/review at synchronization time,
so this wiki does not mark R2 complete.

A graphical interaction prototype can show backend-derived device controls and change
model state. It is not a functioning live DAW. There is currently no audio backend,
callback bridge, audible Play path, VST3 host, recording path, arrangement canvas, or
performance launcher. Project-safe filesystem save/reload and crash recovery are also
later milestone work.

## Roadmap map

The dependency-ordered plan is intentionally incremental:

- **R0–R1:** foundation and musical kernel — exited with dated evidence.
- **R2:** offline graph and deterministic rendering — active, in progress.
- **R3:** live audio shell and callback bridge.
- **R4:** credible one-track alpha with MIDI clip, native synth/effect, save/reload,
  transport, and bounce.
- **R5–R10:** project safety, routing and mixer, recording and arrangement, VST3
  hosting, automation/modulation, and session/live performance.
- **R11:** the original Spectre modular and flagship-synth identity layer.
- **R12:** release qualification, including performance, recovery, accessibility,
  packaging, and documentation.

Each milestone closes on demonstrated capability and named evidence, not compilation or
screen presence alone. See the [roadmap](roadmap.md) and
[quality model](quality-and-validation.md) for the distinctions.

## What Spectre does not claim

Spectre is not pursuing a feature-parity race. Studying mature tools can reveal useful
behavior and hard questions, but it does not define completeness. Spectre does not claim
preset or project compatibility with other DAWs or synths, and it does not copy their
formats, numeric limits, assets, visual identity, or proprietary implementation.

The current plan also excludes cloud services and pre-1.0 CLAP, LV2, and Audio Unit
hosting. VST3 hosting is a later gated milestone. Open-source intent and a serious
architecture do not by themselves establish alpha, beta, 1.0, or professional
readiness.

## Source pointers

Product and current-state claims are grounded in the Spectre repository's
`docs/00-product/vision.md`, `docs/06-plans/rebuild-roadmap.md`,
`docs/06-plans/current-milestone.md`, `docs/status/STATUS.md`, `docs/status/NEXT.md`,
`docs/01-requirements/traceability.md`, and `docs/03-architecture/`.
