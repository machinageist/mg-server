---
title: "Routing technologies and route selection"
date: 2026-09-14
summary: "Static and dynamic routing, BGP, EIGRP, OSPF, route selection, address translation, first-hop redundancy, and router subinterfaces."
tags: [education, networking, routing, bgp, ospf, eigrp, nat]
---

## Overview

A router forwards packets between networks. It examines the destination IP address, consults its routing table, and sends the packet toward a next hop. Each router crossed is a hop. A route can be configured manually or learned from other routers.

The source notes for this page are study material, not a production design. The useful operational distinction is simple: static routing is explicit and quiet; dynamic routing exchanges information so the network can adapt.

## Static and dynamic routing

A **static route** is entered by an administrator. It is predictable and consumes no routing-protocol bandwidth, which makes it useful for small, stable networks and carefully chosen default or backup paths. It does not adapt when a link fails unless another mechanism changes it.

A **dynamic routing protocol** lets routers discover, maintain, and withdraw routes. The protocol exchanges information and calculates a preferred path when the topology changes. Common examples include **BGP**, **EIGRP**, and **OSPF**.

A route may be configured with a command such as:

```text
Router(config)# ip route <destination-network> <subnet-mask> <next-hop-ip>
```

## Routing protocol families

### BGP

**Border Gateway Protocol (BGP)** exchanges reachability between **autonomous systems (ASes)**. An AS is a network or group of networks operated under one administrative policy and identified by an **autonomous system number (ASN)**. BGP is the protocol that makes inter-domain routing on the public internet possible.

BGP chooses paths using policy and path attributes rather than a single “fastest link” measurement. The AS path, for example, records the autonomous systems a route has crossed and can be used in policy decisions.

### EIGRP

**Enhanced Interior Gateway Routing Protocol (EIGRP)** is an interior gateway protocol used within one organization. Its composite metric primarily considers bandwidth and delay, with optional load and reliability inputs. EIGRP uses the **DUAL** algorithm to converge while avoiding routing loops.

### OSPF

**Open Shortest Path First (OSPF)** is a link-state interior gateway protocol. Routers advertise the state of their links within an area, build a shared topology view, and calculate paths using a cost metric. OSPF cost is commonly derived from interface bandwidth, so a path with more hops can still win when its links have lower total cost.

## How a router selects a route

When several routes match a destination, selection proceeds from the most specific information to the protocol-specific metric:

1. Choose the longest matching prefix.
2. If the candidates came from different sources, prefer the lowest **administrative distance (AD)**.
3. If candidates are from the same protocol and AD is equal, prefer the lowest protocol metric.

Administrative distance is a local trust ranking, not a measure of path speed. Typical Cisco defaults include static routes at 1, external BGP at 20, internal EIGRP at 90, OSPF at 110, RIP at 120, and internal BGP at 200. Defaults vary by platform, so verify them on the device being operated.

A longer prefix is more specific: `203.0.113.0/25` wins over `203.0.113.0/24` for a destination inside the `/25`. The protocol metric only matters after prefix length and administrative distance leave a tie.

## Address translation

**Network address translation (NAT)** maps an address between private and public address spaces. A one-to-one mapping is commonly called NAT. **Port address translation (PAT)** lets many internal hosts share one public address by assigning distinct source ports to their sessions. Translation is not a firewall; filtering and stateful policy are separate controls.

## First-hop redundancy and virtual IPs

**First-hop redundancy protocols (FHRPs)** keep a default gateway available when one router fails. HSRP and GLBP are Cisco protocols; VRRP is an open standard. The routers present a shared **virtual IP (VIP)** to hosts. One router forwards traffic while another can take over after the active router stops sending its health signal.

A VIP can also front a service on multiple physical hosts. That simplifies client configuration, supports failover, and can participate in load distribution. The exact failover and health-check behavior depends on the implementation.

## Subinterfaces and VLANs

A router or switch can use **subinterfaces** to represent multiple logical interfaces on one physical interface. Each subinterface can receive its own IP address and VLAN tag, which is a common way to route between VLANs (often called “router-on-a-stick”). For example, `GigabitEthernet0/1.10` and `GigabitEthernet0/1.20` can represent two logical networks on `GigabitEthernet0/1`.

The design separates the physical link from the logical networks carried over it. It does not remove the need for consistent tagging, trunk configuration, addressing, and policy on both sides.

## A route-selection checklist

When troubleshooting a path, inspect the destination prefix, the installed route, the next hop, the interface, and the source protocol. Then ask whether a more-specific route, a lower administrative distance, or a lower protocol metric explains the result. Verify the actual forwarding table rather than assuming that the protocol with the “best” general reputation is the one in use.

## Related pages

- [Software-defined networking](/learn/software-defined-networking)
- [IPv4 addressing](/learn/ipv4-addressing)
- [Subnetting, CIDR, and VLSM](/learn/subnetting)
- [Network functions](/learn/network-functions)

## Suggested practice

On a network you own or are authorized to inspect, compare the installed route for one destination with the routing protocol's view. Record the destination prefix, next hop, interface, administrative distance, and metric. Then change one lab route or link, observe the withdrawal or replacement, and restore the original state. The exercise is complete when the forwarding table and the protocol explanation agree.

## Sources and further reading

- Ian Neil, *Network+ Guide to Networks* — study reference for routing, addressing, and protocol fundamentals.
- [RFC 4271 — A Border Gateway Protocol 4 (BGP-4)](https://www.rfc-editor.org/rfc/rfc4271)
- [RFC 2328 — OSPF Version 2](https://www.rfc-editor.org/rfc/rfc2328)
- [RFC 5798 — VRRP Version 3 for IPv4 and IPv6](https://www.rfc-editor.org/rfc/rfc5798)
- [Cisco — Administrative distance](https://www.cisco.com/c/en/us/support/docs/ip/routing-information-protocol-rip/8651-21.html)
