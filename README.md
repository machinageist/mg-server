# mg-server

Personal website and web server for **machinageist.dev**, built in Rust.

---

## Overview

`mg-server` powers **machinageist.dev**, an evidence-first portfolio for an
**infrastructure technician — Linux, networking, and virtualization**. The site is
organized around four pillars — a Proxmox homelab, networking, Linux / SysAdmin, and a
small defensive-security section — each anchored to a homelab project with real evidence.

<!-- Cert claims removed 2026-07-25 by request: no public cert claims until an exam
     voucher is booked. When one is, state only that single exam with its scheduled
     date. See mg-coreforge/PUBLIC_FACE.md for the wording rules. -->

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
│   │   ├── labs.rs          # /labs index + per-lab procedure pages
│   │   ├── glossary.rs      # terms and command reference
│   │   ├── search.rs        # server-rendered /search
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

## Running Locally

Requires a stable Rust toolchain (install via [rustup](https://rustup.rs)).

```sh
git clone https://github.com/machinageist/mg-server.git
cd mg-server
RUST_LOG=info cargo run
```

Binds to `127.0.0.1:3000` by default — visit `http://127.0.0.1:3000`. No database or
external service is required; content is read from `content/` at startup. Set
`MG_BIND_ADDR` to override the bind address (e.g. `0.0.0.0` for LAN testing) and
`RUST_LOG` to control log verbosity (`RUST_LOG=debug` for full `tower` internals).

```sh
cargo test    # includes a drift guard checking wiki SIDEBAR slugs against content/pages/
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
