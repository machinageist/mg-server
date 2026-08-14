---
title: "IPv6 addressing"
date: 2026-08-14
summary: "128-bit addresses, hextets and zero compression, the address types you actually meet on a link, and the mechanisms that let IPv6 and IPv4 share a network."
tags: [education, networking, addressing, ipv6]
---

## Overview

IPv4 offers about 4.3 billion addresses. That was generous in 1981 and is not
generous now, and the workarounds — NAT, private ranges, careful reclamation —
are the reason [IPv4 addressing](/learn/ipv4-addressing) has as many special
cases as it does. **Address exhaustion** is the problem IPv6 was designed to
end.

IPv6 uses 128-bit addresses. That is 2^128, or roughly 3.4 × 10^38 — enough that
address conservation stops being a design constraint. The larger space is the
headline, but the more useful change for anyone operating a network is that IPv6
removed broadcast, replaced ARP, and made autoconfiguration a normal part of the
protocol rather than a separate service.

This page covers the address format, the address types you will actually see on
an interface, and how IPv6 coexists with the IPv4 network it has not replaced.

## The shape of an address

An IPv4 address is four 8-bit **octets** in decimal, separated by dots. An IPv6
address is eight 16-bit **hextets** in hexadecimal, separated by colons:

```text
2001:0db8:0000:0000:0000:ff00:0042:8329
```

Two conventions make that readable, both defined in RFC 5952:

- Leading zeros within a hextet are dropped: `0db8` becomes `db8`, `0042`
  becomes `42`.
- One run of consecutive all-zero hextets is replaced with `::`.

Applying both gives:

```text
2001:db8::ff00:42:8329
```

The `::` may appear only once in an address. If it appeared twice there would be
no way to work out how many zero hextets belonged to each run. RFC 5952 also
asks for lowercase hex and for `::` to shorten the *longest* zero run, so that
the same address is written the same way in a log, a config file, and a firewall
rule.

## Network and interface portions

An IPv6 address is normally split down the middle: the first 64 bits identify
the network, and the last 64 bits identify an interface on it. The network half
is structured further, in the arrangement described by RFC 3587:

| Bits | Field | Purpose |
|---:|---|---|
| 3 | Format prefix | Marks the address type — `001` for global unicast |
| 45 | Global routing prefix | The block routed to a site |
| 16 | Subnet ID | Which subnet within that site |
| 64 | Interface identifier | Which interface on that subnet |

The first 48 bits together are what a site receives from its provider, which is
why a `/48` to a site and a `/64` per subnet are the common allocations. Sixteen
subnet bits give a single site 65,536 subnets — the point is that subnetting in
IPv6 is a question of organization rather than scarcity.

The `/64` boundary is a strong convention rather than a property of the address
format. Stateless autoconfiguration requires it, and most tooling assumes it, so
narrower prefixes on a normal LAN cause more problems than they solve.

### The interface identifier

The last 64 bits are the **interface identifier**. It is often described as an
**EUI-64**, and EUI-64 is one way to build it: take the 48-bit MAC address,
insert `fffe` in the middle, and flip the seventh bit. That derivation is worth
knowing because it explains addresses that look like a MAC address wearing a
disguise, and because it embeds a hardware serial number in every packet a host
sends.

That privacy problem is why it is no longer the default. Current systems
generate interface identifiers that are stable per network but not derived from
hardware (RFC 7217), and add temporary addresses that rotate for outbound
connections (RFC 8981). A modern Linux host commonly holds several IPv6
addresses on one interface at once, and this is why.

## Address types

IPv6 defines several address types, distinguished by prefix. These are the ones
that appear on a working interface:

| Type | Prefix | What it does |
|---|---|---|
| Global unicast | `2000::/3` | Publicly routable, the IPv6 equivalent of a public IPv4 address |
| Unique local | `fc00::/7` | Site-local, not routed on the internet — the analogue of RFC 1918 |
| Link-local unicast | `fe80::/10` | Valid only on one link; every interface has one |
| Multicast | `ff00::/8` | Delivery to a group of interfaces |
| Link-local multicast | `ff02::/16` | Multicast scoped to the local link |
| Solicited-node multicast | `ff02::1:ff00:0/104` | Neighbor discovery for one specific address |
| Unspecified | `::/128` | "No address yet" — a source address during configuration |
| Loopback | `::1/128` | The host talking to itself |

**Global unicast** addresses are routable across the internet. The `2000::/3`
prefix covers everything from `2000::` to `3fff:ffff:ffff:ffff:ffff:ffff:ffff:ffff`.

**Unique local addresses (ULAs)** fill the role RFC 1918 fills in IPv4. In
practice the usable half is `fd00::/8`, where a site generates a random 40-bit
global ID so that two networks merging later are unlikely to collide. ULAs are
not a security boundary — they are simply not routed off-site.

**Link-local addresses** are the ones that make IPv6 feel different. Every IPv6
interface configures one automatically, and it works before DHCP, before router
advertisements, and before any routing exists. Neighbor discovery and routing
protocols use link-local addresses for their own traffic. Because the same
`fe80::/10` prefix exists on every link, a link-local address is ambiguous
without a **zone index** naming the interface: `fe80::1%eth0`.

**Multicast** replaces IPv4 broadcast entirely. There is no broadcast address in
IPv6. Where IPv4 would shout at every host on the segment, IPv6 sends to a
group: `ff02::1` is all nodes on the link, `ff02::2` is all routers.

**Solicited-node multicast** is the mechanism that makes this cheap. Each
unicast address has a corresponding solicited-node group derived from its last
24 bits. A host asking "who has this address?" sends to that group instead of to
every host, so only the few interfaces sharing those 24 bits process the
request. This is the structural improvement over ARP, which every host on the
segment must inspect.

