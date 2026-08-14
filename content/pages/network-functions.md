---
title: "Network functions: tunnels, traffic priority, and packet lifetime"
date: 2026-07-23
summary: "How VPNs and IPsec protect traffic, how QoS treats competing traffic, and how IP limits packet lifetime."
tags: [education, networking, vpn, ipsec, qos, ttl]
---

## Overview

Forwarding is only part of what a network does. Networks can wrap one protocol inside
another, protect traffic across an untrusted path, prioritize competing traffic, and
stop packets from circulating forever. VPNs, IPsec, quality of service (QoS), and IP
packet lifetime cover those jobs.

They solve different problems:

- Tunneling carries one packet or protocol inside another.
- A virtual private network (VPN) connects users or networks across infrastructure
  they do not control, normally with authentication and encryption.
- IPsec is a standards-based suite for protecting IP traffic.
- QoS classifies and manages competing traffic.
- IPv4 Time to Live (TTL) and the IPv6 Hop Limit bound a packet's path through a
  routed network.

## Tunneling and VPNs

Tunneling is encapsulation: an inner packet becomes the payload of an outer packet.
Encapsulation alone provides no confidentiality, integrity, or peer authentication. A
secure VPN adds suitable cryptography and identity checks.

There is no universal VPN protocol. The design depends on the use case, operating
systems, and administrative environment. L2TP can be paired with IPsec, but that is one
design rather than "the" modern standard. Browser-based remote-access products also
vary; using HTML5 says little by itself about their security.

## IPsec

IPsec protects communication at the IP layer. Its Security Associations (SAs) are
one-way agreements that define how selected traffic is protected, including the
algorithms, keys, and lifetimes. Bidirectional communication needs SAs in both
directions.

The architecture includes two IPsec protocols:

- Authentication Header (AH) provides integrity and data-origin authentication for
  protected parts of a packet, but no confidentiality.
- Encapsulating Security Payload (ESP) can provide confidentiality, integrity,
  data-origin authentication, and anti-replay protection, depending on the selected
  services.

ESP is the usual choice when encryption is needed. DES, 3DES, MD5, and SHA-1 may appear
in old material, but they are not modern defaults. Current deployments should follow
the relevant standards and implementation documentation.

### Transport and tunnel modes

- Transport mode protects the upper-layer payload while keeping the original IP
  header for routing.
- Tunnel mode protects the complete inner IP packet and wraps it in a new outer IP
  packet. Gateway-to-gateway VPNs commonly use this mode.

## Internet Key Exchange

Internet Key Exchange (IKE) authenticates IPsec peers, negotiates cryptographic
parameters, and creates the SAs and key material used to protect traffic.
Diffie–Hellman key agreement lets the peers derive shared secret material across an
untrusted network without sending the resulting secret directly.

Older descriptions divide IKEv1 into "Phase 1" and "Phase 2"; Quick Mode belongs to
IKEv1 Phase 2. IKEv2 instead uses exchanges such as `IKE_SA_INIT`, `IKE_AUTH`, and
`CREATE_CHILD_SA`. IKE commonly uses UDP port 500. IPsec traversal through network
address translation uses UDP port 4500.

The sequence, stripped to its main steps, is:

1. negotiate supported cryptographic parameters;
2. perform key agreement;
3. authenticate the peers;
4. establish an IKE SA; and
5. establish child SAs for the selected IP traffic.

## Quality of Service

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

## Packet lifetime: TTL and Hop Limit

The IPv4 header has an eight-bit TTL field. Each forwarding router decreases it. When
it reaches zero, the router discards the packet. IPv6 calls the corresponding field
Hop Limit. Without this bound, a routing loop could circulate a packet indefinitely.

Despite the name "Time to Live," modern forwarding treats TTL as a hop bound rather
than a wall-clock timer. Operating systems choose different initial values, so `128`
is common but not universal. Tools such as `traceroute` and `tracepath` vary the value
to expose successive routers along a path.

## Suggested practice: observe packet lifetime

On a system and destination you are authorized to use:

1. Run `tracepath` or `traceroute` to a reachable destination.
2. Record each visible hop and each hop that does not reply.
3. Compare the result with the local routing table.
4. Explain why a missing reply does not prove that forwarding stopped.
5. Repeat with a local destination and compare the paths.

This shows packet lifetime and routing behavior. It does not demonstrate VPN
confidentiality or the presence of a QoS policy.

## Related pages

- [The OSI model](/learn/osi-model) — the layered reference model that places these
  functions in a larger communication system.
- [Network appliances](/learn/network-appliances) — routers, firewalls, and other
  systems that implement these functions.
- [Network applications](/learn/network-applications) — distributed applications whose
  traffic may be tunneled, filtered, or prioritized.

## Sources and further reading

I checked these networking notes against:

- [RFC 791: Internet Protocol](https://www.rfc-editor.org/rfc/rfc791.txt) — IPv4 TTL.
- [RFC 2474: Differentiated Services](https://www.rfc-editor.org/rfc/rfc2474.txt) — an
  architecture for differentiated traffic treatment.
- [RFC 4301: Security Architecture for IP](https://www.rfc-editor.org/rfc/rfc4301.txt)
  — IPsec architecture and Security Associations.
- [RFC 7296: Internet Key Exchange Protocol Version 2](https://www.rfc-editor.org/rfc/rfc7296.txt)
  — IKEv2 exchanges and negotiation.
