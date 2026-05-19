---
title: "mg-ipa"
date: 2026-05-18
summary: "iOS IPA static analysis for ATS exceptions, hardcoded secrets, and debuggable binary flags."
tags: [geistscope, cli, mobile, static-analysis]
---

## Purpose

Treats the IPA as a ZIP container and inspects its `Payload/AppName.app/` directory.
Checks `Info.plist` for App Transport Security exceptions that allow cleartext HTTP
or disable certificate validation. Scans the Mach-O binary for hardcoded secrets and
URL patterns. Flags the debuggable entitlement when present. Fully synchronous, no
async runtime.

## Output

- `ipa/<name>-findings.json` — categorized findings with severity, source location,
  and matched value (secrets masked to first 8 chars).

## CLI

```bash
mg-ipa acme-bounty --ipa ./AcmeApp.ipa
mg-ipa acme-bounty --ipa ./AcmeApp.ipa --output-dir ./results
```

## Notes

- No Xcode, device, or macOS requirement; runs on any platform.
- `Info.plist` is scanned as raw bytes and handles both binary plist and XML plist
  formats.
- ATS keys scanned: `NSAllowsArbitraryLoads`, `NSExceptionAllowsInsecureHTTPLoads`,
  `NSExceptionMinimumTLSVersion`, and domain-specific exceptions.
