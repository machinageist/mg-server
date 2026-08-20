---
title: "What I Changed After Reviewing This Site's Security"
date: 2026-08-20
summary: "A review found a path-boundary bug, a shared rate-limit bucket, exposed process telemetry, and too much cumulative infrastructure detail. Here is what changed."
category: "Security"
tags: [security, defensive, rust, axum, opsec, owned-scope]
---

I reviewed this site as both an application and a piece of public writing. The
code matters, but so does the story the site tells about the system behind it.
A site can avoid leaking a password and still publish a very convenient map.

The review found no committed secrets. The dependency advisory scan was clean at
the time of the review, and the site already had a strict header baseline,
escaped query output, bounded search input, and content tests that reject private
addresses and internal machine identifiers.

It still had real problems.

## A route slug could cross its content directory

Some study routes decoded a slug and passed it into a filesystem join. A forced
Markdown extension and sanitized error pages kept the result from becoming a
straight arbitrary-file download, but the boundary was wrong: a path-like value
could reach Markdown outside the intended directory.

I moved validation into the content models, before any filesystem join. Every
Markdown-backed loader now uses the same rule: a slug is one non-empty lowercase
ASCII component containing letters, digits, and hyphens. Tests cover the blog,
wiki pages, quizzes, and performance scenarios, including encoded path cases at
the router boundary.

The important part is where the check lives. A handler check protects one route.
A loader check protects every current and future caller.

## The rate limiter let one visitor throttle everyone

The old token bucket was global. Sixty quick requests from one client could
empty it for the whole process, so the next unrelated visitor would get a 429.
That is a denial-of-service mechanism disguised as protection.

The limiter is now keyed by the client address supplied by the trusted edge
path. Exhausting one bucket does not spend another visitor's quota. Missing or
malformed address metadata falls into one shared bucket instead of creating a
new key.

That comes with a deployment condition: the origin must remain behind the
trusted local proxy and connector. If the origin becomes directly reachable,
forwarded client-address headers are not trustworthy. Application throttling is
also not a replacement for edge controls against large floods.

## The status page said too much

The old footer and status endpoint published uptime, request totals, resident
memory, build time, application version, and listener classification. None of
those values was a secret. Together they made restarts, deployments, and rough
process behavior easy to track.

I removed the site-wide vitals strip. The human and JSON status endpoints now say
only that the application is available. Private monitoring can keep the useful
numbers without publishing them to every visitor.

## CSP got three smaller locks

The existing Content Security Policy already restricted scripts, styles,
images, fonts, connections, and framing. I added three explicit directives:

- `base-uri 'none'` prevents injected markup from changing how relative URLs are
  resolved.
- `form-action 'self'` prevents forms from posting to another origin.
- `object-src 'none'` disables plugin and object content the site does not use.

This is defense in depth. CSP can reduce the damage from some injected markup;
it cannot make an injection bug safe. Repository-controlled Markdown can still
contain raw HTML, so the content repository remains a trusted-author boundary.

## The public writing needed an OPSEC edit

This was the uncomfortable part. The site did not publish private addresses,
hostnames, credentials, or machine IDs. It did publish enough smaller facts to
assemble a useful operational picture: named components, management
dependencies, current gaps, recovery mechanics, and planned trust boundaries.

I kept the parts that teach something and removed the parts that mostly help
reconnaissance. The hosting article still explains an edge proxy, outbound
connector, reverse proxy, and Rust origin. It no longer includes exact service
identifiers, copied DNS answers, or a public checklist of missing controls. The
network incident still covers stale dependencies, quorum, DNS, and bottom-up
verification. It no longer describes the live cluster shape or a product-specific
recovery sequence.

The Labs section now calls itself what it is publicly: sanitized design notes and
verification patterns. It is not a mirror of the runbooks used to operate the
network.

That distinction is the policy going forward. Architecture classes, failure
analysis, and reproducible public checks are useful portfolio evidence. Current
control gaps, recovery paths, maintenance timing, and internal identifiers stay
private.

## Keeping the review alive

The regression tests now cover path boundaries, per-client quotas, CSP
directives, coarse status output, and a short list of high-value copy disclosures.
CI also runs dependency-advisory and secret scans, and its third-party actions are
pinned to full commits.

This was a bounded review of an owned personal site, not a penetration test. It
did not verify the Cloudflare account, reverse proxy, service sandbox, host
firewall, switch, hypervisor, or backups because those configurations are not in
this repository. The accurate claim is that I found and fixed specific issues,
added tests for them, and reduced unnecessary public exposure. That is narrower
than calling the site secure, and it is much easier to defend.
