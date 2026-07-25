---
title: "Network traffic types"
date: 2026-07-24
summary: "Unicast, multicast, anycast, and broadcast: the delivery patterns that describe who receives a transmission."
tags: [education, networking, addressing, multicast, broadcast, network-plus]
---

## Overview

Most everyday network traffic is **unicast**: one sender addressing one
receiver. Three other delivery patterns exist for cases where "one specific
destination" isn't the right model — a group of interested listeners, the
nearest of several equivalent destinations, or every host on a local segment.

## Unicast

A unicast transmission has exactly one sender and one destination address.
Ordinary client/server connections — a browser fetching a page, an SSH
session — are unicast. It is the default assumption in most networking
discussion, which is why the other three patterns are usually defined by
contrast to it.

## Multicast

A multicast transmission is addressed to a **group**, and only hosts that
have joined that group receive it — it is not sent to every host on the
network the way broadcast is. IPv4 reserves 224.0.0.0/4 for multicast
groups; hosts join and leave groups using the Internet Group Management
Protocol (IGMP), and multicast-aware switches and routers use that membership
information to forward traffic only where a listener actually exists.
Streaming and some real-time applications use multicast so one stream can
serve many subscribers without a separate unicast copy per viewer.

## Anycast

An anycast address is advertised from multiple, topologically distinct
servers, and a sender's traffic is routed to whichever advertising instance
is closest or preferred by normal routing decisions — not one fixed
destination. This depends on the routing protocol converging on a "nearest"
answer; it is a property of how the address is advertised into routing, not a
special packet-level mechanism. The DNS root server system is the standard
example: the same anycast address is announced from many physical locations,
and ordinary IP routing sends a given query to a nearby instance.

## Broadcast

A broadcast transmission is delivered to every host within one **broadcast
domain** — typically a single subnet or VLAN — not to "the whole network"
without qualification. Routers do not forward broadcast traffic between
subnets by default, which is one reason broadcast domains and subnet
boundaries are discussed together. The Address Resolution Protocol (ARP) is
the clearest everyday example: a host broadcasts "who has this IP address?"
to its local segment, and the owner replies directly, letting the sender map
an IPv4 address to a MAC address.

## Suggested practice: find each pattern in real traffic

On a network you own or are authorized to inspect, using Wireshark or
`tcpdump`:

1. Capture traffic while pinging a neighbor on the same subnet and identify
   the ARP broadcast request and its unicast reply.
2. Filter for multicast destination addresses (the 224.0.0.0/4 range, or an
   IPv6 `ff00::/8` address) — mDNS traffic is common on most home networks and
   is easy to find this way.
3. Resolve a public DNS name and note that the reply came from one specific
   server address, even if the same anycast address is announced from many
   locations you cannot see from a single vantage point.
4. Compare a broadcast frame's destination MAC (`ff:ff:ff:ff:ff:ff`) against a
   multicast frame's destination MAC and an ordinary unicast frame's
   destination MAC.

## Related pages

- [The OSI model](/learn/osi-model) — where MAC and IP addressing, including
  broadcast and multicast addresses, fit in the layered model.
- [Network appliances](/learn/network-appliances) — how switches and routers
  bound broadcast domains and handle group membership.
- [Network protocols and ports](/learn/network-protocols) — DNS and ARP as
  protocols that rely on these delivery patterns.

## Sources and further reading

This page was edited from my networking reading notes and checked against:

- [RFC 826: An Ethernet Address Resolution Protocol](https://www.rfc-editor.org/rfc/rfc826.txt)
  — ARP's broadcast request / unicast reply behavior.
- [RFC 1112: Host Extensions for IP Multicasting](https://www.rfc-editor.org/rfc/rfc1112.txt)
  — IGMP and the IPv4 multicast address range.
- [RFC 4786: Operation of Anycast Services](https://www.rfc-editor.org/rfc/rfc4786.txt)
  — anycast as a routing-driven delivery pattern, including its limitations.

Real deployments vary in how aggressively they filter or rate-limit broadcast
and multicast traffic; the RFCs describe the mechanism, not any one
network's policy.
