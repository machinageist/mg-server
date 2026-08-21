---
title: "How machinageist.dev Is Hosted"
date: 2026-07-08
summary: "How this small Rust site moves from an edge proxy to a private origin, and how I verify DNS, HTTP, and service health across the request path."
category: "Linux / SysAdmin"
tags: [self-hosting, linux, systemd, caddy, cloudflare, dns, proxmox]
---

This site runs on hardware I own. There is no managed application platform and
no database. Traffic reaches an edge proxy, crosses an outbound connector, passes
through a local reverse proxy, and lands on a small Rust service.

## The request path

```text
Browser
  -> DNS (Cloudflare authoritative)
  -> Cloudflare edge
  -> outbound private connector
  -> local reverse proxy
  -> mg-server (Rust/Axum)
```

The connector is outbound, so the application origin does not need a public
listener. Cloudflare terminates browser-facing TLS.

## DNS: where the name points

`machinageist.dev` is a Cloudflare-hosted zone. Anyone can verify the public DNS
boundary without needing a copied snapshot from this post:

```console
$ dig +short machinageist.dev A
$ dig +short machinageist.dev NS
```

The returned addresses belong to the public edge rather than the origin. The
nameserver query confirms where the public zone is delegated.

## The edge response

Relevant response headers from a live request:

```console
$ curl -sSI https://machinageist.dev
HTTP/2 200
content-type: text/html; charset=utf-8
content-security-policy: default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; base-uri 'none'; form-action 'self'; object-src 'none'; frame-ancestors 'none'
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

## The origin service

The local reverse proxy hands requests to a service manager, which starts the
app on boot and restarts it after failure. I verify the service with:

```console
$ systemctl status <service>
$ journalctl -u <service> --since today
```

One deployment failed because the service manager's executable path did not
match the deployed binary. The general lesson was to validate the unit, binary,
permissions, and startup behavior together before replacing the known-good
release. The deployment check now covers the unit configuration, binary path,
permissions, and a fresh-process startup before cutover.

## Why no database

Blog posts and wiki pages are flat Markdown files on disk. The server reads a
`.md` file, parses its YAML frontmatter, converts the body to HTML at request
time, and renders it into a compile-time Askama template. No SQL, no ORM, no
admin panel, no login form. That is partly an architecture preference and partly
a security property: a smaller surface has fewer things to get wrong. It is not a
claim that the site is "secure" — only that there is less of it to attack.

## Operational scope

This is a personal service, not a production platform. The demonstrated scope is
narrow: run, diagnose, and verify a small Linux-hosted Rust service across its
public request path.
