---
title: "Network applications: content delivery networks"
date: 2026-07-23
summary: "A foundational overview of content delivery networks and how distributed infrastructure supports web applications."
tags: [education, networking, applications, cdn]
---

## Overview

A **content delivery network (CDN)** is a group of servers in different
locations that deliver content from a location suited to the user and current
network conditions. It sits between an application's origin and many of its
users. Instead of making one server handle every request, it distributes the
delivery work.

CDNs commonly serve web pages, images, scripts, video, and software downloads.
They also support Software as a Service (SaaS) applications and often integrate
with cloud platforms.

## Why CDNs exist

One origin server may be far from its users. It can also become a bottleneck or
a single point of failure. A CDN can:

- shorten the network path between users and content;
- reduce repeated work at the origin;
- spread demand across systems and locations;
- keep serving content when one delivery node is unhealthy; and
- apply traffic management and security controls closer to incoming requests.

The result depends on the configuration: what the CDN caches, how it chooses a
node, and what it does when the origin or a node fails.

## A simple request path

A basic CDN-backed request follows this path:

1. A user requests an application's hostname.
2. The CDN's routing system sends the request to an appropriate edge location
   or delivery server.
3. The delivery server returns the object if it has a usable cached copy.
4. If not, it gets the object from an upstream cache or the origin server.
5. Cache and application policy decide whether to store the response for later
   requests.

The **origin** remains the authoritative source for application content. An
edge server is a delivery point, not necessarily the source of truth.

## CDN and application boundaries

A CDN serves an application; it is not the application itself. Static files are
usually easier to cache than personalized or frequently changing responses. A
dynamic application can still use a CDN for routing, TLS termination, static
assets, traffic filtering, and selected cacheable responses while its
application logic runs elsewhere.

That boundary is useful when troubleshooting. A request can fail at DNS, the
CDN edge, the edge-to-origin connection, or the application behind the origin.

## Suggested practice: identify the visible delivery path

Choose a site you own or have permission to inspect, then:

1. Resolve its hostname and record the returned addresses.
2. Request only the response headers and look for cache, age, server, or
   edge-related fields.
3. Repeat the request and compare the headers.
4. Draw only the path the evidence supports. Mark provider-internal steps as
   unknown instead of guessing.

This shows the path visible from outside. It does not reveal a provider's
private architecture.

## Related pages

- [The OSI model](/learn/osi-model) — a functional map for following application
  data through transport, routing, local links, and physical media.
- [Network appliances](/learn/network-appliances) — load balancers, proxies,
  firewalls, and other roles that may participate in a delivery path.
- [Network functions](/learn/network-functions) — traffic handling, tunneling,
  quality of service, and packet lifetime.
- [How machinageist.dev is hosted](/blog/hosting-machinageist-dev) — documented
  evidence from this site's request path.

## Sources and further reading

This page was edited from my own study notes, taken from Ian Neil's CompTIA
Network+ certification guide, and checked against the primary sources:

- [RFC 3466: A Model for Content Internetworking](https://www.rfc-editor.org/rfc/rfc3466.txt)
  — terminology and functional model for content networks.

Check the CDN vendor's documentation for its exact caching, routing, security,
and failure behavior.
