---
title: "S6 — Management plane last"
date: 2026-08-14
summary: "The stage that looks cosmetic and is the most dangerous — re-tagging the network that carries cluster traffic and your own access to fix whatever you break."
tags: [labs, networking, segmentation, cluster, console]
---

## Why this is last, and why it is the most dangerous

Move a cluster's management plane only after every lower-risk zone has proved the
cutover and rollback pattern.

The change can affect:

- hypervisor management, both SSH and the web UI;
- **cluster membership traffic**; and
- your own access to fix whatever you break.

A tagging mismatch can isolate a node or cost the cluster quorum. Record exact
cluster membership and a tested recovery path for each node before cutover.

## Console access, per node, non-negotiable

Not SSH. Not the web UI. A physical or out-of-band console on the node you are
changing, open and *confirmed working* before you change anything.

Every other stage in this program could be recovered over the network. This one
is the network.

## Procedure

1. **Keep a direct console on the node being changed.**
2. **Convert one host path and one switch port at a time.**
3. **Verify cluster peer connectivity and quorum after each node** using the
   platform's documented health checks.

4. **Verify authorized management paths and explicit denies** from controlled
   test clients.
5. Only then move to the next node.

## The specific hazard: tagging and cluster traffic together

Cluster membership protocols are latency-sensitive and unhappy about transient
layer-2 changes. A port that briefly carries nothing while PVID changes take
effect can trip a token timeout, and a token timeout on the wrong node costs you
quorum.

Sequence matters:

- Prepare the trunk to carry **both** the current native VLAN and tagged
  management, before removing anything.
- Move the host onto the tagged interface.
- Remove the native VLAN only after the node is confirmed healthy on tagged.

Overlapping the two rather than cutting over hard is what keeps the window short
enough that the cluster does not notice.

## Stop immediately on any of these

This list is not advisory:

- cluster token loss;
- retransmits;
- any quorum change; or
- unexpected firewall drops.

Revert from the console. Diagnose only after cluster health is restored.

## Done when

- [ ] All nodes on tagged management, quorate, peers connected
- [ ] Cluster configuration hash still identical across every node
- [ ] Switch configuration re-exported — it changed, so the earlier export is
      now stale
- [ ] Firewall configuration re-exported
- [ ] Management reachable only from the approved administrative path
- [ ] Topology document updated to record management as tagged, with the date

## This completes the segmentation project

Check it against the full definition of done. The items most likely still open
at this point:

- [ ] One VLAN mismatch **and** one firewall mistake safely induced, diagnosed,
      and rolled back. The VLAN one should be done from
      [S2](/labs/segmentation-lab); the firewall one may not be.
- [ ] Configuration backups **restored in a controlled drill** — restored, not
      merely exported. An export you have never restored is a file.
- [ ] Stale references to the pre-migration addressing absent from active
      scripts, or explicitly marked historical
- [ ] Evidence records each test, result, and rollback confirmation
- [ ] The writeup distinguishes a homelab validation from production-scale
      network engineering

That last line is a claim-discipline requirement rather than a modesty gesture.
A small managed network and virtualization cluster can be real, defensible work.
Describing it as enterprise network engineering turns a strong claim into one
that collapses under a single follow-up question.
