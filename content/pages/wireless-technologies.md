---
title: "Wireless technologies: channels, security, and access points"
date: 2026-09-25
summary: "How Wi-Fi splits its bands into channels, how wireless networks are named and laid out, how WPA2 and WPA3 protect them, and how access points are managed."
tags: [education, networking, wireless, wifi, wpa3, authentication]
---

## Overview

Most people reach a network over Wi-Fi now. Phones, tablets, laptops, and most IoT
devices have no cable at all, and plenty of homes and offices are reached almost
entirely over radio.

The [wireless media](/learn/wireless-media) page covers the physical side: how
stations share a channel, and the 802.11 generations and their speeds. This page
covers deployment. A working wireless network comes down to a handful of decisions:
which band and channel each access point uses, how wide those channels are, how the
network is named and laid out, how traffic is encrypted, and how users prove who
they are. All of it also has to fit the radio rules in the country where the
equipment runs.

## Channels

A channel is a specific frequency range inside one of the Wi-Fi bands: 2.4 GHz,
5 GHz, or 6 GHz. An access point and its clients talk on one channel at a time.
Channel numbers are spaced 5 MHz apart, but a channel is much wider than 5 MHz, and
most of the planning problems come from that.

The 2.4 GHz band has 14 channel numbers. Channel 1 is centered on 2,412 MHz, and
each number after it moves the center up by 5 MHz, through channel 13 at 2,472 MHz.
Channel 14 breaks the pattern. It sits 12 MHz above channel 13, at 2,484 MHz, and
Japan is the only country that allowed it, and only for 802.11b.

In the US and Canada, Wi-Fi equipment normally uses channels 1 through 11. Channels
12 and 13 sit close to the top edge of the band, where the limits on stray emissions
are strict, so they either run at reduced power or are not offered at all.

### Channel width

Channel width is how much spectrum a channel takes up, measured in MHz. A wider
channel carries more data. It also covers more of the band, so it is more likely to
overlap a neighbor and pick up interference.

A 2.4 GHz channel is about 20 MHz wide, or 22 MHz for the original 802.11b signal.
Using the 22 MHz figure, channel 6 is centered on 2,437 MHz and covers roughly 2,426
to 2,448 MHz.

The 2.4 GHz band is also shared with devices that are not Wi-Fi at all. Bluetooth,
baby monitors, and microwave ovens all run in or near it and can interfere with a
wireless network.

### Non-overlapping channels

Channels spaced 5 MHz apart and 22 MHz wide have to overlap. Channel 1 runs up to
about 2,423 MHz, which is past the bottom edge of channels 2, 3, 4, and 5.

Only three channels in the 2.4 GHz band stay clear of each other: 1, 6, and 11. In
practice, if you have three access points close together, put one on channel 1, one
on 6, and one on 11. A fourth access point has to share one of those channels.

Sharing a channel is better than partly overlapping one. Two access points on the
same channel hear each other's transmissions as Wi-Fi and take turns. Two on
overlapping channels, say 1 and 3, only hear each other as noise. Neither one waits
for the other, and their transmissions damage each other.

The 5 GHz band has about 25 non-overlapping 20 MHz channels in the US, so it has far
fewer interference problems.

### Wider channels

The 5 and 6 GHz bands have much more spectrum to work with, so they can use wider
channels. Two neighboring 20 MHz channels can be bonded into one 40 MHz channel, two
of those into 80 MHz, and two of those into 160 MHz. Wi-Fi 7 (802.11be) adds 320 MHz
channels in the 6 GHz band.

Width is a tradeoff. In 2.4 GHz, a 40 MHz channel uses up two of the three clean
channels, so it causes trouble anywhere there are neighbors. In 5 GHz, 80 MHz is a
common default, but in a dense building it leaves only a few separate channels to go
around. 160 MHz channels suit high-bandwidth use in places with few other networks.

### Regulation

Radio emissions are regulated by each country's government. In the US that is the
Federal Communications Commission (FCC), and Wi-Fi falls under the rules for
unlicensed devices in Part 15. The regulator decides which frequencies are open,
which are reserved, and how much power a transmitter can use. Power limits are also
range limits.

