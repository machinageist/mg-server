---
title: "File permissions and links"
date: 2026-08-14
summary: "Reading ls -l, the user/group/other model, chmod in both notations, what the same three bits mean on a directory, umask defaults, and symbolic links."
tags: [education, linux, permissions, chmod, umask]
---

## Overview

Every file on a Linux system has an owner, a group, and a set of permissions.
That triple is the whole access control model in its basic form, and it is
doing more work than it appears to — it is why one user cannot read another's
private keys, why a web server can serve a file it cannot modify, and why
"Permission denied" is the most common error a new administrator meets.

The model is small enough to hold in your head, which is its main virtue. Ten
characters of `ls -l` output tell you what a file is and who may do what with
it.

## Reading ls -l

```text
$ ls -l
-rw-r--r--  1 you  staff   4096  Aug 14 09:12  notes.txt
drwxr-xr-x  2 you  staff   4096  Aug 12 17:03  projects
lrwxrwxrwx  1 you  staff     11  Aug 10 08:44  current -> projects
```

The fields are: mode, link count, owner, group, size in bytes, modification
time, and name.

The mode is one character of file type followed by nine permission bits:

```text
-  rw-  r--  r--
│   │    │    └── other:  read
│   │    └─────── group:  read
│   └──────────── user:   read, write
└──────────────── type:   regular file
```

The type character is the one worth learning first:

| Character | Type |
|---|---|
| `-` | Regular file |
| `d` | Directory |
| `l` | Symbolic link |
| `c` | Character device — `/dev/null`, a terminal |
| `b` | Block device — a disk |
| `s` | Socket |
| `p` | Named pipe |

The nine bits that follow are three groups of three, and each group is `r`
(read), `w` (write), `x` (execute) in that fixed order. A dash means the
permission is absent. The three groups apply to:

- the **user** who owns the file;
- the **group** the file belongs to; and
- **other** — everyone else.

Only one group applies to any given request. If you own the file, the user bits
decide, and the group and other bits are irrelevant to you. This trips people up
when they remove their own write permission and find that being in the group
does not save them.

## Changing permissions with chmod

`chmod` accepts two notations, and both are worth knowing because each is
clearer in different situations.

### Symbolic notation

Symbolic notation names who and what, and changes only what you mention:

```text
$ chmod g+r report.txt        # add read for the group
$ chmod o-w report.txt        # remove write from other
$ chmod u+x script.sh         # make it executable by its owner
$ chmod a-x notes.txt         # remove execute from everyone
$ chmod g=rx shared/          # set group to exactly r-x
```

The targets are `u` (user), `g` (group), `o` (other), and `a` (all). The
operators are `+` to add, `-` to remove, and `=` to set exactly. This is the
safer form for adjusting one thing, because it leaves everything else alone.

### Numeric notation

Numeric notation sets all nine bits at once. Each group of three is one octal
digit:

| Bits | Octal | Meaning |
|---|---:|---|
| `r--` | 4 | Read |
| `-w-` | 2 | Write |
| `--x` | 1 | Execute |

Add them for combinations: `rw-` is 4+2 = 6, `r-x` is 4+1 = 5, `rwx` is 7.

```text
$ chmod 644 notes.txt         # rw-r--r--
$ chmod 755 script.sh         # rwxr-xr-x
$ chmod 600 ~/.ssh/id_ed25519 # rw------- and nothing else
```

Three values cover most of what you will actually type. `644` for a document,
`755` for a program or a directory, `600` for a secret. SSH refuses to use a
private key that is readable by anyone else, which makes `600` a habit worth
forming early.

## Permissions on directories

The same three bits mean different things on a directory, and this is the part
that is genuinely counterintuitive:

- **read** lets you list the names inside it;
- **write** lets you create, rename, and delete entries in it; and
- **execute** lets you traverse into it and reach files by name.

Two consequences follow. First, `r` without `x` means you can see the names but
cannot open anything — `ls` works and `cat dir/file` does not. Second, `x`
without `r` means the opposite: you cannot list the directory, but you can open
a file inside it if you already know its name. That combination is a real
technique for a directory that should be reachable but not enumerable.

The other consequence surprises people: deleting a file requires write
permission on the *directory*, not on the file. The file is just data; the name
is an entry in the directory, and removing the name is a change to the
directory.

In practice a usable directory needs `r-x` for anyone expected to work in it,
and `rwx` for anyone expected to add to it.

### What else you will see

Occasionally an `s` or `t` appears where an `x` would be. These are the setuid,
setgid, and sticky bits — `/usr/bin/passwd` is setuid root so that an ordinary
user can change their own password in a file they cannot write, and `/tmp` is
sticky so that users can create files there without deleting each other's. They
are worth recognizing in `ls -l` output now and studying properly later.

