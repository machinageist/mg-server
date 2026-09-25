---
title: "Switching technologies: VLANs, tagging, and loop prevention"
date: 2026-09-20
summary: "VLANs and trunks, the 802.1Q tag, inter-VLAN routing, link aggregation, spanning tree, and what bounds the size of a frame."
tags: [education, networking, switching, vlan, spanning-tree, ethernet]
---

## Overview

A router decides which network a packet belongs in. A switch gets the frame to a
port. Both are forwarding decisions, but they read different addresses and answer
different questions, and an ordinary network needs both.

Past that basic forwarding, most of what a switch does comes down to three jobs:
dividing one physical switch into separate broadcast domains, carrying several of
those domains over a single cable without mixing them, and keeping redundant links
between switches from turning into loops. VLANs, 802.1Q tagging, and Spanning Tree
Protocol are the mechanisms for those three jobs.

When reading a switch configuration, ask three things about every port: which
broadcast domain it belongs to, whether its traffic is tagged, and what happens to a
frame that arrives on it without a tag.

## VLANs

A virtual LAN (VLAN) is a logical division of a switch rather than a physical one.
Ports on the same switch can sit in different VLANs, and ports on different switches
can sit in the same one. Each VLAN is its own broadcast domain: a broadcast sent
inside VLAN 10 reaches VLAN 10 and stops there.

The usual reasons to do this are performance and containment. A smaller broadcast
domain carries less flooded traffic, and a device in one VLAN cannot reach a device
in another without passing through a router, and a router is a place where policy
can be applied.

A VLAN and an IP subnet are not the same thing. A VLAN is a Layer 2 broadcast domain;
a subnet is a Layer 3 address range. They are almost always configured one to one,
and the habit of using the words interchangeably comes from that convention rather
than from anything in the standards.

### Access ports and the VLAN database

A switch keeps a list of the VLANs it knows about, each with a numeric ID, such as
10, 20, or 30. A port assigned to exactly one of them is an access port. Frames
leaving an access port toward an end device carry no VLAN tag, and the device is not
expected to know it is in a VLAN at all.

### Trunk ports

A port carrying traffic for more than one VLAN is a trunk port, and the link between
two trunk ports is a trunk link. Trunks are what let a single cable between two
switches, or between a switch and a router, carry several VLANs while keeping them
apart. The separation is done by tagging, described below.

### The native VLAN

A trunk has one native VLAN, and traffic in it crosses the trunk untagged while
everything else is tagged. Without a native VLAN, a frame arriving on a trunk with no
tag has nothing to identify it and the switch has to drop it.

The native VLAN is a standing security consideration. If the two ends of a trunk
disagree about which VLAN is native, traffic silently changes VLAN as it crosses.
Worse, a frame carrying two tags, the outer one matching the native VLAN, can have
that outer tag stripped by the first switch and then be forwarded into whatever VLAN
the inner tag names. The usual mitigations are to make the native VLAN one that
carries no user traffic, and to configure both ends identically.

### Voice VLANs

An access port can be configured to put untagged traffic in a data VLAN and tagged
traffic from an attached IP phone in a voice VLAN. This works because most IP phones
contain a small switch: the phone takes the wall port, and the computer plugs into
the phone.

Voice is separated because it tolerates loss and delay badly. A few hundred
milliseconds of jitter that a file transfer absorbs invisibly is audible on a call.
Giving voice its own VLAN creates somewhere for queuing and priority policy to be
applied, and keeps the traffic out of the data VLAN's broadcasts.

Separating the traffic is necessary but not sufficient. A voice VLAN only improves
call quality if something along the path acts on the priority marking.
[Network functions](/learn/network-functions) covers how that marking is honored, or
not.

### Private VLANs

Sometimes devices need to share a subnet and still be unable to reach each other:
guests on a hotel network, tenants in a hosting environment, hosts in a screened
segment. A private VLAN does that without giving every device its own subnet.

It divides one primary VLAN into secondary VLANs with different reachability:

