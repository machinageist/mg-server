---
title: "Network applications: content delivery networks"
date: 2026-07-23
summary: "A foundational overview of content delivery networks and how distributed infrastructure supports web applications."
tags: [education, networking, applications, cdn, network-plus]
---

## Overview

A **content delivery network (CDN)** is a geographically distributed collection of
servers used to deliver content to users from infrastructure selected for their
location or network conditions. A CDN sits between an application's origin and many
of its users, turning content delivery into a distributed network function rather
than work performed by one server in one place.

CDNs are most visible when serving web pages, images, scripts, video, and software
downloads. They can also support Software as a Service (SaaS) applications and are
commonly integrated with cloud platforms.

## Why CDNs exist

A single origin server can be physically distant from its users and can become a
bottleneck or single point of failure. Distributing delivery infrastructure can:

- reduce the distance between users and content;
- reduce repeated work at the origin;
- spread demand across multiple systems and locations;
- keep content available when an individual delivery node is unhealthy; and
- place traffic management and security controls closer to incoming requests.

These benefits are not automatic. What a CDN caches, how it selects a delivery node,
and what happens when the origin or a node fails depend on the provider and
configuration.

## A simple request path

A simplified CDN-backed request can be read from broad to narrow:

1. A user requests an application's hostname.
2. The CDN's request-routing system directs the request toward an appropriate edge
   location or delivery server.
3. The delivery server returns a cached object when it has a usable copy.
4. Otherwise, it retrieves the object from an upstream cache or the origin server.
5. Cache and application policy determine whether that response can be stored for a
   later request.

The **origin** remains the authoritative source for application content. An edge
server is a strategically placed delivery point, not automatically the source of
truth.

## CDN and application boundaries

A CDN is network infrastructure serving an application; it is not the application
itself. Static files are usually easier to cache than personalized or rapidly
changing responses. Dynamic applications may still use a CDN for routing, TLS
termination, static assets, traffic filtering, or selected cacheable responses while
application logic runs elsewhere.

This distinction matters when troubleshooting. A failure may originate in DNS, the
CDN edge, the connection from the edge to the origin, or the application behind the
origin.

## Suggested practice: identify the visible delivery path

Choose a site you own or have permission to inspect, then:

1. Resolve its hostname and record the returned addresses.
2. Request only the response headers and look for cache, age, server, or edge-related
   fields.
3. Repeat the request and compare the headers.
4. Draw the path you can support with evidence, marking any provider-internal steps
   as unknown rather than guessed.

This is an observation exercise, not proof of a provider's private architecture.

## Related pages

- [The OSI model](/learn/osi-model) — a functional map for following application data
  through transport, routing, local links, and physical media.
- [Network appliances](/learn/network-appliances) — load balancers, proxies, firewalls,
  and other roles that may participate in a delivery path.
- [Network functions](/learn/network-functions) — traffic handling, tunneling, quality
  of service, and packet lifetime.
- [How machinageist.dev is hosted](/blog/hosting-machinageist-dev) — documented
  evidence from this site's request path.

## Sources and further reading

This page was edited from my networking reading notes and checked against:

- [RFC 3466: A Model for Content Internetworking](https://www.rfc-editor.org/rfc/rfc3466.txt)
  — terminology and functional model for content networks.

Vendor documentation is still necessary for the exact caching, routing, security,
and failure behavior of a particular CDN.
