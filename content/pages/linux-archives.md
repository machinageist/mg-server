---
title: "Archives and compression"
date: 2026-08-14
summary: "Why gzip and tar are separate tools, the positional tar flags worth memorising, what metadata an archive preserves that cp does not, and decompressing through a pipe."
tags: [education, linux, archives, tar, gzip, compression]
---

## Overview

Two jobs get confused because one command usually does both. **Compression**
makes a file smaller. **Archiving** bundles many files into one. On Linux they
are separate tools that compose, which is why the canonical filename is
`archive.tar.gz` — a tar archive that has then been gzipped.

The reason to care beyond disk space is metadata. Ownership, permission bits,
timestamps, and symbolic links are properties the filesystem holds *about* a
file, not content inside it. Copy a directory tree the naive way and you can
lose all of them. An archive carries them along, which makes `tar` the right
tool for moving a tree between machines intact — and the wrong thing to skip
when the tree contains anything whose
[permissions](/learn/linux-permissions) matter.

## gzip compresses one file

```text
$ gzip logfile.txt        # produces logfile.txt.gz, removes the original
$ gunzip logfile.txt.gz   # restores it
```

Note that `gzip` replaces the original rather than sitting alongside it. That
surprises people once. Use `gzip -k` to keep the source, and `gzip -l` to see
the compression ratio without decompressing.

It compresses; it does not bundle. `gzip` on a directory does nothing useful,
because there is no single stream of bytes to compress. That is `tar`'s job.

## tar bundles many files

```text
$ tar cvf archive.tar dir1 dir2 file.txt   # create
$ tar tvf archive.tar                      # list contents, change nothing
$ tar xvf archive.tar                      # extract
```

The flags are positional and predate the convention of writing a dash:

| Flag | Meaning |
|---|---|
| `c` | create a new archive |
| `x` | extract from an archive |
| `t` | list contents without extracting |
| `v` | verbose — name each file as it is processed |
| `f` | the next argument is the archive filename |

`f` must come last of the letters, because the filename follows it immediately.
`tar cvf archive.tar dir/` works; `tar cfv archive.tar dir/` tries to use `v` as
the filename.

The habit worth building is running `tar tvf` before `tar xvf`. A well-made
archive expands into a single directory. A badly made one scatters files into
whatever directory you are standing in, and there is no undo.

## Combining the two

Modern `tar` will compress as it archives:

```text
$ tar czf archive.tar.gz dir/    # create and compress
$ tar xzf archive.tar.gz         # decompress and extract
```

`z` selects gzip. `j` selects bzip2 and `J` selects xz, both of which compress
harder and more slowly. GNU tar can also infer the algorithm from the filename
with `-a` on create, or detect it automatically on extract, so `tar xf` usually
works whatever the extension.

Compressing an existing archive separately produces exactly the same result:

```text
$ gzip archive.tar               # archive.tar.gz
```

## Decompressing through a pipe

To read a compressed archive without writing the decompressed copy to disk,
`zcat` decompresses to standard output and `tar` reads from the pipe:

```text
$ zcat archive.tar.gz | tar xvf -
```

The `-` is the archive filename, meaning "standard input." This is ordinary
[stream plumbing](/learn/linux-streams#pipes) applied to a case where the
intermediate file would be large and pointless — which matters when the archive
is bigger than the free space you have.

`zcat` is `gzip -dc`: decompress, write to stdout. The same pattern works for
`bzcat` and `xzcat`.

## What survives, and what does not

`tar` preserves modes, ownership, timestamps, and symbolic links. Two caveats
are worth knowing before trusting that:

- Ownership is restored by numeric UID and GID. Extracting as an ordinary user
  gives everything to *you*, because a non-root user cannot give a file away.
  Only root extracting with `-p --same-owner` reproduces the original ownership.
- Access control lists, SELinux contexts, and extended attributes need
  `--acls`, `--selinux`, and `--xattrs` respectively. They are not included by
  default.

`cp -a` is the equivalent for a local copy — archive mode, preserving the same
metadata without producing a file.

## Suggested practice: prove what an archive preserves

Do this in a scratch directory. Nothing here touches the system.

1. Create a few files, set distinctive permissions on each with `chmod`, and add
   a symbolic link among them. Record the `ls -l` output.
2. Run `tar czf test.tar.gz .`, extract into a fresh directory, and compare
   `ls -l` against what you recorded. Confirm the modes and the link survived.
3. Repeat with `cp -r` instead of tar, then with `cp -a`. Note precisely what
   each one lost.
4. Run `tar tvf test.tar.gz` and read the listing. Check whether the paths are
   relative or absolute, and whether extracting would create a directory or
   scatter files.
5. Compare sizes: `ls -l` the original directory, the `.tar`, and the `.tar.gz`.
   Then try `-j` and `-J` and compare both size and how long each took.
6. Extract through a pipe with `zcat test.tar.gz | tar xvf -` and confirm it
   produces the same result as `tar xzf`.
7. Compress an already-compressed file — a `.jpg` or a `.gz` — and check the
   size. Compression works on redundancy, and there is none left to find.

## Related pages

- [File permissions and links](/learn/linux-permissions) — the modes and
  symbolic links an archive is preserving.
- [Streams, redirection, and pipes](/learn/linux-streams) — the pipe the `zcat`
  example depends on.
- [The Linux filesystem hierarchy](/learn/linux-filesystem-hierarchy) — why
  `/var` fills up, and what you will end up archiving out of it.

## Sources and further reading

This page was edited from my own study notes, taken from Brian Ward's *How Linux
Works: What Every Superuser Should Know* (No Starch Press), and checked against
the primary documentation:

- [`tar(1)`](https://man7.org/linux/man-pages/man1/tar.1.html) — the full flag
  set, including which metadata is preserved and which needs asking for.
- [`gzip(1)`](https://man7.org/linux/man-pages/man1/gzip.1.html) — compression
  levels, `-k`, and the `zcat` equivalence.
- [RFC 1952: GZIP file format specification](https://www.rfc-editor.org/rfc/rfc1952.txt)
  — what is actually in a `.gz` header.
- [`cp(1)`](https://man7.org/linux/man-pages/man1/cp.1.html) — what `-a`
  preserves, for comparison.

GNU tar and BSD tar differ in their handling of long options and some
extensions, so a command copied from a Linux answer may behave differently on
macOS. `tar --version` tells you which one you have.