- Isolated — these ports reach the promiscuous port and nothing else, not even each
  other.
- Community — these ports reach the promiscuous port and the other ports in their own
  community, but no other community and no isolated port.
- Promiscuous — the port that reaches everything, normally the one toward the default
  gateway.

All of them keep the same IP subnet and the same default gateway, so nothing changes
on the attached hosts. The enforcement happens on the switch, so that is where the
control is.

## Inter-VLAN routing and switch virtual interfaces

VLANs stop traffic at Layer 2, so anything crossing between them has to be routed.
There are two common ways to arrange that.

A switch virtual interface (SVI) is a logical Layer 3 interface on the switch itself,
one per VLAN, each holding an IP address that serves as that VLAN's default gateway.
A Layer 3 switch with SVIs routes between its VLANs internally.

The alternative, called router-on-a-stick, puts the routing on a separate router
reached over a trunk, using subinterfaces. That side is covered on
[routing technologies](/learn/routing-technologies), where the subinterfaces live.

Either way, the moment traffic crosses between VLANs it is being routed, and the
isolation the VLAN provided ends there. A VLAN boundary is only a policy boundary if
something at the routing point enforces policy.

## 802.1Q tagging

IEEE 802.1Q is the standard that lets a trunk carry several VLANs. A tagged frame
carries four extra bytes naming the VLAN it belongs to. A switch receiving it reads
the VLAN ID and forwards the frame only within that VLAN.

The tag is inserted after the source MAC address, ahead of the original frame's
EtherType:

| Field                | Purpose                        | Size   |
|----------------------|--------------------------------|--------|
| Destination MAC      | The receiving interface        | 6 B    |
| Source MAC           | The sending interface          | 6 B    |
| 802.1Q tag           | VLAN ID and priority           | 4 B    |
| EtherType / Length   | The protocol of the payload    | 2 B    |
| Payload              | The encapsulated data          | Varies |
| Frame check sequence | Error detection over the frame | 4 B    |

A common misreading is that the tag follows the EtherType. It does not. The tag's
first two bytes sit where the EtherType would have been, and the original EtherType
is pushed along behind them. Those two bytes are the tag protocol identifier (TPID),
always `0x8100` for a VLAN tag, and their only job is to tell a receiver that a tag
follows instead of a payload.

The other two bytes are the tag control information (TCI):

| Field                         | Purpose                              | Size    |
|-------------------------------|--------------------------------------|---------|
| Priority code point (PCP)     | Traffic class, 0 to 7, used by QoS   | 3 bits  |
| Drop eligible indicator (DEI) | Marks the frame droppable if queues congest | 1 bit |
| VLAN identifier (VID)         | Which VLAN the frame belongs to      | 12 bits |

Twelve bits gives 4,096 values, of which 4,094 are usable VLAN IDs: 0 means the frame
carries priority information but no VLAN, and 4,095 is reserved. That limit is one
of the reasons overlay encapsulations exist, as covered on
[software-defined networking](/learn/software-defined-networking).

Tags are added and removed continuously as frames move. A switch tags a frame on its
way out to a trunk and strips the tag before handing it to an access port, so the end
device normally never sees one. The four bytes do mean a tagged frame is larger than
an untagged one, which matters for frame size below.

## Interface configuration

An interface is a port, physical or logical, on a network device. One physical
interface can carry several logical subinterfaces, each with its own address and
VLAN, which is how a single cable serves several networks.

The settings that most often have to agree between two ends are VLAN assignment,
tagging, speed, and duplex. A mismatch in any of them looks like a failing cable
instead of a configuration error, so check them first.

## Link aggregation

Link aggregation combines several physical links into one logical link. The bundle
has the combined capacity of its members and survives losing any one of them. It is
common between switches, and between a switch and a busy server, wherever a single
link is the constraint.

The capacity is aggregate, not per-flow. A bundle spreads traffic by hashing fields
from each frame, such as MAC addresses, IP addresses, and port numbers, so every
frame in one conversation takes the same member link and arrives in order. Four 1 Gbps links give
4 Gbps of total capacity, not a 4 Gbps single transfer.

