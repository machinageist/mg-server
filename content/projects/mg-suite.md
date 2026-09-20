---
title: "mg-suite"
date: 2026-09-20
summary: "A set of small local-first tools, each owning its own data and talking through explicit boundaries rather than a shared database. An architecture study more than a product."
tags: [rust, architecture, local-first, sqlite, postgresql, data-ownership]
---

## What this is

`mg-suite` is a collection of small command-line tools that each own one kind of
data — notes, plans, calendar events, reminders, contacts, source material — and
that are deliberately forbidden from reaching into each other's storage.

It started as a way to get exposure to systems-shaped problems by building
something with enough parts to have real boundaries. A single program teaches
you very little about ownership, staleness, or what happens when two components
disagree. Seven of them, with a rule that none may read another's database,
teaches you a great deal.

That framing is the honest one. This is a hobby project and a study, built with
heavy AI assistance. It is not a product, most of it is unfinished, and I am
more confident about the architecture than about any particular line of the
implementation.

## The rule the whole thing is built around

One owner per kind of data. No application reads or writes another's store.
Where two need to cooperate, the data crosses as an explicit request or a
published projection, never as a shared table.

This sounds obvious and is constantly tempting to break. The moment the
calendar wants to show today's reminders, the shortest path is to open the
reminder database and select from it. That shortcut is how you end up unable to
change either schema, and how you end up with two components that disagree about
what is true while both being certain.

The boundaries as written in the suite README:

- **mg-brief** explains and preserves provenance. It never remediates. It does
  not own curated knowledge, project work, or task completion.
- **mg-vault** keeps Markdown as the authoritative representation. Its index is
  a disposable projection and is never the source of truth.
- **mg-plan** owns verification judgments, not the facts inside the evidence. It
  does not own raw evidence, repositories, CI, or calendar events.
- **mg-plan** owns work and scheduling *intent*; **mg-calr** owns the resulting
  temporal allocation.
- **mg-remindr** is a transitional authority kept for compatibility while
  planning ownership moves to mg-plan.

That last one is the interesting entry, because it is an admission rather than a
design. It names a boundary that is in the wrong place, says why it is still
there, and warns against deepening it. Writing that down was more useful than
pretending the design was clean.

## What owns what

| Tool | Owns | Storage |
|---|---|---|
| `mg-vault` | Notes, concepts, claims, citations, revisions | Markdown files; disposable SQLite index |
| `mg-plan` | Plans, work items, dependencies, acceptance criteria, verification records | SQLite |
| `mg-brief` | Registered sources, fetched artifacts, CVE records, provenance | SQLite |
| `mg-calr` | Calendars, events, time blocks, recurrence, availability | PostgreSQL |
| `mg-remindr` | Todos, projects, tags, lifecycle transitions | PostgreSQL |
| `mg-contacts` | Contact identity, encrypted fields, audit history | Encrypted local store |
| `mg-calcr` | Expression evaluation and graph sampling | Stateless |

The storage column is not uniform, and that is on purpose. Notes are files
because a notes system whose data you cannot read without its own software has
failed at the one job. Calendars are PostgreSQL because recurrence and
availability queries are genuinely relational. The calculator stores nothing
because it has nothing worth keeping.

## Where it actually stands

`docs/MVP-SCOPE.md` in the repository tracks this, and it is the source I would
point at rather than my own summary. As it records: `mg-vault`, `mg-plan`,
`mg-brief`, and `mg-contacts` have their scoped MVP behaviour and quality gates
implemented; `mg-calr` has its scoped behaviour with persistence verification.

Several other tools in the tree are early — some are a single file and one
commit. The repository holds more directories than it holds finished software,
and I would rather say that here than let a directory listing imply otherwise.

The suite rule for "done" is worth repeating because it is a low bar
deliberately set: a tool is MVP-complete when you can do its core job from the
CLI, close it, reopen it, and still find the authoritative result, with tests
covering the happy path, restart, invalid input, and the most important safety
boundary. Nothing about that requires a daemon, a dashboard, or a plugin system,
and the scope document explicitly forbids building those to make an MVP feel
finished.

## What I built, what I directed, and what I still don't understand

**What I can explain end to end.** The architecture, which is the part I
actually care about: why one owner per domain, why projections rather than
shared tables, why the index is disposable, what each boundary is protecting,
and which boundary (`mg-remindr`) is in the wrong place and why it is still
there. The storage choices and their reasoning. What "done" means for each tool
and why the bar is set where it is.

**What I directed rather than wrote.** Nearly all of the Rust. I specified
behaviour, boundaries, and failure cases; agents wrote the implementations; I
reviewed, tested, and sent work back. Roughly fifty thousand lines exist in this
tree and I did not type most of them.

**What I do not understand yet.** Substantial parts of the implementation in
detail — I could not sit down and reproduce `mg-calr`'s recurrence handling from
memory. Async Rust beyond using it: lifetimes and ownership in async contexts
are still something I work through rather than know. The SQLite and PostgreSQL
behaviour underneath the query layer — transaction isolation in particular is
something I have read about and not yet had to reason about under pressure.
Cryptographic review of `mg-contacts`: it encrypts fields, and I am not
qualified to tell you the scheme is sound.

Lower-level Rust is where I want to end up, and `mg-server` — this site, which I
did write and can explain line by line — is the work where I am actually getting
there. This suite is the wider-scope counterpart: it taught me architecture, and
it did not teach me Rust.

## Status

In progress. Some tools are usable daily; most are not finished; the
cross-application projections are barely started. The umbrella repository is
public for its documentation — the architecture, the scope fence, and the
boundary reasoning. The individual application repositories stay private until
they are finished enough that I would be comfortable defending what is in them.

## Related writing

- [Markdown is the database](/blog/markdown-is-the-database) — why mg-vault's
  SQLite index is disposable, and what it took to mean it.

## Source

[github.com/machinageist/mg-suite](https://github.com/machinageist/mg-suite) —
the umbrella repository. What is there is documentation: the architecture, the
scope fence, and the boundary reasoning. The application repositories are
gitignored from it and remain private.

The desktop that launches these tools is
[documented separately](/portfolio/geistos).
