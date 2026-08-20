---
title: "S3 — Untrusted and approved clients"
date: 2026-08-14
summary: "The two zones the household actually lives on, gated on whether the access point does real per-SSID VLAN tagging — and the one property GUEST must never lose."
tags: [labs, networking, segmentation, wireless, firewall]
---

## This stage is gated on a hardware capability

This is a sanitized wireless-segmentation pattern. It does not publish the
deployed zone names, management allowances, or probe targets.

Which zone you deploy first is not a preference. It depends on one question you
have to answer from the access point's own configuration:

**Does the AP support per-SSID 802.1Q VLAN tags?**

| If yes | If no |
|---|---|
| Trunk only the required VLANs to the AP | Use it as a single-VLAN access port |
| GUEST and TRUSTED can be genuinely separate broadcast domains over one AP | Do not treat the AP's built-in "guest mode" as a separate broadcast domain |

That second column matters more than it looks. Most consumer guest modes are
client isolation plus a firewall rule — not a distinct layer-2 segment. A guest
network sharing a broadcast domain with trusted clients fails the only test this
zone exists to pass.

Verify it in the configuration, not from the marketing copy.

## The non-negotiable GUEST property

GUEST gets DNS through the approved resolver and internet access. Nothing else.

```bash
# From a GUEST client — every one of these MUST fail
ping -c2 <a management host>
ping -c2 <the trusted gateway>
ping -c2 <a servers host>
ping -c2 <any other private address>
curl -sS --max-time 5 http://<the switch management address>

# And this MUST succeed
curl -sSI https://example.com | head -1
```

There is a subtlety worth checking: if GUEST's resolver is the firewall's own
address *on the GUEST interface*, that is fine. If it points at a resolver
sitting on another VLAN, you have punched a hole straight through your own deny
rule and the ping tests above will still pass. Confirm which is actually
configured.

## Approved clients have broader allowances, and more risk

Approved clients should receive only the services their role requires. Do not
turn the label "trusted" into unrestricted east-west access; default-deny
everything not named in the private policy matrix.

This is your admin workstation's home, which makes it the zone whose
misconfiguration locks you out of everything else. Before moving your own
machine onto it:

- [ ] Confirm the management allow rules exist and are correct **first**
- [ ] Keep the wired recovery path below available
- [ ] Move a second, non-critical device first if you have one

## Keep a wired recovery path while changing Wi-Fi

If you reconfigure the access point from a wireless client *on that access
point*, you will disconnect yourself mid-change with no way back in. Use a wired
port for this stage. This is not a precaution, it is the difference between a
change window and an incident.

One specific hazard: modern phones enable private or randomized MAC addresses by
default, per network. Any MAC-based VLAN assignment or DHCP reservation for such
a device will silently stop matching when the address rotates. Either disable
randomization for this network on that device, or accept that it lands in the
default VLAN and design for that.

## Verification

- [ ] GUEST client: internet yes, DNS yes, **every** private destination denied
- [ ] Approved client: required services reachable and unrelated zones denied
- [ ] Cross-check from the other direction — a host in the servers zone must not
      be able to initiate a connection to TRUSTED
- [ ] AP capability documented honestly: real per-SSID tagging, or single-VLAN
      access port

That third check is the one people skip. A policy tested in one direction has
proven a route exists, not that the reverse route is closed.

## Stop conditions

- A GUEST client reaches any internal destination → the zone is not a zone.
  Revert and fix the policy before continuing.
- The AP turns out not to support real VLAN tagging and you deployed as though
  it did → stop and redesign. This one cannot be papered over with firewall
  rules, because the traffic never reaches the firewall.
- You lose the wired recovery path mid-change → stop, restore it, resume.

## Done when

- [ ] AP VLAN capability verified in its configuration
- [ ] Whichever zone was deployed proves both its allows and its denies
- [ ] GUEST, if deployed, cannot reach any internal subnet — tested and
      evidenced
- [ ] Wired recovery path was available throughout and is documented for reuse
- [ ] Rollback rehearsed for the AP configuration specifically
