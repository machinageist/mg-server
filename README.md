# mg-server

Personal website and web server for **machinageist.dev**, built in Rust.

---

## Overview

`mg-server` is the Rust application behind **machinageist.dev**, my portfolio site for
infrastructure work in Linux, networking, and virtualization. The site covers four
areas: a Proxmox homelab, networking, Linux administration, and a small defensive
security section. Each one is tied to homelab work with evidence behind it.

<!-- Cert claims removed 2026-07-25 by request: no public cert claims until an exam
     voucher is booked. When one is, state only that single exam with its scheduled
     date. See mg-coreforge/PUBLIC_FACE.md for the wording rules. -->

It is also how I am learning Rust and backend development. The app is small and only
does what this site needs:

- It hosts the portfolio, with the homelab, networking, and Linux work and the
  evidence for each.
- It publishes technical writeups with the commands I ran and how I checked the
  result.

The server is self-hosted on hardware I own (a Proxmox Debian VM behind Caddy and a
Cloudflare Tunnel), and running it is part of the portfolio. `IMPROVEMENT_PLAN.md`
covers which claims the site makes and which it does not.

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

Binds to `127.0.0.1:3000` by default, so visit `http://127.0.0.1:3000`. No database or
external service is required. Content is read from `content/` on each request. Set
`MG_BIND_ADDR` to override the bind address (e.g. `0.0.0.0` for LAN testing) and
`RUST_LOG` to control log verbosity (`RUST_LOG=debug` for full `tower` internals).

```sh
cargo test    # includes a drift guard checking wiki SIDEBAR slugs against content/pages/
```

---

## Deployment

The public deployment follows a conventional private-origin pattern:

```text
Browser → managed edge → outbound private connector → reverse proxy → application
```

This keeps the application origin separate from the browser-facing edge. The
architecture, and how I verified it, are in
["How machinageist.dev Is Hosted"](https://machinageist.dev/blog/hosting-machinageist-dev).

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

These are a browser-side baseline for a personal site. They do not make the
application secure on their own. The walkthrough, with live evidence, is in the
["Security Headers on machinageist.dev"](https://machinageist.dev/blog/security-headers-on-machinageist-dev)
post.
