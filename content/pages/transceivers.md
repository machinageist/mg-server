---
title: "Transceivers and connectors"
date: 2026-08-02
summary: "Pluggable transceiver modules, the form factors that define their shape, and the connectors that terminate copper and fiber."
tags: [education, networking, physical-layer, fiber, cabling]
---

## Overview

A **transceiver** combines a **transmitter** and a **receiver** in one unit. It
is the interface between a device's electronics and the medium carrying its
traffic: it converts signals the device produces into light or electrical
signals on the wire, and converts what arrives back again.

Network equipment increasingly uses pluggable transceivers rather than fixed
ports. One switch port can be fitted for twisted-pair copper, short-range
multimode fiber, or long-haul single-mode fiber by changing the module instead
of the switch. Most pluggable modules are **hot-swappable** — they can be
replaced while the device is powered and in service.

## What a transceiver carries

The IEEE and other standards bodies define what a module must do on the wire so
that equipment from different vendors interoperates.

### Ethernet

Ethernet is the protocol most transceivers serve. The 802.3 standards cover
both copper and fiber at every speed grade, and each port type — 1000BASE-LX,
10GBASE-SR, 100GBASE-LR4 — specifies the medium, wavelength, and reach the
module has to support. [Transmission media](/learn/transmission-media) covers
those port types in more detail.

### Fibre Channel

Fibre Channel is a separate high-speed transport built for storage traffic,
most often in a **storage area network (SAN)**, and it runs predominantly over
fiber optics. **Fibre Channel Protocol (FCP)** is the mapping that carries SCSI
commands across it — the protocol riding the transport, not another name for
the transport itself. Fibre Channel equipment uses the same families of
pluggable modules as Ethernet gear.

## Form factors

A **form factor** describes a module's physical shape, size, and electrical
interface. These come from multi-source agreements between manufacturers rather
than from the IEEE, which is why the naming runs on a separate track from the
Ethernet port types.

| Form factor | Meaning | Typical speed |
|---|---|---:|
| SFP | Small form-factor pluggable | 1 Gbps |
| SFP+ | Same body, faster electrical interface | 10 Gbps |
| SFP28 | Single-lane 25G variant | 25 Gbps |
| QSFP+ | Quad small form-factor pluggable, four lanes | 40 Gbps |
| QSFP28 | Four 25G lanes | 100 Gbps |

The SFP family holds one channel per module and covers most access and
distribution links. The QSFP family carries four lanes in one module, which is
what makes 40G and 100G practical in high-performance data centers and cloud
infrastructure — and what allows a breakout cable to split one QSFP port into
four independent SFP-speed links.

## Connector types

Connectors attach cabling to network devices and interfaces. Fiber and copper
each have their own families, and several remain in service mainly because
existing plant was built with them.

| Connector | Medium | Where it appears |
|---|---|---|
| LC (local connector) | Fiber | The modern default; small snap-in latch, pairs into SFP modules |
| SC (subscriber connector) | Fiber | Larger square push-pull body, patch panels and older plant |
| ST (straight tip) | Fiber | Bayonet twist-lock, legacy multimode installations |
| MPO (multi-fiber push-on) | Fiber | Ribbon connector carrying 8–24 fibers for high-density trunks and QSFP breakouts |
| RJ11 | Twisted pair | Telephone and **digital subscriber line (DSL)** connections |
| RJ45 | Twisted pair | Ethernet on Cat 5e, 6, and 7 cabling |
| F-type | Coaxial | Threaded connector for cable television and cable broadband |
| BNC (Bayonet Neill–Concelman) | Coaxial | Legacy coaxial LANs, still common on video and test equipment |

BNC **barrels** couple two BNC cables end to end, and T-connectors tap a cable
— the arrangement that shared-media coaxial Ethernet depended on.

## Reading the label

- **An SFP cage is not a media converter.** The port accepts a module; the
  module determines whether the link is copper or fiber. Converting between two
  media requires a media converter, or two devices each fitted for their own
  side.
- **"Hot-swappable" describes the hardware, not the link.** Pulling and
  reseating a module avoids powering the chassis down, but the link still has
  to negotiate, and the port may need configuration before it comes up.
- **RJ45 is a colloquial name.** The Ethernet connector is an 8P8C modular
  plug. The registered jack designation RJ45S refers to a different telephone
  wiring specification that never described Ethernet.
- **Matching the form factor is not the same as being supported.** Modules
  carry vendor identification in their EEPROM, and some platforms refuse optics
  they do not recognize even when the module is electrically correct.
- **Both ends have to agree on more than shape.** An 850 nm multimode optic
  will not link with a 1310 nm single-mode optic, though both accept an LC
  connector and both fit an SFP cage.

## Suggested practice: identify what is plugged into your own gear

On equipment you own:

1. Inventory the ports on a switch, router, or network card and name the
   connector on each one. Note which ports are fixed and which take a module.
2. On Linux, run `ethtool <interface>` and record the port type and negotiated
   speed. Where a pluggable module is present and the driver supports it,
   `ethtool -m <interface>` reads the module's EEPROM: vendor, part number,
   wavelength, and rated reach.
3. Compare the module's rated reach with the length of the run it serves. Note
   how much margin exists.
4. Trace one link end to end and write down every connector and coupling it
   passes through. Each one is a point of loss on fiber and a point of failure
   on either medium.

Handle fiber carefully while doing this. Keep dust caps on unused connectors,
avoid touching the endface, and never look into a fiber or a populated optical
port — the light is invisible and can injure your eye.

## Related pages

- [Transmission media](/learn/transmission-media) — the copper, fiber, and
  radio these modules and connectors terminate.
- [The OSI model](/learn/osi-model) — where physical signaling sits relative to
  framing and addressing.
- [Network appliances](/learn/network-appliances) — the switches, routers, and
  storage systems these modules plug into.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [IEEE 802.3 Ethernet Working Group](https://www.ieee802.org/3/) — the port
  types a transceiver has to implement, including medium, wavelength, and
  reach.

Form factors are defined by multi-source agreements and the SFF specifications
maintained by the SFF Technology Affiliate group; fiber connector geometry
comes from the TIA-604 (FOCIS) series. Neither is published openly. For a
specific module, the vendor datasheet and the platform's compatibility matrix
are the references that determine whether a link will come up.
