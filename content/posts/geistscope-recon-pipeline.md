---
title: "GeistScope Part 2: The Recon Pipeline"
date: 2026-05-03
summary: "Four tools, one orchestrator: how subdomain enumeration, port scanning, and tech stack fingerprinting come together into a resumable recon pipeline written in async Rust."
tags: [rust, security, bug-bounty, geistscope, recon, networking]
---

## What Recon Actually Is

Before you can find vulnerabilities in a web application, you need to know what you're
looking at. "Target: target.example.com" is a domain name. What it resolves to,
what services are running on each host, what software stack those services use —
that's what recon tells you.

In a professional engagement, recon is where you figure out the actual attack surface.
The main domain might be a static marketing page with nothing interesting. The subdomain
`api.target.example.com` running an outdated version of Spring Boot with Actuator
endpoints exposed — that's where the findings are.

GeistScope's recon pipeline has four stages, each implemented as a standalone Rust
binary that can also run independently:

1. `subdomain-enum` — find all the subdomains
2. `mg-fingerprint` — identify what technology each host runs
3. `mg-scan` — find open ports on each discovered host
4. `mg-recon` — orchestrate all three stages, produce `summary.json`

---

## Stage 1: Subdomain Enumeration

Subdomains are discovered two ways, and the tool does both:

**Passive — Certificate Transparency logs:**
Every TLS certificate issued for a domain is logged publicly by certificate authorities.
`crt.sh` aggregates these logs and exposes an API. If `api.target.example.com` ever
had a cert issued, it appears in CT logs regardless of whether it's still resolvable.

```bash
subdomain-enum target.example.com --mode passive
```

**Active — DNS brute force:**
A wordlist of common subdomain prefixes (`admin`, `api`, `dev`, `staging`, `mail`,
`auth`, `internal`, ...) is tried as DNS queries. Anything that resolves gets added
to the list. This catches hosts that were never issued a TLS cert.

```bash
subdomain-enum target.example.com --mode active --wordlist common.txt
```

**Both at once**, with results merged and deduplicated:

```bash
subdomain-enum target.example.com --mode all
```

Entries found by both passive and active are tagged `ct_log+brute` in the output —
a good signal that the host is well-established and worth attention.

The async implementation fires up to `--concurrency` (default 100) DNS resolution
tasks simultaneously using Tokio's `JoinSet`. Each subdomain name gets resolved to
IPs concurrently, and the results are merged into a deduplicated hashmap.
When used with an engagement, out-of-scope hosts are dropped before output and
before the JSON is written to `recon/subdomain-enum.json`.

---

## Stage 2: Tech Stack Fingerprinting

Knowing a host exists isn't the same as knowing what it is. `mg-fingerprint` sends
an HTTP request to a URL and classifies the response:

```bash
mg-fingerprint https://api.target.example.com --engagement target-bounty
```

It identifies things like:
- **Server software** from the `Server` header (nginx, Apache, IIS, Caddy)
- **Application frameworks** from headers like `X-Powered-By`, cookie names (`PHPSESSID`
  → PHP, `JSESSIONID` → Java), response patterns
- **CDNs** from headers like `CF-Ray` (Cloudflare), `X-Cache` (various), `Via`
- **Security header presence** — or absence — at a glance

The fingerprint is written into `recon/fingerprint.json` as a map keyed by hostname,
so running fingerprint on multiple hosts accumulates in one file without overwriting
earlier entries.

Why does this matter? Because the attack surface depends entirely on what's running.
A Java application with Spring Boot might have Actuator endpoints at `/actuator/health`,
`/actuator/env`, `/actuator/mappings` — all of which leak internal information and
have been the source of real CVEs. An nginx proxy in front of a Node.js app might
have path normalization quirks. A React frontend calling a Laravel API has a
completely different attack surface than a Rails monolith.

You can't know where to look without knowing what's there.

---

## Stage 3: Port Scanning

