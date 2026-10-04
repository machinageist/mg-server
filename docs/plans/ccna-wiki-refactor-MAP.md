# CCNA wiki refactor: objective map

**Written:** 2026-10-04
**Blueprint:** CCNA 200-301 v1.1. Objective numbers and wording are from Cisco's exam
topics PDF (`learningcontent.cisco.com/documents/marketing/exam-topics/200-301-CCNA-v1.1.pdf`),
read on 2026-10-04.
**Use:** the working plan for refactoring the 21 networking pages under `content/pages/`.
The Linux pages are out of scope. The 14 CCNA scaffolds stay empty until Jeff's notes
arrive (see `docs/ccna-reading-list.md`).

The page plans below were made from each page's outline and its Exam key points. Each one
is checked against the full text when the page is refactored, and this file is corrected
if the plan changes.

---

## 1. By objective

"Scaffold" means the objective's proper home is one of the 14 draft pages. Until that
page is written, any coverage already published stays where it is.

### 1.0 Network fundamentals

| Objective | Covered now | Gap | Action |
|---|---|---|---|
| 1.1 Explain the role and function of network components (routers, L2 and L3 switches, NGFW and IPS, access points, controllers, endpoints, servers, PoE) | `network-appliances` | Endpoints, servers, and controllers get a line at most | Revise `network-appliances`: lead with the eight listed components in blueprint order |
| 1.2 Describe characteristics of network topology architectures (two-tier, three-tier, spine-leaf, WAN, SOHO, on-premises and cloud) | `network-topologies`. On-premises and cloud on `cloud-computing` | WAN and SOHO are thin | Revise `network-topologies`: lead with the six listed architectures. Mesh, ring, and bus go below the line |
| 1.3 Compare physical interface and cabling types (single-mode, multimode, copper, shared media and point-to-point) | `transmission-media`, `wired-media`, `transceivers` | Spread over three pages with no single comparison | Revise `wired-media` to carry the comparison. The other two point at it |
| 1.4 Identify interface and cable issues (collisions, errors, mismatch duplex, speed) | `wired-media` (collisions), `switching-technologies` (speed and duplex) | No section on reading interface counters | Add a section to `wired-media` on what each symptom looks like |
| 1.5 Compare TCP to UDP | `osi-model` (Layer 4) | Covered inside a layer description, not as a comparison | Add a TCP and UDP comparison section to `osi-model` |
| 1.6 Configure and verify IPv4 addressing and subnetting | `ipv4-addressing`, `subnetting` | Configuration on a device is Jeff's hidden section | Reorder both. No new concept gaps |
| 1.7 Describe private IPv4 addressing | `ipv4-addressing` (Public and private addresses) | None | Keep, moved up |
| 1.8 Configure and verify IPv6 addressing and prefix | `ipv6-addressing` | Configuration on a device is Jeff's hidden section | Reorder |
| 1.9 Describe IPv6 address types (global, unique local, link local, anycast, multicast, modified EUI-64) | `ipv6-addressing` (Address types), `traffic-types` | Modified EUI-64 steps need checking in the body | Revise `ipv6-addressing`. `traffic-types` links to it |
| 1.10 Verify IP parameters for client OS (Windows, Mac OS, Linux) | `ipv4-addressing` key points only | No body section | Add a section to `ipv4-addressing` |
| 1.11 Describe wireless principles (nonoverlapping channels, SSID, RF, encryption) | `wireless-technologies`, `wireless-media` | RF terms (amplitude, frequency, dBm, RSSI, SNR) are missing | Add an RF section to `wireless-media` |
| 1.12 Explain virtualization fundamentals (server virtualization, containers, VRFs) | `cloud-computing` | VRFs are missing. Hypervisors and containers are thin | Add a virtualization section to `cloud-computing` |
| 1.13 Describe switching concepts (MAC learning and aging, frame switching, flooding, MAC address table) | Not covered as a topic | Whole objective | Scaffold `ethernet-and-arp` |

### 2.0 Network access

