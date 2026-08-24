---
title: "Designing mg-calr: Calendar Correctness Before Calendar Polish"
date: 2026-08-23
summary: "A working draft on designing a Rust calendar, reminder, and todo system around recurrence integrity, explicit synchronization, and testable terminal interfaces."
tags: [rust, calendar, caldav, postgresql, cli, local-first]
---

I started designing `mg-calr` because I wanted a calendar that belongs naturally
in a terminal-centered Linux workstation without becoming isolated from the
calendar I use on macOS.

That sounds like an interface problem: build a calendar view, add event forms,
and synchronize it. The design work showed that the difficult part comes first.
Calendar software is a temporal data-integrity system. Recurrence, time zones,
conflicts, reminders, and round-trip interoperability can silently damage data
long before a view looks wrong.

This draft records the product contract and the proposed implementation
boundary. The current `mg-calr` repository contains only an initial foundation
slice; the behavior below is planned scope unless explicitly identified as
verified.

## The authority model

`mg-calr` uses PostgreSQL as application authority. Interoperability with iCloud
uses a durable vdir mirror and `vdirsyncer`:

```text
PostgreSQL
    │
    │ project and reconcile
    ▼
durable vdir mirror
    │
    │ vdirsyncer
    ▼
iCloud CalDAV
```

The mirror persists because `vdirsyncer` needs file and synchronization metadata,
but it is not a second application authority.

Synchronization is explicit:

```sh
mg-calr sync
```

Ordinary local creation, editing, display, search, reminder scans, and todo work
do not initiate network access. An explicit sync projects local state, exchanges
calendar files, imports remote changes, and reconciles the result.

## Why conflict handling is not last-write-wins

Two machines can edit the same event before the next synchronization. Choosing
the newest timestamp or always preferring PostgreSQL would be simple, but it
could erase a legitimate edit.

The intended conflict model preserves:

- the common base;
- the local version; and
- the remote version.

Reconciliation stops for the affected item until the conflict is resolved
explicitly. This costs more implementation work, but the alternative is hidden
calendar corruption.

## Recurrence is a first-class domain

A repeating event is not a pile of copied rows. It is a series definition plus
occurrences, exceptions, and scoped mutations.

The product contract includes full RRULE behavior, including:

- bounded occurrence expansion;
- excluded and overridden occurrences;
- edits to one occurrence;
- edits to this and future occurrences;
- edits to a whole series;
- scoped deletions;
- stable event UIDs; and
- timezone and daylight-saving transitions.

Unbounded future instances must not be materialized. A daily event without an
end date cannot be expanded forever into the database.

This domain receives the highest gauntlet weight because a convenient interface
cannot compensate for an event drifting by an hour or an exception disappearing.

## Events and todos remain distinct

Events and todos are separate domain types.

Events can have zero or multiple reminders. An event is allowed to live silently
on the calendar.

Todos are local-only in the first release. They support:

- projects or lists;
- multiple tags;
- priority and notes;
- due date or time;
- completion history;
- recurrence;
- multiple reminders;
- nested subtasks; and
- a separate acyclic dependency graph.

Hierarchy and dependency are not interchangeable. A subtask says that work is
part of a larger item. A dependency says that one item is blocked by another.
Blocked work remains visible, but its reminders are suppressed until it becomes
actionable.

Recurring todos create fresh instances. Resetting the same row would destroy
completion history and make the audit trail difficult to trust.

## Reminder delivery as durable work

Reminder delivery is split between two systemd user components:

1. A one-minute timer scans and durably claims due reminders.
2. A small persistent service sends notifications and owns the callback actions
   for dismiss and snooze.

This division matters because notification buttons depend on a living process
that can receive DBus actions.

The scanner must be idempotent. Timer overlap, crashes, restarts, sleep, and
shutdown must not produce duplicate delivery. After downtime, the system should
notify once for reminders that are still relevant rather than replaying every
missed scan.

Desktop do-not-disturb state suppresses presentation, not durable reminder
state.

## Terminal interface contract

The CLI is both a human interface and an integration boundary.

Interactive creation prompts for missing fields. Every prompt has equivalent
flags, and `--no-input` makes scripts fail rather than hang waiting for input.

Output behavior:

- human-readable by default;
- stable JSON for automation and Quickshell;
- explicit no-color support;
- respect for `NO_COLOR`;
- deterministic identifier and ambiguity behavior.

Planned views include daily, weekly, and monthly terminal presentations. A full
screen TUI comes after the CLI and JSON contracts are stable.

The executable is named `mg-calr` rather than `cal`, because Arch already ships
`/usr/bin/cal` through `util-linux`.

## iCalendar preservation

The first release does not pretend that exchanging `.ics` files implements the
full invitation and RSVP workflow from CalDAV Scheduling.

It does require attendee and scheduling properties to survive round trips. Full
invitation and RSVP actions are a later branch. Preserving unsupported
properties now keeps that later work possible and avoids damaging events created
by other clients.

Credentials must come from a configurable external secret command. They do not
belong in the repository, generated configuration, logs, or diagnostics.

## Quality gates

The proposed Spec Gauntlet weights are:

1. Temporal and Data Integrity — 35%
2. CLI Usability and Automation — 25%
3. Standards Interoperability and Sync — 25%
4. Operational Security and Reliability — 15%

The benchmark set is Apple Calendar, `khal` with `vdirsyncer`, Taskwarrior, and
calcurse.

Automatic failures include:

- silent data loss;
- unconfirmed overwrite;
- event UID or recurrence corruption;
- timezone or DST drift;
- duplicate reminder delivery;
- plaintext or logged credentials;
- implicit network access;
- automatic conflict overwrite; and
- lossy iCalendar round trips.

## Build order

The implementation is intentionally dependency-ordered:

1. configuration, PostgreSQL migrations, domain errors, and the test harness;
2. event CRUD, calendars, time zones, views, and JSON;
3. recurrence and series mutation;
4. todos and dependency graphs;
5. reminder scanning and notification actions;
6. search, bulk dry-runs, audit history, and undo;
7. lossless iCalendar and JSON import/export;
8. vdirsyncer reconciliation;
9. backup, doctor, documentation, and packaging;
10. TUI;
11. Quickshell card; and
12. full scheduling/invitations.

Each deployable slice is tested, independently reviewed, graded against the
accepted criteria, corrected if necessary, and committed before the next domain
boundary expands.

## Desktop integration

The eventual Quickshell pill and card consume public `mg-calr` JSON. They never
query PostgreSQL.

The pill provides quiet next-event or reminder state. The card handles common
inspection and actions. The TUI remains the deep workflow, and the CLI remains
the automation and recovery interface.

This separation is the central design lesson: calendar correctness belongs in
the application domain, not in whichever UI happens to display it.
