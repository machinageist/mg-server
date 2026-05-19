---
title: "GeistScope: Workflow, Post-Access, and Orchestration"
date: 2026-05-18
summary: "mg-screenshot, mg-takeover, mg-vhost, mg-diff, mg-notify, mg-timeline, mg-nuclei-bridge, mg-privesc-linux, mg-privesc-windows, mg-loot, and mg-dns-rebind are the tools that sharpen the overall engagement workflow."
tags: [rust, security, bug-bounty, geistscope, workflow, post-exploitation, orchestration]
---

## Tools That Don't Fit One Category

Some tools in a security toolchain don't map cleanly to a single vulnerability class.
Screenshots organize visual reconnaissance. Subdomain takeover detection spans recon
and exploitation. Diff analysis shows what changed between engagement sessions. These
tools make the workflow sharper without belonging to any single testing phase.

Eleven tools fall into this category.

---

## mg-screenshot: Visual Inventory

```bash
mg-screenshot target-bounty
```

`mg-screenshot` reads HTTP and HTTPS hosts from `recon/summary.json` and visits each
one with a headless Chromium instance via `chromiumoxide 0.7`. Chromium must be on
`PATH` or specified with `--chromium`.

Each host produces a PNG saved to `screenshots/<hostname>.png`. After all hosts are
captured, the tool writes `screenshots/index.html`: an HTML page with each hostname,
the screenshot as an embedded image, and the HTTP status code from the visit.

The practical use: after running the full recon pipeline against a large program with
many subdomains, reviewing screenshots manually takes minutes instead of visiting
each host individually. Parked domains, default server pages, and interesting-looking
applications are immediately visible. Interesting ones get flagged for crawling and
deeper testing.

---

## mg-takeover: Subdomain Takeover Detection

```bash
mg-takeover target-bounty
```

Subdomain takeover happens when a DNS record points to a third-party service that
no longer has the target's account registered. An attacker registers an account on
that service and claims the subdomain.

The tool reads `recon/subdomain-enum.json` and the CNAME chain data produced by
[`mg-cname-chain`](/wiki/geistscope-recon-expansion). For each subdomain with an
external CNAME, it matches the terminal CNAME against a catalog of known-vulnerable
service fingerprints: GitHub Pages (`github.io`), Heroku (`herokudns.com`), Fastly,
Netlify, Surge, Cargo, and others.

Two confidence levels: if the CNAME matches a known-vulnerable fingerprint AND the
HTTP response body contains the service's "unclaimed" page text (e.g., "There isn't
a GitHub Pages site here"), the finding is CONFIRMED HIGH. If the CNAME matches
but the HTTP check fails or returns ambiguously, the finding is `potential` — worth
manual review.

---

## mg-vhost: Virtual Host Discovery

```bash
mg-vhost target-bounty
```

Multiple virtual hosts may run on the same IP. The `Host` header controls which
application the server routes to. VHost discovery sends requests with different `Host`
values and compares responses to the baseline.

The baseline is a request to the bare IP address. Each wordlist entry is tried as a
`Host` header value. Responses that differ in status code or body length from the
baseline are flagged. A wordlist of common subdomain prefixes is built in; `--wordlist`
specifies a custom list.

The tool also tests host header injection: sending `attacker.example.com` as the
`Host` value and checking whether the server reflects it in a redirect `Location`,
a `<base>` tag, or a password reset link. Host header injection in password reset
flows is a consistently paid bug class.

---

## mg-diff: Recon Change Detection

```bash
mg-diff target-bounty
```

Running `mg-recon` at the start of an engagement and again a week later produces two
`summary.json` files. `mg-diff` compares them.

Without arguments, it auto-selects the two most recent `recon/summary.json` files
in the engagement directory. With arguments, any two summary files can be compared:

```bash
mg-diff target-bounty --a recon/summary-2026-05-01.json --b recon/summary-2026-05-15.json
```

Output covers four change categories: new hosts (appeared in the second scan but not
the first), removed hosts, port changes (new ports open or previously-open ports now
closed), and tech stack changes (fingerprint changed between scans).

New hosts during an engagement are high-priority targets. Infrastructure changes during
active testing sometimes expose services that were behind a firewall when the engagement
started.

---

## mg-notify: Real-Time Finding Alerts

```bash
MG_SLACK_WEBHOOK=<url> mg-notify target-bounty
```

`mg-notify` watches the `findings/` directory for new files and pushes notifications
to configured channels. Supported destinations: Slack webhook, Discord webhook, and
ntfy.sh topic.

State is persisted to `notify/state.json` so the watcher can restart without re-sending
previously seen findings. The tool handles SIGINT and writes final state before exiting.

```bash
# Keep running while testing, get Slack alerts for each new finding
mg-notify target-bounty &

# Run injection tests in another terminal
mg-sqli target-bounty
mg-xss target-bounty
```

