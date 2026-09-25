---
title: "Quality of service: which traffic goes first"
date: 2026-09-25
summary: "How network devices sort, mark, queue, and limit competing traffic, and why a marking only matters if the next device honors it."
tags: [education, networking, qos, dscp]
---

## Overview

Quality of service (QoS) is a set of techniques for deciding how network devices treat
competing traffic. It can support performance targets and service-level agreements
through:

- classification and marking;
- traffic priority and scheduling;
- bandwidth allocation;
- shaping and policing; and
- congestion avoidance.

Routers and switches apply QoS policy, but end-to-end results depend on consistent
handling across the path. A packet marking cannot force the next network to honor it,
and QoS cannot create bandwidth that is not there.

## The mechanisms

QoS only matters when a link is congested. On an idle link every packet goes
straight out, and there is nothing to decide. When packets arrive faster than a
link can send them, they wait in a queue, and QoS decides the order.

- Classification and marking sorts packets into classes, by address, port, or
  application, and writes the class into the packet so later devices do not have
  to sort it again. In IP the mark is the DSCP field in the header. On an 802.1Q
  trunk it can also be the three priority bits in the VLAN tag, called CoS.
- Priority and scheduling gives each class its own queue and decides which queue
  sends next. A strict priority queue always goes first, which suits voice, but it
  can starve the other queues if nothing limits it.
- Bandwidth allocation guarantees each class a share of the link when it is busy.
- Shaping and policing both hold traffic to a rate. Policing drops or re-marks
  whatever goes over the rate. Shaping holds the excess in a buffer and sends it
  later, which trades loss for delay.
- Congestion avoidance drops a few packets early as a queue fills, so TCP senders
  slow down before the queue overflows and drops everything at once.

## Suggested practice: see a priority marking on the wire

On a Linux machine you own:

1. Run `tc qdisc show dev <interface>` to see the queuing discipline your system
   already uses on that interface. On many distributions it is `fq_codel`, which is
   a congestion management mechanism in its own right.
2. In one terminal, start a capture with `sudo tcpdump -v -n -i <interface> icmp`.
3. In another, run `ping -c 3 -Q 0xb8 <a host on your LAN>`. The `-Q` option sets
   the type of service byte, and `0xb8` is DSCP 46, expedited forwarding, the value
   usually used for voice.
4. Read `tos 0xb8` in the IP header of each echo request in the capture. Then
   compare the `tos` value on the replies. Whether a host copies the marking is up
   to that host.

This shows a marking being set and carried. It does not show any device acting on
it. That needs a congested link and a policy that reads the marking.

## Exam key points

Where this page's material shows up in the exam objectives, and what each exam
expects beyond it.

### CCNA 200-301

Objective 4.7 in v1.1 explains QoS per-hop behavior: classification, marking,
queuing, congestion, policing, and shaping. v2.0, which replaces it in February
2027, drops QoS.

- Know the marking fields: DSCP in the IP header, and CoS in the 802.1Q tag. Voice
  is usually marked DSCP EF, which is 46.
- Policing drops or re-marks traffic over the limit. Shaping buffers and delays
  it. Policing is common at the edge of a provider network, and shaping on the
  customer side of it.
- Trust boundaries: a switch port connected to an IP phone commonly trusts the
  phone's marking, and one connected to a PC does not.

### Network+ N10-009

Objective 1.2 lists QoS as a network function.

- QoS decides which traffic goes first when a link is congested. It does not
  create bandwidth.
- A marking cannot force the next network to honor it.

## Related pages

- [Switching technologies](/learn/switching-technologies) — voice VLANs, which give
  QoS policy somewhere to apply.
- [Network appliances](/learn/network-appliances) — the routers and switches where
  QoS policy is configured.
- [VPNs and IPsec](/learn/vpns-and-ipsec) — why a marking can be hidden inside an
  encrypted tunnel.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [RFC 2474: Differentiated Services](https://www.rfc-editor.org/rfc/rfc2474.txt) — the
  DS field and how DSCP values are carried in the IP header.
- [RFC 2475: An Architecture for Differentiated Services](https://www.rfc-editor.org/rfc/rfc2475.txt)
  — classification, marking, and per-hop behavior.
- [RFC 3246: An Expedited Forwarding PHB](https://www.rfc-editor.org/rfc/rfc3246.txt)
  — the EF behavior used for voice.
- [RFC 4594: Configuration Guidelines for DiffServ Service Classes](https://www.rfc-editor.org/rfc/rfc4594.txt)
  — which traffic classes usually get which markings.
- [tc(8)](https://man.archlinux.org/man/tc.8) and [ping(8)](https://man.archlinux.org/man/ping.8)
  — the Linux tools used in the practice section.
