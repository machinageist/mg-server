---
title: "mg-metadata"
date: 2026-05-18
summary: "Document metadata extractor for PDFs, Office docs, and JPEGs found in the crawl corpus, surfacing author names, internal paths, and software versions."
tags: [geistscope, cli, osint, recon]
---

## Purpose

Download documents linked in the crawl corpus and extract metadata that may
reveal internal infrastructure details, software versions, author names, and
internal file paths. Common in pre-engagement recon on corporate targets that
publish reports or marketing materials.

## Output

- `recon/metadata-findings.json` — per-file metadata: document URL, file
  type, extracted author, creator application, internal paths, and any version
  strings found.

## CLI

```bash
mg-metadata acme-bounty
mg-metadata acme-bounty --concurrency 5
```

## Notes

- Source URLs are taken from the crawl corpus (mg-crawl output). Only URLs
  with `.pdf`, `.docx`, `.xlsx`, `.pptx`, or `.jpeg`/`.jpg` extensions are
  fetched.
- DOCX, XLSX, and PPTX files are ZIP containers. The tool reads
  `docProps/core.xml` directly from the archive without extracting to disk.
- JPEG EXIF data is located by scanning for the `FFE1` magic byte sequence
  rather than using an EXIF library.
- No external metadata libraries are used. Parsing is done from raw bytes.
- Internal paths (Windows-style `C:\Users\...`) in document metadata are
  flagged as findings.
