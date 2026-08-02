---
title: "Transmission media"
date: 2026-08-02
summary: "The physical media that carry data — radio, copper, and fiber — and the 802.11 and 802.3 standards that govern how each one is shared."
tags: [education, networking, physical-layer, wireless, cabling, network-plus]
---

## Overview

**Transmission media** are the physical elements that carry data between
systems: radio waves, copper conductors, and glass fiber. Everything above them
in [the OSI model](/learn/osi-model) depends on a medium moving a signal from
one place to another successfully.

Two questions organize the subject. First, how does a medium carry a signal,
and what limits its speed and distance? Second, when several systems share one
medium, how do they take turns without destroying each other's transmissions?

## Wireless media

Radio is the medium most people meet first: cellular networks, Bluetooth, smart
home devices, and Wi-Fi. It is convenient because nothing has to be cabled to
the client, and difficult for the same reason. The medium is shared with every
other transmitter in range, including ones on networks nobody in the building
controls.

### Taking turns on a shared channel

A Wi-Fi station cannot listen while it transmits, so it cannot detect a
collision the way a wired station can. It tries to avoid one instead.
**Carrier-sense multiple access with collision avoidance (CSMA/CA)** works like
this:

1. A station listens for activity on the channel it intends to use.
2. If the channel is busy, it waits and checks again.
3. If the channel is clear, it transmits.
4. The receiver checks the frame for errors and returns an **acknowledgment
   (ACK)**.
5. If no ACK arrives, the sender treats the frame as lost and retries.

Two optional control frames protect longer transmissions. A **request to send
(RTS)** announces how long the sender expects to occupy the channel. The
receiver answers with a **clear to send (CTS)**, which other stations in range
hear as an instruction to wait.

A typical Wi-Fi network is hub-and-spoke: several devices associate with one
wireless router or access point. In 802.11 terminology the client devices are
**stations**.

### 802.11 standards

The **Institute of Electrical and Electronics Engineers (IEEE)** publishes the
standards for both wireless and wired LAN technology. The 802.11 family covers
wireless LANs.

| Standard | Max PHY rate | Band | Nominal range | Notes |
|---|---:|---|---:|---|
| 802.11a | 54 Mbps | 5 GHz | 35 m | Early OFDM standard, less crowded band |
| 802.11b | 11 Mbps | 2.4 GHz | 40 m | Slower, longer reach, noisy band |
| 802.11g | 54 Mbps | 2.4 GHz | 40 m | 802.11b-compatible, faster |
| 802.11n (Wi-Fi 4) | 600 Mbps | 2.4 and 5 GHz | 70 m | Dual-band, MIMO antennas, channel bonding |
| 802.11ac (Wi-Fi 5) | 6.9 Gbps | 5 GHz | 35 m | Wider channels, more simultaneous devices |
| 802.11ax (Wi-Fi 6) | 9.6 Gbps | 2.4 and 5 GHz | 70 m | OFDMA; holds up better in crowded environments |

Wi-Fi 6E extends 802.11ax into the 6 GHz band where regulators allow it. A
separate amendment, **802.11h**, added dynamic frequency selection and transmit
power control so 5 GHz networks can share spectrum with radar systems; it is an
amendment to 5 GHz operation, not a band variant of 802.11g.

### Cellular networks

**Cellular networks** are built and operated by telecom carriers. A mobile
device associates with a nearby tower, which connects onward to the carrier's
core network.

- **2G** and **3G** are deprecated and largely decommissioned.
- **4G** and **5G** carry modern mobile traffic.
- **6G** is still in research and standardization.

**Long Term Evolution (LTE)** drove the transition from voice-centric cellular
networks to data networks capable of ordinary Internet access. 5G improves on
it with higher throughput and lower latency, but at shorter range per site,
which means denser tower deployments.

**Narrowband-IoT (NB-IoT)** is a low-power profile that occupies a narrow slice
of the carrier's spectrum. It transmits slowly — roughly 20–100 kbps — but
penetrates walls and reaches underground locations well, which suits metering
and sensor devices that send small readings infrequently.

The **Global System for Mobile Communications (GSM)** family identifies
subscribers with a SIM card, which is what makes a device portable between
networks and carriers internationally. **Code division multiple access (CDMA)**
was a competing approach used in the United States by carriers such as Verizon
and Sprint; CDMA networks identified devices in the network rather than with a
removable card. Those networks have been retired in favor of LTE and 5G.

### Satellite

Satellite links carry traffic where terrestrial coverage does not reach. The
distance a signal travels to orbit and back introduces **latency**, signal
strength falls off with the square of the distance, and atmospheric conditions
degrade the link further.

**Geostationary (GEO)** satellites orbit at roughly 35,000 km (22,000 miles)
and hold a fixed position over one point on the ground. A single GEO satellite
covers a large area consistently and the ground dish can stay pointed in one
direction, but the round trip adds substantial latency.

**Low-earth orbit (LEO)** satellites orbit much closer, which cuts latency
considerably. They do not hold a fixed position, so the ground station has to
track them — mechanically, or with an electronically steered phased array — and
coverage depends on a constellation of many satellites handing off to each
other. Providers such as Starlink offer consumer Internet access this way.