The IEEE writes the 802.11 standard, but it does not decide what is legal. The
standard includes features that let equipment follow local rules, and each device
is set to a country code that decides which channels and power levels it offers.

### 802.11h: DFS and TPC

Parts of the 5 GHz band are shared with radar, including weather and military radar.
The 802.11h amendment added two features so Wi-Fi can use those channels without
getting in the way:

- Dynamic Frequency Selection (DFS) listens for radar. Before using a DFS channel,
  the access point listens for a set time, 60 seconds under FCC rules. If it detects
  radar while it is using the channel, it has to leave.
- Transmit Power Control (TPC) lowers transmit power to what the link needs, which
  reduces interference with radar and with other networks.

A DFS event is visible to users. When the access point changes channel, clients can
drop for a moment. Some networks leave the DFS channels off for that reason and
accept having fewer channels to choose from.

## Frequency bands compared

Each band trades range for capacity. Lower frequencies travel farther and pass
through walls better. Higher frequencies carry more data but fade faster.

- 2.4 GHz reaches the farthest, and nearly every device supports it. It has only
  three clean channels and the most interference.
- 5 GHz has shorter range and much more room, with more channels and wider ones.
- 6 GHz has the shortest range and the most spectrum. Both the access point and the
  client need Wi-Fi 6E or newer to use it.

