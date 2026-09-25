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

Everything a router does starts with its routing table, so it helps to be precise
about what is in it. Each entry pairs a destination prefix with a way to reach it: a
next-hop address, an outgoing interface, or both. The first sections below cover
where entries come from, whether typed in by an administrator or learned from a
protocol. After that comes how a router picks between two entries that both match,
which is where troubleshooting usually ends up.

## Static and dynamic routing

A static route is one an administrator enters. It does not change on its own, uses
no bandwidth advertising itself, and does exactly what it says. Because it is
predictable, static routes are still the normal choice for a default route out of a
small network, for a stub network with only one way in, and for a planned backup
path.

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

DHCP is not dynamic routing. DHCP hands an address
and a default gateway to a host. Dynamic routing is routers exchanging reachability
with each other. The word "dynamic" turns up in both, and the two mechanisms have
nothing to do with one another.

## Routing protocol families

Routing protocols divide first by scope. An interior gateway protocol (IGP) runs
inside one administrative domain and optimizes for a best path. An exterior gateway
protocol runs between domains and optimizes for policy: whose traffic you are
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

This is where BGP is most often misunderstood. It has no metric for speed, latency,
or load. Its best-known attribute is the AS path, the list of autonomous systems a
route has crossed. A shorter AS path beats a longer one by default, but only by
default. Local preference, which an operator sets, is checked before AS path.
Networks routinely prefer a longer, slower path because of a contract.

BGP also runs inside an AS, between that AS's own border routers, where it is called
internal BGP. That is a different job from an IGP, and the two run together rather
than in competition: the IGP works out how to reach a next hop, while BGP works out
which prefixes are out there at all.

### EIGRP

Enhanced Interior Gateway Routing Protocol (EIGRP) is an IGP, originally Cisco
proprietary and later published. It computes a composite metric from bandwidth and
delay, with load and reliability available but off by default. Turning those on is
generally discouraged, because a metric that changes with traffic can make the
routing itself flap back and forth.

Its distinguishing piece is the Diffusing Update Algorithm (DUAL). Each router keeps
not only the route it is using but a precomputed alternative that is provably
loop-free, called a feasible successor. If the route in use fails and such an
alternative exists, it is installed immediately with no recomputation at all. Where
none exists, DUAL queries its neighbors and converges the slow way.

### OSPF

Open Shortest Path First (OSPF) is a link-state IGP and an open standard. Each router
advertises the state of its own links. Every router in an area builds the same
topology database from those advertisements and runs a shortest-path calculation
over it. So routers inside an area share one view of the topology, instead of
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
because an administrator entered it, not because it is fast. A static route pointed
at a congested link still beats an OSPF route over a clear one. Values also vary by
platform, so read them from the device you are working on, not from a table.

The ranking is useful because it can be adjusted. A backup static route given an AD
higher than the routing protocol's is called a floating static route. It stays out
of the table until the protocol's route disappears, and then it takes over.

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

Port address translation (PAT) is the one almost everyone uses. Many
internal hosts share a single public address, and the router keeps a table keyed on
the transport port it assigned each outbound session, so replies can be matched back
to the host that started them. Every home router does this, which is why a household
with a dozen devices needs one public address.

Two corrections. NAT is not a firewall. A translation table drops unsolicited inbound
traffic because it has nowhere to send it, not because a policy decided to. That is a
side effect, not filtering, and it disappears as soon as a port forward is
configured. And NAT is not the reason private addressing exists. It is what makes
private addresses usable on the public internet.

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
another router takes over the VIP and the virtual MAC. The hosts notice nothing,
because to them the gateway never changed.

Hot Standby Router Protocol (HSRP) and Gateway Load Balancing Protocol (GLBP) are
Cisco protocols. Virtual Router Redundancy Protocol (VRRP) is the open standard, and
it is what Linux implementations such as keepalived use.

The same idea appears one layer up, where a VIP fronts several servers running the
same service. What differs is what makes the decision. In an FHRP, routers elect
among themselves. A service VIP is normally held by a load balancer that
health-checks its backends. Both give clients one address to use, and in both,
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
   picks, its next hop, and the outgoing interface.
4. Find a destination covered by two entries. A host inside a connected network is
   covered both by that network and by the default route. Confirm that the more
   specific entry wins.
5. Add a more specific route through a different next hop with
   `sudo ip route add <a prefix> via <a gateway on your LAN>`, re-run
   `ip route get`, and watch the selection change. Remove it again with
   `sudo ip route del <a prefix>`.
6. Run `traceroute` or `tracepath` to a destination beyond your own network, and
   match its first hop against what the table said.

A Linux host has no administrative distance. That is a router feature, and Linux
carries a metric on the route instead. So steps 4 and 5 show longest-prefix match,
not the full decision. The exercise is finished when you can predict
what `ip route get` will say before you run it.

## Exam key points

Where this page's material shows up in the exam objectives, and what each exam
expects beyond it.

### CCNA 200-301

Objectives 3.1 to 3.5 in v1.1 cover the routing table, the forwarding decision,
static routes, single-area OSPFv2, and FHRPs, and 4.1 covers NAT. In v2.0, which
replaces it in February 2027, OSPFv3 joins OSPFv2 in 3.3, 3.4 asks you to read
HSRP and VRRP status, and NAT and PAT move to 4.3.

- Routing table codes: C connected, L local, S static, O OSPF, D EIGRP, B BGP, and
  an asterisk for a candidate default. The gateway of last resort is the default
  route.
- Default administrative distances: connected 0, static 1, external BGP 20, EIGRP
  90, OSPF 110, RIP 120, and internal BGP 200.
- Static routes: `ip route 0.0.0.0 0.0.0.0 <next-hop>` for a default, a /32 for a
  host route, and a distance at the end for a floating static, such as
  `ip route 10.0.0.0 255.0.0.0 192.0.2.2 150`. IPv6 uses `ipv6 route`.
- OSPF: `router ospf 1`, then `network <address> <wildcard> area 0` or
  `ip ospf 1 area 0` on the interface. The router ID comes from `router-id`, then
  the highest loopback address, then the highest active interface address.
- OSPF neighbors must match area, subnet, hello and dead timers, and MTU for a
  full adjacency. On broadcast networks the DR is the highest priority, then the
  highest router ID, and priority 0 never becomes DR. Point-to-point links have no
  DR. Check with `show ip ospf neighbor`.
- HSRP is Cisco's FHRP with active and standby routers, VRRP is the open standard
  with master and backup, and GLBP adds load balancing. Check with
  `show standby brief` or `show vrrp brief`.
- NAT: mark interfaces with `ip nat inside` and `ip nat outside`, then use
  `ip nat inside source static` or a pool with an ACL. `overload` turns it into
  PAT. Know inside local, inside global, outside local, and outside global, and
  check with `show ip nat translations`.
- EIGRP and BGP are not configured on the CCNA. Know their route codes and
  distances.

### Network+ N10-009

Objective 2.1 lists static and dynamic routing (BGP, EIGRP, and OSPF), route
selection by administrative distance, prefix length, and metric, NAT and PAT,
FHRP, VIPs, and subinterfaces. Objective 5.3 adds routing table and default route
problems.

- BGP is the path-vector protocol between autonomous systems. EIGRP is an advanced
  distance-vector protocol from Cisco. OSPF is link-state.
- Route selection order: longest prefix first, then administrative distance, then
  metric.
- PAT lets many hosts share one public address by tracking ports.

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
