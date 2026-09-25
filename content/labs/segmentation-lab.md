---
title: "S2 — Prove the isolated test zone first"
date: 2026-08-14
summary: "The stage that proves the VLAN design works, run on disposable machines. It includes a deliberate misconfiguration, diagnosed and rolled back."
tags: [labs, networking, segmentation, vlan, firewall]
---

## Why LAB goes first

This is the stage that can break the least. The machines in this zone are
disposable, so if it goes wrong, nothing public breaks and nothing you depend on
stops.

This is the stage where you find out whether your VLAN design works. Everything
after it repeats a pattern you have already proven. A failure here is cheap to
learn from, which is why it comes first, before you have moved anything that
matters.

## Procedure, in this order

1. **Trunk the host port** for the node hosting your chosen test guest: retain
   the recovery network and add only the test tag.
2. **Make that node's bridge VLAN-aware.** One node only. Leave the others
   alone until this one is proven.
3. **Enable the test zone on the firewall**, with DHCP or static routing as
   appropriate.
4. **Apply the default-deny test policy before moving the guest.** A lab machine
   that lands on a permissive VLAN gets east-west access you never intended,
   right when you are least likely to be watching.
5. **Tag one** disposable guest for the test zone.
6. **Verify** address, gateway, DNS, and updates, and verify that the
   management, servers, and trusted zones are denied.
7. **Break it on purpose**, below.

Do not move the second lab guest until the first has passed and rollback has
been demonstrated.

## Break it on purpose

Induce one misconfiguration, observe it, diagnose it, roll it back, and keep
the evidence. Pick one:

- Tag the guest for a VLAN the trunk does not carry → expect no DHCP, no
  gateway
- Set the switch port PVID wrong → expect traffic in the wrong broadcast domain
- Leave the bridge non-VLAN-aware while tagging the guest → expect the tag
  stripped and the guest on the wrong subnet

Capture what you changed, what the symptom looked like, how you diagnosed it,
the revert, and confirmation of restored state. Do not skip this.

The diagnosis is the important part. Anyone can break a network. The skill is
tracing a symptom back to its cause. This is also the most useful thing the
stage produces, because it leaves you with a rollback you have run, not just one
you have written down.

## Verification

**Positive** — from the lab guest, all of these should work:

Verify an address, default route, name resolution, and permitted update access.

**Negative** — all of these must fail. Failing is the pass condition:

Verify representative management, service, and user destinations are denied.
Record the representative targets and the result of each probe in the evidence
record.

Record both sets. Positive tests only prove the VLAN carries traffic. They do
not prove it contains traffic, and containing traffic is the reason to segment.

## Stop conditions

- The node loses cluster peer connectivity when its bridge becomes VLAN-aware →
  revert from the console immediately. If that node also hosts the router,
  losing it takes the network with it.
- The negative tests pass, meaning the lab guest can reach management → the
  policy is wrong. Revert before investigating, not after.
- The deliberate-failure rollback does not restore state → stop the project and
  fix rollback first. Everything after this stage depends on being able to undo
  a change.

## Done when

- [ ] Host port trunked, bridge VLAN-aware, cluster still healthy
- [ ] Test zone live on the firewall with default-deny applied before the guest
      moved
- [ ] One lab guest on the LAB range with working gateway, DNS, and updates
- [ ] All negative tests fail as required, evidenced
- [ ] Deliberate failure induced, diagnosed, rolled back, evidence preserved
- [ ] Only then: second lab guest moved and re-verified
