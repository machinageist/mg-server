---
title: "Moving the Management Layer First: A Flat-Network Migration That Taught Me to Slow Down"
date: 2026-07-31
summary: "I set out to move my homelab from one flat subnet to another so the management layer would be solid before any VLAN work. The addresses moved; the cluster state did not follow. This is what I was trying to do, where it went sideways, and how working the problem from first principles got the site back."
category: "Networking"
tags: [networking, proxmox, homelab, corosync, dns, cloudflare-tunnel, incident, segmentation]
---

I had a plan for my homelab network, and the plan had an order to it: get the
management layer onto a clean subnet first, prove it stable, and only then start
carving the network into VLANs. Segmentation is the interesting work. Moving a
flat network from one address range to another is the boring prerequisite.

The boring prerequisite took about eight hours across two days, broke my Proxmox
cluster, locked me out of every management interface I had, and took this site
offline. It also taught me more about operating infrastructure than the
segmentation work would have on its own.

This is a checkpoint post, not a victory lap. Service is restored and the cluster
is healthy again. Segmentation has not started, deliberately.

## The change I set out to make

The homelab is a three-node Proxmox cluster on owned hardware, plus a router VM,
a managed switch, and a handful of guests — this website among them. Everything
lived on one flat `10.0.1.0/24` network: management traffic, cluster traffic,
guests, and clients all in the same broadcast domain with no policy between them.

That is fine for a lab that is just running. It is not fine as a foundation for
segmentation, and it is not how I want to practice building a network.

The target design uses one subnet per trust zone — management, trusted clients,
servers, admin/bastion, lab, and guest. Management would become `10.0.10.0/24`.
The step I picked first was the least clever one available: move the existing
flat network onto the new management range, unchanged in every other respect. No
VLAN tags, no new firewall zones, no policy matrix. Same flat network, new
address space.

The reasoning still holds up. If I cannot land a straight address migration
cleanly, I have no business layering 802.1Q tagging, inter-VLAN policy, and a
router-on-a-stick on top of it. Establish the management layer, then build on it.

I want to be honest about one thing up front: this was not a scheduled
maintenance window. My homelab is a just-in-time learning environment, and this
was simply the next thing to learn. There was no change record, no rollback plan
written down, and no maintenance window. That decision shows up later in this
post, repeatedly.

## Why "just move the subnet" is never just moving the subnet

Changing an IP address sounds like a single edit. It is not. An address is
referenced in more places than the interface that owns it, and every one of those
references is a separate copy of the truth that does not update itself.

In my case, the old subnet was written into at least all of the following:

- the host network configuration on each node
- the cluster transport configuration, which names each node by address
- the hypervisor firewall rules, which allowed management traffic *from* the old
  subnet
- `/etc/hosts` files across nodes, several of which disagreed with each other
- the resolver configuration inside guests
- backup and audit scripts with hardcoded addresses

I knew about the first item. I discovered the rest the hard way, in roughly that
order. The mental model I started with — "change the addresses, then everything
follows" — had the dependency arrow backwards. Everything else was pointing *at*
the addresses, and none of it moved on its own.

## Where it went wrong: the network moved, the cluster state did not

Dropping the host addresses took everything down, exactly as expected. That part
was planned; you cannot re-address a network without losing it for a moment.

The problem showed up after the switch, the router, and every device were on the
new subnet and things should have come back. Instead:

- cluster quorum broke
- I lost the web UI **and** SSH to all three nodes
- this site went offline

Losing every management path at once was the real problem. The nodes were up —
the hardware was fine, the guests were running — but I had no remote way in.
Physical console access is the only reason this stayed a bad evening instead of a
rebuild.

Two separate failures were tangled together here, and it took me a while to
separate them:

**The cluster transport was still pointing at the old subnet.** Corosync — the
layer that lets nodes agree on who is in the cluster — does not discover its
peers. Its configuration contains a static list naming each node by address, and
it binds a socket to that literal address and sends to those literal addresses.
Those entries still named the old subnet. So corosync kept trying to reach
addresses that no longer existed on the network, even though every node was
perfectly reachable at its new one. The token that circulates between members
never completed a lap, and each node concluded it was alone.

**The hypervisor firewall was still allowing the old subnet.** The management
allow-rules permitted SSH and the web UI *from* `10.0.1.0/24` and dropped those
ports from other sources. My workstation was now on the new subnet — which the
rules did not know about. I had firewalled myself out of my own cluster from the
management side while simultaneously breaking the cluster from the transport
side.

Both of those are the same root problem wearing two hats: **the data plane moved
and the control/policy state did not move coherently with it.**

