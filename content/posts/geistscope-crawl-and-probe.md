---
title: "GeistScope Part 3: Crawling and Probing"
date: 2026-05-05
summary: "mg-crawl maps the application surface and extracts secrets hidden in JavaScript. mg-probe checks security posture without sending attack payloads. Both run before a single fuzz request goes out."
tags: [rust, security, bug-bounty, geistscope, crawler, web-security]
---

## Before You Attack, You Observe

There's a mental model in security research that distinguishes passive from active work.
Passive: look, listen, read what's already there. Active: send something, change state,
probe with a payload.

Good recon is mostly passive. You want to understand as much as possible about a target
before you start sending unusual requests — because unusual requests show up in logs,
and because understanding the application first means your active testing is targeted
rather than a spray.

After `mg-recon` finishes mapping subdomains, ports, and tech stacks, two tools run
before the fuzzer ever touches the target: `mg-crawl` and `mg-probe`.

---

## mg-crawl: Mapping the Surface

A web application has more to it than its front page.
There are API endpoints, JavaScript files, form actions, internal links, admin paths.
The crawler finds them.

`mg-crawl` runs a breadth-first search from one or more starting URLs:

```bash
mg-crawl target-bounty https://www.target.example.com
```

It follows links, collects JavaScript files, and stays within scope. The defaults
are conservative: depth limit of 2, one request per second, robots.txt honored.
The rate and depth are configurable; robots.txt compliance can be disabled with
`--ignore-robots` for targets where the program explicitly permits it.

For each crawled page and JavaScript file, it writes:
- Raw HTML files to `crawl/<host>/pages/` named by SHA-256 hash
- Raw JS files to `crawl/<host>/js/`
- A link graph in `crawl/<host>/index.json`
- Discovered API endpoints in `crawl/<host>/endpoints.json`
- Any secrets found in `crawl/<host>/secrets.json`

### Secret Extraction

This is the part that finds real bugs.

JavaScript files in modern web applications often contain more than they should.
The client-side code bundles API keys, hardcoded tokens, internal endpoint URLs.
Developers don't always notice what made it into the build artifact.

`mg-crawl` runs a regex catalog over every JS file it downloads. The patterns cover:
- AWS access keys (`AKIA...`)
- GitHub tokens (`ghp_...`, `gho_...`, `ghx_...`)
- JWTs (the three-part base64 structure)
- Slack webhook URLs (`hooks.slack.com/services/...`)
- Stripe keys (`sk_live_...`, `pk_live_...`)
- Google API keys (`AIza...`)
- PEM-encoded private keys
- Generic patterns: `api_key`, `password`, `secret` followed by a value

When a match is found, it's written to `secrets.json` with the source file,
line number, and matched value. This is exactly the kind of output you'd submit
as a "sensitive data exposure" finding — AWS keys in client-side JavaScript have
been critical-severity bugs in multiple bug bounty programs.

### What The Crawler Doesn't Do

It doesn't send attack payloads. It doesn't modify state. It's an observer.
Every request it makes could have come from a legitimate browser. The rate limit
ensures it doesn't look like a DDoS tool to the application's WAF.

The crawler result is also direct input to the next tool.

---

## mg-probe: Security Posture Without Attack Payloads

After crawling, `mg-probe` reads `recon/summary.json` and the crawl output, then
runs a set of checks against every HTTP-accessible in-scope host:

```bash
mg-probe target-bounty
```

It performs four categories of checks:

### 1. Security Headers

An HTTP GET to the root path. The response headers are checked against
a list of expected security headers:

- `Content-Security-Policy` — controls what the browser loads
- `Strict-Transport-Security` — forces HTTPS, prevents SSL stripping
- `X-Frame-Options` or CSP `frame-ancestors` — prevents clickjacking
- `X-Content-Type-Options: nosniff` — prevents MIME sniffing
- `Referrer-Policy` — controls what the Referer header sends on cross-origin links
- `Permissions-Policy` — restricts browser feature access

