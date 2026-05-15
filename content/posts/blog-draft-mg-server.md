---
title: "Building mg-server: A Rust Web Server From Scratch"
date: 2026-04-05
summary: "How I built a personal portfolio and blog in Rust — from zero to a hardened, production-ready server — and what it taught me about systems programming and web security."
tags: [rust, axum, web, security, learning]
---

## Why Build a Web Server in Rust

The easy answer would have been WordPress. Install it, pick a theme, write posts.
Done in an afternoon.

I didn't want the easy answer. I wanted to understand what a web server actually
does — every request, every response, every byte between the browser and the disk.
And I wanted to learn Rust, which has been sitting at the top of my list since I
realized that most of the CVEs I was reading about in C codebases were memory safety
bugs that Rust's compiler would have rejected outright.

So I built mg-server: a personal site and web server written entirely in Rust, with
no framework shortcuts I didn't understand, no database, and security baked in at
every layer rather than bolted on at the end. This is the writeup of how it went.

---

## The Stack

Before writing any code, I had to make decisions about the technology stack. Each
decision taught me something.

**Axum** for the web framework. Axum is built on top of Tokio (Rust's async runtime)
and Hyper (a low-level HTTP library). It adds routing and typed request extractors
while keeping the underlying machinery visible. The plan is to eventually rewrite the
backend in raw Tokio and Hyper — stripping away the framework to understand what it
was doing. But Axum was the right starting point because it let me learn Rust's
ownership model through building rather than through toy exercises.

**Askama** for HTML templates. Askama compiles your template files into Rust code at
build time. If a template references a variable that doesn't exist in your struct —
compile error. If a template file is missing — compile error. Nothing breaks at
runtime in production because the compiler already verified everything.
This is not how Python's Jinja2 works. The difference matters in ways I'll get to.

**Markdown files on disk** instead of a database. Blog posts live as `.md` files in
`content/posts/`. The server reads them, parses the frontmatter and body, converts
Markdown to HTML with `pulldown-cmark`, and renders the result into a template.
No SQL, no ORM, no database connection strings in environment variables.

The security implication of this choice is real: no database means no SQL injection
surface. No admin panel means no brute-forceable login page. No PHP runtime means no
remote code execution via file upload. A smaller attack surface is not just cleaner
architecture — it is a security property.

---

## Building It — The Parts That Worked

### Step 1: Getting a Response Back

The first thing the server did was return a string.

```rust
async fn hello() -> &'static str {
    "Hello from your Rust server."
}
```

This feels trivial. It isn't. To get here I had to understand: what `#[tokio::main]`
does (starts the async runtime), what `async fn` means (returns a Future, not a value),
what `&'static str` means (a string slice that lives for the program's lifetime), and
why Axum could turn that into an HTTP response automatically (the `IntoResponse` trait).

Four concepts hidden in one line. Rust doesn't let you skip them.

### Step 2: Static Files and the First Security Test

Axum's `tower-http` crate provides `ServeDir` — a service that maps a URL prefix to
a directory on disk. Adding it to the router:

```rust
.nest_service("/static", ServeDir::new("static"))
```

That's the implementation. The interesting part was testing it.

Once the static file server was running, I ran `gobuster` against it — a tool that
brute-forces URL paths using a wordlist, the same recon technique an attacker uses
to find hidden directories and files.

```bash
gobuster dir -u http://localhost:3000 -w /usr/share/wordlists/dirb/common.txt
```

Gobuster found `/static`. It found nothing else. No admin panel, no backup files,
no configuration endpoints — because none of those exist.

Then I tried a directory traversal:

```bash
curl "http://localhost:3000/static/../../../etc/passwd"
```

`ServeDir` sanitizes paths before opening files. The request returned 404.
I had just verified my first defensive control.

### Step 3: Templates and Why Compile-Time Matters

With Askama, you define a struct in Rust and link it to a template file:

```rust
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: String,
}
```

Every field in the struct is available as `{{ name }}` in the template. If the
template tries to use a field that doesn't exist, `cargo build` fails with an error
pointing at the exact line.

Compare this to server-side template injection (SSTI) — a vulnerability class in
Python and PHP applications where user-supplied input reaches the template engine at
runtime. The engine evaluates it. An attacker who controls what the template processes
can execute arbitrary code on your server. It's a real attack with a real CVE class
(SSTI, CWE-94).

Askama doesn't have a template engine at runtime. It was compiled away at build time.
There's nothing to inject into.

---

## Building It — The Parts That Didn't

Real projects have bugs. Mine had several. Documenting them here because the bugs
are where the learning actually happened.

### The Compiler Can't See String Literals

Rust's compiler is famously strict. It verifies ownership, lifetimes, types, and
trait implementations exhaustively. It is not able to verify that `"contents/posts"`
is a typo for `"content/posts"`. Both are valid strings.

I spent longer than I'd like to admit on a `post not found` error that turned out
to be a single extra character in a path string. The fix was to define a constant:

```rust
const POSTS_DIR: &str = "content/posts";
```

One declaration. One place to fix. One place to typo. The compiler still can't check
the value — but now there's only one value to check.

### Frontmatter Parsers Are Whitespace-Sensitive

The `gray_matter` crate parses YAML frontmatter from Markdown files. The `---`
delimiter must start on column 1 of line 1 with nothing before it.

My text editor inserted a single leading space before the `---`. The parser saw no
frontmatter block and returned `None`. The server returned `post not found` for
every blog post.

Diagnosis: `cat -A content/posts/port-scanner-in-rust.md | head -1`

Output: ` ---$` — space visible before the dashes.

The `cat -A` flag makes invisible characters visible. It is now in my permanent
debugging toolkit. When a parser silently returns nothing, check for invisible
characters at the delimiter.

### Feature Flags Are Opt-In

Rust crates expose optional functionality behind feature flags. I needed Rust's
`RUST_LOG`-driven log filtering, which lives behind the `env-filter` feature in
`tracing-subscriber`. The compiler error:

```
the item is gated behind the `env-filter` feature
```

Fix: in `Cargo.toml`, change:

```toml
tracing-subscriber = "0.3"
```

to:

```toml
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt"] }
```

This is a security consideration as much as a build consideration. Every feature
flag you enable is additional compiled code — additional attack surface. Enabling
only what you need is a form of minimizing exposure. `cargo geiger` reports how
much `unsafe` code each enabled feature pulls in.

---

## The Security Layer

After Phase 1 (a working site), Phase 2 added hardening. One middleware function
that runs on every response and adds security headers:

```
Content-Security-Policy: default-src 'self'; script-src 'self' ...
Strict-Transport-Security: max-age=31536000; includeSubDomains
X-Content-Type-Options: nosniff
X-Frame-Options: DENY
Referrer-Policy: strict-origin-when-cross-origin
Permissions-Policy: camera=(), microphone=(), geolocation=(), payment=()
```

Each header is a sentence to a browser describing how to behave defensively.

CSP tells the browser which sources are allowed to load scripts. Even if an attacker
finds an XSS vector and injects a `<script src="https://evil.example/payload.js">`,
the browser refuses to load it — the policy said only `'self'` is allowed.

HSTS tells the browser to always use HTTPS and never accept HTTP — even if the
first request goes out over HTTP, the browser will have cached the HSTS policy from
a previous visit and will upgrade the connection itself before sending anything.
SSL stripping attacks, where an attacker intercepts the HTTP request before it can
be upgraded, can't work against a browser that won't make HTTP requests in the first place.

X-Frame-Options prevents the page from loading in an iframe. Clickjacking — where
an attacker embeds your site invisibly inside their page and tricks users into
clicking your invisible buttons — requires iframe embedding. One header eliminates it.

The rate limiter caps requests at 60 per minute per server instance. Running a brute
force or enumeration tool against the server produces `429 Too Many Requests` after
the first 60. I tested this:

```bash
for i in $(seq 1 70); do
    curl -s -o /dev/null -w "%{http_code}\n" http://localhost:3000/
done
```

Output: `200` sixty times, then `429`.

---

## What Running It Against Scanners Taught Me

Before the server was deployed publicly, I ran attack tools against it locally.
Not to find vulnerabilities — to verify the defenses worked and to understand what
each tool was seeing.

**nmap** fingerprinted port 3000 as HTTP within seconds. Service version detection
works by sending known protocol probes and matching responses against a database of
signatures. My server doesn't advertise its version — but the HTTP response structure
is recognizable regardless.

**gobuster** found `/static` and nothing else. No `/wp-admin`, no `/phpmyadmin`,
no `/.env`. Those paths don't exist — a scanner that finds nothing is working correctly.

**curl with traversal payloads** confirmed that `ServeDir` sanitized paths before
touching the filesystem. CVE-2021-41773 was a Critical-rated Apache path traversal
bug. It was exploited widely in the wild within days of disclosure. The fix was
exactly what `tower-http` does by default: normalize and reject paths that escape
the root directory.

Knowing about CVE-2021-41773 is different from having tested the defense that
prevents it. I've now done both.

---

## What Comes Next

**Step 9**: Deploy to a homelab VM on a mini-PC (arriving Thursday). The plan is
a Proxmox hypervisor, Ubuntu VM, Caddy reverse proxy, and Cloudflare Tunnel for
public access — an outbound tunnel that creates inbound connectivity without
exposing the home IP or opening firewall ports. The same architecture is used in
legitimate remote access tools and in red team C2 infrastructure. Understanding
how it works here means understanding it in both contexts.

**Step 10**: Decompose the monolith. Strip out Axum and rewrite the HTTP layer
with raw Tokio and Hyper — implementing routing, request parsing, and response
building from scratch. Add a Svelte frontend that calls a JSON API. This is the
maximum-learning version of the stack, and it mirrors the real production
architecture you encounter in a web application penetration test.

---

## What This Project Taught Me

Before this project I understood HTTP conceptually. Now I've built a server that
speaks it. I've handled routes, parsed requests, built responses, applied middleware,
and watched the whole exchange in `curl -v` output and Wireshark captures.

Before this project I had read about XSS, SSTI, path traversal, clickjacking, and
rate limiting. Now I've implemented defenses against all of them and verified those
defenses with real attack tools.

The Rust compiler rejected my code dozens of times before it ran once. Every
rejection was correct. The bugs it caught before runtime were exactly the class of
bugs that become CVEs in C codebases: type mismatches, missing error handling,
incorrect lifetimes, unreachable code paths.

The bugs it couldn't catch — string literal typos, whitespace in frontmatter,
missing feature flags — are the bugs that tests and debugging tools exist for.
The full picture is: Rust catches what the type system can model, tests catch
what it can't, and `cat -A` catches the rest.

---

*This site is built with mg-server. Source available on GitHub.*
*Next post: building the port scanner this server replaced as the demo project.*
