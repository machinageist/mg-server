---
title: "Geist roadmap"
date: 2026-08-06
summary: "A dependency-ordered map of Geist milestones R0 through R12, separating verified foundations, active work, and deferred outcomes."
tags: [draft, geist, roadmap, milestones, validation]
---

> **DRAFT / UNPUBLISHED.** This page is not routed or listed on the public site.
>
> **Last synchronized:** 2026-08-06. R0 and R1 have exited. R2 is active and in
> progress; R3 through R12 are planned or deferred, not implemented milestones.

## How to read the roadmap

Geist uses milestones to prove dependency-ordered capabilities. A milestone is not done
because a crate compiles, a control appears in a window, or a design document exists.
Its named outcome and exit evidence must agree.

The labels below mean:

- **Exited:** the accepted milestone record says its exit evidence passed.
- **Active / in progress:** implementation and targeted evidence exist, but the
  milestone is not closed.
- **Deferred:** the outcome is intentionally later; detailed requirements may wait for
  that milestone's intake.

## R0–R12

| Milestone | Outcome and evidence bar | Synchronized state |
|---|---|---|
| **R0 — Foundation** | Stable pinned toolchain, workspace rules, formatting and strict linting, tests, and deterministic offline harness | **Exited** on 2026-07-17 |
| **R1 — Musical kernel** | Stable IDs; explicit musical/sample time; tempo and meter; transport; deterministic events; parameters; command seed; versioned project round trip, backed by properties and fixtures | **Exited** on 2026-07-17; later reorder and migration evidence remains correctly gated to R4/R5 |
| **R2 — Offline graph** | Editable graph → validated immutable compiled plan → deterministic offline render, with source/gain device evidence and silence, impulse, allocation, and deterministic-hash gates | **Active / in progress.** Targeted graph, fixture, and app-snapshot evidence is recorded; the latest full workspace gate awaited rerun/review at synchronization |
| **R3 — Live shell** | Qualified audio backend, callback bridge using the same plan, timestamped MIDI, and health telemetry; allocation/lock guards and device lifecycle drill | **Deferred; not implemented** |
| **R4 — Credible alpha** | One track to master, MIDI clip, small original synth and effect, minimal UI, transport, atomic save/reload, and offline bounce; end-to-end fixture and manual QA | **Deferred; not an alpha today** |
| **R5 — Project safety** | Atomic-save qualification, journaled autosave, recovery, migrations, undo/redo, and missing-media diagnostics; crash/recovery drills | **Deferred.** Design contracts exist for part of persistence, but that is not implementation or crash proof |
| **R6 — Tracks, routing, mixer** | Track types, groups, sends/returns, monitoring, compensation, and meters; latency matrix | **Deferred** |
| **R7 — Arrangement and recording** | Audio/MIDI recording and editing, piano roll, fades, count-in, metronome, and salvage drills | **Deferred** |
| **R8 — VST3 host** | Isolated scanning, processing, state, editor integration, placeholders, and fixture matrix; binding/license decision at intake | **Deferred and gated** |
| **R9 — Automation and modulation** | Stable bindings, required sample accuracy, override/restore semantics, and visible overlays; semantics tests | **Deferred** |
| **R10 — Session and live performance** | Clip slots, scenes, quantized launch, explicit per-track timeline/performance authority, and performance capture drills | **Deferred** |
| **R11 — Geist identity** | Original modular surface, flagship synth, deeper MIDI, and effect catalog; identity-layer QA | **Deferred.** The identity is accepted direction, not a completed surface |
| **R12 — Release qualification** | Performance and soak evidence, accessibility, recovery, packaging, documentation, and published release gates | **Deferred** |

## Release bars are not current claims

The accepted vision places a credible alpha at verified R4, a musician beta after the
loop-first production core and project-safety work across R5–R10, and a 1.0 only after
VST3 hosting, the Geist identity layer, accessibility, and packaging are ready. A later
professional-ready bar also requires workflow, reliability, performance-budget,
recovery, compatibility, and documentation evidence.

These bars are definitions of future readiness. They do not rename the current R2
prototype as an alpha or imply that later features are partly complete because they
appear in a roadmap.

## Important gated decisions

Some future work has safe planning defaults but still requires evidence or owner review:

- explicit delayed feedback is a design default, gated before the R11 modular surface;
- the VST3 binding and license choice is gated at R8 intake;
- sample decode/streaming and time-stretch details wait for R7 evidence;
- undo architecture is revisited at R5 exit;
- accessibility receives an R4 audit and blocks beta if its baseline is unmet; and
- every numeric limit needs a Geist rationale rather than inheritance from a reference
  product.

Deferring these decisions is deliberate dependency control, not evidence that the
corresponding capability already exists.

## Source pointers

Milestone outcomes come from `docs/06-plans/rebuild-roadmap.md`. Current state comes
from `docs/06-plans/current-milestone.md`, `docs/status/STATUS.md`,
`docs/status/NEXT.md`, and `docs/01-requirements/traceability.md`. Release bars and
non-goals come from `docs/00-product/vision.md`; future gates come from
`docs/01-requirements/decision-gates.md`.
