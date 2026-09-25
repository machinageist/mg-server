---
entries:

  - term: "802.1Q"
    category: networking
    definition: >
      The IEEE bridging standard that defines the four-byte VLAN tag. The tag
      sits after the source MAC address, where the EtherType would otherwise
      be, and its first two bytes are always `0x8100`. Spanning tree was later
      folded into the same standard.
    see_also: ["vlan", "trunk-port", "spanning-tree-protocol"]
    learn:
      - { slug: "switching-technologies", anchor: "802-1q-tagging", label: "Switching technologies" }

  - term: "802.1X"
    category: networking
    definition: >
      The IEEE standard for port-based network access control. A device gets
      onto a wired or wireless network only after it authenticates. The
      client is the supplicant, the switch port or access point is the
      authenticator, and a RADIUS server usually makes the decision.
    see_also: ["extensible-authentication-protocol", "radius"]
    learn:
      - { slug: "wireless-technologies", anchor: "802-1x-and-eap", label: "Wireless technologies" }

  - term: "Access port"
    category: networking
    definition: >
      A switch port assigned to exactly one VLAN. Frames leaving it toward an
      end device carry no VLAN tag, so the device need not know it is in a
      VLAN at all.
    see_also: ["vlan", "trunk-port"]
    learn:
      - { slug: "switching-technologies", anchor: "access-ports-and-the-vlan-database", label: "Switching technologies" }

  - term: "Ad hoc network"
    aka: ["IBSS", "Independent basic service set"]
    category: networking
    definition: >
      A temporary wireless network where devices connect directly to each
      other, peer to peer, with no access point. Wi-Fi Direct fills the same
      role on phones today.
    see_also: ["wireless-access-point"]
    learn:
      - { slug: "wireless-technologies", anchor: "ad-hoc-networks", label: "Wireless technologies" }

  - term: "Administrative distance"
    aka: ["AD"]
    category: networking
    definition: >
      A local trust ranking a router assigns to routes learned from different
      sources. When the same prefix is available from multiple protocols, the
      lower administrative distance is preferred; it does not measure link speed.
    see_also: ["cidr", "subnet-mask"]
    learn:
      - { slug: "routing-technologies", anchor: "how-a-router-selects-a-route", label: "Routing technologies and route selection" }

  - term: "Anycast"
    category: networking
    definition: >
      One address advertised from several locations at once, with the routing
      system delivering each packet to whichever instance is nearest. Used to
      put a service close to its callers without giving it a different address
      in each place.
    see_also: ["multicast", "unicast"]
    learn:
      - { slug: "traffic-types", anchor: "anycast", label: "Network traffic types" }

  - term: "Archiving"
    category: linux
    definition: >
      Bundling many files into one, which `tar` does. Distinct from
      compression, which makes a file smaller — the two are separate tools on
      Linux, which is why the canonical filename is `archive.tar.gz`.
    see_also: ["compression"]
    learn:
      - { slug: "linux-archives", anchor: "tar-bundles-many-files", label: "Archives and compression" }

  - term: "Authentication Header"
    aka: ["AH"]
    category: networking
    definition: >
      The IPsec protocol providing integrity and data-origin authentication
      for protected parts of a packet, with no confidentiality.
    see_also: ["ipsec", "encapsulating-security-payload"]
    learn:
      - { slug: "network-functions", anchor: "ipsec", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Autonomous system"
    aka: ["AS"]
    category: networking
    definition: >
      A network or group of networks operated under one administrative routing
      policy and identified by an autonomous system number. BGP exchanges routes
      between autonomous systems.
    see_also: ["autonomous-system"]
    learn:
      - { slug: "routing-technologies", anchor: "bgp", label: "Routing technologies and route selection" }

  - term: "Autonomous system number"
    aka: ["ASN"]
    category: networking
    definition: >
      The number identifying an autonomous system for routing policy and BGP
      path exchange.
    see_also: ["autonomous-system"]
    learn:
      - { slug: "routing-technologies", anchor: "bgp", label: "Routing technologies and route selection" }

  - term: "Band steering"
    category: networking
    definition: >
      An access point nudging a dual-band client toward the better band,
      usually 5 GHz, by delaying its 2.4 GHz replies or asking the client to
      move. The client still makes the final decision.
    see_also: ["channel-width"]
    learn:
      - { slug: "wireless-technologies", anchor: "band-steering", label: "Wireless technologies" }

  - term: "Bastion host"
    category: networking
    definition: >
      A single, hardened, logged entry point into a network zone. It is only a
      bastion if it is the *only* path in; if the zone is reachable around it,
      it is a jump box with extra steps.
    learn:
      - { slug: "zero-trust-architecture", anchor: "the-policy-components", label: "Zero-trust architecture" }

  - term: "Border Gateway Protocol"
    aka: ["BGP"]
    category: networking
    definition: >
      The inter-domain routing protocol that exchanges reachability between
      autonomous systems and selects paths using policy and path attributes.
    see_also: ["autonomous-system"]
    learn:
      - { slug: "routing-technologies", anchor: "bgp", label: "Routing technologies and route selection" }

  - term: "Bounded media"
    category: networking
    definition: >
      Media confining a signal to a physical path — a copper pair, a glass
      core. The path is private and measurable, and a station can listen to it
      while transmitting, which is what made collision detection possible.
    see_also: ["unbounded-media", "csma-cd"]
    learn:
      - { slug: "transmission-media", anchor: "bounded-and-unbounded-media", label: "Transmission media" }

  - term: "Bridge protocol data unit"
    aka: ["BPDU"]
    category: networking
    definition: >
      The frame switches exchange to elect a root bridge and detect topology
      changes. A blocked port still receives them, which is how it learns that
      a path it is standing in for has failed.
    see_also: ["spanning-tree-protocol", "root-bridge"]
    learn:
      - { slug: "switching-technologies", anchor: "spanning-tree-protocol", label: "Switching technologies" }

  - term: "Broadcast"
    category: networking
    definition: >
      Delivery to every host on a segment at once. IPv4 has it; IPv6 removed
      it entirely and uses multicast groups instead, which is why an IPv6
      network is quieter than an IPv4 one of the same size.
    see_also: ["multicast", "unicast"]
    learn:
      - { slug: "traffic-types", anchor: "broadcast", label: "Network traffic types" }

  - term: "Broadcast domain"
    category: networking
    definition: >
      The set of devices a broadcast frame reaches. A VLAN is one, and a
      router bounds one. Dividing a network into smaller broadcast domains is
      most of why VLANs exist.
    see_also: ["vlan", "broadcast", "collision-domain"]
    learn:
      - { slug: "switching-technologies", anchor: "vlans", label: "Switching technologies" }

  - term: "BSSID"
    aka: ["Basic service set identifier"]
    category: networking
    definition: >
      The identifier of one access point radio, usually its MAC address.
      Several access points can share an SSID, but each has its own BSSID,
      and that is how a client tells them apart.
    see_also: ["ssid", "extended-service-set"]
    learn:
      - { slug: "wireless-technologies", anchor: "bssid", label: "Wireless technologies" }

  - term: "Captive portal"
    category: networking
    definition: >
      A login page that a new client's web traffic is sent to before it gets
      access, for signing in or accepting terms of service. It controls
      access but does not encrypt anything.
    see_also: ["wpa3"]
    learn:
      - { slug: "wireless-technologies", anchor: "guest-networks-and-captive-portals", label: "Wireless technologies" }

  - term: "Channel width"
    category: networking
    definition: >
      How much spectrum a Wi-Fi channel takes up: 20, 40, 80, 160, or 320
      MHz. Wider channels carry more data but overlap neighbors more easily
      and pick up more interference.
    see_also: ["non-overlapping-channels"]
    learn:
      - { slug: "wireless-technologies", anchor: "channel-width", label: "Wireless technologies" }

  - term: "CIDR"
    aka: ["Classless Inter-Domain Routing"]
    category: networking
    definition: >
      Writing the network/host boundary as an explicit prefix length — the /24
      in 203.0.113.0/24 — instead of inferring it from an address class. It
      replaced classful addressing in the 1990s and is what every current
      router actually uses.
    see_also: ["subnet-mask", "supernetting"]
    learn:
      - { slug: "subnetting", anchor: "cidr", label: "Subnetting, CIDR, and VLSM" }

  - term: "Collapsed core"
    category: networking
    definition: >
      A design merging the core and distribution layers into one tier, common
      where traffic does not justify a separate core. The merged tier carries
      both roles, so losing it takes more of the network with it.
    see_also: ["three-tier-hierarchical-model"]
    learn:
      - { slug: "network-topologies", anchor: "collapsed-core", label: "Network topologies" }

  - term: "Collision domain"
    category: networking
    definition: >
      The region of a network where two transmissions can collide. Switched
      full-duplex links give each port its own and remove the contention
      entirely, which is why CSMA/CD is legacy behavior rather than an active
      mechanism.
    see_also: ["csma-cd", "hub", "broadcast-domain", "duplex"]
    learn:
      - { slug: "wired-media", anchor: "collisions-on-a-shared-cable", label: "Wired media" }

  - term: "Compression"
    category: linux
    definition: >
      Making a file smaller, which `gzip` does to one stream of bytes. It does
      not bundle, so `gzip` on a directory does nothing useful; that is
      `tar`'s job.
    see_also: ["archiving"]
    learn:
      - { slug: "linux-archives", anchor: "gzip-compresses-one-file", label: "Archives and compression" }

  - term: "Container"
    category: networking
    definition: >
      A package of an application and its dependencies that shares the host
      kernel. Lighter and faster to start than a VM, with less isolation for
      the same reason.
    see_also: ["virtual-machine", "network-function-virtualization"]
    learn:
      - { slug: "cloud-computing", anchor: "network-function-virtualization", label: "Cloud computing concepts for networking" }

  - term: "Content delivery network"
    aka: ["CDN"]
    category: networking
    definition: >
      Servers in many locations delivering content from a place suited to the
      user, sitting between an application's origin and its users. It serves
      an application; it is not the application.
    see_also: ["origin-server", "reverse-proxy"]
    learn:
      - { slug: "network-applications", anchor: "overview", label: "Network applications: content delivery networks" }

  - term: "Context switch"
    category: linux
    definition: >
      Moving a CPU from running one process to running another: the kernel
      saves the outgoing process's state, picks the next one, sets its time
      slice, and returns to user mode. Fast enough that every program appears
      to run at once.
    see_also: ["kernel", "time-slice"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "process-management", label: "Linux abstraction layers" }

  - term: "CSMA/CA"
    aka: ["Carrier-sense multiple access with collision avoidance"]
    category: networking
    definition: >
      The access method for Wi-Fi: listen, transmit when clear, and treat a
      missing acknowledgment as a lost frame. A wireless station cannot hear
      the channel over its own transmission, so it must avoid collisions
      rather than detect them.
    see_also: ["csma-cd", "rts-cts", "wireless-access-point"]
    learn:
      - { slug: "wireless-media", anchor: "taking-turns-on-a-shared-channel", label: "Wireless media" }

  - term: "CSMA/CD"
    aka: ["Carrier-sense multiple access with collision detection"]
    category: networking
    definition: >
      The access method for shared half-duplex Ethernet: listen, transmit when
      quiet, and on a collision back off a random interval before retrying. A
      wired station can hear the cable while transmitting, which is what makes
      detection possible.
    see_also: ["csma-ca", "collision-domain", "hub"]
    learn:
      - { slug: "wired-media", anchor: "collisions-on-a-shared-cable", label: "Wired media" }

  - term: "Default route"
    category: networking
    definition: >
      The route `0.0.0.0/0`, matching every destination on zero bits. Being
      the shortest possible prefix is exactly what makes it a last resort —
      anything more specific beats it.
    see_also: ["prefix-length", "longest-prefix-match", "routing-table"]
    learn:
      - { slug: "routing-technologies", anchor: "prefix-length", label: "Routing technologies and route selection" }

  - term: "Direct attach copper"
    aka: ["DAC", "Twinax"]
    category: networking
    definition: >
      A short copper cable of twinaxial construction with transceiver ends
      permanently attached, common for top-of-rack switch connections a few
      meters long.
    see_also: ["small-form-factor-pluggable", "transceiver"]
    learn:
      - { slug: "wired-media", anchor: "direct-attach-copper", label: "Wired media" }

  - term: "Duplex"
    category: networking
    definition: >
      Whether an interface can send and receive at the same time. Full duplex
      does both and is what every switched link uses; half duplex allows one
      direction at a time. A mismatch leaves the link up but collapses
      throughput under load, which reads like a failing cable.
    see_also: ["collision-domain"]
    learn:
      - { slug: "switching-technologies", anchor: "speed-and-duplex", label: "Switching technologies" }

  - term: "Dynamic Frequency Selection"
    aka: ["DFS"]
    category: networking
    definition: >
      The 802.11h feature that lets Wi-Fi share parts of the 5 GHz band with
      radar. An access point listens before using a DFS channel and has to
      leave it if it detects radar.
    see_also: ["channel-width"]
    learn:
      - { slug: "wireless-technologies", anchor: "802-11h-dfs-and-tpc", label: "Wireless technologies" }

  - term: "Dynamic routing"
    category: networking
    definition: >
      Routing in which routers exchange reachability information and recalculate
      paths as the topology changes, using a routing protocol.
    see_also: ["static-routing"]
    learn:
      - { slug: "routing-technologies", anchor: "static-and-dynamic-routing", label: "Routing technologies and route selection" }

  - term: "East-west traffic"
    category: networking
    definition: >
      Traffic moving within a network, between servers or between devices in
      one environment. Its growth is what drove data center design from the
      three-tier model toward spine-and-leaf.
    see_also: ["north-south-traffic", "spine-and-leaf"]
    learn:
      - { slug: "network-topologies", anchor: "traffic-flow-north-south-and-east-west", label: "Network topologies" }

  - term: "Elasticity"
    category: networking
    definition: >
      Automatic, near-real-time scaling in response to current demand.
      Distinct from scalability, which is the ability to add or remove
      resources at all and may be planned in advance.
    see_also: ["multitenancy"]
    learn:
      - { slug: "cloud-computing", anchor: "scalability-elasticity-and-multitenancy", label: "Cloud computing concepts for networking" }

  - term: "Electromagnetic interference"
    aka: ["EMI"]
    category: networking
    definition: >
      Electrical noise picked up by a copper conductor from motors, lighting,
      or power runs. Fiber is immune to it, though not indifferent to bend
      radius, dirty endfaces, or accumulated loss.
    see_also: ["single-mode-fiber", "multimode-fiber"]
    learn:
      - { slug: "wired-media", anchor: "fiber-ethernet-and-om-grades", label: "Wired media" }

  - term: "Encapsulating Security Payload"
    aka: ["ESP"]
    category: networking
    definition: >
      The IPsec protocol that can provide confidentiality, integrity, data-
      origin authentication, and anti-replay protection. The usual choice when
      encryption is needed.
    see_also: ["ipsec", "authentication-header"]
    learn:
      - { slug: "network-functions", anchor: "ipsec", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Encapsulation"
    category: networking
    definition: >
      Each layer wrapping the data it receives from the layer above with its
      own header, and sometimes a trailer, before handing it down. The receiver
      unwraps it in reverse. This is why the words segment, packet, and frame
      describe the same data at different depths.
    see_also: ["protocol-data-unit"]
    learn:
      - { slug: "osi-model", anchor: "encapsulation-and-decapsulation", label: "The OSI model" }

  - term: "Enhanced Interior Gateway Routing Protocol"
    aka: ["EIGRP"]
    category: networking
    definition: >
      An interior gateway protocol that uses a composite metric and DUAL to
      calculate loop-free paths within an organization.
    see_also: ["dynamic-routing"]
    learn:
      - { slug: "routing-technologies", anchor: "eigrp", label: "Routing technologies and route selection" }

  - term: "Environment variable"
    category: linux
    definition: >
      A shell variable marked for inheritance by child processes, using
      `export`. It does not write anything to disk and is not permanent —
      permanence comes from a startup file setting it again each time a shell
      starts.
    see_also: ["shell-variable", "path"]
    learn:
      - { slug: "linux-shell", anchor: "shell-and-environment-variables", label: "The shell and the command line" }

  - term: "Ephemeral port"
    category: networking
    definition: >
      A short-lived source port a client picks from the dynamic range,
      49152–65535, for the life of one connection.
    see_also: ["well-known-port"]
    learn:
      - { slug: "network-protocols", anchor: "port-number-ranges", label: "Network protocols and ports" }

  - term: "Extended service set"
    aka: ["ESS", "ESSID"]
    category: networking
    definition: >
      A group of access points that share one SSID and connect to the same
      wired network. The shared name is the ESSID. It is what lets a client
      roam between access points without joining a new network.
    see_also: ["ssid", "bssid"]
    learn:
      - { slug: "wireless-technologies", anchor: "ess-and-essid", label: "Wireless technologies" }

  - term: "Extensible Authentication Protocol"
    aka: ["EAP", "EAP-TLS", "PEAP", "EAP-TTLS", "EAP-FAST"]
    category: networking
    definition: >
      The authentication framework that 802.1X carries. EAP is not a single
      method. EAP-TLS, PEAP, EAP-TTLS, and EAP-FAST each decide differently
      what the client and the server have to prove.
    see_also: ["802-1x", "radius"]
    learn:
      - { slug: "wireless-technologies", anchor: "802-1x-and-eap", label: "Wireless technologies" }

  - term: "Fibre Channel"
    category: networking
    definition: >
      A high-speed transport built for storage traffic, most often in a
      storage area network, running predominantly over fiber. Fibre Channel
      Protocol is the mapping carrying SCSI commands across it, not another
      name for the transport.
    see_also: ["storage-area-network", "transceiver"]
    learn:
      - { slug: "transceivers", anchor: "fibre-channel", label: "Transceivers and connectors" }

  - term: "File descriptor"
    category: linux
    definition: >
      The small integer a process uses to refer to an open stream. Every
      process starts with three: 0 for standard input, 1 for standard output,
      2 for standard error. Redirection is the shell repointing them before
      the program runs.
    see_also: ["standard-streams"]
    learn:
      - { slug: "linux-streams", anchor: "the-three-standard-streams", label: "Streams, redirection, and pipes" }

  - term: "First-hop redundancy protocol"
    aka: ["FHRP"]
    category: networking
    definition: >
      A protocol that lets multiple gateways present a shared virtual IP so a
      standby router can take over when the active gateway fails.
    see_also: ["virtual-ip"]
    learn:
      - { slug: "routing-technologies", anchor: "first-hop-redundancy-and-virtual-ips", label: "Routing technologies and route selection" }

  - term: "Floating static route"
    category: networking
    definition: >
      A backup static route given an administrative distance above the routing
      protocol's, so it stays out of the table until the protocol's route
      disappears and then takes over.
    see_also: ["static-routing", "administrative-distance"]
    learn:
      - { slug: "routing-technologies", anchor: "administrative-distance", label: "Routing technologies and route selection" }

  - term: "Forward proxy"
    category: networking
    definition: >
      A proxy representing clients reaching external services, commonly to log
      requests, enforce URL or content policy, or cache responses.
    see_also: ["reverse-proxy"]
    learn:
      - { slug: "network-appliances", anchor: "proxy-servers", label: "Network appliances" }

  - term: "Geostationary orbit"
    aka: ["GEO"]
    category: networking
    definition: >
      An orbit at roughly 35,000 km holding a fixed position over one point on
      the ground. One satellite covers a large area and the dish need not
      move, at the cost of substantial round-trip latency.
    see_also: ["low-earth-orbit"]
    learn:
      - { slug: "wireless-media", anchor: "satellite", label: "Wireless media" }

  - term: "Hextet"
    category: networking
    definition: >
      One of the eight 16-bit groups an IPv6 address is written in, four hex
      digits each. The IPv6 counterpart to IPv4's octet.
    see_also: ["octet"]
    learn:
      - { slug: "ipv6-addressing", anchor: "the-shape-of-an-address", label: "IPv6 addressing" }

  - term: "Hop"
    category: networking
    definition: >
      One router a packet passes through. The frame around the packet is
      rewritten at every hop while the IP addresses stay the same.
    see_also: ["routing-table", "time-to-live"]
    learn:
      - { slug: "routing-technologies", anchor: "overview", label: "Routing technologies and route selection" }

  - term: "Hop limit"
    category: networking
    definition: >
      IPv6's name for the field IPv4 calls TTL, serving the same purpose of
      bounding a packet's path through a routed network.
    see_also: ["time-to-live", "hop"]
    learn:
      - { slug: "network-functions", anchor: "packet-lifetime-ttl-and-hop-limit", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Hub"
    category: networking
    definition: >
      A device that repeats an incoming Ethernet signal to every other port,
      putting all attached devices in one shared collision domain. Half-duplex
      by nature and replaced by switches.
    see_also: ["collision-domain", "csma-cd"]
    learn:
      - { slug: "network-appliances", anchor: "hubs", label: "Network appliances" }

  - term: "Infrastructure as a Service"
    aka: ["IaaS"]
    category: networking
    definition: >
      A cloud service model providing virtual compute, storage, and
      networking, with the customer managing the operating system and
      everything above it.
    see_also: ["platform-as-a-service", "software-as-a-service"]
    learn:
      - { slug: "cloud-computing", anchor: "cloud-service-models", label: "Cloud computing concepts for networking" }

  - term: "Inter-VLAN routing"
    category: networking
    definition: >
      Routing traffic between VLANs, which is required because a VLAN stops
      traffic at Layer 2. Done on a Layer 3 switch with SVIs, or on a router
      reached over a trunk. The isolation a VLAN provides ends wherever this
      happens.
    see_also: ["vlan", "switch-virtual-interface", "subinterface"]
    learn:
      - { slug: "switching-technologies", anchor: "inter-vlan-routing-and-switch-virtual-interfaces", label: "Switching technologies" }

  - term: "Interface identifier"
    aka: ["EUI-64"]
    category: networking
    definition: >
      The last 64 bits of an IPv6 address, identifying an interface on its
      link. EUI-64 is one way to build it — derived from the MAC address —
      but modern systems prefer identifiers that are stable per network and
      not derived from hardware, because embedding a serial number in every
      packet is a privacy problem.
    learn:
      - { slug: "ipv6-addressing", anchor: "the-interface-identifier", label: "IPv6 addressing" }

  - term: "Interior gateway protocol"
    aka: ["IGP"]
    category: networking
    definition: >
      A routing protocol running inside one administrative domain, optimizing
      for a best path. OSPF and EIGRP are the common examples. An exterior
      protocol such as BGP runs between domains and optimizes for policy
      instead.
    see_also: ["open-shortest-path-first", "enhanced-interior-gateway-routing-protocol", "border-gateway-protocol", "autonomous-system"]
    learn:
      - { slug: "routing-technologies", anchor: "routing-protocol-families", label: "Routing technologies and route selection" }

  - term: "Internet Key Exchange"
    aka: ["IKE"]
    category: networking
    definition: >
      The protocol that authenticates IPsec peers, negotiates cryptographic
      parameters, and creates the security associations and key material.
      Commonly UDP port 500, or 4500 when traversing NAT.
    see_also: ["ipsec", "security-association"]
    learn:
      - { slug: "network-functions", anchor: "internet-key-exchange", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Intrusion detection system"
    aka: ["IDS"]
    category: networking
    definition: >
      A system that analyzes activity and raises alerts, usually passive with
      respect to the traffic path it observes.
    see_also: ["intrusion-prevention-system"]
    learn:
      - { slug: "network-appliances", anchor: "intrusion-detection-and-prevention", label: "Network appliances" }

  - term: "Intrusion prevention system"
    aka: ["IPS"]
    category: networking
    definition: >
      A system positioned where it can automatically block or alter selected
      traffic. Being inline is the difference from an IDS, and it is also what
      lets a failure in its rules affect availability.
    see_also: ["intrusion-detection-system"]
    learn:
      - { slug: "network-appliances", anchor: "intrusion-detection-and-prevention", label: "Network appliances" }

  - term: "IPsec"
    category: networking
    definition: >
      A standards-based suite protecting traffic at the IP layer, through
      security associations that define how selected traffic is protected.
    see_also: ["security-association", "encapsulating-security-payload", "authentication-header", "internet-key-exchange", "virtual-private-network"]
    learn:
      - { slug: "network-functions", anchor: "ipsec", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Jumbo frame"
    category: networking
    definition: >
      A frame carrying an MTU well above 1,500 bytes, typically around 9,000.
      Fewer, larger frames cut per-frame overhead, but only when every device
      along the path agrees — one device left at 1,500 produces a stalling
      path.
    see_also: ["maximum-transmission-unit"]
    learn:
      - { slug: "switching-technologies", anchor: "jumbo-frames", label: "Switching technologies" }

  - term: "Kernel"
    category: linux
    definition: >
      The core of the operating system: it loads at startup, schedules the
      CPU, manages memory, drives hardware, and services the system calls
      processes make. It owns memory that no user process may touch, which is
      why a misbehaving program can crash itself without taking down the
      machine.
    see_also: ["system-call", "user-space"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "the-kernel", label: "Linux abstraction layers" }

  - term: "Lightweight access point"
    aka: ["LWAP", "Wireless LAN controller", "WLC"]
    category: networking
    definition: >
      An access point managed by a central wireless LAN controller instead
      of being configured on its own. The controller handles its settings,
      security, channel, and power, usually over a CAPWAP tunnel.
    see_also: ["wireless-access-point"]
    learn:
      - { slug: "wireless-technologies", anchor: "autonomous-and-lightweight-access-points", label: "Wireless technologies" }

  - term: "Link aggregation"
    category: networking
    definition: >
      Combining several physical links into one logical link with their
      combined capacity, surviving the loss of any member. The capacity is
      aggregate, not per-flow: each conversation is hashed onto one member so
      its frames stay in order.
    see_also: ["link-aggregation-control-protocol"]
    learn:
      - { slug: "switching-technologies", anchor: "link-aggregation", label: "Switching technologies" }

  - term: "Link Aggregation Control Protocol"
    aka: ["LACP"]
    category: networking
    definition: >
      The protocol that negotiates a link-aggregation bundle with the device
      at the far end rather than assuming one, so a miscabled member is left
      out instead of used. Standardized in IEEE 802.3ad and now held in
      802.1AX.
    see_also: ["link-aggregation"]
    learn:
      - { slug: "switching-technologies", anchor: "link-aggregation", label: "Switching technologies" }

  - term: "Link-local address"
    aka: ["APIPA"]
    category: networking
    definition: >
      An address valid only on one link, with no gateway and no routing. In
      IPv4 it is the 169.254 fallback a host self-assigns when DHCP does not
      answer — which makes it a diagnostic signal. In IPv6 every interface has
      one permanently, and routing protocols use them.
    see_also: ["unique-local-address"]
    learn:
      - { slug: "ipv4-addressing", anchor: "link-local-addresses-when-dhcp-fails", label: "IPv4 addressing" }
      - { slug: "ipv6-addressing", anchor: "address-types", label: "IPv6 addressing" }

  - term: "Load balancer"
    category: networking
    definition: >
      A system distributing incoming traffic across backend servers, using
      health checks to keep an unavailable one out of rotation. Layer 4
      balancing works on transport connections; Layer 7 can use HTTP hosts and
      paths.
    see_also: ["reverse-proxy", "virtual-ip"]
    learn:
      - { slug: "network-appliances", anchor: "load-balancers", label: "Network appliances" }

  - term: "Logical topology"
    category: networking
    definition: >
      How data actually moves between nodes, as opposed to how the cables run.
      Replacing a hub with a switch changes nothing physically and changes
      this completely.
    see_also: ["physical-topology", "topology", "collision-domain"]
    learn:
      - { slug: "network-topologies", anchor: "physical-and-logical-topology", label: "Network topologies" }

  - term: "Logical unit number"
    aka: ["LUN"]
    category: networking
    definition: >
      The identifier for a logical storage unit presented to a host over a
      SAN. It names the unit; it is not the size of an individual read or
      write.
    see_also: ["storage-area-network"]
    learn:
      - { slug: "network-appliances", anchor: "storage-area-networks", label: "Network appliances" }

  - term: "Long Term Evolution"
    aka: ["LTE"]
    category: networking
    definition: >
      The cellular generation that drove the move from voice-centric networks
      to data networks capable of ordinary internet access. 5G improves
      throughput and latency but at shorter range per site, so towers must be
      denser.
    learn:
      - { slug: "wireless-media", anchor: "cellular-networks", label: "Wireless media" }

  - term: "Longest prefix match"
    category: networking
    definition: >
      The first and strongest rule of route selection: of the entries matching
      a destination, the most specific wins. A `/25` learned from a poorly
      trusted protocol still beats a `/24` from a better one.
    see_also: ["prefix-length", "administrative-distance", "metric"]
    learn:
      - { slug: "routing-technologies", anchor: "how-a-router-selects-a-route", label: "Routing technologies and route selection" }

  - term: "Low-earth orbit"
    aka: ["LEO"]
    category: networking
    definition: >
      A much closer orbit that cuts latency considerably. Satellites do not
      hold position, so the ground station tracks them and coverage depends on
      a constellation handing off between its members.
    see_also: ["geostationary-orbit"]
    learn:
      - { slug: "wireless-media", anchor: "satellite", label: "Wireless media" }

  - term: "MAC address table"
    aka: ["CAM table"]
    category: networking
    definition: >
      The table a switch builds by learning source MAC addresses from incoming
      frames and associating them with ports. Entries age out after a period
      without traffic, and a frame for an unknown address is flooded rather
      than dropped.
    see_also: ["broadcast-domain", "vlan"]
    learn:
      - { slug: "network-appliances", anchor: "switches", label: "Network appliances" }

  - term: "Maximum transmission unit"
    aka: ["MTU"]
    category: networking
    definition: >
      The largest payload a link will carry in one unit — 1,500 bytes for
      standard Ethernet. Set too large for some link on the path, the
      correction happens elsewhere: IPv4 routers may fragment, IPv6 routers do
      not, and a filtered ICMP report turns it into a connection that stalls
      on the first large transfer.
    see_also: ["jumbo-frame", "sliding-window"]
    learn:
      - { slug: "switching-technologies", anchor: "frame-size", label: "Switching technologies" }

  - term: "Memory management unit"
    aka: ["MMU"]
    category: linux
    definition: >
      Hardware in modern CPUs that lets the kernel give each process a private,
      contiguous-looking virtual address space regardless of how the physical
      memory is actually arranged.
    see_also: ["kernel", "virtual-memory"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "memory-management", label: "Linux abstraction layers" }

  - term: "Mesh topology"
    category: networking
    definition: >
      An arrangement where each node connects to several others. A full mesh
      connects every node to every other and needs `(n * (n - 1)) / 2` links —
      a count of links, not nodes — which is why partial mesh is the common
      compromise.
    see_also: ["topology", "point-to-point-link"]
    learn:
      - { slug: "network-topologies", anchor: "mesh", label: "Network topologies" }

  - term: "Metric"
    category: networking
    definition: >
      A protocol's own measure for comparing routes it learned itself: OSPF a
      cost from bandwidth, EIGRP a composite of bandwidth and delay, RIP a hop
      count, BGP none at all. Metrics from different protocols are not
      comparable, which is why administrative distance settles that first.
    see_also: ["administrative-distance", "longest-prefix-match"]
    learn:
      - { slug: "routing-technologies", anchor: "metric", label: "Routing technologies and route selection" }

  - term: "Multicast"
    category: networking
    definition: >
      Delivery to a group of interested interfaces rather than to one host or
      to everybody. IPv6 leans on it heavily: neighbour discovery asks a
      solicited-node group rather than shouting at the whole segment, which is
      the structural improvement over ARP.
    see_also: ["broadcast", "unicast", "anycast"]
    learn:
      - { slug: "traffic-types", anchor: "multicast", label: "Network traffic types" }

  - term: "Multimode fiber"
    category: networking
    definition: >
      Fiber with a wider core admitting several light paths at once. They
      arrive at slightly different times — modal dispersion — which limits
      usable distance, making it the practical choice inside buildings and
      data centers.
    see_also: ["single-mode-fiber", "optical-multimode-grade"]
    learn:
      - { slug: "wired-media", anchor: "single-mode-and-multimode-fiber", label: "Wired media" }

  - term: "Multitenancy"
    category: networking
    definition: >
      Several customers sharing the same physical infrastructure while their
      workloads stay logically isolated.
    see_also: ["elasticity", "virtual-private-cloud"]
    learn:
      - { slug: "cloud-computing", anchor: "scalability-elasticity-and-multitenancy", label: "Cloud computing concepts for networking" }

  - term: "NAT gateway"
    category: networking
    definition: >
      A cloud gateway letting systems in a private subnet start outbound
      internet connections without accepting unsolicited inbound ones.
    see_also: ["virtual-private-cloud", "network-address-translation"]
    learn:
      - { slug: "cloud-computing", anchor: "cloud-gateways", label: "Cloud computing concepts for networking" }

  - term: "Native VLAN"
    category: networking
    definition: >
      The one VLAN whose traffic crosses a trunk untagged. Without it, an
      untagged frame arriving on a trunk has nothing identifying it and is
      dropped. Ends that disagree about which VLAN is native pass traffic
      between VLANs silently.
    see_also: ["trunk-port", "vlan", "802-1q"]
    learn:
      - { slug: "switching-technologies", anchor: "the-native-vlan", label: "Switching technologies" }

  - term: "Network address translation"
    aka: ["NAT"]
    category: networking
    definition: >
      Translating an address between private and public address spaces. Port
      address translation extends the idea by distinguishing sessions with ports.
    see_also: ["port-address-translation"]
    learn:
      - { slug: "routing-technologies", anchor: "address-translation", label: "Routing technologies and route selection" }

  - term: "Network function virtualization"
    aka: ["NFV"]
    category: networking
    definition: >
      Replacing purpose-built network hardware with software on general-
      purpose infrastructure. Its parts are the virtualized network function,
      the infrastructure supplying its resources, and the management and
      orchestration that deploys them.
    see_also: ["virtual-machine", "container", "virtual-private-cloud"]
    learn:
      - { slug: "cloud-computing", anchor: "network-function-virtualization", label: "Cloud computing concepts for networking" }

  - term: "Network-attached storage"
    aka: ["NAS"]
    category: networking
    definition: >
      File-level shared storage reached over protocols such as SMB or NFS.
      Clients work with files and directories, and the result may look much
      like a local drive.
    see_also: ["storage-area-network"]
    learn:
      - { slug: "network-appliances", anchor: "network-attached-storage", label: "Network appliances" }

  - term: "Next-generation firewall"
    aka: ["NGFW"]
    category: networking
    definition: >
      A firewall adding application and identity awareness to stateful traffic
      control. More features in one device can simplify operations and can
      equally concentrate failure.
    see_also: ["stateful-firewall", "web-application-firewall"]
    learn:
      - { slug: "network-appliances", anchor: "firewalls", label: "Network appliances" }

  - term: "Non-overlapping channels"
    category: networking
    definition: >
      Wi-Fi channels far enough apart that they do not interfere with each
      other. The 2.4 GHz band has three, channels 1, 6, and 11. The 5 GHz
      band has about 25 in the US.
    see_also: ["channel-width"]
    learn:
      - { slug: "wireless-technologies", anchor: "non-overlapping-channels", label: "Wireless technologies" }

  - term: "North-south traffic"
    category: networking
    definition: >
      Traffic entering or leaving a network. It crosses a trust boundary, so
      the primary concern is usually security policy and inspection.
    see_also: ["east-west-traffic", "three-tier-hierarchical-model"]
    learn:
      - { slug: "network-topologies", anchor: "traffic-flow-north-south-and-east-west", label: "Network topologies" }

  - term: "Octet"
    category: networking
    definition: >
      One of the four 8-bit groups an IPv4 address is written in, each shown
      in decimal from 0 to 255. Called an octet rather than a byte because the
      standards predate the byte being reliably eight bits everywhere.
    see_also: ["hextet"]
    learn:
      - { slug: "ipv4-addressing", anchor: "binary-and-the-shape-of-an-address", label: "IPv4 addressing" }

  - term: "Open Shortest Path First"
    aka: ["OSPF"]
    category: networking
    definition: >
      A link-state interior gateway protocol that builds a topology view and
      selects paths using a cost metric.
    see_also: ["dynamic-routing"]
    learn:
      - { slug: "routing-technologies", anchor: "ospf", label: "Routing technologies and route selection" }

  - term: "Optical multimode grade"
    aka: ["OM"]
    category: networking
    definition: >
      The OM1 through OM5 classes describing the reach a multimode fiber
      supports at a given speed. Treat published figures as nominal: actual
      reach depends on modal bandwidth, wavelength, transceiver, and the loss
      budget of the run.
    see_also: ["multimode-fiber", "single-mode-fiber"]
    learn:
      - { slug: "wired-media", anchor: "fiber-ethernet-and-om-grades", label: "Wired media" }

  - term: "Origin server"
    category: networking
    definition: >
      The authoritative source for an application's content. An edge server is
      a delivery point, not necessarily the source of truth — which matters
      when a request could be failing at DNS, the edge, the edge-to-origin
      connection, or the application.
    see_also: ["content-delivery-network"]
    learn:
      - { slug: "network-applications", anchor: "a-simple-request-path", label: "Network applications: content delivery networks" }

  - term: "PATH"
    category: linux
    definition: >
      The environment variable listing, in order, the directories the shell
      searches for the program a command names. Position matters: a directory
      early in the list can shadow a system command with something else of the
      same name.
    see_also: ["environment-variable"]
    learn:
      - { slug: "linux-shell", anchor: "path", label: "The shell and the command line" }

  - term: "Physical topology"
    category: networking
    definition: >
      The cabling and radio layout — what is plugged into what. It can differ
      completely from how data actually moves: hub-based and switch-based
      stars look identical on a cable diagram and behave nothing alike.
    see_also: ["logical-topology", "topology"]
    learn:
      - { slug: "network-topologies", anchor: "physical-and-logical-topology", label: "Network topologies" }

  - term: "Pipe"
    category: linux
    definition: >
      A connection joining one process's standard output to the next one's
      standard input, with nothing touching the disk. Pipes are what make many
      small single-purpose tools add up to work none of them was written for.
    see_also: ["file-descriptor", "standard-streams"]
    learn:
      - { slug: "linux-streams", anchor: "pipes", label: "Streams, redirection, and pipes" }

  - term: "Platform as a Service"
    aka: ["PaaS"]
    category: networking
    definition: >
      A cloud service model where the provider also manages the operating
      system, runtime, and supporting services, so the customer deploys
      application code rather than servers.
    see_also: ["infrastructure-as-a-service", "software-as-a-service"]
    learn:
      - { slug: "cloud-computing", anchor: "cloud-service-models", label: "Cloud computing concepts for networking" }

  - term: "Plenum space"
    category: networking
    definition: >
      A void used to move air for heating, ventilation, and air conditioning,
      typically above a drop ceiling. Cable run through one must be plenum-
      rated by fire and building code, which is a requirement rather than an
      upgrade.
    learn:
      - { slug: "wired-media", anchor: "plenum-cabling", label: "Wired media" }

  - term: "Point-to-point link"
    category: networking
    definition: >
      A link connecting exactly two nodes, with no shared medium and no
      intermediate nodes — a leased line, or a fiber run between two
      buildings.
    see_also: ["topology", "mesh-topology"]
    learn:
      - { slug: "network-topologies", anchor: "point-to-point", label: "Network topologies" }

  - term: "Policy enforcement point"
    aka: ["PEP"]
    category: networking
    definition: >
      The component sitting in the traffic path that carries out an access
      decision made elsewhere. In the zero-trust model it is deliberately
      separate from the policy engine that decides — deciding and enforcing
      are different jobs.
    see_also: ["bastion-host"]
    learn:
      - { slug: "zero-trust-architecture", anchor: "the-policy-components", label: "Zero-trust architecture" }

  - term: "Port address translation"
    aka: ["PAT"]
    category: networking
    definition: >
      A form of address translation that lets many private hosts share one public
      address by mapping their sessions to distinct port numbers.
    see_also: ["network-address-translation"]
    learn:
      - { slug: "routing-technologies", anchor: "address-translation", label: "Routing technologies and route selection" }

  - term: "Power over Ethernet"
    aka: ["PoE"]
    category: networking
    definition: >
      Carrying electrical power and data over the same Ethernet cabling, used
      for access points, IP cameras, and VoIP phones so they need no separate
      supply.
    see_also: ["wireless-access-point", "voice-vlan"]
    learn:
      - { slug: "network-appliances", anchor: "switches", label: "Network appliances" }

  - term: "Pre-shared key"
    aka: ["PSK"]
    category: networking
    definition: >
      The single password every device uses on a WPA2-Personal network.
      Someone who records a device joining can test guesses against it
      offline, so its length matters.
    see_also: ["wpa2", "simultaneous-authentication-of-equals"]
    learn:
      - { slug: "wireless-technologies", anchor: "wpa2", label: "Wireless technologies" }

  - term: "Prefix length"
    category: networking
    definition: >
      How many leading bits of an address identify the network. A longer
      prefix describes a smaller, more specific range, and route selection
      settles on it first, before trust or metric are consulted at all.
    see_also: ["longest-prefix-match", "cidr", "subnet-mask", "default-route"]
    learn:
      - { slug: "routing-technologies", anchor: "prefix-length", label: "Routing technologies and route selection" }

  - term: "Private VLAN"
    aka: ["PVLAN"]
    category: networking
    definition: >
      A division of one primary VLAN into secondary VLANs whose ports cannot
      all reach each other, while all of them keep the same subnet and default
      gateway. Isolated ports reach only the promiscuous port, community ports
      also reach their own community.
    see_also: ["vlan", "broadcast-domain"]
    learn:
      - { slug: "switching-technologies", anchor: "private-vlans", label: "Switching technologies" }

  - term: "Protected Management Frames"
    aka: ["PMF", "802.11w"]
    category: networking
    definition: >
      Protection against forged Wi-Fi management frames, such as the fake
      deauthentication frames used to knock clients off a network. WPA3
      requires it.
    see_also: ["wpa3"]
    learn:
      - { slug: "wireless-technologies", anchor: "wpa3", label: "Wireless technologies" }

  - term: "Protocol data unit"
    aka: ["PDU"]
    category: networking
    definition: >
      The name for the data at a given layer: bits at the physical layer, a
      frame at the data link layer, a packet at the network layer, a segment
      or datagram at the transport layer. Saying which one you mean says which
      information is available to you.
    see_also: ["encapsulation"]
    learn:
      - { slug: "osi-model", anchor: "encapsulation-and-decapsulation", label: "The OSI model" }

  - term: "Pseudo-filesystem"
    category: linux
    definition: >
      A filesystem with nothing on a disk behind it — the kernel presenting its
      own state through the file interface so that ordinary tools work on it.
      `/proc` and `/sys` are the two you meet first, which is why `cat` works
      on kernel state.
    see_also: ["kernel"]
    learn:
      - { slug: "linux-filesystem-hierarchy", anchor: "proc-and-sys", label: "The Linux filesystem hierarchy" }

  - term: "Quality of service"
    aka: ["QoS"]
    category: networking
    definition: >
      Techniques deciding how devices treat competing traffic — classification
      and marking, priority and scheduling, shaping and policing. A marking
      cannot force the next network to honor it, and none of it creates
      bandwidth that is not there.
    see_also: ["voice-vlan"]
    learn:
      - { slug: "network-functions", anchor: "quality-of-service", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "RADIUS"
    aka: ["Remote Authentication Dial-In User Service"]
    category: networking
    definition: >
      The protocol an access point or switch uses to ask a central server
      whether a user may connect. The server often checks the credentials
      against a directory such as Active Directory.
    see_also: ["802-1x", "extensible-authentication-protocol"]
    learn:
      - { slug: "wireless-technologies", anchor: "enterprise-wireless-authentication", label: "Wireless technologies" }

  - term: "Rapid Spanning Tree Protocol"
    aka: ["RSTP"]
    category: networking
    definition: >
      The faster spanning tree introduced in IEEE 802.1w, converging in
      seconds rather than tens of seconds. It collapses blocking and listening
      into one discarding state, leaving discarding, learning, and forwarding.
    see_also: ["spanning-tree-protocol"]
    learn:
      - { slug: "switching-technologies", anchor: "faster-variants", label: "Switching technologies" }

  - term: "Reverse proxy"
    category: networking
    definition: >
      A proxy representing servers to incoming clients. It can terminate TLS,
      cache, enforce policy, and route to different backends — which also
      moves the trust boundary to wherever it terminates.
    see_also: ["forward-proxy", "load-balancer"]
    learn:
      - { slug: "network-appliances", anchor: "proxy-servers", label: "Network appliances" }

  - term: "Root bridge"
    category: networking
    definition: >
      The switch elected as the common point of reference for a spanning tree.
      Every other switch calculates its lowest-cost path toward it, and links
      off those paths are blocked.
    see_also: ["spanning-tree-protocol", "bridge-protocol-data-unit"]
    learn:
      - { slug: "switching-technologies", anchor: "spanning-tree-protocol", label: "Switching technologies" }

  - term: "Routing table"
    category: networking
    definition: >
      The set of entries a router consults to forward a packet, each pairing a
      destination prefix with a next-hop address, an outgoing interface, or
      both.
    see_also: ["longest-prefix-match", "default-route", "administrative-distance"]
    learn:
      - { slug: "routing-technologies", anchor: "overview", label: "Routing technologies and route selection" }

  - term: "RTS/CTS"
    aka: ["Request to send", "Clear to send"]
    category: networking
    definition: >
      Optional Wi-Fi control frames protecting a longer transmission. The
      sender announces how long it expects to occupy the channel, and the
      receiver's reply tells other stations in range to wait.
    see_also: ["csma-ca"]
    learn:
      - { slug: "wireless-media", anchor: "taking-turns-on-a-shared-channel", label: "Wireless media" }

  - term: "Security association"
    aka: ["SA"]
    category: networking
    definition: >
      A one-way agreement defining how IPsec protects selected traffic,
      including algorithms, keys, and lifetimes. Bidirectional communication
      needs one in each direction.
    see_also: ["ipsec", "internet-key-exchange"]
    learn:
      - { slug: "network-functions", anchor: "ipsec", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Security group"
    category: networking
    definition: >
      Cloud traffic policy attached to a network interface, stateful by
      default so return traffic for an allowed request is allowed
      automatically. Subnet-level constructs such as AWS Network ACLs are
      stateless and need rules in both directions.
    see_also: ["virtual-private-cloud", "stateful-firewall"]
    learn:
      - { slug: "cloud-computing", anchor: "traffic-controls-security-groups-and-access-lists", label: "Cloud computing concepts for networking" }

  - term: "Setuid"
    category: linux
    definition: >
      A permission bit making a program run as its owner rather than as the
      user who launched it. It is how an ordinary user can change their own
      password in a file they cannot write, and it is why an unexpected setuid
      binary is worth investigating.
    see_also: ["umask"]
    learn:
      - { slug: "linux-permissions", anchor: "what-else-you-will-see", label: "File permissions and links" }

  - term: "Shell variable"
    category: linux
    definition: >
      A named value belonging to one shell, set with `NAME=value` and gone when
      that shell exits. It becomes an environment variable — visible to child
      processes — only when exported.
    see_also: ["environment-variable"]
    learn:
      - { slug: "linux-shell", anchor: "shell-and-environment-variables", label: "The shell and the command line" }

  - term: "Simultaneous Authentication of Equals"
    aka: ["SAE", "Dragonfly"]
    category: networking
    definition: >
      The password-based key exchange in WPA3-Personal. Recording a device
      joining gives an attacker nothing to test guesses against offline, and
      it provides forward secrecy.
    see_also: ["pre-shared-key", "wpa3"]
    learn:
      - { slug: "wireless-technologies", anchor: "wpa3", label: "Wireless technologies" }

  - term: "Single-mode fiber"
    category: networking
    definition: >
      Fiber with a core around 9 µm carrying light along essentially one path.
      With no modal dispersion it holds a signal over long distances. It
      carries further because the core is narrow, which is a distance property
      before it is a speed one.
    see_also: ["multimode-fiber", "optical-multimode-grade"]
    learn:
      - { slug: "wired-media", anchor: "single-mode-and-multimode-fiber", label: "Wired media" }

  - term: "Sliding window"
    category: networking
    definition: >
      TCP's flow control: how much data a sender may have in flight before the
      receiver acknowledges it, advertised by the receiver as its buffer
      drains. A different mechanism at a different layer from MTU, and
      adjusting one does not do the other's job.
    see_also: ["maximum-transmission-unit"]
    learn:
      - { slug: "switching-technologies", anchor: "where-the-sliding-window-fits", label: "Switching technologies" }

  - term: "Small form-factor pluggable"
    aka: ["SFP", "QSFP"]
    category: networking
    definition: >
      The dominant family of pluggable transceiver form factors. SFP holds one
      channel per module; the QSFP family carries four lanes, which is what
      makes 40G and 100G practical. Matching the form factor does not
      guarantee a platform accepts the module.
    see_also: ["transceiver", "direct-attach-copper"]
    learn:
      - { slug: "transceivers", anchor: "form-factors", label: "Transceivers and connectors" }

  - term: "Software as a Service"
    aka: ["SaaS"]
    category: networking
    definition: >
      A cloud service model providing a complete application, usually through
      a browser. The provider carries more operational responsibility; the
      customer is still responsible for how its data is used in the service.
    see_also: ["infrastructure-as-a-service", "platform-as-a-service"]
    learn:
      - { slug: "cloud-computing", anchor: "cloud-service-models", label: "Cloud computing concepts for networking" }

  - term: "Spanning Tree Protocol"
    aka: ["STP"]
    category: networking
    definition: >
      The protocol that reduces a physically looped switch topology to a loop-
      free logical one by blocking every link not on a lowest-cost path to the
      root bridge. Without it a broadcast circulates indefinitely, because an
      Ethernet frame has no TTL to expire it.
    see_also: ["root-bridge", "bridge-protocol-data-unit", "broadcast-domain"]
    learn:
      - { slug: "switching-technologies", anchor: "spanning-tree-protocol", label: "Switching technologies" }

  - term: "Spine and leaf"
    category: networking
    definition: >
      A two-tier data center fabric where every leaf switch connects to every
      spine and none connect within their own tier. Every leaf-to-leaf path is
      the same length, so latency is consistent. Attached servers are
      endpoints, not a third tier.
    see_also: ["topology", "east-west-traffic", "three-tier-hierarchical-model"]
    learn:
      - { slug: "network-topologies", anchor: "spine-and-leaf", label: "Network topologies" }

  - term: "SSID"
    aka: ["Service set identifier"]
    category: networking
    definition: >
      The name of a wireless network, up to 32 bytes long. Hiding it does
      not hide the network, because clients that have joined before ask for
      it by name.
    see_also: ["bssid", "extended-service-set"]
    learn:
      - { slug: "wireless-technologies", anchor: "service-set-identifiers", label: "Wireless technologies" }

  - term: "Standard streams"
    category: linux
    definition: >
      The three streams every process starts with: standard input, standard
      output, and standard error. Results go to output and diagnostics to
      error, which is what lets you pipe a command's results somewhere without
      also piping its complaints there.
    see_also: ["file-descriptor", "pipe"]
    learn:
      - { slug: "linux-streams", anchor: "the-three-standard-streams", label: "Streams, redirection, and pipes" }

  - term: "Star topology"
    aka: ["Hub and spoke"]
    category: networking
    definition: >
      An arrangement attaching every node to one central point, through which
      traffic between any two of them passes. The shape of most ordinary local
      networks, and its central device is a single point of failure.
    see_also: ["topology", "physical-topology"]
    learn:
      - { slug: "network-topologies", anchor: "star-or-hub-and-spoke", label: "Network topologies" }

  - term: "Stateful firewall"
    category: networking
    definition: >
      A firewall that tracks connections rather than judging each packet
      alone, so it can recognize a later packet as part of an established
      flow.
    see_also: ["next-generation-firewall", "web-application-firewall"]
    learn:
      - { slug: "network-appliances", anchor: "firewalls", label: "Network appliances" }

  - term: "Static routing"
    category: networking
    definition: >
      Routing in which an administrator explicitly configures a route rather than
      learning it through a routing protocol.
    see_also: ["dynamic-routing"]
    learn:
      - { slug: "routing-technologies", anchor: "static-and-dynamic-routing", label: "Routing technologies and route selection" }

  - term: "Storage area network"
    aka: ["SAN"]
    category: networking
    definition: >
      A dedicated or logically separate network giving servers block-level
      storage. The server sees addressable block devices and puts its own
      filesystem on them, which is the main distinction from a NAS.
    see_also: ["network-attached-storage", "logical-unit-number", "fibre-channel"]
    learn:
      - { slug: "network-appliances", anchor: "storage-area-networks", label: "Network appliances" }

  - term: "Subinterface"
    category: networking
    definition: >
      A logical interface carved out of a physical one, each with its own
      address and VLAN tag, named by a suffix such as `GigabitEthernet0/1.10`.
      Routing between VLANs across a single trunked link this way is called
      router-on-a-stick.
    see_also: ["inter-vlan-routing", "switch-virtual-interface", "trunk-port"]
    learn:
      - { slug: "routing-technologies", anchor: "subinterfaces-and-vlans", label: "Routing technologies and route selection" }
      - { slug: "switching-technologies", anchor: "interface-configuration", label: "Switching technologies" }

  - term: "Subnet mask"
    category: networking
    definition: >
      The value marking where an address stops being network and starts being
      host. In binary it is always a run of ones followed by a run of zeros. A
      host applies it with a bitwise AND to decide whether a destination is
      local or belongs to someone else.
    see_also: ["cidr", "vlsm"]
    learn:
      - { slug: "subnetting", anchor: "subnet-masks", label: "Subnetting, CIDR, and VLSM" }

  - term: "Supernetting"
    category: networking
    definition: >
      Expressing several adjacent networks as one shorter prefix, to keep
      routing tables smaller than the number of allocations. Blocks only
      aggregate when they are contiguous *and* the aggregate starts on a
      multiple of its own size.
    see_also: ["cidr"]
    learn:
      - { slug: "subnetting", anchor: "cidr", label: "Subnetting, CIDR, and VLSM" }

  - term: "Switch virtual interface"
    aka: ["SVI"]
    category: networking
    definition: >
      A logical Layer 3 interface on a switch, one per VLAN, holding the IP
      address that serves as that VLAN's default gateway. It is how a Layer 3
      switch routes between its own VLANs internally.
    see_also: ["vlan", "subinterface", "inter-vlan-routing"]
    learn:
      - { slug: "switching-technologies", anchor: "inter-vlan-routing-and-switch-virtual-interfaces", label: "Switching technologies" }

  - term: "Symbolic link"
    category: linux
    definition: >
      A small file whose contents are a path to something else. Its own
      permission bits are meaningless — access is decided by the target — and
      because it stores a path rather than a reference to the data, deleting
      the target leaves the link pointing at nothing.
    learn:
      - { slug: "linux-permissions", anchor: "symbolic-links", label: "File permissions and links" }

  - term: "System call"
    aka: ["syscall"]
    category: linux
    definition: >
      A request from a user process asking the kernel to do something only the
      kernel can. Opening, reading, and writing files are all system calls, and
      so are `fork()` and `exec()` — which is how every process other than the
      first one comes to exist.
    see_also: ["kernel", "user-space"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "system-calls", label: "Linux abstraction layers" }

  - term: "Three-tier hierarchical model"
    category: networking
    definition: >
      An enterprise design splitting the network by role into core,
      distribution, and access layers, so each can be sized, secured, and
      upgraded on its own terms.
    see_also: ["collapsed-core", "spine-and-leaf", "north-south-traffic"]
    learn:
      - { slug: "network-topologies", anchor: "the-three-tier-hierarchical-model", label: "Network topologies" }

  - term: "Time slice"
    category: linux
    definition: >
      The window of CPU time the kernel gives a process before considering
      whether to run something else. Rotating between processes this way is
      multitasking, and it runs fast enough that a sequence of turns reads as
      simultaneity.
    see_also: ["context-switch"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "process-management", label: "Linux abstraction layers" }

  - term: "Time to live"
    aka: ["TTL"]
    category: networking
    definition: >
      The eight-bit IPv4 header field each forwarding router decreases,
      discarding the packet at zero. Despite the name it bounds hops rather
      than wall-clock time, and it is what stops a routing loop circulating a
      packet forever.
    see_also: ["hop", "hop-limit"]
    learn:
      - { slug: "network-functions", anchor: "packet-lifetime-ttl-and-hop-limit", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Topology"
    category: networking
    definition: >
      The arrangement of a network's nodes and the links between them — what
      paths exist, and what happens when one fails. Worth separating from
      equipment: the same switches can be wired into arrangements with very
      different failure behavior.
    see_also: ["physical-topology", "logical-topology"]
    learn:
      - { slug: "network-topologies", anchor: "overview", label: "Network topologies" }

  - term: "Transceiver"
    category: networking
    definition: >
      A transmitter and receiver in one unit, converting between a device's
      electronics and the medium carrying its traffic. Pluggable modules let
      one switch port be fitted for copper or fiber by changing the module.
    see_also: ["small-form-factor-pluggable", "direct-attach-copper"]
    learn:
      - { slug: "transceivers", anchor: "overview", label: "Transceivers and connectors" }

  - term: "Trunk port"
    category: networking
    definition: >
      A switch port carrying traffic for more than one VLAN, keeping them
      apart by tagging. The link between two of them is a trunk link, which is
      what lets a single cable between switches carry several VLANs.
    see_also: ["vlan", "access-port", "802-1q", "native-vlan"]
    learn:
      - { slug: "switching-technologies", anchor: "trunk-ports", label: "Switching technologies" }

  - term: "Tunneling"
    category: networking
    definition: >
      Encapsulating one packet or protocol as the payload of another.
      Encapsulation alone provides no confidentiality, integrity, or peer
      authentication — those come from cryptography added on top.
    see_also: ["virtual-private-network", "encapsulation", "ipsec"]
    learn:
      - { slug: "network-functions", anchor: "tunneling-and-vpns", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Umask"
    category: linux
    definition: >
      The mask of permission bits removed from newly created files and
      directories. It subtracts from 666 for files and 777 for directories, so
      a new file never gets execute permission no matter what you set — a
      deliberate safety property.
    see_also: ["setuid"]
    learn:
      - { slug: "linux-permissions", anchor: "default-permissions-and-umask", label: "File permissions and links" }

  - term: "Unbounded media"
    category: networking
    definition: >
      Media radiating into shared space, where nothing confines the signal and
      a station cannot hear the channel over its own transmission. That single
      physical fact is why wireless avoids collisions rather than detecting
      them.
    see_also: ["bounded-media", "csma-ca"]
    learn:
      - { slug: "transmission-media", anchor: "bounded-and-unbounded-media", label: "Transmission media" }

  - term: "Unicast"
    category: networking
    definition: >
      One sender, one recipient. The ordinary case, and the pattern almost all
      traffic uses.
    see_also: ["multicast", "broadcast", "anycast"]
    learn:
      - { slug: "traffic-types", anchor: "unicast", label: "Network traffic types" }

  - term: "Unique local address"
    aka: ["ULA"]
    category: networking
    definition: >
      IPv6's counterpart to the RFC 1918 private ranges — addresses that route
      inside a site and never on the internet. A site generates a random
      identifier so that two networks merging later are unlikely to collide.
      Not a security boundary; simply not routed off-site.
    see_also: ["link-local-address"]
    learn:
      - { slug: "ipv6-addressing", anchor: "address-types", label: "IPv6 addressing" }

  - term: "User space"
    category: linux
    definition: >
      The memory the kernel allocates to user processes, and everything running
      in it. The boundary between it and kernel space is the reason a crashing
      program takes only itself down.
    see_also: ["kernel", "system-call"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "user-space", label: "Linux abstraction layers" }

  - term: "Virtual IP"
    aka: ["VIP"]
    category: networking
    definition: >
      An address presented by a service or group of devices rather than belonging
      to one physical interface, commonly used for failover or load distribution.
    see_also: ["first-hop-redundancy-protocol"]
    learn:
      - { slug: "routing-technologies", anchor: "first-hop-redundancy-and-virtual-ips", label: "Routing technologies and route selection" }

  - term: "Virtual machine"
    aka: ["VM"]
    category: networking
    definition: >
      An emulated computer running its own operating system on a hypervisor,
      several of which can share one physical host in isolation from each
      other.
    see_also: ["container", "network-function-virtualization"]
    learn:
      - { slug: "cloud-computing", anchor: "network-function-virtualization", label: "Cloud computing concepts for networking" }

  - term: "Virtual memory"
    category: linux
    definition: >
      An address space that looks private and contiguous to a process
      regardless of how the underlying physical memory is arranged. The MMU is
      the hardware that makes it possible.
    see_also: ["memory-management-unit"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "memory-management", label: "Linux abstraction layers" }

  - term: "Virtual private cloud"
    aka: ["VPC"]
    category: networking
    definition: >
      A logically isolated part of a provider's network for one customer,
      usually holding several subnets separated by exposure. A private subnet
      has no route to the internet gateway, so "private" describes the routing
      design rather than absolute isolation.
    see_also: ["security-group", "nat-gateway", "bastion-host"]
    learn:
      - { slug: "cloud-computing", anchor: "virtual-private-clouds", label: "Cloud computing concepts for networking" }

  - term: "Virtual private network"
    aka: ["VPN"]
    category: networking
    definition: >
      A connection joining users or networks across infrastructure they do not
      control, normally with authentication and encryption. There is no single
      universal VPN protocol; the design depends on the use case and
      environment.
    see_also: ["tunneling", "ipsec"]
    learn:
      - { slug: "network-functions", anchor: "tunneling-and-vpns", label: "Network functions: tunnels, traffic priority, and packet lifetime" }

  - term: "Virtual Router Redundancy Protocol"
    aka: ["VRRP"]
    category: networking
    definition: >
      The open-standard first-hop redundancy protocol, and what Linux
      implementations such as keepalived speak. HSRP and GLBP are the Cisco
      equivalents.
    see_also: ["first-hop-redundancy-protocol", "virtual-ip"]
    learn:
      - { slug: "routing-technologies", anchor: "first-hop-redundancy-and-virtual-ips", label: "Routing technologies and route selection" }

  - term: "VLAN"
    aka: ["Virtual LAN"]
    category: networking
    definition: >
      A logical division of a switch into separate broadcast domains. Ports on
      one switch can sit in different VLANs and ports on different switches in
      the same one. A VLAN is a Layer 2 construct; the subnet usually mapped
      onto it is a Layer 3 one, and they are not the same thing.
    see_also: ["broadcast-domain", "access-port", "trunk-port"]
    learn:
      - { slug: "switching-technologies", anchor: "vlans", label: "Switching technologies" }

  - term: "VLSM"
    aka: ["Variable length subnet masking"]
    category: networking
    definition: >
      Splitting one network into subnets of different sizes to match what each
      segment actually needs. Allocate the largest block first, or the later
      ones have no correctly aligned boundary to start on.
    see_also: ["subnet-mask", "cidr"]
    learn:
      - { slug: "subnetting", anchor: "variable-length-subnet-masking", label: "Subnetting, CIDR, and VLSM" }

  - term: "Voice VLAN"
    category: networking
    definition: >
      A VLAN carrying traffic from IP phones, separate from the data VLAN on
      the same access port. Separation gives voice somewhere for priority
      policy to be applied; it improves call quality only if something on the
      path acts on the marking.
    see_also: ["vlan", "quality-of-service"]
    learn:
      - { slug: "switching-technologies", anchor: "voice-vlans", label: "Switching technologies" }

  - term: "VXLAN"
    category: networking
    definition: >
      Encapsulating layer 2 frames inside UDP and carrying them over a routed
      network, which raises the segment identifier from 12 bits to 24. The
      larger consequence is architectural: two hosts can share a layer 2 domain
      without sharing a physical one.
    learn:
      - { slug: "software-defined-networking", anchor: "vxlan", label: "Software-defined networking" }

  - term: "Web application firewall"
    aka: ["WAF"]
    category: networking
    definition: >
      A firewall inspecting HTTP for application-layer patterns and policy
      violations. It can reduce exposure to some web attacks; it cannot repair
      vulnerable application code.
    see_also: ["stateful-firewall", "next-generation-firewall"]
    learn:
      - { slug: "network-appliances", anchor: "firewalls", label: "Network appliances" }

  - term: "Well-known port"
    category: networking
    definition: >
      A port in the range 0–1023, registered by IANA for long-established
      system services. The ranges are a registration convention rather than a
      technical restriction — a service can still be run on an unusual port.
    see_also: ["ephemeral-port"]
    learn:
      - { slug: "network-protocols", anchor: "port-number-ranges", label: "Network protocols and ports" }

  - term: "Wi-Fi Protected Setup"
    aka: ["WPS"]
    category: networking
    definition: >
      A shortcut for joining a WPA2-Personal network without typing the
      password, usually by pressing a button on the router. The PIN version
      has a design flaw that lets an attacker guess the PIN in hours.
    see_also: ["pre-shared-key"]
    learn:
      - { slug: "wireless-technologies", anchor: "wi-fi-protected-setup", label: "Wireless technologies" }

  - term: "Wireless access point"
    aka: ["AP"]
    category: networking
    definition: >
      The device bridging radio clients onto a wired network. An autonomous AP
      holds its own configuration; in a controller-based design a controller
      coordinates policy and radio management across many.
    see_also: ["power-over-ethernet", "csma-ca", "lightweight-access-point"]
    learn:
      - { slug: "network-appliances", anchor: "wireless-appliances", label: "Network appliances" }
      - { slug: "wireless-technologies", anchor: "autonomous-and-lightweight-access-points", label: "Wireless technologies" }

  - term: "WPA2"
    aka: ["Wi-Fi Protected Access 2"]
    category: networking
    definition: >
      The Wi-Fi security standard from 2004, still common. It encrypts with
      CCMP and 128-bit AES, and uses one shared password in Personal mode or
      per-user logins in Enterprise mode.
    see_also: ["wpa3", "pre-shared-key"]
    learn:
      - { slug: "wireless-technologies", anchor: "wpa2", label: "Wireless technologies" }

  - term: "WPA3"
    aka: ["Wi-Fi Protected Access 3"]
    category: networking
    definition: >
      The 2018 replacement for WPA2. It swaps the pre-shared key exchange
      for SAE, requires Protected Management Frames, and is required in the
      6 GHz band.
    see_also: ["wpa2", "simultaneous-authentication-of-equals", "protected-management-frames"]
    learn:
      - { slug: "wireless-technologies", anchor: "wpa3", label: "Wireless technologies" }

  - term: "Zero-trust architecture"
    aka: ["ZTA"]
    category: networking
    definition: >
      A model that stops treating network location as evidence of
      authorisation. Identity is verified continuously and no position on the
      network grants trust by itself; the practical goal is that one
      compromised host is not a path to everything else.
    see_also: ["policy-enforcement-point"]
    learn:
      - { slug: "zero-trust-architecture", anchor: "overview", label: "Zero-trust architecture" }
---

The glossary's term definitions. Data lives in the frontmatter above; this body
is intentionally empty. See `docs/agent-context/README.md` for the authoring
contract.
