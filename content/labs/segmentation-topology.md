---
title: "S0 — Document the physical topology"
date: 2026-08-14
summary: "Record what is actually plugged in where before configuring a single VLAN — switch config, port map, PVIDs, per-node NIC names, and the out-of-band path for every cutover."
tags: [labs, networking, segmentation, vlan, documentation]
---

## Why this comes first

This is a sanitized planning template. Keep the completed port map, addresses,
and device identifiers in the private change record.

This stage changes nothing. It only records — and it is still the stage most
worth doing carefully, because every stage after it acts on what it produces.

The failure it prevents is specific: configuring VLANs against a *remembered*
port map is how you trunk the wrong port and lose management access to the
device you are in the middle of configuring. A port map written a year ago
describes a network that no longer exists.

## What to record

Work through the switch first, then the hypervisors, then the guests.

**The switch**

- [ ] Model and firmware version
- [ ] Exported configuration, stored somewhere private — not in a public repo
- [ ] Physical port → device → NIC mapping, for every port in use
- [ ] Current PVID and VLAN membership for every port in use

**The firewall / router**

- [ ] Which physical NIC carries WAN, and whether the ISP device is in routed
      or bridged mode
- [ ] The parent interface and its current assignments

**Each hypervisor node**

- [ ] The exact NIC names, **confirmed per node rather than assumed**. Interface
      naming differs between machines — `eno1` on one, a renamed alias on
      another — and assuming symmetry across nodes is a reliable way to apply a
      bridge change to the wrong interface.
- [ ] Which bridge each guest currently sits on

**Each guest**

- [ ] Its node, bridge, current address, and target zone

**Per cutover, not in general**

- [ ] Which controlled client and switch port provide an independent recovery
      path *during that specific change*. "I can get to the console" is not a
      plan; the private record needs the exact port and access method.

## The design this feeds

An example design separates systems by role:

| Role | Holds |
|---|---|
| Management | Hypervisor, cluster, and switch management |
| User | Approved workstations and personal devices |
| Service | Public and internal application guests |
| Administration | Narrow remote-administration entry points |
| Test | Disposable and rebuild-often machines |
| Untrusted | Internet-only client access |

Choose identifiers and ranges privately. The property that matters is that roles
are legible during troubleshooting without publishing the deployed mapping.

## WAN transit is a separate decision

Do not fold the ISP handoff into this stage to preserve a tidy port map. Before
placing it anywhere, answer:

- Is the ISP device routed or bridged?
- Which physical NIC carries WAN, and is the firewall's interface design
  compatible with that?
- **What happens if the node hosting the router VM is down?** A router that is a
  virtual machine on one hypervisor node is a single point of failure. That may
  be an acceptable trade in a lab — it is not an acceptable *unnamed* one.
- Is physical separation or an additional NIC available?

If a VLAN-backed WAN transit turns out to be unavoidable, document its exact
switch ports and prove that no access port or host bridge can join it
unintentionally.

## Verification

There is nothing to verify functionally — nothing changed. The check is whether
the document is usable by someone who is not you:

1. Hand the port map to someone else and ask them which port they would unplug
   to isolate the management network. If they can answer, it is complete.
2. Confirm every NIC name against the node itself, not against the other nodes.
3. Confirm the exported switch config actually restores. An export you have
   never restored is a file, not a backup.

## Stop conditions

- The switch cannot export its configuration, or the export cannot be restored
  in a test → resolve that before any VLAN work. Rollback depends on it.
- A device answers on the management network that you cannot identify → find out
  what it is first. An unidentified host inside the zone you are about to make
  trusted is the wrong thing to discover later.

## Done when

- [ ] Switch configuration exported and its restore tested
- [ ] Full port map with PVIDs recorded
- [ ] Per-node NIC names confirmed individually
- [ ] Per-guest table complete with target zones
- [ ] Out-of-band management path identified per cutover
- [ ] Every address on the management network attributed to a known device
