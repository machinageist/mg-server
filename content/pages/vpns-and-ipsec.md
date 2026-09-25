---
title: "VPNs and IPsec: tunnels, security associations, and key exchange"
date: 2026-09-25
summary: "What a tunnel adds and what it does not, how a VPN protects traffic across a network you do not control, and how IPsec's AH, ESP, and IKE fit together."
tags: [education, networking, vpn, ipsec, tunneling]
---

## Overview

A virtual private network (VPN) connects users or networks across infrastructure
they do not control, such as the internet, as if they shared a private link. Two
separate ideas make that work. Tunneling carries one packet inside another so it
can cross a network that would not carry it on its own. Cryptography makes the
tunnel private and proves who is on each end.

Keeping those two ideas apart makes the rest of this page easier to follow. A
tunnel with no cryptography is still a tunnel, and it hides nothing. IPsec is a
standards-based suite that adds the cryptography to IP traffic.

## Tunneling and VPNs

Tunneling is encapsulation: an inner packet becomes the payload of an outer packet.
Encapsulation alone provides no confidentiality, integrity, or peer authentication. A
secure VPN adds suitable cryptography and identity checks.

There is no universal VPN protocol. The design depends on the use case, operating
systems, and administrative environment. L2TP can be paired with IPsec, but that is one
design rather than "the" modern standard. Browser-based remote-access products also
vary; using HTML5 says little by itself about their security.

A VPN comes in two common shapes. A site-to-site VPN joins two networks through
gateways at each end, and the hosts behind them do not know it is there. A
remote-access VPN connects one device, usually through a client application, to a
network it is not on. A remote-access VPN can send all of the device's traffic
through the tunnel (full tunnel) or only the traffic for the private network
(split tunnel).

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

## Suggested practice: look inside a tunnel

Any VPN you already use will do, whether it is WireGuard, OpenVPN, or a work
client. If you do not use one, set up WireGuard between two machines or virtual
machines you own, following the `wg(8)` manual page.

1. With the VPN connected, run `ip -br link` and find the tunnel interface. It
   often has a name such as `wg0` or `tun0`.
2. Run `ip route` and see which destinations go through the tunnel interface. If
   only the private network does, the VPN is a split tunnel. If the default route
   does, it is a full tunnel.
3. Ping something on the far side of the tunnel. In another terminal, capture on
   the tunnel interface with `sudo tcpdump -n -i <tunnel interface> -c 5`. You see
   the inner packets, readable.
4. Repeat the capture on your physical interface. You see only the outer packets,
   such as UDP to the VPN server's port, with the inner packets encrypted inside
   them.

That difference between steps 3 and 4 is the whole idea of a VPN: encapsulation
plus encryption. WireGuard and OpenVPN are not IPsec, but the tunnel behaves the
same way from the outside.

## Exam key points

Where this page's material shows up in the exam objectives, and what each exam
expects beyond it.

### CCNA 200-301

Objective 5.5 in v1.1 describes IPsec remote-access and site-to-site VPNs. In v2.0,
which replaces it in February 2027, that becomes 4.5 and adds IPsec protocols and
transport modes.

- A site-to-site VPN joins networks through gateways. A remote-access VPN connects
  individual clients, usually with a client application.
- ESP provides encryption and AH does not. IKE negotiates the security
  associations. Know transport mode versus tunnel mode for v2.0.

### Network+ N10-009

Objective 1.2 lists VPN as a network function, 1.4 lists GRE and IPsec with AH,
ESP, and IKE, and 3.5 compares site-to-site and client-to-site VPNs, including
clientless and split versus full tunnel.

- GRE tunnels traffic without encrypting it, which is why it is often paired with
  IPsec.
- AH gives integrity without confidentiality. ESP gives both. IKE uses UDP 500,
  and UDP 4500 through NAT.
- A clientless VPN runs in a web browser over TLS instead of an installed client.

## Related pages

- [Network protocols and ports](/learn/network-protocols) — GRE and the other
  protocols a tunnel can carry, and the ports IKE uses.
- [Software-defined networking](/learn/software-defined-networking) — VXLAN, which
  uses the same encapsulation idea to stretch a layer 2 segment.
- [Zero-trust architecture](/learn/zero-trust-architecture) — the model that is
  replacing "on the VPN means trusted" for remote access.
- [Cloud computing concepts](/learn/cloud-computing) — site-to-site VPNs as one way
  to connect to a cloud network.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [RFC 4301: Security Architecture for IP](https://www.rfc-editor.org/rfc/rfc4301.txt)
  — IPsec architecture and Security Associations.
- [RFC 4302: IP Authentication Header](https://www.rfc-editor.org/rfc/rfc4302.txt)
  — AH, and what it does and does not protect.
- [RFC 4303: IP Encapsulating Security Payload](https://www.rfc-editor.org/rfc/rfc4303.txt)
  — ESP.
- [RFC 7296: Internet Key Exchange Protocol Version 2](https://www.rfc-editor.org/rfc/rfc7296.txt)
  — IKEv2 exchanges and negotiation.
- [RFC 2784: Generic Routing Encapsulation](https://www.rfc-editor.org/rfc/rfc2784.txt)
  — GRE, a tunnel with no cryptography of its own.
- [wg(8)](https://man.archlinux.org/man/wg.8) — the WireGuard configuration tool used
  in the practice section.
