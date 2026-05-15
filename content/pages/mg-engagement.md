---
title: "mg-engagement"
date: 2026-05-15
summary: "Create and manage engagement workspaces, scope, notes, finding skeletons, and session profiles."
tags: [geistscope, cli, engagement, workspace]
---

## Purpose

`mg-engagement` is the entry point for every authorized engagement. It owns
the workspace lifecycle: directory creation, scope rules, notes, findings,
and session credential profiles. Other tools read and write inside the
directory this one builds.

## Outputs

- `engagement.json` — name, target, platform, tags, created-at.
- `scope.json` — default-deny rule set; in-scope and explicit deny patterns.
- `notes.md` — operator scratchpad with timestamped append-only entries.
- `audit.log` — append-only record of every tool invocation against the engagement.
- `findings/<id>-<slug>.md` — finding skeletons created by hand or by other tools.
- `session.json` — env-var-backed auth profile (no plaintext secrets).

## CLI

```bash
mg-engagement init acme-bounty --target acme.example.com --platform hackerone
mg-engagement scope-add  acme-bounty "*.acme.example.com"
mg-engagement scope-deny acme-bounty "*.dev.acme.example.com"
mg-engagement note       acme-bounty "noticed JWT in localStorage"
mg-engagement finding new acme-bounty --title "Open redirect on /login" --severity medium
mg-engagement credentials-set  acme-bounty --token-env ACME_TOKEN
mg-engagement credentials-test acme-bounty --url https://acme.example.com/api/me
```

## Notes

- `scope.json` is consulted by every active tool. Out-of-scope hosts are
  refused, not warned.
- `credentials-set` only writes references like `token_env`/`password_env`.
  The actual secrets stay in the operator's shell env or secrets manager.
- See [mg-harness](/wiki/mg-harness) for `engagement.open`, `engagement.status`,
  `scope.check`, `session.set`, and `session.get_headers` endpoints.
