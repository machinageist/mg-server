---
title: "Moving My Homelab Management Network First"
date: 2026-07-31
summary: "A management-network change exposed stale dependencies across cluster, access-control, and DNS layers. What failed, how I narrowed it down, and how the change process improved."
category: "Networking"
tags: [networking, homelab, clustering, dns, incident, segmentation]
---

My plan was to move the homelab management layer onto a clean subnet, verify it,
and then start adding VLANs. I expected the flat-network move to be the simple
part.

The change caused a long management-plane outage and interrupted this site. This
post traces the failure across cluster, access-control, and DNS layers and records
the verification method I used to recover.

## The intended change

The affected environment is a small virtualization lab on hardware I own. At the
time of the incident, management, cluster, guest, and client traffic shared a
single change domain.

The long-term design separates systems by role and trust. This first change was
only an address migration; it deliberately did not combine new tagging and
firewall policy with the move.

I still think that order makes sense. It is easier to troubleshoot an address
change before adding 802.1Q tagging, router-on-a-stick configuration, and more
firewall policy. The problem was not the order of the larger plan. It was my
preparation for this step.

This was not a scheduled maintenance window. I did not have a written change
record or rollback plan. I treated it as the next homelab task rather than a
change that could interrupt every management path at once.

## The addresses had more dependencies than I listed

An IP address is copied into more places than the interface that owns it. In this case, stale references existed across several categories:

- host networking;
- cluster transport configuration;
- management access rules;
- local name resolution and resolver settings; and
- automation with embedded addresses.

I began with the host interfaces. The other references did not follow the new
addresses, and I found them during recovery.

The expected brief loss of connectivity became a longer outage. Cluster
membership broke and the existing remote management paths no longer matched the
new source network. An independent console path made recovery possible without
rebuilding the hosts.

There were two main management failures.

First, the cluster transport still named each member by its old address. It did
not discover peers from the host interface configuration; it used its own static
peer configuration. Once the host addresses changed, those entries no longer
described the network and each member behaved as an isolated system.

Second, management access rules still matched the old source network. New
connections therefore failed even where the hosts were otherwise reachable.
The incident also exposed that policy distribution and cluster health were not
independent in this design.

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
service status, recent logs, and checksums. That preserved evidence of the broken
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

The recovery reinforced that cluster health has several separate layers:

| Layer | What it is | Question |
|---|---|---|
| Membership | Nodes exchanging tokens | Can the nodes see each other? |
| Quorum | Votes counted over that membership | Is there an authoritative majority? |
| Cluster filesystem | Configuration replicated among members | Is shared state present and in sync? |

Quorum is a prerequisite for authoritative writes, but it does not prove that
every member has rejoined or that shared state has synchronized. I now verify
membership, authority, and replicated state separately before considering the
cluster recovered.

## The website outage was a DNS problem

The management plane began recovering, but the public service was still
unavailable. The application worked locally and could reach internet addresses,
while its outbound connector could not establish a session.

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
the local origin check passed, and the site became reachable again. The useful
lesson is dependency ordering: raw IP reachability does not prove DNS, and a
healthy local application does not prove its public edge path.

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
3. Add one trust zone at a time, beginning with the lowest-blast-radius case.
4. Deliberately test a bad VLAN assignment and firewall rule, then practice the
   rollback while the scope is small.

Before any future network change, I require tested configuration-restore evidence,
an independent access path, and a rollback for every layer involved.

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
