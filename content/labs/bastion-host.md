---
title: "Bastion host"
date: 2026-08-14
summary: "What actually makes something a bastion rather than a jump box with extra steps, the honest question to answer before building one, and why ProxyJump beats agent forwarding."
tags: [labs, linux, ssh, bastion, hardening]
---

## What makes something a bastion

Not "a Linux box you SSH to first." A bastion is defined by three properties,
and without them it is a jump box with extra steps:

1. It is the **only** path into a zone.
2. Its access is **logged**.
3. It has a **narrow allowlist** of what it may reach.

## The honest question to answer before building it

This is a generic hardening pattern. The examples do not describe current
management reachability or a deployed bastion.

If a policy matrix allows clients to reach management directly, then management
is reachable without the bastion and the bastion is not the only path into
anything.

That is a perfectly legitimate design. But then say what it actually is: the
landing point for remote administration, not an internal segmentation control.
Claiming "bastion host architecture" for something anyone on the trusted network
can bypass is the kind of claim that collapses on the first follow-up question.

Decide and record:

- [ ] May trusted reach management directly, or must it traverse the bastion?
- [ ] Which hosts may be reached **through** the bastion, explicitly listed?
- [ ] Where do session logs go, and who reviews them?

## Build

Minimal is the point. A bastion with a large package set is a large attack
surface at your most privileged network position.

```bash
sudo apt update && sudo apt upgrade -y
sudo apt install -y openssh-server fail2ban
```

Harden `/etc/ssh/sshd_config`:

```text
PermitRootLogin no
PasswordAuthentication no
PubkeyAuthentication yes
AllowUsers <your-admin-user>
X11Forwarding no
MaxAuthTries 3
LoginGraceTime 30
ClientAliveInterval 300
ClientAliveCountMax 2
```

```bash
sudo sshd -t                 # validate BEFORE restarting
sudo systemctl restart ssh
```

Run `sshd -t` before every restart, and keep a second session open until you
have confirmed the new config accepts a fresh login. A syntax error in this file
locks you out of the host whose entire job is being reachable.

## Use ProxyJump, not agent forwarding

Agent forwarding exposes your local SSH agent to the bastion — anyone with root
there can use your key while your session is open. `ProxyJump` keeps
authentication on your workstation and uses the bastion purely as transport:

```text
# ~/.ssh/config on your workstation
Host bastion
    HostName <the bastion address>
    User <your-admin-user>

Host node-*
    ProxyJump bastion
    User <your-admin-user>
```

Then `ssh node-01` connects through the bastion without your key ever being
usable on it. This is the difference between a bastion that reduces risk and one
that concentrates it.

## Verification

```bash
ssh bastion                          # reachable from the trusted zone
ssh -J bastion <a management host>   # jump works
```

And the denies, which are the half worth testing:

```bash
# From the bastion — these must fail unless explicitly allowlisted
ping -c2 <a guest-zone host>
curl -sS --max-time 5 https://<an unapproved service>
```

Confirm the login was logged, and that you can find it. A bastion whose logs
nobody can locate satisfies property 2 on paper only.

## Stop conditions

- `sshd -t` fails → fix it before restarting. Do not restart to "see if it
  works".
- The bastion can reach hosts outside its allowlist → the policy is wrong, and
  the bastion is currently making lateral movement easier rather than harder.

## Done when

- [ ] Role decided and written down: only path, or remote-access landing point
- [ ] Allowlist of reachable hosts explicitly defined
- [ ] Key-only SSH, root login disabled, validated config
- [ ] `ProxyJump` configured on the workstation, agent forwarding not required
- [ ] Session logging on, destination known, and a test login located in it
- [ ] Denies tested from the bastion itself