| Objective | Covered now | Gap | Action |
|---|---|---|---|
| 2.1 Configure and verify VLANs spanning multiple switches (access ports data and voice, default VLAN, interVLAN connectivity) | `switching-technologies` | Default VLAN is thin | Revise `switching-technologies` (pilot) |
| 2.2 Configure and verify interswitch connectivity (trunk ports, 802.1Q, native VLAN) | `switching-technologies` | None found | Reorder so trunks follow VLANs directly |
| 2.3 Configure and verify Layer 2 discovery protocols (CDP and LLDP) | Not covered | Whole objective | Scaffold `device-management-protocols` |
| 2.4 Configure and verify EtherChannel (LACP) | `switching-technologies` (Link aggregation) | | Stays for now. Scaffold `etherchannel` |
| 2.5 Interpret basic operations of Rapid PVST+ (root port and bridge, port states and roles, PortFast, root guard, loop guard, BPDU filter, BPDU guard) | `switching-technologies` (Spanning Tree Protocol) | Guards are in key points only | Stays for now. Scaffold `spanning-tree` |
| 2.6 Describe Cisco wireless architectures and AP modes | `wireless-technologies` (Autonomous and lightweight access points) | AP modes are in key points only | Add an AP modes section to `wireless-technologies` |
| 2.7 Describe physical infrastructure connections of WLAN components (AP, WLC, access and trunk ports, LAG) | `wireless-technologies` key points only | No body section | Add a section to `wireless-technologies` |
| 2.8 Describe network device management access (Telnet, SSH, HTTP, HTTPS, console, TACACS+/RADIUS, cloud managed) | Not covered | Whole objective | Scaffold `securing-device-access` |
| 2.9 Interpret the wireless LAN GUI configuration for client connectivity | Not covered | Needs the controller GUI in a lab | Left for Jeff. Noted in key points |

### 3.0 IP connectivity

| Objective | Covered now | Gap | Action |
|---|---|---|---|
| 3.1 Interpret the components of routing table (code, prefix, mask, next hop, AD, metric, gateway of last resort) | `routing-technologies` | Reading one routing table entry field by field | Add a section to `routing-technologies` |
| 3.2 Determine how a router makes a forwarding decision (longest prefix match, AD, metric) | `routing-technologies` (How a router selects a route) | None found | Keep, moved to the top |
| 3.3 Configure and verify IPv4 and IPv6 static routing (default, network, host, floating static) | `routing-technologies` (Static and dynamic routing) | The four route kinds are in key points only | Add them to the static routing section |
| 3.4 Configure and verify single area OSPFv2 | `routing-technologies` (Routing protocol families) | | Stays for now. Scaffold `ospf` |
| 3.5 Describe first hop redundancy protocols | `routing-technologies` (First-hop redundancy and virtual IPs) | | Stays for now. Scaffold `first-hop-redundancy` |

### 4.0 IP services

| Objective | Covered now | Gap | Action |
|---|---|---|---|
| 4.1 Configure and verify inside source NAT using static and pools | `routing-technologies` (Address translation) | | Stays for now. Scaffold `nat` |
| 4.2 Configure and verify NTP in client and server mode | `network-protocols` (one line) | | Scaffold `device-management-protocols` |
| 4.3 Explain the role of DHCP and DNS within the network | `network-protocols` | | Revise `network-protocols`. Detail goes to scaffold `dhcp-and-dns-services` |
| 4.4 Explain the function of SNMP in network operations | `network-protocols` (one line) | | Scaffold `device-management-protocols` |
| 4.5 Describe the use of syslog features, including facilities and severity levels | `network-protocols` key points | | Scaffold `device-management-protocols` |
| 4.6 Configure and verify DHCP client and relay | `network-protocols` key points | | Scaffold `dhcp-and-dns-services` |
| 4.7 Explain the forwarding per-hop behavior for QoS (classification, marking, queuing, congestion, policing, shaping) | `quality-of-service` | Check all six terms are in the body | Revise `quality-of-service` around the six terms |
| 4.8 Configure network devices for remote access using SSH | `network-protocols` key points | | Scaffold `securing-device-access` |
| 4.9 Describe the capabilities and functions of TFTP/FTP in the network | `network-protocols` | Comparison of the two | Add to `network-protocols` |

