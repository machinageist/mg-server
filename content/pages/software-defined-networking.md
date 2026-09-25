---
title: "Software-defined networking"
date: 2026-08-14
summary: "Separating the control plane from the data plane, SD-WAN across dispersed sites, VXLAN overlays and data center interconnect, and defining infrastructure in files instead of consoles."
tags: [education, networking, sdn, sd-wan, vxlan, automation]
---

## Overview

A traditional network is configured device by device. Someone logs into a
switch, sets a VLAN, logs into the next one, and repeats. That works at the
scale it was designed for and stops working somewhere past it. No single step
is hard. The problem is that every added device is one more place a mistake can
hide.

Everything on this page is a response to that problem. The designs differ in
scope, but they all move decisions out of individual devices and into one
central place that can be reviewed, versioned, and audited. The device keeps
forwarding packets, but policy no longer lives on it.

Centralizing control also means a mistake in the central place affects
everything, which is why each of these comes with a policy framework. The
security side, verifying every request instead of trusting a location, is
covered on [zero-trust architecture](/learn/zero-trust-architecture).

## Separating the planes

Software-defined networking (SDN) separates two jobs that traditional
network hardware performs together:

- The control plane decides how traffic should flow: which path a packet takes,
  which routes exist, and what is permitted.
- The data plane, also called the forwarding plane, moves packets along
  those paths.

In a conventional switch or router both planes live in the same chassis. Each
device makes its own decisions from its own view of the network. SDN pulls the
control plane out into a central controller, leaving the hardware to forward.
One device with a complete picture can make choices no individual switch could.

(A third plane, the management plane, is the configuration and monitoring
interface, and it is usually described alongside these. SDN discussions focus on
the control and data split because that is the one SDN changes.)

The architecture is usually drawn as three layers, with the controller in the
middle:

| Layer | Contains | Talks to the controller via |
|---|---|---|
| Application layer | Business logic and policy the operator defines | Northbound API |
| Control layer | The SDN controller | — |
| Infrastructure layer | Physical switches, routers, virtual appliances | Southbound API |

The northbound API is how intent enters the system: an application or an
operator states what should be true. The southbound API is how the
controller pushes the resulting configuration down to hardware. OpenFlow is the
best-known southbound protocol, though vendor implementations vary widely.

The business logic at the top is ordinary operational policy, expressed once
instead of per-device:

- prioritize live video and audio over bulk transfer;
- scale bandwidth for an e-commerce tier during a sale; or
- enforce a security policy across every edge simultaneously.

## SD-WAN

A wide area network (WAN) is geographically dispersed by definition, which
makes every hands-on maintenance task expensive. SD-WAN applies the SDN
model to that problem: centralize the control plane, and let branch sites take
policy from it rather than from a local engineer.

SD-WAN manages traffic across whatever links a site already has, such as
multiprotocol label switching (MPLS), broadband, or LTE and 5G, and chooses
between them per application. In practice, traffic no longer has to pass
through a central hub to be inspected before reaching its destination. A branch office reaching a cloud service can go directly, under
policy, instead of hairpinning through headquarters.

### What these designs buy you

**Application-aware routing.** Because the controller sees traffic in terms of
applications and not only ports and addresses, it can prioritize in finer
detail than a standalone device can. Voice takes the low-latency link, and a
backup takes the cheap one.

**Zero-touch provisioning (ZTP).** A new device powers on, retrieves its
configuration from a known URL, and applies it without anyone typing. Across a
large deployment this removes both the labor and the most common source of
error, which is a person configuring the same thing fifty times and doing it
differently once.

**Transport independence.** The overlay does not care what carries it. With
real-time monitoring, an SD-WAN can shift traffic to 5G when a fiber circuit
degrades and shift back when it recovers, without a configuration change.

**Central policy management.** One place to manage policy for the whole
network. Misconfiguration is a leading cause of network outages and comes up
again and again in breach reports. Reducing the number of places a
configuration can be wrong helps security as much as operations.

## VXLAN

VLAN IDs are 12 bits, which allows 4,096 values, or 4,094 usable ones, since 0
and 4095 are reserved. In a single building that is ample. In a service provider
or a multi-tenant data center it is a hard ceiling, and reaching it means
hardware reconfiguration rather than a policy change.

Virtual Extensible LAN (VXLAN) removes the ceiling by encapsulating Layer 2
frames inside UDP datagrams and carrying them over a Layer 3 network. The
identifier grows from 12 bits to 24, giving about 16.7 million virtual network
identifiers (VNIs).

The more important consequence is architectural. Because the segment is now an
overlay riding on routed infrastructure, two hosts can share a Layer 2 domain
without sharing a physical one. The underlying network only has to route IP.

VXLAN tunnel endpoints (VTEPs) do the encapsulation and decapsulation. A VTEP
can be a switch, a hypervisor, or software on a host. It is the boundary between
the virtual segment and the physical network, and the point where the VNI is
added or removed.

### Data center interconnect

Data center interconnect (DCI) uses this to join physically separate
facilities into what workloads experience as one. A virtual machine can migrate
between sites without changing address, because the segment it lives on spans
both. That supports resource sharing, workload migration, and disaster recovery
where a standby site can take over as it is, instead of being a copy that needs
reconfiguring first.

