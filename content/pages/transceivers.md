---
title: "Transceivers and connectors"
date: 2026-08-02
summary: "Pluggable transceiver modules, the form factors that define their shape, and the connectors that terminate copper and fiber."
tags: [education, networking, physical-layer, fiber, cabling]
---

## Overview

A transceiver combines a transmitter and a receiver in one unit. It
is the interface between a device's electronics and the medium carrying its
traffic: it converts signals the device produces into light or electrical
signals on the wire, and converts what arrives back again.

More and more network equipment uses pluggable transceivers instead of fixed
ports. One switch port can be fitted for twisted-pair copper, short-range
multimode fiber, or long-haul single-mode fiber by changing the module instead
of the switch. Most pluggable modules are hot-swappable, meaning they can be
replaced while the device is powered on and in service.

## What a transceiver carries

The IEEE and other standards bodies define what a module must do on the wire so
that equipment from different vendors interoperates.

### Ethernet

Ethernet is the protocol most transceivers serve. The 802.3 standards cover
both copper and fiber at every speed grade, and each port type, such as
1000BASE-LX, 10GBASE-SR, or 100GBASE-LR4, specifies the medium, wavelength, and
reach the module has to support. [Transmission media](/learn/transmission-media) covers
those port types in more detail.

### Fibre Channel

Fibre Channel is a separate high-speed transport built for storage traffic,
most often in a storage area network (SAN), and it runs predominantly over
fiber optics. Fibre Channel Protocol (FCP) is the mapping that carries SCSI
commands across it. FCP runs over Fibre Channel. It is not another name for it. Fibre Channel equipment uses the same families of
pluggable modules as Ethernet gear.

## Form factors

A form factor describes a module's physical shape, size, and electrical
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
distribution links. The QSFP family carries four lanes in one module. That is
what makes 40G and 100G practical in data centers and cloud infrastructure, and
it is what lets a breakout cable split one QSFP port into four separate
SFP-speed links.

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

BNC barrels join two BNC cables end to end, and T-connectors tap into a cable.
Shared-media coaxial Ethernet depended on that arrangement.

## Reading the label

- **An SFP cage is not a media converter.** The port accepts a module, and the
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
port. The light is invisible and can injure your eye.

## Exam key points

Where this page's material shows up in the exam objectives, and what each exam
expects beyond it.

### CCNA 200-301

Objective 1.3.a in v1.1 compares single-mode fiber, multimode fiber, and copper.
In v2.0, which replaces it in February 2027, 1.1 covers diagnosing fiber problems,
including distance and signal levels.

- Pick an optic by fiber type and distance. A short-range multimode optic will not
  work on single-mode fiber.
- `show interfaces transceiver` shows transmit and receive power for modules that
  support monitoring. Low receive power points at a dirty connector, a bad patch,
  or too much distance.
- Fibre Channel and BNC connectors are not on the CCNA blueprint.

### Network+ N10-009

Objective 1.5 lists transceiver protocols (Ethernet and Fibre Channel), the SFP
and QSFP form factors, and the connector types this page covers.

- Know the connectors by sight: SC, LC, ST, MPO, RJ11, RJ45, F-type, and BNC.
- SFP is one channel per module. QSFP carries four lanes and makes 40G and 100G
  practical.
- Transceiver mismatch and signal strength also appear as troubleshooting items in
  5.2.

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
maintained by the SFF Technology Affiliate group. Fiber connector geometry
comes from the TIA-604 (FOCIS) series. Neither is published openly. For a
specific module, the vendor datasheet and the platform's compatibility matrix
are the references that determine whether a link will come up.