There is a detail in the firewall half that took me a while to appreciate. The
Proxmox firewall reads its rules from files that live inside `/etc/pve` — the
cluster filesystem that was itself broken. My firewall configuration was stored
in the thing I had just knocked over. Layers I had been treating as independent
turned out to be stacked on each other.

## The turn: stepping away and working from the OSI model

Here is the part I would rather not publish, and the part most worth publishing.

For the first stretch, I did not troubleshoot. I flailed. I ran commands I found
in search results without understanding what they did, on a system I had already
destabilized, while the clock ran. I was not panicking, but I was very aware that
one lost day could easily become three.

What broke the loop was leaving. I stopped, ate something, and thought about how
I actually wanted to solve the problem instead of continuing to poke it.

What I came back with was a framework rather than a fix: **work the OSI model
from the bottom up.** Is the link up? Do the addresses and routes make sense? Can
these hosts reach each other at all? Does name resolution work? Does the
application layer work? Stop guessing which component is broken and instead
establish, layer by layer, what is verifiably true.

That reframing is what turned an unbounded mess into a sequence of answerable
questions. Every question I could answer shrank the space the failure could be
hiding in.

About four hours in, I brought in an AI agent for hands-on help. I want to be
precise about what that did and did not mean, because it matters for what this
post claims. It worked the problem in the order I had already decided on — the
sequence was mine. What it gave me was speed on specifics I would otherwise have
spent an hour searching for, and, more valuably, the discipline to capture
evidence *before* changing anything. I wanted my site back online; it got me
there faster. The parts of the mechanism I am still learning, I flag as such
below rather than dressing them up.

## Recover by evidence, not by guessing

The first thing that changed after the reset was the order of operations. Before
attempting any further repair, I captured read-only state from all three nodes:
network configuration, cluster configuration, service status, cluster database
files, recent logs, and checksums of everything collected.

Nothing was modified during that pass. The point was to have a known record of
the broken state before I started changing it.

This mattered more than it sounds like it should. When you are several hours into
a self-inflicted outage, the temptation is to *do something*, because doing
something feels like progress. But the state you are standing in is also the
evidence you need to diagnose the problem, and every unconsidered command
overwrites some of it. I had already burned some of that evidence during the
flailing stage — my early actions were undocumented, and my recollection of them
is genuinely poor, which is its own lesson about working while rattled.

The pattern I would keep exactly as-is:

1. Capture state read-only, with checksums, before mutating anything.
2. Verify with independent evidence, not with the tool that is already suspect.
3. Make one change at a time and re-verify.

Packet capture deserves a specific mention. Rather than trusting a status command
to tell me whether two nodes were talking, I watched for the actual cluster
traffic between them on the wire. A status output is an assertion; packets are
evidence. That distinction — *is this tool telling me the truth, or is it telling
me what it believes?* — was the single most useful habit to come out of this.

## Quorum is not the same as a healthy cluster filesystem

Correcting the transport addresses was not a live edit. Corosync binds its
sockets at startup, and a bound socket cannot be re-pointed at a new address —
the daemon has to restart to bind new ones.

There is a more awkward reason too. Normally you never hand-edit a node's local
corosync configuration on Proxmox. You edit the cluster's authoritative copy,
increment its version, and the cluster distributes it to every node. But that
distribution runs *over the cluster itself*. With no quorum, there is nothing to
distribute it with. The automatic mechanism depends on the thing that is broken,
so recovery drops back to the manual path: edit the local file on each node,
restart the service on each node.

Once that was done, two nodes began exchanging traffic and formed a working
quorum — while the third was still unresolved. That is worth unpacking, because
"two out of three" is not a partial result. It is the designed outcome.

Quorum is majority arithmetic. Three nodes, three votes, majority is two. The
pair holds two of three, so they are authoritative and allowed to write cluster
configuration. The isolated node holds one vote and loses. Only one partition can
ever hold a majority, which is precisely the point — it makes split-brain
arithmetically impossible rather than merely unlikely.

The isolated node's `/etc/pve` was unhealthy as a *consequence* of that, not as a
separate fault. A node without majority puts its cluster filesystem into
read-only and stops syncing, deliberately, so it cannot make configuration
changes that would diverge from the authoritative side. Isolated, outvoted,
read-only — one causal chain.

Which brings me to the distinction I actually needed to learn. These are three
different layers:

| Layer | What it is | What it answers |
|---|---|---|
| Membership | Nodes exchanging tokens over the network | "Can we see each other?" |
| Quorum | Vote counting on top of membership | "Do enough of us agree to be authoritative?" |
| Cluster filesystem | Configuration replicated across members | "Is our shared state actually in sync?" |

