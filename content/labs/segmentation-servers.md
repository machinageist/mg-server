---
title: "S5 — Public services last"
date: 2026-08-14
summary: "The only publicly visible cutover. Prove the pattern on a service nobody watches, keep the outbound-tunnel model, and verify the public path from genuinely outside."
tags: [labs, networking, segmentation, servers, cloudflare-tunnel]
---

## The one publicly visible stage

In this reference plan, a public service moves **last**, behind a lower-risk
service that proves the pattern first.

## Order

1. **A lower-risk service first** — the container host, or anything nobody is
   watching. Prove egress and the internal denies with something whose failure
   costs nothing.
2. Prove the policy matrix from the new zone.
3. **Prepare everything for the public service before touching its tag** — target
   address, DNS, firewall aliases, rollback. All of it, in advance.
4. Move it, and update its address consistently everywhere it is referenced.
5. Verify local origin, resolver behaviour, tunnel registration, and public HTTP
   **from outside the home network**.

## Keep the outbound-tunnel model

The reference public path uses an outbound connector to an edge proxy. Do not
silently add a second ingress path during a migration.

Changing the ingress model to solve a routing problem adds a new attack surface
and invalidates the design being tested.

If the tunnel breaks after the move, fix the tunnel. Do not route around it.

## The denies are the interesting half

Per the reference matrix, a public-service zone gets only name resolution,
time synchronization, update access, connector egress, and explicit
dependencies. Administrative and unrelated internal networks are denied.

That last part is the point: a compromised public-facing service should not be
able to reach the management plane. Test the deny policy with representative
fixtures from your own authorized environment.

Record both allowed and denied probes, including the policy under test, result,
and time of verification.

## Verify the public path from outside

```bash
# From cellular, or any host NOT behind this router
curl -sS -o /dev/null -w '%{http_code} %{time_total}s\n' https://<your domain>
dig +short <your domain>
```

Checking from inside proves the origin responds. It does not prove the tunnel
registered, that public DNS resolves, or that the path works end to end. Test
from outside or you have not tested.

## Tunnel specifics after an address change

The tunnel is an outbound connection from the origin host to the edge. Changing
the host's local address means:

- [ ] The daemon must still resolve DNS and reach the internet from the new zone
- [ ] The tunnel re-registers — confirm a fresh connection, not a stale cached
      one
- [ ] The local origin target may need updating if it referenced the old subnet
- [ ] **Nothing about the public DNS record should need to change.** If you find
      yourself editing public DNS to fix this, stop and work out why first —
      that is a symptom of a different problem.

## Stop conditions

- The site is unreachable from outside after the move → revert first, diagnose
  second.
- The servers zone can reach management → policy failure, and precisely the risk
  segmentation exists to remove. Revert.
- The tunnel will not re-register → revert. Do not "temporarily" open an inbound
  port; temporary firewall exceptions have a way of becoming the architecture.

## Done when

- [ ] Lower-risk service moved, egress proven, internal denies proven
- [ ] Public service on its authoritative address in the service zone
- [ ] Service-zone deny policy verified against administrative and unrelated zones
- [ ] Local origin, resolver, and tunnel registration verified
- [ ] Public HTTP verified from outside the home network
- [ ] No inbound WAN port-forward exists — confirmed, not assumed
- [ ] The topology document updated with the new address
