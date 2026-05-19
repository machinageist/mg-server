---
title: "mg-snmp"
date: 2026-05-18
summary: "SNMP community string brute force and MIB walk for SNMPv1 and SNMPv2c, using manually constructed BER-encoded PDUs."
tags: [geistscope, cli, network]
---

## Purpose

Test SNMP services for weak or default community strings and extract system
information via OID queries. A successful community string match often yields
network topology, interface addresses, and running software details.

## Output

- `recon/snmp.json` — per-host SNMP result: community strings that responded,
  system OID values (sysDescr, sysName, sysLocation, sysContact, ifTable),
  and a severity flag when `public` or `private` succeeds.

## CLI

```bash
mg-snmp acme-bounty
mg-snmp acme-bounty --community-file communities.txt
```

## Notes

- Only probes hosts with UDP port 161 open (from `recon/udp-scan.json` or
  `recon/summary.json`). Run [mg-udp-scan](/wiki/mg-udp-scan) first.
- PDUs are constructed manually as BER-encoded raw bytes. No SNMP library is
  used.
- Both SNMPv1 and SNMPv2c are tested for each community string.
- Default community list: `public`, `private`, `community`, `manager`,
  `snmpd`. Pass `--community-file` to extend it.
- MIB walk is limited to the system and interfaces subtrees by default to
  keep output size manageable.
