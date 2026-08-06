# UNPUBLISHED — GeistScope restructuring tracker

> **Publication state:** internal draft only. `content/drafts/` is not scanned or routed by
> `mg-server`. Do not move this file into `content/pages/` or `content/posts/`, add it to the wiki
> sidebar, create a route, or add publication tests until every item in the republication gate is
> satisfied and Jeff explicitly chooses to republish it.
>
> **Evidence boundary:** this tracker describes repository implementation and local controlled-test
> evidence only. It does **not** claim that GeistScope has been proven on an authorized engagement,
> produced a bounty, or is ready for operational use.

Last reviewed: 2026-08-06

## Change log

- **2026-08-06 — tracker created.** Reconciled the current restructuring decision log, the SSRF
  vertical-slice plan/spec, commit `a140fa7a86deab78016bcc658da09736a502a461`, the checked-in
  `hunt-engine` implementation, its tests, and the current working trees. Recorded implemented and
  pending work without restoring any public GeistScope page.
- **2026-08-06 — first independent product comparison and blind AAA review recorded.** Current
  first-party documentation for Burp Scanner/Collaborator, ProjectDiscovery Nuclei/Interactsh,
  OWASP ZAP Active Scanner/OAST, and Caido was reviewed independently. A separate reviewer then
  inspected the GeistScope implementation without reading its plans and returned **FAIL AAA**:
  architecture/authority **F**, adversarial safety **F**, and acceptance/product utility **C**.
  Corrective sub-slices are now required before any implemented slice can be called complete.

## Current product and architecture direction

The current direction is a local-first, single-user adaptive hunting system, not the former collection
of standalone scanners. The intended closed loop is:

```text
program rules + scope + controlled identities
                    ↓
          normalized engagement state
                    ↓
 observations → hypotheses → immutable action plans
                    ↓
 deterministic policy and budget gate
                    ↓
 bounded execution → evidence → deterministic assessment
                    ↓
 verified findings / human-verification candidates / negative tests / invalid tests
                    ↓
 hunt ledger, adaptive follow-up, verification runbooks, report drafts
```

The target boundary is one authoritative engine with CLI, TUI, built-in AI, and external agents as
clients of the same versioned local API. AI may propose, prioritize, explain, and draft; deterministic
Rust code must own scope, policy, credentials, budgets, request execution, callback correlation,
evidence integrity, and result promotion. The initial depth priorities are SSRF, broken access
control/IDOR, and RCE or critical server-side injection. Capability breadth beyond the web/API slice
is deferred.

### Architecture actually present at the reviewed commit

A new `hunt-engine` Rust library is a workspace member and currently contains the first cohesive
engine contracts:

- capability-neutral observations, hypotheses, action plans, budgets, stop conditions, evidence
  requirements, result classes, and approval-sensitive plan hashes;
- an engine-owned normalized HTTP observation and deterministic, network-inert SSRF candidate
  generator for absolute HTTP(S) query values;
- immutable callback-only SSRF probe plans with a public redacted view, policy-safe action envelope,
  and crate-private executable state;
- a narrow adapter to the existing `action-policy` crate as the sole policy authority;
- a bounded `reqwest` executor with one-request/concurrency limits, redirects disabled, explicit
  stop outcomes, no ambient proxy use, and sanitized transaction records;
- engine-owned, plan-bound callback registration, recording, correlation, and deterministic SSRF
  assessment foundations, with listener-owned routing/time and streaming-ingestion boundaries still
  incomplete;
- evidence/result contracts that keep verified findings distinct from candidates, negative tests,
  and invalid tests.

This is a library slice, not yet the complete product architecture. The existing broad workspace and
old binaries remain in the repository. No versioned local engine API, SQLite-owned campaign state,
thin operator CLI for this rebuilt path, completed immutable evidence writer, or end-to-end campaign
orchestrator was found at the reviewed commit. The old surfaces therefore must not be presented as
clients of the new engine yet.

## Implemented and locally tested slices — not AAA-complete

The current source and tests support these narrower statements:

1. **Engine/result foundations:** implemented domain contracts prevent arbitrary serialized/model
   confidence from directly constructing a verified result. Local evidence references are typed,
   hashed, engagement-relative, and checked against canonical paths; tests cover missing files,
   wrong kinds, changed content, and symlink escapes.
2. **Controlled benchmark fixtures:** implemented vulnerable and secure HTTP fixtures bind to
   ephemeral loopback ports. Tests assert one outbound GET for the vulnerable fixture and no
   callback contact from the secure fixture.
3. **Plan-bound callback lifecycle foundation:** implemented unique expiring callback tokens,
   plan-hash binding, redacted callback records, route parsing helpers, and rejection of unknown,
   expired, reused, or mismatched tokens. The blind review found that the recorder still accepts the
   token separately from the request path, trusts caller-supplied receipt time, and receives an
   already-buffered body before applying its nominal cap. Exact listener-owned route/time authority
   and streaming size enforcement therefore remain pending.
