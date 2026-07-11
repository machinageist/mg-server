---
title: "mg-k8s"
date: 2026-05-18
summary: "Kubernetes API server, etcd, kubelet, and dashboard misconfiguration scanner."
tags: [geistscope, cli, cloud, infra]
---

## Purpose

Scans Kubernetes infrastructure for common misconfigurations: anonymous API server
access, missing RBAC, exposed etcd, and unauthenticated dashboards. Reads target
hosts from the engagement `recon/summary.json`. API server is probed on ports 6443,
8443, 443, and 8080. Kubelet TLS verification is intentionally disabled because
self-signed certs are the norm on kubelets.

## Output

- `k8s/results-<timestamp>.json` — per-host findings: anonymous access flags, RBAC
  status, exposed endpoints, and dashboard presence.

## CLI

```bash
mg-k8s acme-bounty
mg-k8s acme-bounty --host k8s.acme.example.com
```

## Notes

- Anonymous access and missing RBAC are flagged as HIGH severity findings.
- Exposed dashboards are flagged separately and written to `findings/` automatically.
- `--host` skips the `recon/summary.json` lookup and targets a single host directly.
