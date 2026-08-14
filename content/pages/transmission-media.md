---
title: "Transmission media"
date: 2026-08-02
summary: "The physical media that carry data — radio, copper, and glass — what limits each one, and how systems sharing a medium take turns without destroying each other's transmissions."
tags: [education, networking, physical-layer, cabling, wireless]
---

## Overview

**Transmission media** are the physical elements that carry data between
systems: radio waves, copper conductors, and glass fiber. Everything above them
in [the OSI model](/learn/osi-model) depends on a medium moving a signal from
one place to another successfully — which is why a surprising share of network
faults turn out to be layer 1.

Two questions organize the whole subject:

1. **How does a medium carry a signal, and what limits its speed and distance?**
   Every medium has physical bounds. Copper attenuates and picks up
   interference, glass has a bend radius and a loss budget, radio has range that
   walls and other transmitters degrade.
2. **When several systems share one medium, how do they take turns?** A shared
   medium needs a rule for who transmits when, and the rule differs depending on
   whether a station can hear the medium while using it.

That second question is the cleanest dividing line in the subject, and it is why
this material splits in two.

## Bounded and unbounded media

**Bounded media** confine the signal to a physical path — a copper pair, a glass
core. The path is private, measurable, and interceptable only by touching it. A
station can listen to the cable while transmitting on it, which is what made
collision *detection* possible on early Ethernet.

**Unbounded media** radiate into shared space. Nothing confines the signal, the
medium is shared with every transmitter in range including ones nobody in the
building controls, and a station cannot hear the channel over its own
transmission. That single physical fact is why wireless has to *avoid*
collisions rather than detect them.

## The two halves

- **[Wireless media](/learn/wireless-media)** — how stations take turns on a
  shared channel with CSMA/CA, the 802.11 generations and the bands they use,
  and where cellular and satellite links differ from Wi-Fi.
- **[Wired media](/learn/wired-media)** — twisted-pair copper and its
  categories, the 802.3 Ethernet standards, single-mode and multimode fiber with
  the OM grades, direct attach copper, coax, and plenum fire ratings.

## Who standardizes what

Media standards come from a different body than the protocols that ride on them,
and knowing which is which saves an argument:

| Body | Owns |
|---|---|
| IEEE | 802.3 Ethernet, 802.11 wireless — the physical and link layers |
| IETF | IP, TCP, UDP and the RFCs above them |
| TIA / ISO | Structured cabling grades and installation practice |
| Local electrical code | Fire ratings for cable run in a building |

The practical consequence is that a cable can satisfy TIA-568 for performance
and still be illegal in the space it is installed in, because those are two
different standards answering two different questions.

## Suggested practice: identify every medium in one path

Trace one real connection end to end and name the medium at each hop.

1. Pick a device on your own network and follow its path to the internet: the
   client's link, the run to the switch, the switch's uplink, the connection to
   the modem or ONT, and the service entering the building.
2. For each hop, write down whether the medium is bounded or unbounded, and what
   physically limits it — distance, interference, contention, or a contract.
3. Identify which hop you would suspect first if throughput dropped by half, and
   why. Usually it is the one you have the least visibility into.
4. Work the medium-specific checks on
   [wired](/learn/wired-media#suggested-practice-read-the-media-in-your-own-links)
   and [wireless](/learn/wireless-media#suggested-practice-watch-a-wireless-link-negotiate)
   links, and compare what each one negotiated against what it is rated for.

## Related pages

- [Wireless media](/learn/wireless-media) — radio, 802.11, cellular, satellite.
- [Wired media](/learn/wired-media) — copper, fiber, and the 802.3 standards.
- [Transceivers and connectors](/learn/transceivers) — the modules and plugs
  that attach a device to one of these media.
- [The OSI model](/learn/osi-model) — where physical media, framing, and
  addressing sit relative to each other.
- [Network appliances](/learn/network-appliances) — the switches, routers, and
  access points terminating these links.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [IEEE 802.3 Ethernet Working Group](https://www.ieee802.org/3/) — the wired
  Ethernet standards, port types, and media specifications.
- [IEEE 802.11 Wireless LAN Working Group](https://www.ieee802.org/11/) — the
  wireless LAN standards and their amendments.

Structured cabling grades (ISO/IEC 11801, TIA-568) and plenum fire ratings (NEC
Article 800 and the UL listings behind it) come from standards bodies that sell
their documents rather than publishing them openly. For a specific cable or
optic, the vendor datasheet and the applicable local code are the references
that matter.
