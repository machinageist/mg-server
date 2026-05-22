---
title: "mg-artifact-audit"
date: 2026-05-22
summary: "Merged passive artifact-analysis CLI for JavaScript, source maps, API keys, document metadata, APKs, and IPAs."
tags: [geistscope, cli, static-analysis, artifact-audit]
---

## Purpose

`mg-artifact-audit` collapses six passive artifact analyzers into one subcommand-routed CLI. The goal is to reduce the public tool chest without losing the useful parsing work: JavaScript, source maps, API key extraction, document metadata, APK inspection, and IPA inspection now share one binary and one harness tool pack.

The legacy harness endpoints remain available for compatibility:

| Endpoint | Subcommand |
|---|---|
| `js.analyze` | `mg-artifact-audit js` |
| `sourcemap.fetch` | `mg-artifact-audit sourcemap` |
| `apikey.extract` | `mg-artifact-audit apikey` |
| `metadata.extract` | `mg-artifact-audit metadata` |
| `apk.analyze` | `mg-artifact-audit apk` |
| `ipa.analyze` | `mg-artifact-audit ipa` |
| `artifact.audit` | `mg-artifact-audit audit` |

## CLI

```bash
# One artifact type
mg-artifact-audit js acme-bounty
mg-artifact-audit apikey acme-bounty
mg-artifact-audit sourcemap acme-bounty
mg-artifact-audit metadata acme-bounty
mg-artifact-audit apk acme-bounty --apk ./app.apk
mg-artifact-audit ipa acme-bounty --ipa ./app.ipa

# Multiple passive artifact checks in one pass
mg-artifact-audit audit acme-bounty --types js,apikey,sourcemap,metadata
```

## Harness behavior

`mg-harness` routes each legacy endpoint to `mg-artifact-audit` with the matching subcommand. `artifact.audit` is the high-level endpoint for running multiple artifact analyzers in one call. All artifact-audit endpoints are passive and sit in the `artifact_audit` pack, visible in the default chat profile.

## Why this exists

The previous sprint grew the tool list too quickly. This merge is the first cleanup slice: retire thin standalone binaries, keep their behavior as safer domain-pack subcommands, and make the AI-facing catalog easier to reason about before pruning anything destructive or noisy.

## Notes

- The retired standalone binaries are `mg-js-analyze`, `mg-sourcemap`, `mg-apikey`, `mg-metadata`, `mg-apk`, and `mg-ipa`.
- Existing wiki pages for those names are kept as compatibility/reference pages and point to the new subcommands.
- The code paths are still passive: they read crawl output or local artifacts and write findings under the engagement workspace.
