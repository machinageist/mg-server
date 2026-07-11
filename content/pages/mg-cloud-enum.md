---
title: "mg-cloud-enum"
date: 2026-05-18
summary: "Cloud storage bucket enumeration for S3, GCS, and Azure Blob, with public access detection and finding generation."
tags: [geistscope, cli, recon, cloud, osint]
---

## Purpose

Generate candidate bucket names from the engagement target and check whether
they exist and are publicly accessible across the three major cloud storage
providers.

## Output

- `recon/cloud-enum.json` — per-bucket result with provider, URL, and access
  status. Public listings are also written as HIGH findings in the engagement
  findings file.

## CLI

```bash
mg-cloud-enum acme-bounty
mg-cloud-enum acme-bounty --extra-names staging,dev,backup
```

## Notes

- Three outcome states: `public_listing` (HIGH finding, bucket lists objects),
  `exists_private` (INFO, bucket exists but is not publicly accessible), and
  `not_found`.
- Candidate names are generated from the target domain: root name, with and
  without hyphens, common suffixes (assets, files, data, backup, etc.).
- `--extra-names` appends additional name fragments to the candidate list.
- Checks are unauthenticated HTTP HEAD/GET; no cloud credentials required.
