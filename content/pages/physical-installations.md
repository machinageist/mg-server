---
title: "Physical installations: racks, cabling, power, and cooling"
date: 2026-10-02
summary: "Where network equipment goes and what the room has to give it: distribution frames, racks and airflow, patch panels and cable pinouts, locks, power, and climate control."
tags: [education, networking, physical-layer, cabling, power, cooling]
---

## Overview

Switches, routers, and access points are physical objects. Each one has to be
mounted somewhere, cabled, powered, kept cool, and kept away from people who should
not touch it. When one of those is done badly, the network has problems that no
configuration change will fix. A switch that overheats shuts down. A cable that was
terminated wrong drops frames. A circuit with too much load trips its breaker and
takes a whole rack down with it.

This page covers the physical side of an installation: the rooms where cabling
comes together, the racks that hold the equipment, how cables are run and
terminated, how the equipment is locked up and powered, and the temperature,
humidity, and fire protection of the room. The cables themselves are on the
[wired media](/learn/wired-media) page, and the connectors on their ends are on the
[transceivers and connectors](/learn/transceivers) page.

## Distribution frames

A building's cabling does not run from every device to one large switch. It comes
together at a few central points called distribution frames. The name comes from
telephone exchanges, where the frame was a metal rack holding the termination for
every line. Today a distribution frame is a room or closet with one or more racks
in it.

The cabling standards call these rooms the equipment room and the
telecommunications room. MDF and IDF are the older telephone names, and they are
still what most people say.

### Main distribution frame

The main distribution frame (MDF) is the center of the building's network. It is
usually where the outside lines come in. A provider's circuit ends at the
demarcation point, where the provider's responsibility stops and the building
owner's starts, and the MDF sits at or near it.

The MDF holds the core equipment: the routers and firewalls that connect to the
provider, the core switches, and often servers. It is usually a dedicated, locked
room, or part of a data center.

### Intermediate distribution frames

A small office can run every cable back to the MDF. A larger building cannot,
because twisted-pair Ethernet only reaches 100 m, and that figure includes the
patch cables at both ends.

An intermediate distribution frame (IDF) is a smaller room or closet closer to the
users, often one per floor or one per wing. The cables from that area's wall jacks
end in the IDF, where access switches connect them. Each IDF then connects back to
the MDF over a few uplinks, usually fiber.

This also saves cable. A few hundred devices on a floor share a handful of uplinks
to the MDF, where otherwise each one would need its own run the whole way.

