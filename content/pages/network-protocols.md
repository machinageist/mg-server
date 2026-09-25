---
title: "Network protocols and ports"
date: 2026-07-24
summary: "Common application protocols, the port numbers they rely on, and how port ranges are allocated."
tags: [education, networking, protocols, ports]
---

## Overview

A **protocol** is a standard set of rules systems use to exchange data. It
defines details such as formatting, addressing, flow control, and error
detection. A **port number** identifies a communication endpoint on one host.
Ports let several services use the same IP address at the same time.

This page collects the application protocols and ports that come up often in
networking, along with the ranges IANA uses to organize port assignments. [The
OSI model](/learn/osi-model) covers the transport details underneath them,
including TCP's handshake and UDP's connectionless delivery.

## Port number ranges

The Internet Assigned Numbers Authority (IANA) divides the 16-bit port space
into three ranges:

| Range | Name | Typical use |
|---:|---|---|
| 0–1023 | Well-known ports | Long-established system services, e.g. 80 (HTTP), 443 (HTTPS), 25 (SMTP) |
| 1024–49151 | Registered ports | Vendor and application-specific services, e.g. 3389 (RDP), 3306 (MySQL) |
| 49152–65535 | Dynamic/private ports | Short-lived client-side ports chosen for the life of one connection |

A server usually listens on a well-known or registered port. A client normally
chooses an ephemeral port from the dynamic range for the connection. An
administrator can still run a service on an unusual port; these ranges are a
registration convention, not a technical restriction.

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

Some study-note shortcuts are easy to misread:

- **SMTP is not itself a "secure" protocol.** Port 25 carries server-to-server
  relay traffic. Mail clients normally submit on port 587 with STARTTLS or port
  465 with implicit TLS. The secure part is the transport wrapper, not SMTP by
  itself.
- **NTP runs over UDP, not TCP.** Its request/response exchange does not need a
  connection-oriented transport.
- **Syslog traditionally uses UDP 514.** TCP and TLS-protected versions also
  exist, but they are not the historical default.
- **TLS and SQL do not each have one port.** TLS protects many protocols on
  different ports. Port 443 belongs to HTTPS, which uses TLS. SQL is a query
  language, not a network protocol: TCP 1433 is Microsoft SQL Server's default,
  while MySQL defaults to 3306 and PostgreSQL to 5432.
- **Spanning Tree Protocol (STP) has no TCP port.** It runs directly over
  Ethernet at Layer 2. The number 32768 is STP's *default bridge priority*, not
  a port number.

## Network-layer and tunneling protocols

- **Internet Control Message Protocol (ICMP)** carries diagnostic and error
  messages for IP rather than application data. `ping` and
  `traceroute`/`tracepath` use ICMP echo and time-exceeded messages.
- **Generic Routing Encapsulation (GRE)** wraps one packet inside another to
  make a point-to-point tunnel. It can carry non-IP traffic or one IP version
  across another. GRE began as a Cisco protocol and is now an open standard. It
  provides **no encryption or authentication** by itself, so it is often paired
  with IPsec when confidentiality is needed.
- **IPsec** secures IP traffic. The [network
  functions](/learn/network-functions) page covers AH, ESP, and IKE.

The familiar 1500-byte figure is the Ethernet MTU, not a universal maximum
packet size. Other link types and jumbo-frame configurations use different
values. IPv6 requires a minimum link MTU of 1280 bytes.

## Suggested practice: read ports off real traffic

On a network you own or are authorized to inspect:

1. Start a capture with `tcpdump` or Wireshark.
2. Run a DNS lookup (`dig` or `nslookup`) and load an HTTPS page.
3. For each exchange, find the server's destination port and the client's
   ephemeral source port.
4. Check whether the server port is in the well-known or registered range and
   whether the client port is in the dynamic range.
5. Find an ICMP message; an echo request or reply from `ping` is enough. Note
   that it has no port numbers because ICMP sits below the transport layer's
   port addressing.

## Exam key points

Where this page's material shows up in the exam objectives, and what each exam
expects beyond it.

### CCNA 200-301

In v1.1 these services are spread across 4.2 (NTP), 4.3 (DHCP and DNS), 4.4
(SNMP), 4.5 (syslog), 4.6 (DHCP client and relay), 4.8 (SSH), and 4.9 (TFTP and
FTP). In v2.0, which replaces it in February 2027, DHCPv4 moves to 1.7 as
troubleshooting, DNS records to 4.4, secure file transfer with SFTP and SCP to
4.2, and SNMP and syslog to 5.4 and 5.6.

- DHCP relay: `ip helper-address` on the interface facing the clients forwards
  their broadcasts to a server in another subnet.
- SSH on IOS needs a hostname, a domain name, RSA keys from
  `crypto key generate rsa`, a local user, and `transport input ssh` on the vty
  lines.
- An NTP client is `ntp server <address>`, checked with `show ntp associations`.
- Syslog severity runs from 0 to 7: emergency, alert, critical, error, warning,
  notification, informational, and debugging.
- SNMP managers poll agents on UDP 161, and agents send traps to UDP 162. SNMPv3
  adds authentication and encryption.
- v2.0 adds DNS record types (A, AAAA, CNAME, MX, NS, and PTR) as a
  troubleshooting objective.

### Network+ N10-009

Objective 1.4 lists specific protocols with their ports, plus the IP protocol
types ICMP, TCP, UDP, GRE, and IPsec.

- The listed ports: FTP 20/21, SSH and SFTP 22, Telnet 23, SMTP 25, DNS 53, DHCP
  67/68, TFTP 69, HTTP 80, NTP 123, SNMP 161/162, LDAP 389, HTTPS 443, SMB 445,
  syslog 514, SMTPS 587, LDAPS 636, SQL Server 1433, RDP 3389, and SIP 5060/5061.
- The objectives list SMTPS as 587. Port 587 is really mail submission with
  STARTTLS, and 465 is SMTP over implicit TLS, but 587 is the answer the exam
  expects.
- Objective 3.4 adds DHCP details (scopes, reservations, lease time, options,
  relay, and exclusions) and DNS record and zone types.

## Related pages

- [The OSI model](/learn/osi-model) — where TCP, UDP, and port-based addressing
  sit in the layered model.
- [Network functions](/learn/network-functions) — IPsec, IKE, and tunneling
  built on top of these protocols.
- [Network appliances](/learn/network-appliances) — firewalls and load
  balancers that make decisions using this same port and protocol information.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [IANA Service Name and Transport Protocol Port Number Registry](https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml)
  — the authoritative port assignment list.
- [RFC 792: Internet Control Message Protocol](https://www.rfc-editor.org/rfc/rfc792.txt)
  — ICMP message types.
- [RFC 2784: Generic Routing Encapsulation (GRE)](https://www.rfc-editor.org/rfc/rfc2784.txt)
  — GRE as a standards-track tunneling protocol.
- [RFC 6409: Message Submission for Mail](https://www.rfc-editor.org/rfc/rfc6409.txt)
  — why mail submission is distinct from server-to-server relay.

Port assignments can be changed on a real system. IANA's registry and the RFCs
describe the convention, not a technical guarantee.
