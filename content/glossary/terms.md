---
entries:
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

  - term: "Bastion host"
    category: networking
    definition: >
      A single, hardened, logged entry point into a network zone. It is only a
      bastion if it is the *only* path in; if the zone is reachable around it,
      it is a jump box with extra steps.
    learn:
      - { slug: "zero-trust-architecture", anchor: "the-policy-components", label: "Zero-trust architecture" }

  - term: "Broadcast"
    category: networking
    definition: >
      Delivery to every host on a segment at once. IPv4 has it; IPv6 removed
      it entirely and uses multicast groups instead, which is why an IPv6
      network is quieter than an IPv4 one of the same size.
    see_also: ["multicast", "unicast"]
    learn:
      - { slug: "traffic-types", anchor: "broadcast", label: "Network traffic types" }

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

  - term: "Hextet"
    category: networking
    definition: >
      One of the eight 16-bit groups an IPv6 address is written in, four hex
      digits each. The IPv6 counterpart to IPv4's octet.
    see_also: ["octet"]
    learn:
      - { slug: "ipv6-addressing", anchor: "the-shape-of-an-address", label: "IPv6 addressing" }

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

  - term: "Octet"
    category: networking
    definition: >
      One of the four 8-bit groups an IPv4 address is written in, each shown
      in decimal from 0 to 255. Called an octet rather than a byte because the
      standards predate the byte being reliably eight bits everywhere.
    see_also: ["hextet"]
    learn:
      - { slug: "ipv4-addressing", anchor: "binary-and-the-shape-of-an-address", label: "IPv4 addressing" }

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

  - term: "Pipe"
    category: linux
    definition: >
      A connection joining one process's standard output to the next one's
      standard input, with nothing touching the disk. Pipes are what make many
      small single-purpose tools add up to work none of them was written for.
    see_also: ["file-descriptor", "standard-streams"]
    learn:
      - { slug: "linux-streams", anchor: "pipes", label: "Streams, redirection, and pipes" }

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

  - term: "Virtual memory"
    category: linux
    definition: >
      An address space that looks private and contiguous to a process
      regardless of how the underlying physical memory is arranged. The MMU is
      the hardware that makes it possible.
    see_also: ["memory-management-unit"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "memory-management", label: "Linux abstraction layers" }

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

  - term: "VXLAN"
    category: networking
    definition: >
      Encapsulating layer 2 frames inside UDP and carrying them over a routed
      network, which raises the segment identifier from 12 bits to 24. The
      larger consequence is architectural: two hosts can share a layer 2 domain
      without sharing a physical one.
    learn:
      - { slug: "software-defined-networking", anchor: "vxlan", label: "Software-defined networking" }

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