The result is a star with the MDF in the middle. It lines up with the
[three-tier model](/learn/network-topologies#the-three-tier-hierarchical-model) on
the network topologies page: access switches sit in the IDFs, and the distribution
and core switches usually sit in the MDF.

### Locations

Where these rooms go affects performance, security, and how easily the network can
grow.

- The MDF goes near the point where the provider's lines enter the building, in a
  room that can be locked, cooled, and given its own power.
- Each IDF goes where no cable run to a wall jack is longer than 90 m. In a
  multi-story building the IDFs are often stacked one above the other, which keeps
  the runs between floors short.
- Every room needs space for more equipment than it holds on the first day.

Spreading equipment out this way makes the network easier to maintain and to grow.
Adding users to one floor means adding a switch to that floor's IDF. The rest of
the building is not touched.

## Racks

A rack is a metal frame of a standard width that equipment bolts into. Routers,
switches, servers, patch panels, and power equipment are all built to fit it.
Stacking equipment in racks fits more of it into the same floor space, and it puts
cabling, power, and airflow in predictable places.

The common rack is 19 inches wide, measured across the front panel of the
equipment including its mounting ears.

### Rack size

Height is measured in rack units. One rack unit (1U) is 1.75 inches, or 44.45 mm.
Equipment is built in whole units:

- Most switches and patch panels are 1U.
- Larger servers are often 2U.
- Storage arrays are often 4U or more.

A full-height rack is usually 42U, which is a little over six feet of mounting
space. Wall-mounted cabinets for a small IDF are much shorter.

Height is not the only dimension. Servers are much deeper than switches, and a rack
that is too shallow will not hold them. A two-post rack suits patch panels and
switches. A four-post rack or an enclosed cabinet supports deep, heavy equipment
at the front and the back.

Planning a rack means adding up the units of everything going into it, then leaving
room for cable management and for growth. Power distribution units are often
mounted vertically at the back of the rack, where they use no rack units at all.

### Hot and cold aisles

Rack equipment is cooled by fans that pull air in on one side and push it out the
other. A server pulls cool air in at the front and pushes hot air out the back.

If the racks are arranged so one row's exhaust blows into the next row's intake,
the second row runs hot. Data centers avoid this by lining racks up in rows that
face each other. The fronts of two rows face a cold aisle, where cooled air is
supplied. The backs face a hot aisle, where the exhaust is collected and sent back
to the cooling units.

Blanking panels cover the unused units in a rack so hot air cannot loop back around
to the front.

### Port-side intake and exhaust

A server's network ports are on its back. A switch's ports are on its front. To
keep the cables short, a switch at the top of a server rack is often mounted
backward, with its ports facing the rear of the rack where the servers' ports are.
That puts the switch's ports in the hot aisle and its power supplies in the cold
aisle.

The fans have to move air the right way for how the switch is mounted, so switches
are sold with either airflow direction:

- Port-side exhaust means cool air enters at the power supply end and hot air
  leaves at the port end. The ports face the hot aisle.
- Port-side intake means cool air enters at the port end and hot air leaves at the
  power supply end. The ports face the cold aisle.

On many data center switches the fans and power supplies are replaceable modules,
and each one comes in both directions. Vendors mark them so they can be told apart.
Cisco's Nexus switches use burgundy for port-side intake and blue for port-side
exhaust.

All the fans and power supplies in one switch have to move air the same way. A
switch installed with the wrong airflow pulls in hot exhaust, and it can overheat
and shut down.

## Cabling

Good cable choice and careful installation keep the signal clean and the network
reliable. A building's cabling is planned in two parts, the backbone and the
horizontal runs, and both end on panels in the distribution frames.

### Backbone and horizontal cabling

Backbone cabling connects the distribution frames to each other: MDF to IDF, floor
to floor, and building to building. It is also called vertical cabling, because in
a multi-story building it runs up through the floors.

The backbone is usually fiber. It has to carry the combined traffic of everything
behind it, often over more than 100 m. Fiber needs protection, so it runs through
conduit or enclosed raceways, and it cannot be bent sharply. Every fiber cable has
a minimum bend radius. A common rule is ten times the cable's outside diameter once
it is installed, and twenty times while it is being pulled. A tighter bend lets
light leak out of the core, and the link loses signal or fails.

Horizontal cabling runs from an IDF to the wall jacks on its floor. It is usually
twisted-pair copper. The run is limited to 90 m, which leaves 10 m of the 100 m
channel for the patch cables at both ends.

A drop cable is the last short piece, from the wall jack to the device. The name is
also used loosely for the whole run to one jack.

The path of every run should be planned before the cable is pulled:

- Go around obstacles, and keep copper away from sources of interference such as
  motors and fluorescent lights.
- Support the cable in trays, ladder racks, or hooks so it does not hang by its own
  weight.
- Label both ends of every run.
- Use plenum-rated cable in air-handling spaces. See
  [plenum cabling](/learn/wired-media#plenum-cabling).

### Patch panels

A cable from a wall jack does not plug straight into a switch. It ends on a patch
panel, which is a rack-mounted panel of jacks, usually 1U with 24 or 48 ports. Each
cable run is permanently terminated on the back of the panel. The matching jack on
the front connects to a switch port with a short patch cable.

The cable in the walls has solid conductors. They carry a signal well, but they
break if they are flexed often. Patch cables have stranded conductors made to be
bent and moved. The patch panel means the permanent cabling is terminated once and
then left alone.

It also keeps changes simple. Moving a wall jack to a different switch or a
different VLAN means moving one patch cable. The labels on the panel match the
labels on the wall jacks, so a problem can be traced to one run and tested from the
panel.

### Fiber distribution panels

A fiber distribution panel does the same job for fiber. The backbone cables end
inside it. Each strand is terminated or spliced to a short lead with a connector on
it, and the spare length is coiled in trays that keep it above its minimum bend
radius.

The front of the panel has a connector for each strand, usually LC or SC. Short
fiber patch cables connect those to the transceivers in the switches. The
[connector types](/learn/transceivers#connector-types) table on the transceivers
page shows what each one looks like.

The enclosure matters more for fiber than for copper. A bare strand is thin glass,
and dust on the end of a connector blocks light, so unused ports keep their dust
caps on.

### T568A and T568B

A twisted-pair Ethernet cable has eight wires in four pairs. Each pair has a color:
orange, green, blue, or brown. One wire of the pair is solid colored, and the other
is white with a stripe of that color. The two wires are twisted together so that
interference reaches both about equally and cancels out.

The plug on the end has eight pins. The cabling standard, TIA-568, defines two
orders for the wires:

| Pin | T568A | T568B |
|---:|---|---|
| 1 | White/green | White/orange |
| 2 | Green | Orange |
| 3 | White/orange | White/green |
| 4 | Blue | Blue |
| 5 | White/blue | White/blue |
| 6 | Orange | Green |
| 7 | White/brown | White/brown |
| 8 | Brown | Brown |

The only difference is that the orange and green pairs trade places. Either one
works. What matters is that both ends of a run match and that a building uses one
order throughout. T568B is the more common of the two in the US, and T568A is more
common in much of the rest of the world.

Pins 3 and 6 are one pair, even though they are not next to each other. That pair
sits on either side of the blue pair on pins 4 and 5. A plug wired with its pairs
on 1-2, 3-4, 5-6, and 7-8 passes a simple continuity test, because every pin still
connects to the same pin at the far end. It fails at speed, because the signal on
pins 3 and 6 is no longer carried on two wires that are twisted together. This
fault is called a split pair.

### Straight-through and crossover cables

In 10 and 100 Mbps Ethernet, one pair sends and one pair receives. A port is wired
one of two ways:

- An MDI port sends on pins 1 and 2 and receives on pins 3 and 6. Computers,
  servers, and routers have MDI ports.
- An MDI-X port sends on pins 3 and 6 and receives on pins 1 and 2. The X stands
  for crossover. Switches and hubs have MDI-X ports.

A straight-through cable has the same wire order on both ends, so pin 1 connects to
pin 1, pin 2 to pin 2, and so on. It connects an MDI port to an MDI-X port, where
one side's sending pair lands on the other side's receiving pair. Computer to
switch and router to switch are both straight-through connections.

A crossover cable has T568A on one end and T568B on the other. That swaps pins 1
and 2 with pins 3 and 6. It connects two ports of the same kind: switch to switch,
computer to computer, or computer to router. A router's ports are wired like a
computer's, so a router counts as an end device here.

Gigabit Ethernet uses all four pairs, and every pair sends and receives at the same
time. A gigabit crossover cable swaps the blue and brown pairs as well.

Crossover cables are now rare because of Auto MDI-X. A port with this feature
detects which pins the other end is sending on and swaps its own to match, so
either cable works between any two devices. The Ethernet standard makes it optional
for gigabit ports, but nearly all of them have it. The two cable types still matter
with old 10 and 100 Mbps equipment, and on ports where the feature has been turned
off.

## Locks and physical access

Anyone who can touch a device can unplug it, plug something into it, or carry it
away. Most network devices also let a person at the console port reset the
password. Physical access is close to full control, so the equipment has to be
locked up.

- Lockable racks and cabinets are the simplest control. An IDF is often a closet
  that other people need to enter, and a locked cabinet keeps the switch and the
  patch panel away from them.
- Server rooms and data centers use doors that open with a keycard or a biometric
  reader. A card can be turned off without changing a lock.
- Tailgating is following an authorized person through a door before it closes. An
  access control vestibule stops it. This is a small space with two doors, where
  the second door will not open until the first has closed, so people go through
  one at a time.
- Access systems should keep a timestamped log of who opened which door. Cameras
  record what the log cannot.

## Power

Network equipment needs steady power. A gap of a fraction of a second can reboot a
switch, and a server that loses power in the middle of a write can corrupt data.

### Voltage, current, and power

Three units describe electrical power:

- Voltage, in volts (V), is electrical pressure. It is how hard the supply pushes.
- Current, in amperes or amps (A), is how much electricity is flowing.
- Power, in watts (W), is the rate at which energy is delivered. It is voltage
  multiplied by current: W = V × A.

Wall power is alternating current (AC). In the US it is 120 V at 60 Hz. In Europe
it is 230 V at 50 Hz, and most of the world uses 220 to 240 V. Most modern power
supplies accept anything from 100 to 240 V.

Higher voltage delivers the same power with less current. A 1,200 W load draws 10 A
at 120 V and about 5.2 A at 230 V. Less current means thinner wiring and more
equipment per circuit, so US data centers often feed their racks at 208 or 240 V.

Most devices use single-phase power, which is one AC waveform. Large data centers
bring three-phase power to the rack. That is three waveforms offset from each
other, and it delivers more power over wiring of the same size.

### Power load

The power load is the total power drawn by everything on a circuit. Every device
added to the circuit adds to the load.

Each circuit is protected by a breaker with a rating in amps, and the circuit's
capacity is that rating multiplied by the voltage. A 30 A circuit at 120 V carries
30 × 120 = 3,600 W. If each device draws 240 W, which is 2 A at 120 V, then
3,600 ÷ 240 = 15 devices bring the circuit to its limit.

A circuit should not be planned to the breaker's limit. The US electrical code
treats a load that runs for three hours or more as continuous, and it limits a
continuous load to 80% of the breaker's rating. Network equipment runs all day, so
all of it counts. On the same 30 A circuit, that is 24 A, or 2,880 W, or 12 of the
240 W devices.

A few more things affect the count:

- The wattage on a device's label is its maximum. The real draw is usually lower,
  and a metered power distribution unit shows it.
- A [PoE switch](/learn/network-appliances#switches) draws power for the phones,
  cameras, and access points it feeds, on top of what it uses itself.
- Many switches and servers have two power supplies, each plugged into a different
  circuit. Each circuit then has to be able to carry the whole rack alone.
  Otherwise losing one circuit overloads the other.

### Uninterruptible power supplies

An uninterruptible power supply (UPS) sits between the wall power and the equipment
and holds a battery. When the wall power fails, the battery takes over the load,
either with no gap or within a few milliseconds, depending on the design.

The battery lasts for minutes. That is long enough for one of three things:

- The power comes back, and the outage was short enough to ride through.
- A backup generator starts and takes over.
- The servers shut down cleanly before the battery runs out.

A UPS can also signal the servers plugged into it to start shutting down when its
battery runs low.

A UPS is rated in volt-amperes (VA) and in watts. The watt rating is the one to
compare with the power load. The lighter the load, the longer the battery lasts.
The batteries wear out, and they have to be tested and replaced on a schedule.

### Power distribution units

A power distribution unit (PDU) distributes power inside a rack. It takes one
circuit and provides outlets for all the equipment in the rack, with a breaker to
protect against overload.

- A basic PDU is a heavy-duty power strip.
- A metered PDU shows how much current the rack is drawing, so the load can be
  checked against the circuit.
- A switched PDU lets an administrator turn each outlet off and on over the
  network, which is how a hung device gets rebooted without a trip to the rack.

A PDU does not keep equipment running through an outage. The UPS does that. In a
typical setup, power goes from the wall to the UPS, from the UPS to the PDU, and
from the PDU to the equipment.

## Environmental factors

Network equipment works best within a certain range of temperature and humidity.
Outside that range it fails sooner, and at the extremes it fails right away.

### Temperature

Equipment turns nearly all the power it draws into heat. A rack that draws 3 kW
needs 3 kW of heat removed. Equipment that gets too hot slows down, shuts itself
off, or wears out early.

A common target for a server room is 20 to 25 °C (68 to 77 °F). ASHRAE, the
engineering society that publishes the guidelines most data centers follow,
recommends a wider range of 18 to 27 °C (64 to 81 °F). Both figures are for the air
entering the equipment. The average temperature of the room is not what counts.

A computer room air conditioner (CRAC) is a cooling unit built for this job. It
runs all day and all year, it moves much more air than a comfort air conditioner,
and it controls humidity as well as temperature.

Some high-density installations use liquid cooling. Coolant is piped to the rack,
or directly to the hottest chips, because liquid carries much more heat than air.

### Humidity

Humidity causes trouble at both ends:

- When it is too high, water condenses on cold surfaces. Water in electrical
  equipment causes short circuits and, over time, corrosion and mold.
- When it is too low, static electricity builds up easily, and a static discharge
  can damage components.

The usual target is 40% to 60% relative humidity. ASHRAE's recommended range is
wider. It is set by dew point, from −9 °C to 15 °C, with relative humidity no
higher than 70% in clean air.

An environmental monitoring system puts temperature and humidity sensors in the
room and in the racks. It sends an alert when a reading goes out of range, and it
can be tied into the HVAC controls so the cooling adjusts on its own.

### Fire suppression

Water puts out a fire and ruins the electronics it lands on. Server rooms use two
layers of protection, with a gas system first and a water system behind it.

A clean agent system floods the room with a gas that puts out the fire, does not
conduct electricity, and leaves no residue. There are two kinds:

- Inert gases, such as nitrogen and argon, lower the oxygen level below what a fire
  needs.
- Chemical agents, such as FK-5-1-12 and HFC-227ea, put the fire out mainly by
  absorbing its heat.

Both replaced Halon, which was phased out because it damages the ozone layer. The
room has to be sealed well enough to hold the gas, and an alarm sounds before the
discharge so people can leave.

A pre-action sprinkler system is the backup, and it uses water. Its pipes are kept
dry. Water enters them only after a smoke or heat detector signals a fire, and it
then comes out only from the sprinkler heads that the heat has opened. Two things
have to happen before any water reaches the equipment, so a broken pipe or a bumped
sprinkler head does not flood the room.

Fire-resistant construction is the last part. Rated walls, doors, floors, and
insulation keep a fire in the rest of the building out of the room for a set time.

## Suggested practice: check the cabling, power, and heat of your own equipment

Everything here works on a home network and a Linux machine. Replace `<interface>`
with a wired interface name from `ip -br link`.

1. Read the wiring of a patch cable. Hold a plug with the clip facing away from you
   and the cable hanging down, and read the wire colors from left to right. White
   and orange first is T568B. White and green first is T568A. Then compare the two
   ends. The same order on both is a straight-through cable, and a different order
   is a crossover cable.
2. Run `ethtool <interface>` and find the `MDI-X` line. `on (auto)` or `off (auto)`
   means the port detected the pinout and set itself. `Unknown` means the link is
   down or the driver does not report it.
3. Work out the power load on one circuit. List every device plugged into it, read
   the watts from each label or power adapter, and add them up. Compare the total
   with 80% of the circuit's capacity. A 15 A circuit at 120 V carries 1,800 W, so
   the limit for a continuous load is 1,440 W.
4. If you have a UPS with a USB port, read it with Network UPS Tools (NUT). Once
   NUT is set up, `upsc <ups name>` lists what the UPS reports. `ups.load` is the
   load as a percentage of capacity, `battery.runtime` is the time left in seconds,
   and `input.voltage` is the wall voltage.
5. Run `sensors` to read the temperatures and fan speeds your machine reports. Run
   it again under load and compare.
6. Find which way air moves through each device, and check that nothing blocks the
   intake or the exhaust. A switch pushed against a wall or stacked under a warm
   router is the home version of the wrong airflow direction.

The numbers on the labels are maximums, so the total is an estimate. For anything
that involves the building's wiring, ask an electrician.

## Exam key points

Where this page's material shows up in the exam objectives, and what each exam
expects beyond it.

### CCNA 200-301

Objective 1.3 in v1.1 compares physical interface and cabling types, 1.4 covers
interface and cable issues, and 5.2 lists physical access control as part of a
security program. In v2.0, which replaces it in February 2027, 1.1 adds pinouts and
cable types to the cable problems to diagnose, and physical access control is no
longer listed. Racks, power, and cooling are not on either blueprint.

- A straight-through cable connects unlike devices, such as a PC or a router to a
  switch. A crossover cable connects like devices: switch to switch, PC to PC, and
  PC to router.
- At 10 and 100 Mbps the pairs in use are on pins 1 and 2 and pins 3 and 6. A
  crossover cable swaps them. Gigabit uses all four pairs.
- Auto-MDIX lets a port work with either cable. On Catalyst switches it is on by
  default as `mdix auto`, and it only works when speed and duplex are left on auto.
- A rollover cable is not covered on this page. It connects a PC to a console port,
  and its pins are reversed end to end: 1 to 8, 2 to 7, and so on.
- For 5.2 in v1.1, physical access control means locked rooms and racks, badge
  readers, and a record of who went in.

### Network+ N10-009

Objective 2.4, important factors of physical installations, is this page's outline
item for item. Locks also appear under physical security in 4.1, rack diagrams and
cable maps under documentation in 3.1, and termination faults in 5.2.

- The MDF is the central point, usually where the outside lines come in. IDFs sit
  closer to the users and uplink to the MDF.
- A rack unit is 1.75 inches, and a standard full rack is 42U.
- Port-side exhaust sends hot air out the port end, so the ports face the hot
  aisle. Port-side intake is the reverse.
- A patch panel terminates copper runs. A fiber distribution panel terminates
  fiber.
- A UPS provides battery power for minutes, enough to shut down cleanly or start a
  generator. A PDU distributes power inside a rack.
- Watts are volts multiplied by amps. The power load is the total draw on a
  circuit, and it has to stay under what the circuit can carry.
- High humidity causes condensation and low humidity causes static. 40% to 60%
  relative humidity is the usual target.
- A clean agent system puts out a fire with gas and no water. A pre-action
  sprinkler holds its water back until a fire is detected.
- For 5.2, improper termination and transposed TX and RX both trace back to the
  pinouts here: wires in the wrong order in a plug, or the wrong one of
  straight-through and crossover on ports without Auto MDI-X.

## Related pages

- [Wired media](/learn/wired-media) — the cable categories, fiber types, and plenum
  ratings of the cables this page runs and terminates.
- [Transceivers and connectors](/learn/transceivers) — the plugs on the ends of
  those cables and the modules they connect to.
- [Network topologies](/learn/network-topologies) — the tiered designs that the MDF
  and IDF layout follows.
- [Network appliances](/learn/network-appliances) — the switches, routers, and
  firewalls that fill the racks, and PoE as part of the power load.
- [Switching technologies](/learn/switching-technologies) — speed and duplex
  negotiation on the ports these cables plug into.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [IEEE 802.3 Ethernet Working Group](https://www.ieee802.org/3/) — the MDI and
  MDI-X pin assignments, and automatic MDI/MDI-X configuration for 1000BASE-T in
  Clause 40.
- [TIA-568 structured cabling](https://tiaonline.org/) — the T568A and T568B pin
  assignments and the 90 m limit on horizontal runs, referenced and not reproduced.
- [ASHRAE TC 9.9 thermal guidelines reference card](https://www.ashrae.org/file%20library/technical%20resources/bookstore/supplemental%20files/therm-gdlns-5th-r-e-refcard.pdf)
  — the recommended temperature and humidity ranges for air-cooled equipment.
- [NFPA 75](https://www.nfpa.org/codes-and-standards/nfpa-75-standard-development/75)
  — fire protection for information technology equipment.
- [NFPA 2001](https://www.nfpa.org/codes-and-standards/nfpa-2001-standard-development/2001)
  — clean agent fire extinguishing systems, both inert gas and halocarbon.
- [NFPA 13](https://www.nfpa.org/codes-and-standards/nfpa-13-standard-development/13)
  — sprinkler systems, including pre-action systems and their interlocks.
- [NFPA 70, the National Electrical Code](https://www.nfpa.org/codes-and-standards/nfpa-70-standard-development/70)
  — the continuous load rule for branch circuits, in 210.20(A).
- [Cisco Nexus 92300YC hardware installation guide](https://www.cisco.com/c/en/us/td/docs/switches/datacenter/nexus9000/hw/n92300yc_hig/guide/b_c92300yc_nxos_mode_hardware_install_guide/b_c92300yc_nxos_mode_hardware_install_guide_chapter_01.html)
  — port-side intake and exhaust airflow, the module colors, and which aisle the
  ports face.
- [IEC 60038](https://webstore.iec.ch/en/publication/153) — the standard supply
  voltages, including 230 V.
- [ethtool(8)](https://man.archlinux.org/man/ethtool.8),
  [sensors(1)](https://man.archlinux.org/man/sensors.1), and
  [upsc(8)](https://networkupstools.org/docs/man/upsc.html) — the Linux tools used
  in the practice section.

Rack dimensions come from EIA-310, which is sold by its standards body and not
published openly, as are TIA-568 and IEC 60038. NFPA codes can be read online for
free with an account. Electrical and fire codes differ by country and by city, so
the local code is the reference for a real installation.
