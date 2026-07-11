---
title: "mg-apk"
date: 2026-05-18
summary: "Android APK static analysis for security misconfigurations, hardcoded secrets, and cleartext traffic permissions."
tags: [geistscope, cli, mobile, static-analysis]
---

> As of 2026-05-22 this tool is a subcommand of `mg-artifact-audit`. The standalone
> `mg-apk` binary has been retired; behavior is unchanged.

## Purpose

Treats the APK as a ZIP container and inspects its internals without installing or
executing anything. Checks `AndroidManifest.xml` for dangerous flags: `debuggable`,
`allowBackup`, and exported components with no permission requirement. Scans DEX
bytecode and asset files for hardcoded secrets and cleartext URLs. Checks
`network_security_config.xml` for cleartext traffic permission. Fully synchronous,
no async runtime.

## Output

- `apk/<name>-findings.json` — categorized findings with severity, file path, and
  matched value (secrets masked to first 8 chars).

## CLI

```bash
mg-artifact-audit apk acme-bounty --apk ./com.acme.app.apk
mg-artifact-audit apk acme-bounty --apk ./app.apk
```

## Notes

- No device, emulator, or Android SDK required.
- DEX scanning reads raw bytes; class and method names are not decompiled, but string
  literals are extracted.
- Exported components with `android:exported="true"` and no `android:permission` are
  flagged as MEDIUM or HIGH depending on component type.
