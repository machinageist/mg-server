---
title: "GeistScope: API Surface Testing"
date: 2026-05-18
summary: "mg-graphql, mg-openapi, mg-grpc, mg-websocket, mg-cors-exploit, mg-cache-poison, mg-proto-pollute, and mg-deser cover modern API attack surfaces beyond standard HTTP parameter fuzzing."
tags: [rust, security, bug-bounty, geistscope, api, graphql, grpc, websocket]
---

## APIs Aren't Standard HTTP

Modern applications expose attack surface that doesn't look like a form with GET
parameters. GraphQL endpoints accept JSON queries with arbitrary depth. gRPC services
use binary framing over HTTP/2. WebSocket handshakes have their own CORS model.
Cache poisoning depends on header handling at the CDN layer, not application logic.

Standard fuzzing tools built for form fields and query strings miss all of this.
Eight tools cover the API surface specifically.

---

## mg-graphql: Introspection and Schema Analysis

```bash
mg-graphql target-bounty
```

The tool starts by detecting GraphQL endpoints from the crawl corpus and common paths
(`/graphql`, `/api/graphql`, `/query`). For each candidate, it sends an introspection
query:

```graphql
{ __schema { queryType { name } types { name fields { name } } } }
```

A successful introspection response means the schema is fully enumerable. The tool
extracts all types and fields, then flags dangerous mutations by name pattern:
`deleteUser`, `resetPassword`, `createAdmin`, `updatePermissions`. These are HIGH
candidates for testing.

Two additional checks run against the schema: **query batching** (sending an array
of query objects in a single request, which can bypass per-query rate limits) and
**depth limit absence** (sending a deeply nested query like `{ user { friends { friends
{ friends { ... } } } } }` to check whether the server enforces query depth limits,
which mitigates DoS via complex queries).

If introspection is disabled, the tool falls back to field guessing from common type
names.

---

## mg-openapi: Spec-Driven Endpoint Testing

```bash
mg-openapi target-bounty --spec https://target.example.com/openapi.json
```

The tool parses JSON OpenAPI specs and builds minimal valid requests for each endpoint
defined in the spec. For each operation, it constructs a request with placeholder
values: numeric parameters get `1`, string parameters get `"test"`, booleans get `true`.

The `--unauthenticated` flag replays the same set of requests without auth headers.
Endpoints that return 200 without authentication when they should require it are
flagged as BROKEN_ACCESS_CONTROL findings.

Required parameters are included; optional parameters are omitted unless they suggest
interesting behavior by name. The tool doesn't attempt to find the spec automatically
beyond common paths — provide the spec URL directly.

---

## mg-grpc: Service Reflection and Unauthenticated Calls

```bash
mg-grpc target-bounty --host grpc.target.example.com:443
```

gRPC uses HTTP/2 with a specific framing format: a 1-byte compression flag followed
by a 4-byte big-endian message length, then the protobuf message body. The tool
constructs these frames manually using `reqwest` with `http2_prior_knowledge` for
cleartext or standard TLS ALPN negotiation for encrypted transport.

Server reflection is attempted first. If the server implements the gRPC reflection
service, the tool enumerates all services and methods. For each method discovered,
it sends an empty request body (valid protobuf: no fields set) and records the response.
Methods that return data instead of `UNIMPLEMENTED` or `UNAUTHENTICATED` are flagged
as potentially accessible without credentials.

Without reflection support, method names must be provided manually with `--service`
and `--method`.

---

## mg-websocket: CSWSH and Payload Fuzzing

```bash
mg-websocket target-bounty
```

WebSocket connection handling has its own CORS model. Unlike XHR and fetch, browsers
don't enforce CORS preflight for WebSocket upgrade requests — the server is responsible
for validating the `Origin` header.

`mg-websocket` attempts a WebSocket handshake with `Origin: https://attacker.example.com`
using `tokio-tungstenite`. A `101 Switching Protocols` response with that Origin means
Cross-Site WebSocket Hijacking is possible — an attacker can read WebSocket messages
from a victim's session by loading a malicious page.

