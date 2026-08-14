---
title: "Wired media"
date: 2026-08-14
summary: "Twisted-pair copper and its categories, the 802.3 Ethernet standards, single-mode and multimode fiber with the OM grades, direct attach copper, coax, and plenum fire ratings."
tags: [education, networking, physical-layer, cabling, fiber, ethernet]
---

## Overview

A cable is a private medium in a way radio never is. Nobody else's traffic
shares your patch lead, the path is bounded and measurable, and a station can
listen to the wire while transmitting on it — which is why wired Ethernet could
*detect* collisions where [wireless](/learn/wireless-media) can only try to
avoid them.

What limits a wired link is the cable itself: how far a signal travels before it
degrades, how much interference it picks up, and how much bandwidth its
construction supports. Those limits are why cable categories, fiber grades, and
distance ratings exist, and why "it plugged in and linked up" is not the same as
"it will work reliably."

However convenient wireless is, nearly every network is wired somewhere — and
wired media are generally the fastest, most reliable, and hardest to intercept
of the three. This page covers the Ethernet standards, copper conductors, fiber
optics, and the material properties that bound each one.

The IEEE defines Ethernet in the **802.3** family, which specifies how data
crosses physical cabling in a wired LAN. A link can be **full-duplex**, sending
in both directions simultaneously, or **half-duplex**, sending in one direction
at a time.

Standards are what make equipment from different vendors interoperate.
Interoperability has limits worth respecting: fiber optics and twisted-pair
copper are not interchangeable, and mixing devices rated for different speeds
means the link negotiates down to what both ends support.

## Collisions on a shared cable

Shared half-duplex Ethernet has the same contention problem as Wi-Fi, and 802.3
solves it with **carrier-sense multiple access with collision detection
(CSMA/CD)**:

- A device listens for a quiet moment on the cable and waits if it is busy.
- Devices sharing the cable transmit by taking turns.
- If two transmit simultaneously, the collision corrupts both frames.
- Both back off for a random interval before retrying, which makes a second
  collision between the same pair unlikely.

The region of the network where collisions are possible is the **collision
domain**. Switched full-duplex links, which is nearly all modern wired
Ethernet, give each port its own collision domain and remove the contention
entirely — CSMA/CD is legacy behavior on those links, not active mechanism.

## Ethernet standards

| Standard | Max speed | Medium | Nominal max distance |
|---|---:|---|---:|
| 10BASE2 | 10 Mbps | Thin coaxial | 185 m |
| 10BASE-T | 10 Mbps | Cat 3 twisted pair | 100 m |
| 10BASE-F | 10 Mbps | Multimode fiber | 2,000 m |
| 100BASE-TX | 100 Mbps | Cat 5 twisted pair | 100 m |
| 1000BASE-T | 1 Gbps | Cat 5e/6 twisted pair | 100 m |
| 1000BASE-SX | 1 Gbps | Multimode fiber | 220–550 m |
| 1000BASE-LX | 1 Gbps | Single-mode fiber | 5,000 m |
| 10GBASE-T | 10 Gbps | Cat 6a/7 twisted pair | 100 m |
| 10GBASE-SR | 10 Gbps | Multimode fiber | 300–400 m |
| 10GBASE-LR | 10 Gbps | Single-mode fiber | 10 km |
| 40GBASE-LR4 | 40 Gbps | Single-mode fiber | 10 km |
| 100GBASE-LR4 | 100 Gbps | Single-mode fiber | 10 km |

The twisted-pair distance limit of 100 m is a property of the whole channel,
including patch cords and terminations, not just the run of cable in the wall.
Fiber distances vary with the grade of fiber and the optic driving it; the
figures above are planning numbers.

## Fiber Ethernet and OM grades

**Fiber Ethernet** transmits data as pulses of light through glass. Light
travels further before needing regeneration than an electrical signal in
copper, and fiber is immune to **electromagnetic interference (EMI)**, which
makes it the reliable choice near motors, fluorescent lighting, and heavy power
runs.

Multimode fiber is graded by **optical multimode (OM)** classes, which describe
the reach a given fiber supports at a given speed:

| Grade | Description |
|---|---|
| OM1 | 62.5 µm core, LED sources. Roughly 1 Gbps to 300 m, 10 Gbps to 33 m |
| OM2 | 50 µm core. Roughly 1 Gbps to 600 m, 10 Gbps to 82 m |
| OM3 | Laser-optimized. 10 Gbps to 300 m, 40/100 Gbps to 100 m |
| OM4 | Enhanced OM3. 10 Gbps to 550 m, 40/100 Gbps to 150 m |
| OM5 | Wideband, supports shortwave wavelength division multiplexing (SWDM) |

