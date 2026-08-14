---
title: "Zero-trust architecture"
date: 2026-08-14
summary: "Why being inside the network stopped meaning authorized, the policy engine / administrator / enforcement point model from NIST SP 800-207, and how SASE moves enforcement to the point of connection."
tags: [education, networking, zero-trust, security, sase, access-control]
---

## Overview

Perimeter security rests on an assumption: that traffic inside the network has
already been vetted, so a host on the LAN can be trusted more than one outside
it. The assumption was never quite true, and remote work, cloud services, and
personal devices finished it off. There is no longer a meaningful inside.

**Zero-trust architecture (ZTA)** discards the assumption instead of patching
it. Identity is verified continuously, and no location on the network grants
trust by itself. The practical goal is containment: a single compromised host
should not be a path to everything else.

This is the security counterpart to
[software-defined networking](/learn/software-defined-networking) — both move
decisions to a central authority and leave the edge to enforce them.

## The policy components

The architecture is often described in slogans — "never trust, always verify" —
which say nothing about how it is built. NIST SP 800-207 gives it structure, and
the useful part is that deciding and enforcing are separate jobs done by
separate components:

- The **policy engine** decides whether to grant access, using identity, device
  posture, and whatever other signals it is given.
- The **policy administrator** carries that decision out, establishing or
  cutting the connection. Together with the policy engine it forms the **policy
  decision point**.
- The **policy enforcement point (PEP)** sits in the traffic path and does what
  the policy administrator instructs.

Behind a PEP is an **implicit trust zone** — the region where traffic is no
longer individually inspected because the PEP already vetted it. Zero-trust
design works by shrinking those zones, so each one covers as little as possible.
A **secured zone** is the same idea applied to sensitive systems: fewer things
inside, more scrutiny at the boundary.

### Policy-based authentication

Access privileges depend on circumstances, not just on who is asking.
**Policy-based authentication** encodes that:

- time restrictions, such as ordinary working hours;
- location restrictions, such as an office network or a known region;
- device restrictions, such as a managed laptop rather than a personal machine;
  and
- required authentication strength, such as mandatory multi-factor.

Expected behavior proceeds normally. Unexpected behavior triggers stronger
authentication or is refused. Continuous evaluation of these signals, rather
than a single check at login, is often called **adaptive identity**.

### Authorization

Confirming identity is not the same as granting access. Authorization decides
what a verified identity may reach, and under zero trust it is scoped to what
the person's work requires. It can be as dynamic as authentication: the same
user may be permitted a system during business hours from a managed device and
refused it at 3 a.m. from an unknown network.

### Least privilege

**Least privilege** is the principle underneath both: grant the minimum access
required for a task, and nothing further. It has a real cost — someone who needs
a tool from another department has to ask, and that takes time. The trade is
containment. An attacker who takes an account inherits only that account's
narrow access, and each additional step has to be earned rather than assumed.

## SASE and SSE

When applications live in the cloud and users work from anywhere, backhauling
all traffic to a corporate data center for inspection stops making sense. The
inspection point is in the wrong place.

**Secure access service edge (SASE)** moves it. SASE combines SD-WAN with
security functions delivered from cloud points of presence near the user,
typically including:

- **firewall as a service (FWaaS)**;
- **secure web gateways (SWGs)**, which filter outbound web traffic; and
- **cloud access security brokers (CASBs)**, which sit between users and cloud
  services to apply policy and provide visibility into their use.

**Security service edge (SSE)** is the security half of SASE without the
networking half. Organizations that already have a WAN they are satisfied with
often adopt SSE alone.

This shifts enforcement toward the point of connection rather than eliminating
the data center. On-premises infrastructure does not disappear; it stops being
the mandatory waypoint for traffic that was never headed there.

## Suggested practice: find the decision and the enforcement

Zero trust is an architecture rather than a product, so the practice here is
reading systems you already have and naming the parts.

1. Look at any service you log into with multi-factor authentication. Identify
   what decided to challenge you and what actually blocked the request until you
   answered. On most consumer services these are the same system; naming them
   separately is the skill.
2. Read your own SSH configuration. `PermitRootLogin`, `PasswordAuthentication`,
   and `AllowUsers` in `/etc/ssh/sshd_config` are policy; `sshd` is the
   enforcement point. Change one and watch where the decision is actually made.
3. Map the implicit trust zone in your own network. Once a device is on the
   Wi-Fi, what can it reach without any further check? That set is the blast
   radius of one compromised device.
4. Apply least privilege to one account you control — remove a permission you
   have not used in six months and see whether anything breaks. Note how long it
   takes to notice.
5. Read a firewall rule set and classify each rule as identity-based or
   location-based. The ratio tells you how close to zero trust the design
   actually is.

## Related pages

- [Software-defined networking](/learn/software-defined-networking) — SD-WAN,
  which SASE combines with these security functions.
- [Cloud computing concepts](/learn/cloud-computing) — the cloud services a CASB
  sits in front of, and the VPC boundaries policy is written against.
- [Network appliances](/learn/network-appliances) — firewalls and proxies, the
  devices that become enforcement points.
- [Network functions](/learn/network-functions) — VPNs and IPsec, the remote
  access model zero trust is replacing.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [NIST SP 800-207: Zero Trust Architecture](https://csrc.nist.gov/pubs/sp/800/207/final)
  — the policy engine, policy administrator, and policy enforcement point model,
  and the definition of an implicit trust zone.
- [NIST SP 1800-35: Implementing a Zero Trust Architecture](https://www.nccoe.nist.gov/projects/implementing-zero-trust-architecture)
  — worked reference builds, useful for seeing the abstractions as products.
- [CISA Zero Trust Maturity Model](https://www.cisa.gov/zero-trust-maturity-model)
  — a staged view of what adoption actually looks like.

One correction from my notes: the decision component is the **policy engine**,
and it pairs with the policy administrator to form the policy decision point. I
had recorded it as a "policy brain," which is descriptive but is not the term in
the standard — and the split between deciding and enforcing is the part that
matters.

"Zero trust" is also a heavily marketed phrase, and a product claiming to
deliver it is usually selling one enforcement point. The architecture is the
whole arrangement, not any single box in it.
