---
title: "mg-smb"
date: 2026-05-18
summary: "SMB/NetBIOS enumeration via raw TCP: signing check, null session probe, and guest session probe over SMB2 on port 445."
tags: [geistscope, cli, network]
---

## Purpose

Enumerate SMB servers in the engagement for signing misconfigurations, null
session access, and guest session access. These conditions are frequently
exploitable for credential relay or unauthenticated share browsing.

## Output

- `recon/smb.json` — per-host SMB result: signing required (yes/no), null
  session accepted (yes/no), guest session accepted (yes/no), and listed
  shares when access was granted.

## CLI

```bash
mg-smb acme-bounty
mg-smb acme-bounty --concurrency 10
```

## Notes

- Only SMB2 over port 445 is probed. SMB1 (port 139) is not tested.
- Raw TCP, no SMB library. The negotiate and session setup exchanges are
  constructed from the SMB2 wire format spec.
- Signing check reads the `SecurityMode` field from the SMB2 NEGOTIATE
  response. Signing required = FALSE is flagged MEDIUM.
- Null session probe sends a SESSION SETUP with empty credentials. Guest
  probe sends `guest` as username with an empty password.
- Share enumeration is attempted only when a session is established.