### 5.0 Security fundamentals

| Objective | Covered now | Gap | Action |
|---|---|---|---|
| 5.1 Define key security concepts (threats, vulnerabilities, exploits, mitigation) | Not covered | Whole objective | **No page planned. Open question for Jeff** |
| 5.2 Describe security program elements (user awareness, training, physical access control) | Not covered | Whole objective | **No page planned. Open question for Jeff** |
| 5.3 Configure and verify device access control using local passwords | Not covered | | Scaffold `securing-device-access` |
| 5.4 Describe security password policy elements (management, complexity, MFA, certificates, biometrics) | Not covered | | Scaffold `securing-device-access` |
| 5.5 Describe IPsec remote access and site-to-site VPNs | `vpns-and-ipsec` | None found | Reorder so the two VPN kinds lead |
| 5.6 Configure and verify access control lists | `cloud-computing` mentions access lists only | | Scaffold `access-control-lists` |
| 5.7 Configure and verify Layer 2 security features (DHCP snooping, DAI, port security) | Not covered | | Scaffold `layer-2-security` |
| 5.8 Compare authentication, authorization, and accounting concepts | `zero-trust-architecture` key points | | Scaffold `securing-device-access` |
| 5.9 Describe wireless security protocols (WPA, WPA2, WPA3) | `wireless-technologies` (Encryption, Authentication) | None found | Keep, grouped under one security section |
| 5.10 Configure and verify WLAN within the GUI using WPA2 PSK | Not covered | Needs the controller GUI in a lab | Left for Jeff. Noted in key points |

### 6.0 Automation and programmability

| Objective | Covered now | Gap | Action |
|---|---|---|---|
| 6.1 Explain how automation impacts network management | `software-defined-networking` | Thin | Add a short section |
| 6.2 Compare traditional networks with controller-based networking | `software-defined-networking` (Separating the planes) | None found | Keep, leads the page |
| 6.3 Describe controller-based, software defined architecture (overlay, underlay, fabric, planes, northbound and southbound APIs) | `software-defined-networking` | Northbound and southbound APIs are in key points only | Add to the body |
| 6.4 Explain AI and machine learning in network operations | Not covered | | Scaffold `ai-in-network-operations` |
| 6.5 Describe characteristics of REST-based APIs | key points only | | Scaffold `rest-json-and-config-management` |
| 6.6 Recognize the capabilities of configuration management mechanisms such as Ansible and Terraform | `software-defined-networking` (Infrastructure as code) | | Stays for now. Scaffold `rest-json-and-config-management` |
| 6.7 Recognize components of JSON-encoded data | key points only | | Scaffold `rest-json-and-config-management` |

---

## 2. By page

Every page follows the same order: Overview, the CCNA sections in objective order, the
hidden `On the device` placeholder where there is one, Suggested practice,
`## Beyond the CCNA blueprint`, Exam key points, Related pages, Sources. Sections moved
below the line become third-level headings and keep their text and their ids. Their own
subsections stay third-level beside them, because the glossary and question-bank tests
resolve links against second and third-level headings only.

