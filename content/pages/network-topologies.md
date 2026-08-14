---
title: "Network topologies"
date: 2026-08-02
summary: "How nodes and links are arranged — mesh, star, ring, spine-and-leaf, and the tiered designs used in enterprise and data center networks."
tags: [education, networking, topology, architecture, data-center]
---

## Overview

A **topology** is the arrangement of a network's **nodes** and the **links**
between them. It answers a structural question that sits underneath most
operational ones: when this device needs to reach that device, what paths exist,
and what happens when one of them fails?

Topology is worth separating from equipment. The same collection of switches can
be wired into arrangements with very different failure behavior, and the diagram
that describes how traffic flows is not always the diagram that describes how the
cables run.

## Physical and logical topology

**Physical topology** is the cabling and radio layout — what is plugged into
what. **Logical topology** is how data actually moves between nodes.

The two often differ. Classic 10BASE-T Ethernet wired every station to a central
hub, so its physical topology was a star. But a hub repeats every incoming
signal to every port, so all stations shared one collision domain and the logical
topology was a bus. Replacing that hub with a switch changes nothing physically
and changes the logical topology completely.

Keep the distinction in mind when reading a network diagram. A drawing that shows
a tidy hierarchy may describe the intended traffic flow, the cable plant, or the
routing design, and those are three different documents.

## Mesh

In a **mesh** topology, each node connects to multiple other nodes.

A **full mesh** connects every node to every other node. For `n` nodes that
requires `(n * (n - 1)) / 2` links — a count of links, not of nodes. The
quadratic growth is the reason full mesh stays rare: 5 nodes need 10 links, 20
nodes need 190, and 50 nodes need 1,225.

A **partial mesh** connects some nodes to several others without connecting
everything to everything. It keeps most of the redundancy at a fraction of the
link count.

Mesh buys fault tolerance. Losing a link leaves other paths available, and no
single node's failure partitions the network. The cost is cabling, interface
count, and the configuration and monitoring burden of every one of those links.
Full and partial mesh appear in wide area network (WAN) backbones and in wireless
mesh deployments, where nodes relay for each other.

## Star, or hub and spoke

A **star** — also called **hub and spoke** — attaches every node to one central
point, and traffic between two nodes passes through that center.

This is the shape of most ordinary local networks. It is easy to reason about,
easy to extend by adding one more link to the center, and a convenient place to
apply configuration or policy for the whole segment at once.

The tradeoff is direct: the central device is a single point of failure. Losing
it isolates every node attached to it. In practice that is managed rather than
eliminated — redundant power supplies, stacked or paired switches, and a second
uplink path all reduce the exposure without changing the basic shape.

Modern star networks put a switch at the center rather than a hub, which is what
moves the logical topology away from a shared bus.

## Ring and token passing

A **ring** connects each node to two neighbors so that traffic circulates around
the loop. **Token Ring**, standardized as IEEE 802.5, controlled access by
passing a token: the node holding the token is the only one permitted to
**transmit**. Receiving is not gated by the token — every node still reads frames
addressed to it as they pass.

Token passing is **deterministic**. Unlike contention-based access, where a
station transmits when it believes the medium is free, the token guarantees a
bounded wait before any node gets its turn and removes collisions by
construction. That property matters when timing must be predictable, which is why
token-passing schemes persist in industrial and manufacturing protocols.

Token Ring itself lost to Ethernet and is obsolete as a LAN technology. The idea
outlived the product.

## Hybrid

A **hybrid** topology combines two or more topology types, typically joined
through a switch or bridge. Most real networks of any size are hybrids: a
spine-and-leaf fabric in the data center, stars out to the access closets, and a
point-to-point or partial-mesh WAN connecting sites.

## Spine and leaf

**Spine-and-leaf** is a two-tier switching fabric built for data centers. Every
**leaf** switch connects to every **spine** switch. Leaves never connect to other
leaves, and spines never connect to other spines. End devices attach to leaf
switches.

The tier count is the point of the design, and it is easy to miscount: servers
attach to leaves, but they are not a switching tier. The fabric is two tiers, in
deliberate contrast to the three-tier model below.

The arrangement gives every leaf-to-leaf path the same length — leaf, spine, leaf
— so latency between any two endpoints is consistent rather than depending on
where they happen to sit. Multiple equal-cost paths spread load across the spines
and let the fabric survive losing one. Capacity grows by adding spines.

Note what this does *not* do: switched full-duplex links have no collisions, so
the redundancy is about congestion, throughput, and failure tolerance rather than
contention. The design became common because virtualization and distributed
applications generate far more traffic between servers than in and out of the
data center.

## Point to point

A **point-to-point** link connects exactly two nodes. A leased line to a cloud
provider, a fiber run between two buildings on a campus, or a link joining two
campuses are all point-to-point.

