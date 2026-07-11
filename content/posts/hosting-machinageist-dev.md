---
title: "How machinageist.dev Is Hosted"
date: 2026-07-08
summary: "The request path for this site — DNS to Cloudflare to a Cloudflare Tunnel to Caddy to a systemd-managed Rust service on a Proxmox Debian VM — with real dig and curl output, and an honest list of what is not yet in place."
category: "Linux / SysAdmin"
tags: [self-hosting, linux, systemd, caddy, cloudflare, dns, proxmox]
---

This site runs on hardware I own: a Proxmox mini-PC server at home. There is no
cloud provider, no managed platform, and no database. This post traces exactly
how a request reaches the page you are reading, with real command output, and it
is honest about what is missing. It is a small self-hosting artifact, not a
production platform.

## The request path

```text
Browser
  -> DNS (Cloudflare authoritative)
  -> Cloudflare edge (TLS terminates here)
  -> Cloudflare Tunnel (outbound connection from my VM)
  -> Caddy (reverse proxy on the VM)
  -> systemd service (mg-server.service)
  -> mg-server (Rust/Axum app)
```

Two things are worth calling out. First, **TLS terminates at Cloudflare's edge**,
not on my VM. Second, the connection from home to Cloudflare is an **outbound**
tunnel — my VM dials out to Cloudflare and Cloudflare routes public traffic back
down that connection. Nothing inbound is opened on my home network, and my home
IP is never exposed. No port-forwarding on the router.

## DNS: where the name points

`machinageist.dev` is a Cloudflare-hosted zone. A live lookup:

```console
$ dig +short machinageist.dev A
104.21.60.62
172.67.192.181

$ dig +short machinageist.dev NS
maeve.ns.cloudflare.com.
rick.ns.cloudflare.com.
```

The A records are Cloudflare anycast addresses, not my home IP — that is the
proxy layer doing its job. The nameservers confirm the zone is delegated to
Cloudflare. (Captured 2026-07-08.)

## The edge response

`curl -I` against the live site (Cloudflare's own reporting headers trimmed for
readability):

```console
$ curl -sSI https://machinageist.dev
HTTP/2 200
content-type: text/html; charset=utf-8
content-security-policy: default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'
strict-transport-security: max-age=63072000; includeSubDomains; preload
x-content-type-options: nosniff
x-frame-options: DENY
referrer-policy: strict-origin-when-cross-origin
permissions-policy: camera=(), microphone=(), geolocation=(), payment=()
server: cloudflare
```

`HTTP/2 200` confirms the whole path is up. `server: cloudflare` is the edge; my
own Rust service removes its `Server` header rather than advertising a version.
The security headers are stamped by mg-server itself — I cover them in a separate
[security-headers post](/blog/security-headers-on-machinageist-dev).

## Caddy and the systemd service

On the VM, Caddy is the reverse proxy in front of the app, and the app runs as a
`systemd` unit so it starts on boot and restarts if it dies. The operational
commands I use are the ordinary ones:

```console
$ systemctl status mg-server.service
$ journalctl -u mg-server.service -n 50 --no-pager
$ systemctl restart mg-server.service
```

An earlier incident on this service was a `203/EXEC` failure — systemd could not
execute the binary because the `ExecStart` path did not match where the binary
actually lived. The fix was aligning the unit's `ExecStart` with the deployed
binary path. I am writing that up separately as a proper incident report; I am
not going to reconstruct log timestamps from memory here.

## Why no database

Blog posts and wiki pages are flat Markdown files on disk. The server reads a
`.md` file, parses its YAML frontmatter, converts the body to HTML at request
time, and renders it into a compile-time Askama template. No SQL, no ORM, no
admin panel, no login form. That is partly an architecture preference and partly
a security property: a smaller surface has fewer things to get wrong. It is not a
claim that the site is "secure" — only that there is less of it to attack.

## What is honestly not here yet

- **No automated monitoring or alerting.** If the service goes down, I find out by
  looking. A homelab monitoring stack is planned for a later cert phase.
- **No tested backup/restore of the VM.** Proxmox backup/restore with real RPO/RTO
  notes is one of my planned homelab projects, not something I have validated yet.
- **No CI/CD.** Deployment is manual. There is no automatic rollback if a new
  binary is wrong — which is exactly how the `203/EXEC` incident happened.
- **TLS boundary caveat.** Because TLS terminates at Cloudflare, the edge sees
  plaintext. That is an accepted tradeoff of this setup, worth stating plainly.

Those gaps are the honest edge of this artifact, and several of them are the next
things I am building toward.