Link Aggregation Control Protocol (LACP) negotiates the bundle with the device at the
other end rather than assuming it, so a miscabled or half-configured member is left
out instead of being used. It was standardized in IEEE 802.3ad and now lives in IEEE
802.1AX. Vendors variously call the result an EtherChannel, a port channel, or a
bond.

A Cisco IOS configuration of that shape creates the logical interface, sets its
behavior, then adds the physical members:

```text
configure terminal

interface Port-channel1
 switchport mode trunk
 switchport trunk native vlan 999
exit

interface GigabitEthernet0/1
 channel-group 1 mode active
exit

interface GigabitEthernet0/2
 channel-group 1 mode active
exit

end
show etherchannel summary
write memory
```

Two things in that are easy to get backwards. `mode active` is what makes a member
speak LACP instead of bundling unconditionally. And `channel-group` belongs on the
physical interfaces, not on the port-channel. The port-channel exists because
members join it. Trunk settings go on the port-channel, and the members inherit them.
Setting them on each member instead is a good way to build a bundle that never
forms.

## Speed and duplex

Speed is the rate an interface negotiates, from megabits to tens of gigabits per
second. The ceiling comes from the medium and the transceivers at both ends, covered
on [wired media](/learn/wired-media) and
[transceivers and connectors](/learn/transceivers).

Duplex is whether an interface can send and receive at the same time. Half duplex
allows one direction at a time and belongs to the shared-media era. Full duplex,
which every switched link uses, allows both at once.

Duplex mostly matters because of what a mismatch does. If one end
autonegotiates and the other is pinned, the negotiating end commonly settles on half
duplex while its partner runs full. The link comes up, pings succeed, and throughput
collapses under load, with late collisions and errors climbing on one side only. It
reads like a failing cable. Confirming that both ends agree costs one command and
rules it out.

## Spanning Tree Protocol

Redundant links between switches are good for availability and dangerous at Layer 2.
An Ethernet frame carries no time-to-live field to expire it, so a broadcast on a
looped topology is forwarded around the loop indefinitely, multiplying at every
switch until the segment is unusable. That is a broadcast storm, and it takes
seconds.

Spanning Tree Protocol (STP) prevents it by reducing the physical topology to a
loop-free logical one. The switches elect a root bridge as a common point of
reference, each switch works out its lowest-cost path toward that root, and every
link that is not on such a path is blocked. Blocked is not disconnected: when an
active path fails, STP recalculates and a blocked link takes over.

Switches learn about each other and notice topology changes by exchanging bridge
protocol data units (BPDUs), which is why a port can be receiving BPDUs while
forwarding nothing else.

### Port states

The original standard, IEEE 802.1D, moves a port through five states:

- Blocking — forwards nothing and learns nothing, listening only for BPDUs.
- Listening — takes part in the topology calculation, still forwarding and learning
  nothing.
- Learning — records source MAC addresses into the address table, but does not yet
  forward.
- Forwarding — carries traffic, and goes on processing BPDUs.
- Disabled — administratively down and outside STP entirely.

The built-in delay through listening and learning is what makes classic STP slow. It
takes roughly thirty seconds before a port that has just come up passes traffic,
which is long enough for people to decide the port is dead.

### Faster variants

Rapid Spanning Tree Protocol (RSTP), introduced in IEEE 802.1w, converges in seconds
rather than tens of seconds. It collapses blocking and listening into a single
discarding state, leaving discarding, learning, and forwarding.

Multiple Spanning Tree Protocol (MSTP), introduced in IEEE 802.1s, runs several
spanning-tree instances and maps VLANs onto them. Different VLANs can then use
different active links, so a link blocked for one instance can be forwarding for
another instead of sitting idle.

Both amendments were later folded into IEEE 802.1Q, which is now the single bridging
standard holding VLAN tagging and spanning tree together. RSTP is the sensible
default on current equipment. Plain 802.1D shows up on old gear and in exam
questions.

