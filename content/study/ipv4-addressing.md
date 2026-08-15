---
topic: "IPv4 addressing"
questions:
  - stem: "A host comes up with an address in 169.254.0.0/16 and no default gateway. What does that tell you?"
    options:
      - "A DHCP server handed it an address from a reserved range"
      - "The link is up at layer 2 and DHCP is not answering"
      - "Someone configured the interface statically and got the range wrong"
      - "The network is IPv6-only and this is a translation artefact"
    answer: 1
    explanation: >
      Link-local self-assignment is what an interface falls back to when DHCP
      does not answer, which makes it a useful diagnostic rather than a
      failure in itself: the interface is up and able to check the segment for
      address conflicts, and the DHCP exchange is the part that is broken. No
      server assigns it — the host picks the address itself, which rules out
      the first option. A static misconfiguration is possible in principle but
      would be an odd range to choose by hand, and the absence of a gateway is
      the giveaway that this is the automatic fallback. Nothing about IPv6
      produces a 169.254 address.
    learn: { slug: "ipv4-addressing", anchor: "link-local-addresses-when-dhcp-fails" }

  - stem: "Two hosts on the same segment both fall back to link-local addressing. What can they do?"
    options:
      - "Nothing — link-local addresses cannot carry traffic"
      - "Reach each other, and also reach the internet once one of them advertises itself as a gateway"
      - "Reach each other on that segment, and nothing beyond it"
      - "Reach each other only after an administrator adds a static route"
    answer: 2
    explanation: >
      Link-local addressing is a working address scheme with no routing
      attached. Hosts on one segment can talk to each other, which is exactly
      what it exists for, and there is no gateway so nothing off the segment is
      reachable. Saying it carries no traffic understates it. The gateway
      option describes something that does not happen automatically and would
      not help anyway, since the upstream network knows nothing about
      169.254.0.0/16. A static route needs a next hop that can actually forward
      the traffic, and neither host has one.
    learn: { slug: "ipv4-addressing", anchor: "link-local-addresses-when-dhcp-fails" }

  - stem: "Why can a host on a private range start a conversation with a public server, while a public host cannot start one with it?"
    options:
      - "Private addresses are one-way by design and can only be used as a source"
      - "The gateway holds a public address and translates on the private network's behalf, so there is no return path until the inside starts the exchange"
      - "ISPs filter inbound traffic to private ranges as a policy choice, and can be asked to stop"
      - "Private addresses use a different protocol number that public routers do not recognise"
    answer: 1
    explanation: >
      Routers on the public internet drop private addresses, so a packet
      addressed to one has nowhere to go. Outbound works because the gateway
      substitutes its own public address and keeps enough state to send the
      reply back. The asymmetry is a consequence of that translation, not a
      property of the addresses themselves — they are ordinary IPv4 addresses
      usable in either direction inside the network. It is not an ISP policy
      that can be lifted, since the whole point of the reserved ranges is that
      everyone reuses them. And there is no separate protocol number involved;
      the packets are the same IPv4 packets.
    learn: { slug: "ipv4-addressing", anchor: "public-and-private-addresses" }

  - stem: "Which of these is not one of the three blocks RFC 1918 reserves for private use?"
    options:
      - "10.0.0.0/8"
      - "172.16.0.0/12"
      - "192.168.0.0/16"
      - "172.32.0.0/12"
    answer: 3
    explanation: >
      The private class B range runs 172.16.0.0 through 172.31.255.255, which
      is what the /12 prefix covers. 172.32 is one step past the end of it and
      is public address space, which is why picking an address there by
      accident produces a network that appears to work locally and quietly
      breaks when someone tries to reach the real owner of that block. The
      other three are the reserved blocks exactly as written, one carved out of
      each of the old classes A, B, and C.
    learn: { slug: "ipv4-addressing", anchor: "public-and-private-addresses" }

  - stem: "`ping 127.0.0.1` succeeds on a machine whose network cable is unplugged. What has that proved?"
    options:
      - "The local IP stack is working; nothing about any cable, switch, or router"
      - "The interface has negotiated a link even without a cable attached"
      - "The default gateway is reachable through a cached route"
      - "The machine has a valid address and mask on its network interface"
    answer: 0
    explanation: >
      Loopback traffic never reaches a wire — the stack turns it around
      internally — so a successful ping exercises the local IP implementation
      and stops there. That is precisely why it is a useful first test: a
      failure points at local configuration rather than at the network. It says
      nothing about link negotiation, which requires a cable and a peer.
      Nothing about the gateway is involved, cached or otherwise. And the
      interface address is a separate matter; loopback works whether or not any
      other interface is configured.
    learn: { slug: "ipv4-addressing", anchor: "loopback" }

  - stem: "A service is bound to 127.0.0.1 rather than 0.0.0.0. What has that achieved?"
    options:
      - "Traffic to the service is encrypted, because loopback traffic never leaves the host"
      - "The service is reachable only from the machine it runs on, which is a real security boundary"
      - "The service is reachable from the local network but not from the internet"
      - "Nothing meaningful — a firewall rule is the only way to restrict a listening service"
    answer: 1
    explanation: >
      A loopback binding means packets from anywhere else simply have no way
      to arrive, which is a stronger guarantee than a filtering rule that
      could be misordered or removed. It is a genuine boundary and is the usual
      way to put a service behind a reverse proxy. It has nothing to do with
      encryption — the bytes are in the clear, they just never leave the
      machine. It is not a local-network exception either: hosts on the same
      LAN are as excluded as hosts on the internet. And the claim that only a
      firewall can restrict a service ignores that not listening on an
      interface is the simplest restriction available.
    learn: { slug: "ipv4-addressing", anchor: "loopback" }

  - stem: "Why is 127 missing from the class A first-octet range of 1 to 126?"
    options:
      - "127 was reserved for future expansion and was never allocated"
      - "127 belongs to class B, whose range begins at 127 rather than 128"
      - "127.0.0.0/8 is reserved for loopback"
      - "127 is the multicast range, which is why class D starts there"
    answer: 2
    explanation: >
      The class A range stops at 126 because the whole of 127.0.0.0/8 is
      loopback and cannot be handed to a network. The other end of the range is
      trimmed for a similar reason: 0.0.0.0/8 is reserved, which is why the
      range starts at 1 rather than 0. Class B begins at 128, not 127. And
      multicast is class D, which starts at 224 — a long way from 127.
    learn: { slug: "ipv4-addressing", anchor: "classful-addressing" }

  - stem: "An organisation in the classful era needed 300 addresses. What made this awkward, and why does the story still matter?"
    options:
      - "A class C gave 254 usable addresses and a class B gave 65,534, so the choice was between too few and enormous waste — the granularity problem that CIDR fixed"
      - "Class B was reserved for governments, so the only lawful option was several class C networks"
      - "Nothing was awkward; classful addressing allowed any prefix length between the class boundaries"
      - "Routers of the period could not hold more than one class C route, so a second network was impossible"
    answer: 0
    explanation: >
      Fixed class sizes meant the network/host boundary could only sit in three
      places, and 300 hosts falls in the gap between two of them. That
      granularity failure is the reason classful addressing was abandoned in
      the 1990s in favour of explicit prefix lengths. Class B was not reserved
      by policy for any particular kind of organisation. The claim that any
      prefix length was allowed describes CIDR, which is the thing that
      replaced classes — under classes the mask was implied by the first
      octet. And routing table capacity is a different problem entirely from
      address allocation granularity.
    learn: { slug: "ipv4-addressing", anchor: "classful-addressing" }

  - stem: "Does any part of a current router's forwarding decision depend on address classes?"
    options:
      - "Yes — the class determines the default mask when none is configured"
      - "Yes, but only for addresses in the private ranges"
      - "No — classful addressing is historical, and it survives because it explains where the default masks and private ranges came from"
      - "No, but classes are still used to decide whether an address may be advertised to the internet"
    answer: 2
    explanation: >
      Nothing in a modern network makes a forwarding decision from an address
      class. Prefixes are explicit, and a router matches the longest one it
      has. The class table is worth learning as history because it explains
      otherwise arbitrary facts — why the default masks are /8, /16, and /24,
      and why the private ranges are the odd shapes they are. Defaulting a mask
      from the first octet is exactly the behaviour that was abandoned. The
      private ranges are recognised by prefix, not by class. And whether an
      address is advertised is a policy and registry matter, not a class one.
    learn: { slug: "ipv4-addressing", anchor: "classful-addressing" }

  - stem: "Convert the octet 10101000 to decimal."
    options:
      - "148"
      - "168"
      - "160"
      - "172"
    answer: 1
    explanation: >
      Add the place values wherever there is a 1, reading left to right from
      128: 128 + 32 + 8 = 168. The other answers each correspond to dropping or
      adding a bit — 148 would be 10010100, 160 would be 10100000 with the 8
      bit clear, and 172 would need the 4 bit set as well. Doing this in your
      head is worth the practice because subnet masks only make sense in
      binary, and 168 in particular turns up constantly as the second octet of
      the most common private range.
    learn: { slug: "ipv4-addressing", anchor: "binary-and-the-shape-of-an-address" }

  - stem: "Why does IPv4 provide roughly 4.3 billion addresses?"
    options:
      - "Because the four octets are decimal values from 0 to 255, and 255 to the fourth power is about 4.2 billion"
      - "Because RFC 791 caps the allocatable space at 2^32 minus the reserved blocks"
      - "Because a 32-bit address gives 2^32 distinct values"
      - "Because each of the five classes contributes a fixed share that totals 2^32"
    answer: 2
    explanation: >
      The number falls straight out of the address width: 32 bits, so
      4,294,967,296 possible values. Everything else about IPv4 addressing is a
      consequence of that one figure being smaller than the demand. Working it
      from 255 to the fourth power is a common slip — each octet has 256
      values, not 255, because 0 counts. The reserved blocks reduce what is
      usable but they are subtracted from 2^32 rather than being the reason for
      it. And the classes are a way of dividing the space, not a source of it.
    learn: { slug: "ipv4-addressing", anchor: "binary-and-the-shape-of-an-address" }

  - stem: "You run `ping` against a server and get replies. What have you established about the service you were trying to reach?"
    options:
      - "The host is reachable — `ping` uses ICMP and says nothing about whether any service is listening"
      - "The service is listening, since a reply requires an open port"
      - "The service is listening on TCP but may be refusing connections at the application level"
      - "Nothing at all, because ICMP replies are generated by intermediate routers rather than the host"
    answer: 0
    explanation: >
      `ping` is an ICMP echo exchange and ICMP sits below the transport layer,
      so there are no ports involved and no service is consulted. A reply
      proves that the host is up and that packets get there and back, which is
      genuinely useful and routinely overinterpreted. The second and third
      options both assume a port, which ICMP does not have. The last option
      overcorrects: an echo reply to your request does come from the target
      host, and intermediate routers only generate different message types such
      as time-exceeded.
    learn: { slug: "ipv4-addressing", anchor: "suggested-practice-read-and-verify-your-own-network" }
---