## Neighbor discovery

IPv6 has no ARP. The **Neighbor Discovery Protocol (NDP)**, defined in RFC 4861,
takes over that job and several others, all carried over ICMPv6:

- **Neighbor solicitation (NS)** asks which link-layer address holds a given
  IPv6 address. It goes to that address's solicited-node multicast group.
- **Neighbor advertisement (NA)** answers with the link-layer address.
- **Router solicitation (RS)** and **router advertisement (RA)** let a host find
  routers and learn the prefix in use on the link.

Router advertisements are what make **stateless address autoconfiguration
(SLAAC)** work: a host hears the prefix, generates its own interface identifier,
and has a usable address without a server allocating one. DHCPv6 still exists
and is still used where an operator wants a record of who has what, but it is no
longer the only path to an address.

**Duplicate address detection (DAD)** reuses the same machinery. Before a host
commits to an address, it sends a neighbor solicitation *for that address* with
the unspecified address `::` as the source. A reply means something else already
holds it.

## Coexisting with IPv4

IPv4 and IPv6 are separate protocols. An IPv6-only host cannot talk to an
IPv4-only host by default — the packet formats and address lengths differ, and
there is no compatibility mode built into either. Several mechanisms bridge the
gap:

**Dual-stack** runs both protocols on the same interface. Each stack operates
independently with its own addresses and routing table, and applications choose
between them per connection — in practice using Happy Eyeballs, which races an
IPv6 and an IPv4 connection and keeps whichever answers first. This is the
approach most networks actually deploy, and the reason IPv6 adoption has been
gradual rather than disruptive.

**Tunneling** carries IPv6 packets inside IPv4 packets, encapsulated as IP
protocol 41 and unwrapped when they reach an IPv6-capable network. Automatic
schemes such as 6to4 (RFC 3056) once made this transparent, but 6to4 relied on
public relays with no accountability and was formally deprecated by RFC 7526.
Configured tunnels between endpoints an operator controls are still useful.

**NAT64** translates between the two protocols so an IPv6-only client can reach
an IPv4-only server. It is normally paired with **DNS64**, which synthesizes
IPv6 answers for names that only have IPv4 records. This is how many mobile
carriers run IPv6-only access networks while the IPv4 internet still exists.

## Suggested practice: read your own IPv6 configuration

Most home connections now carry IPv6, and every Linux host has link-local
addressing whether or not the upstream network does. Nothing here needs root.

1. Run `ip -6 addr show` and identify each address by prefix. Expect at least a
   `fe80::` link-local on each interface and `::1` on `lo`. If your ISP provides
   IPv6, you will also see a global address — often several, one of them
   temporary.
2. Compare an address that looks EUI-64-derived (`fffe` in the middle) against
   one that does not. Check `ip -6 addr` for the `temporary` and `mngtmpaddr`
   flags to see which is which.
3. Run `ip -6 route show` and find the default route. Note that its next hop is
   a link-local address, not a global one.
4. Ping the all-nodes multicast group on a link with `ping -6 ff02::1%eth0`,
   substituting your interface name. Every IPv6 host on that segment answers.
5. Run `ip -6 neigh show` to see the neighbor cache NDP built — the IPv6
   equivalent of `arp -n`, mapping addresses to link-layer addresses with a
   reachability state.
6. Watch discovery happen: `sudo tcpdump -i eth0 icmp6 and ip6[40] == 135` shows
   neighbor solicitations. Flush an entry with `sudo ip -6 neigh flush dev eth0`
   and watch it get rebuilt.

Write down which of these worked and which did not. "My ISP does not hand out
IPv6, so I only have link-local" is a real and useful finding.

## Related pages

- [IPv4 addressing](/learn/ipv4-addressing) — the addressing model IPv6
  replaces, and the scarcity workarounds it removes.
- [Network traffic types](/learn/traffic-types) — unicast, multicast, anycast,
  and broadcast, and why IPv6 dropped one of them.
- [Software-defined networking](/learn/software-defined-networking) — the SDN
  and overlay designs that assume plentiful addressing.
- [The OSI model](/learn/osi-model) — where addressing sits relative to
  framing and transport.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [RFC 4291: IP Version 6 Addressing Architecture](https://www.rfc-editor.org/rfc/rfc4291.txt)
  — the address types, prefixes, and interface identifier format.
- [RFC 5952: A Recommendation for IPv6 Address Text Representation](https://www.rfc-editor.org/rfc/rfc5952.txt)
  — the canonical rules for writing an address down.
- [RFC 4861: Neighbor Discovery for IP version 6](https://www.rfc-editor.org/rfc/rfc4861.txt)
  — NDP, including neighbor solicitation and advertisement.
- [RFC 4862: IPv6 Stateless Address Autoconfiguration](https://www.rfc-editor.org/rfc/rfc4862.txt)
  — SLAAC and duplicate address detection.
- [RFC 4193: Unique Local IPv6 Unicast Addresses](https://www.rfc-editor.org/rfc/rfc4193.txt)
  — the `fc00::/7` range and how the global ID is generated.
- [RFC 6146: Stateful NAT64](https://www.rfc-editor.org/rfc/rfc6146.txt) and
  [RFC 6147: DNS64](https://www.rfc-editor.org/rfc/rfc6147.txt) — the
  translation pair.

Two corrections worth recording, because my notes had them wrong: IPv6 multicast
is `ff00::/8`, not `/16`, and the loopback address is `::1/128` while `::/128`
is the unspecified address. They are one character apart and mean entirely
different things.