I wrote about `mg-scan` in an earlier post on the port scanner itself, but in context
of the recon pipeline: it scans each discovered host for open ports and maps them to
services.

The scan output records which ports are open, what service is likely running on each
(by well-known port number), and any banner text grabbed from the initial connection.
A banner is the first bytes a service sends when you connect — SSH sends a version
string, SMTP says `220 mail.target.example.com ESMTP`, HTTP/1.1 servers send their
response headers.

Banners matter because they sometimes include version numbers. A version number for
a known service is the first thing you cross-reference against published CVEs.

The scanner supports deliberate evasion: randomised port order breaks sequential IDS
signatures, configurable delay and jitter control probe rate, and an optional source
port binding flag lets you send from port 53 or 80 — which occasionally bypasses
naive firewall rules that permit traffic from those ports without inspecting where
it's going.

---

## The Orchestrator: mg-recon

Running three tools manually against a target with 47 subdomains is tedious.
`mg-recon` runs all four stages in sequence, in-process, with one command:

```bash
mg-recon target-bounty
```

What that does, in order:

1. Load the engagement, verify it has a `scope.json`
2. Run subdomain enumeration — skip if `recon/subdomain-enum.json` already exists
3. Run fingerprinting on every HTTP-accessible host — skip already-fingerprinted hosts
4. Run port scanning on each host — skip if `recon/mg-scan.json` is current
5. Merge all results into `recon/summary.json`

The skip logic is the key design decision here. Recon pipelines take time.
Subdomain enumeration against a large program might take several minutes.
Port scanning 47 hosts across a 1024-port range is not instant.
If you run `mg-recon` and the connection drops partway through, you can restart it
and it picks up where it left off. Only stages with missing output files re-run.
Pass `--force` to override.

The `summary.json` output is the canonical input to every downstream tool:
`mg-probe`, `ai-prioritize`, `mg-crawl`. It contains a record for each discovered host:

```json
{
  "hostname": "api.target.example.com",
  "ips": ["203.0.113.42"],
  "source": "ct_log+brute",
  "http_accessible": true,
  "fingerprint": { "server": "nginx", "frameworks": ["node.js"] },
  "open_ports": [80, 443, 8080],
  "services": ["http", "https", "http-alt"]
}
```

---

## In-Process vs Subprocess

An earlier design had `mg-recon` spawning subprocesses — calling the individual tool
binaries as shell commands. That works but has friction: you need the binaries
installed, you parse stdout, error handling is awkward.

The refactor moved to calling the library functions directly in-process.
`subdomain-enum`, `mg-scan`, and `fingerprint` are each a library crate
(`subdomain_enum`, `mg_scan`, `fingerprint`) with a binary entrypoint that just calls
the library. `mg-recon` depends on the library crates and calls the same functions.

This means:
- No subprocess overhead
- Shared types — the same `SubdomainEntry` struct that the library produces is what
  `mg-recon` reads, with the compiler verifying the shape matches
- Richer error handling — `anyhow::Result` propagates properly rather than getting
  serialized to stderr and parsed out of text

The individual binaries still exist and work standalone. The library crate structure
just gives `mg-recon` direct access to the same code.

---

## Running It

```bash
# Initialize an engagement
mg-engagement init target-bounty \
    --target target.example.com \
    --platform hackerone

# Set scope
mg-engagement scope-add target-bounty "*.target.example.com"

# Run full recon pipeline
mg-recon target-bounty --ports 1-1024 --concurrency 100

# See what was found
cat engagements/target-bounty/recon/summary.json | jq '.hosts[] | .hostname'
```

After `mg-recon` finishes, you have a complete picture of the attack surface:
every subdomain, every open port, every tech stack. That's the input to the next
phase — crawling, probing, and finding vulnerabilities.

*Part 3 covers `mg-crawl` and `mg-probe`: crawling the application surface and
checking security posture without sending a single attack payload.*
