---
title: "The shell and the command line"
date: 2026-08-14
summary: "What a shell is, how to read a command line, dot files, shell versus environment variables and what export really does, PATH, and using the manual pages already on the machine."
tags: [education, linux, shell, bash, streams]
---

## Overview

The **shell** is a program that reads commands and runs them. It is the oldest
interface Unix systems have, and it is still the one that scales: anything you
can type, you can put in a file and run again, schedule, or hand to someone
else.

That is the property worth holding onto. A graphical file manager and `cp` both
copy a file, but only one of them leaves behind something you can repeat exactly
next month. Most of the Linux system is itself made of **shell scripts** — text
files containing sequences of the same commands you type interactively.

The original was the Bourne shell, at `/bin/sh`. Linux systems use an enhanced
version called the **Bourne-again shell**, or `bash`. What `/bin/sh` points to
varies: on Fedora and RHEL it is bash, while Debian and Ubuntu link it to
`dash`, a smaller and stricter shell. Scripts written for bash should say `bash`
in their `#!` line rather than assuming `sh` will behave the same way.

Use an ordinary user account for this. Working as `root` all the time is a habit
that eventually costs a system.

## Reading a command line

Opening a terminal gives you a shell and a **prompt**, which by default looks
something like:

```text
you@hostname:~/projects$
```

- `you` is the account running commands.
- `hostname` is the machine you are on — which matters more than it seems once
  you have several SSH sessions open.
- `~/projects` is the current working directory. `~` is shorthand for your home
  directory.
- `$` marks the end of the prompt and the start of what you type. A `#` there
  instead means the shell is running as root.

That last convention carries into documentation, including this page. A line
beginning `$` is run as a normal user; a line beginning `#` needs root. Neither
character is part of the command.

A command line has up to four kinds of part:

```text
$ ls -l --color=auto /etc
```

- `ls` is the **command** — the program to run.
- `-l` is an **option** or flag, altering behavior. Single-letter options take
  one dash and can usually be combined: `-la` is `-l -a`.
- `--color=auto` is a long option, taking two dashes, and this one takes its own
  argument.
- `/etc` is an **argument** — the data the command acts on.

Some commands add a **subcommand** before the options, which is common in newer
tooling: `systemctl restart sshd`, `git commit -m "..."`, `ip addr show`.

## Where the output goes

A command's results and its error messages are separate streams, and the shell
can point either one at a file or at another program. That plumbing is the
subject of its own page:
[streams, redirection, and pipes](/learn/linux-streams).

## Error messages

Linux error messages are terse but literal, and they follow a shape:

```text
$ cat /etc/shadow
cat: /etc/shadow: Permission denied
```

That is the program name, the thing it failed on, and why. Reading it as three
fields rather than as one wall of text usually identifies the problem
immediately — and "Permission denied" versus "No such file or directory" are
very different failures that look similar when skimmed.

## Dot files

A file whose name begins with `.` is hidden from ordinary listings. This is not
a security feature; it is a convention that keeps configuration out of the way.
Most of what is in your home directory is hidden, including `.bashrc`,
`.ssh/`, and `.config/`.

```text
$ ls -a
```

`-a` shows everything, including `.` (the current directory) and `..` (the
parent).

## Shell and environment variables

A **shell variable** is set with an assignment, and no spaces around the `=`:

```text
$ EXAMPLE=one
$ echo $EXAMPLE
one
```

Assigning uses the bare name; reading it uses `$` in front. It exists only in
the shell that created it, and disappears when that shell exits.

An **environment variable** is a shell variable marked for inheritance by child
processes:

```text
$ export EXAMPLE
```

This is the part my own notes had wrong, and it is worth stating plainly:
`export` does not make a variable permanent and does not write anything to disk.
It marks the variable so that programs the shell launches receive a copy. Close
the terminal and it is gone with everything else.

Persistence comes from startup files. Putting the assignment in `~/.bashrc` or
`~/.profile` means it is set again each time a new shell starts — which looks
like permanence but is really repetition.

### PATH

`PATH` is the environment variable that tells the shell where to look for the
programs commands name. It is a colon-separated list, searched left to right:

