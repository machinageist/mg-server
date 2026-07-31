---
title: "The OSI model"
date: 2026-07-23
summary: "A seven-layer reference model for connecting network functions, protocols, devices, encapsulation, and troubleshooting."
tags: [education, networking, osi, tcp-ip, troubleshooting, network-plus]
---

## Overview

The Open Systems Interconnection (OSI) model splits network communication into seven
layers. It gives us a shared vocabulary for protocols, devices, and failures without
turning the whole network into one large black box.

It is a reference model, not a literal diagram of every protocol stack. Real software
does not always implement seven separate layers, and some protocols cross the lines.
The model is still useful because it narrows a problem: which function failed, what
information is available there, and which component handles it?

## Where it came from

Computer networks grew out of incompatible hardware, operating systems, and
vendor-specific protocol families. The International Organization for Standardization
(ISO) developed the OSI Basic Reference Model as a common architecture for open
communication between systems. Separating the layers also made it possible to change
one part without redesigning everything around it.

The Internet standardized on TCP/IP rather than the complete OSI protocol suite. The
seven-layer model remained useful for teaching, documentation, design, and
troubleshooting because these functions still exist in real networks.

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

Treat these as working mappings, not rigid ownership rules. TLS does
presentation-like work, but it runs above TCP and serves application protocols.
Firewalls and load balancers may inspect several layers at once.

## Layer 1: Physical

The Physical layer carries bits as electrical, optical, or radio signals. It covers
the medium and signaling between adjacent devices:

- copper and fiber-optic cabling;
- connectors and transceivers;
- radio frequencies used by wireless networks;
- repeaters that regenerate a weakened signal; and
- hubs that repeat signals to every connected port.

Cable category, signal quality, interference, and the medium itself limit speed and
distance. Typical Layer 1 failures include no link light, damaged cabling, an
unplugged interface, radio interference, or a mismatched transceiver.

## Layer 2: Data Link

The Data Link layer moves data across one local link. It builds frames, addresses
interfaces on that link, controls access to shared media, and detects some transmission
errors.

An Ethernet frame includes source and destination MAC addresses and a frame check
sequence based on a cyclic redundancy check (CRC). The CRC detects corruption; it
does not repair the frame.

Common Layer 2 systems include:

- switches, which learn source MAC addresses and forward frames toward associated
  ports;
- bridges, which connect and regulate traffic between link segments; and
- network interface cards (NICs), which handle both physical signaling and link-layer
  framing.

IEEE 802 divides these responsibilities into Logical Link Control (LLC) and Media
Access Control (MAC) sublayers. VLANs create separate logical broadcast domains over
the same switching infrastructure.

## Layer 3: Network

The Network layer handles logical addressing and forwards packets between networks.
IP addresses identify interfaces, subnetting divides address space, and routers choose
the next hop toward a destination.

This layer includes:

- IPv4 and IPv6 packets;
- IP addresses and prefixes;
- routing tables;
- routers and Layer 3 switches;
- ICMP control and diagnostic messages; and
- packet filters that use network-layer information.

Layers 1 and 2 can work while Layer 3 is broken. A host may have a valid local link but
the wrong address, prefix, gateway, or route to a remote network.

## Layer 4: Transport

The Transport layer connects processes on endpoint systems. Port numbers identify the
transport endpoints, allowing many applications to share one host and network
interface.

### TCP

The Transmission Control Protocol (TCP) is connection-oriented. Its three-way
handshake—SYN, SYN/ACK, ACK—establishes state before application data moves. TCP
numbers bytes, acknowledges data, retransmits data it infers was lost, controls flow,
and responds to congestion.

A successful TCP connection gives the application reliable, ordered delivery. Calling
this "error correction" hides the mechanism: checksums detect some corruption, while
acknowledgements and retransmission recover missing transport data.

### UDP

The User Datagram Protocol (UDP) sends independent datagrams without a TCP-style
connection. It does not guarantee delivery, ordering, or duplicate suppression. The
application adds reliability or sequencing as needed.

UDP is useful for a small transport mechanism, application-controlled timing, and
simple request/response exchanges. Streaming and games are common examples, but UDP
is not always faster and is not limited to real-time traffic.

Stateful firewalls and Layer 4 load balancers inspect transport information alongside
network addresses. That does not place the entire appliance neatly in one layer.

## Layer 5: Session

The Session layer covers the coordination of an ongoing exchange: opening the dialog,
maintaining state, adding checkpoints or synchronization, and ending it cleanly.

Modern Internet software often puts this work in applications, libraries, or
application protocols rather than a separate Layer 5 protocol. Login state, an RPC
conversation, and a recoverable long-running transfer can all behave like sessions
without mapping to a distinct component.

NetBIOS historically provided session services, but that does not make every protocol
using those services Layer 5. SMB, for example, is normally treated as an
application-layer file-sharing protocol.

## Layer 6: Presentation

