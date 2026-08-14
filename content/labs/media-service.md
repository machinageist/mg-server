---
title: "Media service, LAN-only by default"
date: 2026-08-14
summary: "The service most likely to tempt you into punching a hole in a security model you just spent months building — and the access models ranked by what they actually cost."
tags: [labs, linux, containers, media, exposure]
---

## Why this page is cautious

This is the service most likely to tempt you into undoing the segmentation. It
does not exist yet — no guest, no install — which makes now the right time to
decide how it will be reached, rather than after it is running and someone wants
it on their phone.

The rule it exists under: do not infer that because the public web service uses
an outbound tunnel, every other service should. Make that a separate risk
decision, after the base segmentation works.

## Default: LAN-only. Public exposure is a separate, documented decision.

A media server transcodes untrusted input, has a large attack surface, and has
had authentication and path-traversal vulnerabilities. It is not a hardened edge
service and should not be treated as one.

| Access model | Risk | Verdict |
|---|---|---|
| LAN-only, trusted zone → servers zone | Low | **Start here** |
| Over the VPN you already built | Low | Good — remote access without exposure |
| Behind a tunnel, with authentication in front | Medium | Only with real access control |
| Behind a tunnel, unauthenticated | High | No |
| Inbound WAN port-forward | High | **Never** — it breaks the model the whole project rests on |

One practical note that catches people: **CDN terms commonly restrict proxying
large volumes of non-HTML content**, which is exactly what video streaming is.
Do not assume the tunnel serving a website is an appropriate transport for
media. Check the current terms before relying on it — this is the kind of
assumption that quietly becomes a published claim you cannot defend.

The honest default: watch it at home, or reach it over the VPN that already
exists for administration. That covers the real use case without adding an
internet-facing service to a network you just finished locking down.

## Placement

On the [container host](/labs/container-host), in the servers zone. Per the
policy matrix that zone gets DNS, NTP, updates, and tunnel egress, and is denied
management, trusted, admin, and lab.

Which means the media service can reach the internet to fetch metadata, and
cannot reach your hypervisors. That is the correct shape.

## Verification

```bash
# From the trusted zone — should work
curl -sSI http://<the container host>:<port> | head -1

# From the guest zone — must fail
curl -sS --max-time 5 http://<the container host>:<port>

# From the media container itself — must fail
ping -c2 <a management host>
```

If you later decide on remote access, re-run the guest-zone test afterwards. The
common mistake is adding an exposure path and verifying only that it works,
never that the paths which should still be closed still are.

## Stop conditions

- The service answers from the guest zone → the [container host's port
  binding](/labs/container-host#containers-punch-holes-in-your-firewall-know-this-first)
  is wildcarded, or the policy is wrong. Fix before adding libraries.
- You are about to add an inbound port-forward → stop. That is the one option on
  the table that invalidates the model every other stage was built to establish.

## Done when

- [ ] Running on the container host, in the servers zone
- [ ] Reachable from the trusted zone
- [ ] **Not** reachable from the guest or lab zones — tested, not assumed
- [ ] Access model chosen deliberately and written down
- [ ] No inbound WAN exposure