## Infrastructure as code

Infrastructure as code (IaC) describes infrastructure in files instead of a
sequence of console actions. A template defines what should exist, and a tool
makes the real environment match it. In a cloud environment such as a virtual
private cloud, a new environment comes from running a definition, not from
someone recreating it from memory.

Speed helps, but the bigger gain is that the definition can be reviewed,
diffed, and repeated. Three environments built from one template are identical
in a way that three built by hand never are.

Orchestration is the layer above: coordinating multiple automated tasks into
a workflow, in the right order, with dependencies respected. Automation makes
one task repeatable. Orchestration makes a sequence of them reliable.

## Source control

IaC only delivers on its promise if the definitions are themselves managed.
Source control tracks changes to files over time, and the features that
matter here are the ordinary ones:

- **version control**, so every change has a history and a previous state to
  return to;
- a central repository, so there is one authoritative copy rather than
  several divergent ones;
- **conflict identification**, so two simultaneous changes to the same thing
  surface rather than silently overwriting; and
- **branching and merging**, so work in progress stays separate until it is
  ready.

Applied to network configuration, this answers questions you otherwise cannot:
what changed, when, who changed it, and what it looked like before. Knowing the
previous state is what lets you roll back instead of rebuilding.

## Suggested practice: build an overlay you can inspect

These designs are mostly enterprise-scale, but the mechanisms are observable on
one machine with free software.

1. Create a VXLAN interface on a Linux host and give it an address:
   `sudo ip link add vxlan0 type vxlan id 42 dev eth0 dstport 4789`, then
   `sudo ip addr add 10.42.0.1/24 dev vxlan0` and bring it up. Confirm the VNI
   with `ip -d link show vxlan0`.
2. Repeat on a second host or VM with a different address on the same subnet,
   add each as the other's remote, and ping across. Capture the traffic with
   `sudo tcpdump -i eth0 udp port 4789` and find the inner frame inside the
   outer UDP datagram. That is encapsulation you can point at.
3. Put a network configuration under version control. Commit a working state,
   change something, and read the diff. Break it on purpose and recover with
   `git revert`.
4. Write the same change as a declarative definition, such as a `netplan` YAML
   file, an Ansible playbook, or a `systemd-networkd` unit, and apply it twice. Confirm the
   second run changes nothing. Idempotence is the property that makes IaC safe
   to re-run.
5. Read your own router configuration and identify where the decision is made
   versus where it is enforced. On a single home router these are the same box.
   Naming the two roles separately is the concept.

## Exam key points

Where this page's material shows up in the exam objectives, and what each exam
expects beyond it.

### CCNA 200-301

Objectives 6.1 to 6.7 in v1.1 cover automation, controller-based networking,
overlay, underlay, and fabric, northbound and southbound APIs, REST, Ansible and
Terraform, and JSON. In v2.0, which replaces it in February 2027, these narrow to
5.3, network management approaches including infrastructure as code, and 5.5,
running commands with Ansible.

- Northbound APIs connect the controller to applications, usually over REST.
  Southbound APIs connect it to devices, with protocols such as NETCONF, RESTCONF,
  and OpenFlow.
- An overlay, such as VXLAN tunnels, runs on an underlay, the physical routed
  network. Together they form the fabric. Cisco SD-Access uses VXLAN for the data
  plane and LISP for the control plane.
- REST verbs map to CRUD: POST creates, GET reads, PUT and PATCH update, and
  DELETE deletes.
- Ansible is agentless, uses YAML playbooks, and connects over SSH. Terraform is
  declarative infrastructure as code.
- For JSON, know objects in braces, arrays in brackets, and key-value pairs.

### Network+ N10-009

Objective 1.8 lists SDN and SD-WAN (application aware, zero-touch provisioning,
transport agnostic, and central policy), VXLAN, infrastructure as code, and source
control.

- Transport agnostic means SD-WAN runs over any link type, such as MPLS,
  broadband, or LTE.
- VXLAN has a 24-bit VNI, about 16 million segments against 4,094 VLANs, and uses
  UDP 4789.
- Infrastructure as code terms to know: playbooks and templates, configuration
  drift and compliance, upgrades, and dynamic inventories.
- Source control terms to know: version control, a central repository, conflict
  identification, and branching.

## Related pages

- [Zero-trust architecture](/learn/zero-trust-architecture) — the security model
  that assumes no location on the network is trustworthy.
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
- [RFC 7426: Software-Defined Networking (SDN): Layers and Architecture Terminology](https://www.rfc-editor.org/rfc/rfc7426.txt)
  — the layered terminology and where the northbound and southbound interfaces sit.
- [IEEE 802.1Q](https://standards.ieee.org/ieee/802.1Q/10323/) — the VLAN tag
  format that sets the 12-bit ceiling VXLAN works around.
- [RFC 8926: Geneve](https://www.rfc-editor.org/rfc/rfc8926.txt) — a later
  encapsulation solving the same problem, useful for seeing what VXLAN fixed in
  place and what it left open.

Vendor implementations vary a lot here, and the marketing terms change faster
than the standards. The basic split, deciding centrally and enforcing at the
edge, is the part that lasts.