```text
$ echo $PATH
/usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/sbin
```

Typing `ls` works because `/usr/bin/ls` exists and `/usr/bin` is on that list.
Adding a directory means rebuilding the variable from itself:

```text
$ export PATH=$PATH:/opt/mytool/bin      # searched last
$ export PATH=/opt/mytool/bin:$PATH      # searched first
```

Position matters. A directory placed first can shadow a system command with
something else of the same name — which is occasionally what you want and
occasionally a security problem, and is the reason `.` should never be on your
`PATH`.

Use `which ls` or `type ls` to see which file a command actually resolves to.

## Manual pages

Nearly every command on the system ships with documentation, readable offline:

```text
$ man ls
```

Manual pages are references, not tutorials. They are organized by structure
rather than by task, list options in alphabetical order, and put the examples —
when there are any — at the end. That makes them frustrating as a first
introduction and excellent once you know roughly what you are looking for.

They are divided into numbered sections, and the same name can appear in more
than one. `passwd` is both a command and a file format:

```text
$ man 1 passwd      # the command
$ man 5 passwd      # the file format
```

Section 1 is user commands, 5 is file formats, and 8 is administration
commands. Those three cover most of what an administrator needs.

There is no `man ls -l` for a single option — the page covers the whole command
and you search within it. Press `/` to search, `n` for the next match, and `q`
to quit. To find a page when you do not know its name, `man -k` searches the
descriptions:

```text
$ man -k "disk space"
```

Getting comfortable here removes a dependency on having a browser open, and the
documentation matches the version installed rather than whatever version a web
result was written about.

## Suggested practice: find out what your shell is doing

Every step runs on any Linux machine as an ordinary user.

1. Run `echo $SHELL` and `ls -l /bin/sh` to find out which shell you have and
   what `sh` points to on your distribution.
2. Set a shell variable, check it with `echo`, then start a new shell with
   `bash` and check again — it is gone. Repeat with `export` before starting the
   child shell, and watch it survive.
3. Run `type ls`, `type cd`, and `type ll`. One is a file on disk, one is built
   into the shell, and the third is probably an alias. Knowing which is which
   explains why `man cd` does not work.
4. Add a directory to `PATH`, put a script in it, and run the script by name
   from somewhere else. Then remove it from `PATH` and confirm the shell can no
   longer find it.
5. Run `ls -a ~` and open one dot file you did not know you had. Most of your
   home directory is configuration.
6. Open `man man`, then find one option of a command you use often that you did
   not know about. Then find the same command's entry with `man -k`.

The redirection and pipeline exercises live on
[streams, redirection, and pipes](/learn/linux-streams#suggested-practice-watch-the-streams-separate).

## Related pages

- [Streams, redirection, and pipes](/learn/linux-streams) — where a command's
  output actually goes, and how to send it somewhere else.
- [The Linux filesystem hierarchy](/learn/linux-filesystem-hierarchy) — the tree
  you are navigating, and where `PATH` points.
- [File permissions and links](/learn/linux-permissions) — reading `ls -l`
  output, and why "Permission denied" happens.
- [Linux abstraction layers](/learn/linux-abstraction-layers) — what actually
  happens when the shell forks and runs a command.

## Sources and further reading

This page was edited from my own study notes, taken from Brian Ward's *How Linux
Works: What Every Superuser Should Know* (No Starch Press), and checked against
the primary documentation:

- [`bash(1)`](https://man7.org/linux/man-pages/man1/bash.1.html) — the shell's
  own manual, including parameter expansion and the shell builtins.
- [The GNU Bash Reference Manual](https://www.gnu.org/software/bash/manual/bash.html)
  — the same material in a form that is easier to read end to end.
- [`man(1)`](https://man7.org/linux/man-pages/man1/man.1.html) and
  [`man-pages(7)`](https://man7.org/linux/man-pages/man7/man-pages.7.html) — the
  section numbering and page conventions.
- [POSIX Shell Command Language](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html)
  — the portable subset, useful when a script must run under `dash` too.

Bash-specific features are common enough to feel universal and are not. When a
script has to run somewhere you do not control, the POSIX specification above is
the line between "works everywhere" and "works on my machine."