**Global Positioning System (GPS)** and its counterpart constellations are a
different use of the same infrastructure. A receiver measures how long signals
took to arrive from several satellites and solves for its own position.

## Wired media

However convenient wireless is, nearly every network is wired somewhere. Wired
media carry data over physical cabling and are generally the fastest, most
reliable, and hardest to intercept of the three. The relevant technologies are
Ethernet standards, copper conductors, fiber optics, and the material
properties that limit each one.

The IEEE defines Ethernet in the **802.3** family, which specifies how data
crosses physical cabling in a wired LAN. A link can be **full-duplex**, sending
in both directions simultaneously, or **half-duplex**, sending in one direction
at a time.

Standards are what make equipment from different vendors interoperate.
Interoperability has limits worth respecting: fiber optics and twisted-pair
copper are not interchangeable, and mixing devices rated for different speeds
means the link negotiates down to what both ends support.

### Collisions on a shared cable

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

### Ethernet standards

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

### Fiber Ethernet and OM grades

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

### Single-mode and multimode fiber

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

### Direct attach copper

Copper is simpler than fiber over short distances: no optics to align, less
power per link, and more tolerance for rough handling. **Direct attach copper
(DAC)** cables use **twinaxial** ("twinax") construction — two insulated
conductors inside a shield — with the transceiver ends permanently attached.
They are common for top-of-rack switch connections a few meters long.

### Coaxial cable

Coaxial cable carries a signal on a single center conductor inside a shield. It
predates twinax in LAN use, reaches further than twinax but at lower speeds,
and is more durable and easier to work with than fiber. It survives today in
cable broadband and video distribution rather than in LANs.

### Plenum cabling

A **plenum space** is a void used to move air for **heating, ventilation, and
air conditioning (HVAC)** — typically above a drop ceiling or below a raised
floor. Cable run through one has to be plenum-rated: flame-retardant jacketing
that produces little smoke and low toxicity when it burns, so a fire is not
distributed through the building's air handling. Plenum cable costs more than
standard cable, and where it is required, it is required by fire and building
code rather than by preference.

## Study-note shortcuts worth correcting

- **Advertised rates are shared, aggregate PHY rates.** The 9.6 Gbps figure for
  Wi-Fi 6 is the theoretical total across the channel under ideal conditions,
  not throughput available to one device.
- **Published ranges are nominal.** Walls, interference, antenna design, and
  transmit power move the real number substantially in either direction.
- **CSMA/CA avoids collisions; CSMA/CD detects them.** The difference is
  physical: a wired station can hear the cable while transmitting, and a
  wireless station cannot hear itself over its own transmission.
- **802.11g has no 5 GHz variant.** Radar coexistence in 5 GHz came from
  802.11h.
- **Cellular did not abandon the SIM card.** CDMA was the network type that
  identified devices without one. LTE and 5G use SIM and eSIM.
- **GPS position fixing is trilateration, not triangulation.** The receiver
  solves from measured distances, not measured angles.
- **Fiber is immune to EMI, not indifferent to everything.** Bend radius,
  contaminated connector endfaces, and accumulated loss all break fiber links.

## Suggested practice: read the media in your own links

On a network you own:

1. List your interfaces with `ip -br link`, then run `ethtool <interface>` on a
   wired one. Record the negotiated speed and duplex, and compare them with
   what the port and cable are rated for.
2. Check `ip -s link` for error and drop counters on that interface. A link can
   negotiate successfully and still be marginal.
3. Read the jacket printing on a patch cable. Note its category and its fire
   rating (CM, CMR, or CMP), then find where it is run.
4. On a wireless client, use `iw dev` or `nmcli dev wifi list` to see the band,
   channel, and channel width in use, and match them to the 802.11 table above.
5. Move the client and repeat step 4. Watch the negotiated rate change while
   the standard stays the same.

This shows what your equipment negotiated. It does not show why — signal
quality, interference, and cable condition need dedicated tools to measure
rather than infer.

## Related pages

- [The OSI model](/learn/osi-model) — where physical media, framing, and
  addressing sit relative to each other.
- [Transceivers and connectors](/learn/transceivers) — the modules and plugs
  that attach a device to one of these media.
- [Network appliances](/learn/network-appliances) — the switches, routers, and
  access points terminating these links.

## Sources and further reading

This page was edited from my networking reading notes and checked against:

- [IEEE 802.3 Ethernet Working Group](https://www.ieee802.org/3/) — the wired
  Ethernet standards, port types, and media specifications.
- [IEEE 802.11 Wireless LAN Working Group](https://www.ieee802.org/11/) — the
  wireless LAN standards and their amendments.
- [GPS.gov](https://www.gps.gov/) — official reference for GPS system operation
  and positioning.

Structured cabling grades (ISO/IEC 11801, TIA-568) and plenum fire ratings
(NEC Article 800 and the UL listings behind it) come from standards bodies that
sell their documents rather than publishing them openly. For a specific cable
or optic, the vendor datasheet and the applicable local code are the references
that decide the question.