## Frame size

Maximum transmission unit (MTU) is the largest payload a link will carry in one unit.
For standard Ethernet it is 1,500 bytes. The frame around that adds 18 bytes of
header and frame check sequence, giving 1,518 on the wire, or 1,522 once an 802.1Q
tag is present.

Getting it wrong is asymmetric. An MTU set too small is a steady tax: more headers
per byte delivered, more packets to process. An MTU too large for some link along the
path is worse, because the correction happens somewhere else. In IPv4 a router may
fragment the packet, unless the don't-fragment bit is set, in which case it drops the
packet and reports back. IPv6 routers do not fragment at all. The sending host is
expected to learn the path MTU and size its packets to fit. When that report never
arrives, commonly because a firewall discards the ICMP messages carrying it, the
result is a connection that completes its handshake and then stalls on the first
large transfer. It is a distinctive failure, and an easy one to blame on the wrong
thing.

### Jumbo frames

A jumbo frame carries an MTU well above 1,500 bytes, typically around 9,000. Fewer,
larger frames mean less per-frame overhead and less processing, which helps on
storage and backup networks.

Jumbo frames only help when every device along the path agrees. One switch left at
1,500 in the middle of an otherwise jumbo-configured path produces the same stall
described above, so this setting applies to a whole path, not to one device.

### Where the sliding window fits

Frame size is often discussed alongside TCP's sliding window, and the two are
separate mechanisms at different layers. MTU bounds how much a link carries in one
unit. The sliding window bounds how much a sender may have in flight before the
receiver acknowledges it, and the receiver advertises that figure based on how fast
it is draining its buffer.

They interact, since the window is counted in bytes that have to travel inside
MTU-sized units, but neither does the other's job. A window too small leaves a fast
link idle waiting for acknowledgements. A window larger than the path can hold causes
loss and retransmission. Changing the MTU fixes neither.

## Suggested practice: make a VLAN tag visible on your own machine

Linux builds a tagged interface without needing a managed switch, which is enough to
see the mechanism. On a machine you own, with a wired interface and `sudo`
(substitute your interface name for `eth0`):

1. Create a tagged subinterface with
   `sudo ip link add link eth0 name eth0.10 type vlan id 10`, then bring it up with
   `sudo ip link set eth0.10 up`.
2. Confirm what it is. `ip -d link show eth0.10` reports `vlan protocol 802.1Q id 10`
   and names `eth0` as its parent.
3. Watch the wire. Run `sudo tcpdump -e -nn -i eth0 vlan` in one terminal, then send
   something from the tagged interface in another. `tcpdump -e` prints the link-layer
   header, so the tag shows up as `vlan 10`.
4. Compare that against untagged traffic on `eth0` itself, and note that the payload
   is unchanged. Only the frame header differs.
5. Read the MTU of both with `ip link show`, then probe the real path MTU with
   `ping -M do -s 1472 <a host on your LAN>`. 1,472 plus 28 bytes of IPv4 and ICMP
   header is exactly 1,500, so raising `-s` should fail with a fragmentation-needed
   message.
6. Remove it with `sudo ip link delete eth0.10`.

This demonstrates tag format, MTU, and path behavior. It does not demonstrate
inter-VLAN routing, trunk negotiation, or spanning tree. Those need at least two
switches, physical or virtual. Build them when you have the hardware.

## Exam key points

Where this page's material shows up in the exam objectives, and what each exam
expects beyond it.

### CCNA 200-301

Objectives 2.1, 2.2, 2.4, and 2.5 in v1.1 cover VLANs, trunks and the native VLAN,
EtherChannel with LACP, and Rapid PVST+, with switching concepts in 1.13. In v2.0,
which replaces it in February 2027, 2.1 covers trunks, port channels, and SVIs
between switches and routers, 2.2 covers edge ports for phones, access points, and
virtualized hosts, and 2.5 keeps Rapid PVST+.

