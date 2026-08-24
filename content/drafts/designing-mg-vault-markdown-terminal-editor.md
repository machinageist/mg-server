---
title: "Designing mg-vault: Plain Markdown With a Serious Terminal Editor"
date: 2026-08-23
summary: "A working draft on designing an Obsidian-compatible Rust knowledge system where files remain authoritative and every derived feature is rebuildable."
tags: [rust, markdown, tui, local-first, knowledge-management, obsidian]
---

I wanted an Obsidian-style knowledge system that fit the rest of my terminal
workflow without turning Markdown into an export format from a hidden database.
That requirement became the central design rule for `mg-vault`:

> Markdown and attachments are authoritative. Everything derived from them must
> be rebuildable.

This draft records the product design and the proposed implementation
boundary. The current `mg-vault` repository contains only an initial file-
authority foundation slice; the behavior below is planned scope unless
explicitly identified as verified.

## A vault remains an ordinary directory

`mg-vault` supports multiple independent vaults. Each one is a normal directory
that can be edited with another tool, synchronized with ordinary filesystem
tools, backed up, inspected with shell commands, and opened in Obsidian.

Portable product configuration lives under `.mg-vault/`. Obsidian's
`.obsidian/` directory is preserved and never rewritten.

The rebuildable SQLite index lives under XDG paths outside the vault. It can
accelerate:

- full-text and fuzzy search;
- headings and blocks;
- backlinks and unresolved links;
- tags and typed properties;
- Markdown tasks;
- attachment text extraction; and
- graph relationships.

Deleting the index must never delete knowledge. If the index disagrees with the
files, the files win.

## Why paths remain identity

Automatically inserting a hidden UUID into every note would make rename tracking
easier, but it would also modify ordinary files solely to satisfy one
application.

The selected contract keeps file paths as public note identity. The index may
use content fingerprints to recognize changes, and an author may define an `id`
property explicitly, but `mg-vault` does not inject one.

Duplicate titles remain legal. When a title or alias is ambiguous, the
application asks which note was intended. It does not silently pick the nearest
folder and change the meaning of a link.

## Editing is part of the product

This is not only a search index wrapped around Neovim. The TUI includes its own
terminal editor with a bounded, documented Vim grammar:

- normal, insert, and visual modes;
- operators and motions;
- text objects and counts;
- registers and macros;
- marks and jumps;
- search and repeat; and
- a command line.

The goal is coherence, not bug-for-bug Vim emulation.

The editor also includes:

- tabs and arbitrary split panes;
- synchronized source and preview;
- Tree-sitter structure, highlighting, folds, and outline;
- persistent undo and registers;
- system clipboard and OSC52 fallback;
- spellcheck and optional LanguageTool diagnostics;
- crash-recovery journals; and
- atomic autosave.

Neovim remains a first-class escape hatch. A note, selection, or fenced code
block can round-trip through `$EDITOR` and return through the same conflict and
atomic-write safeguards.

## External edits are expected

Git, Syncthing, Obsidian, Neovim, and shell tools can all change a vault while it
is open.

A clean in-memory buffer reloads automatically. A dirty buffer does not. The
application preserves both versions and uses the last common file content for a
three-way merge.

That policy follows directly from file authority. Locking every note would make
other tools second-class citizens. Reloading over unsaved work would lose data.
Silently choosing a winner would make synchronization untrustworthy.

## Links and safe refactoring

The target syntax includes Obsidian-compatible wikilinks, aliases, heading and
block references, tags, embeds, callouts, and YAML properties.

The index provides:

- backlinks;
- outgoing links;
- unresolved links;
- unlinked mentions;
- heading and block relationships; and
- local and global graphs.

Rename and move operations are multi-file transactions. The application first
shows a dry-run of affected files and inbound links. It then commits every file
change or rolls the transaction back.

The same requirement applies to structural refactors:

- extract selection into a note;
- split a note at headings;
- merge notes; and
- move a section between notes.

A convenient refactor that leaves half the vault rewritten is a data-loss bug.

