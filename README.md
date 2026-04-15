# mg-server

Personal website and web server for **machinageist.dev**, built in Rust.

Claude Sonnet 4.6 assisted the creation of this project.

---

## Overview

This project exists primarily as a learning platform for backend development and
the Rust ecosystem. It serves as:

- A portfolio and resume for security and systems work
- A blog for technical writeups documenting the learning process
- A foundation for understanding web security from the implementation side

The server is self-hosted on a Proxmox hypervisor inside a dedicated Debian VM,
accessible publicly without exposing the home network or IP address.

---

## Tech Stack

| Component | Purpose |
|---|---|
| **Axum** | Web framework and routing |
| **Askama** | Compile-time HTML templating — template errors are build errors, not runtime crashes |
| **Pulldown-cmark** | Markdown → HTML conversion for blog content |
| **tower-http** | Static file serving and request tracing |
| **governor** | Application-level rate limiting |
| **Caddy** | Reverse proxy and automatic TLS |
| **Cloudflare Tunnel** | Public access without exposing home IP |

---

## Project Structure

```
mg-server/
├── Cargo.toml                  # Dependencies and project metadata
├── Cargo.lock                  # Pinned dependency versions — committed to git
├── content/
│   └── posts/                  # Blog posts as Markdown files with YAML frontmatter
├── src/
│   ├── main.rs                 # Entry point — logging, router, TCP listener
│   ├── router.rs               # All route definitions and middleware wiring
│   ├── errors.rs               # SiteError enum — typed errors mapped to HTTP responses
│   ├── handlers/
│   │   ├── pages.rs            # Home, about, portfolio handlers
│   │   └── blog.rs             # Blog list and single post handlers with slug validation
│   ├── models/
│   │   ├── post.rs             # BlogPost — frontmatter parsing and Markdown rendering
│   │   └── project.rs          # Project struct and hardcoded portfolio list
│   └── middleware/
│       ├── security_headers.rs # Adds all defensive HTTP headers to every response
│       └── rate_limit.rs       # Token bucket — 60 req/min, returns 429 when exceeded
├── templates/                  # Askama HTML templates — validated at compile time
│   ├── base.html               # Master layout: nav, main content slot, footer
│   ├── index.html
│   ├── about.html
│   ├── portfolio.html
│   ├── blog_list.html
│   ├── blog_post.html
│   ├── error_404.html
│   └── error_500.html
└── static/                     # Files served directly — no processing
    ├── css/style.css
    └── js/main.js
```

---

## Running Locally

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/mg-server
cd mg-server

# Build and run
cargo build --release
RUST_LOG=info ./target/release/mg-server

# Visit http://localhost:3000
```

Requirements: Rust toolchain (rustup.rs). No other dependencies.

---

## Deployment

Hosted on a Proxmox hypervisor inside a dedicated Debian 13 VM.
Managed as a systemd service — starts automatically on boot, restarts on failure.

Traffic flow:
```
Client → Cloudflare Tunnel → Caddy reverse proxy → mg-server (localhost:3000)
```

This architecture keeps the home IP out of DNS entirely. Caddy handles TLS
termination. Cloudflare handles the public edge. mg-server only speaks HTTP
on localhost — it is never directly exposed to the network.

---

## Security

Defensive HTTP response headers are applied to every response by a single
middleware layer in `src/middleware/security_headers.rs`.

- **Content-Security-Policy** — restricts which sources the browser can load
  scripts, styles, and resources from. Significantly reduces XSS and content
  injection attack surface. All JS and CSS are in external files — no inline
  scripts means CSP enforcement is clean.

- **Strict-Transport-Security (HSTS)** — forces browsers to use HTTPS for all
  future connections. Prevents SSL stripping and downgrade attacks. Cached by
  the browser for one year — enforcement happens client-side before any request
  is sent.

- **X-Content-Type-Options** — prevents MIME-type sniffing. Ensures browsers
  only interpret files as their declared content type, closing a file-upload
  attack vector.

- **X-Frame-Options** — prevents the site from loading inside an iframe.
  Mitigates clickjacking attacks where an attacker embeds the site invisibly
  to steal clicks.

- **Referrer-Policy** — limits URL information shared when users navigate away.
  Prevents leaking path structure and query parameters to external sites.

- **Permissions-Policy** — explicitly disables browser features this site does
  not use (camera, microphone, geolocation, payment). Injected scripts cannot
  request these features even if they execute.

- **Server header removed** — does not advertise software name or version.
  Forces active fingerprinting rather than passive header reading.

Application-level rate limiting caps requests at 60 per minute using a token
bucket algorithm, returning 429 Too Many Requests when exceeded.

---

## License

MIT — see LICENSE file.
```
