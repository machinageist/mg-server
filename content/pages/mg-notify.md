---
title: "mg-notify"
date: 2026-05-18
summary: "Finding webhook notifier that watches the engagement findings directory and pushes to Slack, Discord, and ntfy.sh."
tags: [geistscope, cli, workflow]
---

## Purpose

Watches the `findings/` directory of an engagement workspace for new JSON files and
sends a notification to configured webhooks: Slack, Discord, and ntfy.sh are all
supported and can be used simultaneously. Seen-file state is persisted to
`notify/state.json` so restarts do not re-send findings that were already delivered.
Exits cleanly on SIGINT.

## Output

- `notify/state.json` — list of finding files already sent; read on startup to avoid
  duplicate notifications.

## CLI

```bash
mg-notify acme-bounty --slack-url $SLACK_WEBHOOK
mg-notify acme-bounty --discord-url $DISCORD_WEBHOOK --ntfy-topic acme-bounty
```

## Notes

- At least one of `--slack-url`, `--discord-url`, or `--ntfy-topic` must be
  provided.
- ntfy.sh messages include severity and finding title in the notification header.
- The watcher polls `findings/` on a short interval; inotify is not required.