Published range figures are ideal-case numbers. Walls, interference, and antenna
design change the real range a lot. The [802.11 standards
table](/learn/wireless-media#802-11-standards) on the wireless media page lists each
generation, the bands it uses, and its maximum rate.

### Band steering

Most access points broadcast the same network on more than one band. A client that
supports several bands still has to pick one, and it does not always pick well. A
laptop can stay on 2.4 GHz a few feet from the access point, where 5 GHz would be
much faster.

Band steering is how the access point nudges dual-band clients toward the better
band, usually 5 GHz. Based on signal strength, it can delay its 2.4 GHz replies to a
client it knows can use 5 GHz, or ask the client to move. The client still makes the
final decision.

## Service set identifiers

A service set identifier (SSID) is the name of a wireless network, such as
"HomeNetwork" or "CoffeeShopWiFi". It is what shows up in the list when a device
looks for networks, and it can be up to 32 bytes long.

Pick a name people can recognize, and keep personal information out of it. A family
name, an apartment number, or a router model tells everyone in range more than they
need to know.

Hiding the SSID does not hide the network. Clients that have joined a hidden network
before ask for it by name, and anyone listening can read the name from those
requests.

### BSSID

A basic service set (BSS) is one access point radio and the clients connected to
it. Its identifier, the BSSID, is usually the radio's MAC address. Several access
points can broadcast the same SSID, but each has its own BSSID, and that is how a
client tells them apart.

A dual-band access point has at least one BSSID per band, and usually one more for
each extra SSID it broadcasts.

### ESS and ESSID

An extended service set (ESS) is a group of access points that share one SSID and
connect to the same wired network. The shared name is the ESSID. An office network
called "OfficeNetwork" might be served by ten access points, each with its own
BSSID, all under one ESSID.

The ESS is what lets someone walk from one end of the office to the other without
joining a new network. When the signal from the current access point gets weak, the
client roams to a stronger one with the same ESSID. The client decides when to roam,
and the move usually takes a fraction of a second.

## Network types

One access point covers a home or a small office. Past that, more access points are
needed, and there is more than one way to connect them. The [network
topologies](/learn/network-topologies) page covers the same shapes for wired
networks.

### Infrastructure networks

In an infrastructure network, every device connects to a central access point or
wireless router. This is the most common type in homes, offices, and public places.
The access point manages the network. It admits clients, relays their traffic, and
connects them to the wired network and the internet.

Two clients on the same access point do not talk to each other directly. Their
traffic goes through the access point.

### Ad hoc networks

An ad hoc network is a simple, temporary, peer-to-peer network. Devices connect
directly to each other with no access point or router. Two laptops could share files
this way in a park with no Wi-Fi around. The 802.11 name for it is an independent
basic service set (IBSS). Wi-Fi Direct, which phones use for screen casting and
printing, fills the same role today.

### Mesh networks

In a mesh network, the wireless nodes link to each other and pass traffic from node
to node until it reaches one with a wired connection. There is no need to run a
cable to every access point, and no single central router that everything depends
on. Consumer "mesh Wi-Fi" systems work this way.

A mesh can be full or partial:

- In a full mesh, every node connects to every other node. It has the most
  redundancy, but the number of links grows fast. Ten nodes need 45 links, which
  makes a full mesh expensive and hard to maintain.
- In a partial mesh, only some nodes connect to each other. It has less redundancy
  but costs less and is easier to maintain. Wireless meshes are nearly always
  partial, because a node can only link to nodes within radio range.

Each wireless hop costs throughput. A node that relays traffic on the same radio it
uses for clients has to receive everything and send it again, so speed drops with
each hop. Better mesh systems add a separate radio just for traffic between nodes.

### Point-to-point links

A point-to-point wireless link connects exactly two sites. It usually uses
directional antennas aimed at each other to make a fast link over a long distance. A
common use is connecting two buildings where running a cable between them is
impractical. The antennas need a clear line of sight.

## Encryption

A wired connection has to be plugged in. A radio signal reaches every device in
range, including devices outside the building. Encryption keeps the traffic private
even though anyone nearby can receive it.

Wi-Fi security has gone through several generations. The original Wired Equivalent
Privacy (WEP) and the first version of WPA are broken and should not be used.
Current networks use WPA2 or WPA3.

### WPA2

Wi-Fi Protected Access 2 (WPA2) came from the Wi-Fi Alliance in 2004. It is still
common, but it is aging. The encryption is done by CCMP (Counter Mode with Cipher
Block Chaining Message Authentication Code Protocol), which uses 128-bit AES.

WPA2 has two modes.

WPA2-Personal uses a pre-shared key (PSK), which is the network's password. Every
device uses the same one. Devices save it after the first connection, so a long,
complex password costs almost nothing in convenience. It also matters more than it
seems. Someone who records a device joining the network can take that recording
away and test password guesses against it offline, as fast as their hardware
allows. A weak password does not last long.

WPA2-Enterprise gives each user their own credentials, checked by a central
authentication server. That makes it possible to log activity per user and to
revoke one person's access without changing the password for everyone. It suits
businesses and schools, where scale and accountability matter. How the login works
is covered under [enterprise wireless
authentication](#enterprise-wireless-authentication) below.

### Wi-Fi Protected Setup

Wi-Fi Protected Setup (WPS) is a shortcut for getting a device onto a WPA2-Personal
network without typing the password. Usually you press a button on the router, and
for a short time a device can join without it. WPS also has a PIN version, and that
version has a design flaw that lets an attacker guess the PIN in hours. If you do
not use WPS, turn it off.

### WPA3

WPA3 was released in 2018 as the more secure replacement for WPA2. The biggest
change is in how a personal network uses its password.

Simultaneous Authentication of Equals (SAE) replaces WPA2's pre-shared key exchange.
It is a Diffie-Hellman key exchange based on the Dragonfly handshake. Recording a
device joining the network no longer gives an attacker anything to test guesses
against offline. Each guess has to be tried against the network itself. SAE also
gives forward secrecy, so someone who learns the password later still cannot
decrypt traffic they recorded earlier.

WPA3 also includes:

- Protected Management Frames (PMF), which stop an attacker from forging management
  frames. Without PMF, anyone in range can send fake deauthentication frames and
  knock clients off the network. WPA3 requires PMF.
- WPA3-Enterprise, which has an optional 192-bit mode for high-security networks
  such as government and finance. That mode uses stronger algorithms throughout,
  including Elliptic Curve Diffie-Hellman Ephemeral (ECDHE) for the key exchange.
- Wi-Fi Easy Connect, which adds a device to the network by scanning a QR code. It
  is aimed at IoT devices with no screen or keyboard.
- Wi-Fi Enhanced Open, released alongside WPA3, which encrypts open networks that
  have no password, such as those in cafes and hotels. Each client gets its own key,
  so other people on the same network cannot read its traffic. It does not prove the
  network is the real one, though.

The 6 GHz band requires WPA3 or Enhanced Open. WPA2 is not allowed there.

Many routers offer a WPA2/WPA3 mixed mode so older devices can still connect. That
keeps the network compatible, but a device that joins with WPA2 only gets WPA2's
protection.

## Guest networks and captive portals

A guest network lets visitors reach the internet without reaching the main network
or its devices. It is usually its own SSID, placed on a separate
[VLAN](/learn/switching-technologies#vlans) or subnet, with firewall rules that
allow internet access and nothing else.

A captive portal controls who gets through. A new client's web traffic goes to a
login page first, where the user signs in, accepts the terms of service, or goes
through some other check before getting access. Cafes, hotels, libraries, and
airports use them.

A captive portal only controls access. It does not encrypt anything. On an open
network with a captive portal, traffic is still unencrypted over the air unless the
network also uses Enhanced Open.

## Authentication

An authentication protocol is a set of rules for how each side proves its identity.
Encryption keeps traffic private, and authentication decides who gets in at all.
Beyond keeping strangers out, it lets the network tie traffic to a user, log it, and
apply policy per user.

### 802.1X and EAP

IEEE 802.1X is the access control standard behind enterprise authentication, on both
wired and wireless networks. A device gets onto the network only after it
authenticates. There are three roles:

- The supplicant is the client asking to join.
- The authenticator is the switch port or access point it connects through.
- The authentication server, usually a RADIUS server, makes the decision.

On a wired network the authenticator is a switch, doing port-based authentication.
On Wi-Fi it is the access point, or the controller behind it. Until authentication
succeeds, the authenticator passes nothing but authentication traffic.

802.1X carries the Extensible Authentication Protocol (EAP). EAP is a framework, not
a single method, and the method decides what each side has to prove. Four are
common:

- EAP-TLS uses certificates on both sides. The server proves its identity to the
  client, and the client proves its identity to the server. It is the strongest of
  the four and the hardest to run, because every device needs its own certificate.
- PEAP (Protected EAP) uses only a server certificate. The client checks it, a TLS
  tunnel is built, and the user signs in with a username and password inside the
  tunnel.
- EAP-TTLS works like PEAP. It builds a TLS tunnel with the server's certificate,
  then checks the client's credentials inside it, and it supports a wider range of
  inner login methods.
- EAP-FAST was developed by Cisco. It builds its tunnel with a Protected Access
  Credential (PAC) instead of certificates.

With PEAP and EAP-TTLS, the client has to be set to verify the server's certificate.
A client that skips the check will send its password to anyone running a fake access
point with the same network name.

### Enterprise wireless authentication

A single shared password does not scale to many users. Every time someone leaves,
the password has to change on every device, and there is no record of who did what.
Enterprise wireless authentication gives each user their own login, checked in one
central place.

The client connects to an access point, which passes the client's credentials to a
RADIUS server. The RADIUS server checks the credentials, often against a directory
such as Active Directory on a domain controller, and checks whether that user is
allowed on this network. Then it tells the access point to let the client in or keep
it out.

## Antennas

There are two main types of antenna: omnidirectional and directional.

An omnidirectional antenna sends and receives in all directions around it. The
coverage is shaped like a doughnut, strong to the sides and weak straight above and
below. Most indoor access points use them.

A directional antenna focuses its signal into a narrower beam, which gives it longer
range in one direction and less everywhere else. Yagi, panel, and dish antennas are
all directional. Point-to-point links use them, and so do access points that cover a
long hallway or one side of a building.

## Autonomous and lightweight access points

An autonomous access point handles everything on its own, including its settings,
its security, and its clients. Each one is configured separately, which works well
for a small network with a few access points.

A lightweight access point (LWAP) does not work alone. A central wireless LAN
controller (WLC) configures it, manages its security, and coordinates it with the
other access points, including channel assignments, power levels, and roaming. The
access points talk to the controller over a tunnel, commonly CAPWAP. The controller
can be a hardware appliance, a virtual machine, or a cloud service.

Larger networks use lightweight access points because configuring hundreds of
access points one at a time is not practical, and keeping them consistent by hand is
harder still.

## Suggested practice: survey the wireless networks around you

Everything here is read-only and works on a Linux laptop with a wireless card.
Replace `<interface>` with your wireless interface name from `iw dev`, often `wlan0`
or a name starting with `wlp`.

1. Run `iw dev <interface> link` to see the network you are on. Note the BSSID, the
   frequency, and the channel width in the bitrate lines. A frequency near 2,400 MHz
   is the 2.4 GHz band, 5,150 to 5,900 MHz is 5 GHz, and 5,925 MHz or higher is
   6 GHz.
2. Run `iw dev <interface> scan dump` to list the networks your card has seen
   recently. For each one, look at `freq`, `primary channel`, `SSID`, and
   `Authentication suites`. `PSK` means WPA2-Personal, `SAE` means WPA3-Personal,
   and `IEEE 802.1X` means an enterprise network. If the list is empty, run
   `sudo iw dev <interface> scan` once to do a fresh scan.
3. Count the 2.4 GHz networks near you on channels 1, 6, and 11, and note any that
   sit on other channels. A network on channel 3 overlaps both 1 and 6.
4. Look for one SSID with several BSSIDs. Those are separate radios in one ESS. It
   might be a mesh system, an office network, or one dual-band router with a BSSID
   per band.
5. Run `iw reg get` to see the regulatory domain your card is following. Each line
   is a frequency range with its maximum channel width and power, and lines marked
   `DFS` are the ranges shared with radar.
6. On your own router, check which security mode is set (WPA2, WPA3, or mixed) and
   whether WPS is on. If you do not use WPS, turn it off.

A scan from one spot is only a sample. Signal changes from room to room, so scan
from a few places before drawing conclusions about coverage.

## Related pages

- [Wireless media](/learn/wireless-media) — how stations share a radio channel, and
  the 802.11 generations with their bands and maximum rates.
- [Switching technologies](/learn/switching-technologies) — the VLANs a guest
  network is placed on, and 802.1X on wired switch ports.
- [Network topologies](/learn/network-topologies) — mesh, star, and point-to-point
  layouts on wired networks, for comparison with the wireless ones here.
- [Network appliances](/learn/network-appliances) — access points and controllers
  alongside the other devices in the traffic path.
- [Zero-trust architecture](/learn/zero-trust-architecture) — per-user
  authentication taken further, where no network location is trusted by default.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [IEEE 802.11 Wireless LAN Working Group](https://www.ieee802.org/11/) — the
  wireless LAN standard and its amendments, including 802.11h (DFS and TPC), 802.11w
  (protected management frames), and 802.11s (mesh).
- [IEEE 802.1X](https://1.ieee802.org/security/802-1x/) — port-based network access
  control, and the supplicant, authenticator, and authentication server roles.
- [RFC 3748: Extensible Authentication Protocol](https://www.rfc-editor.org/rfc/rfc3748)
  — the EAP framework that 802.1X carries.
- [RFC 5216: EAP-TLS](https://www.rfc-editor.org/rfc/rfc5216) — mutual certificate
  authentication.
- [RFC 5281: EAP-TTLS](https://www.rfc-editor.org/rfc/rfc5281) — the tunneled method
  with a server certificate and inner credentials.
- [RFC 4851: EAP-FAST](https://www.rfc-editor.org/rfc/rfc4851) — the tunnel built
  from a Protected Access Credential.
- [RFC 2865: RADIUS](https://www.rfc-editor.org/rfc/rfc2865) — the authentication
  server protocol behind enterprise Wi-Fi.
- [RFC 7664: Dragonfly Key Exchange](https://www.rfc-editor.org/rfc/rfc7664) — the
  handshake that WPA3's SAE is based on.
- [RFC 8110: Opportunistic Wireless Encryption](https://www.rfc-editor.org/rfc/rfc8110)
  — the encryption behind Wi-Fi Enhanced Open.
- [RFC 5415: CAPWAP](https://www.rfc-editor.org/rfc/rfc5415) — the protocol between
  lightweight access points and their controller.
- [Wi-Fi Alliance: Security](https://www.wi-fi.org/discover-wi-fi/security) — WPA2,
  WPA3, Easy Connect, and Enhanced Open as certification programs.
- [FCC Part 15 rules](https://www.ecfr.gov/current/title-47/chapter-I/subchapter-A/part-15)
  — the unlicensed-device rules that set channel availability and power limits in
  the US.
- [iw(8)](https://man.archlinux.org/man/iw.8) — the Linux wireless configuration
  tool used in the practice section.

Channel availability and power limits differ by country, so check the rules where
the equipment will run.
