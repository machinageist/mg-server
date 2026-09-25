---
title: "mg-suite"
date: 2026-09-20
summary: "A set of small local-first tools, each with its own data, that talk through explicit interfaces instead of a shared database. More an architecture study than a product."
tags: [rust, architecture, local-first, sqlite, data-ownership]
---

## What this is

`mg-suite` is a collection of small command-line tools. Each one owns one kind of
data, such as notes, plans, calendar events, reminders, contacts, or source
material, and none of them is allowed to reach into another's storage.

I started it to get practice with problems that only show up when a system has
several parts. A single program teaches you very little about ownership, stale
data, or what happens when two components disagree. Six of them, with a rule that
none can read another's database, teach a lot.

This is a hobby project and a study, built with heavy AI assistance. It is not a
product, most of it is unfinished, and I am more confident about the architecture
than about any given line of the code.

## The rule the whole thing is built around

One owner per kind of data. No application reads or writes another's store.
Where two need to cooperate, the data crosses as an explicit request or a
published projection, never as a shared table.

This sounds obvious, and it is tempting to break all the time. As soon as the
calendar wants to show today's reminders, the shortest path is to open the
reminder database and query it. Take that shortcut and neither schema can change
without breaking the other. You also end up with two components that disagree
about what is true, each sure it is right.

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

The last entry is the one I find most useful. It names a boundary that is in the
wrong place, says why it is still there, and warns against making it worse.
Writing that down helped more than pretending the design was clean.

## What owns what

These are the tools that work, meaning their scoped behavior is implemented and
their quality gates pass. The repository has other work that is not finished,
and this page only describes the six that are.

| Tool | Owns | Storage |
|---|---|---|
| `mg-vault` | Notes, concepts, claims, citations, revisions | Markdown files; disposable SQLite index |
| `mg-plan` | Plans, work items, dependencies, acceptance criteria, verification records | SQLite |
| `mg-brief` | Registered sources, fetched artifacts, CVE records, provenance | SQLite |
| `mg-calr` | Calendars, events, time blocks, recurrence, availability | SQLite |
| `mg-remindr` | Todos, projects, tags, lifecycle transitions | SQLite |
| `mg-contacts` | Contact identity, encrypted fields, audit history | Encrypted local store |

The storage column used to be more varied. Two of these ran on PostgreSQL, so a
local-first suite needed a database server set up and running before it could
open a calendar. That was the wrong tradeoff for software meant to run on one
person's machine, and both were moved to SQLite.

The storage is not uniform, but each choice has a reason. Notes are files,
because a notes system whose data you cannot read without its own software has
failed at its main job. Everything relational is a SQLite file under
`$XDG_DATA_HOME`, with no server to install. Contacts are encrypted at rest
because of what they contain.

## Where it stands

`docs/MVP-SCOPE.md` in the repository tracks this, and it is more reliable than
my summary. According to it, `mg-vault`, `mg-plan`, `mg-brief`, and `mg-contacts`
have their scoped MVP behavior and quality gates implemented. `mg-calr` has its
scoped behavior, with persistence verified.

Work that has not reached that bar is not written up here. It can get a page once
it is finished.

The suite's rule for "done" is a low bar on purpose. A tool is MVP-complete when
you can do its core job from the CLI, close it, reopen it, and still find the
authoritative result, with tests covering the happy path, a restart, invalid
input, and the most important safety boundary. None of that needs a daemon, a
dashboard, or a plugin system, and the scope document forbids building those just
to make an MVP feel finished.

## What I built, what I directed, and what I still don't understand

**What I can explain end to end.** The architecture, which is the part I care
about most: why each domain has one owner, why data crosses as projections
instead of shared tables, why the index is disposable, what each boundary
protects, and which boundary (`mg-remindr`) is in the wrong place and why it is
still there. The storage choices and the reasons for them. What "done" means for
each tool and why the bar is where it is.

**What I directed rather than wrote.** Nearly all of the Rust. I specified the
behavior, the boundaries, and the failure cases. Agents wrote the
implementations, and I reviewed them, tested them, and sent work back. These six
tools are tens of thousands of lines, and I did not type most of them.

**What I do not understand yet.** Large parts of the implementation in detail. I
could not sit down and rewrite `mg-calr`'s recurrence handling from memory. Async
Rust past the point of using it. Lifetimes and ownership in async code are still
something I work through each time. SQLite's behavior below the query layer. I
have read about transaction isolation and WAL, but I have not had to reason about
them under pressure. The cryptography in `mg-contacts`. It encrypts fields, and I
am not qualified to say whether the scheme is sound.

Lower-level Rust is where I want to end up. `mg-server`, the application behind
this site, which I wrote and can explain line by line, is where I am working on
that. This suite is the wider counterpart. It taught me architecture. It did not
teach me Rust.

## Status

In progress. The six tools above work, and I use some of them daily. The
projections between applications have barely started. Other work in the
repository has not reached the bar this page describes, so it is not described
here.

The umbrella repository is public for its documentation: the architecture, the
scope limits, and the reasoning behind the boundaries. The individual application
repositories stay private until they are finished enough that I can defend what
is in them.

## Related writing

- [Markdown is the database](/blog/markdown-is-the-database) — why mg-vault's
  SQLite index is disposable, and what it took to keep it that way.

## Source

[github.com/machinageist/mg-suite](https://github.com/machinageist/mg-suite) is
the umbrella repository. It holds documentation: the architecture, the scope
limits, and the reasoning behind the boundaries. The application repositories
are gitignored from it and are private.

The desktop that launches these tools is [documented separately](/portfolio/geistos).
