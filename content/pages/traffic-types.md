---
title: "Network traffic types"
date: 2026-07-24
summary: "Unicast, multicast, anycast, and broadcast: the delivery patterns that describe who receives a transmission."
tags: [education, networking, addressing, multicast, broadcast, network-plus]
---

## Overview

Most network traffic is **unicast**: one sender sends to one receiver. Multicast,
anycast, and broadcast cover three other cases: sending to a group, routing to
one of several equivalent destinations, or reaching every host on a local
segment.

## Unicast

A unicast transmission has one sender and one destination address. Normal
client/server connections, such as a browser loading a page or an SSH session,
use unicast. It is the default traffic pattern; the other types are easiest to
understand by comparing them with it.

## Multicast

A multicast transmission goes to a **group**. Only hosts that have joined the
group receive it, unlike a broadcast sent to every host in a broadcast domain.
IPv4 reserves 224.0.0.0/4 for multicast groups. Hosts use the Internet Group
Management Protocol (IGMP) to join and leave those groups. Multicast-aware
switches and routers use the membership information to forward traffic only
toward active listeners.

Streaming and other real-time applications can use multicast to serve many
subscribers with one stream instead of sending a separate unicast copy to each
one.

## Anycast

With anycast, several servers in different network locations advertise the same
address. Normal routing sends traffic to the closest or preferred instance, not
to one permanently fixed destination. "Closest" is the routing protocol's
answer, which may not mean the shortest physical distance.

Anycast comes from how an address is advertised into routing; it is not a
special packet type. DNS root servers are a common example. Many physical
locations announce the same anycast address, and ordinary IP routing sends each
query to a nearby instance.

## Broadcast

A broadcast reaches every host in one **broadcast domain**, usually a subnet or
VLAN. It does not reach the entire routed network. Routers do not forward
broadcast traffic between subnets by default, which is why broadcast domains
and subnet boundaries are closely related.

The Address Resolution Protocol (ARP) is a familiar example. A host broadcasts
"who has this IP address?" on its local segment. The owner replies directly,
allowing the sender to map the IPv4 address to a MAC address.

## Suggested practice: find each pattern in real traffic

On a network you own or are authorized to inspect, use Wireshark or `tcpdump`:

1. Capture traffic while pinging a neighbor on the same subnet. Find the ARP
   broadcast request and its unicast reply.
2. Filter for multicast destinations in IPv4's 224.0.0.0/4 or IPv6's
   `ff00::/8`. mDNS is common on home networks and is usually easy to find.
3. Resolve a public DNS name. The reply comes from one server address even when
   many locations announce that same anycast address; one capture cannot show
   those other locations.
4. Compare the broadcast destination MAC (`ff:ff:ff:ff:ff:ff`) with the
   destination MACs in multicast and unicast frames.

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

Networks may filter or rate-limit broadcast and multicast traffic. The RFCs
describe the mechanisms, not a particular network's policy.
