---
title: "RHEL study box"
date: 2026-08-14
summary: "A golden template for practising RHEL-family administration, and the trap of conflating careful documented building with timed rebuilding — they train opposite habits."
tags: [labs, linux, rhel, selinux, practice]
---

## Why this machine exists

The Linux pages on this site are checked against man pages and the Filesystem
Hierarchy Standard, but the server they are written on runs Debian. A
RHEL-family box is where a claim about RHEL tooling can be *tested* rather than
assumed — where `dnf`, SELinux, and `firewalld` behave the way the documentation
for those tools says, rather than the way the Debian equivalents do.

Use a free RHEL-compatible distribution. Knowledge from other families transfers
conceptually but not command-for-command, and the gap is exactly where mistakes
live.

## Two tracks, and they must stay separate

This is the thing most likely to go wrong, and it is not obvious:

| Track | Purpose | Where |
|---|---|---|
| **Speed** | Build, destroy, rebuild against a clock, from memory | A throwaway clone — not this machine |
| **Evidence** | A durable, documented system worth writing up | The lab fleet |

Conflating them is how someone ends up with an impressive homelab and slow
hands. A carefully built, carefully documented system trains you to build
carefully. A timed rebuild trains something else entirely — recall under
pressure, with no notes and no internet.

Both are worth having. They are not the same exercise, and doing one while
believing you are doing the other is the failure mode.

Use this machine as the **golden template**: clone from it for each timed run,
wreck the clone, delete it.

## Set the template up deliberately incomplete

If the template already has SELinux configured, storage laid out, and containers
running, you never practise doing any of it. The template should give you a
booted box and nothing else.

```bash
cat /etc/os-release     # confirm the family and version
sudo dnf update -y      # baseline, and nothing more
```

## What to practise, hardest first

Sequence by measured weakness rather than by book order — the topics you are
worst at are the ones with the most to gain, and they are never the ones that
feel most rewarding to revise.

**SELinux**, because it is the one that has no Debian-side intuition to fall
back on:

```bash
getenforce                    # Enforcing / Permissive / Disabled
sudo setenforce 0             # runtime only
```

Know the difference between `setenforce` and the configuration file — one
survives a reboot and the other does not, and finding that out during a timed
rebuild is expensive. Then work through contexts and booleans:

```bash
ls -Z /var/www/html           # see the context, not just the mode
sudo restorecon -Rv /var/www  # reset to policy default
getsebool -a | head           # what is toggleable
sudo semanage fcontext -l | head
```

Then storage — partitions, LVM, filesystems, and persistent mounts — followed by
service management, users and permissions, and firewall rules with `firewalld`.

## Verification

You have learned it when you can do it without the notes:

1. Clone the template.
2. Set a timer.
3. Perform the task from memory, no browser.
4. Reboot, and confirm the change survived. A configuration that works until
   restart has not been made.
5. Destroy the clone.

Step 4 is the one that catches people. Most of the difference between "it works"
and "it is configured" is whether it comes back.

## Stop conditions

- You find yourself documenting the timed clone → stop. That is the evidence
  track leaking into the speed track, and it slows both.
- The golden template accumulates configuration → rebuild it. A template that
  has drifted into a working system is no longer a template.

## Done when

- [ ] Golden template built and deliberately minimal
- [ ] Clone-and-destroy workflow tested end to end
- [ ] Weakest topics identified honestly and worked first
- [ ] Every practised change verified to survive a reboot
- [ ] The durable, documented work kept on separate machines from the timed work