## Default permissions and umask

New files do not appear with arbitrary permissions. The system starts from a
base — `666` for files, `777` for directories — and removes the bits set in the
**umask**:

```text
$ umask
0022
```

A umask of `022` removes write permission for group and other, giving new files
`644` and new directories `755`. A umask of `077` removes everything for group
and other, giving `600` and `700` — private by default, which is the right
setting on a shared machine.

Note that the base for files is `666`, not `777`. New files never get execute
permission from the umask no matter what you set, which is a deliberate safety
property: a downloaded or generated file is not runnable until someone says so.

Setting `umask` in a shell affects that shell only. To make it apply to your
sessions generally, put it in a startup file such as `~/.bashrc` — the same
repetition-rather-than-persistence pattern that [shell
variables](/learn/linux-shell) follow.

## Symbolic links

A **symbolic link** is a small file whose contents are a path to something else.
Opening the link opens the target.

```text
$ ln -s /var/log/nginx/access.log current.log
$ ls -l current.log
lrwxrwxrwx 1 you staff 26 Aug 14 09:20 current.log -> /var/log/nginx/access.log
```

The syntax is `ln -s target linkname`, in that order — the thing that exists
first, the name being created second. Reversing them is the most common mistake
with this command.

Two properties matter:

- The `lrwxrwxrwx` permissions on a symlink are meaningless. Access is decided
  by the target's permissions, not the link's.
- The link stores a path, not a reference to the data. Delete or move the
  target and the link remains, now pointing at nothing — a **broken link**. `ls`
  will usually colorize it, and `ls -lL` fails on it.

Symlinks are why `/bin` can be a link to `/usr/bin` and every path that ever
referenced `/bin/ls` still resolves. They are used throughout the system for
exactly this: giving one thing several names, and giving a stable name to
something that moves.

## Moving files with their permissions intact

Modes, ownership, and timestamps are properties the filesystem holds about a
file rather than content inside it, so copying a tree the naive way can lose all
of them. [Archives and compression](/learn/linux-archives) covers `tar`, `gzip`,
and what each one preserves.

## Suggested practice: break and repair access

Do this in a scratch directory. Nothing here touches the system, and all of it
is reversible.

1. Create a file and run `ls -l`. Name each of the ten characters out loud
   before checking yourself.
2. Remove your own read permission with `chmod u-r file`, then try to `cat` it.
   Read the error. Restore with `chmod u+r`.
3. Set `chmod 644 script.sh` on a shell script and try to run it. Then `chmod
   +x` and run it again. This is the difference between a text file and a
   program on Linux.
4. Make a directory `r--` and try to `ls` it, then `cat` a file inside it by
   name. Make it `--x` and try both again. Predict each outcome first.
5. Run `umask 077`, create a file, and check its mode. Run `umask 022`, create
   another, and compare. Confirm neither is executable.
6. Create a symlink, verify it works, delete the target, and run `ls -l` and
   `cat` on the link. Then recreate the target and watch the link start working
   again.
7. Give a file an unusual mode, copy it with `cp` and then with `cp -a`, and
   compare `ls -l` on both. Only one of them preserved what you set.

## Related pages

- [Archives and compression](/learn/linux-archives) — `tar` and `gzip`, and
  which of these modes survive a copy between machines.
- [The shell and the command line](/learn/linux-shell) — reading the
  "Permission denied" errors this page produces.
- [The Linux filesystem hierarchy](/learn/linux-filesystem-hierarchy) — where
  these files live, and why `/bin` is a symbolic link.
- [Linux abstraction layers](/learn/linux-abstraction-layers) — users, groups,
  and the kernel boundary permissions are enforced at.

## Sources and further reading

This page was edited from my own study notes, taken from Brian Ward's *How Linux
Works: What Every Superuser Should Know* (No Starch Press), and checked against
the primary documentation:

- [`chmod(1)`](https://man7.org/linux/man-pages/man1/chmod.1.html) and
  [`chmod(2)`](https://man7.org/linux/man-pages/man2/chmod.2.html) — the
  command and the system call beneath it.
- [`umask(2)`](https://man7.org/linux/man-pages/man2/umask.2.html) — the base
  modes and how the mask is applied.
- [`symlink(7)`](https://man7.org/linux/man-pages/man7/symlink.7.html) — symlink
  resolution, including why their permission bits are ignored.
- [`inode(7)`](https://man7.org/linux/man-pages/man7/inode.7.html) — the mode
  bits as the kernel stores them, including setuid, setgid, and sticky.

This page covers the traditional permission bits. Access control lists (`getfacl`
and `setfacl`) and SELinux contexts sit on top of them and can deny access that
these nine bits appear to allow — worth knowing exists before a file that looks
readable is not.