With no shared medium and no intermediate nodes, the path is simple to secure and
simple to reason about, which suits high-priority or sensitive traffic. Small
point-to-point links also connect IoT and sensor devices where a full network
would be excessive.

## The three-tier hierarchical model

The **three-tier hierarchical model** divides an enterprise network into layers
by role:

- The **core** is the backbone. It moves high volumes of traffic between parts of
  the network as fast as possible and avoids doing anything that would slow that
  down.
- The **distribution layer** sits between core and access. It aggregates access
  connections and is where routing between segments, network policy, and security
  policy are usually enforced.
- The **access layer** is where end devices attach: workstations, printers, VoIP
  phones, wireless access points.

Separating the layers by role means each can be sized, secured, and upgraded on
its own terms, which is what makes the design scale in large enterprises.
Spanning Tree Protocol (STP) is commonly used within it to keep redundant Layer 2
links from forming loops that would otherwise flood the network with circulating
broadcast traffic.

## Collapsed core

A **collapsed core** merges the core and distribution layers into one tier.
Smaller networks often do not generate enough traffic to justify a separate core,
and combining the layers reduces equipment, cost, and configuration. The
concentration is the tradeoff: the merged tier carries both roles, so its failure
takes more of the network with it.

## Traffic flow: north-south and east-west

**Traffic flow** describes the paths data takes through a network, and two
directions are named because they raise different concerns:

- **North-south traffic** enters or leaves the network — a user reaching the
  internet, or a request arriving from outside. It crosses a trust boundary, so
  the primary concern is usually security policy and inspection.
- **East-west traffic** moves within the network, between servers or between
  devices in the same cluster or environment. The primary concern is usually
  efficiency: throughput, latency, and available capacity between internal nodes.

The growth of east-west traffic is what drove data center design from the
three-tier model toward spine-and-leaf. A hierarchy optimized for traffic heading
out to the internet handles server-to-server traffic poorly, because two servers
on different access switches may have to travel up to the distribution or core
layer and back down.

## Study-note shortcuts worth correcting

- **`(n * (n - 1)) / 2` counts links, not nodes.** It answers how much cabling a
  full mesh needs for a given number of nodes.
- **The token holder may transmit; receiving is not gated.** Nodes read frames
  addressed to them whenever those frames pass.
- **Spine-and-leaf is two tiers, not three.** Attached servers are endpoints, not
  a switching layer.
- **Switched fabrics do not have collisions to reduce.** Redundant paths address
  congestion and failure, not contention. Collision domains belong to shared
  half-duplex media.
- **A star's physical shape says nothing about its logical topology.** Hub-based
  and switch-based stars look identical on a cable diagram and behave nothing
  alike.

## Suggested practice: map both topologies of a network you own

Physical and logical topology have to be observed separately:

1. Draw the physical layout first: every device, every cable or radio link, and
   what each one connects to. Include anything that only carries power, such as a
   PoE run to an access point.
2. Draw the logical layout second, from the client's point of view. Use
   `ip neigh` to see which devices share a link, and `ip route` to see where
   traffic leaves for other networks.
3. Run `traceroute` or `tracepath` to a destination outside the network and mark
   which hops are yours and which belong to a provider.
4. Compare the two drawings. Note every place a single device carries several
   roles, and every place one cable failure would partition the network.
5. Identify one north-south and one east-west path, and note which device would
   apply policy to each.

Provider infrastructure will remain opaque past your own edge. Mark it as unknown
rather than drawing a topology you cannot verify.

## Related pages

- [The OSI model](/learn/osi-model) — the layered model that separates physical
  arrangement from logical forwarding.
- [Network appliances](/learn/network-appliances) — the switches, routers, and
  wireless systems these topologies are built from.
- [Transmission media](/learn/transmission-media) — the copper, fiber, and radio
  forming the links between nodes.
- [Network traffic types](/learn/traffic-types) — unicast, multicast, and
  broadcast delivery across these arrangements.

## Sources and further reading

This page was edited from my networking reading notes and checked against:

- [IEEE 802.3 Ethernet Working Group](https://www.ieee802.org/3/) — the Ethernet
  standards behind physical-star, logical-bus behavior and switched full-duplex
  links.
- [RFC 7938: Use of BGP for Routing in Large-Scale Data Centers](https://www.rfc-editor.org/rfc/rfc7938.txt)
  — Clos and spine-and-leaf fabrics, equal-cost paths, and why data centers moved
  to them.

The three-tier hierarchical and collapsed-core models come from vendor design
guidance rather than an open standard, and IEEE 802.5 Token Ring is a retired
specification. Vendor design guides and platform documentation are the reference
for a specific network.
