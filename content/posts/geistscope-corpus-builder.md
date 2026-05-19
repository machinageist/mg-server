---
title: "GeistScope Part 7: Mining the Historical Attack Surface"
date: 2026-05-13
summary: "corpus-builder queries certificate transparency logs and the Wayback Machine CDX API to build a persistent SQLite corpus of subdomains and paths — passive intelligence gathered before any active probing."
tags: [rust, security, bug-bounty, geistscope, recon, osint]
---

## The Surface That Already Exists

Before you run a port scan or send a single HTTP request to a target, a significant
portion of the attack surface is already documented in two public archives: CT logs
and the Wayback Machine.

Certificate transparency logs record every TLS certificate ever issued, including
all the Subject Alternative Names. If `api.internal.target.com` ever had a cert,
it's in crt.sh. The Wayback Machine's CDX API exposes every URL it crawled for a
domain — paths, query parameters, and endpoints that may no longer be in the sitemap
but still respond to requests.

`corpus-builder` queries both sources and stores the results in a SQLite database
you can query across engagements.

---

## Two Sources, One Store

**CT log mining.** The `ct` module queries crt.sh's JSON API for each domain in
your list:

```
https://crt.sh/?q=%.target.com&output=json
```

Each entry contains a `name_value` field, which may be a single hostname or a
newline-separated block of SANs. The miner splits those blocks, normalizes to
lowercase, strips wildcards, and only keeps entries that are proper subdomains
of the queried domain — not sibling domains that happened to share a cert.

```bash
corpus-builder mine-ct \
    --domains targets.txt \
    --db corpus.db \
    --rate-limit-ms 500
```

**Wayback CDX mining.** The Wayback Machine's CDX API returns every archived URL
matching a wildcard. The `wayback` module queries:

```
https://web.archive.org/cdx/search/cdx?url=*.target.com/*
    &output=json&fl=original&collapse=urlkey
    &filter=statuscode:200&limit=50000
```

The `collapse=urlkey` parameter deduplicates by URL structure rather than literal
URL, so you don't get 10,000 variations of the same paginated endpoint. The
`filter=statuscode:200` restriction keeps the path list relevant — 404s from ten
years ago aren't interesting.

Extracted paths go into the corpus store keyed by domain. A path like
`/api/v1/users/export` found in Wayback history for `api.target.com` will show
up when you query that host.

```bash
corpus-builder mine-wayback \
    --domains targets.txt \
    --db corpus.db \
    --rate-limit-ms 2000
```

The Wayback CDX endpoint is slower and rate-limited more aggressively than crt.sh,
hence the higher default interval.

---

## The SQLite Store

Both sources write into the same `Corpus` struct backed by a local SQLite file.
The schema tracks three things: domain roots, subdomains with their discovery source
(`ct_log` or `wayback`), and paths keyed by domain.

```bash
corpus-builder stats --db corpus.db
# Corpus: 3 domains, 247 subdomains, 8,412 paths

corpus-builder query --domain target.com --db corpus.db
# Subdomains for target.com (247 total):
#   api.target.com
#   admin.target.com
#   ...
```

The database is separate from any individual engagement — it's program-level
intelligence that accumulates across sessions. When you start a new engagement
against a target you've seen before, the corpus already has years of historical
surface data. When subdomain enumeration finds something new, it gets compared
against what Wayback already knew about.

---

## What This Catches

Bug bounty programs have long-lived targets. An admin panel that was accessible
three years ago and then moved behind a VPN still shows up in Wayback. An API
version path (`/api/v2/`) that replaced an older one (`/api/v1/`) often leaves
the old routes live even after the documentation removes them.

CT logs catch infrastructure changes: a subdomain that was briefly on a different
IP, a staging environment that had a cert issued for it, a deployment that used
a different domain before the final name was registered.

Neither source requires sending a packet to the target. The corpus is built purely
from passive public records — which matters both for program rules and for not
alerting a WAF before you've decided what to test.

---

*Part 8 covers `session` and `payload-engine`: how authenticated testing is set up
without storing credentials in plaintext, and how payload selection adapts to
what the fingerprinter found about the target stack.*