High and critical findings produce an alert with the title, severity, affected URL,
and a note that the full finding is in `findings/`. Informational findings are
suppressed by default; `--all-severities` includes them.

---

## mg-timeline: Engagement Chronology

```bash
mg-timeline target-bounty
```

After an engagement, understanding what happened and when matters for both reporting
and debugging. `mg-timeline` walks the entire engagement workspace and builds a
sorted chronological view.

Two time sources are used: filesystem `mtime` for tool output files, and the
`"timestamp"` JSON field in finding files and tool output JSON. The JSON timestamp
is preferred when available.

The output is a sorted list of events: `[2026-05-15T14:32:11] mg-recon completed:
47 hosts discovered`, `[2026-05-15T16:44:02] FINDING HIGH: CORS misconfiguration
on api.target.example.com`. This reconstructs the sequence of tool runs and findings
for after-action review or report writing.

---

## mg-nuclei-bridge: Nuclei Template Integration

```bash
mg-nuclei-bridge target-bounty --templates /path/to/nuclei-templates/
```

Nuclei is an external scanner with an extensive template library. `mg-nuclei-bridge`
runs the `nuclei` binary against in-scope hosts, streams its JSONL stdout, and
normalizes each finding into the engagement's `findings/` directory using the standard
GeistScope finding format.

Each nuclei finding gets a deduplicated engagement finding ID based on the template ID,
host, and matched URL. Running the bridge a second time with the same templates doesn't
produce duplicate findings for already-seen results.

The bridge adds Nuclei's coverage to the engagement without coupling the toolchain to
Nuclei's output format. All downstream tools (`mg-timeline`, `mg-notify`, `ai-prioritize`)
see Nuclei findings the same way they see findings from any other GeistScope tool.

---

## mg-privesc-linux: Local Privilege Escalation Checks

```bash
mg-privesc-linux
```

This tool runs locally on a target Linux system after gaining access. It has no
engagement dependency and makes no network requests.

Checks cover the most common local escalation paths: SUID and SGID binaries (`find /
-perm -4000`), directories in `PATH` that are writable by the current user, `sudo -l`
output parsed for NOPASSWD entries, cron jobs owned or writable by non-root users,
world-writable files in sensitive directories, and the kernel version for reference
against known local privilege escalation CVEs.

Output is printed to stdout. There's no engagement integration because this tool runs
on the compromised host, not the operator's machine.

---

## mg-privesc-windows: Windows Privilege Escalation

```bash
mg-privesc-windows
```

The Windows equivalent, gated with `cfg(target_os = "windows")` so it only compiles
for Windows targets. Checks: unquoted service executable paths (a path with spaces
and no quotes that traverses a writable directory), writable service binary paths,
`AlwaysInstallElevated` registry key (allows MSI packages to install with SYSTEM
privileges), scheduled tasks running as higher-privileged users, and enabled token
privileges (`SeImpersonatePrivilege`, `SeAssignPrimaryTokenPrivilege`) that enable
potato-family exploits.

---

## mg-loot: Credential Harvesting

```bash
mg-loot
```

Post-access credential collection. `mg-loot` scans common locations on the current
system for credentials and secrets: `~/.ssh/` for private keys and known_hosts,
`~/.aws/credentials` and `~/.aws/config`, `~/.config/` for application configs that
embed tokens, `.env` files in common web application paths (`/var/www/`, `/srv/`),
shell history files (`.bash_history`, `.zsh_history`), and common application config
paths for databases and services.

All secret values are masked to the first 8 characters in output. The full paths and
key names are reported so the operator knows where to look.

---

## mg-dns-rebind: DNS Rebinding Payload Generation

```bash
mg-dns-rebind target-bounty target.example.com
```

DNS rebinding bypasses same-origin policy by having a domain resolve to the attacker's
IP first (for the initial page load) and then resolve to the victim's internal IP
(for subsequent requests). The victim's browser, believing it's talking to the original
domain, allows the cross-origin request.

The tool resolves the target's current TTL, checks whether the target IP is in private
RFC 1918 space (a rebinding attack targeting an internal service), and writes a ready-to-use
HTML payload that uses a cooperating DNS rebinding service to perform the resolution
switch. No active probing beyond the DNS resolution itself.

---

## The Engagement as a Unit

These tools reinforce a design principle that runs through GeistScope: the engagement
directory is the single source of truth. `mg-diff` compares two snapshots of it.
`mg-notify` watches it. `mg-timeline` walks it. `mg-nuclei-bridge` writes into it.

Every tool that reads from or writes to `engagements/<name>/` is interoperable with
every other tool that does the same. Adding Nuclei coverage doesn't require changing
any other tool. Enabling Slack notifications doesn't require modifying the scanner.
The directory structure is the integration layer.
