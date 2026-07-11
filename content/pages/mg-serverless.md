---
title: "mg-serverless"
date: 2026-05-18
summary: "Serverless runtime metadata prober via SSRF, covering AWS Lambda, GCP Cloud Functions, and Azure Functions endpoints."
tags: [geistscope, cli, ssrf, cloud]
---

## Purpose

Probes serverless runtime metadata endpoints by appending each metadata URL as a query parameter value to `--ssrf-url`. Covers AWS Lambda (`AWS_CONTAINER_CREDENTIALS_RELATIVE_URI`), GCP Cloud Functions metadata, and Azure Functions runtime endpoints. A HIGH finding is emitted if credentials or runtime event data are exposed. Sensitive fields are masked in output.

## Output

- `serverless/results-<timestamp>.json` — per-platform findings with metadata URL probed, response excerpt, and masked credential fields.

## CLI

```bash
mg-serverless acme-bounty --ssrf-url "https://api.acme.example.com/fetch?url="
```

## Notes

- Requires a confirmed SSRF endpoint; use [mg-ssrf](/wiki/mg-ssrf) to identify one first.
- GCP Cloud Functions metadata requires `Metadata-Flavor: Google` to be forwarded; a note about this is included in findings when the header appears to be stripped.
- For deeper cloud credential extraction, chain into [mg-aws](/wiki/mg-aws), [mg-gcp](/wiki/mg-gcp), or [mg-azure](/wiki/mg-azure).