4. **Inert SSRF candidate generation and immutable planning:** implemented query-parameter candidate
   discovery without network traffic, exact occurrence mutation, callback-origin restrictions,
   one-request safety ceilings, authorization-expiry stops, canonical hash material, and redacted
   public plan views.
5. **Policy gating:** implemented fail-closed adaptation to `action-policy`; tests cover scope,
   ambiguity, rule freshness, automation permission, bounty eligibility, approval, artifact/hash
   mismatches, audit write failure, and audit-path escape before granting an execution permit.
6. **Bounded execution:** implemented a permit-bound HTTP executor. Tests cover exact mutation,
   redirect refusal, proxy bypass, response-size/time bounds, rate-limit/server/TLS stops, expired
   authorization, plan/input mismatch, and zero-I/O fail-closed cases.
7. **Deterministic callback assessment:** implemented callback-proven, pending, negative, and invalid
   assessments from transaction and callback state. Target response text alone cannot prove SSRF.

These statements describe code and focused tests, not completed product slices. The first blind AAA
review found blocking cross-slice authority and safety defects, including incomplete executable
request identity in the plan hash, caller-controlled policy/execution time, repeat permit issuance,
secret-bearing path/header serialization, and unbounded callback-body ingestion. The corrective loop
started with sub-slice 0A: canonical full executable-request identity and secret-safe public metadata.

Verification run on 2026-08-06 against clean GeistScope commit
`a140fa7a86deab78016bcc658da09736a502a461`:

- `cargo test -p hunt-engine`: **passed** (all reported package and documentation tests passed;
  one dead-code warning).
- `cargo clippy -p hunt-engine --all-targets -- -D warnings`: **failed** because
  `EvidenceContract::validate_local` is currently unused in non-test library code. This is consistent
  with the pending evidence/result-emission slice and means the plan's clean-Clippy final gate is not
  currently met.
- The GeistScope working tree was clean when reviewed.

The plan header says “Task 3.2 implemented,” but the checked-in code and tests also implement the
policy gate, bounded executor, and deterministic callback assessment corresponding to Tasks 3.3,
4.1, and 4.2. This tracker follows the code and tests while retaining the plan's explicit pending
status for later slices.

## Pending or unverified slices

- **Immutable evidence bundle and result emission (Task 4.3):** write the observation, candidate,
  plan, policy decision, transaction, callback/correlation, audit references, redacted summary,
  verification runbook, manifest, and hashes; refuse overwrite; fail closed on write errors; wire
  locally validated evidence into result promotion.
- **End-to-end campaign benchmark (Task 5.1):** prove vulnerable, secure, blocked, and repeated-run
  cases through one complete orchestration path, including exact network counts, immutable evidence,
  token non-reuse, and confinement to the engagement root.
- **Local API and persistence checkpoint:** choose and implement the versioned local API and SQLite
  boundary from observed campaign transitions. These are planned architecture, not current behavior.
- **Operator surface:** add one thin CLI client with stable JSON after the engine API is chosen. TUI,
  native-AI, and external-agent clients remain future integration work.
- **Pruning/migration:** classify old SSRF/OOB surfaces as adapters, quarantined code, or deletions
  only after the replacement passes the complete benchmark. The broad legacy workspace is still
  present and must not be counted as completed product capability.
- **Later product depth:** authenticated/multi-identity flows, IDOR/broken-access-control and critical
  injection slices, adaptive scheduling/ledger, expand-around-success behavior, browser workers,
  report drafting, resume/watch mode, and capability-pack migration remain planned.
- **Full verification gate:** the plan still requires clean formatting, workspace tests, workspace
  Clippy with warnings denied, and `git diff --check`. Only package tests were re-run for this tracker;
  package Clippy is currently not clean.

## Product-comparison and AAA review status

- **Current product comparison: completed for the first SSRF review cycle.** An independent research
  agent checked current first-party documentation for Burp Scanner/Collaborator, ProjectDiscovery
  Nuclei/Interactsh, OWASP ZAP Active Scanner/OAST, and Caido. Burp is the integrated workflow,
  callback-authority, and evidence reference; Nuclei/Interactsh is the mutation precision,
  reproducibility, extensibility, and self-hosted OAST reference; ZAP is an open provider/API and
  active-scan safety reference; Caido is used only for request-centric UX and plugin ergonomics, not
  as the primary SSRF/OAST capability baseline.
- **AAA is now defined on three independent axes:** Architecture/authority integrity, Adversarial
  safety, and Acceptance/product utility. Each axis has ten scored criteria; AAA requires an A on all
  three, every mandatory criterion at full credit, no hard-fail condition, and black-box evidence
  from a clean build/configuration.
