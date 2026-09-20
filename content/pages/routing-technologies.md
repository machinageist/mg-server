---
title: "Routing technologies and route selection"
date: 2026-09-14
summary: "How a router chooses between candidate routes, what static and dynamic routing each cost, and what NAT, first-hop redundancy, and subinterfaces do."
tags: [education, networking, routing, bgp, ospf, eigrp, nat]
---

## Overview

A router moves packets between networks. It reads a packet's destination address,
finds the best matching entry in its routing table, and hands the packet to the next
hop named there. Each router that does this is one hop, and the frame around the
packet is rewritten at every one of them while the IP addresses stay the same.

The routing table is the whole story, and it is worth being precise about what sits
in it. Each entry pairs a destination prefix with a way to reach it: a next-hop
address, an outgoing interface, or both. Where those entries come from — typed in by
an administrator, or learned from a protocol — is the first thing below. How a router
picks between two entries that both match is the part most worth understanding,
because that is where troubleshooting usually ends up.

## Static and dynamic routing

A static route is one an administrator enters. It does not change on its own, spends
no bandwidth advertising itself, and does exactly what it says. That predictability
is why static routes remain the normal choice for a default route out of a small
network, for a stub network with only one way in, and for a deliberate backup path.

The cost is that the route goes on claiming to be valid after it stops being true. If
the next hop disappears, the route stays in the table and traffic keeps going into
the hole until something else removes it. A small, stable network absorbs that. A
network with enough links to have alternatives does not, and maintaining static
routes by hand across one is where the approach stops scaling.

A route is configured with a command of this shape on Cisco IOS:

```text
Router(config)# ip route <destination-network> <subnet-mask> <next-hop-ip>
```

A dynamic routing protocol has routers tell each other what they can reach. Routes
are discovered, maintained while they stay valid, and withdrawn when they do not, so
the table follows the topology instead of describing what it looked like on the day
it was configured. The price is bandwidth, CPU, and one more protocol to understand
and secure.

One thing worth separating out: DHCP is not dynamic routing. DHCP hands an address
and a default gateway to a host. Dynamic routing is routers exchanging reachability
with each other. The word "dynamic" turns up in both, and the two mechanisms have
nothing to do with one another.

## Routing protocol families

Routing protocols divide first by scope. An interior gateway protocol (IGP) runs
inside one administrative domain and optimizes for a best path. An exterior gateway
protocol runs between domains and optimizes for policy — whose traffic you are
willing to carry, and through whom.

An autonomous system (AS) is that administrative domain: a network or group of
networks run under one routing policy, identified by an autonomous system number
(ASN). ASNs are allocated by IANA through the regional internet registries.
Autonomous systems are the units the public internet's routing is expressed in.

### BGP

Border Gateway Protocol (BGP) exchanges reachability between autonomous systems, and
is what makes inter-domain routing on the public internet work. It carries prefixes
along with path attributes, and decides between them by policy rather than by
measuring anything.

That last point is where BGP is most often misread. It has no metric for speed,
latency, or load. Its best-known attribute is the AS path — the list of autonomous
systems a route has crossed — and a shorter AS path beats a longer one by default,
but "by default" is doing real work in that sentence. Local preference, which an
operator sets, is consulted ahead of AS path. A network can and routinely does prefer
a longer, slower path because of a contract.

BGP also runs inside an AS, between that AS's own border routers, where it is called
internal BGP. That is a different job from an IGP, and the two run together rather
than in competition: the IGP works out how to reach a next hop, while BGP works out
which prefixes are out there at all.

### EIGRP

Enhanced Interior Gateway Routing Protocol (EIGRP) is an IGP, originally Cisco
proprietary and later published. It computes a composite metric from bandwidth and
delay, with load and reliability available but off by default — turning those on is
generally discouraged, because a metric that moves with traffic can set the routing
itself oscillating.

Its distinguishing piece is the Diffusing Update Algorithm (DUAL). Each router keeps
not only the route it is using but a precomputed alternative that is provably
loop-free, called a feasible successor. If the route in use fails and such an
alternative exists, it is installed immediately with no recomputation at all. Where
none exists, DUAL queries its neighbors and converges the slow way.

### OSPF

Open Shortest Path First (OSPF) is a link-state IGP and an open standard. Each router
advertises the state of its own links; every router in an area assembles the same
topology database out of those advertisements and runs a shortest-path calculation
over it. Routers inside an area therefore share a view of the topology, rather than
trading summaries of each other's conclusions.