| Page | v1.1 | CCNA sections, in order | Below the line | To write |
|---|---|---|---|---|
| `network-appliances` | 1.1 | Routers and switches, firewalls and IPS, access points and controllers, endpoints and servers, PoE | Storage appliances, application-delivery appliances | Endpoints, servers, controllers, PoE in the body |
| `network-topologies` | 1.2 | Two-tier (collapsed core), three-tier, spine and leaf, WAN, SOHO, on-premises and cloud, traffic flow | Mesh, ring and token passing, hybrid, point to point, physical and logical | WAN and SOHO sections |
| `transmission-media` | 1.3 | Bounded and unbounded media, shared media and point-to-point | Who standardizes what | Short. Mostly a pointer to `wired-media` |
| `wired-media` | 1.3, 1.4 | Copper, single-mode and multimode fiber, the comparison, collisions, reading interface problems | Coaxial, plenum, direct attach copper, OM grades detail | Copper, multimode, single-mode comparison. Interface and cable issues section |
| `transceivers` | 1.3 | What a transceiver carries, picking an optic by fiber and distance | Form factors, connector types, reading the label | Little |
| `osi-model` | 1.5 | Layers at a glance, encapsulation, TCP and UDP compared, OSI and TCP/IP, using layers to troubleshoot | Where it came from, session and presentation detail | TCP and UDP comparison |
| `ipv4-addressing` | 1.6, 1.7, 1.10 | Shape of an address, dividing the address, public and private, checking a client's settings, link-local, loopback | Classful addressing | Client OS section |
| `subnetting` | 1.6 | Subnet masks, counting hosts and networks, CIDR, VLSM | None | Link to the drill at `/study/drills/subnetting` |
| `ipv6-addressing` | 1.8, 1.9 | Shape, prefix and interface ID, address types, modified EUI-64, neighbor discovery | Coexisting with IPv4 | Modified EUI-64 worked through if the body lacks it |
| `traffic-types` | 1.9 | Unicast, multicast, anycast | Broadcast stays (IPv4 context) | Tie each to its IPv6 address type |
| `wireless-media` | 1.11 | RF, taking turns on a channel, 802.11 standards | Cellular, satellite | RF terms section |
| `wireless-technologies` | 1.11, 2.6, 2.7, 5.9 | Channels, bands, SSIDs, architectures (autonomous, lightweight), AP modes, how APs and controllers connect, security protocols | Antennas, guest networks and captive portals, network types | AP modes. Physical connections |
| `cloud-computing` | 1.2, 1.12 | On-premises and cloud, server virtualization, containers, VRFs | Service models, deployment models, gateways, VPCs, NFV, scalability | Virtualization section with VRFs |
| `switching-technologies` (pilot) | 2.1, 2.2, 2.4, 2.5 | VLANs (access ports, voice, default VLAN), trunks and 802.1Q and native VLAN, inter-VLAN routing, link aggregation, spanning tree, interface configuration, speed and duplex | Private VLANs, frame size and jumbo frames | Default VLAN |
| `routing-technologies` | 3.1 to 3.5, 4.1 | Reading the routing table, how a router selects a route, static routes, dynamic routing and OSPF, first-hop redundancy, address translation, TTL | Other protocol families detail (RIP, EIGRP, BGP beyond route codes), subinterfaces if duplicated on switching | Routing table fields. Four static route kinds |
| `network-protocols` | 4.3, 4.9 | DHCP and DNS, TFTP and FTP, the management protocols in brief, port ranges | Tunneling protocols, the long port table | TFTP and FTP comparison |
| `quality-of-service` | 4.7 | Classification, marking, queuing, congestion, policing, shaping | Anything else | Rebuilt around the six terms |
| `vpns-and-ipsec` | 5.5 | Site-to-site and remote access, IPsec, IKE | Other tunneling | Little |
| `software-defined-networking` | 6.1, 6.2, 6.3 | Automation and network management, traditional and controller-based, planes, overlay, underlay, and fabric, northbound and southbound APIs | SD-WAN, VXLAN detail, source control, infrastructure as code | 6.1 section. APIs in the body |
| `content-delivery-networks` | none | Not on the blueprint. No reorder | | Overview says so in one sentence |
| `zero-trust-architecture` | none | Not on the blueprint. No reorder | | Overview says so in one sentence |

No slug changes are planned.

---

## 3. Open questions for Jeff

1. **Objectives 5.1 and 5.2** (security concepts and security program elements) have no
   page and no scaffold. A fifteenth scaffold, `security-fundamentals`, would cover them.
2. **Objectives 2.9 and 5.10** need a wireless controller GUI. They are left as notes in
   the Exam key points of `wireless-technologies` until Jeff has done the lab in week 10.
3. **When a scaffold is written**, the matching section on the older page (spanning tree
   and link aggregation on `switching-technologies`, OSPF, FHRP, and NAT on
   `routing-technologies`) should shrink to a summary and a link. That is a later pass.
