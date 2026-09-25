---
title: "S1 — Prepare without moving traffic"
date: 2026-08-14
summary: "Create every VLAN object inert, defined and validated but carrying nothing. Then rehearse the rollback for real before any host changes broadcast domain."
tags: [labs, networking, segmentation, vlan, rollback]
---

## The principle

Every object gets created inert. VLAN interfaces disabled, VLAN IDs defined
but no port membership changed, bridges written but not applied. Nothing carries
traffic any differently at the end of this stage than it did at the start.

Doing this as its own stage makes the first real cutover a small change instead
of a big one. If something breaks in the next stage, the new configuration is
already known to exist and parse, so there is one likely cause instead of
several.

## Procedure

1. **Export current configurations** — hypervisor networking, firewall, switch.
   These are your rollback baseline. The previous stage should have produced
   them. Re-export anything that has changed since.

2. **Firewall: create the VLAN objects and aliases with their interfaces
   disabled.** Aliases first. A policy matrix built on literal addresses is
   unmaintainable the first time a subnet moves, and you will move one.

3. **Switch: define the VLAN IDs without changing any access port or PVID.**
   Defining a VLAN on the switch does nothing until a port joins it. Define all
   six now so the later stages only change membership.

4. **Hypervisors: write the VLAN-aware bridge configuration per node, using
   each node's real interface name.** Interface names differ between nodes, so
   write separate configs instead of one template with an assumed name.

5. **Validate syntax without applying it:**

   ```bash
   ifquery --check -a
   ifreload -s          # syntax check only, if your ifupdown2 supports it
   ```

   Check which flags your version supports before relying on them.

   **Do not restart all nodes together.** A syntax error applied to every node
   at once takes down the whole cluster. Applied to one node, it is a problem
   you can walk to the console and fix.

6. **Write and rehearse the rollback** for one node and one switch port.

## The rollback rehearsal is the stage

Rehearsing means running the rollback for real. A rollback that has only been
written down has not been tested, and an outage is a bad time to find its
mistakes.

| Rehearse | Confirm |
|---|---|
| Node network config revert, **from the console** | Node returns to its management address, SSH and the web UI reachable, cluster peers reconnect |
| One switch port PVID and membership revert | The attached device regains its previous connectivity |
| Firewall config restore from export | Rules and interfaces match the pre-change export |

Save evidence of each one. Without it, "at least one tested rollback path" is
only a checklist item.

## Verification

The pass condition for this stage is that nothing changed:

- Every host still has the address it had before.
- The cluster is still healthy.
- The public request path still works.
- The switch reports the new VLAN IDs as defined, with no port membership
  differences from the export you took in step 1.

If any traffic behaves differently, something was applied that should not have
been. Find it before continuing.

## Stop conditions

- The syntax check fails → fix the file. Never apply a config "to see what
  happens" on a network you are also managing over.
- **A rollback rehearsal does not restore the previous state → stop the whole
  stage.** An unproven rollback means the next stage has no safety net, and the
  next stage is the one that moves real traffic.
- Any configuration export cannot be produced → you cannot revert what you
  cannot capture.

## Done when

- [ ] Current hypervisor, firewall, and switch configs exported, restore-tested,
      and stored outside the devices they protect
- [ ] All six VLANs defined on the firewall as disabled objects, with aliases
- [ ] All six VLAN IDs defined on the switch with no port membership changes
- [ ] Per-node bridge config written using each node's verified interface name
- [ ] Syntax validated on every node, applied to none
- [ ] Rollback rehearsed and evidenced for one node and one switch port
- [ ] Traffic behavior verified unchanged, so the network works the same as it
      did before this stage
