---
title: "mg-azure"
date: 2026-05-18
summary: "Azure IMDS managed identity token extractor and instance info prober via a confirmed SSRF endpoint."
tags: [geistscope, cli, ssrf, cloud]
---

## Purpose

Extracts Azure managed identity tokens and instance metadata via a confirmed SSRF endpoint. Also probes the `userData` endpoint for leaked environment variables. The `Metadata: true` header must be forwarded by the SSRF target. The `access_token` field is masked to the first 8 characters in output.

## Output

- `azure/results-<timestamp>.json` — managed identity token (masked), subscription ID, resource group, VM name, and userData content if present.

## CLI

```bash
mg-azure acme-bounty --ssrf-url "https://api.acme.example.com/fetch?url="
```

## Notes

- Requires a confirmed SSRF endpoint; use [mg-ssrf](/wiki/mg-ssrf) to identify one first.
- `Metadata: true` header forwarding is required; test manually if results are empty.
- userData endpoint can expose startup scripts, environment variables, and cloud-init configs.
- Related: [mg-serverless](/wiki/mg-serverless) for Azure Functions runtime metadata.
