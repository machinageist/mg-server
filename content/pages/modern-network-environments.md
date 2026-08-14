---
title: "Modern network environments"
date: 2026-08-14
summary: "Software-defined networking, SD-WAN, VXLAN overlays, zero-trust architecture, SASE, and infrastructure as code — the shift from configuring devices to declaring policy."
tags: [education, networking, sdn, zero-trust, automation, ccna]
---

## Overview

A traditional network is configured device by device. Someone logs into a
switch, sets a VLAN, logs into the next one, and repeats. That works at the
scale it was designed for and stops working somewhere past it — not because any
single step is hard, but because the number of places a mistake can hide grows
with every device.

The designs on this page are responses to that problem. They differ in scope,
but they share a direction: move decisions out of individual boxes and into a
central place that can be reasoned about, versioned, and audited. The device
keeps forwarding packets; it stops being where policy lives.

Each of these also requires a policy framework to go with it. Centralizing
control means centralizing the consequences of getting control wrong.

## Software-defined networking

**Software-defined networking (SDN)** separates two jobs that traditional
network hardware performs together:

- The **control plane** decides how traffic should flow — which path a packet
  takes, which routes exist, what is permitted.
- The **data plane**, also called the **forwarding plane**, moves packets along
  those paths.

In a conventional switch or router both planes live in the same chassis. Each
device makes its own decisions from its own view of the network. SDN pulls the
control plane out into a central controller, leaving the hardware to forward.
One device with a complete picture can make choices no individual switch could.

(A third **management plane** — the configuration and monitoring interface —
is usually described alongside these. SDN discussions focus on the control/data
split because that is the one SDN changes.)

The architecture is usually drawn as three layers, with the controller in the
middle:

| Layer | Contains | Talks to the controller via |
|---|---|---|
| Application layer | Business logic and policy the operator defines | Northbound API |
| Control layer | The SDN controller | — |
| Infrastructure layer | Physical switches, routers, virtual appliances | Southbound API |

The **northbound API** is how intent enters the system: an application or an
operator states what should be true. The **southbound API** is how the
controller pushes the resulting configuration down to hardware. OpenFlow is the
best-known southbound protocol, though vendor implementations vary widely.

The business logic at the top is ordinary operational policy, expressed once
instead of per-device:

- prioritize live video and audio over bulk transfer;
- scale bandwidth for an e-commerce tier during a sale; or
- enforce a security policy across every edge simultaneously.

## SD-WAN

A **wide area network (WAN)** is geographically dispersed by definition, which
makes every hands-on maintenance task expensive. **SD-WAN** applies the SDN
model to that problem: centralize the control plane, and let branch sites take
policy from it rather than from a local engineer.

SD-WAN optimizes traffic across whatever links a site already has —
**multiprotocol label switching (MPLS)**, broadband, LTE or 5G — and chooses
between them per application. The practical consequence is that traffic no
longer has to transit a central hub just to be inspected before reaching its
destination. A branch office reaching a cloud service can go directly, under
policy, instead of hairpinning through headquarters.

### What these designs buy you

**Application-aware routing.** Because the controller sees traffic in terms of
applications rather than ports and addresses alone, it can prioritize at a
granularity a standalone device cannot. Voice takes the low-latency link; a
backup takes the cheap one.

**Zero-touch provisioning (ZTP).** A new device powers on, retrieves its
configuration from a known URL, and applies it without anyone typing. Across a
large deployment this removes both the labor and the most common source of
error — a human configuring the same thing fifty times and doing it differently
once.

**Transport independence.** The overlay does not care what carries it. With
real-time monitoring, an SD-WAN can shift traffic to 5G when a fiber circuit
degrades and shift back when it recovers, without a configuration change.

**Central policy management.** One control surface for the whole estate.
Misconfiguration is a leading cause of network outages and a recurring finding
in breach analyses; reducing the number of places a configuration can be wrong
is a security control as much as an operational one.

## VXLAN

VLAN IDs are 12 bits, which allows 4,096 values — and 4,094 usable ones, since 0
and 4095 are reserved. In a single building that is ample. In a service provider
or a multi-tenant data center it is a hard ceiling, and reaching it means
hardware reconfiguration rather than a policy change.

