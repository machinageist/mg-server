---
title: "mg-aws"
date: 2026-05-18
summary: "AWS IMDS credential extractor that chains through a confirmed SSRF endpoint, attempting IMDSv2 first with v1 fallback."
tags: [geistscope, cli, ssrf, cloud]
---

## Purpose

Extracts AWS IAM credentials from the Instance Metadata Service via a confirmed SSRF endpoint. Attempts IMDSv2 first (PUT token request through the SSRF, then GET with the token); falls back to IMDSv1 if the PUT fails. Sensitive fields (AccessKeyId, SecretAccessKey, Token) are masked to the first 4 characters in output. A HIGH finding is emitted if credentials are successfully extracted.

## Output

- `aws/results-<timestamp>.json` — extracted credential fields (masked), IAM role name, and IMDSv1/v2 path used.

## CLI

```bash
mg-aws acme-bounty --ssrf-url "https://api.acme.example.com/fetch?url="
```

## Notes

- Requires a confirmed SSRF endpoint; use [mg-ssrf](/wiki/mg-ssrf) to identify one first.
- IMDSv2 requires the target SSRF to forward the `X-aws-ec2-metadata-token` header; not all SSRF implementations do.
- Related: [mg-serverless](/wiki/mg-serverless) for Lambda-specific metadata endpoints.
