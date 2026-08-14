---
title: "Linux abstraction layers"
date: 2026-08-07
summary: "Hardware, kernel, and user space: the three layers Linux is usually divided into, and what the kernel does at the boundary between them."
tags: [education, linux, kernel, processes, memory, rhcsa]
---

## Overview

Linux looks arcane from the outside, but most of it becomes tractable once you
split the system into three layers: **hardware**, the **kernel**, and the
**processes** that run on top of it. Each layer only talks to its neighbor, and
almost every Linux concept you meet later — permissions, memory pressure,
device files, system calls — sits at one of those two boundaries.

- **Hardware** is the physical machine: processing units, memory, and
  input/output devices.
- The **kernel** is the core of the operating system. It loads into memory at
  startup, schedules instructions for the CPU, and mediates between hardware
  and the programs a user runs.
- **Processes** are the running programs the kernel manages. Collectively they
  make up **user space**.

The memory the kernel reserves for itself is **kernel space**, and nothing in
user space is allowed to touch it. That single boundary is the reason a
misbehaving program can crash itself without taking down the machine.

## Hardware: main memory

**Main memory** is the most important piece of hardware to understand first. It
is a large array of bits, each of which can be 0 or 1. Programs live and run
there, and the CPU acts on instructions it finds in memory, moving bits around
according to them. A **memory state** is one discrete arrangement of those bits.

This matters for the layers above it: nearly everything the kernel does is
ultimately about deciding who gets which region of that array, and when.

## The kernel

One of the kernel's central jobs is to **allocate** memory — tracking what is
available and how much each process is asking for. Its work is usually grouped
into four areas:

- **Process management** — scheduling processes for CPU access.
- **Memory management** — tracking and distributing memory.
- **Device drivers** — interfacing with hardware such as disks and I/O streams.
- **System calls** — servicing requests that processes make of the kernel.

### Process management

**Process management** covers starting, pausing, resuming, scheduling, and
terminating processes. Many processes need to run at once, but a CPU core
handles one instruction stream at any given moment. The kernel resolves this by
giving each process a **time slice** — a window of CPU time long enough to make
progress — and rotating between them. This rotation is **multitasking**, and it
runs fast enough that every program appears to be running simultaneously, the
way a sequence of still frames reads as motion.

Moving the CPU from one process to another is **context switching**. A single
switch looks roughly like this:

1. A timer interrupt stops the running user process; the CPU enters kernel mode
   and hands control to the kernel.
2. The kernel records the CPU and memory state of the outgoing process so it can
   be resumed later.
3. The kernel performs whatever system tasks are queued.
4. The kernel examines the processes asking for CPU time and chooses one.
5. The kernel sets the length of the new time slice.
6. The kernel switches the CPU back into user mode, and the chosen process runs.

### Memory management

Memory has to be managed across those switches, under several constraints at
once:

- The kernel needs private memory that no process can reach.
- Each user process needs its own memory.
- One process must not read another process's private memory.
- Processes must still be able to share memory deliberately.
- Some memory is read-only.
- Disk space can serve as auxiliary memory when physical memory runs out.

Modern CPUs include a **memory management unit (MMU)**, which lets the kernel
give each process **virtual memory** — an address space that looks private and
contiguous to the process regardless of how the underlying physical memory is
arranged.

### Device management

Devices are normally reached only through the kernel, so that unsafe direct
access cannot crash the system. Because vendors expose different proprietary
interfaces, **device drivers** exist to translate between the kernel and a
particular piece of hardware.

### System calls

A **system call** (or **syscall**) is a request from a user process asking the
kernel to do something only the kernel can do. Opening, reading, and writing
files are all system calls.

Two are worth knowing early because they explain how processes come to exist:

- `fork()` — the kernel creates a nearly identical copy of the calling process.
- `exec()` — the kernel loads and starts a new program, replacing the current
  one.

Every process other than `init` is started with `fork()`. Running `ls` in a
shell forks the shell, and the new copy calls `exec()` on `ls`.

**Pseudodevices** are virtual software devices that look like real hardware to
processes. `/dev/random` is a familiar example. They are usually implemented in
kernel space for practical reasons, though nothing requires that.

### User space

**User space** is the region of main memory the kernel allocates to user
processes, and it is where most running programs live. User space tends to be
layered in its own right: basic services at the bottom, utility services in the
middle, and user-facing applications on top.

## Users

A **user** is an entity that can run processes and own files. Each is associated
with a **username** and a numeric **user ID**. Users exist to support permission
boundaries.

`root` is a special administrative user with full access to the local system,
sometimes called the **superuser**; accounts able to operate as `root` have
**root access**. **Groups** are sets of users that can share file access.

## Suggested practice: watch the layers from user space

All of this is observable on an ordinary Linux machine with tools that ship with
the system. Nothing here requires root or risks the host.

1. Run `ps -ef` to list running processes, then `ps -ef --forest` to see the
   parent/child tree that `fork()` produces. Find `init` or `systemd` at PID 1.
2. Look at `/proc/$$/status` for your own shell. `/proc` is a pseudo-filesystem —
   the kernel presenting its own state as files.
3. Run `free -h` and `vmstat 1 5` to see physical memory, swap, and context
   switch counts. Watch the `cs` column while the system is busy.
4. Trace the system calls behind a simple command with `strace -f ls`. Look for
   the `execve` call at the top and the file-related calls beneath it.
5. Compare `cat /dev/random | head -c 16 | xxd` with reading an ordinary file to
   see a pseudodevice behaving like a regular one.

## Related pages

- [The Linux filesystem hierarchy](/learn/linux-filesystem-hierarchy) — where
  `/proc`, `/dev`, and the rest of the tree put these abstractions on disk.
- [The shell and the command line](/learn/linux-shell) — the user-space program
  that turns what you type into the `fork()` and `exec()` calls described above.
- [File permissions and links](/learn/linux-permissions) — how the user and
  group boundaries introduced here are enforced on individual files.
- [The OSI model](/learn/osi-model) — another layered model, where each layer
  talks only to its neighbors and hides its internals from the rest.

## Sources and further reading

This page was edited from my Linux study notes and checked against:

- [`fork(2)`](https://man7.org/linux/man-pages/man2/fork.2.html) and
  [`execve(2)`](https://man7.org/linux/man-pages/man2/execve.2.html) — the exact
  semantics of process creation and program replacement.
- [`proc(5)`](https://man7.org/linux/man-pages/man5/proc.5.html) — what the
  kernel exposes through `/proc`.
- [The Linux Kernel documentation](https://docs.kernel.org/) — memory
  management, scheduling, and driver interfaces in detail.

The three-layer split here is a teaching abstraction, not a description of the
source tree. Real kernels blur these boundaries: drivers can live in user space,
and scheduling behavior is considerably more involved than fixed time slices.