Its metric is cost, conventionally derived from interface bandwidth, and it is summed
along the path. A four-hop path over fast links can beat a two-hop path over a slow
one. Despite the name, OSPF is not counting hops.

Areas exist because both the database and the calculation grow with the topology.
Dividing a large OSPF domain into areas joined through a backbone area bounds how far
a single link change has to propagate.

## How a router selects a route

When more than one entry matches a destination, the decision runs in a fixed order,
and each step only matters if the one before it tied:

1. Longest prefix match. The most specific route wins, full stop.
2. Lowest administrative distance, between routes of equal prefix length learned from
   different sources.
3. Lowest metric, between routes from the same source.

Getting that order right explains most surprising forwarding. A `/25` learned by RIP
beats a `/24` learned by OSPF, even though OSPF is the more trusted protocol and the
better one, because prefix length is settled first and nothing later can overturn it.

### Prefix length

The prefix length is how many leading bits of the address identify the network. A
longer prefix describes a smaller, more specific range. For a destination inside
`203.0.113.0/25`, that `/25` wins over a `203.0.113.0/24` covering the same address,
because it matches more bits.

The default route, `0.0.0.0/0`, is the shortest prefix there is: it matches every
destination on zero bits, so anything more specific beats it. That is precisely what
makes it work as a last resort.

See [subnetting, CIDR, and VLSM](/learn/subnetting) for how prefix length is
calculated and what it does to the address range.

### Administrative distance

Administrative distance (AD) ranks sources of routing information against each other.
It is local to one router, configurable, and never advertised to anyone. Typical
Cisco defaults:

| Source                       | Administrative distance |
|------------------------------|-------------------------|
| Directly connected interface | 0                       |
| Static route                 | 1                       |
| External BGP                 | 20                      |
| Internal EIGRP               | 90                      |
| OSPF                         | 110                     |
| RIP                          | 120                     |
| Internal BGP                 | 200                     |

It is a statement about trust, not about the path. A static route has an AD of 1
because an administrator asserted it, not because it is fast — a static route pointed
at a congested link still beats an OSPF route over a clear one. Values also vary by
platform, so read them off the device being operated rather than from a table.

The ranking is useful precisely because it can be adjusted. Giving a backup static
route an AD above the routing protocol's — a floating static route — keeps it out of
the table until the protocol's route disappears, at which point it takes over.

### Metric

Metric compares routes learned from the same protocol, and every protocol measures
something different: OSPF a cost derived from bandwidth, EIGRP a composite of
bandwidth and delay, RIP a hop count, and BGP nothing at all, since its decision is a
sequence of attribute comparisons rather than a number.

Metrics are therefore not comparable across protocols. An OSPF cost of 10 and an
EIGRP metric of 10 have no relationship to each other, which is the reason
administrative distance has to settle that comparison first.

## Address translation

Network address translation (NAT) rewrites addresses as packets cross between private
and public address space. A one-to-one mapping, one private address to one public
address, is NAT in the narrow sense.

Port address translation (PAT) is the one almost everyone actually runs. Many
internal hosts share a single public address, and the router keeps a table keyed on
the transport port it assigned each outbound session, so replies can be matched back
to the host that started them. Every home router does this, which is why a household
with a dozen devices needs one public address.

Two corrections worth carrying away. NAT is not a firewall: a translation table drops
unsolicited inbound traffic because it has nowhere to send it, not because a policy
decided to. That is a side effect rather than filtering, and it disappears the moment
a port forward is configured. And NAT is not the reason private addressing exists —
it is what makes private addressing usable against the public internet, which is a
different claim.

NAT also breaks the end-to-end addressing model, which is why protocols that carry
addresses inside their own payloads need helpers to work through it, and part of why
IPv6 was designed without needing it. See
[IPv6 addressing](/learn/ipv6-addressing).

## First-hop redundancy and virtual IPs

A host's default gateway is a single configured address, so losing that router takes
out everything behind it even when a second router sits alongside with an equally
good path. First-hop redundancy protocols (FHRPs) fix that without touching the
hosts.

Two or more routers share a virtual IP address (VIP), and a virtual MAC address with
it. Hosts are configured with the VIP as their gateway. One router is active and
answers for it while the others listen for its periodic hello. When the hellos stop,
another router takes over the VIP and the virtual MAC, and the hosts notice nothing —
as far as they are concerned, the gateway never changed.

