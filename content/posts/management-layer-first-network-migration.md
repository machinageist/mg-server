---
title: "Moving My Homelab Management Network First"
date: 2026-07-31
summary: "I moved my homelab management network before starting VLAN work. The addresses changed, but several systems still pointed at the old network. What failed and how I recovered."
category: "Networking"
tags: [networking, homelab, clustering, dns, incident, segmentation]
---

My plan was to move the homelab management layer onto a clean subnet, verify it,
and then start adding VLANs. I expected the flat-network move to be the simple
part.

It took about eight hours across two days. I lost remote access to every
management interface, and this site went offline. This post goes through what
failed in the cluster, the access rules, and DNS, and how I checked each one
during recovery.

## The intended change

The lab is a small virtualization cluster on hardware I own. At the time,
management, cluster, guest, and client traffic all shared one network.

The long-term plan separates systems by role and trust. This first step was only
an address change. New VLAN tagging and firewall policy would come later.

I still think that order makes sense. It is easier to troubleshoot an address
change before adding 802.1Q tagging, router-on-a-stick configuration, and more
firewall policy. The problem was not the order of the larger plan. It was my
preparation for this step.

This was not a scheduled maintenance window. I did not have a written change
record or rollback plan. I treated it as the next homelab task rather than a
change that could interrupt every management path at once.

## The addresses had more dependencies than I listed

An IP address is copied into more places than the interface that owns it. In
this case the old addresses were still in:

- host networking
- the cluster transport configuration
- management access rules
- local name resolution and resolver settings
- automation scripts with addresses written into them

I began with the host interfaces. The other references did not follow the new
addresses, and I found them during recovery.

I expected a brief loss of connectivity. It became a long outage. Cluster
membership broke, and my remote management access no longer matched the new
network. The hosts and guests were still running, and physical console access
let me recover without rebuilding anything.

There were two main management failures.

First, the cluster transport still named each node by its old address. It does
not discover peers from the host's interface settings. It uses its own static
list of peers. Once the host addresses changed, that list no longer matched the
network, and each node acted as if it were alone.

Second, the management access rules still only allowed the old network. My
workstation had moved to the new one, so new connections failed even though the
hosts were reachable.

I had also treated the access rules and the cluster as separate parts of the
change. They were not. The rules are distributed by the cluster, so they
depended on it being healthy.

## Starting again from the bottom

My first troubleshooting attempts were not systematic. I ran commands from
search results without understanding all of them and did not keep good notes. A
few hours into the outage I stopped, ate, and came back with a simpler approach:
check each network layer from the bottom up.

I checked link state first, then addresses and routes, direct reachability, name
resolution, and finally the applications. This did not provide an immediate fix,
but it replaced a broad outage with smaller questions I could answer.

About four hours in, I also used an AI agent for hands-on help. The bottom-up
sequence was already the approach I wanted to take. The agent helped with
specific commands and, more importantly, with collecting evidence before making
more changes. I was still learning parts of the cluster stack, so I have tried
not to claim more than I verified here.

Before the next repair attempt, I collected read-only network and cluster state,
service status, recent logs, and checksums. That gave me a record of the broken
state before I changed it again.

The recovery pattern I want to keep:

1. Capture state before changing it.
2. Verify with evidence independent of the component under test.
3. Make one change at a time and check the result.

Packet capture was useful for the second step. Instead of relying only on a
cluster status command, I watched for cluster traffic between the nodes. The
status output showed what the service believed; the capture showed whether
packets were actually crossing the network.

## Membership, quorum, and shared state

Recovery made it clear that cluster health has several separate layers:

| Layer | What it is | Question |
|---|---|---|
| Membership | Nodes exchanging tokens | Can the nodes see each other? |
| Quorum | Votes counted over that membership | Is there an authoritative majority? |
| Cluster filesystem | Configuration replicated among members | Is shared state present and in sync? |

The cluster needs quorum before it accepts writes, but quorum does not prove that
every node has rejoined or that the shared configuration is in sync. Now I check
membership, quorum, and the shared filesystem separately before I call the
cluster recovered.

## The website outage was a DNS problem

The cluster was coming back, but the site was still down. The application worked
locally and could reach internet addresses, but its outbound connector could not
start a session.

The guest still referred to a resolver on the old network, so queries timed out.

Public DNS for the domain continued to work. The failed lookup happened in the
other direction: the connector needed working outbound name resolution before
it could contact the edge.

| Layer | State |
|---|---|
| Link and addressing | New subnet and gateway configured |
| Raw IP connectivity | Internet addresses reachable |
| Name resolution | Broken because the resolver address was stale |
| Local application | Working locally |
| Public reachability | Broken because the tunnel could not resolve its edge |

Correcting the resolver restored DNS. The connector re-established its session,
the local origin check passed, and the site came back. The lesson was about
order. Reaching an IP address does not prove DNS works, and an application that
works locally has not proven its public path.

The simultaneous symptoms came from separate stale references:

| Stale reference | What broke | Effect |
|---|---|---|
| Cluster peer references | Membership and quorum | No working cluster management plane |
| Management source policy | Administrative access | No remote host access |
| Guest resolver reference | DNS and then the connector | Public service unavailable |

They shared the same change but failed independently. Asking what each service
actually depended on made them easier to separate.

## A passing preflight did not test the apply step

I had a preflight check for the firewall migration. It inspected the current
rules and confirmed that they matched the expected starting state. That check
passed. The apply then failed because I had assumed the firewall tool had an
`update` subcommand that did not exist.

The failed command did not change anything, but it exposed a gap in the test. A
preflight can validate starting conditions without validating the command that
will perform the change. I now check the end state as well as preconditions and
return codes.

I also learned two relevant firewall details. The tool has a `compile` step that
renders the ruleset without applying it, which makes the intended result
reviewable. Connection tracking can also preserve an established SSH session
after a rule change blocks new sessions. Testing only through an existing
connection can therefore hide a lockout.

## Revised change plan

The original runbook moved the router, switch, hosts, and guests into the final
segmented design in one cutover. I have replaced it with a staged plan:

1. Verify the recovered flat network.
2. Find and reconcile stale addressing, DNS, firewall, host-file, and automation
   references in one inventory.
3. Add one trust zone at a time, starting with the one that can break the least.
4. Test a bad VLAN assignment and a bad firewall rule on purpose, then practice
   the rollback while the scope is small.

Before any future network change, I want a tested configuration restore, a
second way in, and a rollback for every layer involved.

## Notes for the next migration

A few practical points came out of this attempt:

- Change one domain at a time where possible. The host addresses, cluster
  transport, and firewall policy changed in the same unscheduled window.
- Search for the old subnet before moving it. Interfaces, peer lists, firewall
  rules, host files, resolver settings, and scripts all need to be in scope.
- Keep a recovery path outside the network being changed. Physical console
  access was necessary here.
- Capture read-only state before repair, then make and verify one change at a
  time.
- Check the layer above the first healthy indicator. Quorum did not mean every
  node's cluster filesystem had synchronized, and a passing preflight did not
  mean the apply would work.
- Stop when troubleshooting becomes random. Taking a break improved the quality
  of the next attempt more than another hurried command would have.

I knew how to assign the new addresses. I had not accounted for everything that
referred to them or how I would prove each dependent system had recovered. That
is the part I need to fix before continuing with segmentation.
