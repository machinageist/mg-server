---
title: "Network functions: tunnels, traffic priority, and packet lifetime"
date: 2026-07-23
summary: "How VPNs and IPsec protect traffic, how QoS treats competing traffic, and how IP limits packet lifetime."
tags: [education, networking, vpn, ipsec, qos, ttl, network-plus]
---

## Overview

Networks do more than forward traffic. They can encapsulate one protocol inside
another, protect traffic across an untrusted path, give some traffic preferential
treatment, and prevent packets from circulating forever. VPNs, IPsec, quality of
service (QoS), and IP packet lifetime are examples of these functions.

They solve different problems:

- **Tunneling** carries one packet or protocol inside another.
- A **virtual private network (VPN)** uses a tunnel to connect users or networks
  across infrastructure they do not control, normally with authentication and
  encryption.
- **IPsec** is a standards-based suite for protecting IP traffic.
- **QoS** classifies and manages competing traffic.
- IPv4 **Time to Live (TTL)** and the IPv6 **Hop Limit** bound how long a packet can
  remain in a routed network.

## Tunneling and VPNs

Tunneling is encapsulation: an inner packet is carried as payload inside an outer
packet. Encapsulation alone does **not** guarantee confidentiality, integrity, or
peer authentication. A secure VPN combines tunneling with appropriate cryptographic
protection and identity checks.

There is no single universal VPN protocol. Designs vary by use case, operating
system, and administrative environment. L2TP can be paired with IPsec, but that is
one design rather than "the" modern standard. Browser-based remote-access products
also vary; their security cannot be judged from the fact that they use HTML5.

## IPsec

IPsec protects communication at the IP layer. Its architecture uses **Security
Associations (SAs)**: one-way agreements describing how selected traffic will be
protected, including algorithms, keys, lifetimes, and other parameters. Bidirectional
communication therefore needs SAs for both directions.

Two IPsec protocols appear in the architecture:

- **Authentication Header (AH)** provides integrity and data-origin authentication
  for protected portions of a packet, but not confidentiality.
- **Encapsulating Security Payload (ESP)** can provide confidentiality, integrity,
  data-origin authentication, and anti-replay protection, depending on the selected
  services.

ESP is the commonly deployed choice when encryption is required. Older algorithms
such as DES, 3DES, MD5, and SHA-1 appear in historical material but should not be
presented as modern defaults. Current deployments must follow the requirements of
the relevant standards and implementation documentation.

### Transport and tunnel modes

- **Transport mode** protects the upper-layer payload while retaining the original IP
  header for routing.
- **Tunnel mode** protects an entire inner IP packet and wraps it in a new outer IP
  packet. This is common for gateway-to-gateway VPNs.

## Internet Key Exchange

**Internet Key Exchange (IKE)** authenticates IPsec peers, negotiates cryptographic
parameters, and creates the SAs and keying material needed to protect traffic.
Diffie–Hellman key agreement allows peers to derive shared secret material across an
untrusted network without sending the resulting secret directly.

Historical descriptions often divide IKEv1 into "Phase 1" and "Phase 2," with
**Quick Mode** belonging to IKEv1 Phase 2. IKEv2 replaces that exchange structure
with exchanges such as `IKE_SA_INIT`, `IKE_AUTH`, and `CREATE_CHILD_SA`. UDP port 500
is commonly used for IKE; UDP port 4500 is used when IPsec traverses network address
translation.

The important conceptual sequence is:

1. negotiate supported cryptographic parameters;
2. perform key agreement;
3. authenticate the peers;
4. establish an IKE SA; and
5. establish child SAs that protect selected IP traffic.

## Quality of Service

**Quality of service (QoS)** is a family of techniques for controlling how network
devices treat competing traffic. It can support performance goals and service-level
agreements through mechanisms such as:

- classification and marking;
- traffic prioritization and scheduling;
- bandwidth allocation;
- shaping and policing; and
- congestion avoidance.

QoS is configured on devices such as routers and switches, but useful end-to-end
behavior depends on consistent policy across the path. Marking a packet does not
force every later network to honor the requested treatment, and QoS does not create
bandwidth that does not exist.

## Packet lifetime: TTL and Hop Limit

The IPv4 header contains an eight-bit **TTL** field. Each router that forwards the
packet reduces the value; when the value reaches zero, the router discards the
packet. IPv6 names the corresponding field **Hop Limit**. This prevents a routing
loop from circulating a packet indefinitely.

Despite the historical name "Time to Live," modern forwarding behavior makes it a
hop bound rather than a wall-clock lifetime. Operating systems choose different
initial values, so `128` is common but not universal. Tools such as `traceroute` and
`tracepath` deliberately vary this value to reveal successive routers along a path.

## Suggested practice: observe packet lifetime

On a system and destination you are authorized to use:

1. Run `tracepath` or `traceroute` to a reachable destination.
2. Record each visible hop and any hop that does not reply.
3. Compare the output with the local routing table.
4. Explain why a missing reply does not necessarily mean that forwarding stopped.
5. Repeat against a local destination and compare the paths.

This lab provides evidence for packet lifetime and routing behavior; it does not by
itself demonstrate VPN confidentiality or QoS policy.

## Related pages

- [The OSI model](/wiki/osi-model) — the layered reference model that places these
  functions in a larger communication system.
- [Network appliances](/wiki/network-appliances) — routers, firewalls, and other
  systems that implement these functions.
- [Network applications](/wiki/network-applications) — distributed applications whose
  traffic may be tunneled, filtered, or prioritized.

## Sources and further reading

This page was edited from my Network+ reading notes and corrected against:

- [RFC 791: Internet Protocol](https://www.rfc-editor.org/rfc/rfc791.txt) — IPv4 TTL.
- [RFC 2474: Differentiated Services](https://www.rfc-editor.org/rfc/rfc2474.txt) — an
  architecture for differentiated traffic treatment.
- [RFC 4301: Security Architecture for IP](https://www.rfc-editor.org/rfc/rfc4301.txt)
  — IPsec architecture and Security Associations.
- [RFC 7296: Internet Key Exchange Protocol Version 2](https://www.rfc-editor.org/rfc/rfc7296.txt)
  — IKEv2 exchanges and negotiation.
