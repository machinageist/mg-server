---
title: "mg-udp-scan"
date: 2026-05-18
summary: "UDP port scanner with service fingerprinting for common protocols: DNS, SNMP, NTP, TFTP, and others."
tags: [geistscope, cli, recon, network]
---

## Purpose

Probe UDP ports on scoped hosts and identify services from response payloads.
Covers the protocols most likely to be exposed and misconfigured on internal
and external targets.

## Output

- `recon/udp-scan.json` — per-host UDP port results with status and identified
  service where the payload was recognizable.

## CLI

```bash
mg-udp-scan acme-bounty
mg-udp-scan acme-bounty --ports 53,161,123 --timeout-ms 500
```

## Notes

- Any non-timeout response, including ICMP port-unreachable, is treated as
  `open`. This matches the behavior of Nmap's UDP scan logic.
- Raw UDP datagrams are sent via `tokio::net::UdpSocket`. No OS raw socket
  privileges required for most probes.
- Default port list covers: 53 (DNS), 67 (DHCP), 69 (TFTP), 111 (RPC), 123
  (NTP), 161 (SNMP), 500 (IKE), 514 (syslog), 1194 (OpenVPN), 5353 (mDNS).
- For detailed SNMP extraction after open ports are identified, run
  [mg-snmp](/wiki/mg-snmp).
