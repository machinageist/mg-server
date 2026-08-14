---
title: "Container host and its workloads"
date: 2026-08-14
summary: "The container runtime punches holes in your firewall by design — how it happens, four ways to stop it, and why rootless Podman is worth choosing on a RHEL-family lab."
tags: [labs, linux, containers, podman, firewall]
---

## Role

The container host for internal services, and the **first** guest to move to the
servers zone — the lower-risk service that proves the pattern before the
publicly visible one follows.

## Containers punch holes in your firewall. Know this first.

Docker manipulates `iptables` directly and **bypasses host firewall rules** in
the common case. A container published with `-p 8096:8096` binds the wildcard
address and is reachable from anywhere the host is reachable — *including zones
your policy matrix denies* — because the Docker chain is evaluated before the
rules you wrote.

This is the single most common way a carefully segmented network leaks, and it
leaks silently: every rule you wrote is still there, still correct, and no longer
reached.

**Mitigations, in order of preference:**

1. **Bind published ports to a specific address**, never the wildcard:

   ```yaml
   ports:
     - "<the host's zone address>:8096:8096"   # not "8096:8096"
   ```

2. Enforce policy on **the firewall** — the inter-zone path — rather than
   relying on the host's own rules.
3. Disable the runtime's firewall management entirely, only if you are prepared
   to manage all container networking rules yourself. Powerful, and easy to get
   wrong.
4. Use **rootless Podman**, which does not manipulate the host firewall the same
   way.

Whichever you choose, **verify from another zone** that only the intended ports
answer. Testing from the host proves nothing about this failure mode.

## Podman is worth choosing here

If you have not already built workloads on Docker, use Podman:

| | Docker | Podman |
|---|---|---|
| Daemon | Yes, root | Daemonless |
| Rootless | Possible, less common | Default |
| Firewall interference | Significant | Less — rootless uses user networking |
| Service integration | Compose | Native systemd units |

The systemd integration is the part that matters operationally: `podman generate
systemd`, or Quadlet `.container` units, makes a container a normal service the
host supervises, logs, and restarts. That is a better fit for a machine you
manage with the same tools as everything else — and it is the same tooling the
RHEL-family work on the [study box](/labs/rhel-study-box) uses.

## Verification

The important test is from a **different zone**, not from the host:

```bash
# From a host in another zone — only intended ports may answer
nmap -Pn --top-ports 100 <the container host>
curl -sS --max-time 5 http://<the container host>:8096
```

Then confirm the containers themselves are constrained:

```bash
# From the container host — the servers-zone denies still apply
ping -c2 <a management host>       # must fail
getent hosts example.com           # must work
```

If a port answers from a zone the matrix denies, the runtime is bypassing the
policy. Fix that before adding any workload.

## Stop conditions

- A published port answers from a denied zone → stop. Adding workloads to a host
  that leaks makes the leak bigger.
- The workload inventory cannot be produced → document what is running before
  moving it. An unknown set of containers on a network being locked down is the
  thing most likely to break in a way nobody can diagnose.

## Done when

- [ ] Every running workload documented — what it is, what it listens on, what
      it needs to reach
- [ ] Runtime chosen deliberately, with the firewall interaction understood
- [ ] Published ports bound to a specific address, not the wildcard
- [ ] Port exposure verified **from another zone**
- [ ] Servers-zone denies confirmed from the host itself
- [ ] Services supervised by systemd rather than started by hand
