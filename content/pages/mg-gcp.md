---
title: "mg-gcp"
date: 2026-05-18
summary: "GCP metadata SSRF chain that extracts service account tokens and project info via a confirmed SSRF endpoint."
tags: [geistscope, cli, ssrf, cloud]
---

## Purpose

Extracts GCP service account tokens and project metadata via a confirmed SSRF endpoint. The `Metadata-Flavor: Google` header must be forwarded by the SSRF target; the tool tries with the header first and retries without it on failure. The `access_token` field is masked to the first 8 characters in all output.

## Output

- `gcp/results-<timestamp>.json` — service account email, project ID, token (masked), and scopes extracted.

## CLI

```bash
mg-gcp acme-bounty --ssrf-url "https://api.acme.example.com/fetch?url="
```

## Notes

- Requires a confirmed SSRF endpoint; use [mg-ssrf](/wiki/mg-ssrf) to identify one first.
- Many GCP metadata endpoints require `Metadata-Flavor: Google`; if the SSRF strips headers, the retry-without-header path may still succeed on misconfigured instances.
- Related: [mg-serverless](/wiki/mg-serverless) for GCP Cloud Functions runtime metadata.
