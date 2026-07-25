---
title: "The OSI model"
date: 2026-07-23
summary: "A seven-layer reference model for connecting network functions, protocols, devices, encapsulation, and troubleshooting."
tags: [education, networking, osi, tcp-ip, troubleshooting, network-plus]
---

## Overview

The **Open Systems Interconnection (OSI) model** divides data communication into
seven functional layers. It gives engineers, operators, vendors, and learners a
shared vocabulary for describing what a communication system must do without treating
every protocol and device as one indivisible mechanism.

The OSI model is a **reference model**, not a claim that every real protocol stack is
implemented as seven separate programs or that every protocol belongs perfectly to
one layer. Its practical value is decomposition: when communication fails, the model
helps ask which function is missing, which information is available at that point,
and which component could be responsible.

## Historical purpose

Computer networks developed from incompatible hardware, operating systems, and
vendor-specific protocol families. The International Organization for Standardization
(ISO) developed the OSI Basic Reference Model to describe a common architecture for
open communication between systems. The model separates concerns so that one layer
can change without requiring every other layer to be redesigned.

The Internet ultimately standardized around the TCP/IP protocol suite rather than a
complete OSI protocol suite. The seven-layer model nevertheless remained useful for
teaching, design discussions, documentation, and troubleshooting because it names
functions that still have to happen in real networks.

## The seven layers at a glance

| Layer | Name | Primary concern | Common examples |
|---:|---|---|---|
| 7 | Application | Network services used by applications | HTTP, DNS, SMTP, FTP, SSH |
| 6 | Presentation | Representation, encoding, compression, encryption | Unicode, data formats, TLS-related presentation functions |
| 5 | Session | Establishing, coordinating, and ending exchanges | Session state, checkpoints, dialogs |
| 4 | Transport | End-to-end transport between processes | TCP, UDP, ports |
| 3 | Network | Logical addressing and forwarding between networks | IPv4, IPv6, ICMP, routers |
| 2 | Data Link | Local-link framing, addressing, and media access | Ethernet frames, MAC addresses, switches, bridges |
| 1 | Physical | Signals and transmission media | Copper, fiber, radio, transceivers, hubs, repeaters |

Examples in this table are teaching mappings rather than rigid ownership claims. TLS,
for example, performs presentation-like encryption but is implemented above TCP and
used by application protocols. Firewalls and load balancers may inspect information
from several layers.

## Layer 1: Physical

The Physical layer carries raw bits as electrical, optical, or radio signals. It is
concerned with the physical medium and signaling needed to move information between
adjacent devices.

Examples include:

- copper and fiber-optic cabling;
- connectors and transceivers;
- radio frequencies used by wireless networks;
- repeaters that regenerate a weakened signal; and
- hubs that repeat received signals to every connected port.

Cable category, signal quality, interference, and the properties of the medium affect
possible speed and distance. A Layer 1 failure may look like no link light, damaged
cabling, an unplugged interface, radio interference, or a mismatched transceiver.

## Layer 2: Data Link

The Data Link layer transfers data across one local link. It organizes data into
**frames**, identifies interfaces with link-layer addresses, controls access to shared
media, and detects certain transmission errors.

An Ethernet frame includes source and destination MAC addresses and a frame check
sequence based on a cyclic redundancy check (CRC). The CRC can detect corruption; it
does not by itself repair the damaged frame.

Layer 2 systems include:

- **switches**, which learn source MAC addresses and forward frames toward associated
  ports;
- **bridges**, which connect and regulate traffic between link segments; and
- **network interface cards (NICs)**, which participate in both physical signaling and
  link-layer framing.

IEEE 802 networking further divides Data Link responsibilities into Logical Link
Control (LLC) and Media Access Control (MAC) sublayers. VLANs extend Layer 2 by
creating separate logical broadcast domains over switching infrastructure.

## Layer 3: Network

