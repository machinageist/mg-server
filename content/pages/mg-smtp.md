---
title: "mg-smtp"
date: 2026-05-18
summary: "SMTP enumeration: user enum via VRFY/EXPN, open relay test, and header injection check, all via raw TCP."
tags: [geistscope, cli, network]
---

## Purpose

Enumerate SMTP servers in the engagement for user account disclosure, open
relay misconfiguration, and header injection vulnerabilities.

## Output

- `recon/smtp.json` — per-host SMTP result: server banner, VRFY/EXPN
  responses for each tested username, relay test outcome, and header injection
  test outcome.

## CLI

```bash
mg-smtp acme-bounty
mg-smtp acme-bounty --users-file /usr/share/wordlists/usernames.txt
```

## Notes

- Only probes hosts with port 25 or 587 open in the engagement recon. Does
  not attempt to discover SMTP servers independently.
- Raw TCP, no SMTP library. Commands are sent as ASCII lines and responses
  are read line by line.
- Default username list is small (common admin names). Pass `--users-file`
  to use a wordlist.
- Open relay test sends a MAIL FROM with an external envelope sender and
  an external RCPT TO and checks whether the server accepts the message.
- Header injection test sends a MAIL FROM with an embedded CRLF sequence
  in the address to check whether the server accepts or rejects the malformed
  input.
