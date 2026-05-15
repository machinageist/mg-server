---
title: "GeistScope Part 4: Fuzzing, Verification, and AI Prioritization"
date: 2026-05-07
summary: "mg-fuzz sends attack payloads and diffs every response. mg-replay re-executes curl evidence to verify findings before submission. ai-prioritize uses an LLM to rank what to look at first."
tags: [rust, security, bug-bounty, geistscope, fuzzing, ai]
---

## Active Testing Starts Here

The first three posts in this series covered the observation phase: mapping subdomains,
scanning ports, fingerprinting tech stacks, crawling the application, checking
security posture. All of that is read-only work — nothing was modified, nothing was
attacked.

This post covers the active phase: sending attack payloads and measuring what changes.

---

## mg-fuzz: Burp Intruder in Rust

If you've done web security testing, you've probably used Burp Suite's Intruder.
The concept: take a raw HTTP request, mark positions in it with a placeholder,
define a list of payloads, and fire a request for each payload while comparing
the responses against a baseline. Response differences indicate something interesting.

`mg-fuzz` is that tool, implemented in async Rust.

### The Template Format

You start with a raw HTTP request saved to a file, marking injection positions
with `§` delimiters:

```http
POST /api/users/search HTTP/1.1
Host: api.target.example.com
Content-Type: application/json

{"query": "§PAYLOAD§", "limit": 10}
```

The `§PAYLOAD§` marker tells `mg-fuzz` where to inject. You can have multiple
markers in one template for multi-position attacks.

### Payload Sets

Built-in payload lists cover the most common attack classes:

- `sqli` — SQL injection strings: `' OR '1'='1`, `'; DROP TABLE users--`, UNION selects, time-based payloads
- `xss` — Cross-site scripting: `<script>`, event handlers, encoded variants
- `ssti` — Server-side template injection: `{{7*7}}`, `${7*7}`, `<%= 7*7 %>` for various template engines
- `traversal` — Path traversal: `../`, URL-encoded variants, null bytes
- `ssrf` — Server-side request forgery: metadata endpoints, internal IP ranges, common cloud provider metadata URLs
- `common-passwords` — For credential-based endpoints
- `http-methods` — Tests non-standard methods (OPTIONS, TRACE, PUT, PATCH, DELETE)
- `numbers:N-M` — Integer range, useful for IDOR testing (object ID enumeration)
- `/path/to/custom.txt` — Your own wordlist

### Attack Modes

Four modes, same names as Burp Intruder's:

**Sniper** — one position at a time. If you have three markers and 100 payloads,
that's 300 requests. Each request injects into one position with the other
positions empty.

**Battering Ram** — all positions get the same payload simultaneously.
Useful when you want to test the same string in multiple fields at once.

**Pitchfork** — parallel iteration. Position 1 gets payload list 1's first item,
position 2 gets payload list 2's first item, simultaneously. Good for username/password pairs.

**Cluster Bomb** — cartesian product. Every combination of every payload across
every list. Can generate huge request counts — use with a tight rate limit.

### Response Diffing

Every fuzz run starts by taking a baseline: sending the template with empty
payloads and recording the response (status code, body hash, content length, response time).

For each subsequent request, the response is diffed against the baseline:
- Status code change → interesting
- Body length delta > 50 bytes → interesting
- Significant timing anomaly (2× the baseline response time) → interesting
- Body hash change with same status and similar length → potentially interesting

"Interesting" responses get flagged in the output. The `--interesting-only` flag
suppresses the noise and shows only flagged results live, while still writing
every result to the JSON report.

The full report is written to `recon/fuzz-<timestamp>.json`. Every request,
every response, every diff result — the full record of the fuzz run.

### Running It

```bash
mg-fuzz target-bounty \
    --template request.txt \
    --payloads sqli \
    --mode sniper \
    --rate-ms 250 \
    --interesting-only
```

---

## mg-replay: Verifying Before You Submit

Bug bounty programs occasionally reject findings where the submitter can't
demonstrate the vulnerability is still present at submission time. Findings
get patched between discovery and submission. `mg-replay` exists to close that gap.

Every finding markdown file has an Evidence section with curl commands:

```markdown
## Evidence

```bash
curl -s -I https://api.target.example.com/ | grep -i hsts
```
```

`mg-replay` extracts those curl commands, parses them into structured requests
(method, URL, headers, body, flags like `-k` for insecure TLS), replays each
one, and compares the response against the original.

