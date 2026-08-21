# Archived portfolio entries

Pulled from `src/models/project.rs::all()` on 2026-07-25. Verbatim field
values at the time of archiving, kept here so the long-form interview rewrite
has the original wording to work from. None of this is live — see this
directory's `README.md`.

> **⚠️ Do not restore the cert wording below. Left verbatim on purpose —
> editing a verbatim archive defeats its point — but it is out of date twice over.**
>
> - **Security+, Linux+, and Server+ were dropped** from the plan on 2026-07-20.
> - **All public cert claims were retired 2026-07-25** until an exam voucher is booked.
>
> So when the interview rewrite pulls from these entries: keep the *project*
> descriptions, **drop every "Anchored to [cert]" line, every cert tag, and the whole
> "Certification track" entry.** Anchor each project to the capability it demonstrates
> instead. See `mg-coreforge/PUBLIC_FACE.md`.

## Homelab project 1 — internal DNS + network map

- description: A small internal DNS service, a role-based network and service
  map, and a reproducible name-resolution troubleshooting writeup
  captured before and after breaking a record. Anchored to Network+.
- tags: homelab, networking, dns, network+
- url: none
- status: in progress

## Homelab project 2 — harden & monitor the homelab

- description: A hardening pass across a lab VM and this server — key-only
  SSH, a host firewall, non-root service users, unattended updates — a
  security-headers audit of the site, and a log-based failed-login detector
  with triage notes. Anchored to Security+.
- tags: homelab, security, linux, hardening, security+
- url: none
- status: in progress

## Homelab project 3 — virtualized cluster operations

- description: Virtualized cluster operations: a baseline and asset inventory,
  a validated guest backup and restore with RPO/RTO notes, a
  monitoring stack, structured incident write-ups, and high availability as
  the capstone. Anchored to Server+ and Linux+.
- tags: homelab, virtualization, backup, monitoring, server+
- url: none
- status: in progress

## Certification track — Network+ to Server+ by Jan 2027

- description: Network+, Security+, Linux+, then Server+, each one anchored
  to a homelab project above. Writeups link from each as the work lands.
- tags: comptia, network+, security+, linux+, server+
- url: none
- status: in progress

## GeistScope (retrospective)

- description: An early AI-assisted-coding security-tooling experiment that
  over-scoped; the project has been removed from the public tool catalog and
  is not presented as professional security work. The retrospective records
  what was real, what was aspirational, and the publication gate future
  tools must meet.
- tags: rust, ai-assisted, retrospective, scope-control
- url: /blog/geistscope-retrospective (dead — the retrospective post is
  archived alongside this file, see geistscope-retrospective.md)
- status: complete
