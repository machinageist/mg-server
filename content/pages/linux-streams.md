---
title: "Streams, redirection, and pipes"
date: 2026-08-14
summary: "The three standard streams every process gets, how the shell repoints them at files and at each other, and why the order of 2>&1 changes the result."
tags: [education, linux, shell, streams, redirection, pipes]
---

## Overview

The single idea behind this page is that a program's input and output do not
have to be a terminal. A process writes to **standard output** without knowing
or caring whether that lands on a screen, in a file, or in another program's
input. The shell decides.

That indirection is why a handful of small Unix tools compose into work none of
them was written for. `grep` does not need a "search a file listing" mode,
because `ls` can hand it one. Learning the redirection syntax is learning most
of what makes [the shell](/learn/linux-shell) worth using over a graphical file
manager.

## The three standard streams

Every process starts with three streams already open:

| Stream | Number | Default |
|---|---:|---|
| Standard input (`stdin`) | 0 | The keyboard |
| Standard output (`stdout`) | 1 | The terminal |
| Standard error (`stderr`) | 2 | The terminal |

The numbers matter, because the redirection syntax below refers to streams by
number rather than by name.

Two output streams rather than one looks redundant until you separate them.
Results go to standard output and diagnostics go to standard error, so that
sending a command's results somewhere does not also send its complaints there.
A pipeline that silently swallowed its own error messages would be much harder
to debug than one where the errors still reach the screen.

## Redirection

**Send output to a file** with `>`, which creates the file or overwrites it:

```text
$ ls /etc > listing.txt
```

**Append instead** with `>>`, which is the difference between keeping a log and
destroying one:

```text
$ date >> logbook.txt
```

**Read input from a file** with `<`:

```text
$ sort < names.txt
```

**Redirect standard error separately** with `2>` — the `2` is the stream number
from the table above:

```text
$ find / -name "*.conf" > found.txt 2> errors.txt
```

Results land in one file, permission-denied complaints in the other. Running
`find /` as an ordinary user is the clearest demonstration of why the two
streams are separate: the results and the errors arrive interleaved, and only
redirection tells them apart.

## Pipes

The pipe character `|` connects the standard output of one process to the
standard input of the next:

```text
$ ps -ef | grep sshd
```

Nothing touches the disk, and neither program knows the other exists. Each stage
does one job and passes its output along:

```text
$ ps -ef | grep bash | wc -l
```

Build these one stage at a time and check the output after each addition. A
pipeline that produces nothing is usually failing at a stage earlier than the
one you are looking at.

## Combining the two streams

`2>&1` means "make stream 2 go wherever stream 1 currently goes":

```text
$ command > everything.txt 2>&1
```

Order matters here, and it is a classic trap. Redirections are applied left to
right:

- `> file 2>&1` — stdout moves to the file, *then* stderr is pointed at the same
  place. Both end up in the file.
- `2>&1 > file` — stderr is pointed at wherever stdout is *at that moment*,
  which is still the terminal, and *then* stdout moves to the file. Errors keep
  appearing on screen.

The two read as though they should be equivalent and are not. Bash also accepts
`&>` as a shorthand for the correct form:

```text
$ command &> everything.txt
```

To discard output entirely, send it to `/dev/null` — a pseudodevice that accepts
anything written to it and returns nothing:

```text
$ command 2> /dev/null        # hide errors, keep results
$ command > /dev/null 2>&1    # hide everything, keep only the exit status
```

## Suggested practice: watch the streams separate

Every step runs on any Linux machine as an ordinary user, and nothing here
changes the system.

1. Run `find /etc -name "*.conf"` as a normal user and watch results and errors
   interleave on screen.
2. Separate them with `> found.txt 2> errors.txt`, then read each file. Confirm
   that the counts add up to what you saw in step 1.
3. Run the same command with `2> /dev/null` and confirm the errors vanish while
   the results remain.
4. Compare `command > out.txt 2>&1` with `command 2>&1 > out.txt` on something
   that produces both kinds of output. Work out which one put the errors on
   screen before reading back up this page.
5. Build a pipeline a stage at a time, checking output after each addition:
   `ps -ef`, then `ps -ef | grep bash`, then `ps -ef | grep bash | wc -l`.
6. Prove a stream does not have to be a terminal: run `ls | cat` and then
   `ls > /dev/null`. Then try `ls -l /proc/self/fd` inside a pipeline and see
   the descriptors the shell actually set up.

## Related pages

- [The shell and the command line](/learn/linux-shell) — the prompt, options and
  arguments, variables, and the manual pages that document every tool used here.
- [File permissions and links](/learn/linux-permissions) — the source of most of
  the "Permission denied" lines that land on stderr.
- [Archives and compression](/learn/linux-archives) — `zcat file.tar.gz | tar xvf -`
  is this page's plumbing applied to a real problem.
- [Linux abstraction layers](/learn/linux-abstraction-layers) — file descriptors
  and pseudodevices as the kernel presents them.

## Sources and further reading

This page was edited from my own study notes, taken from Brian Ward's *How Linux
Works: What Every Superuser Should Know* (No Starch Press), and checked against
the primary documentation:

- [`bash(1)`](https://man7.org/linux/man-pages/man1/bash.1.html) — the
  REDIRECTION section is the authoritative statement of the ordering rule.
- [POSIX Shell Command Language — Redirection](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html#tag_18_07)
  — the portable subset, which `&>` is not part of.
- [`pipe(7)`](https://man7.org/linux/man-pages/man7/pipe.7.html) — what a pipe
  actually is at the system-call level, including its buffer.
- [`null(4)`](https://man7.org/linux/man-pages/man4/null.4.html) — `/dev/null`
  and its siblings.

`&>` and a few other conveniences here are bash extensions. A script that has to
run under `dash` — which is `/bin/sh` on Debian and Ubuntu — needs the `> file
2>&1` form instead.
