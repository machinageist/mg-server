---
title: "Subnetting, CIDR, and VLSM"
date: 2026-08-14
summary: "Counting hosts and networks, what a subnet mask does bit by bit, prefix notation and supernetting, and sizing subnets to what each segment actually needs."
tags: [education, networking, subnetting, cidr, vlsm, routing]
---

## Overview

An [IPv4 address](/learn/ipv4-addressing) is 32 bits. Subnetting is the practice
of deciding where to cut those 32 bits — how many belong to the network and how
many are left to identify hosts on it.

That single decision answers the question every router asks about every packet:
*is this destination on my network, or does it belong to someone else?* A host
answers it the same way, which is why a wrong mask breaks connectivity in ways
that look like a broken cable.

This page covers the arithmetic, the mask, the notation that replaced address
classes, and the technique for dividing a network into pieces of different sizes.

## Counting hosts and networks

Two formulas cover most subnetting arithmetic, and the thing to get right is
which part of the address each one counts.

**Hosts per network: 2ⁿ − 2**, where *n* is the number of **host** bits. The
subtraction removes two addresses that cannot be assigned to an interface:

- The **network address** — all host bits 0 — names the network itself.
- The **broadcast address** — all host bits 1 — reaches every host on it.

So a /24 has 8 host bits: 2⁸ = 256 addresses, 254 of them usable.

**Networks: 2ⁿ**, where *n* is the number of **network** bits available to
subdivide. Under the old [class system](/learn/ipv4-addressing#classful-addressing),
class A had 7 usable network bits after the leading bit that identifies the
class, giving 2⁷ = 128 networks, of which 126 are usable once 0 and 127 are
removed. Class B had 14 (2¹⁴ = 16,384) and class C had 21 (2²¹ = 2,097,152).

Mixing the two formulas up is the easiest arithmetic mistake to make here. Hosts
count host bits and subtract two; networks count network bits and subtract
nothing.

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

## Suggested practice: verify a subnet by hand, then check yourself

All of this is visible on a machine you already own, with tools that ship with
most Linux systems. Nothing here needs root or changes any configuration.

1. Run `ip -4 addr show` and find your own address and prefix length. Convert
   the prefix to a dotted mask by hand before checking it.
2. Do the AND by hand for your address and mask to get your network address.
   Verify with `ipcalc <address>/<prefix>` or `sipcalc`, and compare its host
   count against 2ⁿ − 2.
3. Work out your network's broadcast address, then confirm it with `ipcalc`.
   Ping it and see how many hosts answer — some systems ignore broadcast pings,
   so a quiet result is not proof of an empty network.
4. On paper, split your own /24 with VLSM for three segments of your choosing.
   Check every boundary with `ipcalc` before trusting it, and deliberately try
   allocating smallest-first to see where the alignment fails.
5. Read `ip route` and identify which entries are directly connected and which
   go through a gateway. Match each prefix against the AND test above.
6. Take two adjacent prefixes from your own network and decide whether they
   aggregate. Most pairs do not, and working out why is the point.

## Related pages

- [IPv4 addressing](/learn/ipv4-addressing) — the 32-bit address these masks
  divide, and the public, private, and reserved ranges.
- [IPv6 addressing](/learn/ipv6-addressing) — keeps prefix notation and drops
  the scarcity that made subnetting an exercise in conservation.
- [Network appliances](/learn/network-appliances) — the routers and gateways
  that act on the boundary a mask defines.
- [Network functions](/learn/network-functions) — what happens to a packet once
  the AND test says it belongs to someone else.
- [Moving my homelab management network first](/blog/management-layer-first-network-migration)
  — a subnet migration I performed, including the outage caused by systems still
  referring to the old addresses.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [RFC 4632: Classless Inter-Domain Routing](https://www.rfc-editor.org/rfc/rfc4632.txt)
  — CIDR as the current standard, and the aggregation rules supernetting follows.
- [RFC 1878: Variable Length Subnet Table for IPv4](https://www.rfc-editor.org/rfc/rfc1878.txt)
  — the prefix-to-mask table, useful as a check on hand arithmetic.
- [RFC 950: Internet Standard Subnetting Procedure](https://www.rfc-editor.org/rfc/rfc950.txt)
  — where the subnet mask was introduced, and why.
- [RFC 6890: Special-Purpose IP Address Registries](https://www.rfc-editor.org/rfc/rfc6890.txt)
  — the reserved blocks that must not be handed out when planning a range.

Subnetting arithmetic rewards being done by hand a few dozen times and then
never again — `ipcalc` exists, and using it in production is not cheating. The
reason to learn the manual method is that it makes a wrong mask recognizable on
sight, which is the failure you will actually be asked to diagnose.
