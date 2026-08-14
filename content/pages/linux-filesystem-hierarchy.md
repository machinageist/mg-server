---
title: "The Linux filesystem hierarchy"
date: 2026-08-14
summary: "One tree from a single root: what each top-level directory is for, why /usr absorbed several of them, and how root access is meant to be used."
tags: [education, linux, filesystem, fhs, sudo, rhcsa]
---

## Overview

Linux has no drive letters. Every file on every disk hangs off a single tree
whose root is `/`, and additional storage is attached at a directory somewhere
inside it. A USB drive does not become `E:`; it becomes `/media/you/label`, and
the paths beneath it work like any other path.

The layout of that tree is not arbitrary. It follows the **Filesystem Hierarchy
Standard (FHS)**, which is why a configuration file lives in `/etc` on Debian,
Fedora, and Arch alike. Knowing the map means being able to guess where
something is before searching for it — and being able to tell, from a path
alone, roughly what a file does and whether you should be editing it.

## The top-level directories

| Path | Contains |
|---|---|
| `/bin` | Essential user commands — `ls`, `cp`, `cat` |
| `/boot` | The kernel and bootloader files |
| `/dev` | Device files |
| `/etc` | System-wide configuration |
| `/home` | Users' home directories |
| `/lib` | Shared libraries the binaries depend on |
| `/media` | Mount points for removable media |
| `/mnt` | Mount point for temporary manual mounts |
| `/opt` | Self-contained third-party software |
| `/proc` | Kernel and process state, presented as files |
| `/root` | The root user's home directory |
| `/run` | Runtime state for currently running processes |
| `/sbin` | System administration commands |
| `/srv` | Data served by this system |
| `/sys` | Device and kernel interfaces |
| `/tmp` | Temporary files |
| `/usr` | The bulk of the system — programs, libraries, shared data |
| `/var` | Data that changes as the system runs |

Several of these repay a closer look.

### /etc

System-wide configuration, in plain text, editable with any editor. `/etc/fstab`
describes what gets mounted at boot; `/etc/passwd` lists accounts;
`/etc/ssh/sshd_config` configures the SSH daemon. The convention that
configuration is text you can read, diff, and put under version control is one
of the reasons Linux systems are administrable at scale.

Nothing in `/etc` should be a binary, and nothing in it should be per-user —
user configuration lives in the home directory, usually in dot files.

### /usr and the merged directories

`/usr` holds most of the installed system: `/usr/bin`, `/usr/lib`, `/usr/sbin`,
`/usr/share` for architecture-independent data, `/usr/include` for C headers,
and `/usr/local` for software installed by the administrator rather than by the
package manager.

On any current distribution, `/bin`, `/sbin`, and `/lib` are symbolic links into
`/usr`. This is the **usr-merge**, adopted by Fedora and RHEL, Debian, Arch, and
others over the last decade. The original split existed because early Unix
systems kept a minimal set of binaries on a small root partition and mounted
`/usr` separately; initramfs made that unnecessary. The old paths still work,
because everything that ever referenced them still resolves.

You can confirm it on your own machine:

```text
$ ls -ld /bin
lrwxrwxrwx 1 root root 7 ... /bin -> usr/bin
```

### /proc and /sys

Neither of these is on a disk. Both are **pseudo-filesystems** — the kernel
presenting its own state through the file interface so ordinary tools can read
it.

`/proc` is process and kernel information. Every running process has a numbered
directory: `/proc/1/status` describes PID 1. `/proc/meminfo` is where `free`
gets its numbers. `/sys` is the newer and more structured of the two, exposing
devices, drivers, and kernel objects; it is what `udev` and most modern
device tooling work against.

This is the same abstraction described in [Linux abstraction
layers](/learn/linux-abstraction-layers): a system call dressed up as a file
read, so that `cat` works on kernel state.

### /var and /run

`/var` is for data that grows and changes while the system runs — logs in
`/var/log`, mail spools, package manager caches, databases. It is the directory
most likely to fill a disk, and `/var/log` is the first place to look when
something has failed.

