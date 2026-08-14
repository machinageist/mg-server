---
title: "Spectre quality and validation"
date: 2026-08-06
summary: "How Spectre separates slice evidence, deterministic DSP checks, realtime contracts, blind review, and product readiness."
tags: [draft, spectre, quality, testing, realtime, audio]
---

> **DRAFT / UNPUBLISHED.** This page is not routed or listed on the public site.
>
> **Last synchronized:** 2026-08-06. The latest accepted status records a green
> full-workspace baseline from 2026-07-17 and newer targeted R2 evidence from
> 2026-08-06; the parent full-workspace rerun was still pending.

## Quality is a chain of scoped claims

A DAW can pass a unit test and still lose a project, glitch under load, misplace an
event, or make a common musical task exhausting. Spectre therefore treats evidence as a
chain: a small implementation slice must pass its own gate, the milestone must pass its
exit gate, and the product must eventually pass release-level workflow, reliability,
performance, recovery, compatibility, accessibility, and documentation gates.

Passing a narrower layer never automatically passes the broader one.

## Slice gates

A slice begins with one bounded claim and closes only when its implementation,
requirements, architecture, traceability, and tests agree. Depending on the slice, the
evidence can include:

1. formatting and strict linting;
2. focused unit, property, fixture, and integration tests;
3. deterministic repeated execution;
4. malformed-input and boundary rejection;
5. allocation or lock instrumentation around process paths;
6. a smoke check or manual protocol for behavior that cannot be established by a unit
   test alone; and
7. a dated update to status and traceability.

A design contract may be **accepted** before its code exists. Code may be
**implemented** before its full qualification gate passes. **Verified** is reserved for
the stated acceptance evidence, not general confidence.

## Current deterministic and audio evidence

The current offline fixture renders Pulse → Gain → Saturator through the immutable
compiled plan. Dated evidence records:

- identical repeated renders and deterministic hashes;
- exact silence through the tested path;
- sample-exact impulse behavior;
- bit identity with a hand-wired reference chain;
- finite-value and event-boundary checks in the DSP layer; and
- no allocations or deallocations during measured steady-state plan quanta.

App snapshots are separately tested for owned typed identity, descriptor clamping,
finite values, fail-closed identity matching, deterministic repeats, and an audible-data
path in which edits alter the offline report.

These results are strong evidence for the named offline fixtures. They do not establish
subjective sound quality, a live callback, broad device coverage, plugin compatibility,
recording correctness, or product readiness.

## Realtime evidence

The accepted callback rule prohibits allocation/deallocation, blocking locks, I/O,
logging, formatting, serialization, UI inspection, and panic across the callback
boundary. Control/render communication must be bounded and wait-free, with explicit
overflow behavior and off-thread reclamation. Non-finite values and denormals require
containment rules.

Today, allocation-free and lock-free evidence applies to the compiled plan's measured
offline process loop. R3 must qualify the actual audio backend, callback bridge, MIDI
timestamps, plan publication, health telemetry, and device lifecycle. The offline test
is architectural preparation for realtime work, not a substitute for a callback drill.

## Blind AAA process: proposed, not yet accepted or run

The accepted Spectre quality documents do not yet define an official “blind AAA”
protocol. Until a quality contract does, this phrase should describe only a proposed
external-quality review process—not a badge, milestone, or claim that Spectre matches a
commercial product.

A defensible process would:

1. define one musical task, source material, level-matching method, and defect rubric
   before rendering;
2. produce multiple anonymized outputs, including the Spectre candidate and lawful
   reference/control renders, without disclosing which system produced which;
3. separate technical defects—clicks, instability, timing errors, aliasing, unexpected
   gain—from preference and genre fit;
4. randomize presentation order and collect independent ratings plus free-form reasons;
5. reveal identities only after judgments are locked;
6. publish the task, versions, signal chain, sample rate, evaluator count, conflicts,
   and limitations; and
7. treat the result as evidence for that exact device, preset, task, and build—not
   blanket parity or “professional quality.”

Reference renders must use properly licensed tools and material. The exercise must not
copy presets, proprietary assets, hidden algorithms, or vendor-specific limits into
Spectre. Failed or mixed results are evidence to improve the design, not results to hide.

Before this process can become authoritative, Spectre needs an accepted quality contract
that defines the acronym, sampling and blinding rules, evaluator qualifications,
artifact-retention policy, thresholds, and how subjective results interact with
objective DSP tests.

## Milestone approval versus product readiness

R0/R1 approval means their foundation and kernel exit evidence passed. A future R2 exit
will mean the offline-graph gate passed. Neither means that Spectre can safely record a
session, recover from a crash, host plugins, perform live, or ship to musicians.

Likewise, R4's future “credible alpha” is a deliberately narrow one-track vertical
slice. It is not beta or 1.0. Product-level readiness arrives only after later milestones
close their own risks and after cross-cutting release qualification covers sustained
performance, supported platforms and filesystems, accessibility, recovery drills,
packaging, and documentation.

## Source pointers

Current evidence comes from `docs/status/STATUS.md`,
`docs/01-requirements/traceability.md`, `docs/06-plans/current-milestone.md`, and
`docs/03-architecture/graph-compilation.md`. Realtime rules come from
`docs/01-requirements/requirements-ledger.md` and
`docs/03-architecture/dsp-device-io.md`. Roadmap and release distinctions come from
`docs/06-plans/rebuild-roadmap.md` and `docs/00-product/vision.md`.
