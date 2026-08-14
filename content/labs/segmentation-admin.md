---
title: "S4 — ADMIN"
date: 2026-08-14
summary: "Moving the bastion and the remote-access endpoint onto the admin zone, one at a time, and testing the policy from genuinely outside rather than from the sofa."
tags: [labs, networking, segmentation, bastion, vpn]
---

## Scope

Move the bastion and the remote-access endpoint onto the admin zone — **one at
a time**. The service-level setup for each has its own page:
[bastion host](/labs/bastion-host) and [remote access](/labs/remote-access).

## The mistake this stage exists to prevent

> Do not grant all admin clients unrestricted east-west access merely because
> the zone is authenticated.

Authentication is not authorization. A VPN client that has authenticated has
proven *who* it is, not that it should reach every subnet. If the admin zone
becomes a flat "trusted once you're in" network, you have rebuilt the flat
network you are in the middle of segmenting — just with a login on the front of
it.

Per the policy matrix, admin gets: management administration, SSH to
**approved** server and lab hosts, approved internal services, and internet as
required. Everything else denied.

## Validate from both directions

This has to be tested from two vantage points, and the second is the one people
fake.

**1. From the local trusted zone** — the normal admin path:

```bash
ssh <the bastion>                       # reachable from TRUSTED
ssh <a management host>                 # bastion → MGMT: allowed
ssh <an approved lab host>              # bastion → LAB: allowed
curl -sS --max-time 5 https://<a servers host>   # only if approved
```

**2. From a genuinely external client** — not from inside the house.

Connecting to your own VPN from your own LAN proves the tunnel process is
running. It does not prove the external path, NAT traversal, DNS inside the
tunnel, or the firewall rules that apply to a remote peer. Test from cellular or
another network.

```bash
# From outside, over the VPN
ip route                        # confirm tunnel routes; watch for a full-tunnel surprise
ssh <the bastion>
ssh <a management host>         # should follow the SAME policy as local admin
ping -c2 <the guest gateway>    # GUEST — must be denied
```

## What makes a bastion a bastion

If the bastion is reachable from the trusted zone and can reach everything, it
is not a bastion — it is a jump box with no policy. A bastion is worth having
because it is the *only* path to a zone, it is logged, and it has a narrow
allowlist.

Decide these explicitly rather than by default:

- [ ] Which hosts may admin clients reach **through** the bastion?
- [ ] Is direct trusted → management allowed, or must it go via the bastion? If
      the policy matrix allows it directly, be honest that the bastion is for
      remote access rather than internal segmentation.
- [ ] Is session logging on, and where does it go?

## Order

1. **The bastion first** — it is the recovery path for the second move.
2. Verify the full matrix from both vantage points.
3. **The VPN endpoint second.**
4. Re-verify remote access *before* you need it.

Do not move the VPN endpoint first. If that move goes wrong and it was your only
remote path, you have removed your own ability to fix it from outside.

## Stop conditions

- The external VPN test fails after the move → revert it. You are now blind from
  outside, and finding that out during an actual outage is the scenario this
  ordering exists to avoid.
- Admin can reach a zone the matrix denies → policy error. Revert before
  investigating.
- The bastion is unreachable from trusted → revert. The second move has no
  safety net without it.

## Done when

- [ ] Bastion on the admin zone, verified from trusted
- [ ] Remote-access endpoint on the admin zone, verified **from a genuinely
      external client**
- [ ] Full admin policy matrix tested — allows and denies — from both vantage
      points
- [ ] The bastion's allowlist explicitly defined and documented
- [ ] Rollback rehearsed for each guest before its move
