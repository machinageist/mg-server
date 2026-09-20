---
title: "Markdown Is the Database"
date: 2026-09-20
summary: "A notes tool where files are authoritative and the SQLite index is disposable. What that rule costs to actually mean, and the failure modes it forces you to name out loud."
category: "Linux / SysAdmin"
tags: [sqlite, local-first, data-ownership, architecture, markdown]
---

I have lost notes to a notes application before. Not to a disk failure — to a
database the application could no longer open, holding content I could not get
at without the application that had broken.

So when I built a notes tool for my own use, the first rule was that the
Markdown files are the truth and anything else is a cache. This post is about
what it costs to mean that, because saying it is easy and the rule breaks the
moment you want search to be fast.

## The rule

Markdown files in a directory are authoritative. There is a SQLite index for
search. Deleting the index and rebuilding it from unchanged sources must produce
equivalent, identically ordered results.

That last clause is what turns a slogan into something testable. "The index is
disposable" is a claim about behaviour, and if `rm index.db && rebuild` gives
you different search results, the claim is false and your files were quietly not
the source of truth after all.

## Why the obvious design fails

The natural way to build this is: index on write. The tool edits a note, so it
updates the file and the index in the same operation. They stay in sync because
nothing changes one without the other.

This works until something changes a file without going through the tool, which
is immediately and constantly. The entire point of files-as-truth is that I can
open them in Neovim, or `sed` across them, or restore one from a backup, or sync
the directory between machines. Every one of those edits a note without telling
the index.

So the index cannot assume it is current. It has to be able to find out.

## Fingerprints and generations

Two mechanisms carry most of the weight.

**Source fingerprints.** Every note has a SHA-256 of its content recorded when
it was read. To replace a note you must supply the fingerprint you read, and the
write is rejected if the file has changed since. This is optimistic concurrency,
and it turns the dangerous case — two things editing the same note, last write
silently winning — into a visible error.

**Index generations.** The index is written as atomic generations with explicit
schema and parser versions, rather than mutated row by row. A rebuild produces a
new generation; readers see the old one until the new one is complete. A crashed
rebuild leaves the previous generation intact, because a half-built index that
looks complete is worse than an obviously stale one.

The schema and parser versions matter more than they look. When the parser
changes, every row it produced is suspect — not wrong necessarily, but produced
by different code. Recording the version means the tool can notice and rebuild
rather than serving rows from a parser that no longer exists.

## Four kinds of not-current

The part that changed how I think about caches: "stale" is not one state.

| State | Meaning |
|---|---|
| `empty` | No index has been built |
| `current` | Index matches an observation of the sources |
| `stale` | Sources have changed since the index was built |
| `degraded` | The index exists but a scan did not complete |

`degraded` is the one I would not have thought to include. It covers a scan that
started, failed partway, and left an index that is neither the old complete
state nor a new complete one. Collapsing that into `stale` would be a lie by
omission — stale implies the index is internally consistent and merely behind.
Degraded means you cannot trust it to be either.

Naming four states is not pedantry. Each one has a different correct response,
and a tool that reports one word for all of them cannot tell you which.

## Observe before trusting

The rule that costs the most performance: `status` and `search` perform a fresh,
confined observation of the source files before treating a persisted generation
as current.

The cheap version would trust the stored generation and report `current`. That
is fast and wrong, because the whole premise is that files change without the
tool's involvement.

And the related rule: observed drift never silently publishes candidate rows or
advances the generation. When the tool notices sources have moved, it reports
that. It does not quietly index what it found and tell you everything is fine.
A cache that repairs itself invisibly is indistinguishable from a cache that is
lying, and the point of this design was to never be in that position.

The cost is real. Every status check walks the vault. For my note count that is
not noticeable; at a much larger scale it would be, and the answer would be a
watcher daemon with a dirty queue — which the scope document explicitly defers
rather than pretending the current design scales.

## Confinement

Separate from the truth question, and worth stating because it is the part with
security consequences: the vault is a directory, and operations are confined to
it. Path traversal is rejected. Symlinks that escape the vault are rejected.
Application-internal paths like `.obsidian` are protected from mutation.

A notes tool takes paths from user input and writes files. Without confinement,
a crafted note path is a write primitive anywhere the process can reach. This
needed to be a tested boundary rather than a careful habit.

## What I would tell myself at the start

- **Decide what is authoritative, then check whether your code agrees.** Writing
  "files are the source of truth" in a README is free. `rm index.db`, rebuild,
  and diff the results is the version that means something.
- **A cache must be able to discover it is wrong.** If the only way the index
  learns about a change is being told by the code that made it, it will be wrong
  the first time anything else touches a file.
- **Enumerate the failure states.** `empty`, `stale`, and `degraded` are
  different situations with different responses. One catch-all word throws away
  the information the user needs.
- **Never repair silently.** Report the drift. A self-healing cache and a lying
  cache look identical from outside.
- **Rebuild atomically.** Partial state that presents as complete is the worst
  outcome available.

## Honest limits

This is a hobby project built with heavy AI assistance. I specified these rules,
reviewed the implementations, and can explain why each mechanism is there — that
is the part I will defend. I did not write most of the Rust, and there are
corners of it I could not reproduce from memory.

I also have not tested this at a scale where the observe-before-trusting cost
bites, so I know the design has a ceiling without knowing where it is. And the
invariant I am most confident about is the one I can actually check:

```text
rm -f index.db && index rebuild && search <term>
```

Same results as before, or the premise is wrong.
