---
topic: "IPv6 addressing"
questions:
  - stem: "Which prefix identifies IPv6 multicast?"
    options:
      - "ff00::/16"
      - "ff00::/8"
      - "fe80::/10"
      - "fc00::/7"
    answer: 1
    explanation: >
      Multicast is `ff00::/8` — the whole first byte is `ff`, and the bits that
      follow encode flags and scope rather than narrowing the prefix. Writing
      it as /16 is a natural slip because multicast addresses are usually met
      as `ff02::1` and `ff02::2`, which makes the first two bytes look fixed;
      `ff02::/16` is in fact the link-local multicast range, a subset. The other
      two prefixes are real and belong elsewhere: `fe80::/10` is link-local
      unicast and `fc00::/7` is unique local.
    learn: { slug: "ipv6-addressing", anchor: "address-types" }

  - stem: "What is the difference between `::` and `::1`?"
    options:
      - "`::` is loopback and `::1` is the first address on the link"
      - "They are the same address written two ways, since leading zeros are dropped"
      - "`::/128` is the unspecified address, used as a source before a host has one; `::1/128` is loopback"
      - "`::` is a wildcard matching any address in a rule, and `::1` is loopback"
    answer: 2
    explanation: >
      One character apart and entirely different jobs. The unspecified address
      appears as a source when a host has not yet committed to an address —
      duplicate address detection is the standard case. Loopback is `::1`, the
      IPv6 equivalent of 127.0.0.1. Swapping them is a common note-taking
      error. They are not the same address: `::` is 128 zero bits and `::1` has
      the last bit set, and zero compression does not erase a significant 1.
      The wildcard reading comes from firewall syntax, where `::/0` rather than
      `::/128` is the match-anything prefix.
    learn: { slug: "ipv6-addressing", anchor: "address-types" }

  - stem: "Why may `::` appear only once in an IPv6 address?"
    options:
      - "Because two occurrences would make it impossible to work out how many zero hextets belong to each run"
      - "Because the address would then be longer than 128 bits"
      - "Because only the leading run of zeros may be compressed"
      - "Because a second `::` is reserved to mark the boundary between the network and interface portions"
    answer: 0
    explanation: >
      The compression works by subtraction — the reader counts the hextets that
      are written and infers that the rest are zero. With two gaps there is no
      way to divide the missing hextets between them, so the address becomes
      ambiguous. Length is not the issue; a compressed address is shorter, not
      longer. Compression is not restricted to a leading run — it can shorten a
      run anywhere, and the convention is to shorten the longest one. And
      nothing in the notation marks the network/interface boundary; that is
      what the prefix length is for.
    learn: { slug: "ipv6-addressing", anchor: "the-shape-of-an-address" }

  - stem: "Compress 2001:0db8:0000:0000:0000:ff00:0042:8329 as RFC 5952 asks."
    options:
      - "2001:db8:0:0:0:ff00:42:8329"
      - "2001:DB8::FF00:42:8329"
      - "2001:0db8::ff00:0042:8329"
      - "2001:db8::ff00:42:8329"
    answer: 3
    explanation: >
      Both conventions apply: leading zeros within each hextet are dropped, and
      the longest run of all-zero hextets collapses to `::`. Lowercase hex is
      part of the recommendation, which rules out the uppercase form. The first
      option drops leading zeros but leaves the zero run written out, and the
      third does the reverse — compressing the run while keeping `0db8` and
      `0042` padded. All four denote the same address; the point of the rule is
      that one address gets written the same way in a log, a config file, and a
      firewall rule, so a search for it finds it.
    learn: { slug: "ipv6-addressing", anchor: "the-shape-of-an-address" }

  - stem: "A site is given a /48 and assigns a /64 to each subnet. How many subnets does that allow?"
    options:
      - "256"
      - "4,096"
      - "65,536"
      - "18,446,744,073,709,551,616"
    answer: 2
    explanation: >
      The 16 bits between the /48 and the /64 are the subnet ID, giving 2^16 =
      65,536 subnets. The point being made is that subnetting in IPv6 is an
      organisational question rather than a conservation one — a single site
      has more subnets than it can find uses for. The very large figure is 2^64,
      which counts interfaces within one subnet, not subnets. The smaller
      figures would correspond to 8 and 12 subnet bits, which is not how the
      /48-to-/64 allocation divides.
    learn: { slug: "ipv6-addressing", anchor: "network-and-interface-portions" }

  - stem: "Is the /64 boundary between network and interface a property of the IPv6 address format?"
    options:
      - "Yes — the format fixes 64 bits of network and 64 of interface identifier"
      - "No — it is a strong convention that stateless autoconfiguration requires and most tooling assumes"
      - "Yes for global unicast, no for link-local"
      - "No — it is an arbitrary default that can be changed on a LAN with no consequences"
    answer: 1
    explanation: >
      The address format itself does not mandate the split; prefix lengths are
      as flexible as they are in CIDR. What makes /64 effectively binding is
      everything built on top of it, starting with stateless address
      autoconfiguration, which needs 64 bits to generate an interface
      identifier into. Calling it a format property overstates it. Making it
      conditional on address type is not the distinction — link-local addresses
      use the same 64-bit interface identifier. And treating it as freely
      changeable understates the consequences: a narrower prefix on a normal
      LAN breaks autoconfiguration and confuses tooling.
    learn: { slug: "ipv6-addressing", anchor: "network-and-interface-portions" }

  - stem: "EUI-64 derives an interface identifier from the hardware MAC address. Why is it no longer the default?"
    options:
      - "It produced collisions on large segments"
      - "It is incompatible with stateless autoconfiguration"
      - "It embeds a hardware serial number in every packet the host sends, which is a privacy problem"
      - "It requires a 64-bit MAC address, which most hardware does not have"
    answer: 2
    explanation: >
      A MAC-derived identifier follows the machine between networks, so the
      lower half of its address is a stable device fingerprint visible to every
      server it contacts. Current systems generate identifiers that are stable
      per network but not derived from hardware, and add temporary addresses
      that rotate for outbound connections — which is why one interface
      commonly holds several IPv6 addresses at once. Collisions were not the
      problem; a MAC address is globally unique, which is what made it
      attractive in the first place. It works with autoconfiguration — it was
      the original mechanism for it. And EUI-64 is built from a 48-bit MAC by
      inserting `fffe` in the middle, so no 64-bit hardware address is needed.
    learn: { slug: "ipv6-addressing", anchor: "the-interface-identifier" }

  - stem: "Why does a link-local address often need a zone index, as in `fe80::1%eth0`?"
    options:
      - "Because the same `fe80::/10` prefix exists on every link, so the address alone does not say which one you mean"
      - "Because the zone index supplies the missing prefix length"
      - "Because link-local addresses are not unique and the index disambiguates duplicates on the same segment"
      - "Because the kernel uses it to select a routing table entry for off-link traffic"
    answer: 0
    explanation: >
      Every interface configures a link-local address from the same prefix, so
      a host with three interfaces has three networks that look alike. The
      address is unambiguous only once you say which link it is on, and the
      zone index does that. It is not a prefix length — that is written after a
      slash and is a separate thing. Duplicates on one segment are prevented by
      duplicate address detection, so uniqueness within a link is not the
      problem. And link-local traffic never leaves the link, so no off-link
      routing decision is involved.
    learn: { slug: "ipv6-addressing", anchor: "address-types" }

  - stem: "IPv6 has no broadcast address. How does a host ask which link-layer address holds a given IPv6 address?"
    options:
      - "It sends an ARP request to the all-ones link-layer address, as IPv4 does"
      - "It sends a neighbor solicitation to that address's solicited-node multicast group"
      - "It queries the router, which keeps a table of every address on the link"
      - "It sends a neighbor solicitation to `ff02::1`, the all-nodes group"
    answer: 1
    explanation: >
      Neighbor Discovery replaces ARP, and its refinement is that the question
      is not asked of everybody. Each unicast address has a solicited-node
      group derived from its last 24 bits, so only the handful of interfaces
      sharing those bits have to process the request — a structural improvement
      over ARP, which every host on the segment must inspect. There is no ARP
      in IPv6 at all. No router holds a registry of link addresses. And sending
      to the all-nodes group would work but would throw away the whole benefit,
      making it as noisy as the broadcast it replaced.
    learn: { slug: "ipv6-addressing", anchor: "neighbor-discovery" }

  - stem: "Duplicate address detection sends a neighbor solicitation for an address the host is about to use. What does it use as the source address, and why?"
    options:
      - "The address being tested, so a reply can be matched to the request"
      - "Its link-local address, which is already known to be unique"
      - "The unspecified address `::`, because the host has not committed to the address it is asking about"
      - "The all-nodes multicast address `ff02::1`"
    answer: 2
    explanation: >
      Using the tentative address as a source would assert ownership of the
      very thing being checked, which is what the exercise is trying to avoid.
      The unspecified address says plainly that the sender has no address yet.
      The link-local answer is tempting but has the same problem in the general
      case — duplicate address detection also runs for the link-local address
      itself, before there is anything else to use. And `ff02::1` is a
      destination group; a multicast address is never valid as a source.
    learn: { slug: "ipv6-addressing", anchor: "neighbor-discovery" }

  - stem: "Can an IPv6-only host reach an IPv4-only server without help?"
    options:
      - "Yes — IPv6 includes a compatibility mode that maps IPv4 addresses into the IPv6 space"
      - "Yes, provided both ends support Happy Eyeballs"
      - "No — they are separate protocols, and something such as NAT64 with DNS64 has to translate"
      - "No, unless the server has a global unicast address in `2000::/3`"
    answer: 2
    explanation: >
      IPv4 and IPv6 are different protocols with different packet formats and
      address lengths, and neither has a built-in fallback to the other. Making
      the two talk requires a translator: NAT64 rewrites between them and DNS64
      synthesises IPv6 answers for names that only have IPv4 records, which is
      how carriers run IPv6-only access networks. Happy Eyeballs is a client
      strategy for choosing between two stacks that both already work, so it
      does not apply to a host with only one. And an IPv4-only server has no
      IPv6 address of any kind to hold.
    learn: { slug: "ipv6-addressing", anchor: "coexisting-with-ipv4" }

  - stem: "Which range fills the role for IPv6 that RFC 1918 fills for IPv4?"
    options:
      - "`fe80::/10`, since link-local traffic never leaves the network"
      - "`fc00::/7`, with `fd00::/8` the half actually used, where a site generates a random global ID"
      - "`2000::/3`, filtered at the site boundary"
      - "`::/128`, which is unroutable by definition"
    answer: 1
    explanation: >
      Unique local addresses are the site-scoped range, and the usable half
      requires a randomly generated 40-bit global ID so that two networks
      merged later are unlikely to collide — a deliberate improvement on
      everyone picking the same private range. Link-local is scoped to a single
      link, not a site, so it cannot serve as a routable internal range.
      `2000::/3` is global unicast and filtering it at the boundary is a policy
      rather than an address property. And `::/128` is the unspecified address,
      a single value rather than a range. Worth noting that unique local
      addresses are not a security boundary — they are simply not routed
      off-site.
    learn: { slug: "ipv6-addressing", anchor: "address-types" }
---
