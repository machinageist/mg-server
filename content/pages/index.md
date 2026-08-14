---
title: "Education wiki"
date: 2026-07-23
summary: "Free, FOSS-first IT foundations paired with suggested practice and evidence from a learning-in-public systems and networking portfolio."
tags: [education, networking, linux, digital-literacy, digital-sovereignty]
---

## A doorway into technical agency

This wiki is for curious adults who use technology every day but want to understand,
operate, troubleshoot, and eventually build more of it themselves. It begins with the
systems and networking foundations I am studying for an early career in Linux systems
administration and network operations.

The goal is not to make every reader a specialist. It is to provide enough context to
see the larger technical landscape, understand ordinary documentation, and choose a
useful next direction.

## Understand → Practice → Evidence

Topics are developed as flexible learning clusters:

1. **Understand** — connect a concept to the larger system and explain it in ordinary
   language before moving toward protocols and technical specifications.
2. **Practice** — use an accessible lab to observe, configure, break, or troubleshoot
   the concept. Most suggested labs should work on an ordinary computer with free and
   open-source software.
3. **Evidence** — connect the subject to completed, documented work where I have
   personally applied it.

Not every topic needs all three parts. A foundational page may stand alone; an applied
claim should be supported by work I performed and can explain.

## Networking foundations

- [OSI model](/learn/osi-model) — a seven-layer reference model for connecting
  protocols, devices, encapsulation, and troubleshooting to the larger communication
  system.
- [Network topologies](/learn/network-topologies) — how nodes and links are arranged,
  from mesh and star to spine-and-leaf and tiered enterprise designs.
- [Transmission media](/learn/transmission-media) — the radio, copper, and fiber
  that carry data, what bounds each one, and who standardizes what.
- [Wireless media](/learn/wireless-media) — taking turns on a shared channel,
  the 802.11 generations and their bands, cellular, and satellite.
- [Wired media](/learn/wired-media) — copper categories, the 802.3 standards,
  single-mode and multimode fiber, coax, and plenum ratings.
- [Transceivers and connectors](/learn/transceivers) — pluggable modules, their form
  factors, and the connectors that terminate copper and fiber.
- [Network appliances](/learn/network-appliances) — the physical and virtual systems
  that forward, filter, distribute, store, and expose network traffic.
- [Network applications](/learn/network-applications) — beginning with content delivery
  networks and the relationship between distributed infrastructure and applications.
- [Network functions](/learn/network-functions) — VPNs, tunneling, IPsec, quality of
  service, and the lifetime of routed packets.
- [Network protocols and ports](/learn/network-protocols) — common application
  protocols, the port ranges IANA allocates, and the network-layer protocols that
  carry them.
- [Network traffic types](/learn/traffic-types) — unicast, multicast, anycast, and
  broadcast delivery patterns.
- [IPv4 addressing](/learn/ipv4-addressing) — binary octets, public and private
  ranges, subnet masks, CIDR, and variable length subnet masking.
- [Subnetting, CIDR, and VLSM](/learn/subnetting) — counting hosts and networks,
  the mask as a bitwise test, prefix notation, and sizing subnets to fit.
- [IPv6 addressing](/learn/ipv6-addressing) — hextets and zero compression, the
  address types on a working interface, neighbor discovery, and coexistence with
  IPv4.
- [Cloud computing concepts](/learn/cloud-computing) — NFV, virtual private clouds,
  cloud traffic controls, and deployment and service models.
- [Software-defined networking](/learn/software-defined-networking) — separating
  the control and data planes, SD-WAN, VXLAN overlays, and infrastructure as code.
- [Zero-trust architecture](/learn/zero-trust-architecture) — verifying every
  request rather than trusting a location, the NIST policy model, and SASE.

## Linux foundations

- [Linux abstraction layers](/learn/linux-abstraction-layers) — hardware, kernel, and
  user space, and what the kernel does at the boundary between them.
- [The Linux filesystem hierarchy](/learn/linux-filesystem-hierarchy) — one tree
  from a single root, what each top-level directory holds, and how root access is
  meant to be used.
- [The shell and the command line](/learn/linux-shell) — reading a command, dot
  files, shell versus environment variables, `PATH`, and the manual pages
  already on the machine.
- [Streams, redirection, and pipes](/learn/linux-streams) — the three streams
  every process gets, how the shell repoints them, and the `2>&1` ordering trap.
- [File permissions and links](/learn/linux-permissions) — the user/group/other
  model, `chmod` in both notations, umask defaults, and symbolic links.
- [Archives and compression](/learn/linux-archives) — why `tar` and `gzip` are
  separate tools, and what metadata an archive preserves that `cp` does not.

This wiki expands as finished notes and completed labs move from my private study
workspace into reviewed public editions.

## Authorship and scope

The educational substance begins in notes I typed while studying or in labs I
performed and documented. AI may help interview me, organize the material, correct
presentation, and fit it into the site, but it is not used to manufacture unlearned
expertise or unperformed evidence.

The material is FOSS-first because free tools and owned systems make practice more
accessible. Proprietary workplace systems will still be addressed where reality
requires them, with a clear distinction between transferable concepts, documentation-
based comparisons, and systems I have personally operated.
