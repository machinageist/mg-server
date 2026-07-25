---
title: "Network protocols and ports"
date: 2026-07-24
summary: "Common application protocols, the port numbers they rely on, and how port ranges are allocated."
tags: [education, networking, protocols, ports, network-plus]
---

## Overview

A **protocol** is a standardized set of rules that lets independent systems
exchange data predictably: how information is formatted, how peers are
addressed, how flow is regulated, and how errors are detected. A **port
number** identifies one of many possible communication endpoints on a single
host, so a server and its neighbors can run several services on the same IP
address at once.

This page catalogs the application protocols and port numbers that come up
constantly in networking work, and the ranges IANA uses to keep port numbers
from colliding. The transport mechanics underneath these protocols — TCP's
handshake and UDP's connectionless delivery — are covered on
[the OSI model](/learn/osi-model) page rather than repeated here.

## Port number ranges

The Internet Assigned Numbers Authority (IANA) divides the 16-bit port space
into three ranges:

| Range | Name | Typical use |
|---:|---|---|
| 0–1023 | Well-known ports | Long-established system services, e.g. 80 (HTTP), 443 (HTTPS), 25 (SMTP) |
| 1024–49151 | Registered ports | Vendor and application-specific services, e.g. 3389 (RDP), 3306 (MySQL) |
| 49152–65535 | Dynamic/private ports | Short-lived client-side ports chosen for the life of one connection |

A listening service usually binds a well-known or registered port; the client
side of a connection typically picks an ephemeral port from the dynamic range
for that one exchange. Nothing prevents an administrator from running a
service on an unusual port — these ranges are a registration convention, not
a technical restriction.

## Common application protocols

| Protocol | Purpose | Transport / port |
|---|---|---:|
| FTP | Unencrypted file transfer | TCP 20 (data), 21 (control) |
| SFTP | File transfer over SSH | TCP 22 |
| SSH | Encrypted remote administration | TCP 22 |
| Telnet | Unencrypted remote administration | TCP 23 |
| SMTP | Mail transfer between servers | TCP 25 (relay), 587 (submission) |
| DNS | Name resolution queries | UDP 53 |
| DNS | Zone transfers and oversized responses | TCP 53 |
| DHCP | Automatic IP address assignment | UDP 67 (server), 68 (client) |
| TFTP | Minimal, connectionless file transfer | UDP 69 |
| HTTP | Unencrypted web traffic | TCP 80 |
| NTP | Time synchronization | UDP 123 |
| SNMP | Device status, polling and traps | UDP 161 (poll), 162 (trap) |
| LDAP | Directory access, e.g. Active Directory | TCP 389 |
| HTTPS | Web traffic over TLS | TCP 443 |
| SMB | Windows file and print sharing | TCP 445 |
| LDAPS | Directory access over TLS | TCP 636 |
| Microsoft SQL Server | Database queries | TCP 1433 |
| RDP | Remote desktop for Windows systems | TCP 3389 |
| SIP | Session setup for voice/video calls | UDP/TCP 5060, TCP 5061 (TLS) |

A few corrections against common study-note shorthand:

- **SMTP is not itself a "secure" protocol.** Port 25 carries plain
  server-to-server relay traffic; mail submission from a client normally uses
  port 587 with STARTTLS, or port 465 for implicit TLS. "Secure SMTP"
  describes the transport wrapper, not the base protocol.
- **NTP runs over UDP, not TCP.** A stateless request/response exchange suits
  time synchronization better than a connection-oriented one.
- **Syslog's traditional transport is UDP 514.** TCP and TLS-protected
  variants exist but are not the historical default.
- **TLS and SQL are not single ports.** TLS is a transport-security layer used
  by many protocols at many ports — 443 belongs to HTTPS, which happens to use
  TLS, not to TLS itself. "SQL" is a query language, not a network protocol;
  TCP 1433 belongs specifically to Microsoft SQL Server — MySQL defaults to
  3306 and PostgreSQL to 5432.
- **Spanning Tree Protocol (STP) has no TCP port.** STP is a Layer 2 protocol
  that runs directly over Ethernet, not a TCP/IP service. 32768 is STP's
  *default bridge priority* value, not a port number — an easy mix-up when
  both concepts get called "priority."

## Network-layer and tunneling protocols

- **Internet Control Message Protocol (ICMP)** carries diagnostic and error
  messages for IP itself rather than application data. `ping` and
  `traceroute`/`tracepath` are built on ICMP echo and time-exceeded messages.
- **Generic Routing Encapsulation (GRE)** wraps one packet inside another to
  build a point-to-point tunnel, including tunnels that carry non-IP or
  mismatched IP versions across an IP network. GRE originated at Cisco but is
  now an open standard. On its own, GRE provides **no encryption or
  authentication**; it is frequently paired with IPsec when confidentiality is
  required.
- **IPsec** secures IP traffic and is covered in detail — AH, ESP, and IKE —
  on the [network functions](/learn/network-functions) page.

A commonly repeated figure worth qualifying: the "maximum packet size" most
people learn is the 1500-byte Ethernet MTU, not a universal ceiling. Other
link types and jumbo-frame configurations use different values, and IPv6
requires a minimum link MTU of 1280 bytes.

## Suggested practice: read ports off real traffic

On a network you own or are authorized to inspect:

1. Start a capture with `tcpdump` or Wireshark.
2. Run a DNS lookup (`dig` or `nslookup`) and load an HTTPS page.
3. For each exchange, identify the destination port on the server side and the
   ephemeral port chosen on the client side.
4. Confirm the destination port falls in the well-known range and the
   client's source port falls in the dynamic range.
5. Find one ICMP message (an echo request/reply from `ping` is enough) and
   note that it has no port numbers at all — a reminder that ICMP sits below
   the transport layer's addressing scheme.

## Related pages

- [The OSI model](/learn/osi-model) — where TCP, UDP, and port-based addressing
  sit in the layered model.
- [Network functions](/learn/network-functions) — IPsec, IKE, and tunneling
  built on top of these protocols.
- [Network appliances](/learn/network-appliances) — firewalls and load
  balancers that make decisions using this same port and protocol
  information.

## Sources and further reading

This page was edited from my networking reading notes and checked against:

- [IANA Service Name and Transport Protocol Port Number Registry](https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml)
  — the authoritative port assignment list.
- [RFC 792: Internet Control Message Protocol](https://www.rfc-editor.org/rfc/rfc792.txt)
  — ICMP message types.
- [RFC 2784: Generic Routing Encapsulation (GRE)](https://www.rfc-editor.org/rfc/rfc2784.txt)
  — GRE as a standards-track tunneling protocol.
- [RFC 6409: Message Submission for Mail](https://www.rfc-editor.org/rfc/rfc6409.txt)
  — why mail submission is distinct from server-to-server relay.

Port assignments and protocol behavior can be reconfigured on any real system;
IANA's registry and the relevant RFCs describe the convention, not a technical
guarantee.
