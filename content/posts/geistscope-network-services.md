---
title: "GeistScope: Network Service Auditing"
date: 2026-05-18
summary: "mg-tls-scan, mg-ssh-audit, mg-smtp, mg-snmp, mg-smb, and mg-http2 audit network services that web application scanners ignore entirely, using raw protocol implementations."
tags: [rust, security, bug-bounty, geistscope, network, tls, ssh, protocol]
---

## What the Web Scanner Misses

Web application testing focuses on HTTP. But ports 25, 161, 445, and 22 are also in
scope on most programs. A misconfigured SMTP relay, a default SNMP community string,
or a weak SSH algorithm set are real findings, and they require protocol-level testing
that reqwest and standard HTTP libraries can't do.

Six tools cover network services using raw protocol implementations, not library
abstractions.

---

## mg-tls-scan: Certificate and Cipher Audit

```bash
mg-tls-scan target-bounty
```

The tool performs TLS handshakes directly using `rustls 0.23` and `tokio-rustls 0.26`,
not through `reqwest`. This matters because reqwest abstracts away the handshake
details that need inspection. Certificate parsing uses `x509-parser` against the
raw DER bytes.

Three categories of findings:

**Certificate issues**: expired certs, self-signed certs (no chain to a trusted CA),
wildcard certs that might cover more than intended. The expiry check uses the
`notAfter` field from the DER-parsed cert, not the handshake summary.

**Cipher suite weakness**: the handshake negotiation is logged and checked against a
catalog of deprecated suites. RC4, 3DES, NULL ciphers, and export-grade ciphers flag
as HIGH. SHA-1 in the signature algorithm flags as MEDIUM.

**Missing SNI**: a handshake without a Server Name Indication extension. Some servers
serve a default certificate when SNI is absent, which can reveal internal hostnames
or serve a certificate intended for a different vhost.

All HTTPS hosts from `recon/summary.json` are tested by default. Specific hosts can
be targeted directly.

---

## mg-ssh-audit: Algorithm Negotiation

```bash
mg-ssh-audit target-bounty
```

SSH auditing happens at the key exchange negotiation level, before authentication.
The tool opens a raw TCP connection to port 22 and sends an RFC 4253 §7.1 KEXINIT
message. The server's KEXINIT response includes its supported algorithm lists:
key exchange algorithms, host key types, encryption ciphers, and MAC algorithms.

No authentication is required and no credentials are needed. The entire audit runs
from the KEXINIT exchange.

Deprecated algorithms that trigger findings:
- `diffie-hellman-group1-sha1` and `diffie-hellman-group14-sha1` for key exchange
- `ssh-rsa` as a host key type (SHA-1 based)
- `arcfour`, `arcfour128`, `arcfour256` (RC4) for encryption
- `hmac-md5` and `hmac-sha1` for MACs

The server's SSH version string is also extracted from the initial handshake banner
and cross-referenced for known CVEs associated with that version.

---

## mg-smtp: Mail Server Vulnerabilities

```bash
mg-smtp target-bounty
```

The tool only runs against hosts with port 25 or 587 open, as identified in
`recon/summary.json`. For each, it connects via raw TCP and tests three vulnerabilities:

**VRFY/EXPN user enumeration**: `VRFY username` asks the server to verify whether
an address is deliverable. Many servers respond differently for valid and invalid
users, leaking account existence. `EXPN mailinglist` expands a mailing list to its
members. Both are MEDIUM findings when the server responds helpfully.

**Open relay**: the test sends a `RCPT TO:<external-address@gmail.com>` after a
`MAIL FROM:<test@attacker.example.com>`. A server that accepts this is relaying mail
for external domains, which is an open relay. Open relay is a HIGH finding: the server
can be used to send spam and the organization's IP reputation suffers.

**Header injection**: `MAIL FROM` is sent with a CRLF injected in the value. A server
that accepts it without stripping the CRLF allows mail headers to be injected into
outgoing messages.

---

## mg-snmp: Community String Brute-Force

```bash
mg-snmp target-bounty
```

SNMP probing uses manually constructed BER-encoded PDUs. No SNMP library. The BER
encoding is straightforward for GET-REQUEST: sequence header, version integer, community
string, request ID, error status, error index, variable bindings.

SNMPv1 and SNMPv2c are both tested. The tool brute-forces a list of community strings:
`public`, `private`, `community`, `manager`, the target organization name, and any
names identified from WHOIS or OSINT. When a valid community string is found, it runs
an OID walk starting from the root (`.1.3.6.1`) to enumerate everything the agent
exposes.

A successful SNMP OID walk from community string `public` commonly yields network
interface details, routing tables, ARP caches, installed software lists, and system
description strings including OS version. This is an INFO-to-MEDIUM finding depending
on what the walk returns.

Only hosts with UDP port 161 open (from `mg-udp-scan` output) are targeted.

---

## mg-smb: Null and Guest Session Testing

```bash
mg-smb target-bounty
```

SMB2 over port 445. The tool checks two misconfigurations without attempting any
credential exploitation:

**SMB signing requirement**: a negotiate protocol response that doesn't require signing
is flagged as MEDIUM. Without required signing, an attacker on the network path can
perform NTLM relay attacks.

**Null session**: an SMB session authenticated with empty username and password. If the
server accepts the null session and allows any share or IPC$ access, it's a finding.
Null sessions on domain controllers have historically allowed anonymous user enumeration.

**Guest session**: the same test with `username="Guest"` and empty password. Guest
access to file shares is a HIGH finding when shares contain sensitive content.

All connections are raw TCP. No Windows-specific libraries or SMB client tools required.

---

## mg-http2: HTTP/2 Misconfiguration

```bash
mg-http2 target-bounty
```

Four checks against HTTP/2 support:

**h2c upgrade**: sends an HTTP/1.1 request with `Upgrade: h2c` and `Connection: Upgrade`.
If the server responds with `101 Switching Protocols`, cleartext HTTP/2 is supported,
which may have different routing and security properties than the HTTPS version.

**ALPN negotiation**: checks whether the server advertises `h2` in the TLS ALPN
extension. Absence doesn't indicate a bug but is noted in the output.

**Rapid reset heuristic**: the server's HTTP/2 implementation version is extracted
from response headers or the SETTINGS frame. Versions known to be affected by
CVE-2023-44487 (HTTP/2 Rapid Reset) are flagged as INFO, noting the version, not
confirming the vulnerability.

**HPACK header size**: sends a request with large headers to test whether the server
enforces the negotiated header table size limit. Accepting headers significantly
exceeding the negotiated limit can indicate implementation bugs.

---

## Source of Targets

All six tools read `recon/summary.json` for their target lists. `mg-tls-scan` runs
against HTTPS hosts. `mg-ssh-audit` and `mg-smb` run against hosts with the relevant
port open. `mg-smtp` runs against mail servers. `mg-snmp` runs against hosts with UDP
161 open from `mg-udp-scan` output.

Running [mg-udp-scan](/wiki/geistscope-recon-expansion) before `mg-snmp` is a requirement,
not a suggestion — there's no way to know which hosts are running SNMP without probing
UDP first.
