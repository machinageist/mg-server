---
title: "IPv4 addressing"
date: 2026-08-07
summary: "How a 32-bit IPv4 address is built from binary octets, what public, private, link-local, and loopback addresses are for, and how masks, CIDR, and VLSM divide a network."
tags: [education, networking, addressing, subnetting, cidr, network-plus]
---

## Overview

An IPv4 address is a 32-bit number that identifies an interface on a network.
Everything else about addressing — private ranges, subnet masks, CIDR notation,
subnetting — is a consequence of that one fact plus a single question every
router has to answer: *is this destination on my network, or does it belong to
someone else?*

This page builds the address up from bits, then works through the ways those 32
bits get divided.

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

## Counting hosts and networks

Two formulas cover most subnetting arithmetic, and the thing to get right is
which part of the address each one counts.

**Hosts per network: 2ⁿ − 2**, where *n* is the number of **host** bits. The
subtraction removes two addresses that cannot be assigned to an interface:

- The **network address** — all host bits 0 — names the network itself.
- The **broadcast address** — all host bits 1 — reaches every host on it.

So a /24 has 8 host bits: 2⁸ = 256 addresses, 254 of them usable.

**Networks: 2ⁿ**, where *n* is the number of **network** bits available to
subdivide. Class A has 7 usable network bits after the leading bit that
identifies the class, giving 2⁷ = 128 networks, of which 126 are usable once 0
and 127 are removed. Class B has 14 (2¹⁴ = 16,384) and class C has 21
(2²¹ = 2,097,152).

## Subnet masks

A **subnet mask** marks the boundary between the network portion of an address
and the host portion. In binary it is always a run of 1s followed by a run of
0s, with no mixing:

```
255.255.255.0
11111111.11111111.11111111.00000000
 network   network   network    host
```

A host determines whether a destination is local by applying a bitwise **AND**
between an address and its own mask. AND returns 1 only when both inputs are 1,
which produces a simple result at the octet level: an octet of 255 in the mask
passes the address octet through unchanged, and an octet of 0 zeroes it out.

```
address   10.10.10.10
mask      255.255.255.0
AND ---------------------
network   10.10.10.0
```

The host does this twice — once for its own address, once for the destination.
Matching results mean the destination is on the same network and can be reached
directly. Differing results mean it is somewhere else, and the packet goes to
the default gateway. This comparison, repeated at every hop, is most of what
routing is.

## CIDR

**Classless Inter-Domain Routing** replaced the class system with an explicit
prefix length. A CIDR address carries its own boundary:

```
10.10.10.10/24
```

The `/24` says the first 24 bits are network, which is the same information as
`255.255.255.0` in fewer characters and without reference to any class. Any
prefix length is legal, so a network can be sized to what it actually needs
instead of to the nearest class.

CIDR also allows **supernetting** — expressing several adjacent networks as one
larger route, which keeps internet routing tables smaller than the number of
allocations would otherwise require. Aggregation only works when the blocks are
contiguous *and* correctly aligned. `10.0.0.0/16` and `10.1.0.0/16` combine into
`10.0.0.0/15`, because dropping one bit from the prefix covers exactly those
two. `10.1.0.0/16` and `10.2.0.0/16` cannot be combined at all: they are
adjacent, but `10.0.0.0/15` covers 10.0 and 10.1, not 10.1 and 10.2. The
aggregate has to start on a boundary that is a multiple of its own size.

Reading a prefix length as a mask and back again is a daily task in a modern
network, and it is the notation you will meet in routing tables, firewall rules,
and cloud console forms.

## Variable length subnet masking

**VLSM** means subnetting one network into pieces of different sizes to match
what each piece needs, rather than splitting it evenly. Take `10.10.10.0/24` —
256 addresses — and three segments that need 45, 25, and 10 hosts.

Allocate largest first, so each subnet starts on a valid boundary:

| Segment | Need | Prefix | Range | Usable |
|---|---:|---|---|---:|
| A | 45 | 10.10.10.0/26 | .0 – .63 | 62 |
| B | 25 | 10.10.10.64/27 | .64 – .95 | 30 |
| C | 10 | 10.10.10.96/28 | .96 – .111 | 14 |

Each block begins exactly where the previous one ended, and .112 – .255 stays
free for later. Allocating smallest first would have left the larger blocks
without an aligned boundary to start on — the same alignment rule that governs
supernetting, applied in the other direction.

Beyond conserving addresses, subnetting is a security and operations tool.
Separate subnets give a natural place to enforce policy between systems that
have no reason to talk to each other, and they bound broadcast traffic.

## Suggested practice: read and verify your own network

All of this is visible on a machine you already own, with tools that ship with
most Linux systems. Nothing here needs root or changes any configuration.

1. Run `ip -4 addr show` and find your own address and prefix length. Run
   `ip route` to see the default gateway. Note whether your address falls in one
   of the RFC 1918 ranges.
2. Convert one octet of your address to binary by hand, then check yourself:
   `printf '%d\n' 0b10101000` converts the other direction.
3. Do the AND by hand for your address and mask to get your network address.
   Verify with `ipcalc <address>/<prefix>` or `sipcalc`, and compare its host
   count against 2ⁿ − 2.
4. Run `ping -c 3 127.0.0.1` and confirm it succeeds with the network cable
   unplugged. Note that `ping` uses ICMP, not TCP — it tests reachability, not
   whether any service is listening.
5. Run `ss -tln` and compare services bound to `127.0.0.1` with those bound to
   `0.0.0.0`. The first group is reachable only from the machine itself; the
   second is reachable from the network.
6. On paper, split your own /24 with VLSM for three segments of your choosing.
   Check each boundary with `ipcalc` before trusting it.

## Related pages

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

This page was edited from my networking reading notes and checked against:

- [RFC 791: Internet Protocol](https://www.rfc-editor.org/rfc/rfc791.txt) — the
  32-bit address, the original class definitions, and the header that carries
  them.
- [RFC 1918: Address Allocation for Private Internets](https://www.rfc-editor.org/rfc/rfc1918.txt)
  — the three private ranges and how routers are expected to treat them.
- [RFC 3927: Dynamic Configuration of IPv4 Link-Local Addresses](https://www.rfc-editor.org/rfc/rfc3927.txt)
  — the 169.254.0.0/16 fallback, including the reserved first and last /24.
- [RFC 4632: Classless Inter-Domain Routing](https://www.rfc-editor.org/rfc/rfc4632.txt)
  — CIDR as the current standard, and the aggregation rules supernetting follows.
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
