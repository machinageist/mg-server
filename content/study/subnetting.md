---
topic: "Subnetting, CIDR, and VLSM"
questions:
  - stem: "How many addresses can be assigned to interfaces on a /26?"
    options:
      - "64"
      - "62"
      - "63"
      - "60"
    answer: 1
    explanation: >
      A /26 leaves 6 host bits, so 2^6 = 64 addresses in the block, of which 62
      are assignable. Two are unavailable: the all-zeros host address names the
      network itself and the all-ones host address is the broadcast address for
      it. 64 is the block size before that subtraction. 63 is the result of
      subtracting one instead of two, which is the most common version of this
      mistake. 60 has no derivation — it is what you get by rounding off, which
      is worth resisting in an arithmetic that is exact.
    learn: { slug: "subnetting", anchor: "counting-hosts-and-networks" }

  - stem: "The two formulas that cover most subnetting arithmetic are 2^n and 2^n minus 2. What decides which one applies?"
    options:
      - "Whether the network is classful or classless"
      - "Whether the prefix is longer or shorter than /24"
      - "Whether you are counting hosts or counting networks — hosts count host bits and subtract two, networks count network bits and subtract nothing"
      - "Whether broadcast is in use on the segment"
    answer: 2
    explanation: >
      Mixing these up is the easiest arithmetic mistake available here. The
      subtraction exists because two host addresses are spoken for — the
      network address and the broadcast address — and that reasoning applies
      only when you are counting hosts. Counting how many subnets a set of
      network bits yields has no such reservation, so nothing is subtracted.
      The distinction has nothing to do with whether the addressing is
      classful, where the prefix falls relative to /24, or whether anything on
      the segment is actually sending broadcasts; the broadcast address is
      reserved regardless of use.
    learn: { slug: "subnetting", anchor: "counting-hosts-and-networks" }

  - stem: "A host with address 10.10.10.10 and mask 255.255.255.0 wants to send to 10.10.20.5. What does it do with the mask, and what does it conclude?"
    options:
      - "It ANDs both addresses with the mask, gets 10.10.10.0 and 10.10.20.0, and sends the packet to its default gateway"
      - "It ANDs both addresses with the mask, gets the same result, and delivers directly on the local segment"
      - "It compares the addresses digit by digit, finds the first two octets match, and delivers directly"
      - "It ORs the destination with the mask to produce the broadcast address and sends there"
    answer: 0
    explanation: >
      The AND test is applied twice, once to each address, and the results are
      compared. Here they differ — 10.10.10.0 against 10.10.20.0 — so the
      destination is not local and the packet goes to the default gateway. The
      second option gets the mechanism right and the arithmetic wrong; with a
      /24 the third octet is part of the network, so 10 and 20 are different
      networks. Comparing octets by eye and stopping at two happens to be how
      people guess and is exactly what the mask exists to replace. ORing with
      the mask is a different operation used to derive a broadcast address, not
      to decide locality.
    learn: { slug: "subnetting", anchor: "subnet-masks" }

  - stem: "Which of these is not a valid subnet mask?"
    options:
      - "255.255.255.128"
      - "255.255.240.0"
      - "255.255.255.192"
      - "255.255.253.0"
    answer: 3
    explanation: >
      A mask is a run of 1s followed by a run of 0s, with no mixing. 253 is
      11111101 in binary — a 0 with a 1 after it — so it cannot be a mask, and
      an interface configured with it will be rejected or will behave
      unpredictably. The other three are clean: 128 is 10000000, 192 is
      11000000, and 240 is 11110000, each a valid boundary. This is the
      quickest reason to be able to convert an octet in your head, since a bad
      mask is invisible in decimal and obvious in binary.
    learn: { slug: "subnetting", anchor: "subnet-masks" }

  - stem: "What does writing 10.10.10.10/24 give you that writing 10.10.10.10 with mask 255.255.255.0 does not?"
    options:
      - "A different network/host boundary, since prefix lengths and masks are not equivalent"
      - "The same information more compactly, and without any reference to an address class"
      - "Automatic route aggregation, which dotted masks cannot express"
      - "A boundary that a router can change dynamically, which a fixed mask cannot"
    answer: 1
    explanation: >
      The two notations carry identical information; /24 and 255.255.255.0 are
      the same 24 leading 1s. What CIDR changed was that the boundary is stated
      explicitly rather than inferred from the first octet, so any prefix
      length is legal and a network can be sized to what it needs. Aggregation
      is something CIDR makes possible, but it is a separate operation that has
      to be configured — the notation does not perform it. And nothing about
      prefix notation makes a boundary dynamic; it is as fixed as a dotted mask.
    learn: { slug: "subnetting", anchor: "cidr" }

  - stem: "Can 10.1.0.0/16 and 10.2.0.0/16 be advertised as a single aggregate route?"
    options:
      - "Yes — they are adjacent, so 10.1.0.0/15 covers both"
      - "Yes — any two /16s inside the same /8 aggregate to a /15"
      - "No — they are adjacent, but a /15 starting at 10.0.0.0 covers 10.0 and 10.1, and no /15 boundary contains 10.1 and 10.2"
      - "No — aggregation only works between blocks with different prefix lengths"
    answer: 2
    explanation: >
      Adjacency is necessary and not sufficient. An aggregate has to start on a
      boundary that is a multiple of its own size, which for a /15 means an even
      second octet. That puts 10.0 with 10.1 and 10.2 with 10.3, so the pair in
      the question straddles a boundary and cannot be combined. The first
      option invents a /15 starting at 10.1, which is not a legal /15. The
      second generalises adjacency into a rule that does not hold. And the last
      option has aggregation backwards: it combines blocks of the same size
      into a shorter prefix.
    learn: { slug: "subnetting", anchor: "cidr" }

  - stem: "You are dividing 10.10.10.0/24 for three segments needing 45, 25, and 10 hosts. Why allocate the largest first?"
    options:
      - "Because the largest segment is usually the most important and should get the lowest addresses"
      - "Because smaller subnets cannot be placed after larger ones without renumbering"
      - "Because allocating smallest first leaves the larger blocks without a correctly aligned boundary to start on"
      - "Because routers process routes in descending order of prefix length"
    answer: 2
    explanation: >
      Each block has to begin on a multiple of its own size, which is the same
      alignment rule that governs aggregation, applied in the other direction.
      Take the 10-host segment first and it consumes a /28 at .0 through .15;
      the next legal /26 boundary is then .64, and the space between is
      stranded. Largest-first packs each block against the previous one with no
      gaps. Importance has nothing to do with it, small-after-large is exactly
      the order that works, and route processing order is a routing table
      matter rather than an allocation constraint.
    learn: { slug: "subnetting", anchor: "variable-length-subnet-masking" }

  - stem: "A segment needs 45 usable addresses. What is the shortest prefix that is not wasteful — that is, the smallest block that fits?"
    options:
      - "/25, giving 126 usable"
      - "/27, giving 30 usable"
      - "/24, giving 254 usable"
      - "/26, giving 62 usable"
    answer: 3
    explanation: >
      Work up the powers of two until one clears the requirement after
      subtracting the network and broadcast addresses. A /27 gives 30, which is
      short of 45, so it is out regardless of how close it looks. A /26 gives
      62, which fits with room to grow. A /25 fits too, but it takes twice the
      space to hold the same 45 hosts, and a /24 wastes the whole block. The
      point of variable length masking is picking the smallest block that
      actually works rather than the first one that does.
    learn: { slug: "subnetting", anchor: "variable-length-subnet-masking" }

  - stem: "Beyond conserving addresses, what does dividing a network into subnets buy you?"
    options:
      - "A natural boundary at which to enforce policy between systems with no reason to talk, and a bound on broadcast traffic"
      - "Encryption between subnets, since traffic crossing a router is protected in transit"
      - "Automatic failover, because a router will reroute around a failed subnet"
      - "Higher link speed within each subnet, because there is less contention on the wire"
    answer: 0
    explanation: >
      Subnet boundaries are where a router sits, and a router is somewhere a
      filtering policy can be applied — which is why segmentation is a security
      design tool and not only an addressing one. Bounding broadcast traffic
      falls out of the same boundary. Nothing about routing encrypts anything;
      that requires a protocol built for it. Routers do reroute around failed
      paths, but that is a routing property rather than something subnetting
      provides. And link speed is a property of the media and the switching,
      unaffected by how the address space is divided.
    learn: { slug: "subnetting", anchor: "variable-length-subnet-masking" }

  - stem: "You ping your network's broadcast address and get two replies on a segment you know has a dozen hosts. What should you conclude?"
    options:
      - "Ten hosts are down or unplugged"
      - "The broadcast address was calculated incorrectly, since a correct one reaches everything"
      - "Little — many systems ignore broadcast pings, so a quiet result is not proof of an empty network"
      - "The router is filtering broadcast traffic within the segment"
    answer: 2
    explanation: >
      Responding to a broadcast echo request is optional and widely disabled,
      so the count of replies is a lower bound on live hosts and nothing more.
      Treating ten silent hosts as down would be exactly the overreading the
      technique invites. A wrong broadcast address usually produces no replies
      rather than two, so getting some back argues the arithmetic was right.
      And routers bound broadcast domains at the subnet edge — they do not
      filter broadcasts within a segment, which never reach them in the first
      place.
    learn: { slug: "subnetting", anchor: "suggested-practice-verify-a-subnet-by-hand-then-check-yourself" }

  - stem: "Given the reason for learning the arithmetic by hand, what is the argument for using `ipcalc` in day-to-day work?"
    options:
      - "There is none — hand calculation is the professional standard and tools are a crutch"
      - "The manual method exists to make a wrong mask recognisable on sight; once you have that, the tool is faster and does not misread"
      - "The tool handles VLSM, which cannot be worked out by hand"
      - "The tool is required because prefix lengths above /24 have no dotted-mask equivalent"
    answer: 1
    explanation: >
      The skill being trained is recognition, not calculation speed. Once a
      wrong mask looks wrong to you, the arithmetic itself is a job for a tool
      that will not slip. The purist answer gets this backwards and produces
      slower, less reliable work. VLSM is entirely hand-workable — that is
      exactly what the largest-first exercise is. And every prefix length has a
      dotted-mask equivalent; /25 is 255.255.255.128 and so on up.
    learn: { slug: "subnetting", anchor: "suggested-practice-verify-a-subnet-by-hand-then-check-yourself" }
---
