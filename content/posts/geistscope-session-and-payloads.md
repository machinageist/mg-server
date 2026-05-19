---
title: "GeistScope Part 8: Authenticated Testing and Stack-Aware Payloads"
date: 2026-05-15
summary: "session manages engagement auth configuration without ever touching plaintext credentials. payload-engine reads what the fingerprinter found and selects payloads tuned to the actual stack — MySQL-specific SQLi, Jinja2-specific SSTI, GCP-specific SSRF."
tags: [rust, security, bug-bounty, geistscope, fuzzing, authentication]
---

## Two Problems That Come Before Fuzzing

By the time you're ready to throw payloads at a target, two things need to be
solved. First: the tool needs to authenticate. Many interesting endpoints are
behind a login — and you're testing with a legitimate account, not bypassing auth.
Second: the payload list needs to be relevant. Sending MySQL-specific SQL injection
strings to a Postgres database wastes requests and dilutes results.

`session` solves the first problem. `payload-engine` solves the second.

---

## `session`: Auth Without Plaintext Secrets

Engagement directories contain a `session.json` file that stores auth configuration.
The word "configuration" is precise — what gets stored is *references* to secrets,
not the secrets themselves:

```json
{
  "token_env": "ACME_API_TOKEN",
  "token_header": "Authorization",
  "token_prefix": "Bearer",
  "login_url": null,
  "session_cookie": null
}
```

At runtime, the session library resolves `ACME_API_TOKEN` from the environment and
builds the `HeaderMap` that gets attached to every outbound request. If the env var
isn't set, the tool errors out before making a single request — you won't accidentally
fuzz an authenticated endpoint without credentials and wonder why everything returns 401.

Three auth patterns are supported in the current slice:

**Bearer token.** The most common for API programs. Set `token_env` to the name
of an environment variable and `token_header` to the header name. Done.

**Cookie-based session.** Set `session_cookie` to the session cookie name and
`token_env` to the variable holding the cookie value. The session library builds
a `Cookie` header from those.

**Custom header.** Same pattern — `token_header` takes any header name, so
`X-Api-Key` or `X-Auth-Token` work the same way as `Authorization`.

Form login and OAuth refresh are in the design but not yet wired — the current
slice covers the patterns that account for the majority of API programs on HackerOne.

The scope guard is wired into session too. Every `build_headers` call validates
that the target URL is in scope before returning auth headers. There's no code path
that attaches your token to a request going to an out-of-scope host.

---

## `payload-engine`: Payloads That Know the Stack

The fingerprinter (covered in Part 2) writes structured tech stack information into
`recon/summary.json`. `payload-engine` reads that file and adjusts what it hands
to the fuzzer.

The payload sets:

| Name | What it probes |
|---|---|
| `sqli` | SQL injection — generic + engine-specific extensions |
| `xss` | Reflected/stored XSS, attribute injection, event handlers |
| `ssrf` | Server-side request forgery, including cloud metadata endpoints |
| `ssti` | Server-side template injection, Jinja2/Twig/Freemarker/Velocity variants |
| `lfi` | Local file inclusion, path traversal |
| `xxe` | XML external entity injection |
| `open-redirect` | Open redirect probes |
| `prompt-injection` | LLM prompt injection for AI-adjacent surfaces |

The interesting part is how stacks narrow the list. A few examples:

**MySQL backend** adds `LOAD_FILE`, `SLEEP`-based timing probes, and `INTO OUTFILE`
variants to the SQLi set. A generic SQLi payload won't confirm MySQL-specific
behavior. A MySQL payload that returns different timing for `SLEEP(5)` vs `SLEEP(0)`
will.

**GCP-hosted target** adds `http://metadata.google.internal/computeMetadata/v1/`
to the SSRF set. AWS targets get `http://169.254.169.254/latest/meta-data/`. Azure
gets `http://169.254.169.254/metadata/instance`. Sending an AWS metadata URL to
a GCP instance will never succeed — the cloud-specific payload does.

**Jinja2 template engine** (Django, Flask) adds `{{7*7}}`, `{{''.__class__}}`,
and the standard Jinja2 SSTI chain. Twig gets its own variants. Freemarker gets
`<#assign ex="freemarker.template.utility.Execute"?new()>`.

**Prompt injection** payloads are categorized by technique: system prompt leak
attempts, instruction injection, jailbreak patterns, exfiltration probes, and
role-reversal attacks. Callers can filter to a single category when testing a
specific attack surface.

The context is inferred automatically from the engagement summary:

```rust
let ctx = get_payload_context_from_engagement(&engagement);
let payloads = get_payloads(PayloadSet::Sqli, &ctx);
```

If the fingerprinter found Django and Postgres, `ctx` has `framework: Some(Django)`
and `backend_db: Some(Postgresql)`, and the payload set includes Postgres-specific
timing probes and Jinja2 SSTI strings without any manual configuration.

---

## Why This Pairing Matters

These two libraries don't run as standalone binaries — they're imported by `mg-fuzz`
and by the `mg-harness` dispatcher. But they encode two decisions that are worth
making once and enforcing everywhere:

Credentials never touch disk as plaintext. A `session.json` committed to the
engagement directory doesn't expose anything — it just says which env var to look up.

Payload selection isn't a text file you curate manually. It's derived from what
the scanner observed. When the fingerprinter updates its summary, the next fuzz
run automatically picks up the right payload variants.

---

*Part 9 covers `security-graph`: a local JSONL-backed graph that models relationships
between hosts, parameters, identities, and findings — the data structure that
connects everything the pipeline discovers.*
