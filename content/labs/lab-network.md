---
title: "Isolated lab VMs"
date: 2026-08-14
summary: "The zone whose members are meant to be broken — why a disposable machine is the correct instrument for proving segmentation, and the scope discipline that governs it."
tags: [labs, networking, isolation, vlan, scope]
---

## The value here is the isolation, not the machines

A zone whose members are expected to be rebuilt, broken, or thrown away is the
correct place to test whether the negative firewall rules actually hold. That is
the entire argument for this zone existing, and it is why
[the lab cutover](/labs/segmentation-lab) goes first: the failure costs nothing.

These machines are instruments. If one becomes something you depend on, it has
left this zone conceptually and should leave it in the configuration too.

## Scope discipline

Everything done from these machines stays inside owned, local, or explicitly
authorised scope. Concretely:

- **In scope:** your own hosts on your own network; deliberately vulnerable
  targets you deploy yourself; hosted training environments, inside their own
  environments.
- **Out of scope:** anything you do not own; anything a neighbour's device
  happens to answer; and — once zones exist — anything outside this zone without
  a deliberate, documented exception.

That second point is not hypothetical. A wireless scan does not respect VLAN
boundaries, and "it answered" is not authorisation. The zone constrains what
routes; it does not constrain radio.

## Why a disposable VM is the right test instrument

For proving the segmentation works, these machines have three properties nothing
else in the lab has:

- genuinely disposable — rebuilt from an installer image in minutes;
- nothing depends on them, so downtime is free; and
- they carry network diagnostic tooling already, which makes them good at
  *measuring* whether a zone behaves as designed.

That third property is what makes them useful beyond being expendable. Proving a
zone contains traffic requires a host inside it that can try, thoroughly, to get
out.

## Verification

From inside the zone, the negative tests are the point:

```bash
# These must all fail — failing is the pass condition
ping -c2 <a management host>
curl -sS --max-time 5 https://<a servers host>
ping -c2 <the trusted gateway>
ssh <any host outside this zone>

# These must succeed
getent hosts example.com
curl -sSI https://example.com | head -1
```

And from outside, confirm the reverse: an admin client should be able to reach
these machines over SSH when the policy allows it, and nothing else should reach
them at all.

```bash
# From an admin host — allowed
ssh <a lab host>

# From the trusted or servers zones — must fail
ssh <a lab host>
```

## Stop conditions

- Any negative test passes → the zone is not containing traffic, which makes it
  a subnet rather than a zone. Fix the policy before using these machines for
  anything.
- A service that something else depends on ends up running here → move it. This
  zone offers no availability guarantees and is the first place you will break
  things deliberately.

## Done when

- [ ] Lab guests on the lab zone with working egress
- [ ] Every negative test fails, evidenced
- [ ] Reachable over SSH from the admin zone only
- [ ] Scope boundaries written down before the machines get used for anything
- [ ] Rebuild-from-image path tested, so "disposable" is a fact rather than an
      intention
