# CCNA reading list for the 14 draft pages

**Written:** 2026-10-04
**For:** Jeff. One entry per draft page under `content/pages/`, in the order they were
scaffolded. Read, take notes in your own form, lab it, then hand the notes over and the
page gets written from them for your approval.

**Where notes go:** `content/drafts/ccna-notes/<slug>.md`. Nothing under `content/drafts/`
is routed or searched. Any format works. The page needs four things from the notes: how
it works in your words, the config you typed, the `show` output you got, and what broke.

**Where each source comes from:**

- **Objectives** are quoted from Cisco's CCNA 200-301 v1.1 exam topics.
- **JITL** (Jeremy's IT Lab) day ranges and **Odom** chapter titles are from your own
  syllabus, `~/mg-coreforge/ccna/course/SYLLABUS.md`. The day range is for the whole
  week the topic sits in, so the topic is somewhere inside it. Odom is the *CCNA 200-301
  Official Cert Guide*, Volume 1 (V1) and Volume 2 (V2).
- **Check against** lists free primary sources. Every link was loaded on 2026-10-04.
  Where a line says none was verified, I did not find one I could confirm, and the
  textbook is the source.

---

## 1. `ethernet-and-arp`: Ethernet frames, MAC learning, and ARP

- **Objectives:** 1.13 Describe switching concepts (MAC learning and aging, frame
  switching, frame flooding, MAC address table).
- **Syllabus week 1.** JITL days 1 to 6. Odom V1: Fundamentals of Ethernet LANs;
  Analyzing Ethernet LAN Switching.
- **Check against:**
  - [RFC 826](https://www.rfc-editor.org/rfc/rfc826), the ARP specification.
  - [IEEE 802.3 Ethernet Working Group](https://www.ieee802.org/3/), the frame format.

## 2. `spanning-tree`: Spanning tree, STP and RSTP

- **Objectives:** 2.5 Interpret basic operations of Rapid PVST+ Spanning Tree Protocol
  (root port, root bridge primary and secondary, and other port names; port states and
  roles; PortFast; root guard, loop guard, BPDU filter, and BPDU guard).
- **Syllabus week 4.** JITL days 19 to 23. Odom V1: Spanning Tree Protocol Concepts;
  RSTP and EtherChannel Configuration.
- **Check against:**
  - [Understand Rapid Spanning Tree Protocol (802.1w)](https://www.cisco.com/c/en/us/support/docs/lan-switching/spanning-tree-protocol/24062-146.html),
    Cisco's explanation of port roles, states, and convergence.

## 3. `etherchannel`: EtherChannel

- **Objectives:** 2.4 Configure and verify (Layer 2/Layer 3) EtherChannel (LACP).
- **Syllabus week 4.** JITL days 19 to 23. Odom V1: RSTP and EtherChannel Configuration.
- **Check against:**
  - [Understand EtherChannel Load Balance and Redundancy on Catalyst Switches](https://www.cisco.com/c/en/us/support/docs/lan-switching/etherchannel/12023-4.html),
    Cisco's account of how a channel shares traffic across its links.

## 4. `ospf`: OSPF, single area

- **Objectives:** 3.4 Configure and verify single area OSPFv2 (neighbor adjacencies,
  point-to-point, broadcast with DR/BDR selection, router ID).
- **Syllabus week 5.** JITL days 24 to 29. Odom V1: the four OSPF chapters.
- **Check against:**
  - [RFC 2328](https://www.rfc-editor.org/rfc/rfc2328), OSPF version 2. Sections 7
    (adjacencies), 9.4 (designated router election), and 10 (neighbor states) are the
    parts the objective covers.

## 5. `first-hop-redundancy`: First-hop redundancy (HSRP, VRRP, GLBP)

- **Objectives:** 3.5 Describe the purpose, functions, and concepts of first hop
  redundancy protocols.
- **Syllabus week 5.** JITL days 24 to 29. Odom V1: IP Routing in the LAN. V2: the FHRP
  section of Miscellaneous IP Services.
- **Check against:**
  - [Understand the Hot Standby Router Protocol Features and Functionality](https://www.cisco.com/c/en/us/support/docs/ip/hot-standby-router-protocol-hsrp/9234-hsrpguidetoc.html),
    Cisco's HSRP reference.
  - [RFC 2281](https://www.rfc-editor.org/rfc/rfc2281), HSRP.
  - [RFC 9568](https://www.rfc-editor.org/rfc/rfc9568), VRRP version 3.

## 6. `access-control-lists`: ACLs and wildcard masks

- **Objectives:** 5.6 Configure and verify access control lists.
- **Syllabus week 6.** JITL days 30 to 35. Odom V2: the three ACL chapters.
- **Check against:**
  - [Configure IP Access Lists](https://www.cisco.com/c/en/us/support/docs/security/ios-firewall/23602-confaccesslists.html),
    Cisco's guide to standard, extended, and named lists and to wildcard masks.

## 7. `dhcp-and-dns-services`: DHCP, DHCP relay, and DNS

- **Objectives:** 4.3 Explain the role of DHCP and DNS within the network. 4.6 Configure
  and verify DHCP client and relay.
- **Syllabus week 7.** JITL days 36 to 40. Odom V2: Miscellaneous IP Services. V1: IP
  Addressing on Hosts.
- **Check against:**
  - [RFC 2131](https://www.rfc-editor.org/rfc/rfc2131), DHCP.
  - [RFC 3046](https://www.rfc-editor.org/rfc/rfc3046), the DHCP relay agent
    information option.
  - [RFC 1034](https://www.rfc-editor.org/rfc/rfc1034) and
    [RFC 1035](https://www.rfc-editor.org/rfc/rfc1035), DNS concepts and the record
    format.

## 8. `device-management-protocols`: NTP, SNMP, syslog, CDP and LLDP

- **Objectives:** 2.3 Configure and verify Layer 2 discovery protocols (Cisco Discovery
  Protocol and LLDP). 4.2 Configure and verify NTP operating in a client and server
  mode. 4.4 Explain the function of SNMP in network operations. 4.5 Describe the use of
  syslog features, including facilities and severity levels.
- **Syllabus weeks 7 and 8.** JITL days 36 to 45. Odom V2: Device Management Protocols.
- **Check against:**
  - [RFC 5905](https://www.rfc-editor.org/rfc/rfc5905), NTP version 4.
  - [RFC 3411](https://www.rfc-editor.org/rfc/rfc3411), the SNMP architecture.
  - [RFC 5424](https://www.rfc-editor.org/rfc/rfc5424), syslog. Section 6.2.1 has the
    facility and severity tables.
  - CDP and LLDP: none verified. LLDP is IEEE 802.1AB.

## 9. `nat`: NAT and PAT

- **Objectives:** 4.1 Configure and verify inside source NAT using static and pools.
- **Syllabus week 8.** JITL days 41 to 45. Odom V2: Network Address Translation.
- **Check against:**
  - [Configure Network Address Translation](https://www.cisco.com/c/en/us/support/docs/ip/network-address-translation-nat/13772-12.html),
    Cisco's configuration guide with worked examples.
  - [RFC 3022](https://www.rfc-editor.org/rfc/rfc3022), traditional NAT.
  - [RFC 2663](https://www.rfc-editor.org/rfc/rfc2663), NAT terminology.

## 10. `securing-device-access`: Securing device access, passwords, SSH, AAA

- **Objectives:** 2.8 Describe network device management access (Telnet, SSH, HTTP,
  HTTPS, console, TACACS+/RADIUS, and cloud managed). 4.8 Configure network devices for
  remote access using SSH. 5.3 Configure and verify device access control using local
  passwords. 5.4 Describe security password policy elements, such as management,
  complexity, and password alternatives (multifactor authentication, certificates, and
  biometrics). 5.8 Compare authentication, authorization, and accounting concepts.
- **Syllabus weeks 8 and 9.** JITL days 41 to 51. Odom V2: Securing Network Devices;
  Security Architectures.
- **Check against:**
  - [Configure SSH on Routers](https://www.cisco.com/c/en/us/support/docs/security-vpn/secure-shell-ssh/4145-ssh.html),
    Cisco's SSH setup and troubleshooting guide.
  - [RFC 4251](https://www.rfc-editor.org/rfc/rfc4251), the SSH architecture.
  - [RFC 2865](https://www.rfc-editor.org/rfc/rfc2865), RADIUS.
  - [RFC 8907](https://www.rfc-editor.org/rfc/rfc8907), TACACS+.

## 11. `layer-2-security`: Port security, DHCP snooping, dynamic ARP inspection

- **Objectives:** 5.7 Configure and verify Layer 2 security features (DHCP snooping,
  dynamic ARP inspection, and port security).
- **Syllabus week 9.** JITL days 46 to 51. Odom V2: Implementing Switch Port Security;
  DHCP Snooping and ARP Inspection.
- **Check against:** none verified. These are Cisco switch features with no RFC. The
  Catalyst software configuration guide for the switch model in your lab is the primary
  source.

## 12. `rest-json-and-config-management`: REST APIs, JSON, Ansible and Terraform

- **Objectives:** 6.5 Describe characteristics of REST-based APIs (authentication types,
  CRUD, HTTP verbs, and data encoding). 6.6 Recognize the capabilities of configuration
  management mechanisms such as Ansible and Terraform. 6.7 Recognize components of
  JSON-encoded data.
- **Syllabus week 11.** JITL days 59 to 63. Odom V2: the four automation chapters.
- **Check against:**
  - [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110), HTTP semantics. Section 9 has
    the methods.
  - [RFC 8259](https://www.rfc-editor.org/rfc/rfc8259), JSON.
  - [Terraform documentation](https://developer.hashicorp.com/terraform/docs).
  - [Ansible documentation](https://docs.ansible.com/).

## 13. `ai-in-network-operations`: AI and machine learning in network operations

- **Objectives:** 6.4 Explain AI (generative and predictive) and machine learning in
  network operations.
- **Syllabus week 11.** JITL days 59 to 63. Odom V2: the four automation chapters.
- **Check against:** none verified. The v1.1 edition of Odom and the JITL videos added
  for v1.1 are the sources.

## 14. `troubleshooting-method`: A troubleshooting method, layer by layer

- **Objectives:** none in v1.1 by number. Outcome 9 of your syllabus ("troubleshoot each
  of the above with a method, not a guess"). In v2.0 this is 2.4, troubleshoot basic
  Layer 2 and Layer 3 connectivity and device operations using show commands, ping,
  extended ping, traceroute, and packet capture output.
- **Syllabus weeks 12 to 14.** The troubleshooting tickets lab in week 13.
- **Check against:** [Using layers to troubleshoot](/learn/osi-model#using-layers-to-troubleshoot)
  on this site, and your own lab tickets. No outside source verified.
