---
title: "GeistScope: SSRF, OOB Callbacks, and Cloud Metadata"
date: 2026-05-18
summary: "mg-oob, mg-ssrf, mg-aws, mg-gcp, mg-azure, and mg-serverless turn a single SSRF into a full credential extraction chain against cloud instance metadata endpoints."
tags: [rust, security, bug-bounty, geistscope, ssrf, cloud, aws, oob]
---

## SSRF as a Starting Point

Server-Side Request Forgery is often listed as a medium severity finding when the
impact is "can reach internal services." In cloud environments, the actual impact
is usually much higher. A single SSRF that can reach the instance metadata endpoint
can yield an IAM role's access key and secret, which may have permissions across
the entire cloud account.

Five tools build the chain: an OOB callback listener, an SSRF tester, and three
cloud-specific metadata extraction tools. A sixth covers serverless runtime metadata.

---

## mg-oob: The Out-of-Band Listener

```bash
mg-oob target-bounty --port 8080
```

`mg-oob` runs an HTTP listener that assigns a unique 32-character hex token to each
test. When an injection causes the target server to make an outbound request (DNS
lookup, HTTP callback), the listener records the callback.

Callbacks are written to `oob/callbacks-<timestamp>.json` in the engagement directory.
Other tools that send blind payloads read this directory to correlate findings. A
command injection payload containing `curl http://<oob-host>:<port>/<token>` confirms
code execution when the callback arrives.

```bash
# In one terminal
mg-oob target-bounty --port 8080

# In another, running injection tests
mg-cmdinject target-bounty --oob-url http://your.server:8080
mg-ssrf target-bounty --oob-url http://your.server:8080
```

The listener handles SIGINT cleanly and writes a final summary before exiting.

---

## mg-ssrf: Testing Injectable Points

```bash
mg-ssrf target-bounty --oob-url http://your.server:8080
```

The tool reads endpoint and parameter data from `crawl/endpoints.json`. Parameters
that suggest URL handling (`url`, `endpoint`, `callback`, `webhook`, `src`, `href`,
`redirect`, `fetch`) are the injection candidates.

Three categories of SSRF payloads are injected:

**Cloud metadata URLs**: the IMDS addresses for AWS, GCP, and Azure. If the target is
running on a cloud provider and the application fetches the injected URL, the response
might contain instance metadata.

**OOB callback URLs**: unique tokens that confirm the application made an outbound
request even when the response isn't reflected in the HTTP response body.

**Localhost targets**: `http://127.0.0.1:80`, `http://localhost:8080`, and port ranges
commonly hosting internal services. Internal SSRF that reaches admin interfaces or
unauthenticated internal APIs is separately impactful.

After testing, the tool reads the `oob/` directory and correlates callbacks with the
requests that triggered them to produce confirmed SSRF findings.

---

## mg-aws: IMDSv2 Credential Extraction

```bash
mg-aws target-bounty --ssrf-url https://target.example.com/fetch?url=
```

Given a confirmed SSRF endpoint, `mg-aws` chains requests through it to extract
AWS credentials.

IMDSv2 requires a token: first a PUT to `http://169.254.169.254/latest/api/token`
with `X-aws-ec2-metadata-token-ttl-seconds: 21600`. The response is the token.
Then a GET to the metadata endpoint with `X-aws-ec2-metadata-token: <token>`.

When the SSRF endpoint doesn't forward custom headers (common with URL-fetching proxies),
the tool falls back to the IMDSv1 path, which uses a plain GET with no token.

The chain extracts: IAM role name (from `/latest/meta-data/iam/security-credentials/`),
then `AccessKeyId`, `SecretAccessKey`, and `Token` from the role credentials endpoint.
Sensitive fields are masked to the first 4 characters in the finding output.

---

## mg-gcp: GCP Metadata Service

```bash
mg-gcp target-bounty --ssrf-url https://target.example.com/fetch?url=
```

GCP's metadata endpoint (`http://metadata.google.internal/computeMetadata/v1/`) requires
the `Metadata-Flavor: Google` header. The tool attempts to pass this header through
the SSRF vector. Many vulnerable URL-fetching implementations forward all request
headers from the injected URL response, but some proxy implementations strip custom
headers.

When the first request with `Metadata-Flavor: Google` fails, the tool retries without
it. Some GCP deployments allow metadata access without the header in certain
configurations.

The extraction chain covers: project ID, instance zone, service account email, and
the OAuth2 access token for the default service account. The access token can be used
directly against GCP APIs if it has sufficient permissions.

---

## mg-azure: Azure IMDS and Managed Identity

```bash
mg-azure target-bounty --ssrf-url https://target.example.com/fetch?url=
```

Azure Instance Metadata Service lives at `http://169.254.169.254/metadata/instance`
and requires the header `Metadata: true`. Like GCP, the tool attempts header forwarding
and records whether it succeeded.

Beyond the base IMDS, `mg-azure` probes the managed identity token endpoint:
`http://169.254.169.254/metadata/identity/oauth2/token?resource=https://management.azure.com/`.
A successful response yields a bearer token for Azure Resource Manager.

The `userData` field is also checked. Azure allows arbitrary data to be stored in
userData, and some deployments store configuration or credentials there for easy
retrieval by the instance.

---

## mg-serverless: Lambda, Cloud Functions, Azure Functions

```bash
mg-serverless target-bounty --ssrf-url https://target.example.com/fetch?url=
```

Serverless runtime metadata differs by platform. AWS Lambda exposes credentials
via environment variables, not IMDS: `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`,
`AWS_SESSION_TOKEN` are accessible at the runtime environment. These aren't reachable
via SSRF to IMDS, but if there's any code execution or environment variable leakage,
they're present.

For Lambda, GCP Cloud Functions, and Azure Functions, the tool appends known runtime
metadata endpoints as query parameter values to the SSRF URL and checks for structured
JSON responses. The specific endpoint paths for each runtime are in the tool's constant
catalog.

---

## The Full Chain

The SSRF tools are designed to chain: confirm SSRF exists with `mg-ssrf`, identify
the cloud provider from `recon/summary.json` or Shodan data, then run the appropriate
metadata tool against the confirmed SSRF endpoint. In the best case, a single SSRF
vulnerability yields a cloud role's temporary credentials, which elevates the finding
to critical severity and unlocks further access to cloud resources.

The OOB listener is a prerequisite for blind SSRF. Running `mg-oob` before any SSRF
testing and keeping it alive throughout the engagement means blind callbacks from
multiple tool runs are all captured in the same directory for correlation.
