---
title: "IPv4 addressing"
date: 2026-08-07
summary: "How a 32-bit IPv4 address is built from binary octets, and what the public, private, link-local, loopback, and historical class ranges are each for."
tags: [education, networking, addressing, subnetting, cidr]
---

## Overview

An IPv4 address is a 32-bit number that identifies an interface on a network.
Everything else about addressing — private ranges, subnet masks, CIDR notation,
subnetting — is a consequence of that one fact plus a single question every
router has to answer: *is this destination on my network, or does it belong to
someone else?*

This page builds the address up from bits and covers the kinds of address you
meet on a real network. Dividing those bits into subnets is the other half of
the story, and it has its own page:
[subnetting, CIDR, and VLSM](/learn/subnetting).

## Binary and the shape of an address

Network addressing rests on binary. A single binary digit is a **bit**, and
eight bits make one **byte**. Because each of those eight positions is either 0
or 1, a byte can represent 256 distinct values — 0 through 255 in ordinary
base 10.

The positions have fixed place values, doubling from right to left:

| 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 0 | 1 | 0 | 1 | 1 | 0 | 1 | 0 |

Add the place values wherever there is a 1: 64 + 16 + 8 + 2 = **90**.

An IPv4 address is four of those bytes — 32 bits total — written as four base-10
numbers separated by dots. Each byte in this context is called an **octet**:

```
192.168.1.10
11000000.10101000.00000001.00001010
```

Thirty-two bits gives 2³² addresses, or 4,294,967,296 — a number that seemed
generous in 1981 and is the reason IPv6 exists. Converting an octet between
binary and decimal in your head is worth practicing, because subnet masks only
make sense in binary.

## Public and private addresses

Addresses fall into two broad groups by where they are meaningful.

**Public addresses** are globally routable. Any host on the internet can, in
principle, address them. They are allocated through regional registries and
usually reach you leased from an ISP.

**Private addresses** are meaningful only inside one local network. Routers on
the public internet drop them, which is exactly the point: everyone can reuse
the same private ranges without colliding, because those addresses never appear
on the open internet. RFC 1918 reserves three blocks:

| Block | Range | Prefix |
|---|---|---|
| 24-bit | 10.0.0.0 – 10.255.255.255 | 10.0.0.0/8 |
| 20-bit | 172.16.0.0 – 172.31.255.255 | 172.16.0.0/12 |
| 16-bit | 192.168.0.0 – 192.168.255.255 | 192.168.0.0/16 |

A home or office network typically uses one of these internally and reaches the
internet through a gateway router that holds a public address and translates on
the private network's behalf. That translation step is why a private host can
start a conversation with a public server but a public host cannot start one
with a private host.

## Link-local addresses when DHCP fails

To communicate on a local network at all, an interface needs an address and a
mask. A default gateway is optional for local traffic and required for anything
beyond the local network.

Most hosts get all three from DHCP. When DHCP does not answer, an interface can
fall back to **link-local addressing** — the mechanism Microsoft named APIPA
(Automatic Private IP Addressing) and RFC 3927 standardized. The host picks a
random address in `169.254.0.0/16`, checks whether anything else on the segment
is already using it, and picks again if so. RFC 3927 reserves the first and last
/24 of that block, so the usable range is 169.254.1.0 – 169.254.254.255.

Link-local addressing has no gateway and no routing. It lets hosts on the same
segment talk to each other and nothing more. It is a fallback, not a
configuration, and the host keeps asking for DHCP in the background.

Practically, a 169.254 address is a diagnostic signal: the interface is up at
layer 2, and DHCP is not answering.

## Loopback

`127.0.0.0/8` is reserved for **loopback** — traffic a host sends to itself.
`127.0.0.1`, conventionally named `localhost`, is the address you almost always
see. Packets sent there never reach a wire; the network stack turns them around
internally.

That makes loopback useful in two ways. `ping 127.0.0.1` exercises the local
IP stack without depending on any cable, switch, or router, so a failure points
at local configuration rather than the network. And a service bound to
`127.0.0.1` is reachable only from the machine it runs on, which is a genuine
security boundary — this site's own server binds loopback by default and is
published through a reverse proxy rather than by listening on a public
interface.

## Classful addressing

Early IPv4 divided the address space into fixed **classes**, distinguished by
the leading bits and therefore by the value of the first octet. The class
determined how much of the address was network and how much was host.

| Class | First octet | Default mask | Networks | Hosts per network |
|---|---:|---|---:|---:|
| A | 1–126 | 255.0.0.0 (/8) | 126 | 16,777,214 |
| B | 128–191 | 255.255.0.0 (/16) | 16,384 | 65,534 |
| C | 192–223 | 255.255.255.0 (/24) | 2,097,152 | 254 |
| D | 224–239 | — | *multicast groups* | — |
| E | 240–255 | — | *reserved, experimental* | — |

