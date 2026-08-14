---
title: "Spectre architecture"
date: 2026-08-06
summary: "The current Spectre crate map, graph compilation boundary, deterministic offline path, realtime rules, and known gaps."
tags: [draft, spectre, architecture, rust, audio, realtime]
---

> **DRAFT / UNPUBLISHED.** This page is not routed or listed on the public site.
>
> **Last synchronized:** 2026-08-06. R2 is the active milestone and remains in
> progress pending review of its full workspace gate.

## Architecture in one sentence

Spectre separates mutable application work from a validated, immutable process plan so
that editing can remain expressive without allowing UI state, file operations, or
unbounded work onto the eventual audio callback.

The current repository is a Rust workspace with six crates. Their present roles are
narrower than the eventual DAW:

| Crate | Current responsibility | Boundary |
|---|---|---|
| `spectre-core` | Stable IDs, explicit time types, tempo and meter maps, transport, deterministic event order, parameter descriptors | Shared musical and identity primitives; not a DAW UI or audio backend |
| `spectre-dsp` | Planar-buffer device contract, bounded note events, deterministic source, Pulse instrument, Gain, Saturator | Native processing seeds; not the flagship synth/effect catalog |
| `spectre-graph` | App-thread editable graph, validation, compilation, immutable plan execution | Offline GRAPH-001 seam; live publication and feedback remain open |
| `spectre-project` | Versioned JSON envelope, semantic decode validation, atomic command transactions, bounded undo/redo | Codec/model seed; filesystem atomic save and recovery are later work |
| `spectre-offline` | Deterministic inspection and Pulse → Gain → Saturator rendering through a compiled plan | Evidence harness; not realtime device output |
| `spectre-app` | Native interaction prototype, backend-derived controls, owned parameter-snapshot and feedback seams | Prototype UI/model; Play changes model state but emits no sound |

## Application thread and render-plan boundary

The editable graph belongs to the application side. It owns graph structure but no DSP
processors or audio buffers, and it cannot render. Compilation selects the ancestors of
a designated output, rejects missing inputs and implicit cycles, verifies processor
layouts, chooses a deterministic order, and preallocates planar `f32` buffers.

The resulting `CompiledPlan` is a distinct type. It exposes processing rather than node
or edge mutation. Processor construction happens before a render quantum begins.
App-model parameter snapshots are owned values with stable backend identities and
finite, descriptor-clamped values; the offline renderer accepts an entry only when both
device and parameter identities match exactly.

This boundary is implemented for the current offline path. It does **not** yet define
the R3 mechanism that will publish and reclaim plans across application and audio
threads. Bounded wait-free communication, overflow policy, and off-thread reclamation
remain requirements rather than proven live behavior.

## Compiled-plan and eventual audio-thread rules

On valid compiled input, process code must not allocate or deallocate, take blocking
locks, perform I/O, log, format strings, serialize, inspect UI state, or panic across
the callback boundary. Events and channel storage are bounded and borrowed. Device
layouts are fixed before processing, output buffers are fully written, and non-finite
values must be contained rather than propagated.

The present plan path uses frozen execution order and preallocated planar buffers.
Take/restore buffer handoff avoids allocation and locking in steady-state process
quanta. These properties have targeted offline evidence. They are necessary groundwork,
not proof that a live audio callback, device lifecycle, MIDI ingress, or underrun policy
exists.

The realtime requirement set also calls for:

- bounded wait-free control-to-render communication with explicit overflow behavior;
- off-thread reclamation of retired state;
- denormal handling; and
- NaN/Inf isolation that silences an offending node and surfaces a diagnostic.

Those requirements are not all implemented. The traceability ledger currently treats
the RT family as policy/proposed work for the live milestone.

## Deterministic offline render

Offline rendering is the architecture's current evidence surface. The native fixture
runs Pulse → Gain → Saturator through the compiled plan. For identical initial state,
inputs, and app snapshot, the path is expected to produce identical output and report.
The synchronized evidence records:

- bit identity between the compiled-plan fixture and its hand-wired reference chain;
- exact-silence behavior;
- sample-exact impulse behavior;
- measured allocation/deallocation freedom for steady-state plan quanta; and
- a deterministic output hash.

This determinism is deliberately scoped. It proves the named fixture and process path;
it does not establish all future devices, plugins, operating systems, sample rates,
projects, or realtime scheduling behavior.

## DSP process contract

The first device contract uses planar `f32` audio. A process call covers a bounded
number of frames at a finite positive sample rate. Initial layouts cover a source, a
note-driven instrument, a stereo insert, and a stereo sidechain effect. Note events are
bounded, sample-offset, and deterministically ordered, with note termination before
note start at the same frame.

V1 applies one parameter value per render quantum. Device-owned smoothing handles
changes that could click or destabilize processing. Sample-accurate automation is
planned for R9 rather than implied by the current snapshot seam.

## Current gaps

As of the synchronization date, the architecture does not yet provide:

- live audio-device discovery, startup, shutdown, or callback bridging;
- MIDI ingress and timestamp qualification;
- realtime plan publication/reclamation and health telemetry;
- latency compensation;
- explicit feedback edges with a priced delay;
- general bus summing or buffer-reuse optimization;
- recording, media streaming, or track/mixer routing;
- VST3 hosting or plugin crash containment;
- complete atomic filesystem save, autosave, recovery, migration, or missing-media UX;
- automation, performance launching, or the modular identity surface.

## Source pointers

This page follows `Cargo.toml`, `docs/03-architecture/dsp-device-io.md`,
`docs/03-architecture/graph-compilation.md`,
`docs/03-architecture/project-persistence.md`,
`docs/01-requirements/requirements-ledger.md`,
`docs/01-requirements/traceability.md`, `docs/status/STATUS.md`, and
`docs/06-plans/current-milestone.md`.