`/run` holds runtime state for the current boot: PID files, sockets, lock files.
It is a tmpfs, meaning it lives in memory and is empty again after a reboot.
That is the point — stale lock files from a crashed process should not survive.

### /boot

The kernel and everything needed to load it. The kernel image is named
`vmlinuz-` followed by a version string on Fedora, RHEL, Debian, and Ubuntu:

```text
/boot/vmlinuz-6.11.0-19-generic
```

Arch installs it as `/boot/vmlinuz-linux` instead. The bootloader configuration
lives alongside it, under `/boot/grub` or `/boot/loader` depending on what is
installed. `/boot` is often a separate partition, which is why it is
occasionally the one that runs out of space after several kernel updates.

## The root user

`root` is the superuser: full access to the local system, with none of the
permission checks that constrain ordinary accounts. Everything on the system is
reachable, including the parts that break it.

The right way to use that power is `sudo` rather than a root shell. A root shell
runs every command with full privilege, including the ones that did not need it,
and it records almost nothing about what happened. `sudo` is narrower and
auditable:

```text
$ sudo systemctl restart sshd
```

Every invocation is logged, and the journal can be queried for it:

```text
$ journalctl SYSLOG_IDENTIFIER=sudo
```

An account gains that ability by being granted it in the sudo configuration —
in practice by being added to a group (`wheel` on RHEL and Fedora, `sudo` on
Debian and Ubuntu) that the configuration already permits.

Edit that configuration with `visudo`, never with a plain editor. `visudo`
validates the syntax before saving. A malformed sudoers file locks everyone out
of privilege escalation on the machine, and recovering from it means booting
into rescue mode. Better still, drop a file into `/etc/sudoers.d/` rather than
editing the main file, so a package update never conflicts with your change.

## Suggested practice: read the tree on your own machine

Everything here is inspectable on any Linux system, and none of it needs root.

1. Run `man hier`. It is the filesystem hierarchy as a manual page, installed on
   the machine in front of you, and it is more authoritative for your
   distribution than any summary.
2. Run `ls -ld /bin /sbin /lib` and check whether they are directories or
   symbolic links. Note which distribution you are on and when it merged them.
3. Run `df -h` and `findmnt` to see which directories are separate filesystems.
   Find `/run` and confirm it is a tmpfs.
4. Read `/proc/$$/status` — the kernel's view of your own shell. Then
   `cat /proc/cpuinfo` and `free -h`, and notice that the second is reading the
   first kind of thing.
5. Find the largest consumers under `/var` with `du -h --max-depth=1 /var/log |
   sort -h`. This is the first command to reach for when a disk fills up.
6. Run `sudo true`, then `journalctl SYSLOG_IDENTIFIER=sudo -n 5` and find your
   own entry. That is the audit trail a root shell would not have produced.

## Related pages

- [Linux abstraction layers](/learn/linux-abstraction-layers) — the kernel and
  user space split that `/proc` and `/dev` sit on the boundary of.
- [The shell and the command line](/learn/linux-shell) — navigating this tree,
  and where `PATH` looks for the executables in `/usr/bin`.
- [File permissions and links](/learn/linux-permissions) — who is allowed to
  read and change what, once you have found it.

## Sources and further reading

This page was edited from my Linux study notes and checked against:

- [Filesystem Hierarchy Standard 3.0](https://refspecs.linuxfoundation.org/FHS_3.0/fhs/index.html)
  — the specification each directory's purpose comes from.
- [`hier(7)`](https://man7.org/linux/man-pages/man7/hier.7.html) — the same
  material as a manual page, installed locally.
- [`proc(5)`](https://man7.org/linux/man-pages/man5/proc.5.html) — what the
  kernel exposes through `/proc`, entry by entry.
- [`sudoers(5)`](https://man7.org/linux/man-pages/man5/sudoers.5.html) — the
  configuration syntax, and why `visudo` exists.

Distributions deviate from the FHS in small ways, and the usr-merge landed in
different releases for each of them. When a path on a specific machine does not
match this page, `man hier` and the distribution's own documentation win.
