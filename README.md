# mg-server

Personal website and web server for **machinageist.dev**, built in Rust.

---

## Overview

`mg-server` powers **machinageist.dev**, an evidence-first portfolio for a
**Systems Administrator / NOC Technician (in training)**. The site is organized
around five pillars — a Proxmox homelab, networking, Linux / SysAdmin, a small
defensive-security section, and a four-CompTIA-cert journey (Network+ → Security+
→ Linux+ → Server+, targeted January 2027) — with each cert anchored to a homelab
project.

It is also, honestly, a **learning platform** for Rust and the backend ecosystem:
the app itself is a small, deliberately narrow self-hosting artifact, not a
production platform. It serves two purposes:

- A public portfolio hub for the homelab/networking/Linux evidence built in
  `~/tech-skill-up/`.
- Publishing technical writeups that document the work with real commands and
  verification.

The server is **self-hosted** on owned hardware (a Proxmox Debian VM behind Caddy
and a Cloudflare Tunnel), which is itself one of the portfolio artifacts. Scope is
kept honest: no production-grade, SRE, high-availability, or "secured the app"
claims — see `IMPROVEMENT_PLAN.md` for the claim-defense discipline.

---

## Tech Stack

| Component | Purpose |
|---|---|
| **Axum** | Web framework and routing |
| **Askama** | Compile-time HTML templating |
| **Pulldown-cmark** | Markdown → HTML conversion for blog content |
| **tower-http** | Static file serving and request tracing |
| **governor** | Application-level rate limiting |
| **Caddy** | Reverse proxy and automatic TLS |
| **Cloudflare Tunnel** | Public access without exposing home IP |

---

## Project Structure

```text
mg-server/
├── content
│   ├── posts                # blog posts, grouped by pillar via `category` frontmatter
│   │   ├── hosting-machinageist-dev.md
│   │   ├── security-headers-on-machinageist-dev.md
│   │   └── solarpunk-is-an-operations-question.md
│   ├── pages                # curated education wiki (served at /learn)
│   └── drafts                # unrouted — archived writing/portfolio text awaiting rewrite
│       ├── port-scanner-in-rust.md
│       ├── memory-safety-c-vs-rust.md
│       ├── geistscope-retrospective.md
│       └── portfolio-entries.md
├── docs                     # planning docs, claim-defense, geistscope page triage
├── IMPROVEMENT_PLAN.md
├── README.md
├── src
│   ├── errors.rs
│   ├── handlers
│   │   ├── blog.rs          # blog list (pillar-grouped) + single post
│   │   ├── pages.rs         # home, start-here, about, portfolio
│   │   ├── wiki.rs          # archive index + pages, hardcoded SIDEBAR
│   │   ├── releases.rs
│   │   ├── well_known.rs    # security.txt, robots.txt
│   │   └── mod.rs
│   ├── main.rs
│   ├── middleware
│   │   ├── mod.rs
│   │   ├── rate_limit.rs
│   │   └── security_headers.rs
│   ├── models
│   │   ├── mod.rs
│   │   ├── page.rs
│   │   ├── post.rs
│   │   └── project.rs
│   └── router.rs
├── static
│   ├── assets
│   ├── css
│   │   └── style.css
│   └── js
│       └── main.js
└── tests
    └── wiki_pages.rs        # drift guard: SIDEBAR slugs <-> content/pages files
```

---

## Deployment

The application is hosted on a **Proxmox** host inside a dedicated **Debian VM**.

Request path:

```text
Browser → DNS (Cloudflare) → Cloudflare edge (TLS terminates) → Cloudflare Tunnel
       → Caddy → systemd (mg-server.service) → mg-server (Axum)
```

This makes the site publicly reachable without exposing the home network or public
IP address and without opening inbound ports; TLS terminates at the Cloudflare
edge. The full walkthrough, with real `dig` and `curl -I` output, is the
["How machinageist.dev Is Hosted"](https://machinageist.dev/blog/hosting-machinageist-dev)
post.

---

## Security

The app stamps several defensive HTTP response headers on every response (see
`src/middleware/security_headers.rs`) to reduce common browser-side attack surface:

- **Content-Security-Policy** — restricts where scripts, styles, and other
  resources may load from (`default-src 'self'`), reducing XSS/content-injection
  impact.
- **Permissions-Policy** — denies camera, microphone, geolocation, and payment.
- **Referrer-Policy** — limits URL leakage on cross-origin navigation.
- **Strict-Transport-Security (HSTS)** — forces HTTPS on repeat visits, closing
  the SSL-stripping window.
- **X-Content-Type-Options: nosniff** — the browser trusts the declared type.
- **X-Frame-Options: DENY** — blocks framing (clickjacking).

These are a browser-enforced baseline for an owned personal site — **not** a claim
that the application is "secured." The defensive walkthrough with live evidence is
the ["Security Headers on machinageist.dev"](https://machinageist.dev/blog/security-headers-on-machinageist-dev)
post.