## Typed properties without database authority

YAML properties can use optional per-vault schemas with types, defaults,
aliases, validation, and migrations. Unknown keys and formatting must survive
edits.

The design also targets Obsidian Bases-style derived views:

- table;
- list;
- cards;
- board;
- calendar;
- timeline; and
- map.

Formulas, relations, inverse relations, and rollups are derived from ordinary
properties. Cycles are detected explicitly. Editing through a view writes back
to Markdown through the normal safe transaction boundary.

## Canvas in a terminal

Canvas uses the open JSON Canvas 1.0 format. Text, file, link, group, and edge
information stays in `.canvas` files rather than being hidden in the index.

The TUI needs spatial navigation, but spatial presentation cannot be the only
way to understand the data. Every Canvas and graph must also have an ordered,
keyboard-navigable textual representation.

This is both an accessibility requirement and a useful terminal design
constraint.

## Notes and calendar todos

Markdown checkboxes remain note-local. They can be searched, queried, and
toggled without becoming records in another application.

An explicit command can promote or link a checkbox to an `mg-calr` todo. After
promotion, `mg-calr` is authoritative for the todo. The two applications use
public command or JSON contracts; neither opens the other's database.

This avoids accidental dual authority while preserving the useful workflow of
turning a note into scheduled work.

## Plugins and AI

The planned extension system uses sandboxed WASM and explicit external-command
adapters. Capabilities such as filesystem scope, network access, process
execution, clipboard access, index queries, and note mutation are denied by
default and granted individually.

AI follows the same boundary:

- local adapters are preferred;
- cloud adapters operate only on explicitly selected notes or blocks;
- the exact payload and provider are shown before transmission;
- there is no ambient whole-vault cloud retrieval; and
- generated edits remain proposed diffs until explicitly applied.

## Git, Syncthing, and recovery

The application does not invent a proprietary synchronization protocol. Git and
Syncthing operate on the ordinary vault files.

Git integration can create configurable checkpoints and show status, diff,
history, and restore operations. It never pushes automatically.

Deletion goes through vault-local trash and recoverable history. Permanent purge
is explicit. Concurrent edit conflicts preserve both versions.

## Quickshell integration

The persistent pill stays minimal:

```text
vault icon + inbox or unprocessed count
```

Accent appears only when indexing, Git, synchronization, or recovery needs
attention.

The card provides:

- quick capture;
- append;
- daily note;
- recent and pinned notes;
- inbox;
- search; and
- health status.

Substantial editing opens the TUI. Quickshell uses stable JSON and commands; it
never opens the SQLite index or mutates vault files itself.

## Quality gates

The proposed Spec Gauntlet weights are:

1. Data Ownership and Format Compatibility — 30%
2. Editor/TUI Excellence — 25%
3. Knowledge Retrieval and Structure — 20%
4. Security, Reliability, and Extensibility — 15%
5. Performance and Accessibility — 10%

The benchmark profile combines Obsidian, Neovim, Logseq, and Notion while keeping
ordinary files and offline ownership.

Automatic failures include:

- source content loss;
- partial multi-file rewrites;
- index state overriding files;
- silent conflict selection;
- loss of unknown syntax;
- destructive imports;
- plugin capability bypass;
- plugin or AI exfiltration; and
- spatial information with no textual equivalent.

The performance target is at least 100,000 notes and 1,000,000 indexed blocks
without blocking editing.

## Build order

The dependency order begins below the visible features:

1. safe paths, file authority, atomic writes, and recovery;
2. index service and search;
3. CLI operations;
4. editor engine;
5. TUI workspace;
6. links, graph, and atomic refactoring;
7. properties, templates, periodic notes, and task links;
8. rich content;
9. Bases;
10. Canvas;
11. Git and synchronization recovery;
12. plugins;
13. clipping, imports, publishing, and AI; and
14. Quickshell integration and packaging.

That order is deliberate. A beautiful graph or editor cannot compensate for a
write path that can damage the files it is supposed to protect.
