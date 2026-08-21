---
title: "Security Headers on machinageist.dev"
date: 2026-07-09
summary: "The HTTP response headers this site sets — what each does, the real curl -I output, where they come from in the code, and the honest limits of header hardening on a personal site."
category: "Security"
tags: [security, defensive, http-headers, csp, hsts, owned-scope]
---

This is a small, defensive writeup for an owned site. mg-server stamps a set of
HTTP security-response headers on every response, and this post documents them:
what each header tells the browser to do, the real headers on the wire, and where
they are set in the code. It is scoped honestly — this is header hardening on a
personal site, not a claim that the application is "secured."

## The headers on the wire

Live capture (Cloudflare's own reporting headers trimmed):

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

(Header set reviewed 2026-08-20. Run it yourself; a header audit should be
reproducible.)

## What each header does

- **Content-Security-Policy** — restricts where the browser may load resources
  from. `default-src 'self'` means only this origin; there are no inline scripts
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
- **Permissions-Policy** — denies camera, microphone, geolocation, and payment
  outright; this site uses none of them.
- **Server header removed** — mg-server strips its own `Server` header so it does
  not advertise a name or version. (`server: cloudflare` above is the edge, not the
  app.)

## Where they come from

These are set in one response middleware applied at the router level so it runs
on every response. A shortened example:

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
// ...
headers.remove("server");
```

A useful verification point: the live `curl -I` output and the source config
agree — same CSP, same two-year HSTS. The header the browser receives is the
header the code sets.

## Honest limits

- Headers are one browser-enforced layer. They do not fix an application logic
  bug; CSP mitigates the impact of injection, it does not remove the vector.
- **TLS terminates at Cloudflare's edge**, not on my VM, so HSTS protects the
  browser-to-edge leg. That tradeoff is worth stating rather than glossing.
- This is an owned personal site with no accounts, write API, or database.
  Search queries and study forms are still attacker-controlled input, so they
  are bounded, parsed, and escaped rather than dismissed as "no user input."
- **Safe claim:** reviewed and documented the HTTP security headers for an owned
  web service, with reproducible `curl` evidence. **Not** a claim to have "secured
  the application."

This is one layer in the site's defensive-security work. The source and tests are
public; private infrastructure procedures and control gaps are not.