Hot Standby Router Protocol (HSRP) and Gateway Load Balancing Protocol (GLBP) are
Cisco protocols; Virtual Router Redundancy Protocol (VRRP) is the open standard, and
what Linux implementations such as keepalived speak.

The same idea appears one layer up, where a VIP fronts several servers running the
same service. What differs is what makes the decision: an FHRP is routers electing
among themselves, while a service VIP is normally held by a load balancer that
health-checks its backends. Both give clients one address to hold, and in both the
failover is only as good as the check behind it. A gateway that is up but no longer
forwarding still sends hellos.

## Subinterfaces and VLANs

A physical interface can be divided into subinterfaces, each with its own IP address
and VLAN tag, written as a suffix on the physical name: `GigabitEthernet0/1.10` and
`GigabitEthernet0/1.20` on `GigabitEthernet0/1`. The router receives tagged frames
over a trunk, associates each tag with a subinterface, and routes between them.

This is router-on-a-stick, and it routes between VLANs across a single physical link.
It is the cheap way to do it, and that link is its limit: all inter-VLAN traffic
crosses it twice, once inbound and once back out. A Layer 3 switch with switch
virtual interfaces does the same job internally without that constraint. See
[switching technologies](/learn/switching-technologies) for the trunking and tagging
that feed these subinterfaces.

## Suggested practice: explain your own routing table

On a machine you own:

1. Print the table with `ip route` on Linux, or `netstat -rn` elsewhere. Identify the
   default route, the directly connected networks, and anything else present.
2. Pick one reachable destination and decide which entry will be used, before
   checking.
3. Check with `ip route get <destination>`, which reports the entry the kernel
   actually selects, its next hop, and the outgoing interface.
4. Find a destination covered by two entries — a host inside a connected network is
   covered both by that network and by the default route — and confirm that the more
   specific entry wins.
5. Add a more specific route through a different next hop with
   `sudo ip route add <a prefix> via <a gateway on your LAN>`, re-run
   `ip route get`, and watch the selection change. Remove it again with
   `sudo ip route del <a prefix>`.
6. Run `traceroute` or `tracepath` to a destination beyond your own network, and
   match its first hop against what the table said.

A Linux host has no administrative distance — that is a router feature, and Linux
carries a metric on the route instead — so steps 4 and 5 demonstrate longest-prefix
match rather than the full decision. The exercise is finished when you can predict
what `ip route get` will say before you run it.

## Related pages

- [Switching technologies](/learn/switching-technologies) — VLANs and tagging, and
  what a trunk delivers to a router's subinterfaces.
- [Network appliances](/learn/network-appliances) — the router as one device among
  the others, and the fields a route entry holds.
- [Subnetting, CIDR, and VLSM](/learn/subnetting) — how prefix length is calculated,
  which is the first thing route selection tests.
- [IPv4 addressing](/learn/ipv4-addressing) — the private ranges NAT translates out
  of.
- [IPv6 addressing](/learn/ipv6-addressing) — addressing designed without needing
  NAT.
- [Network functions](/learn/network-functions) — packet lifetime, which is what
  stops a routing loop lasting forever.
- [Software-defined networking](/learn/software-defined-networking) — moving the
  route computation off the individual router.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [RFC 4271: A Border Gateway Protocol 4 (BGP-4)](https://www.rfc-editor.org/rfc/rfc4271.txt)
  — the path attributes and the decision process BGP uses in place of a metric.
- [RFC 2328: OSPF Version 2](https://www.rfc-editor.org/rfc/rfc2328.txt) — link-state
  advertisement, areas, and the cost metric.
- [RFC 7868: Cisco's Enhanced Interior Gateway Routing Protocol](https://www.rfc-editor.org/rfc/rfc7868.txt)
  — the composite metric and DUAL's feasible-successor condition.
- [RFC 5798: VRRP Version 3 for IPv4 and IPv6](https://www.rfc-editor.org/rfc/rfc5798.txt)
  — virtual router election, the virtual address, and failover behavior.
- [RFC 3022: Traditional IP Network Address Translator](https://www.rfc-editor.org/rfc/rfc3022.txt)
  — basic NAT and the port-multiplexed form.
- [RFC 1812: Requirements for IPv4 Routers](https://www.rfc-editor.org/rfc/rfc1812.txt)
  — forwarding behavior and the longest-match rule.

Administrative distance is a vendor convention rather than a standard. The values
above are Cisco defaults and should be confirmed on the platform being operated.
