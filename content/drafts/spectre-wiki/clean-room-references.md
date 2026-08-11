---
title: "Geist clean-room references"
date: 2026-08-06
summary: "How Geist studies public DAW, synth, and modular workflows without cloning products or turning research gaps into promises."
tags: [draft, geist, research, clean-room, daw, synthesis]
---

> **DRAFT / UNPUBLISHED.** This page is not routed or listed on the public site.
>
> **Last synchronized:** 2026-08-06. Reference research remains draft evidence with
> uneven coverage; it is not a compatibility matrix or a feature-completeness claim.

## Why study other music tools?

Mature DAWs, synthesizers, and modular environments expose years of accumulated design
questions: how playback authority is communicated, how automation is overridden, how
modulation remains legible, how routing handles feedback, how a musician recovers from
mistakes, and where a workflow loses context.

Geist studies those questions to make original decisions. It does not use another
product as an architecture, skin, file-format template, or checklist for parity.
Research informs candidates; accepted Geist requirements and architecture contracts
determine the product.

## Reference roles

The current research program gives different products bounded roles:

| Reference | What public research can illuminate | What it does not authorize |
|---|---|---|
| **Ableton Live** | Arrangement/session interaction, automation states, commands, capture, and loop-to-performance workflows | Live parity, Set compatibility, copied UI, bindings, devices, or numeric limits |
| **Bitwig Studio / Grid** | Launcher/arranger relationships, visible modulation, typed/polyphonic signal concepts, controller and modular workflows | Bitwig/Grid compatibility, copied layouts, architecture, presets, or limits |
| **Kilohearts Phase Plant** | Modulation, generator/effect routing, and layered sound-design behavior described by public documentation | Copied lanes, macros, module limits, DSP, factory content, or a Phase Plant clone |
| **Xfer Serum 2** | Bounded sound-design and CPU/workflow observations from exposed official material | Claims of complete coverage; the inspected public sources do not provide a complete manual |
| **VCV Rack 2** | Modular patching, polyphony, signal conventions, commands, and performance questions | Copied source, panel art, module layouts, patch files, voltage conventions, or library assets |
| **REAPER** | Editing, routing, actions/customization, backup, latency adjustment, and end-to-end workflow candidates | Claims that its candidate videos represent common use, or a REAPER-compatible action system |
| **FL Studio, Logic Pro, Cubase, and others** | Additional evidence about composition, recording, editing, scoring, and production where the source and Geist relevance are explicit | A popularity vote, universal workflow, or permission to import proprietary behavior wholesale |

The role of a product can narrow or change as source quality improves. A supported-host
list, marketing page, or isolated tutorial does not establish a broad requirement.

## Clean-room boundary

Research uses publicly authorized sources. It excludes leaked or private manuals,
decompiled code, proprietary project files and schemas, copied screenshots, factory
presets, samples, wavetables, artwork, names, and distinctive expression.

Public behavior can still be studied, but observations and Geist decisions remain
separate. If a manual says a feature exists, that establishes only the documented
behavior within the inspected version and section. It does not reveal the algorithm,
prove edge cases, or require Geist to implement it.

Numeric limits are especially controlled. Geist does not inherit another product's
voice count, wavetable dimensions, time-signature envelope, modulation count, buffer
size, or feedback rule merely because it is documented. Every adopted limit needs an
original Geist rationale and acceptance evidence.

## From source to requirement

Each claim belongs to one layer:

1. **Observed:** a cited public source explicitly supports the behavior.
2. **Source gap:** the inspected source is silent, ambiguous, inaccessible, or
   insufficiently versioned.
3. **Geist candidate:** a possible product implication, without authority.
4. **Geist requirement:** an adopted, stable requirement in the requirements ledger.
5. **Implementation decision:** an original Geist design linked to an accepted
   architecture contract or decision record.

Skipping a layer creates false authority. A research dossier cannot make a product
commitment simply by sounding comprehensive.

A strong source record identifies the product and version, direct source, access date,
sections or timestamps inspected, mutable-URL risk, limitations, and exactly which
claims it does and does not support. Coverage is tracked by section. Unknown algorithms,
defaults, limits, schemas, and edge cases remain unknown.

## Manuals and real workflows are different evidence

Official manuals are good evidence for documented capability and terminology. They do
not establish what musicians do most often, what sequence they follow in a complete
project, or where they encounter friction.

Workflow evidence therefore records context, ordered actions, shortcuts or gestures,
state changes, workarounds, confidence, and corroboration. Artist interviews can provide
useful self-report, but self-report is not a visible action sequence. A candidate video
is not reviewed evidence until its relevant actions and limitations are extracted.

At synchronization time, the workflow corpus was explicitly below its qualitative
sampling floors. It contained four timestamped FL Studio action-sequence observations
and two Ableton thematic self-reports, while visible-session Bitwig and corroborating
Ableton evidence remained important gaps. The research therefore supports no
population-level frequency, workflow-prevalence, convergence, priority, or usability
claim.

## No parity or compatibility promises

Cross-product convergence can justify investigating a design problem. It does not prove
that Geist needs identical semantics, and it never proves implementation completeness.
Geist's accepted product vision explicitly rejects a feature-parity race and project or
preset compatibility with other DAWs and synths.

The intended result is an original open-source instrument shaped by evidence: linked
lenses, visible modulation, an explicit typed signal model, realtime-safe Rust
boundaries, and a calm keyboard-first interface. Those ideas still require Geist's own
requirements, contracts, tests, and musician review.

## Source pointers

Research policy comes from `docs/02-reference-research/methodology.md`. Product roles
and current gaps are tracked in `docs/02-reference-research/external-reference-register.md`,
`docs/02-reference-research/source-ledger.json`, the product dossiers, and
`docs/02-reference-research/workflow-field-study/`. Adopted product direction and
requirements remain in `docs/00-product/vision.md` and
`docs/01-requirements/requirements-ledger.md`.