The cluster filesystem is a *consumer* of the layers beneath it. It needs quorum
before it will accept writes — but having quorum does not prove it successfully
joined the group or finished syncing its database. That is exactly the state I
found: the service running, quorum satisfied, and the shared configuration still
missing on a node. The layer below was healthy and the layer above had not caught
up.

The two are entangled in one more way that closes the loop. The authoritative
copy of the cluster's own transport configuration lives *in* the cluster
filesystem, and gets copied down to the local file the transport actually reads
at startup. If the filesystem cannot mount and sync, that copy never happens. The
config that fixes the cluster lives inside the cluster.

The practical takeaway: **do not stop diagnosing when the first green indicator
appears.** "Quorate" was a real signal and it was not completion. Check the layer
above the one that just went green.

Guests, incidentally, kept running throughout. A running virtual machine is an
ordinary process on its host, holding its own memory and disks; nothing about it
consults the cluster to keep running. The cluster filesystem holds VM
*configuration* — what you need to start, stop, migrate, or edit a VM, not what
you need to keep an already-running one alive. So the entire management plane can
be broken while every guest hums along untouched.

The caveat worth knowing is that this is not free. If high-availability fencing
were configured, an isolated node could trip its watchdog and reboot itself,
taking its guests down to protect the cluster. Mine does not run HA fencing,
which is why this stayed a management outage instead of a guest outage. That was
a property of my configuration, not a guarantee of the platform.

## The second failure hiding in plain sight: stale DNS

With the cluster coming back, this site was still down. And this failure looked
nothing like the first one.

The web server VM was up. It was on the new subnet. It could reach the internet
by raw IP. The web application was running fine locally. But the Cloudflare
Tunnel that makes it publicly reachable could not establish a connection.

The cause was one stale file: `/etc/resolv.conf` inside the guest still pointed
at the **old** subnet's gateway as its DNS resolver. That address no longer
existed on the network, so every DNS query timed out.

It is worth being precise about what that broke, because the obvious reading is
wrong. Public DNS for this domain never failed. The zone is hosted at Cloudflare,
the public records point at Cloudflare's anycast addresses, and they answered
correctly throughout — my home address is never in DNS at all. What failed was
much more ordinary: **the tunnel client is itself a client, and it has to resolve
Cloudflare's edge hostname to know where to dial.** Same as `curl` resolving a
URL before fetching it. With a dead resolver that lookup timed out, so it never
got an address to connect to, so the tunnel never came up.

The direction is the thing to hold onto. Nothing out there resolves my name to my
server. My server resolves *Cloudflare's* name in order to dial *out*, and public
traffic is routed back down the connection it opened.

This is the clearest illustration of the whole incident's theme:

| Layer | State |
|---|---|
| Link / addressing | Correct — new subnet, correct gateway |
| Raw IP connectivity | Working — could reach the internet by address |
| Name resolution | **Broken** — resolver pointed at a decommissioned address |
| Local application | Working — served correctly on the loopback |
| Public reachability | **Broken** — tunnel could not resolve the edge |

Two working layers on either side of one broken one, and the symptom presented as
"the website is down." If I had kept guessing at the application layer, I would
never have found it. Going bottom-up did.

Fixing the resolver restored outbound DNS, the tunnel client reconnected and
registered, and the local origin check passed. The site came back.

Here is the part I got wrong in my own head for most of the incident, and it is
the most useful thing in this post. **This web server never needed the cluster at
all.** To serve this site it needs four things: its host powered on, a network
path out, working name resolution, and a connected tunnel. Cluster membership is
not on that list. The cluster is how I *administer* the VM; it is not part of the
VM's service path. The cluster could have stayed broken all week and this site
would have been fine.

So I did not have one enormous problem. I had **two independent failures sharing
a single root cause.** The shared cause was stale references to a subnet that no
longer existed. But they broke different things at different layers, with
different victims:

| Failure | What broke | Who it affected |
|---|---|---|
| Stale cluster transport addresses | Cluster membership and quorum | Me — no management plane |
| Stale firewall source rules | SSH and web UI to the hosts | Me — no remote access |
| Stale resolver in the guest | Name resolution, then the tunnel | Visitors — site offline |

They happened simultaneously, which is exactly why it felt like one catastrophic
event instead of three ordinary ones. Untangling them was most of the work, and
nothing untangled them faster than asking "what does this specific thing actually
depend on?" — which is also the strongest argument I know for changing one domain
at a time.

## Preflight is not apply

One more failure is worth its own section, because it corrected a wrong
assumption I did not know I had.

