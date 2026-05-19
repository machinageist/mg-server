---
title: "GeistScope Part 9: The Security Graph"
date: 2026-05-17
summary: "security-graph models relationships between hosts, URLs, parameters, identities, and findings in a local JSONL-backed store. It answers questions a flat findings list can't: which parameters share an identity, which finding chains from a host to a credential."
tags: [rust, security, bug-bounty, geistscope, graph, data-modeling]
---

## What a Flat List Can't Answer

After a full recon and fuzz run, you have a findings directory full of individual
markdown files. Each finding knows its target host and its severity. But the
pipeline also accumulates information that doesn't map to a single finding:

- A JWT found in a crawled response that's also referenced by a parameter in a
  different endpoint
- An identity (authenticated user) that shows up in both an IDOR finding and a
  mass-assignment finding on different hosts
- A replay chain that connects a login request to a token extraction to an
  admin API call

None of that structure lives in the flat `findings/` directory. `security-graph`
is where it goes.

---

## Node Kinds

The graph recognizes eleven node kinds, each representing a class of security-relevant
entity:

| Kind | What it represents |
|---|---|
| `host` | A resolved hostname or IP in scope |
| `url` | A specific URL discovered during crawl or fuzz |
| `parameter` | A named input — query param, body field, header, cookie |
| `identity` | An authenticated principal used during testing |
| `jwt` | A captured JSON Web Token |
| `session` | A session identifier or session cookie |
| `cookie` | A named HTTP cookie |
| `api` | An API endpoint or resource |
| `finding` | A link back to a finding in the engagement directory |
| `technology` | A tech stack element — framework, server, library |
| `replay_chain` | A sequence of requests that together demonstrate a vulnerability |

Nodes have a stable content-addressed ID: SHA-256 of `kind:label`, hex-encoded.
That means the same node (same host, same parameter name) gets the same ID across
runs, and inserting it twice is idempotent.

---

## Edges and Relationships

Edges connect nodes with a typed relationship. Some examples:

- `host → url` (contains)
- `url → parameter` (has_parameter)
- `parameter → finding` (triggers)
- `identity → jwt` (holds)
- `jwt → api` (authenticates)
- `replay_chain → finding` (demonstrates)

The file-backed adapter stores nodes and edges as JSONL — one JSON object per line,
appended on insert. The store keeps a hash set of seen node IDs in memory to skip
duplicate writes without reading the file back.

---

## Querying the Graph

The current store supports three read operations:

**Neighbors.** Given a node ID and an optional relationship filter, return the
nodes connected to it (up to a configurable limit, capped at 100):

```rust
let neighbors = store.neighbors(node_id, Some("triggers"), 50)?;
```

**Node lookup.** Given a node ID, return the full node record including all
metadata attached at insertion time.

**Node list by kind.** Return all nodes of a given kind — all findings, all
parameters, all identities. Useful for building a summary view.

The harness (Part 10) exposes these as endpoints so an AI operator can query the
graph via JSON without reading JSONL directly.

---

## Why JSONL Instead of a Graph Database

The engagement directory is already the source of truth. Findings, recon output,
replay evidence — it's all flat files. Introducing a graph database (Neo4j, DGraph)
would mean another process to manage, another dependency to install, and a store
that lives outside the engagement directory.

JSONL keeps everything in the same place. The graph file is human-readable, can be
committed alongside the engagement, and doesn't require any external process. The
tradeoff is that queries are O(n) in the number of edges — acceptable for single-
engagement scale where the graph has hundreds of nodes, not millions.

The `FileGraphStore` interface mirrors a future `GraphStore` trait, so swapping
in a Postgres-backed implementation later is a boundary change rather than a rewrite.

---

## How the Graph Gets Populated

Nodes and edges are inserted by the harness endpoints during tool dispatch.
When `mg-crawl` discovers a URL and the harness writes the result, it inserts
a `url` node connected to the `host` node. When `mg-fuzz` confirms a finding,
the harness inserts a `finding` node and edges from the relevant parameter and
URL. When `mg-recopilot` identifies an exploit primitive in decompiled code, that
goes in as a `finding` node connected to a `technology` node representing the
binary.

The graph accumulates during an engagement rather than being computed at the end.
By the time you're ready to write a report, the structure is already there.

---

*Part 10 covers `mg-harness`: the JSON dispatcher that gives AI operators
typed, risk-controlled access to every tool endpoint in the GeistScope pipeline.*
