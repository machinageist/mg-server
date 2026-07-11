---
title: "mg-docker"
date: 2026-05-18
summary: "Docker daemon and Portainer exposure scanner that flags unauthenticated API access and privileged host-mount containers."
tags: [geistscope, cli, cloud, infra]
---

## Purpose

Detects exposed Docker daemons and Portainer instances across engagement hosts.
Probes the Docker API on ports 2375, 2376, and 4243; probes Portainer on 9000 and
9443. When unauthenticated access is confirmed, lists running containers and images
and checks for privileged or host-mount containers. Reads hosts from
`recon/summary.json`.

## Output

- `docker/results-<timestamp>.json` — per-host findings: API auth status, container
  inventory, privileged access flags, and Portainer exposure.

## CLI

```bash
mg-docker acme-bounty
mg-docker acme-bounty --host docker.acme.example.com
```

## Notes

- The privileged-mount container CREATE payload is never sent; it is documented in
  findings for manual verification only.
- Portainer findings include version and whether default credentials responded.
- `--host` skips `recon/summary.json` and targets a single host directly.
