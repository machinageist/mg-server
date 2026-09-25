---
title: "Security Headers on machinageist.dev"
date: 2026-07-09
summary: "The HTTP response headers this site sets: what each one does, the real curl -I output, where they come from in the code, and what they do not cover."
category: "Security"
tags: [security, defensive, http-headers, csp, hsts, owned-scope]
---

mg-server adds a set of security headers to every HTTP response. This post goes
through them: what each header tells the browser to do, what they look like on the
wire, and where they are set in the code. Headers are one layer of defense on a
personal site, and the last section covers what they do not do.

## The headers on the wire

Security headers returned by a live request:

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

(Header set reviewed 2026-08-20. You can run the same command and compare.)

## What each header does

- **Content-Security-Policy** — restricts where the browser may load resources
  from. `default-src 'self'` means only this origin. There are no inline scripts
  and no third-party CDNs. Even if an injection vector existed, the browser would
  refuse to load an off-origin `<script>`. `base-uri`, `form-action`, and
  `object-src` close other HTML-injection paths. `frame-ancestors 'none'` blocks
  framing at the CSP level.
- **Strict-Transport-Security** — tells the browser to use HTTPS for two years
  (`max-age=63072000`), including subdomains, and marks the site preload-eligible.
  After the first visit the browser upgrades to HTTPS on its own, which closes the
  SSL-stripping window on repeat visits.
- **X-Content-Type-Options: nosniff** — the browser trusts the declared
  `Content-Type` instead of guessing, so a file can't be re-interpreted as script.
- **X-Frame-Options: DENY** — no framing from any origin, a second clickjacking
  control alongside `frame-ancestors`.
- **Referrer-Policy: strict-origin-when-cross-origin** — cross-origin navigations
  leak only the origin, not the full path.
- **Permissions-Policy** — denies camera, microphone, geolocation, and payment.
  This site uses none of them.
- **Server header removed** — mg-server strips its own `Server` header so it does
  not advertise a name or version. (`server: cloudflare` above is the edge, not the
  app.)

## Where they come from

They are set in one response middleware, applied at the router so it runs on
every response. The two main policies:

```rust
headers.insert(
    "content-security-policy",
    "default-src 'self'; script-src 'self'; style-src 'self'; \
     img-src 'self' data:; font-src 'self'; connect-src 'self'; \
     base-uri 'none'; form-action 'self'; object-src 'none'; \
     frame-ancestors 'none'".parse().unwrap(),
);
headers.insert(
    "strict-transport-security",
    "max-age=63072000; includeSubDomains; preload".parse().unwrap(),
);
```

The complete implementation is in the
[router-wide response middleware](https://github.com/machinageist/mg-server/blob/main/src/middleware/security_headers.rs).

I checked the live `curl -I` output against the source. The CSP and the two-year
HSTS match.

## Limits

- Headers are one layer, and the browser enforces them. They do not fix bugs in
  the application. CSP limits what an injection can do, but it does not remove
  the injection.
- TLS terminates at Cloudflare's edge, not on my VM, so HSTS protects the
  connection from the browser to the edge.
- The site has no accounts, no write API, and no database. Search queries and
  study answers are still input from strangers, so they are bounded, parsed, and
  escaped like any other input.

The source and tests for these headers are in the repository.