- Access ports: `switchport mode access`, `switchport access vlan 10`, and
  `switchport voice vlan 20` for a phone. Check with `show vlan brief`.
- Trunks: `switchport mode trunk`, `switchport trunk native vlan 99`, and
  `switchport trunk allowed vlan 10,20`. Some switches need
  `switchport trunk encapsulation dot1q` first. Check with
  `show interfaces trunk`.
- DTP: two ends set to dynamic auto stay access ports. Turn negotiation off with
  `switchport nonegotiate`.
- EtherChannel: LACP uses active and passive, and at least one end must be active.
  PAgP uses desirable and auto. `mode on` does not negotiate. Members must match
  speed, duplex, and VLAN settings. Check with `show etherchannel summary`.
- Rapid PVST+: the root bridge has the lowest bridge ID, which is the priority
  (32768 by default, plus the VLAN number) followed by the MAC address. Set it
  with `spanning-tree vlan 10 root primary`. Port roles are root, designated,
  alternate, and backup, and states are discarding, learning, and forwarding.
- Use PortFast with BPDU guard on edge ports: `spanning-tree portfast` and
  `spanning-tree bpduguard enable`. v1.1 also lists root guard and loop guard.
- Inter-VLAN routing uses an SVI (`interface vlan 10` plus `ip routing`) or
  router-on-a-stick subinterfaces (`encapsulation dot1Q 10`).
- CDP and LLDP (2.3 in both versions) are not covered on this page.

### Network+ N10-009

Objective 2.2 lists the VLAN database, SVIs, native and voice VLANs, 802.1Q
tagging, link aggregation, speed, duplex, spanning tree, MTU, and jumbo frames.
Objective 5.3 adds switching problems to troubleshoot.

- For 5.3, know how STP loops, the wrong root bridge, port roles and states, and
  incorrect VLAN assignment show up.
- A duplex mismatch shows up as late collisions and errors on one side, with the
  link still up.
- Private VLANs are background. Neither exam lists them.

## Related pages

- [Network appliances](/learn/network-appliances) — the switch itself, its MAC
  address table, and where it sits among the other appliances.
- [Routing technologies and route selection](/learn/routing-technologies) — what
  happens once traffic has to leave its VLAN.
- [The OSI model](/learn/osi-model) — why a VLAN is a Layer 2 construct while the
  subnet mapped onto it is a Layer 3 one.
- [Network traffic types](/learn/traffic-types) — the broadcast and flooded-unicast
  traffic a VLAN boundary contains.
- [Wired media](/learn/wired-media) — the copper and fiber that set the speed a
  switch port can negotiate.
- [Software-defined networking](/learn/software-defined-networking) — the overlays
  built to get past the 12-bit VLAN ceiling.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [IEEE 802.1Q](https://standards.ieee.org/ieee/802.1Q/10323/) — the bridging
  standard that defines the VLAN tag and the priority code point, and that now
  carries spanning tree after the amendments were folded in.
- [IEEE 802.1 Working Group](https://www.ieee802.org/1/) — 802.1D, 802.1w, and
  802.1s, and the record of their consolidation.
- [IEEE 802.3 Ethernet Working Group](https://www.ieee802.org/3/) — the frame format
  the tag is inserted into, and the link aggregation now held in 802.1AX.
- [RFC 5517: Cisco Systems' Private VLANs](https://www.rfc-editor.org/rfc/rfc5517.txt)
  — isolated, community, and promiscuous port behavior.
- [RFC 1191: Path MTU Discovery](https://www.rfc-editor.org/rfc/rfc1191.txt) — how a
  sender learns the largest packet a path will carry, and what breaks when the ICMP
  messages are filtered.
- [RFC 9293: Transmission Control Protocol](https://www.rfc-editor.org/rfc/rfc9293.html)
  — the receive window, which is flow control rather than a frame-size mechanism.

Vendor documentation is the final reference for configuration syntax, and for private
VLAN behavior, which is not uniform across platforms.