127 is missing from the class A range because `127.0.0.0/8` is loopback, and 0
is excluded because `0.0.0.0/8` is reserved. Class D is the multicast range
described in [network traffic types](/learn/traffic-types), and class E has
never been allocated for general use.

Classes A through C each contain one of the RFC 1918 private blocks, which is
where those oddly-shaped private ranges come from: `10.0.0.0/8` is one class A
network, `172.16.0.0/12` is sixteen class B networks, and `192.168.0.0/16` is
256 class C networks.

Classful addressing was abandoned in the 1990s. Its granularity was hopeless: an
organization needing 300 addresses had to choose between a class C that was too
small and a class B that wasted 65,000. Nothing in a modern network uses it. It
survives in documentation and exams because it explains the default masks and
the shape of the private ranges.

## Dividing the address

Everything above concerns the shape of one address. Splitting a network into
smaller ones — counting usable hosts, reading a subnet mask bit by bit, prefix
notation, and sizing each subnet to what it actually needs — is covered in
[subnetting, CIDR, and VLSM](/learn/subnetting).

## Suggested practice: read and verify your own network

All of this is visible on a machine you already own, with tools that ship with
most Linux systems. Nothing here needs root or changes any configuration.

1. Run `ip -4 addr show` and find your own address and prefix length. Run
   `ip route` to see the default gateway. Note whether your address falls in one
   of the RFC 1918 ranges.
2. Convert one octet of your address to binary by hand, then check yourself:
   `printf '%d\n' 0b10101000` converts the other direction.
3. Run `ping -c 3 127.0.0.1` and confirm it succeeds with the network cable
   unplugged. Note that `ping` uses ICMP, not TCP — it tests reachability, not
   whether any service is listening.
4. Run `ss -tln` and compare services bound to `127.0.0.1` with those bound to
   `0.0.0.0`. The first group is reachable only from the machine itself; the
   second is reachable from the network.
5. Disconnect from your network, wait for DHCP to give up, and check `ip -4 addr
   show` for a `169.254` address. Reconnect and watch it be replaced.

The mask arithmetic that goes with this — the AND test, host counts, and
splitting a range — is practised on
[subnetting, CIDR, and VLSM](/learn/subnetting#suggested-practice-verify-a-subnet-by-hand-then-check-yourself).

## Related pages

- [Subnetting, CIDR, and VLSM](/learn/subnetting) — how these 32 bits get
  divided, and the mask that marks the boundary.
- [IPv6 addressing](/learn/ipv6-addressing) — the successor protocol, which
  keeps prefix notation and drops nearly everything else on this page.
- [The OSI model](/learn/osi-model) — where IP addressing sits at layer 3, and
  how it relates to the MAC addressing beneath it.
- [Network traffic types](/learn/traffic-types) — unicast, broadcast, and the
  multicast range that class D reserves.
- [Network appliances](/learn/network-appliances) — the routers and gateways
  that act on the network/host boundary this page describes.
- [Network functions](/learn/network-functions) — routing, tunneling, and what
  happens to a packet once it leaves its own subnet.
- [Moving my homelab management network first](/blog/management-layer-first-network-migration)
  — a subnet migration I performed, including the outage caused by systems still
  referring to the old addresses.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [RFC 791: Internet Protocol](https://www.rfc-editor.org/rfc/rfc791.txt) — the
  32-bit address, the original class definitions, and the header that carries
  them.
- [RFC 1918: Address Allocation for Private Internets](https://www.rfc-editor.org/rfc/rfc1918.txt)
  — the three private ranges and how routers are expected to treat them.
- [RFC 3927: Dynamic Configuration of IPv4 Link-Local Addresses](https://www.rfc-editor.org/rfc/rfc3927.txt)
  — the 169.254.0.0/16 fallback, including the reserved first and last /24.
- [RFC 6890: Special-Purpose IP Address Registries](https://www.rfc-editor.org/rfc/rfc6890.txt)
  — one authoritative list of every reserved IPv4 block, loopback included.

IPv4 is defined by the IETF through these RFCs, not by the IEEE — IEEE standards
govern the layer beneath, such as Ethernet and Wi-Fi.

Two limits are worth stating. The class table is historical: it explains where
default masks and private ranges came from, but no current router makes
forwarding decisions from it. And everything here is IPv4 only. IPv6 keeps the
prefix-length notation and discards nearly all of the rest — 128-bit addresses,
no broadcast, and no equivalent of the address scarcity that made subnetting an
exercise in conservation.
