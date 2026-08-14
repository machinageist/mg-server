---
title: "Moving My Homelab Management Network First"
date: 2026-07-31
summary: "Moving my homelab between flat subnets before starting VLAN work. The addresses changed; several systems still referred to the old ones. Notes on the outage, the recovery, and the revised plan."
category: "Networking"
tags: [networking, proxmox, homelab, corosync, dns, cloudflare-tunnel, incident, segmentation]
---

My plan was to move the homelab management layer onto a clean subnet, verify it,
and then start adding VLANs. I expected the flat-network move to be the simple
part.

It took about eight hours across two days. I broke the Proxmox cluster, lost
remote access to every management interface, and took this site offline. The
services are restored now, but segmentation has not started.

## The intended change

The homelab is a three-node Proxmox cluster on hardware I own, with a router VM,
a managed switch, and several guests. This website is one of those guests.
Management traffic, cluster traffic, guests, and clients all shared the flat
`10.0.1.0/24` network.

The eventual design has separate subnets for management, trusted clients,
servers, admin/bastion, lab, and guest systems. Management will use
`10.0.10.0/24`. For this first step I only wanted to move the existing flat
network to that range. There would be no VLAN tags, new firewall zones, or
inter-VLAN policy yet.

I still think that order makes sense. It is easier to troubleshoot an address
change before adding 802.1Q tagging, router-on-a-stick configuration, and more
firewall policy. The problem was not the order of the larger plan. It was my
preparation for this step.

This was not a scheduled maintenance window. I did not have a written change
record or rollback plan. I treated it as the next homelab task rather than a
change that could interrupt every management path at once.

## The addresses had more dependencies than I listed

An IP address is copied into more places than the interface that owns it. In this
case the old subnet appeared in:

- each node's host network configuration
- the Corosync transport configuration
- hypervisor firewall source rules
- `/etc/hosts` files that did not all agree
- guest resolver configuration
- backup and audit scripts with hardcoded addresses

I began with the host interfaces. The other references did not follow the new
addresses, and I found them during recovery.

After moving the router, switch, and hosts to the new subnet, the expected brief
loss of connectivity became a longer outage. Cluster quorum broke, and I lost
both SSH and the Proxmox web UI on all three nodes. The hosts and guests were
still running, but I had no remote management route. Physical console access
made recovery possible without rebuilding anything.

There were two main management failures.

First, Corosync still named each node by its old address. It does not discover
peers from the host interface configuration. It binds to configured addresses
and sends cluster traffic to the peers listed in its own configuration. Once the
host addresses changed, those entries no longer described the network. The nodes
stopped exchanging tokens and each behaved as an isolated member.

Second, the Proxmox firewall allowed SSH and the web UI from `10.0.1.0/24`. My
workstation had moved to the new subnet, so new connections no longer matched
the allow rules.

The firewall configuration also lives under `/etc/pve`, the Proxmox cluster
filesystem. I had treated firewall policy and cluster state as separate parts of
the change, but the storage and distribution of that policy depend on the
cluster.

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
more changes. I am still learning some of the Proxmox and Corosync internals, so
I have tried not to claim more than I verified here.

Before the next repair attempt, I collected read-only state from all three nodes:
network and cluster configuration, service status, cluster database files,
recent logs, and checksums of the collected files. That gave me a record of the
broken state before I changed it again.

The recovery pattern I want to keep:

1. Capture state before changing it.
2. Verify with evidence independent of the component under test.
3. Make one change at a time and check the result.

Packet capture was useful for the second step. Instead of relying only on a
cluster status command, I watched for cluster traffic between the nodes. The
status output showed what the service believed; the capture showed whether
packets were actually crossing the network.

## Corosync, quorum, and `/etc/pve`

Changing the Corosync addresses required restarting the daemon because it binds
its sockets at startup.

Under normal conditions, Proxmox keeps the authoritative Corosync configuration
in the cluster filesystem. A change there gets distributed to each node. That
path was unavailable without a working cluster, so recovery required updating
the local configuration on each node and restarting the service on each one.

