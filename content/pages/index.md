---
title: "Education wiki"
date: 2026-07-23
summary: "Free, FOSS-first foundations in networking and Linux, each page paired with practice you can run on hardware you already own."
tags: [education, networking, linux, digital-literacy, digital-sovereignty]
---

## Who this is for

This wiki is for curious adults who use technology every day but want to understand,
operate, troubleshoot, and eventually build more of it themselves. It begins with the
systems and networking foundations I am studying for an early career in Linux systems
administration and network operations.

The aim is enough context to see how the pieces fit together, read ordinary
documentation, and pick a useful next step. It is not meant to make anyone a
specialist.

## Understand → Practice → Evidence

Each topic has up to three parts:

1. **Understand** — connect a concept to the larger system and explain it in ordinary
   language before getting into protocols and specifications.
2. **Practice** — use a simple lab to observe, configure, break, or troubleshoot the
   concept. Most suggested labs work on an ordinary computer with free and
   open-source software.
3. **Evidence** — link the topic to finished, documented work where I have used it
   myself.

Not every topic needs all three. A foundational page can stand alone. An applied claim
should be backed by work I did and can explain.

## Practice what is here

Two other parts of the site are built from these pages:

- [Glossary](/glossary) — every term and command defined here, each linked back
  to the section that explains it.
- [Study](/study) — practice questions and command scenarios. Every answer
  carries an explanation of why the other options are wrong, and a link to the
  page it came from.

## Networking foundations

Grouped by subject, from how networks are described down to the wire and back up
to the services on top. The sidebar has a toggle that lists the same pages by CCNA
or Network+ objective instead.

### Models and patterns

- [OSI model](/learn/osi-model) — a seven-layer reference model for connecting
  protocols, devices, encapsulation, and troubleshooting to the larger communication
  system.
- [Network topologies](/learn/network-topologies) — how nodes and links are arranged,
  from mesh and star to spine-and-leaf and tiered enterprise designs.
- [Network traffic types](/learn/traffic-types) — unicast, multicast, anycast, and
  broadcast delivery patterns.

### Physical layer

- [Transmission media](/learn/transmission-media) — the radio, copper, and fiber
  that carry data, what bounds each one, and who standardizes what.
- [Wired media](/learn/wired-media) — copper categories, the 802.3 standards,
  single-mode and multimode fiber, coax, and plenum ratings.
- [Wireless media](/learn/wireless-media) — taking turns on a shared channel,
  the 802.11 generations and their bands, cellular, and satellite.
- [Transceivers and connectors](/learn/transceivers) — pluggable modules, their form
  factors, and the connectors that terminate copper and fiber.

### Addressing

- [IPv4 addressing](/learn/ipv4-addressing) — binary octets, public and private
  ranges, subnet masks, CIDR, and variable length subnet masking.
- [Subnetting, CIDR, and VLSM](/learn/subnetting) — counting hosts and networks,
  the mask as a bitwise test, prefix notation, and sizing subnets to fit.
- [IPv6 addressing](/learn/ipv6-addressing) — hextets and zero compression, the
  address types on a working interface, neighbor discovery, and coexistence with
  IPv4.

### Local networks

- [Switching technologies](/learn/switching-technologies) — VLANs and trunks, the
  802.1Q tag, link aggregation, spanning tree, and what bounds the size of a frame.
- [Wireless technologies](/learn/wireless-technologies) — channels and channel
  width, SSIDs and roaming, wireless network types, WPA2 and WPA3, 802.1X, and
  access point management.

### Between networks

- [Routing technologies and route selection](/learn/routing-technologies) — static
  and dynamic routing, BGP, EIGRP, and OSPF, how a router breaks a tie between two
  matching routes, NAT, and first-hop redundancy.
- [VPNs and IPsec](/learn/vpns-and-ipsec) — tunneling, site-to-site and remote-access
  VPNs, and how AH, ESP, and IKE fit together.
- [Quality of service](/learn/quality-of-service) — classifying, marking, queuing, and
  limiting competing traffic.

### Services and devices

- [Network protocols and ports](/learn/network-protocols) — common application
  protocols, the port ranges IANA allocates, and the network-layer protocols that
  carry them.
- [Network appliances](/learn/network-appliances) — the physical and virtual systems
  that forward, filter, distribute, store, and expose network traffic.
- [Content delivery networks](/learn/content-delivery-networks) — serving content
  from the edge, and where the CDN ends and the application begins.

### Modern environments

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

New pages are added as I finish notes and labs and review them as standalone
lessons.

## Authorship and scope

Every page starts from notes I typed while studying, or from labs I ran and
documented. AI helps interview me, organize the material, fix the presentation, and
fit it into the site. It is not used to make up expertise I have not learned or
evidence for work I have not done.

The material is FOSS-first because free tools and your own hardware make practice
easier to get to. Proprietary workplace systems still come up where they matter, and
the pages say which parts are general concepts, which are comparisons from
documentation, and which are systems I have run myself.
