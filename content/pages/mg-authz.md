---
title: "mg-authz"
date: 2026-05-18
summary: "IDOR/BOLA multi-role authorization tester that replays recorded requests under two sessions and flags broken object-level access control."
tags: [geistscope, cli, auth]
---

## Purpose

Replays a set of recorded requests under two named sessions (session-A and session-B) to detect broken object-level authorization. A finding is emitted when session-B successfully reaches a resource scoped to session-A ownership. Session headers are loaded via `session::load_session_config` for named sessions, or from an environment variable JSON blob for ad-hoc use. Concurrency is bounded via JoinSet.

## Output

- `authz/results-<timestamp>.json` — per-request findings with session used, status codes for both sessions, and resource identifier.

## CLI

```bash
mg-authz acme-bounty --session-a alice --session-b bob --requests-file recorded.json
```

## Notes

- Named sessions map to stored header configs; create them via the session management conventions in the engagement directory.
- `--requests-file` format mirrors [mg-replay](/wiki/mg-replay) recorded request JSON.
- Works well after [mg-crawl](/wiki/mg-crawl) to source ownership-scoped endpoint candidates.
- Related: [mg-jwt](/wiki/mg-jwt) for forging session tokens to use as session-B.