After that, two nodes exchanged traffic and formed quorum while the third was
still unresolved. In a three-node cluster, two votes are a majority. The pair
can remain authoritative while the isolated node cannot write divergent cluster
state. This is the quorum mechanism working as designed, not a complete cluster
recovery.

The isolated node's `/etc/pve` was read-only and not syncing because it did not
have majority membership. That produced three distinct checks:

| Layer | What it is | Question |
|---|---|---|
| Membership | Nodes exchanging tokens | Can the nodes see each other? |
| Quorum | Votes counted over that membership | Is there an authoritative majority? |
| Cluster filesystem | Configuration replicated among members | Is shared state present and in sync? |

Quorum is required before the cluster filesystem accepts writes, but quorum does
not prove that every node has joined and synchronized its database. I saw the
service running and a quorate pair while shared configuration was still missing
on the remaining node.

There is a circular dependency to account for during recovery. The authoritative
transport configuration lives in the cluster filesystem, then gets copied to the
local file Corosync reads at startup. If the filesystem cannot mount and sync,
that copy does not occur. A quorate status was therefore one recovery check, not
the final one.

The guests kept running during this. A running VM is a process on its host with
its own memory and disks. `/etc/pve` contains the configuration needed to start,
stop, migrate, or edit it, but an already-running guest does not consult cluster
membership to continue executing.

My cluster does not use HA fencing. If it did, an isolated node could trip its
watchdog and reboot, taking its guests down to protect cluster consistency. The
guests remained up because of my configuration, not because management-plane
failures are always harmless to guests.

## The website outage was a DNS problem

The cluster began recovering, but this site was still unavailable. The web VM
was running on the new subnet. It could reach the internet by IP, and the
application worked locally. The Cloudflare Tunnel could not connect.

The guest's `/etc/resolv.conf` still used the old gateway address as its DNS
resolver. That address no longer existed, so queries timed out.

Public DNS for the domain continued to work. Cloudflare hosts the zone, and its
anycast addresses remained in the public records. My home address is not
published there. The failed lookup happened in the other direction: the tunnel
client needed to resolve Cloudflare's edge hostname before opening its outbound
connection. With no working resolver, it had no edge address to contact.

| Layer | State |
|---|---|
| Link and addressing | New subnet and gateway configured |
| Raw IP connectivity | Internet addresses reachable |
| Name resolution | Broken because the resolver address was stale |
| Local application | Working locally |
| Public reachability | Broken because the tunnel could not resolve its edge |

Correcting the resolver restored DNS. The tunnel reconnected and registered, the
local origin check passed, and the site became reachable again.

This also clarified that the site did not depend on cluster membership for its
service path. It needed the host to stay up, a network path out, DNS, and the
Cloudflare Tunnel. The cluster is how I administer the VM, but it is not in the
request path.

The simultaneous symptoms came from separate stale references:

| Stale reference | What broke | Effect |
|---|---|---|
| Corosync peer addresses | Membership and quorum | No working cluster management plane |
| Firewall source subnet | SSH and Proxmox web access | No remote host access |
| Guest resolver address | DNS and then the tunnel | Public site unavailable |

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

## Current state and next steps

The current state is:

- The public service is restored. The tunnel is registered and the site is
  reachable.
- All three Proxmox nodes are present and quorate, with peers connected.
- The firewall allows the new management subnet and is running.
- The network is still flat. Segmentation has not started.

This is the same basic network architecture on a different address range. Before
adding VLANs, I want to observe normal operation, test a reboot, and confirm that
the flat baseline stays stable.

The original runbook moved the router, switch, hosts, and guests into the final
segmented design in one cutover. I have replaced it with a staged plan:

1. Verify the recovered flat network.
2. Find and reconcile stale addressing, DNS, firewall, host-file, and automation
   references in one inventory.
3. Add one trust zone at a time, beginning with the lab zone because it has the
   smallest blast radius.
4. Deliberately test a bad VLAN assignment and firewall rule, then practice the
   rollback while the scope is small.

Before the first VLAN change, I also want fresh configuration backups that I
have proved I can restore, confirmed console access to each device, and a tested
rollback path for every layer involved.

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
