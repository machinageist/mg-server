---
title: "Remote access: pick one primary"
date: 2026-08-14
summary: "Self-hosted WireGuard or a coordinated overlay, the claim each one commits you to, and why running both undoes the segmentation you just built."
tags: [labs, networking, vpn, wireguard, remote-access]
---

## Read this before installing anything

These two options overlap. A coordinated mesh service like Tailscale is
WireGuard, with a coordination server, key distribution, and NAT traversal on
top. Running both gives you two remote-access paths, two sets of policy, and two
ways around the firewall matrix you just spent six stages building.

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

Self-hosted WireGuard needs an inbound UDP port forward. That does not make it
the wrong choice, but it changes what you can claim. If you run it, an accurate
statement looks something like:

> The public web service uses an outbound tunnel with no inbound HTTP exposure.
> Remote administration uses a WireGuard endpoint on a single UDP port.

That is still a good setup. It is just not "no open inbound ports", and the
difference matters as soon as someone asks a follow-up question. Decide which
claim you are making before you publish it, not after someone asks.

## Placement, and why it matters

Whichever you pick terminates remote access, which means everything a remote
peer can reach, it reaches *through* this host. Putting it behind a narrow administrative policy is what constrains it:

> Remote peers get only the approved administrative destinations and required
> supporting services. Everything else is denied.

The failure to avoid: a VPN peer that lands in a zone with full internal reach
has undone the segmentation for anyone who obtains a key. Authentication proved
*who* the peer is. It did not decide what they may reach.

## Verification from outside the network

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
  VPN is a hole in the policy instead of a control.
- You find yourself running both options "for now" → stop and pick. Two
  remote-access paths is the state this page exists to prevent.

## Done when

- [ ] One primary chosen, with the reason recorded
- [ ] Endpoint on the admin zone, under the policy matrix
- [ ] Full policy tested from an external client, both allows and denies
- [ ] DNS behavior inside the tunnel confirmed
- [ ] If self-hosted: the inbound-port claim scoped correctly wherever the
      posture is stated publicly
- [ ] The other option removed, or documented as intentionally unused
