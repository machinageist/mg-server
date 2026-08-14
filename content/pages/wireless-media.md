---
title: "Wireless media"
date: 2026-08-14
summary: "How stations take turns on a shared radio channel, the 802.11 generations and the bands they use, and where cellular and satellite links differ from Wi-Fi."
tags: [education, networking, physical-layer, wireless, wifi, cellular]
---

## Overview

Radio is the medium most people meet first — cellular networks, Bluetooth, smart
home devices, and Wi-Fi. It is convenient because nothing has to be cabled to
the client, and difficult for exactly the same reason. The medium is shared with
every other transmitter in range, including ones on networks nobody in the
building controls.

The organizing question for any shared medium is how systems take turns without
destroying each other's transmissions. Wireless answers it differently from
[wired media](/learn/wired-media), and the reason is physical: a station cannot
hear the channel over its own transmission, so it cannot detect a collision
while causing one.

## Taking turns on a shared channel

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

## 802.11 standards

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

## Cellular networks

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

## Satellite

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

## Suggested practice: watch a wireless link negotiate

On a network you own:

1. On a wireless client, use `iw dev` or `nmcli dev wifi list` to see the band,
   channel, and channel width in use, and match them to the 802.11 table above.
2. Move the client further from the access point and repeat. Watch the
   negotiated rate change while the standard stays the same — the generation is
   a ceiling, not a promise.
3. Run `iw dev <interface> scan | grep -E "SSID|freq|signal"` and count how many
   networks share your channel. That contention is invisible and is usually the
   real explanation for a slow link.
4. Compare the rate your client negotiated against the headline number for its
   Wi-Fi generation. The gap is the difference between an aggregate PHY rate and
   one device's share of it.
5. Check `ip -s link` on the wireless interface for error and drop counters. A
   link can associate successfully and still be marginal.

This shows what your equipment negotiated. It does not show why — signal
quality and interference need dedicated tools to measure rather than infer.

## Related pages

- [Transmission media](/learn/transmission-media) — the overview, and how radio
  compares with copper and glass.
- [Wired media](/learn/wired-media) — copper and fiber, and the collision
  handling that a cable makes possible.
- [The OSI model](/learn/osi-model) — where physical signalling and framing sit
  relative to each other.
- [Network appliances](/learn/network-appliances) — the access points
  terminating these links.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [IEEE 802.11 Wireless LAN Working Group](https://www.ieee802.org/11/) — the
  wireless LAN standards and their amendments.
- [GPS.gov](https://www.gps.gov/) — official reference for GPS system operation
  and positioning.
- [FCC Part 15 rules](https://www.ecfr.gov/current/title-47/chapter-I/subchapter-A/part-15)
  — the unlicensed-band power limits that bound real-world range in the US.

Regulatory domains differ by country, so channel availability and permitted
transmit power vary with where the equipment is operating, not just what it
supports.
