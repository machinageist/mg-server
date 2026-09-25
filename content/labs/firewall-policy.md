---
title: "Firewall and router configuration"
date: 2026-08-14
summary: "Prove which device owns the gateway before designing around it, name virtualization dependencies, and build policy on aliases rather than literal addresses."
tags: [labs, networking, firewall, policy, opnsense]
---

## Settle one question before anything else

Before using a virtual firewall design, prove which device owns the gateway
address.

Two possibilities, and they lead to different projects:

- **The firewall VM owns the gateway address.** It enforces policy between
  zones, the design works as written, and everything else depends on it.
- **The ISP device owns it.** The firewall is not in the traffic path at all,
  and every rule in the policy matrix has nowhere to be enforced. The whole
  segmentation design needs rework before any VLAN is prepared.

Determine which, before preparing anything:

```bash
# From a host on the management network
ip neigh show <the gateway address>   # read the MAC
```

Then compare that MAC against the firewall VM's virtual NIC. A hypervisor OUI
means a guest owns the address. A hardware vendor OUI means physical equipment
does. On the node hosting it, read the VM's config and compare directly.

This is the single most consequential unknown in the project, and it costs ten
minutes to resolve.

## The single point of failure nobody named

A firewall virtual machine on one hypervisor creates a node dependency. If that
node is down, routed zones lose their gateway at the same moment the operator is
trying to repair the host.

Write this down now instead of finding out during an outage. None of the options
is free:

- **Accept it**, with a documented manual failback path. This is the cheapest
  option.
- **A second firewall instance with address failover** on another node. It gives
  real redundancy and adds real complexity.
- **Physical router hardware.** Removes the dependency, costs money and a NIC.

Do not add high availability until the base design and its recovery path have
been proved. Complexity added to an unproven base usually creates more failure
modes than it removes.

## Interface plan

Create one interface per role, prepared disabled during the preparation
stage and enabled one stage at a time. Record the addressing plan before
activation.

The [preparation stage](/labs/segmentation-prepare) covers creating interfaces
inert before any client moves.

## Rules: aliases, not literals

Create aliases for every network and service group before writing a single
rule. A matrix built on literal addresses has to be rewritten by hand the first
time a subnet moves, and a subnet moves during the servers cutover.

Rules belong on the interface where traffic enters. A rule on the wrong interface
either does nothing or does something surprising, and from the other end the two
look the same.

## Backups are the rollback path

- [ ] Export after every accepted change, not on a schedule
- [ ] Store the export outside the firewall itself
- [ ] Store exports encrypted and outside the repository. They contain rules,
      keys, and the complete interface map
- [ ] Practice a restore, not only an export. A backup you have never restored
      is untested.

## Verification

```bash
# Per zone, as each comes online
ping -c2 <that zone's gateway>
dig +short @<that zone's gateway> example.com
```

The allows are the easy part. Each segmentation stage lists the denies that
matter for its zone, and those are the tests to record.

## Stop conditions

- The gateway turns out to be owned by the ISP device → stop and redesign.
  Continuing would build a policy matrix with no enforcement point.
- A configuration export cannot be produced → resolve that before any rule
  change. You cannot revert what you cannot capture.

## Done when

- [ ] Gateway ownership proven and recorded
- [ ] Internal interface and zone configuration documented
- [ ] Config exported, stored securely, and a restore tested
- [ ] Aliases defined before rules
- [ ] The single-node dependency explicitly acknowledged in the topology
      document, with the chosen mitigation named
