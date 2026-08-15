---
topic: "The OSI model"
questions:
  - stem: "Real protocol stacks do not implement seven separate layers, and several protocols cross the boundaries. Given that, why is the model still worth keeping?"
    options:
      - "Because vendors are required to document their equipment against it"
      - "Because it describes the protocol architecture the Internet actually runs on"
      - "Because it narrows a problem down to which function failed, what information is available there, and which component handles it"
      - "Because each layer can be tested in isolation with a tool built for that layer alone"
    answer: 2
    explanation: >
      The model earns its place as a diagnostic and vocabulary tool, not as a
      picture of any real stack. Vendors document against nothing in particular
      and are under no such obligation. The claim that it describes the
      Internet's actual architecture belongs to the TCP/IP model, which is the
      four-layer view; OSI is explicitly a reference model rather than the
      protocol suite that won. And there is no per-layer tool that isolates one
      layer cleanly — a single capture shows several layers at once, which is
      part of why you need a way to decide which one you are asking about.
    learn: { slug: "osi-model", anchor: "overview" }

  - stem: "A switch reports a rising count of frames failing their frame check sequence. What has that CRC actually established?"
    options:
      - "The frames were corrupted in transit, and the switch repaired them before forwarding"
      - "The frames were corrupted in transit; the check detects the damage but does not repair it"
      - "The frames arrived out of order and were put back into sequence"
      - "The destination MAC address was unknown, so the frames were flooded to every port"
    answer: 1
    explanation: >
      A cyclic redundancy check is a detection mechanism. It tells the receiver
      that the bits it holds are not the bits that were sent, and that is the
      end of its contribution — recovering the data is somebody else's job,
      usually a higher layer's retransmission. Reordering is a transport
      concern and has nothing to do with the frame check sequence. Flooding an
      unknown destination is a forwarding decision made from the MAC address
      table, which is a separate mechanism that a failing CRC says nothing
      about.
    learn: { slug: "osi-model", anchor: "layer-2-data-link" }

  - stem: "A host has a link light, exchanges frames with its neighbours on the local segment without trouble, and cannot reach anything on a remote network. Which layer does the model point at first?"
    options:
      - "Layer 1, because reaching a remote network depends on signal strength over distance"
      - "Layer 2, because forwarding between networks is the switch's job"
      - "Layer 4, because traffic to a remote network requires a completed TCP handshake before any packet leaves the host"
      - "Layer 3, because layers 1 and 2 can be entirely healthy while the address, prefix, gateway, or route is wrong"
    answer: 3
    explanation: >
      The evidence given rules the lower layers in, not out: a link light is a
      layer 1 pass, and successful local frame exchange is a layer 2 pass. What
      remains is logical addressing and forwarding between networks, which is
      layer 3. Signal strength governs the local link, and the local link is
      demonstrably working. Forwarding between networks is a router's job, not
      a switch's. And the transport layer answer inverts the order of
      operations — a TCP handshake requires packets to reach the far end
      first, so a broken layer 3 explains a failed handshake rather than the
      other way round.
    learn: { slug: "osi-model", anchor: "layer-3-network" }

  - stem: "Someone describes TCP as providing error correction. What does TCP actually do about damaged and missing data?"
    options:
      - "Its checksum detects some corruption, and acknowledgements plus retransmission recover data it infers was lost"
      - "It repairs corrupted bytes in place from the checksum, so damaged data never has to be sent again"
      - "It recomputes the layer 2 frame check sequence and repairs the frame"
      - "It passes corrupted segments up with an error flag and leaves the application to repair them"
    answer: 0
    explanation: >
      Calling it error correction hides the mechanism, which is worth knowing
      because it is what makes TCP expensive. Detection and recovery are two
      different things: a checksum notices damage, and acknowledgement plus
      retransmission fetches the data again. A checksum carries nowhere near
      enough information to reconstruct what was lost, so in-place repair is
      not on offer. Frame check sequences belong to layer 2 and are not TCP's
      to touch. And a TCP connection's whole promise to the application is
      reliable, ordered delivery — handing up corrupted data with a flag would
      break exactly that contract.
    learn: { slug: "osi-model", anchor: "tcp" }

  - stem: "A colleague proposes UDP for a bulk file transfer on the grounds that UDP is faster. What is wrong with the reasoning?"
    options:
      - "UDP is always slower than TCP because it has no windowing"
      - "UDP is restricted by specification to real-time traffic"
      - "UDP is a smaller transport rather than an inherently faster one, and it guarantees nothing about delivery, ordering, or duplicates"
      - "UDP cannot carry more than one datagram per exchange"
    answer: 2
    explanation: >
      The speed claim is a folk belief. UDP is minimal, not fast — it omits
      connection setup, acknowledgement, and congestion response, and an
      application that needs those properties has to build them back, usually
      less well than TCP already does. The blanket claim that UDP is always
      slower is the same mistake pointing the other way. UDP is commonly used
      for streaming and games but nothing in the specification limits it to
      them, and it has no per-exchange datagram limit.
    learn: { slug: "osi-model", anchor: "udp" }

  - stem: "An IP packet crosses a router between two Ethernet segments. What happens to the frame the router received?"
    options:
      - "The router forwards the same frame unchanged, having rewritten only the destination IP address"
      - "The router keeps the original frame and wraps a second frame around it for the next hop"
      - "The router strips the IP header and forwards the frame toward the destination MAC address"
      - "The router removes the incoming frame, acts on the layer 3 packet, and builds new layer 2 framing for the next link"
    answer: 3
    explanation: >
      Framing is per-link. A MAC address is only meaningful on the segment it
      belongs to, so the frame cannot survive the hop and a new one is built on
      the other side. The first option describes something a router never does
      — routing does not rewrite the destination IP, that is what address
      translation does, and it would leave a frame addressed to the wrong
      segment. Wrapping a second frame around the first describes tunneling,
      not routing. Stripping the IP header would discard the only information
      that says where the packet is going.
    learn: { slug: "osi-model", anchor: "encapsulation-and-decapsulation" }

  - stem: "Where does TLS sit in a working Internet stack, and what is SSL?"
    options:
      - "TLS does presentation-layer work but runs above a reliable transport such as TCP; SSL is TLS's obsolete predecessor"
      - "TLS replaces TCP at layer 4; SSL is the name for its use with HTTP"
      - "TLS encrypts frames at layer 2; SSL is the same protocol under an older marketing name"
      - "TLS is an application protocol alongside HTTP at layer 7; SSL is the transport it depends on"
    answer: 0
    explanation: >
      TLS is the standard example of a protocol that resists the layer chart.
      The function it performs — encryption and peer authentication, changing
      how data is represented — is presentation-like, but its position in the
      real stack is concrete and above TCP. It does not replace the transport;
      it requires one, because a lost or reordered record would break the
      cipher stream. It is not a layer 2 mechanism, and calling SSL a current
      protocol or the transport beneath TLS gets the history backwards: SSL is
      the deprecated ancestor, not a partner.
    learn: { slug: "osi-model", anchor: "layer-6-presentation" }

  - stem: "A web application firewall rejects a request whose query string carries a SQL injection pattern. What makes that a layer 7 decision rather than a layer 4 one?"
    options:
      - "It matched the destination port against a rule"
      - "It reassembled the TCP stream, which only layer 7 systems can do"
      - "It terminated the TLS session, and TLS termination is what defines layer 7"
      - "It understood the structure and meaning of the HTTP request, not merely its addressing"
    answer: 3
    explanation: >
      What puts a decision at layer 7 is that it depends on the content and
      grammar of an application protocol — this firewall had to know what a
      query string is and where it sits in an HTTP request. Matching a
      destination port is layer 4 work and any stateful firewall does it.
      Reassembling a TCP stream is a transport-layer operation that plenty of
      layer 4 devices perform. TLS termination is a prerequisite for seeing the
      HTTP at all, but decrypting is not the same as understanding, and a
      device can terminate TLS and forward blindly.
    learn: { slug: "osi-model", anchor: "layer-7-application" }

  - stem: "Why is a hub placed at layer 1 while a switch is placed at layer 2?"
    options:
      - "A hub repeats signals to every port without reading addresses; a switch learns source MAC addresses and forwards toward the associated port"
      - "A hub is slower, and throughput is what separates the two layers"
      - "A hub runs half duplex, and duplex is a layer 1 property while speed is a layer 2 one"
      - "A hub cannot detect collisions, and collision detection is the defining layer 2 function"
    answer: 0
    explanation: >
      Layer assignment follows the information a device acts on. A hub acts on
      signals only and has no notion of an address, so it belongs with the
      cabling and the transceivers. A switch reads addresses out of frames and
      makes a forwarding choice from them, which is link-layer work. Speed is
      not a layer property at all — a fast hub is still a hub. Duplex is a
      property of a link rather than a criterion for classifying a device. And
      collision handling on a shared medium is real layer 2 business, but it is
      not what distinguishes these two devices; the address table is.
    learn: { slug: "osi-model", anchor: "layer-1-physical" }

  - stem: "SMB relies on services that NetBIOS historically provided at the session layer. Does that make SMB a layer 5 protocol?"
    options:
      - "Yes — a protocol belongs to the layer whose services it consumes"
      - "No — SMB is normally treated as an application-layer file-sharing protocol"
      - "Yes, but only on networks where NetBIOS is still enabled"
      - "No — SMB is a transport-layer protocol, because it runs directly over TCP 445"
    answer: 1
    explanation: >
      Consuming a layer's services does not place you in that layer, or every
      protocol on the Internet would be layer 4. SMB defines file and print
      operations, which is application semantics, and that is where it is
      normally classified. Making the classification depend on whether NetBIOS
      is enabled would mean the same protocol changed layers depending on a
      configuration setting. Running over TCP 445 is an argument that SMB sits
      *above* the transport layer, not in it — everything at layer 7 runs over
      some transport port.
    learn: { slug: "osi-model", anchor: "layer-5-session" }

  - stem: "In the four-layer TCP/IP view, which OSI layers does the single Application layer correspond to?"
    options:
      - "Layer 7 only"
      - "Layers 6 and 7"
      - "Layers 4 through 7"
      - "Layers 5 through 7"
    answer: 3
    explanation: >
      TCP/IP collapses session, presentation, and application into one layer,
      which is a fair description of how Internet software is actually built —
      session state and encoding are handled inside applications and libraries
      rather than by separate protocols. Mapping it to layer 7 alone, or to 6
      and 7, leaves the session functions unaccounted for. Including layer 4
      is wrong in the other direction: TCP/IP names Transport as its own
      distinct layer, so it cannot also be inside Application.
    learn: { slug: "osi-model", anchor: "osi-and-the-tcp-ip-model" }

  - stem: "A troubleshooting guide insists that every investigation must begin at layer 1 and work upward. How should you treat that rule?"
    options:
      - "Follow it: without a signal, nothing above it can be meaningfully tested"
      - "Invert it and always start at layer 7, since that is where users report the problem"
      - "There is no such rule — the model's value is in making the starting point and its assumptions explicit"
      - "Follow it on wired networks and ignore it on wireless ones"
    answer: 2
    explanation: >
      Bottom-up is one reasonable habit, not a requirement. If a service
      answers on one host and not another, starting at the cable wastes the
      evidence you already have. The point of the layered questions is that
      each one states what it assumes, so you can enter wherever the evidence
      puts you and know what you have skipped. Always starting at layer 7 has
      the same rigidity problem in the opposite direction, and nothing about
      the medium — wired or wireless — changes where the useful entry point
      is.
    learn: { slug: "osi-model", anchor: "using-layers-to-troubleshoot" }
---
