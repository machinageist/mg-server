---
title: "Firewall and router configuration"
date: 2026-08-14
summary: "Prove what the firewall VM actually is before designing around it, name the single point of failure it introduces, and build the policy matrix on aliases rather than literal addresses."
tags: [labs, networking, firewall, policy, opnsense]
---

## Settle one question before anything else

The firewall VM exists and has an installer image attached. The gateway address
routes traffic and answers DNS. What is **not** proven is that those are the
same thing.

Two possibilities, and they lead to different projects:

- **The firewall VM owns the gateway address.** It is the inter-zone policy
  enforcement point, the design works as written, and it is load-bearing
  infrastructure.
- **The ISP device owns it.** The firewall is not in the traffic path at all,
  and every rule in the policy matrix has nowhere to be enforced. The whole
  segmentation design needs rework before any VLAN is prepared.

Determine which, before preparing anything:

```bash
# From a host on the management network
ip neigh show <the gateway address>   # read the MAC
```

Then compare that MAC against the firewall VM's virtual NIC. A hypervisor OUI
means a guest owns the address; a hardware vendor OUI means physical equipment
does. On the node hosting it, read the VM's config and compare directly.

This is the single most consequential unknown in the project, and it costs ten
minutes to resolve.

## The single point of failure nobody named

The firewall is a virtual machine on one hypervisor node. If that node is down,
the router is down — which means every zone loses its gateway, including
management, at exactly the moment you are trying to fix that node.

State this plainly rather than discovering it during an outage. The options,
none of them free:

- **Accept it**, with a documented manual failback path. Cheapest, and honest.
- **A second firewall instance with address failover** on another node. Real
  redundancy, real complexity.
- **Physical router hardware.** Removes the dependency, costs money and a NIC.

Do not solve it by adding high availability to a cluster that has not finished
recovering. Complexity added to an unproven base is how the original outage
happened.

## Interface plan

Create one interface per zone, prepared **disabled** during the preparation
stage and enabled one stage at a time. Each holds its zone's gateway address —
the `.1` of its `/24`.

The zones are management, trusted, servers, admin, lab, and guest. The
[preparation stage](/labs/segmentation-prepare) covers creating them inert.

## Rules: aliases, not literals

Create aliases for every network and service group **before** writing a single
rule. A matrix built on literal addresses has to be rewritten by hand the first
time a subnet moves — and during the servers cutover, a subnet does exactly
that.

**Rules belong on the interface where traffic enters.** A rule on the wrong
interface either does nothing at all or does something surprising, and the two
failure modes look identical from the far side.

## Backups are the rollback path

- [ ] Export after **every accepted change**, not on a schedule
- [ ] Store the export outside the firewall itself
- [ ] **Never commit an unsanitized export.** It contains rules, keys, and your
      full internal topology.
- [ ] Practise a **restore**, not just an export. Untested backups are hope.

## Verification

```bash
# Per zone, as each comes online
ping -c2 <that zone's gateway>
dig +short @<that zone's gateway> example.com
```

The allows are the easy half. Each segmentation stage carries the denies that
matter for its zone, and those are the tests worth recording.

## Stop conditions

- The gateway turns out to be owned by the ISP device → stop and redesign.
  Continuing would build a policy matrix with no enforcement point.
- A configuration export cannot be produced → resolve that before any rule
  change. You cannot revert what you cannot capture.

## Done when

- [ ] Gateway ownership proven and recorded
- [ ] Internal interface and zone configuration documented — currently a blank
- [ ] Config exported, stored privately, and a restore tested
- [ ] Aliases defined before rules
- [ ] The single-node dependency explicitly acknowledged in the topology
      document, with the chosen mitigation named