**Virtual Extensible LAN (VXLAN)** removes the ceiling by encapsulating Layer 2
frames inside UDP datagrams and carrying them over a Layer 3 network. The
identifier grows from 12 bits to 24, giving about 16.7 million **virtual network
identifiers (VNIs)**.

The more important consequence is architectural. Because the segment is now an
overlay riding on routed infrastructure, two hosts can share a Layer 2 domain
without sharing a physical one. The underlying network only has to route IP.

**VXLAN tunnel endpoints (VTEPs)** do the encapsulation and decapsulation. A
VTEP — a switch, a hypervisor, or software on a host — is the boundary between
the virtual segment and the physical network, and the point where the VNI is
applied or stripped.

### Data center interconnect

**Data center interconnect (DCI)** uses this to join physically separate
facilities into what workloads experience as one. A virtual machine can migrate
between sites without changing address, because the segment it lives on spans
both. That supports resource sharing, workload migration, and disaster recovery
where a standby site is genuinely interchangeable rather than a copy that needs
reconfiguring to take over.

## Zero-trust architecture

Perimeter security assumes that being inside the network implies authorization.
**Zero-trust architecture (ZTA)** discards that assumption. Identity is verified
continuously, and no location on the network grants trust by itself. The
practical goal is containment: a single compromised host should not be a path to
everything else.

NIST SP 800-207 describes the machinery in three parts:

- The **policy engine** decides whether to grant access, using identity, device
  posture, and whatever other signals it is given.
- The **policy administrator** carries that decision out, establishing or
  cutting the connection. Together with the policy engine it forms the **policy
  decision point**.
- The **policy enforcement point (PEP)** sits in the traffic path and does what
  the policy administrator instructs.

Behind a PEP is an **implicit trust zone** — the region where traffic is no
longer individually inspected because the PEP already vetted it. Zero-trust
design works by shrinking those zones, so each one covers as little as possible.
A **secured zone** is the same idea applied to sensitive systems: fewer things
inside, more scrutiny at the boundary.

### Policy-based authentication

Access privileges depend on circumstances, not just on who is asking.
**Policy-based authentication** encodes that:

- time restrictions, such as ordinary working hours;
- location restrictions, such as an office network or a known region;
- device restrictions, such as a managed laptop rather than a personal machine;
  and
- required authentication strength, such as mandatory multi-factor.

Expected behavior proceeds normally. Unexpected behavior triggers stronger
authentication or is refused. Continuous evaluation of these signals, rather
than a single check at login, is often called **adaptive identity**.

### Authorization

Confirming identity is not the same as granting access. Authorization decides
what a verified identity may reach, and under zero trust it is scoped to what
the person's work requires. It can be as dynamic as authentication: the same
user may be permitted a system during business hours from a managed device and
refused it at 3 a.m. from an unknown network.

### Least privilege

**Least privilege** is the principle underneath both: grant the minimum access
required for a task, and nothing further. It has a real cost — someone who needs
a tool from another department has to ask, and that takes time. The trade is
containment. An attacker who takes an account inherits only that account's
narrow access, and each additional step has to be earned rather than assumed.

## SASE and SSE

When applications live in the cloud and users work from anywhere, backhauling
all traffic to a corporate data center for inspection stops making sense. The
inspection point is in the wrong place.

**Secure access service edge (SASE)** moves it. SASE combines SD-WAN with
security functions delivered from cloud points of presence near the user,
typically including:

- **firewall as a service (FWaaS)**;
- **secure web gateways (SWGs)**, which filter outbound web traffic; and
- **cloud access security brokers (CASBs)**, which sit between users and cloud
  services to apply policy and provide visibility into their use.

**Security service edge (SSE)** is the security half of SASE without the
networking half. Organizations that already have a WAN they are satisfied with
often adopt SSE alone.

This shifts enforcement toward the point of connection rather than eliminating
the data center. On-premises infrastructure does not disappear; it stops being
the mandatory waypoint for traffic that was never headed there.

## Infrastructure as code