Treat these as nominal. The reach a link actually achieves depends on the
fiber's modal bandwidth, the wavelength, the transceiver, and the loss budget
of the installed run.

## Single-mode and multimode fiber

**Single-mode fiber** has a narrow core, around 9 µm, that carries light along
essentially one path. Without multiple paths there is no modal dispersion, so
it holds a signal over long distances. Telecoms and **Internet service
providers (ISPs)** use it for backhaul and long-haul links.

**Multimode fiber** has a wider core that admits several light paths at once.
Those paths arrive at slightly different times — modal dispersion — which
limits usable distance. It is the practical choice inside buildings, campuses,
and data centers.

The usual "single-mode costs more" shorthand is about the optics, not the
glass. Single-mode transceivers need tighter alignment and more precise light
sources, and that is where the cost difference lives.

## Direct attach copper

Copper is simpler than fiber over short distances: no optics to align, less
power per link, and more tolerance for rough handling. **Direct attach copper
(DAC)** cables use **twinaxial** ("twinax") construction — two insulated
conductors inside a shield — with the transceiver ends permanently attached.
They are common for top-of-rack switch connections a few meters long.

## Coaxial cable

Coaxial cable carries a signal on a single center conductor inside a shield. It
predates twinax in LAN use, reaches further than twinax but at lower speeds,
and is more durable and easier to work with than fiber. It survives today in
cable broadband and video distribution rather than in LANs.

## Plenum cabling

A **plenum space** is a void used to move air for **heating, ventilation, and
air conditioning (HVAC)** — typically above a drop ceiling or below a raised
floor. Cable run through one has to be plenum-rated: flame-retardant jacketing
that produces little smoke and low toxicity when it burns, so a fire is not
distributed through the building's air handling. Plenum cable costs more than
standard cable, and where it is required, it is required by fire and building
code rather than by preference.

## Study-note shortcuts worth correcting

- **CSMA/CD detects collisions; CSMA/CA avoids them.** A wired station can hear
  the cable while transmitting. On a modern full-duplex switched link there are
  no collisions to detect at all, which is why CSMA/CD is effectively history.
- **Category ratings describe the cable and the whole channel.** A Cat 6A patch
  lead does not rescue a Cat 5e run in the wall, and connectors count.
- **Fiber is immune to EMI, not indifferent to everything.** Bend radius,
  contaminated connector endfaces, and accumulated loss all break fiber links.
- **Single-mode is not simply "the fast one."** It carries further because the
  core is narrow enough to admit one path, which is a distance property before
  it is a speed one.
- **Plenum rating is a fire-code requirement, not an upgrade.** Running
  non-plenum cable in a plenum space is a code violation regardless of how well
  the link performs.

## Suggested practice: read the media in your own links

On a network you own:

1. List your interfaces with `ip -br link`, then run `ethtool <interface>` on a
   wired one. Record the negotiated speed and duplex, and compare them with what
   the port and cable are rated for.
2. Check `ip -s link` for error and drop counters on that interface. A link can
   negotiate successfully and still be marginal.
3. Read the jacket printing on a patch cable. Note its category and its fire
   rating (CM, CMR, or CMP), then find where it is run and decide whether the
   rating matches the space.
4. Swap a suspect cable for a known-good one of the same category and re-check
   `ethtool` and the error counters. This is the cheapest layer-1 test there is.
5. If you have fiber, identify the connector type and whether the run is
   single-mode or multimode from the jacket colour and printing, then check it
   against the optic in the transceiver. A mismatch links up and performs badly.

This shows what your equipment negotiated. It does not show why — signal
quality and cable condition need dedicated tools to measure rather than infer.

## Related pages

- [Transmission media](/learn/transmission-media) — the overview, and how copper
  and glass compare with radio.
- [Wireless media](/learn/wireless-media) — the shared-channel problem a cable
  does not have.
- [Transceivers and connectors](/learn/transceivers) — the modules and plugs
  that attach a device to these media.
- [The OSI model](/learn/osi-model) — where physical signalling sits relative to
  framing and addressing.
- [Network appliances](/learn/network-appliances) — the switches and routers
  terminating these links.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [IEEE 802.3 Ethernet Working Group](https://www.ieee802.org/3/) — the wired
  Ethernet standards, port types, and media specifications.
- [TIA-568 structured cabling](https://tiaonline.org/) — the category
  definitions and channel requirements, referenced rather than reproduced.

Structured cabling grades (ISO/IEC 11801, TIA-568) and plenum fire ratings (NEC
Article 800 and the UL listings behind it) come from standards bodies that sell
their documents rather than publishing them openly. For a specific cable or
optic, the vendor datasheet and the applicable local code are the references
that matter.
