---
title: "GeistScope: OSINT Tools"
date: 2026-05-17
summary: "mg-github, mg-breach, mg-social, mg-artifact-audit metadata, mg-google-dork, and mg-leak-monitor gather passive intelligence from public sources before any active testing begins."
tags: [rust, security, bug-bounty, geistscope, osint, recon]
---

## Passive Before Active

Before sending any attack payloads or probing application endpoints, there's a category
of work that's purely observational: reading what's already public. Source code
repositories, breach databases, document metadata, search engine indexes. None of this
touches the target's infrastructure in any meaningful way, and it frequently produces
the highest-value findings in a program.

Six capabilities cover this space in GeistScope; document metadata extraction now lives under `mg-artifact-audit metadata`.

---

## mg-github: Code Search for Secrets

```bash
MG_GITHUB_TOKEN=<token> mg-github target-bounty target.example.com
```

The tool uses GitHub's code search API to find references to the target domain in
public repositories. It searches for several categories: API keys and tokens,
internal domain names appearing in configuration files, hardcoded credentials in
deploy scripts, and connection strings.

Rate limits are handled precisely. The tool reads `X-RateLimit-Remaining` and
`X-RateLimit-Reset` from every response. When remaining hits zero, it sleeps until
the reset timestamp rather than failing or retrying blindly. Without a token, the
limit is 10 requests per minute. With a token, it's 30.

The search targets the organization's repos specifically when the GitHub org name
can be inferred from the target domain. Common patterns: an employee's fork of an
internal tool that includes a config file with real credentials, a public CI script
that references `MG_API_KEY`, a build artifact accidentally committed with a secrets
file still attached.

---

## mg-breach: HIBP Domain Lookup

```bash
MG_HIBP_KEY=<key> mg-breach target-bounty target.example.com
```

HaveIBeenPwned's v3 API requires an API key. `mg-breach` queries for all breaches
containing email addresses from the target domain, then fetches breach details for
any breach that included password data.

The rate limit is strict: 1 request per 1.5 seconds. The tool enforces this with
a sleep between each request rather than hoping the API is forgiving.

The output tells you which credential dumps contain target-domain email addresses
and passwords. This informs password spraying strategy, reveals credential reuse
patterns, and is directly relevant to any login endpoint discovered during crawling.
A company with employees in a major breach is a candidate for credential stuffing.

---

## mg-social: Employee Enumeration

```bash
mg-social target-bounty target.example.com
```

`mg-social` enumerates GitHub organization members when a GitHub org can be identified
for the target. From the member list, it generates email address candidates by combining
usernames and real names against the target domain's email format (first.last@domain,
flast@domain, firstl@domain).

For LinkedIn, the tool generates a dork URL and prints it to stdout. There's no API
for LinkedIn enumeration, so the tool does what it can: give you the query string
and let you run it manually. The employee list feeds into `mg-breach` analysis and
social engineering context.

A token is optional but raises the GitHub API rate limit. Without it, member
enumeration on large organizations will hit rate limits and produce partial results.

---

## mg-artifact-audit metadata: Document Metadata Extraction

```bash
mg-artifact-audit metadata target-bounty
```

`mg-artifact-audit metadata` reads the crawl corpus and downloads PDFs, Office documents (DOCX, XLSX,
PPTX), and JPEG images. From each file, it extracts metadata that developers and
organizations routinely forget to strip before publishing.

PDF and Office documents often embed the author's name, the software version used to
create them, internal file paths, and revision history. DOCX, XLSX, and PPTX files
are ZIP containers — the tool unzips them in memory and reads `docProps/core.xml`,
which contains author, company, and creation/modification timestamps.

JPEG EXIF data is found by scanning for magic bytes in the file stream. GPS coordinates
appear in EXIF when photos are taken on smartphones with location enabled. Camera make
and model are routinely present. Some organizations publish press photos taken at their
offices with GPS intact.

Internal file paths from Office documents are particularly useful: `C:\Users\jsmith\Documents\CompanyInternal\ProjectX\` tells you the username format, the operating system,
and sometimes internal project names that don't appear anywhere public-facing.

---

## mg-google-dork: Structured Search Engine Queries

```bash
mg-google-dork target-bounty target.example.com
```

The tool runs 14 built-in dork templates against the target domain. The templates
cover login pages (`inurl:login site:target.example.com`), exposed configuration
files (`filetype:env site:target.example.com`), API documentation
(`inurl:swagger site:target.example.com`), directory listings, backup files,
and several categories of juicy file types.

All 14 dork strings are printed to stdout every time, regardless of whether API
execution is configured. This is by design: copy-paste the list and run them manually
in a browser, or configure a Google Custom Search Engine key and CX ID to execute
them programmatically.

```bash
MG_GOOGLE_CSE_KEY=<key> MG_GOOGLE_CX=<cx> mg-google-dork target-bounty target.example.com
```

With the API keys set, results are written to `recon/dorks.json`. Without them,
the tool still produces the dork list, which is the primary output anyway.

---

## mg-leak-monitor: Continuous GitHub Monitoring

```bash
mg-leak-monitor target-bounty target.example.com
```

The other GitHub tool runs once and searches existing code. `mg-leak-monitor` is a
long-running process that polls the GitHub Search API for new commits from the target
organization that mention the target domain. It's designed to run in the background
during an engagement.

State is persisted to `recon/leak-monitor-state.json` across restarts. When the tool
starts, it loads the last-seen commit timestamp and only processes newer results.
Findings are appended to `findings/` as they arrive, following the standard engagement
finding format.

```bash
# Run in background, findings appear as they're detected
mg-leak-monitor target-bounty target.example.com &
```

The use case: a developer accidentally commits a secrets file to a public repo during
your engagement window. Without monitoring, you'd only catch it if it happened to fall
within your one-time `mg-github` query window. With monitoring running, the finding
appears within the next poll interval.

---

## Reading the Output

OSINT tools write to `recon/` for structured data and `findings/` for anything that
meets a severity threshold. The metadata and GitHub outputs are worth reviewing manually
before moving to active testing — internal usernames and path structures from documents
inform the wordlists and account targets used by [auth testing tools](/wiki/geistscope-auth-session).
Breach data informs what credentials to try against login endpoints discovered during
[crawling](/wiki/geistscope-crawl-and-probe).

Passive intelligence shapes where active testing focuses. Running these before the
first scanner request means the active phase is more targeted.