**Infrastructure as code (IaC)** describes infrastructure in files rather than
in a sequence of console actions. A template defines what should exist; a tool
makes reality match it. In a cloud environment such as a virtual private cloud,
this means a new environment is produced by running a definition rather than by
someone recreating it from memory.

The gain is not primarily speed. It is that the definition is reviewable,
diffable, and repeatable — three environments built from one template are
identical in a way three environments built by hand never are.

**Orchestration** is the layer above: coordinating multiple automated tasks into
a workflow, in the right order, with dependencies respected. Automation makes
one task repeatable; orchestration makes a sequence of them reliable.

## Source control

IaC only delivers on its promise if the definitions are themselves managed.
**Source control** tracks changes to files over time, and the features that
matter here are the ordinary ones:

- **version control**, so every change has a history and a previous state to
  return to;
- a **central repository**, so there is one authoritative copy rather than
  several divergent ones;
- **conflict identification**, so two simultaneous changes to the same thing
  surface rather than silently overwriting; and
- **branching and merging**, so work in progress stays separate until it is
  ready.

Applied to network configuration, this answers questions that are otherwise
unanswerable: what changed, when, by whom, and what did it look like before.
That last one is what turns an outage into a rollback.

## Suggested practice: build an overlay you can inspect

The concepts on this page are mostly enterprise-scale, but the mechanisms are
observable on one machine with free software.

1. Create a VXLAN interface on a Linux host and give it an address:
   `sudo ip link add vxlan0 type vxlan id 42 dev eth0 dstport 4789`, then
   `sudo ip addr add 10.42.0.1/24 dev vxlan0` and bring it up. Confirm the VNI
   with `ip -d link show vxlan0`.
2. Repeat on a second host or VM with a different address on the same subnet,
   add each as the other's remote, and ping across. Capture the traffic with
   `sudo tcpdump -i eth0 udp port 4789` and find the inner frame inside the
   outer UDP datagram. That is encapsulation you can point at.
3. Put a network configuration under version control. Commit a working state,
   change something, and read the diff. Break it deliberately and recover with
   `git revert`.
4. Write the same change as a declarative definition — a `netplan` YAML file, an
   Ansible playbook, a `systemd-networkd` unit — and apply it twice. Confirm the
   second run changes nothing. Idempotence is the property that makes IaC safe
   to re-run.
5. Read your own firewall or router configuration and identify where the
   decision is made versus where it is enforced. On a single home router these
   are the same box. Naming the two roles is the concept.

## Related pages

- [Cloud computing concepts](/learn/cloud-computing) — NFV, virtual private
  clouds, and the virtualization these designs assume.
- [Network appliances](/learn/network-appliances) — the physical devices whose
  control planes are being centralized.
- [Network functions](/learn/network-functions) — tunneling and VPNs, the
  encapsulation pattern VXLAN generalizes.
- [IPv6 addressing](/learn/ipv6-addressing) — the addressing model that makes
  large overlay designs practical.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [RFC 7348: Virtual eXtensible Local Area Network (VXLAN)](https://www.rfc-editor.org/rfc/rfc7348.txt)
  — the encapsulation format, the 24-bit VNI, and VTEP behavior.
- [NIST SP 800-207: Zero Trust Architecture](https://csrc.nist.gov/pubs/sp/800/207/final)
  — the policy engine, policy administrator, and policy enforcement point model.
- [RFC 7426: Software-Defined Networking (SDN): Layers and Architecture Terminology](https://www.rfc-editor.org/rfc/rfc7426.txt)
  — the layered terminology and where the interfaces sit.
- [IEEE 802.1Q](https://standards.ieee.org/ieee/802.1Q/10323/) — the VLAN tag
  format that sets the 12-bit ceiling VXLAN works around.

One correction from my notes: the zero-trust decision component is the **policy
engine**, and it pairs with the policy administrator to form the policy decision
point. I had recorded it as a "policy brain," which is descriptive but is not
the term in the standard, and the distinction between deciding and enforcing is
the part that matters.

Vendor implementations of all of this vary considerably, and the marketing terms
move faster than the standards. The architectural split — decide centrally,
enforce at the edge — is the durable part.
