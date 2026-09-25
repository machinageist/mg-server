---
title: "Markdown Is the Database"
date: 2026-09-20
summary: "A notes tool where the Markdown files are authoritative and the SQLite index can be thrown away. What that rule costs, and the failure states it made me name."
category: "Linux / SysAdmin"
tags: [sqlite, local-first, data-ownership, architecture, markdown]
---

I have lost notes to a notes application before. The disk was fine. The
application's database could no longer be opened, and the notes were stuck
inside it.

So when I built a notes tool for my own use, the first rule was that the
Markdown files are the truth and everything else is a cache. Saying that is
easy. Keeping it true gets harder once you want search to be fast, and this post
is about what that took.

## The rule

Markdown files in a directory are authoritative. There is a SQLite index for
search. Deleting the index and rebuilding it from unchanged files must give the
same results, in the same order.

That last part is what makes the rule testable. If `rm index.db && rebuild`
gives different search results, the index was holding something the files were
not, and the files were not really the source of truth.

## Why the obvious design fails

The obvious way to build this is to update the index on every write. The tool
edits a note, so it updates the file and the index together. They stay in sync
because nothing changes one without the other.

That lasts until something changes a file without going through the tool, which
happens all the time. The reason to keep notes as files is so I can open them in
Neovim, run `sed` across them, restore one from a backup, or sync the directory
between machines. Every one of those changes a note without telling the index.

So the index cannot assume it is current. It has to be able to check.

## Fingerprints and generations

Two mechanisms do most of the work.

**Source fingerprints.** Every note has a SHA-256 of its content, recorded when
it was read. To replace a note you have to supply the fingerprint you read, and
the write is rejected if the file has changed since. This is optimistic
concurrency. When two things edit the same note, the second write fails with an
error instead of silently overwriting the first.

**Index generations.** The index is written as complete generations, each with
an explicit schema version and parser version, instead of being changed row by
row. A rebuild produces a new generation, and readers keep seeing the old one
until the new one is finished. If a rebuild crashes, the previous generation is
still intact. I would rather have an index that is obviously out of date than a
half-built one that looks complete.

The version numbers are there for when the parser changes. Rows from the old
parser might still be right, but different code produced them. Because the
version is recorded, the tool can notice the change and rebuild instead of
serving rows from a parser that no longer exists.

## Four kinds of not-current

I used to think of a cache as either current or stale. This project needed four
states:

| State | Meaning |
|---|---|
| `empty` | No index has been built |
| `current` | Index matches an observation of the sources |
| `stale` | Sources have changed since the index was built |
| `degraded` | The index exists but a scan did not complete |

I would not have thought to include `degraded`. It covers a scan that started,
failed partway through, and left an index that is neither the old complete state
nor a new complete one. Calling that `stale` would hide something. A stale index
is consistent and just behind. A degraded one might not be consistent at all.

Each state needs a different response, which is why each needs its own name. A
tool that reports one word for all four cannot tell you what to do next.

## Observe before trusting

The rule that costs the most performance is that `status` and `search` look at
the source files again, inside the vault, before treating a saved generation as
current.

The cheaper option would be to trust the saved generation and report `current`.
That would be fast, and it would be wrong every time a file changed outside the
tool, which is the case this design exists for.

A related rule: when the tool notices the files have changed, it reports that.
It does not index the changes in the background, publish the new rows, and say
everything is fine. From the outside, a cache that fixes itself without saying
so looks the same as one that is wrong.

The cost is real. Every status check walks the whole vault. With the number of
notes I have, I do not notice it. At a much larger scale I would, and the fix
would be a watcher daemon with a queue of changed files. The scope document puts
that off for later.

## Confinement

This is separate from the source-of-truth question, and it is the part with
security consequences. The vault is a directory, and every operation is confined
to it. Path traversal is rejected. Symlinks that point outside the vault are
rejected. Application paths like `.obsidian` cannot be modified.

A notes tool takes paths from user input and writes files. Without confinement,
a crafted note path could write anywhere the process can reach. So confinement
is enforced in code and covered by tests.

## What I would tell myself at the start

- **Decide what is authoritative, then check that the code agrees.** Writing
  "files are the source of truth" in a README costs nothing. Deleting the index,
  rebuilding, and diffing the results is how you find out whether it is true.
- **A cache has to be able to find out it is wrong.** If the index only learns
  about a change from the code that made it, it will be wrong the first time
  anything else touches a file.
- **List the failure states.** `empty`, `stale`, and `degraded` are different
  situations with different responses. One word for all of them loses the
  information the user needs.
- **Report drift instead of repairing it silently.**
- **Rebuild atomically.** A partial index that looks complete is the worst result.

## Limits

This is a hobby project built with heavy AI assistance. I specified these rules,
reviewed the implementations, and can explain why each mechanism is there. I did
not write most of the Rust, and there are parts of it I could not reproduce from
memory.

I also have not tested it at a scale where checking the files on every status
call gets slow. I know the design has a ceiling, but I do not know where it is.
The rule I am most sure of is the one I can check directly:

```text
rm -f index.db && index rebuild && search <term>
```

If the results are not the same as before, the design is wrong.
