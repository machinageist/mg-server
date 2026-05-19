---
title: "GeistScope: Mobile App and Static Analysis"
date: 2026-05-18
summary: "mg-apk, mg-ipa, mg-js-analyze, mg-sourcemap, mg-secret-validate, and mg-csp find vulnerabilities in distributed artifacts without running them."
tags: [rust, security, bug-bounty, geistscope, mobile, static-analysis, apk, ipa]
---

## Artifacts Before Runtime

Applications ship more than a server. iOS and Android binaries, JavaScript bundles,
source maps, and Office documents are all distributed to clients. Each one can contain
information that the server's HTTP responses never reveal: hardcoded credentials,
internal API paths, developer usernames, and sometimes production secrets that made it
into the build.

Six tools analyze distributed artifacts statically, without executing them.

---

## mg-apk: Android Package Analysis

```bash
mg-apk target-bounty --apk /path/to/app.apk
```

An APK file is a ZIP archive. The tool opens it in memory and reads specific files
without extracting to disk.

`AndroidManifest.xml` is scanned for four misconfigurations:
- `android:debuggable="true"`: the application can be debugged with `adb`, allowing
  memory inspection and method hooking at runtime.
- `android:allowBackup="true"`: ADB backup can extract the application's data directory
  without root on older Android versions.
- Exported components (`Activity`, `Service`, `BroadcastReceiver`) declared without a
  `permission` attribute: any application on the device can start or send data to these
  components.
- `network_security_config` referencing a config that sets `cleartextTrafficPermitted="true"`:
  the app communicates over HTTP.

DEX files and the `assets/` directory are scanned as raw bytes for hardcoded strings
matching the secret regex catalog: API keys, tokens, hardcoded passwords, internal
base URLs.

---

## mg-ipa: iOS Application Package Analysis

```bash
mg-ipa --ipa /path/to/app.ipa
```

An IPA file is also a ZIP. The bundle is identified by finding the entry in
`Payload/*.app/` — the app name is the directory name without `.app`.

`Info.plist` is scanned as raw bytes (both binary and XML plist formats appear in
the wild) for App Transport Security exceptions:
- `NSAllowsArbitraryLoads: true`: disables ATS for all connections.
- `NSExceptionAllowsInsecureHTTPLoads: true` for specific domains.
- `NSExceptionMinimumTLSVersion` set below TLS 1.2.

The Mach-O binary is scanned as a strings extraction: any sequence of printable ASCII
longer than 8 characters. The output is run through the secret regex catalog. Internal
paths (`/Users/builduser/`, `/var/jenkins/`), server addresses (`https://internal.corp.`),
and credential patterns are flagged.

---

## mg-js-analyze: JavaScript Bundle Analysis

```bash
mg-js-analyze target-bounty
```

The tool reads the crawl corpus for JavaScript file paths, then fetches each JS file
and runs the regex catalog across the content. Fetches run concurrently via `JoinSet`.

The catalog covers the same secret patterns as `mg-crawl`'s inline extraction but runs
as a dedicated post-crawl pass that also scans for:
- Endpoint paths embedded in the bundle (`/api/v2/internal/`, `/admin/`)
- Source map references in `//# sourceMappingURL=` comments
- Framework-specific patterns (Redux store shape hints, Next.js internal routes)

Findings are deduplicated by `(source_url, finding_type, value)` triple. The same
AWS key appearing in three different bundle files produces one finding.

Source map references found here feed directly into `mg-sourcemap`.

---

## mg-sourcemap: Source Map Extraction

```bash
mg-sourcemap target-bounty
```

Source maps reconstruct the original source code from minified JavaScript bundles.
They're generated during the build process and, when deployed to production accidentally,
expose the entire pre-minification codebase.

The tool finds `.js.map` files from three sources: references found by `mg-js-analyze`,
`<link rel="sourcemap">` tags in crawled HTML, and by probing each `.js` file's URL
with a `.map` suffix appended.

From each source map, it downloads `sourcesContent[]` (the embedded source file
contents) and `sources[]` (the original file paths). The source paths are themselves
a finding: a path like `/home/deploy/myapp/src/services/auth.js` reveals the deployment
user, server directory structure, and project layout.

The extracted source is scanned for secrets and internal paths. Source files contain
comments, variable names, and configuration patterns that the minified bundle obscures.
A database connection string in a comment in the original TypeScript won't survive
to production server-side, but it will survive into the source map if it was there
when the build ran.

---

## mg-secret-validate: Live Secret Verification

```bash
mg-secret-validate target-bounty
```

Finding a string that looks like an AWS access key is not the same as confirming it's
valid. `mg-secret-validate` reads secrets found by `mg-js-analyze`, `mg-sourcemap`,
`mg-apikey`, and `mg-apk`, classifies each by regex, and attempts live validation
against the relevant API.

AWS keys are validated using the STS `GetCallerIdentity` trick: sign a request with
the candidate key and send it to `https://sts.amazonaws.com/`. An `AuthFailure` error
(not `InvalidClientTokenId`) means the key exists and was valid at some point but
is not authorized for STS. An actual response means the key is active and has STS
permissions. Either non-`InvalidClientTokenId` response confirms the key is real.

Other validation endpoints are called similarly for GitHub tokens (check the authenticated
user endpoint), Stripe keys (retrieve the account), and Slack tokens (auth.test).

All secret values are masked to the first 8 characters in output. The finding records
the secret type, the source file where it was found, and the validation result.

---

## mg-csp: Content Security Policy Analysis

```bash
mg-csp target-bounty
```

CSP is the browser's primary defense against XSS. A misconfigured CSP provides
false confidence — it looks like a mitigation is in place while leaving trivial
bypass paths open.

The tool reads CSP from two sources: the stored response headers from the crawl
corpus (preferred, as it's already collected) and a fresh GET to the page if headers
aren't available.

Flags and their severity:
- `'unsafe-inline'` in `script-src` or `default-src`: HIGH. Inline scripts are
  permitted, which defeats CSP as an XSS mitigation entirely.
- `'unsafe-eval'` in `script-src`: MEDIUM. `eval()` and `Function()` can execute
  strings as code.
- Wildcard `*` in `script-src` or `default-src`: HIGH. Scripts can be loaded from
  any origin.
- `data:` URI in `script-src`: MEDIUM. Scripts encoded as data URIs can execute.

When multiple of these are present, the tool suggests bypass chains: for example,
a policy with `'nonce-abc'` but also `'unsafe-inline'` is entirely bypassed by
inline scripts because `unsafe-inline` takes precedence in browsers that don't support
nonces.

---

## Static Analysis in the Workflow

These tools run against artifacts collected during engagement setup and crawling.
`mg-apk` and `mg-ipa` require the application binaries, which are downloaded manually
from the app stores. `mg-js-analyze`, `mg-sourcemap`, and `mg-csp` run from the crawl
corpus automatically. `mg-secret-validate` runs after any tool that produces secret
findings.

Static analysis often produces the fastest high-severity finds in an engagement.
A hardcoded production API key in an Android APK requires no vulnerability to exploit
and no active testing — just reading what was shipped.
