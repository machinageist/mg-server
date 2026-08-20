---
title: "Remote access — pick one primary"
date: 2026-08-14
summary: "Self-hosted WireGuard or a coordinated overlay, the claim each one commits you to, and why running both quietly undoes the segmentation you just built."
tags: [labs, networking, vpn, wireguard, remote-access]
---

## Read this before installing anything

This is a sanitized decision framework, not a statement of the remote-access
tools or placement currently in use.

**These two options overlap.** A coordinated mesh service like Tailscale *is*
WireGuard, with a coordination server, key distribution, and NAT traversal
layered on top. Running both gives you two remote-access paths, two policy
surfaces, and two ways to bypass the firewall matrix you have just spent six
stages building.

Pick a primary. The tradeoff is genuine in both directions:

| | Self-hosted WireGuard | Coordinated overlay |
|---|---|---|
| Inbound port required | **Yes** — a UDP port must be reachable | No — outbound only |
| Depends on a third party | No | Yes, for coordination |
| Key management | Yours | Handled |
| Works behind CGNAT | Poorly | Yes |

Neither is the correct answer. They are different bets: one trades an inbound
port for independence, the other trades a third-party dependency for reachability.

## The claim this decision commits you to

Assume the reference public service uses an outbound connector with no inbound
HTTP listener. A remote-access choice should not silently broaden that claim.

Self-hosted WireGuard requires an inbound UDP port forward. That does not make
WireGuard the wrong choice — it makes the *claim* need scoping. If you run it,
the honest statement becomes something like:

> The public web service uses an outbound tunnel with no inbound HTTP exposure.
> Remote administration uses a WireGuard endpoint on a single UDP port.

That is still a good posture. It is just not "no open inbound ports" full stop,
and the difference matters the moment someone asks a follow-up question. Decide
this before publishing rather than after being asked — a claim you cannot defend
cold is worse than no claim.

## Placement, and why it matters

Whichever you pick terminates remote access, which means everything a remote
peer can reach, it reaches *through* this host. Putting it behind a narrow administrative policy is what constrains it:

> Remote peers get only the approved administrative destinations and required
> supporting services. Everything else is denied.

The failure to avoid: a VPN peer that lands in a zone with full internal reach
has undone the segmentation for anyone who obtains a key. Authentication proved
*who* the peer is. It did not decide what they may reach.

## Verification — from genuinely outside

```bash
# From cellular or another network, NOT from your own LAN
ip route                          # confirm tunnel routes; watch for a full-tunnel surprise
ssh <the bastion>                 # allowed
ssh <a management host>           # follows the same policy as a local admin client
ping -c2 <a guest-zone host>      # must be denied
dig +short <an internal name>     # does DNS work inside the tunnel?
```

Connecting to your own VPN from your own LAN proves the process is running. It
proves nothing about the external path, NAT traversal, DNS inside the tunnel, or
the rules that apply to a remote peer. This is the single most commonly faked
test in a homelab writeup.

## Stop conditions

- The external test fails after moving the endpoint → revert. You are now blind
  from outside, and the next problem will be the one you cannot reach.
- A remote peer reaches a zone the matrix denies → the policy is wrong, and the
  VPN is currently a hole rather than a control.
- You find yourself running both options "for now" → stop and pick. Two
  remote-access paths is the state this page exists to prevent.

## Done when

- [ ] One primary chosen, with the reason recorded
- [ ] Endpoint on the admin zone, under the policy matrix
- [ ] Full policy tested from a genuinely external client — allows and denies
- [ ] DNS behaviour inside the tunnel confirmed
- [ ] If self-hosted: the inbound-port claim scoped correctly wherever the
      posture is stated publicly
- [ ] The other option removed, or explicitly documented as deliberately
      unused