If the finding's frontmatter includes the original response status and body hash:

```yaml
original_status: 200
original_body_hash: "a3f9b2..."
```

The replay is exact: same status + matching body hash = `still_vulnerable`.
Different status or body = `appears_fixed`. One of those, not the other = `indeterminate`.

Without the frontmatter baseline, the verdict is heuristic — a 200 with a body
containing the injection marker might be `still_vulnerable`, a 403 might be `appears_fixed`.

The result is written to `findings/<id>-replay-<date>.json`. Run it before
you submit, attach the replay JSON as additional evidence.

```bash
mg-replay target-bounty 2026-05-08-001
```

---

## ai-prioritize: Ranked Attack Surface

At this point in an engagement you might have 47 subdomains, hundreds of endpoints
from the crawler, a fingerprint file showing Node.js, nginx, and a Java microservice,
a probe report with 12 findings already, and an open question: *what do I look at next?*

`ai-prioritize` answers that question.

It reads `recon/summary.json` — the complete host picture — alongside a set of
bug-hunting skill files. The skills are markdown documents, each focused on one
vulnerability class: SQL injection, SSRF, authentication bypasses, insecure direct
object references, GraphQL misconfigurations, and so on. Each skill describes
what to look for, what tech stacks are commonly vulnerable, what payloads to try,
and roughly what payout to expect.

That package — recon summary + skill files — goes to an LLM:

```bash
ANTHROPIC_API_KEY=sk-ant-... ai-prioritize target-bounty
```

The model returns a ranked table of attack surface, highest-priority targets first,
with reasoning. It considers the intersection of what's running (tech stack,
framework, exposed ports) with what the skills say about that tech's historical
vulnerabilities.

For example: if `mg-fingerprint` identified Spring Boot and `mg-probe` found
`/actuator/env` returning 200, the AI knows Spring Boot Actuator exposure has
a well-documented CVE history and that `/actuator/env` specifically has returned
plaintext database credentials in past disclosures. That surfaces to the top.

The output is appended to `recon/priorities.md` with a timestamp, so multiple
runs over the course of an engagement accumulate in one document. Each run checks
whether `summary.json` has been updated since the last run — if nothing has changed
in the past 24 hours, it skips the LLM call.

**Ollama fallback:** If `ANTHROPIC_API_KEY` isn't set, the tool uses a local
Ollama instance. The quality difference is significant for this task — local models
are less good at reasoning about obscure CVE history and attack surface intersections.
But it works without an API key for local testing.

---

## The AI Operator Pattern

There's a broader design philosophy in how `ai-prioritize` works that's worth naming.

The tool doesn't try to do AI reasoning itself. It doesn't have a hardcoded list
of "if framework X then check Y." Instead, it assembles structured data and delegates
reasoning to a model that has the breadth of security knowledge to connect the dots.

The model's knowledge of what vulnerabilities historically affect what frameworks —
the kind of knowledge you'd get from years of reading CVEs, security advisories,
and bug bounty writeups — is what makes the prioritization useful.
The tool's job is to present the right data in the right format.

This pattern extends to the file-native design of the whole toolchain.
Every tool writes structured output that a human or an AI can read directly.
The audit log, findings, priorities — all of it is plain text and JSON.
Nothing requires a custom API or special client to consume.

When I run a full engagement and want Claude to help me dig into specific findings,
I don't need to build a bridge between the toolchain and the AI.
I just read the files.

---

## Putting It Together

The full pipeline, end to end:

```bash
# Initialize
mg-engagement init target-bounty \
    --target target.example.com --platform hackerone
mg-engagement scope-add target-bounty "*.target.example.com"

# Recon
mg-recon target-bounty

# Crawl
mg-crawl target-bounty https://www.target.example.com

# Probe security posture
mg-probe target-bounty

# Get AI-ranked priorities
ai-prioritize target-bounty

# Fuzz a specific endpoint
mg-fuzz target-bounty \
    --template api-search.txt \
    --payloads sqli xss \
    --mode cluster-bomb

# Verify a finding before submission
mg-replay target-bounty 2026-05-08-003
```

That's a complete pipeline from zero knowledge of a target to a list of verified,
ready-to-submit findings. Each tool is independent enough to run standalone
but composable enough to chain together through the shared engagement directory.

*Part 5 will cover `mg-tui` — the Ratatui terminal dashboard that gives you a
live view of all engagements, hosts, findings, and fuzz results without leaving
the terminal.*