The Network layer provides logical addressing and forwards packets between different
networks. IP addresses identify interfaces in an internetwork, subnetting divides
address space, and routers choose a next hop toward a destination.

Layer 3 concepts and devices include:

- IPv4 and IPv6 packets;
- IP addresses and prefixes;
- routing tables;
- routers and Layer 3 switches;
- ICMP control and diagnostic messages; and
- packet-filtering rules based on network-layer information.

A Layer 3 problem can exist even when Layers 1 and 2 work correctly. A host may have a
valid local link but an incorrect address, prefix, gateway, or route to a remote
network.

## Layer 4: Transport

The Transport layer provides communication between processes on endpoint systems.
Port numbers identify transport endpoints so multiple applications can use the same
host and network interface.

### TCP

The **Transmission Control Protocol (TCP)** is connection-oriented. It uses a
three-way handshake—SYN, SYN/ACK, ACK—to establish state before application data is
exchanged. TCP numbers bytes, acknowledges received data, retransmits data inferred to
be lost, controls flow between endpoints, and reacts to congestion.

TCP provides reliable, ordered delivery to the application when the connection
succeeds. Describing this as "error correction" can be misleading: checksums detect
certain corruption, while acknowledgements and retransmission recover from missing
transport data.

### UDP

The **User Datagram Protocol (UDP)** sends independent datagrams without establishing
a TCP-style connection. UDP does not guarantee delivery, ordering, or duplicate
suppression. An application can add any reliability or sequencing it needs.

UDP is used where a small transport mechanism, application-controlled timing, or
request/response simplicity is useful. Streaming and games are common examples, but
UDP is not automatically faster in every system and is not limited to real-time
traffic.

Stateful firewalls and Layer 4 load balancers inspect transport information as well as
network-layer addresses. Their behavior crosses conceptual layer boundaries rather
than making the appliance itself belong exclusively to one layer.

## Layer 5: Session

The Session layer describes coordination of an ongoing exchange between applications:
establishing the dialog, maintaining its state, adding synchronization or checkpoints,
and ending it cleanly.

In modern Internet software, these responsibilities are often implemented inside
application protocols, libraries, or the applications themselves rather than exposed
as an independent Session-layer protocol. Login state, an RPC conversation, or a
recoverable long-running transfer may all have session-like behavior without mapping
cleanly to a distinct Layer 5 component.

NetBIOS historically provided session services, but protocols that use those services
should not all be labeled Layer 5. SMB, for example, is normally treated as an
application-layer file-sharing protocol.

## Layer 6: Presentation

The Presentation layer describes how data is represented so communicating
applications can interpret it consistently. Its concerns include:

- character encoding, such as ASCII and Unicode;
- serialization and data formats;
- compression and decompression; and
- encryption and decryption.

These functions operate on the representation of application data rather than routing
packets. Real implementations frequently place them in application libraries or
protocol stacks instead of a visibly separate Presentation layer.

TLS performs encryption and peer-authentication functions often associated with this
layer, but TLS has a concrete position in the Internet stack: applications use it
above a reliable transport such as TCP. SSL is its obsolete predecessor and should
not be used as a current protocol name.

## Layer 7: Application

The Application layer provides network services directly to software such as web
browsers, mail clients, command-line tools, and server applications. It is the layer
closest to the user's intended task, not the graphical interface itself.

Common examples include:

- HTTP and HTTPS for web communication;
- DNS for name resolution;
- SMTP, POP3, and IMAP for email;
- FTP for file transfer; and
- SSH for encrypted remote access.

Application-layer systems understand the meaning and structure of a specific protocol.
A web application firewall (WAF), for example, can inspect HTTP requests for patterns
associated with attacks such as SQL injection or cross-site scripting. Intrusion
prevention systems may inspect application content as well as information from lower
layers.

## Encapsulation and decapsulation

When an application sends data, each relevant layer adds information needed for its
part of delivery. A simplified path is:

```text
Application data
  → TCP segment or UDP datagram
    → IP packet
      → link-layer frame
        → physical signals
```

