# mg-server

Personal website and web server for **machinageist.dev**, built in Rust.

---

## Overview
This project exists primarily as a learning platform for backend development and the Rust ecosystem. It serves two main purposes:

- Hosting personal programming projects
- Publishing technical blog posts that document the learning process

The server is **self-hosted locally**, providing hands-on experience with infrastructure, deployment, and security.

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

mg-server/
├── content
│   └── posts
│       ├── blog-draft-mg-server.md
│       ├── memory-safety-c-vs-rust.md
│       └── port-scanner-in-rust.md
├── README.md
├── src
│   ├── errors.rs
│   ├── handlers
│   │   ├── blog.rs
│   │   ├── mod.rs
│   │   └── pages.rs
│   ├── main.rs
│   ├── middleware
│   │   ├── mod.rs
│   │   ├── rate_limit.rs
│   │   └── security_headers.rs
│   ├── models
│   │   ├── mod.rs
│   │   ├── post.rs
│   │   └── project.rs
│   └── router.rs
├── static
│   ├── assets
│   ├── css
│   │   └── style.css
│   └── js
│       └── main.js


---

## Deployment

The application is hosted on a **Proxmox cluster** inside a dedicated **Debian VM**.

Traffic flow: 
 - Client → Cloudflare Tunnel → Caddy Reverse Proxy → mg-server

 - This architecture allows the site to be publicly accessible without exposing the home network or public IP address, while Caddy handles TLS termination and reverse proxying.

---

## Security

The server is configured with several defensive HTTP response headers to reduce common web attack surfaces and enforce safer browser behavior.

- **Content-Security-Policy**  
  Restricts which sources the browser can load scripts, styles, images, and other resources from. This significantly reduces the risk of Cross-Site Scripting (XSS) and content injection attacks.

- **Permissions-Policy**  
  Disables or limits access to sensitive browser features such as camera, microphone, and geolocation. This enforces a least-privilege model for client-side capabilities.

- **Referrer-Policy**  
  Limits how much URL information is shared when users navigate away from the site, reducing the risk of sensitive data leakage.

- **Strict-Transport-Security (HSTS)**  
  Forces browsers to use HTTPS for all future connections, preventing downgrade and SSL-stripping attacks.

- **X-Content-Type-Options**  
  Prevents MIME-type sniffing and ensures browsers only interpret files as their declared content type.

- **X-Frame-Options**  
  Prevents the site from being embedded in iframes, mitigating clickjacking attacks.

Together, these controls provide a strong baseline of browser-enforced protections that complement server-side security practices.
