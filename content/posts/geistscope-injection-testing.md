---
title: "GeistScope: Injection and Web Vulnerability Testing"
date: 2026-05-17
summary: "Eight focused injection testers pull parameter lists from the crawl corpus and cover XSS, SQLi, SSTI, command injection, XXE, path traversal, open redirect, and HTTP request smuggling."
tags: [rust, security, bug-bounty, geistscope, injection, xss, sqli, web-security]
---

## The Crawl as Input

Every injection tester in this group reads its targets from the [crawl corpus](/wiki/geistscope-crawl-and-probe). The endpoints and parameters discovered by `mg-crawl` are the injection points. This avoids the brute-force approach of guessing parameter names and instead focuses on parameters the application actually uses.

Eight tools, each focused on a specific vulnerability class. They're designed to run
independently or in sequence after crawling completes.

---

## mg-xss: Reflected XSS Detection

```bash
mg-xss target-bounty
```

The tool uses a two-phase approach. Phase one sends a unique marker string into each
parameter and checks the response body for reflection. This confirms the application
echoes the value without modification before any payload goes in. Phase two runs XSS
payloads only against parameters that reflected the marker — reducing noise and false
positives significantly.

Blind XSS uses an out-of-band URL via `--oob-url`:

```bash
mg-xss target-bounty --oob-url https://your.oob-listener.example.com/token
```

When the payload triggers in a context where the tester can't see the response (admin
panels, email templates, server-side rendering of user input), the OOB callback confirms
execution. The [mg-oob listener](/wiki/geistscope-ssrf-cloud) handles the callback side.

---

## mg-sqli: Error-Based and Time-Based Blind

```bash
mg-sqli target-bounty
```

Two detection modes run in sequence. Error-based injection sends payloads that cause
database errors when the parameter is injectable. These run concurrently via Tokio's
`JoinSet` — many parameters tested in parallel.

Time-based blind is different. The test baseline is measured first: how long does
this endpoint take to respond normally? Then the time-delay payload goes in. If the
response takes significantly longer than baseline, the parameter is injectable. This
only works sequentially because concurrent time-based probes against the same endpoint
produce unreliable timing data — server load from parallel requests skews the baseline.

The distinction between concurrent error-based and sequential time-based is an
intentional design decision, not a performance oversight.

---

## mg-ssti: Template Engine Detection and RCE

```bash
mg-ssti target-bounty
```

Server-side template injection starts with engine identification. The tool injects
`{{7*7}}` and `{{7*'7'}}` into each parameter and checks the response for `49` or
`7777777`. These two payloads produce different output in different engines: Jinja2
evaluates `{{7*7}}` to `49` but `{{7*'7'}}` to an error; Twig evaluates both to `49`;
Smarty treats them differently. The response matrix narrows the likely engine.

After engine identification, a code execution payload runs `id`. If the response
contains `uid=` followed by a number, that's RCE confirmed. SSTI-to-RCE is a common
path in applications that build templates from user input.

---

## mg-cmdinject: Command Injection with OOB

```bash
mg-cmdinject target-bounty --oob-url https://your.oob-listener.example.com/token
```

Three detection methods: blind OOB (inject a `curl` or `nslookup` payload that hits
the OOB listener), error-based (inject syntax that causes shell errors visible in the
response), and time-based (inject `sleep 5` and measure response time delta).

Like SQLi, time-based command injection runs sequentially. Concurrent `sleep` payloads
to the same endpoint produce unreliable timing results when the server processes
requests in parallel.

OOB is the most reliable method when available. A DNS callback from a `nslookup`
payload confirms code execution even when the application swallows error output and
returns a generic 200.

---

## mg-xxe: XML External Entity Injection

```bash
mg-xxe target-bounty --oob-url https://your.oob-listener.example.com/token
```

The tool finds XML-accepting endpoints by content type — looking for `application/xml`,
`text/xml`, or `application/soap+xml` in the crawl endpoint data. These are the
candidates for XXE.

Classic XXE (reading local files and reflecting them in the response) and blind XXE
(triggering an OOB connection that carries file contents in the URL or body) are
both tested. Blind XXE requires the OOB URL to receive the exfiltrated data.

The most common payloads attempt to read `/etc/passwd` and `/etc/hostname`. A
successful response or OOB callback with file contents is a HIGH finding.

---

## mg-traversal: Path Traversal with Smart Targeting

```bash
mg-traversal target-bounty
```

Not every parameter is a path traversal candidate. `mg-traversal` only injects into
parameters whose names suggest file handling: `file`, `path`, `doc`, `page`, `template`,
`include`, and similar. It also targets parameters whose values contain a dot, which
suggests a filename.

Payloads use classic `../` sequences and URL-encoded variants. Confirmation requires
matching known file content patterns in the response: root shell entry format from
`/etc/passwd`, Windows boot file signatures from `C:\boot.ini`. A reflection without
a matching pattern isn't flagged as a finding — this keeps false positives low at
the cost of occasionally missing a traversal that doesn't reach a known file.

---

## mg-redirect: Open Redirect Detection

```bash
mg-redirect target-bounty
```

`mg-redirect` runs with `reqwest`'s redirect policy set to none. Every request that
receives a 3xx response gets its `Location` header inspected manually rather than
followed automatically. This is the correct approach: following redirects would just
land the requester at the destination and miss the finding entirely.

The tool injects redirect targets into parameters that suggest URL handling: `url`,
`redirect`, `next`, `return`, `continue`, `goto`. Each injected value is an external
domain. A `Location` header pointing to the injected domain is an open redirect.

Multi-hop chains are detected by following the chain manually, step by step, inspecting
each intermediate location header.

---

## mg-smuggle: HTTP Request Smuggling

```bash
mg-smuggle target-bounty
```

Request smuggling requires raw TCP, not an HTTP library. `mg-smuggle` opens raw
TCP connections (and TLS connections via `tokio-rustls`) and writes crafted HTTP/1.1
requests byte by byte.

Three variants are tested: CL.TE (frontend uses Content-Length, backend uses
Transfer-Encoding), TE.CL (reversed), and TE.TE (both support Transfer-Encoding but
handle an obfuscated header differently). Detection uses two methods: timing (a
smuggled chunk that the backend waits for causes the connection to hang) and
status-code analysis of the pipelined response, where the backend processes part of
the smuggled request and returns an unexpected response code.

Smuggling findings go directly to HIGH or CRITICAL depending on impact. A CL.TE
vulnerability on an edge proxy can be used to poison requests for other users.

---

## Finding Format

Each tool writes to `findings/` using the same frontmatter format as every other
GeistScope tool: title, severity, tool, timestamp, affected URL, parameter, evidence.
This consistency matters when `mg-timeline` and `ai-prioritize` aggregate findings
across a full engagement. Every finding from every tool is structured the same way
and can be read without knowing which tool produced it.
