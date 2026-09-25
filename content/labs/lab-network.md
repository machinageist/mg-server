---
title: "Isolated lab VMs"
date: 2026-08-14
summary: "The zone whose machines are meant to be broken: why a disposable machine is the right tool for proving segmentation, and the scope rules that apply to it."
tags: [labs, networking, isolation, vlan, scope]
---

## The value here is the isolation, not the machines

A zone whose machines are expected to be rebuilt, broken, or thrown away is the
right place to test whether the deny rules hold. That is why this zone exists,
and it is why [the lab cutover](/labs/segmentation-lab) goes first. A failure
here costs nothing.

These machines are instruments. If one becomes something you depend on, it has
left this zone conceptually and should leave it in the configuration too.

## Scope discipline

Everything done from these machines stays inside owned, local, or explicitly
authorized scope. Concretely:

- **In scope:** your own hosts on your own network, intentionally vulnerable
  targets you deploy yourself, and hosted training environments, used inside
  those environments.
- **Out of scope:** anything you do not own, anything a neighbor's device
  happens to answer, and, once zones exist, anything outside this zone without a
  documented exception.

That second point comes up in practice. A wireless scan does not stop at VLAN
boundaries, and "it answered" is not permission. The zone limits what gets
routed. It does not limit radio.

## Why a disposable VM is the right test instrument

For proving the segmentation works, these machines have three properties nothing
else in the lab has:

- They are disposable, and can be rebuilt from an installer image in minutes.
- Nothing depends on them, so downtime is free.
- They already carry network diagnostic tools, which makes them good at
  measuring whether a zone behaves as designed.

The third property is what makes them useful for more than being expendable. To
prove a zone contains traffic, you need a host inside it that tries hard to get
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
  a subnet instead of a zone. Fix the policy before using these machines for
  anything.
- A service that something else depends on ends up running here → move it. This
  zone offers no availability guarantees and is the first place you will break
  things on purpose.

## Done when

- [ ] Lab guests on the lab zone with working egress
- [ ] Every negative test fails, evidenced
- [ ] Reachable over SSH from the admin zone only
- [ ] Scope boundaries written down before the machines get used for anything
- [ ] Rebuild-from-image path tested, not just planned