Each missing header becomes a finding in `findings/`. Missing HSTS is medium
severity. Missing CSP is medium. Some programs pay for these — they're low-effort
findings that real deployments routinely miss.

I built these checks partly because I harden my own server with the same headers.
Writing `mg-probe` was an exercise in encoding what I'd already done manually
into a tool that can check any target automatically.

### 2. CORS Misconfiguration

A GET with a foreign `Origin: https://evil.example.com` header.
If the response includes:

```
Access-Control-Allow-Origin: https://evil.example.com
Access-Control-Allow-Credentials: true
```

That's a critical CORS misconfiguration. An attacker can make authenticated
requests from their domain to this API and read the response. This is a real
bug class that has resulted in significant payouts.

The check pattern: origin reflection + credentials = critical.
Origin reflection without credentials = informational (no session data at risk).

### 3. Cookie Flags

The same GET request that checks headers. The response cookies are inspected
for three flags:

- `Secure` — cookie only sent over HTTPS
- `HttpOnly` — JavaScript cannot read the cookie (blocks XSS-based session theft)
- `SameSite` — controls cross-site cookie submission (mitigates CSRF)

A session cookie missing `HttpOnly` means that if there's any XSS on the
application — even a minor one — an attacker can steal the session token.
Missing `Secure` on a session cookie means the cookie could be sent over HTTP
if the browser is downgraded. These findings compound with other vulnerabilities.

### 4. Exposed Debug Paths

Actual HTTP probes against a list of paths that should never be publicly accessible:
`/swagger-ui.html`, `/.env`, `/actuator/env`, `/phpinfo.php`, `/server-status`,
`/debug/`, `/.git/config`, `/admin`, `/console`.

A 200 response on any of these is a finding. These paths exist in every
major tech stack and are commonly left exposed in development or staging deployments
that got promoted to production.

The check is configurable: `--passive-only` skips the active path probing if
you want to stay entirely passive.

### 5. Stack Traces in Crawl HTML

The crawl output stored HTML for every page. `mg-probe` reads those files
and searches for patterns that indicate an application error leaked into the response:
Python tracebacks, Java stack traces, PHP `Fatal error`, Laravel debug pages,
Django debug pages.

Stack traces in HTTP responses are information disclosure. They reveal file paths,
library versions, database connection strings, and internal architecture. They're
frequently medium severity, and fixing them is trivial — disable debug mode
in production.

---

## What Gets Written

`mg-probe` writes two outputs:

**Finding markdown files** in `findings/` — one per distinct issue, formatted the
same as manually-created findings with frontmatter, description, evidence curl commands,
and remediation notes.

**`recon/probe-report.json`** — a structured summary of all issues found, suitable
for reading by `ai-prioritize` or processing programmatically.

---

## Semi-Active vs Fully Passive

The probe is described as "semi-active" in the code because checking exposed paths
requires making real HTTP requests to specific paths — not just reading response
headers from a GET to root. But it's not *attacking* anything: every request is
a normal GET, no payloads, no mutations, nothing that would show up as malicious
in logs. It's the same class of request that a search engine bot makes.

The distinction matters for bug bounty programs that specify whether passive or
active testing is permitted. `mg-probe --passive-only` restricts to header and
cookie checks, which require only a single GET to the root — indistinguishable
from a normal browser visit.

---

## Why This Order Matters

Recon → Crawl → Probe → Fuzz is a deliberate sequence.

Crawling happens first because it gives the fuzzer input. The endpoints found by
the crawler are the targets for fuzz testing. The secrets found in JS might be
API keys that unlock additional endpoints. You can't intelligently fuzz an API
you haven't mapped yet.

Probing happens before fuzzing because many of the highest-value findings in bug
bounty are passive misconfigurations, not vulnerabilities that require attack payloads.
CORS bugs, missing HSTS, exposed Swagger UI — these are real findings that require
no fuzzing at all. Finding them first means you're not burning fuzz budget on
a target that's already yielding findings from basic header checks.

*Part 4 covers `mg-fuzz`, `mg-replay`, and `ai-prioritize`: active testing,
finding verification, and LLM-ranked attack surface.*
