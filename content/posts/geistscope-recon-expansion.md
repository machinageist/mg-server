---
title: "GeistScope: Recon Expansion"
date: 2026-05-17
summary: "mg-whois, mg-shodan, mg-dns-enum, mg-dns-history, mg-cloud-enum, mg-cname-chain, and mg-udp-scan extend the recon pipeline into DNS internals, historical data, and cloud storage exposure."
tags: [rust, security, bug-bounty, geistscope, recon, dns, cloud]
---

## Beyond the Initial Pipeline

The [core recon pipeline](/wiki/geistscope-recon-pipeline) covers subdomain enumeration,
port scanning, and tech stack fingerprinting. That gets you a map of what exists and
what it runs. Seven additional tools go deeper: who owns the infrastructure, what the
DNS history reveals, whether cloud storage buckets are publicly exposed, and what UDP
services are listening.

---

## mg-whois: Ownership and ASN Mapping

`mg-whois` does two things: standard WHOIS lookups and ASN enumeration.

```bash
mg-whois target-bounty target.example.com
```

WHOIS is implemented over raw TCP, not via a library. The tool opens a connection to
`whois.iana.org:43`, queries for the TLD referral, parses the `refer:` field to get
the authoritative WHOIS server, then opens a second TCP connection to that server and
sends the actual query. Two round trips, raw sockets, no dependency on system WHOIS
binaries.

ASN data comes from two sources: `ipinfo.io` for the ASN number and org name, and
`api.bgpview.io` for the BGP prefixes that ASN announces. This matters because a
target might own a /20 that contains services not reached through the domain name.
The prefix list goes into the engagement directory for reference during network-level
testing.

---

## mg-shodan: Host Intelligence Without Scanning

```bash
MG_SHODAN_KEY=<key> mg-shodan target-bounty target.example.com
```

`mg-shodan` supports two modes: host lookup and facet search. Host lookup resolves
the domain to an IP via `hickory-resolver`, then queries `api.shodan.io/shodan/host/<ip>`
for everything Shodan has collected: open ports, banners, detected CVEs, historical
data, SSL cert info. Facet search queries Shodan's indexed data by domain for broader
exposure analysis.

The key practical value here is Shodan's historical view. Ports that are closed today
may have been open when Shodan last scanned. Services that have been taken down still
appear in the data. Shodan findings go into `recon/shodan.json` and feed the summary.

A Shodan API key is required. Without it, the tool exits with a clear error rather
than silently producing partial results.

---

## mg-dns-enum: Zone Transfers, DNSSEC, and Service Records

`mg-dns-enum` attempts a zone transfer against each nameserver for the target domain,
then runs several additional DNS checks:

```bash
mg-dns-enum target-bounty target.example.com
```

Zone transfer (AXFR) is sent over raw TCP to the NS server on port 53. Most production
nameservers refuse zone transfers from unauthorized sources, but misconfigurations exist
and when they do, you get the entire zone in one response. Every hostname, every IP,
every record type.

Beyond AXFR, the tool checks wildcard DNS (a wildcard response changes how you interpret
subdomain enumeration results), DNSSEC validation status, SRV records (which reveal
internal service topology: XMPP servers, SIP infrastructure, custom protocols), and PTR
records via a limited reverse sweep of the /24 containing the target's primary IP.

---

## mg-dns-history: Stale Records and Origin IP Leaks

Historical DNS data is where you find IP addresses that bypass CDN protection.

```bash
mg-dns-history target-bounty target.example.com
```

If a SecurityTrails API key is set, the tool queries their historical DNS API. Without
a key, it falls back to HackerTarget's free API. Either way, the goal is the same:
collect IP addresses that the domain pointed to in the past.

After collecting historical IPs, `mg-dns-history` cross-references them against the
current `recon/summary.json`. IPs that appeared historically but are absent from the
current recon are flagged as potentially stale but worth investigating. The most
actionable case: a domain that now sits behind Cloudflare used to point directly at
an origin server. If that origin IP is still alive and accepting connections, the CDN
is bypassed.

---

## mg-cloud-enum: Public Storage Buckets

```bash
mg-cloud-enum target-bounty target.example.com
```

The tool generates candidate bucket names from the target domain name, including
variations: the base name, `<name>-prod`, `<name>-dev`, `<name>-backup`, `<name>-assets`,
and similar patterns. Each candidate is checked against S3, GCS, and Azure Blob Storage.

Three outcomes are possible for each candidate:

- `public_listing`: The bucket exists and its contents are listable. Written as a HIGH
  severity finding. Public bucket listings have paid out at critical severity in multiple
  programs when sensitive files were present.
- `exists_private`: The bucket exists but access is denied. Written as INFO. Still worth
  noting that the name is claimed.
- `not_found`: No bucket by that name.

The checks are HTTP requests to the bucket URL patterns — no cloud SDK required, no
credentials needed for the check itself.

---

## mg-cname-chain: Subdomain Takeover Prep

`mg-cname-chain` reads `recon/subdomain-enum.json` and resolves the full CNAME chain
for every subdomain found. This is groundwork for `mg-takeover`.

```bash
mg-cname-chain target-bounty
```

The tool flags four conditions: NXDOMAIN intermediates in a chain (a CNAME points to
a name that doesn't resolve, which is the core condition for subdomain takeover), external
CNAMEs (the chain terminates on a third-party domain like `github.io` or `herokudns.com`),
circular references, and chains longer than five hops. All of these go into
`recon/cname-chains.json` for consumption by `mg-takeover`.

---

## mg-udp-scan: Services the TCP Scanner Misses

TCP scanning finds web servers, SSH, databases. UDP scanning finds DNS servers, SNMP,
NTP, TFTP, SSDP, and a range of services that are invisible to TCP probes.

```bash
mg-udp-scan target-bounty --ports 53,67,69,111,123,161,500,514,1900
```

`mg-udp-scan` uses raw datagrams via `tokio::net::UdpSocket`. For each port, it sends
a service-appropriate probe packet and waits for a response. UDP doesn't have a
three-way handshake, so the definition of "open" is any non-timeout response. An ICMP
port unreachable means closed. No response means open|filtered.

Service fingerprinting checks the response bytes against known patterns. An SNMP
response has a specific BER structure. A DNS response starts with the query ID. NTP
responses have a recognizable header. Matches are noted in the output; unrecognized
responses are recorded as raw bytes.

UDP services are frequently forgotten in assessments. An exposed SNMP community string
can leak the entire network topology. An open DNS resolver can be abused. A TFTP
server might serve firmware. These don't show up in a TCP port scan.

---

## What Gets Written

Each tool writes its output to the engagement's `recon/` directory:
`whois.json`, `shodan.json`, `dns-enum.json`, `dns-history.json`, `cloud-enum.json`,
`cname-chains.json`, `udp-scan.json`. Findings that meet severity thresholds also
get written to `findings/` in the standard engagement format.

`mg-recon` doesn't orchestrate these tools automatically. They run after the core
pipeline when you want the expanded view. For active bug bounty work, running all of
them before moving to [crawling and probing](/wiki/geistscope-crawl-and-probe) means
the attack surface is as complete as passive and semi-passive methods can make it.