- **First blind result: FAIL AAA (F/F/C).** The reviewer was denied the GeistScope plans and prior
  conclusions and evaluated current code, tests, and public interfaces. Hard failures included
  executable requests that can differ in unselected query values while sharing a plan hash,
  caller-controlled policy/execution time, repeat permit issuance against a one-request plan,
  callback proof accepting a token outside the request route, callback bodies buffered before the
  size cap, and secret-bearing path or nonstandard header values crossing serialized boundaries.
  Unit tests still passed, demonstrating that green focused tests are not sufficient evidence of
  product readiness.
- **Remediation state:** corrective work is active, beginning with sub-slice 0A. Every correction must
  be inspected by a new blind reviewer against the same real-world references. A failed axis creates
  another corrective sub-slice; it is not waived or relabeled complete.

A future comparison/review should evaluate demonstrated workflows only: installation and local-first
operation, scope/policy enforcement, secret isolation, request capture and replay, deterministic
proof/evidence integrity, human and agent control surfaces, extension model, recovery/resume,
reporting, and the amount of manual proxy work still required. Planned features must remain visibly
separate from tested behavior.

## Republication gate and checklist

Do **not** republish GeistScope tool documentation merely because a crate exists or package tests pass.
Republication requires all of the following:

- [ ] Jeff explicitly decides that GeistScope is stable enough to reconsider public documentation.
- [ ] A complete pipeline works from engagement/scope intake through observation, policy-gated
      execution, immutable evidence, deterministic result classification, and report/runbook output.
- [ ] The same engine path is operable through a maintained human interface and a stable typed
      machine interface for AI agents; neither duplicates or bypasses policy logic.
- [ ] Required package/workspace tests, formatting, Clippy with warnings denied, and integrity checks
      pass at the exact candidate commit; command output and commit hash are retained.
- [ ] Product-comparison review is completed against current versions using demonstrated behavior,
      with planned capabilities labeled as planned.
- [ ] “AAA review” is explicitly defined, performed, documented, and any blocking findings are closed.
- [ ] Sanitized evidence exists from an **actual authorized engagement**, with authorization and
      disclosure constraints checked. Local fixtures, unit tests, simulations, and lab benchmarks do
      not satisfy this item.
- [ ] Publication material separates verified findings, human-verification candidates, negative
      coverage, invalid tests, and planned work, and does not imply bounties, professional engagement
      history, exhaustive coverage, or unsupported severity.
- [ ] Secrets, target identifiers, program-private rules, raw traffic, callback tokens, credentials,
      and identifying engagement metadata are removed or safely represented by placeholders.
- [ ] Claims are rechecked against the exact source commit and independently reviewed for technical
      accuracy, authorship, authorization, and overclaiming.
- [ ] Jeff manually chooses the public format and destination. Only then should a separate change add
      content under `content/pages/` or `content/posts/`, sidebar/navigation entries, routes, and
      publication regression tests.

Until every checkbox is satisfied, the public state remains unchanged: no GeistScope tool pages and
no routing from this draft.

## Evidence used for this review

Primary implementation evidence:

- `/home/mgeist/geistscope/.hermes/plans/2026-07-23_hunting-engine-restructure.md`
- `/home/mgeist/geistscope/.hermes/plans/2026-07-31_ssrf-vertical-slice/SPEC.md`
- `/home/mgeist/geistscope/.hermes/plans/2026-07-31_ssrf-vertical-slice/PLAN.md`
- `/home/mgeist/geistscope/.hermes/plans/2026-07-31_ssrf-vertical-slice/agent-specs/README.md`
- `/home/mgeist/geistscope/crates/Cargo.toml`
- `/home/mgeist/geistscope/crates/hunt-engine/Cargo.toml`
- `/home/mgeist/geistscope/crates/hunt-engine/src/` and `/tests/`
- GeistScope commit `a140fa7a86deab78016bcc658da09736a502a461` (`hunt engine rebuild`,
  committed 2026-08-02)
- Independent implementation-status audit:
  `/home/mgeist/.hermes/cache/delegation/subagent-summary-0-20260806_093033_640218.txt`
- Independent real-world comparison and AAA rubric:
  `/home/mgeist/.hermes/cache/delegation/subagent-summary-1-20260806_093033_643100.txt`
- Blind hostile implementation review:
  `/home/mgeist/.hermes/cache/delegation/subagent-summary-2-20260806_093033_644189.txt`

Publication-policy and history evidence:

- `/home/mgeist/mg-server/README.md`
- `/home/mgeist/mg-server/docs/public-portfolio-structure.md`
- `/home/mgeist/mg-server/content/drafts/README.md`
- `/home/mgeist/mg-server/content/drafts/geistscope-retrospective.md`
- `mg-server` commits `edaef94` (pruned the earlier public GeistScope surface), `3fc0354` (removed
  stale GeistScope wiki pages), and `96985c6` (moved the retrospective into unrouted drafts)
