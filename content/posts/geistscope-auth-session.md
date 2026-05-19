---
title: "GeistScope: Auth and Session Testing"
date: 2026-05-17
summary: "mg-jwt, mg-authz, mg-oauth, mg-session-audit, mg-apikey, mg-brute, and mg-csrf cover the full auth surface: token attacks, IDOR, credential stuffing, and cross-site request forgery."
tags: [rust, security, bug-bounty, geistscope, auth, jwt, session, csrf]
---

## The Auth Surface

Authentication and authorization bugs pay well. JWT vulnerabilities, IDOR, and CSRF
have consistently produced medium-to-critical findings across programs. The auth surface
is also broader than it looks: it's not just the login page. It's every endpoint that
makes an access decision, every token that gets issued, every session that can be fixed
or stolen.

Seven tools cover this space.

---

## mg-jwt: Token Analysis and Attack

```bash
mg-jwt target-bounty --token <jwt>
```

The tool decodes the token immediately and prints the header, claims, and signature.
From there, several attack paths are available:

**HMAC brute-force** tries a wordlist against the signature:

```bash
mg-jwt target-bounty --token <jwt> --brute --wordlist /usr/share/wordlists/rockyou.txt
```

Without a wordlist, a built-in list of common JWT secrets is used (`secret`, `password`,
`jwt`, the domain name, and similar). HMAC-SHA256 is implemented directly in the tool
with no JWT library. This avoids library variations in how "none" and algorithm
confusion attacks are handled.

**Algorithm confusion** tests RS256-to-HS256: grab the public key from the JWKS
endpoint, sign a modified token with it as the HMAC secret, and submit it. Applications
that don't validate the algorithm field accept this.

**Claim manipulation** modifies the `sub`, `role`, or `exp` fields and re-signs with
a known secret (if brute-force succeeded) or submits unsigned to test for algorithm
acceptance without verification.

---

## mg-authz: IDOR and BOLA Testing

```bash
mg-authz target-bounty --session-a <cookie-a> --session-b <cookie-b>
```

Insecure Direct Object References require two accounts. Session A creates or accesses
resources. Session B, which should not have access, tries to reach the same resources.

The tool replays requests from the crawl corpus under both sessions. When session B
receives a 200 response on a resource created by session A, that's an IDOR finding.
The test covers endpoint parameters that suggest object IDs: numeric IDs in path
segments, UUID patterns, `id=`, `user_id=`, `account=` query parameters.

IDOR findings are written with the specific URL, parameter, and both response bodies
for comparison. The evidence section of the finding includes the full session-A resource
URL and the session-B replay that succeeded.

---

## mg-oauth: OAuth Flow Testing

```bash
mg-oauth target-bounty
```

The tool tests four OAuth weaknesses. First, state parameter validation: it initiates
a flow, intercepts the state value, and tries submitting the authorization code without
it, with a modified value, and with a predictable pattern (incrementing integer, MD5
of the client ID).

Second, `redirect_uri` bypass: common bypass techniques including path traversal
(`/callback/../attacker`), adding a path suffix, open redirect chaining to an attacker
domain, and loose validation where only the domain prefix is checked.

Third, PKCE downgrade: attempting to initiate a flow without a code challenge when
PKCE is supposed to be required.

Fourth, implicit flow: testing whether the authorization server allows `response_type=token`
when it should only support `code`.

`reqwest` runs with `redirect::Policy::none()` so each `Location` header in the OAuth
flow is inspected rather than followed, which is necessary for detecting redirect URI
issues that only appear in the intermediate redirect.

---

## mg-session-audit: Session Security Properties

```bash
mg-session-audit target-bounty
```

Four checks against the session handling:

**Session fixation**: set a known session ID in the cookie before authenticating, then
check whether the same session ID is preserved after login. If it is, an attacker who
plants a session cookie before the user logs in inherits the authenticated session.

**Cookie flags**: every Set-Cookie header inspected for `Secure`, `HttpOnly`, `SameSite`,
and `Max-Age`. Missing `HttpOnly` means XSS can steal the token. Missing `Secure` means
the cookie transmits over HTTP. These findings compound with other vulnerabilities.

**Token entropy**: the session token is base64 decoded and run through a basic entropy
check. Short tokens or tokens with obvious patterns (incrementing, timestamp-based) are
flagged.

**Logout invalidation**: makes an authenticated request to confirm the session works,
logs out, then replays the same authenticated request. A 200 response after logout means
the session token is still valid server-side.

---

## mg-apikey: Static Secret Detection

```bash
mg-apikey target-bounty
```

`mg-apikey` operates on the crawl corpus without making any network requests. It reads
every file in `crawl/` and runs a regex catalog against the content. The catalog is
compiled once at startup via `OnceLock` and covers AWS keys, GitHub tokens, Stripe
keys, Slack webhooks, Twilio tokens, generic `api_key` and `secret` patterns, and
several others.

Matched secrets are masked to the first 8 characters in the output to avoid full
exposure in findings files. The full value is available in the raw crawl file if
needed for validation.

This is similar to what `mg-crawl` does during the crawl phase, but runs post-hoc
against any crawl corpus, including ones that have been augmented manually.

---

## mg-brute: Credential Testing

```bash
mg-brute target-bounty --login-url https://target.example.com/login
```

When no credential lists are provided, the tool uses built-in lists of common
username and password combinations. Custom lists are loaded with `--userlist` and
`--passlist`.

Lockout detection watches for 429 and 403 responses. When lockout is detected, the
tool pauses and logs the condition rather than continuing and locking out all tested
accounts.

Successful credentials are printed to stdout immediately when found and written to
`findings/` as a CRITICAL finding. The finding includes the URL, the successful
credential pair (password masked), and the response evidence.

---

## mg-csrf: Cross-Site Request Forgery

```bash
mg-csrf target-bounty
```

The tool first identifies state-changing endpoints: POST, PUT, PATCH, DELETE requests
from the crawl corpus. For each, it checks three things: whether a CSRF token is
present in the form or request, whether the session cookie has a `SameSite` attribute,
and whether the server validates the `Origin` header.

Severity depends on the combination:

- No CSRF token and no `SameSite` cookie: HIGH. Both mitigations are absent.
- One mitigation present, one absent: MEDIUM.
- Both present: informational note only.

Origin validation is tested by replaying the state-changing request with a modified
`Origin` header pointing to an external domain. If the server accepts it, the
Origin check is not a real mitigation.

---

## Auth Testing in Sequence

These tools work best run in order: `mg-jwt` and `mg-apikey` first (static analysis,
no traffic), then `mg-session-audit` (single authenticated session), then `mg-authz`
(two-session comparison), then `mg-csrf` and `mg-oauth` (flow testing). Credentials
found by `mg-brute` or secrets found by `mg-apikey` feed directly into subsequent
tests. An API key found in the crawl corpus might unlock additional endpoints that
become new targets for [injection testing](/wiki/geistscope-injection-testing) or
IDOR testing.