The firewall migration — updating the management allow-rules from the old subnet
to the new one — had a preflight check that inspected the current rules and
confirmed they were exactly what was expected before changing anything. The
preflight passed cleanly.

Then the apply step failed, because the command it intended to run did not exist.
I had assumed an `update` subcommand that the firewall tool simply does not have.

Nothing was harmed; the change simply did not happen. But the lesson stuck: **a
passing preflight tells you the environment is what you expected. It tells you
nothing about whether your change will execute.** Those are two independent
failure modes, and I had been mentally treating a green preflight as
significantly more assurance than it actually provided. Verifying the *end state*
after an apply — not just the return code, and not just the preconditions — is
now part of how I work.

Two things I did not know then and use now. The firewall tool has a `compile`
step that renders the ruleset it *would* apply without applying it, so you can
read what you are about to enforce before you enforce it. And connection tracking
will keep an already-established session alive after a rule change that would
block a new one — so an open SSH session can keep working while every fresh
connection is refused. That makes a firewall lockout look intermittent and
environmental, and it is very good at convincing you the firewall is not the
problem.

## Restored is not segmented

Where things actually stand, stated plainly:

- **Public service: restored and verified.** The tunnel is registered and this
  site is reachable.
- **Cluster: recovered.** All three nodes show present and quorate, and each node
  reports its peers connected.
- **Firewall: migrated and running**, allowing the correct management subnet.
- **Segmentation: not started.**

That last line is the deliberate one. The network I recovered is *flat* — one
subnet, no VLANs, no tagging, no inter-zone policy. It is the same architecture I
started with, at a different address. The management layer is established, which
was the goal, but nothing is segmented yet.

It would be easy to call this project finished and move on to the interesting
work. I am not doing that, for one reason: I have just demonstrated to myself
what happens when I build a change on top of a foundation I have not proven. The
flat network needs to prove itself boring — through normal operation, a reboot
test, and a stretch of uneventful uptime — before I start tagging VLANs on top of
it.

So the plan changed. The original runbook was a greenfield all-at-once cutover:
router, switch, hosts, and guests moved together into a fully segmented design.
That plan is retired. The replacement is recovery-first and staged:

1. Close out recovery and prove a stable flat baseline.
2. Reconcile every stale reference — addressing, DNS, firewall, and automation —
   into one authoritative inventory.
3. Only then introduce VLANs, one trust zone at a time, starting with the lab
   zone because it has the smallest blast radius.
4. Deliberately break one VLAN assignment and one firewall rule, observe the
   failure, and practice the rollback — while it is cheap.

Between the baseline and the first VLAN sits an explicit exit gate: a checklist
that must be satisfied with fresh evidence before any segmentation change
happens. Config backups exported and proven restorable. Console access confirmed
on every device. A tested rollback path for each layer I intend to touch. The
gate exists because I now know exactly what it costs to skip it.

## What I would carry into the next window

**Change one domain at a time.** I combined an address migration, a cluster
transport change, and a firewall policy change in one unplanned window. Any one
of those alone would have been diagnosable in minutes. Together, each one masked
the others.

**An address is a dependency, not a value.** Before changing one, enumerate
everything that references it. Configuration files, firewall rules, host files,
resolver settings, automation scripts. Grep for the old subnet across everything
you own — that list is your actual change scope.

**Ask what each thing actually depends on.** My site does not depend on my
cluster. Once I could say that plainly, one overwhelming outage became separate,
ordinary problems. Dependency confusion is what makes simultaneous failures feel
like a single unsolvable one.

**Preserve your recovery path before you need it.** Physical console access is
the only reason this was recoverable. Never let the thing you are changing be the
only way back in.

**Capture state before you change it.** The broken state is evidence. Read-only
first, checksums, then mutate.

**A framework beats a fix when you are lost.** The OSI model did not tell me the
answer. It told me what question to ask next, which was worth more.

**Know when to walk away.** The most productive thing I did in eight hours was
stop for a meal. Diagnosis quality degrades sharply under time pressure, and the
degradation is invisible from the inside.

**Green is not done.** Quorum restored was not the cluster healthy. A passing
preflight was not a successful apply. Site reachable is not segmentation
complete. Check the layer above the one that just went green.

The thing I actually learned is the difference between configuring infrastructure
and operating it. Configuring is knowing which values to set. Operating is
knowing what breaks when you change them, what evidence proves the change worked,
and how you get back if it did not. I have done plenty of the first. This was my
first real dose of the second, and I would rather have learned it on my own
hardware than on someone else's.

Segmentation resumes when the boring baseline has earned it.