The Presentation layer covers how data is represented so both applications interpret
it the same way. That includes:

- character encoding, such as ASCII and Unicode;
- serialization and data formats;
- compression and decompression; and
- encryption and decryption.

These functions change the representation of application data; they do not route
packets. Applications and libraries often perform them without exposing a separate
Presentation layer.

TLS provides encryption and peer authentication associated with this layer, but its
place in the Internet stack is concrete: applications use it above a reliable
transport such as TCP. SSL is TLS's obsolete predecessor, not the name of a current
protocol.

## Layer 7: Application

The Application layer gives software access to network services. It is closest to the
user's task, but it is not the graphical interface itself.

Common protocols include:

- HTTP and HTTPS for the web;
- DNS for name resolution;
- SMTP, POP3, and IMAP for email;
- FTP for file transfer; and
- SSH for encrypted remote access.

Systems at this layer understand a protocol's meaning and structure. A web application
firewall (WAF), for example, can inspect HTTP requests for patterns associated with SQL
injection or cross-site scripting. Intrusion prevention systems may combine that
content with information from lower layers.

## Encapsulation and decapsulation

As application data moves down the stack, each relevant layer adds what it needs for
delivery:

```text
Application data
  → TCP segment or UDP datagram
    → IP packet
      → link-layer frame
        → physical signals
```

The receiver reverses the process. Its interface reconstructs a frame from signals;
IP handles the packet; TCP or UDP identifies the destination endpoint; and the
application interprets the remaining data.

Terms such as segment, datagram, packet, frame, and bits tell us which information is
available at each stage. People also use "packet" more loosely in ordinary discussion,
so context matters.

Intermediate devices only process what their job requires. A switch normally reads
Layer 2 information to move a frame within a LAN. A router removes the incoming frame,
checks the Layer 3 packet, and builds new Layer 2 framing for the next link. An
application proxy can terminate one connection, create another, and act on Layer 7
information.

## OSI and the TCP/IP model

The Internet architecture is usually described with fewer layers:

| TCP/IP view | Approximate OSI relationship |
|---|---|
| Application | OSI Layers 5–7 |
| Transport | OSI Layer 4 |
| Internet | OSI Layer 3 |
| Link | OSI Layers 1–2 |

This is an approximation. OSI is useful for asking detailed questions about network
functions. TCP/IP more closely describes the protocol architecture the Internet
actually uses.

## Using layers to troubleshoot

Instead of stopping at "the network is down," ask smaller questions:

1. Physical: Is the interface connected and signaling?
2. Data Link: Can the host exchange frames on the local link? Is it in the expected
   VLAN?
3. Network: Does it have the correct address and route? Can packets reach the
   destination network?
4. Transport: Is the expected port reachable? Does the TCP handshake complete?
5. Application: Does the service understand the request and return a valid response?

Layers 5 and 6 can still guide checks of session state, encoding, encryption, and
certificates, even when the software does not expose them separately. There is no rule
that every investigation must begin at Layer 1. The model helps make the starting
point and its assumptions clear.

## Suggested practice: map one web request

Use a site you own or have permission to inspect, then map a `curl` request through
the model:

1. Record the application request and response with `curl -v`.
2. Identify the hostname resolution needed before the connection.
3. Identify the transport protocol and destination port.
4. Record the local address, gateway, and selected route.
5. Identify the local link interface and its MAC address.
6. Describe the physical or virtual medium carrying the first link.
7. Mark TLS, proxies, tunnels, and other components that cross layer boundaries.
8. Separate what the output proves from what you infer about infrastructure you cannot
   inspect.

For this site, the [hosting walkthrough](/blog/hosting-machinageist-dev) follows a real
request through DNS, Cloudflare's edge, a tunnel, Caddy, systemd, and the Rust
application. The OSI model classifies the functions; the captured commands provide
the evidence.

## Related pages

- [Network appliances](/learn/network-appliances) — switches, routers, firewalls,
  proxies, load balancers, storage, and wireless systems.
- [Network applications](/learn/network-applications) — content delivery networks and
  distributed request paths.
- [Network functions](/learn/network-functions) — tunnels, IPsec, QoS, and IP packet
  lifetime.

## Sources and further reading

I checked these networking notes against:

- [ISO/IEC 7498-1:1994 — Basic Reference Model](https://www.iso.org/standard/20269.html)
  — the formal OSI reference model.
- [RFC 1122: Requirements for Internet Hosts — Communication Layers](https://www.rfc-editor.org/rfc/rfc1122.txt)
  — the Internet host communication-layer model.
- [RFC 9293: Transmission Control Protocol](https://www.rfc-editor.org/rfc/rfc9293.txt)
  — current TCP specification.
- [RFC 768: User Datagram Protocol](https://www.rfc-editor.org/rfc/rfc768.txt) — UDP's
  minimal datagram service.

Protocol specifications define the actual behavior. The OSI model is a way to organize
and discuss it.
