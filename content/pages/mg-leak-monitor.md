---
title: "mg-leak-monitor"
date: 2026-05-18
summary: "Continuous GitHub leak monitor that polls for new commits from a target org containing secrets, with state persisted across restarts."
tags: [geistscope, cli, osint, monitoring]
---

## Purpose

Run as a long-lived process during an engagement to catch secrets committed
to GitHub in real time. State is persisted so the monitor can restart without
re-alerting on previously seen commits.

## Output

- `monitor/state.json` — cursor (last-seen commit timestamp per query),
  updated after each poll cycle.
- Engagement findings file — new findings are appended as they are discovered,
  using the same schema as other tools.

## CLI

```bash
mg-leak-monitor acme-bounty --token $GH_TOKEN
mg-leak-monitor acme-bounty --org acme-corp --interval-secs 300 --token $GH_TOKEN
```

## Notes

- Polls the GitHub Search API for commits matching secret patterns and the
  target domain or org name.
- Token is required for reliable operation. Unauthenticated rate limit (10
  req/min) is too low for continuous polling at useful intervals.
- Default poll interval is 300 seconds. Shorter intervals increase rate limit
  pressure and are not recommended without a GitHub Enterprise token.
- State in `monitor/state.json` tracks the last-seen result per query so that
  restarts do not produce duplicate alerts.
- For a one-shot search rather than continuous monitoring, use
  [mg-github](/wiki/mg-github).