At the receiving endpoint, the process is reversed. The network interface receives
signals and reconstructs a frame; the IP implementation processes the packet; TCP or
UDP identifies the destination transport endpoint; and the application interprets the
remaining data.

Names such as **segment**, **datagram**, **packet**, **frame**, and **bits** are useful
because they identify the information available at different stages. In ordinary
speech, people often use "packet" more broadly, so context still matters.

Intermediate devices process only what their function requires. A switch normally
uses Layer 2 information to move a frame within a LAN. A router removes the incoming
link-layer framing, evaluates the Layer 3 packet, and creates new Layer 2 framing for
the next link. An application proxy may terminate one connection and create another,
allowing it to act on Layer 7 information.

## OSI and the TCP/IP model

The Internet architecture is commonly described with fewer layers:

| TCP/IP view | Approximate OSI relationship |
|---|---|
| Application | OSI Layers 5–7 |
| Transport | OSI Layer 4 |
| Internet | OSI Layer 3 |
| Link | OSI Layers 1–2 |

This is an approximate comparison, not a mathematical equivalence. OSI is valuable for
asking detailed functional questions; the TCP/IP model more closely reflects the
protocol architecture actually used by the Internet.

## Using layers to troubleshoot

Layered troubleshooting replaces "the network is down" with narrower questions:

1. **Physical:** Is the interface connected and signaling?
2. **Data Link:** Can the host exchange frames on its local link? Is it in the expected
   VLAN?
3. **Network:** Does it have the correct address and route? Can packets reach the
   destination network?
4. **Transport:** Is the expected port reachable? Does a TCP handshake complete?
5. **Application:** Does the service understand the request and return a valid
   response?

Layers 5 and 6 may still guide investigation of session state, encoding, encryption,
or certificates even when the software does not expose them as separate components.
The model does not require troubleshooting strictly from Layer 1 upward; it helps make
the chosen starting point and assumptions explicit.

## Suggested practice: map one web request

Use a site you own or have permission to inspect and map a `curl` request through the
model:

1. Record the application request and response with `curl -v`.
2. Identify the hostname resolution needed before the connection.
3. Identify the transport protocol and destination port.
4. Record the local address, gateway, and selected route.
5. Identify the local link interface and its MAC address.
6. Describe the physical or virtual medium carrying the first link.
7. Mark TLS, proxies, tunnels, or other components that do not fit neatly in one box.
8. Separate what your output demonstrates from what you infer about infrastructure you
   cannot inspect.

For this site, the completed [hosting walkthrough](/blog/hosting-machinageist-dev)
provides evidence for a real request path through DNS, Cloudflare's edge, a tunnel,
Caddy, systemd, and the Rust application. The OSI model supplies one way to classify
the functions; the captured commands supply the evidence.

## Related pages

- [Network appliances](/learn/network-appliances) — switches, routers, firewalls,
  proxies, load balancers, storage, and wireless systems.
- [Network applications](/learn/network-applications) — content delivery networks and
  distributed request paths.
- [Network functions](/learn/network-functions) — tunnels, IPsec, QoS, and IP packet
  lifetime.

## Sources and further reading

This page was edited from my networking reading notes and study log, with technical
clarifications checked against:

- [ISO/IEC 7498-1:1994 — Basic Reference Model](https://www.iso.org/standard/20269.html)
  — the formal OSI reference model.
- [RFC 1122: Requirements for Internet Hosts — Communication Layers](https://www.rfc-editor.org/rfc/rfc1122.txt)
  — the Internet host communication-layer model.
- [RFC 9293: Transmission Control Protocol](https://www.rfc-editor.org/rfc/rfc9293.txt)
  — current TCP specification.
- [RFC 768: User Datagram Protocol](https://www.rfc-editor.org/rfc/rfc768.txt) — UDP's
  minimal datagram service.

Protocol specifications define actual behavior. The OSI model remains a tool for
organizing and communicating that behavior.
