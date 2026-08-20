# Security review: 2026-08-20

## Purpose

This review covered two related questions:

1. Can an untrusted web request cross a code or filesystem boundary it should not cross?
2. Does the public site reveal enough separate operational facts to make reconnaissance materially easier?

The second question matters even when every individual fact looks harmless. Product names, topology, current control gaps, recovery order, and process telemetry can combine into a useful map.

## Scope

Reviewed:

- Axum routing, handlers, middleware, error responses, and shared state
- Markdown loading and HTML rendering boundaries
- query strings, route parameters, and study-form input
- HTTP response headers and public status endpoints
- Rust dependency advisories and repository secret scanning
- public posts and lab pages that discuss hosting, management, segmentation, and remote access
- CI security checks

Not reviewed because the configuration is not in this repository:

- Cloudflare account and edge rules
- tunnel configuration
- reverse-proxy configuration
- service-manager sandboxing
- host firewall, hypervisor, switch, and backup configuration

This is a repository and public-surface review, not a penetration test or a claim that the deployed system is free of vulnerabilities.

## Threat model

The application is public, read-mostly, and has no accounts, write API, or database. Attackers can still control:

- URL paths and encoded route parameters
- search queries
- study form bodies
- request headers
- request rate and concurrency

Repository-controlled Markdown is trusted content, but its source HTML is not trusted as executable markup. The renderer now escapes author-supplied HTML before the compiled templates render the generated Markdown HTML.

## Findings and remediation

### 1. Markdown route values reached filesystem joins

Severity: medium

Several content loaders constructed a Markdown path from a decoded route slug before applying a common allowlist. The forced `.md` extension and sanitized error pages limited impact, but path-like values could leave the intended content directory and reach another Markdown file.

Fixed by:

- adding one lowercase ASCII slug validator in `src/models/slug.rs`
- applying it inside the `Page`, `BlogPost`, question-set, and scenario loaders before any path join
- leaving handler-level allowlists in place where a route already has a narrower model allowlist
- adding regression tests for path-like values at every filesystem-backed loader

The validation lives in the model layer so a future handler cannot bypass it by calling a loader directly.

### 2. One global rate-limit bucket covered every visitor

Severity: medium

The previous limiter was not keyed. One client could spend the process-wide burst budget and cause unrelated visitors to receive HTTP 429 responses.

Fixed by:

- changing the governor limiter to a keyed limiter
- assigning an independent token bucket to each parsed `CF-Connecting-IP` address
- mapping missing or malformed metadata to one shared fail-closed key so arbitrary header strings cannot create unlimited buckets
- adding a regression test that exhausts one client's quota and proves another client still succeeds

Deployment constraint: this trust model is valid only while the origin remains reachable through the trusted local reverse-proxy/Cloudflare path. If the origin is exposed directly, client-supplied forwarding headers must not be trusted. Edge rate limiting remains the preferred first layer for volumetric abuse.

### 3. Public status surfaces exposed process telemetry

Severity: low

The footer and status endpoints published process uptime, request totals, resident memory, build time, crate version, and listener classification. The values were not secrets, but together they exposed restart timing, deployment cadence, rough process behavior, and listener details.

Fixed by:

- removing the process-vitals strip from every page
- reducing `/status` and `/status.json` to a coarse availability value
- deleting collection and formatting code used only by the removed public metrics
- retaining `Cache-Control: no-store` on status responses
- adding negative tests that reject the removed fields and labels

### 4. CSP omitted several useful restrictions

Severity: low, defense in depth

The CSP already restricted scripts, styles, images, fonts, connections, and framing. It did not explicitly restrict document base URLs, form destinations, or plugin/object content.

Fixed by adding:

- `base-uri 'none'`
- `form-action 'self'`
- `object-src 'none'`

A regression test now asserts all three directives. CSP limits the consequences of some injection; it does not fix an injection bug or make raw HTML automatically safe.

### 5. Markdown source accepted active HTML

Severity: low to medium

Markdown output is intentionally inserted into compiled templates as generated HTML. The parser previously passed source HTML through unchanged, so a compromised content commit could add active markup to public pages.

Fixed by:

- converting source `Html` and `InlineHtml` parser events to text events before rendering
- allowing pulldown-cmark to escape those events
- keeping only the heading permalink HTML generated by application code active
- adding a regression test with script and form markup

This removes raw source HTML as a content extension point without adding a second sanitizer dependency.

### 6. Public copy created cumulative reconnaissance value

Severity: moderate operational exposure

No credential, private address, internal hostname, or VM identifier was found. Existing content linting already catches many direct identifiers. The larger issue was accumulation: public pages named the request chain, current topology, management dependencies, missing controls, recovery mechanics, and planned trust boundaries in enough detail to reduce discovery work.

Fixed by:

- removing exact service identifiers and copied DNS answers from the hosting article
- replacing the public weakness inventory with a bounded portfolio-scope statement
- generalizing the cluster incident to dependency classes and verification lessons
- labeling lab pages as sanitized reference designs rather than live runbooks
- removing current management reachability, cluster size, product-specific recovery order, and real target examples where they added little teaching value
- correcting stale claims that the site had no user input or CI
- adding a content lint that blocks a small set of high-value disclosure phrases from returning

The rule is not "say nothing technical." Public writing can explain architecture classes, failure analysis, and verification methods. Current identifiers, weakness inventories, maintenance timing, and recovery paths stay private.

### 7. Security checks were not enforced in CI

Severity: low process risk

CI already ran formatting, Clippy, tests, and a release build. Dependency advisories and secret scanning were manual.

Fixed by:

- adding RustSec advisory checking and Gitleaks to CI
- pinning every GitHub Action to a full commit SHA
- using the lockfile for Clippy, tests, and release builds

## Verification performed

The remediation was checked with:

- targeted red/green regression tests for each behavior change
- `cargo fmt --all -- --check`
- `cargo test`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo build --release --locked`
- `cargo audit`
- Gitleaks against the working tree
- local HTTP probes for status JSON, CSP, independent client quotas, and encoded path rejection
- `git diff --check`

The exact final command results belong in the associated change record rather than in this document, because counts and dependency totals change as the repository evolves.

## Residual risks and follow-up

- Markdown source HTML is escaped. Future renderer features should preserve that boundary rather than reintroducing raw pass-through.
- Application rate limiting depends on trusted proxy metadata and is not a substitute for edge abuse controls.
- Infrastructure controls outside this repository were not verified.
- Public copy needs periodic cumulative review; identifier linting alone cannot detect every useful operational narrative.
- Security claims should remain narrow: reviewed, tested, and hardened is accurate. "Secure" or "penetration tested" is not.