Optional fuzz mode sends common payloads after a successful handshake:

```bash
mg-websocket target-bounty --fuzz
```

Payloads include XSS strings, SQL injection, JSON with special characters, and oversized
messages. Response content and any server-side errors are recorded.

---

## mg-cors-exploit: Origin Reflection Testing

```bash
mg-cors-exploit target-bounty
```

Four Origin header values are tested against every HTTP endpoint: a random subdomain
of the target domain, `https://attacker.example.com`, the `null` origin (sent by
sandboxed iframes), and a pre-domain variant (`https://attacker.target.example.com`).

The exploitable condition requires both headers in the response:
`Access-Control-Allow-Origin` reflecting the injected origin AND
`Access-Control-Allow-Credentials: true`. Without credentials, CORS reflection is
informational. With credentials, an attacker can make authenticated requests from a
foreign origin and read the response — reading session data, user info, or any
authenticated API response.

This shares logic with `mg-probe`'s CORS check but runs a broader origin test matrix
against the full endpoint list rather than just the root path.

---

## mg-cache-poison: Unkeyed Header Injection

```bash
mg-cache-poison target-bounty
```

Cache poisoning requires a cache to be present. The tool first confirms this by looking
for cache indicator headers in responses: `Age`, `X-Cache`, `CF-Cache-Status`, `Via`.
Endpoints without these headers are skipped.

The attack pattern is two requests. First: inject a value into an unkeyed header
(`X-Forwarded-Host`, `X-Forwarded-For`, `X-Original-URL`) and request the page.
Second: request the same page without the injected header. If the second response
reflects the injected value, the response was served from cache with the poisoned
content.

The most impactful finding is `X-Forwarded-Host` reflected in a `Location` redirect
or in an absolute URL in the response body — this allows serving the cached response
to other users with attacker-controlled URLs embedded.

---

## mg-proto-pollute: Prototype Pollution

```bash
mg-proto-pollute target-bounty
```

Prototype pollution targets JavaScript applications that merge user-controlled objects
into application state without sanitizing `__proto__` or `constructor` keys.

For POST endpoints, the tool injects `{"__proto__":{"polluted":true}}` into the
request body. For GET endpoints, it adds `?__proto__[polluted]=true` to the URL.

Detection is behavioral: a 500 response where there was a 200, or a response body
change, suggests the pollution caused an error in the application. Definitive
confirmation requires controlled execution context, which the tool flags as requiring
manual verification.

---

## mg-deser: Deserialization Artifact Detection

```bash
mg-deser target-bounty
```

`mg-deser` scans the crawl corpus for serialized object artifacts, then tests injectable
endpoints.

**Java**: magic bytes `0xaced 0x0005` in any crawled response indicate Java
deserialization. This triggers manual review — Java deserialization gadget chains
can achieve RCE when a vulnerable library is on the classpath.

**PHP**: patterns matching PHP's serialization format (`O:`, `a:`, `s:`) in responses
or parameters. The tool sends a harmless `O:8:"stdClass":0:{}` payload to relevant
endpoints and checks for 500 responses that suggest the server is attempting to
deserialize the input.

**Python pickle**: magic bytes `\x80\x04\x95` or `\x80\x03` in responses indicate
pickled data. Pickle deserialization of attacker-controlled input is arbitrary code
execution.

Findings from `mg-deser` are flagged as requiring manual exploitation confirmation,
not as confirmed RCE. The detection is reliable; the impact verification is manual.

---

## Combining API Tests

GraphQL and OpenAPI specs discovered during [crawling](/wiki/geistscope-crawl-and-probe)
feed directly into `mg-graphql` and `mg-openapi`. gRPC services appear in port scan
output with TLS ALPN indicating `h2`. WebSocket endpoints are found in the crawl
link graph. The API surface tools pick up where the standard HTTP testing tools leave
off, covering the protocol and format variations that parameter fuzzing doesn't reach.
