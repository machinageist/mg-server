---
entries:

  - name: "cat"
    synopsis:
      - "cat file"
      - "cat -n file"
    category: linux
    purpose: "Write files to standard output."
    context: >
      Named for concatenate — `cat a b` joins two files onto one stream, which
      is the job it was built for. Most of its daily use is reading one short
      file, where it is really a way of putting a file onto stdout so
      something else can have it. `-n` numbers the lines.
    example: "cat /etc/os-release"
    see_also: ["grep", "ls", "gzip"]
    learn:
      - { slug: "linux-streams", anchor: "the-three-standard-streams", label: "Streams, redirection, and pipes" }
      - { slug: "linux-filesystem-hierarchy", anchor: "suggested-practice-read-the-tree-on-your-own-machine", label: "The Linux filesystem hierarchy" }
    man: "https://man7.org/linux/man-pages/man1/cat.1.html"

  - name: "chmod"
    synopsis:
      - "chmod 644 file"
      - "chmod g+r file"
    category: linux
    purpose: "Change a file's permission bits."
    context: >
      Two notations, and both are worth knowing. Symbolic (`g+r`) changes only
      what you name and leaves the rest alone. Numeric (`644`) sets all nine
      bits at once. Reach for symbolic when adjusting one thing, numeric when
      you want a known end state.
    example: "chmod 600 ~/.ssh/id_ed25519"
    caution: "Removing your own read or execute bit is easy and immediately confusing. Numeric notation overwrites every bit, not just the one you were thinking about."
    see_also: ["umask", "ls"]
    learn:
      - { slug: "linux-permissions", anchor: "changing-permissions-with-chmod", label: "File permissions and links" }
    man: "https://man7.org/linux/man-pages/man1/chmod.1.html"

  - name: "cp"
    synopsis:
      - "cp file copy"
      - "cp -a source/ destination/"
    category: linux
    purpose: "Copy files and directories."
    context: >
      Plain `cp` copies content and discards most of what the filesystem knows
      about a file. `-a` preserves ownership, permissions, timestamps, and
      symbolic links, which is what makes the difference between a copy and a
      duplicate.
    example: "cp -a /etc/skel/. ~/"
    caution: "Without `-a`, ownership and timestamps are rewritten to the copying user and the current time."
    see_also: ["ln", "tar", "chmod"]
    learn:
      - { slug: "linux-archives", anchor: "what-survives-and-what-does-not", label: "Archives and compression" }
    man: "https://man7.org/linux/man-pages/man1/cp.1.html"

  - name: "curl"
    synopsis:
      - "curl -sSI https://example.com"
    category: networking
    purpose: "Make an HTTP request and show what comes back."
    context: >
      The fastest way to prove a service answers, and to see the headers it
      sets. `-I` fetches headers only, `-s` silences the progress meter, and
      `-S` puts errors back after `-s` removed them.
    example: "curl -sS -o /dev/null -w '%{http_code} %{time_total}s\\n' https://example.com"
    see_also: ["dig", "ss"]
    learn:
      - { slug: "osi-model", anchor: "suggested-practice-map-one-web-request", label: "The OSI model" }
    man: "https://man7.org/linux/man-pages/man1/curl.1.html"

  - name: "dig"
    synopsis:
      - "dig +short example.com"
    category: networking
    purpose: "Query DNS and show the answer."
    context: >
      Use it rather than `ping` when the question is about name resolution.
      `+short` gives just the answer; `@resolver` asks a specific server, which
      is how you tell "DNS is broken" apart from "this resolver is broken".
    example: "dig +short @9.9.9.9 example.com"
    see_also: ["curl"]
    learn:
      - { slug: "network-protocols", anchor: "common-application-protocols", label: "Network protocols and ports" }
    man: "https://man7.org/linux/man-pages/man1/dig.1.html"

  - name: "ethtool"
    synopsis:
      - "ethtool eth0"
      - "ethtool -m eth0"
    category: networking
    purpose: "Read and set an interface's link settings."
    context: >
      The layer-1 instrument. It reports negotiated speed and duplex, which is
      how you catch a duplex mismatch pretending to be a failing cable. `-m`
      dumps a pluggable module's EEPROM — vendor, part number, wavelength,
      rated reach — where the driver supports it.
    example: "ethtool eth0 | grep -E 'Speed|Duplex'"
    caution: "Setting speed or duplex by hand is how most mismatches are created. Read first, and change only when you control both ends."
    see_also: ["ip", "tcpdump"]
    learn:
      - { slug: "wired-media", anchor: "suggested-practice-read-the-media-in-your-own-links", label: "Wired media" }
      - { slug: "transceivers", anchor: "suggested-practice-identify-what-is-plugged-into-your-own-gear", label: "Transceivers and connectors" }
      - { slug: "switching-technologies", anchor: "speed-and-duplex", label: "Switching technologies" }
    man: "https://man7.org/linux/man-pages/man8/ethtool.8.html"

  - name: "free"
    synopsis:
      - "free -h"
    category: linux
    purpose: "Show physical memory and swap use."
    context: >
      Reads its numbers from `/proc/meminfo`. The `available` column is the one
      to look at — Linux deliberately uses spare memory for cache, so a small
      `free` figure is normal rather than a problem.
    see_also: ["ps"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "suggested-practice-watch-the-layers-from-user-space", label: "Linux abstraction layers" }
    man: "https://man7.org/linux/man-pages/man1/free.1.html"

  - name: "grep"
    synopsis:
      - "grep -r pattern path"
    category: linux
    purpose: "Find lines matching a pattern."
    context: >
      Most often the middle of a pipeline rather than a command on its own.
      `-r` recurses, `-n` gives line numbers, `-i` ignores case, and `-v`
      inverts the match.
    example: "ps -ef | grep sshd"
    see_also: ["ps"]
    learn:
      - { slug: "linux-streams", anchor: "pipes", label: "Streams, redirection, and pipes" }
    man: "https://man7.org/linux/man-pages/man1/grep.1.html"

  - name: "gzip"
    synopsis:
      - "gzip logfile.txt"
      - "gunzip logfile.txt.gz"
    category: linux
    purpose: "Compress or decompress a single file."
    context: >
      It compresses one stream of bytes; it does not bundle, so running it on
      a directory does nothing useful — that is `tar`'s job. `-k` keeps the
      source, and `-l` reports the ratio without decompressing.
    example: "gzip -k logfile.txt"
    caution: "It replaces the original rather than sitting alongside it. That surprises people exactly once."
    see_also: ["tar", "cat"]
    learn:
      - { slug: "linux-archives", anchor: "gzip-compresses-one-file", label: "Archives and compression" }
    man: "https://man7.org/linux/man-pages/man1/gzip.1.html"

  - name: "ip"
    synopsis:
      - "ip -br address"
      - "ip route"
    category: networking
    purpose: "Show and configure addresses, routes, and interfaces."
    context: >
      The modern replacement for `ifconfig` and `route`. `-br` gives a brief
      one-line-per-interface view, `-4` and `-6` restrict it to one family, and
      `ip -s link` adds error and drop counters — a link can negotiate fine and
      still be marginal.
    example: "ip -6 neigh show"
    see_also: ["ss", "dig"]
    learn:
      - { slug: "ipv4-addressing", anchor: "suggested-practice-read-and-verify-your-own-network", label: "IPv4 addressing" }
      - { slug: "ipv6-addressing", anchor: "suggested-practice-read-your-own-ipv6-configuration", label: "IPv6 addressing" }
    man: "https://man7.org/linux/man-pages/man8/ip.8.html"

  - name: "ipcalc"
    synopsis:
      - "ipcalc 192.0.2.0/26"
    category: networking
    purpose: "Calculate the network, broadcast, and host range for a prefix."
    context: >
      The check on subnetting done by hand. Work the boundaries out yourself
      first, then confirm them here — using it in production is not cheating,
      and using it instead of ever learning the arithmetic is.
    example: "ipcalc 198.51.100.0/27"
    see_also: ["ip"]
    learn:
      - { slug: "subnetting", anchor: "suggested-practice-verify-a-subnet-by-hand-then-check-yourself", label: "Subnetting, CIDR, and VLSM" }
    man: "https://man7.org/linux/man-pages/man1/ipcalc.1.html"

  - name: "iw"
    synopsis:
      - "iw dev"
      - "iw dev wlan0 scan"
    category: networking
    purpose: "Inspect wireless interfaces, and scan for networks."
    context: >
      `iw dev` reports the band, channel, and channel width actually in use,
      which is usually a different story from the Wi-Fi generation on the box.
      Scanning and counting how many networks share your channel explains most
      slow links.
    example: "iw dev wlan0 link"
    caution: "Scanning briefly interrupts the interface's association on some drivers."
    see_also: ["ip", "ethtool"]
    learn:
      - { slug: "wireless-media", anchor: "suggested-practice-watch-a-wireless-link-negotiate", label: "Wireless media" }
    man: "https://man7.org/linux/man-pages/man8/iw.8.html"

  - name: "ln"
    synopsis:
      - "ln -s target linkname"
    category: linux
    purpose: "Create a link to a file or directory."
    context: >
      With `-s` it makes a symbolic link. The argument order is the thing that
      catches people: the target that already exists comes first, the name
      being created second.
    see_also: ["ls"]
    learn:
      - { slug: "linux-permissions", anchor: "symbolic-links", label: "File permissions and links" }
    man: "https://man7.org/linux/man-pages/man1/ln.1.html"

  - name: "ls"
    synopsis:
      - "ls -l"
      - "ls -la"
    category: linux
    purpose: "List directory contents."
    context: >
      `-l` is the form worth reading closely: ten characters of mode, then
      links, owner, group, size, time, and name. `-a` includes dot files, which
      is most of a home directory.
    example: "ls -ld /bin"
    see_also: ["chmod", "ln"]
    learn:
      - { slug: "linux-permissions", anchor: "reading-ls-l", label: "File permissions and links" }
    man: "https://man7.org/linux/man-pages/man1/ls.1.html"

  - name: "man"
    synopsis:
      - "man 5 passwd"
      - 'man -k "disk space"'
    category: linux
    purpose: "Read the manual page for a command, file format, or system call."
    context: >
      Sections matter — the same name can appear in several. Section 1 is user
      commands, 5 is file formats, 8 is administration. `man -k` searches the
      descriptions when you do not know the name. Press `/` to search within a
      page.
    see_also: ["ls"]
    learn:
      - { slug: "linux-shell", anchor: "manual-pages", label: "The shell and the command line" }
    man: "https://man7.org/linux/man-pages/man1/man.1.html"

  - name: "ping"
    synopsis:
      - "ping -c2 host"
    category: networking
    purpose: "Send ICMP echo requests to test reachability."
    context: >
      Proves a host answers ICMP. It says nothing about whether a service is
      listening — use `ss` or `curl` for that. Plenty of hosts drop ICMP by
      policy, so silence is not proof of absence.
    caution: "A quiet ping is weak evidence in both directions. Do not conclude a host is down from it."
    see_also: ["ss", "curl", "ip"]
    learn:
      - { slug: "ipv4-addressing", anchor: "loopback", label: "IPv4 addressing" }
    man: "https://man7.org/linux/man-pages/man8/ping.8.html"

  - name: "ps"
    synopsis:
      - "ps -ef"
      - "ps -ef --forest"
    category: linux
    purpose: "List running processes."
    context: >
      `--forest` draws the parent/child tree, which makes the result of
      `fork()` visible — every process descends from the first one. Usually
      piped into `grep` to find a particular service.
    example: "ps -ef --forest | head -20"
    see_also: ["grep", "free"]
    learn:
      - { slug: "linux-abstraction-layers", anchor: "process-management", label: "Linux abstraction layers" }
    man: "https://man7.org/linux/man-pages/man1/ps.1.html"

  - name: "ss"
    synopsis:
      - "ss -tln"
    category: networking
    purpose: "Show sockets — what is listening, and what is connected."
    context: >
      The replacement for `netstat`. `-t` is TCP, `-l` listening only, `-n`
      skips name resolution so the output is fast and unambiguous. Compare
      services bound to loopback against those bound to a wildcard: the first
      group is reachable only from the machine itself.
    example: "ss -tlnp"
    see_also: ["ip", "ping"]
    learn:
      - { slug: "network-protocols", anchor: "port-number-ranges", label: "Network protocols and ports" }
    man: "https://man7.org/linux/man-pages/man8/ss.8.html"

  - name: "tar"
    synopsis:
      - "tar czf archive.tar.gz dir/"
      - "tar tvf archive.tar.gz"
    category: linux
    purpose: "Bundle many files into one archive, optionally compressed."
    context: >
      The flags are positional and old-fashioned: `c` create, `x` extract, `t`
      list, `v` verbose, `f` naming the file, which must come last of the
      letters. Run `tar tvf` before extracting so you know whether the archive
      expands into a directory or scatters files where you stand.
    example: "zcat archive.tar.gz | tar xvf -"
    caution: "Ownership is restored by numeric ID and needs root to reproduce. ACLs, extended attributes, and SELinux contexts each need asking for explicitly."
    see_also: ["ls"]
    learn:
      - { slug: "linux-archives", anchor: "tar-bundles-many-files", label: "Archives and compression" }
    man: "https://man7.org/linux/man-pages/man1/tar.1.html"

  - name: "tcpdump"
    synopsis:
      - "tcpdump -i eth0 udp port 4789"
    category: networking
    purpose: "Capture and print packets crossing an interface."
    context: >
      The instrument for questions no status command answers — what is actually
      on the wire, and whether an encapsulated frame really carries what you
      think. Needs root, and a filter expression, or it will drown you.
    caution: "Captures may contain credentials and payload data. Use purpose-built fixtures for examples you intend to share."
    see_also: ["ip", "ss"]
    learn:
      - { slug: "software-defined-networking", anchor: "suggested-practice-build-an-overlay-you-can-inspect", label: "Software-defined networking" }
    man: "https://man7.org/linux/man-pages/man8/tcpdump.8.html"

  - name: "traceroute"
    synopsis:
      - "traceroute example.com"
      - "tracepath example.com"
    category: networking
    purpose: "Show the routers along the path to a destination."
    context: >
      It varies the TTL to make each successive router report itself.
      `tracepath` needs no root and also discovers the path MTU. A hop that
      does not reply is a router declining to send ICMP, not proof that
      forwarding stopped there.
    example: "tracepath example.com"
    see_also: ["ping", "ip"]
    learn:
      - { slug: "network-functions", anchor: "suggested-practice-observe-packet-lifetime", label: "Network functions: tunnels, traffic priority, and packet lifetime" }
      - { slug: "routing-technologies", anchor: "suggested-practice-explain-your-own-routing-table", label: "Routing technologies and route selection" }
      - { slug: "network-topologies", anchor: "suggested-practice-map-both-topologies-of-a-network-you-own", label: "Network topologies" }
    man: "https://man7.org/linux/man-pages/man8/traceroute.8.html"

  - name: "type"
    synopsis:
      - "type ls"
      - "type -a python"
    category: linux
    purpose: "Say what the shell will actually run for a name."
    context: >
      The answer to "which one is it": a binary on `PATH`, a shell builtin, a
      function, or an alias. `-a` lists every match in order, which is how you
      find the copy earlier on `PATH` that is shadowing the one you meant.
    example: "type -a ls"
    see_also: ["man", "ls"]
    learn:
      - { slug: "linux-shell", anchor: "suggested-practice-find-out-what-your-shell-is-doing", label: "The shell and the command line" }
      - { slug: "linux-shell", anchor: "path", label: "The shell and the command line" }
    man: "https://man7.org/linux/man-pages/man1/type.1p.html"

  - name: "umask"
    synopsis:
      - "umask 022"
      - "umask 077"
    category: linux
    purpose: "Set the permission bits removed from newly created files."
    context: >
      Subtractive, and from different bases: 666 for files and 777 for
      directories. `022` gives 644 and 755; `077` gives 600 and 700. It applies
      to the shell that runs it, so persistence means putting it in a startup
      file.
    see_also: ["chmod"]
    learn:
      - { slug: "linux-permissions", anchor: "default-permissions-and-umask", label: "File permissions and links" }
    man: "https://man7.org/linux/man-pages/man2/umask.2.html"
---

The glossary's command reference. Data lives in the frontmatter above; this body
is intentionally empty. See `docs/agent-context/README.md` for the authoring
contract.
