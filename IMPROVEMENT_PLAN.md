# mg-server Improvement Plan

Copied into this repository from the `tech-skill-up` portfolio/curriculum planning context.

Current status: development repository for the public `machinageist.dev` portfolio site.

## Corrected scope

This repository is not the deployment server. Treat it as the source for a public-facing professional portfolio. The immediate work is site structure, homepage/about/portfolio copy, article taxonomy, and publishable artifact paths that showcase the skills being built in `~/tech-skill-up/`.

Operational/deployment writeups still matter, but only as public portfolio artifacts when they are written in a hiring-manager-readable way with evidence, limits, and claim-defense notes. Do not spend the first pass on private deployment-server runbooks unless they directly improve public structure or content.

## Purpose

`mg-server` should become the professional public hub for the `~/tech-skill-up/` evidence system, organized around five pillars: a homelab & Proxmox operations lab, networking, Linux / SysAdmin, a small defensive-security section, and an honest certification journey. The homelab + networking writeups are the content engine; the four-CompTIA-cert spine (Network+ → Security+ → Linux+ → Server+, targeted January 2027) is the visible through-line, each cert anchored to a homelab project.

Do not position this repo as proof of advanced Rust backend engineering, production SRE maturity, high availability, enterprise cloud architecture, or DevOps seniority. The strongest honest story is:

> Building and publishing evidence-backed SysAdmin/NOC portfolio artifacts: a Proxmox homelab, Linux service operations, DNS/HTTP/request-path troubleshooting, incident documentation, a small defensive-security section, and a certification journey where each cert leaves behind a real, defensible artifact.

## Target role support

Primary:

- Systems Administrator (junior Linux/Windows)
- NOC Technician / Network Operations

Fallback (only if the primary funnel stalls past the certs):

- Data Center Technician / Operations
- Remote Hands / Rack-and-Stack / Infra Delivery
- Infrastructure-heavy IT Support

Secondary later:

- Security Product Support as the defensive section (headers, TLS, SSH hardening, log/auth detection) grows past Security+
- Systems-to-AI infrastructure learning only after Linux/networking/monitoring/virtualization evidence exists, and only after the first stable role

## How the mg-server ops docs map to the certs and homelab projects

The operational writeups below are not generic "infrastructure-support" busywork; each one feeds a specific cert lane and one of the three homelab projects in `~/tech-skill-up/HOMELAB_PROJECTS.md`:

- Architecture / request-path + DNS/HTTP triage (§1) → **Network+** and Homelab Project 1 (internal DNS + network map).
- Security headers + TLS + SSH hardening + auth-log detection (§6) → **Security+** and Homelab Project 2 (harden & monitor).
- systemd runbook, 502→`203/EXEC` incident, deployment/rollback, ops-gap and backup analysis (§2–§5, §7) → **Linux+ / Server+** and Homelab Project 3 (backup/restore + monitoring + incident log).

The mg-server artifacts are the narrow, already-underway slice; the fuller evidence for each cert comes from the homelab projects themselves.

## Claim posture

Safe claim direction:

- Documented and operated an owned self-hosted Rust/Axum website on a Debian VM behind Caddy and Cloudflare Tunnel.
- Diagnosed and documented service behavior using HTTP checks, systemd status, logs, and deployment notes.
- Built conservative runbooks and gap analyses for backups, rollback, logging, monitoring, and service health.

Dangerous claims to avoid:

- production-grade deployment
- SRE / DevOps engineer
- high availability
- enterprise cloud infrastructure
- advanced Rust backend platform
- zero-trust architecture
- complete observability stack
- secure web platform

## Improvement themes

### 1. Architecture and request-path documentation

Document how a visitor reaches the app:

```text
Browser
  -> DNS
  -> Cloudflare
  -> Cloudflare Tunnel
  -> Caddy
  -> Debian VM
  -> systemd service
  -> mg-server Rust/Axum app
```

Required evidence:

- `dig machinageist.dev` or equivalent DNS output
- `curl -I https://machinageist.dev`
- sanitized Caddy/Tunnel/systemd overview
- diagram or sequence table
- failure-mode matrix per layer

Deliverable:

- `docs/architecture.md` or README section: “How machinageist.dev is hosted”

### 2. systemd service runbook

Document operational commands for the service.

Required content:

- unit name and sanitized unit-file explanation
- start/stop/restart/status commands
- log commands
- expected healthy state
- common failure modes
- rollback/recovery notes

Commands/concepts to know cold:

- `systemctl status <unit>`
- `systemctl restart <unit>`
- `journalctl -u <unit>`
- `ExecStart`
- service user/permissions
- binary path
- environment variables without exposing secrets

Deliverable:

- `docs/systemd-runbook.md`

### 3. 502 -> systemd `203/EXEC` incident report

Turn the known incident into a defensible NOC/Linux troubleshooting artifact.

Required content:

- symptom
- impact
- timeline
- first checks
- commands/logs
- root cause: binary-path mismatch / executable issue
- fix
- verification
- prevention
- what remains fragile

Important rule:

- Use real logs/output if available.
- If logs are unavailable, label the writeup as reconstruction or reproduce the failure safely in a local test service.
- Do not invent timestamps, prompts, or command output.

Deliverable:

- `docs/incidents/502-systemd-203-exec.md`

### 4. Deployment tradeoffs

Document the current deployment flow and its limits.

Questions to answer:

- How does code get from local repo/GitHub to the running service?
- Is there a cron/git-pull/rebuild/redeploy flow?
- What does the current approach prevent?
- What risks does it create?
- What happens if build succeeds but restart fails?
- What happens if the new binary is wrong?
- Is there a rollback path?
- How is service health verified after deployment?

Deliverable:

- `docs/deployment.md`

Safe framing:

- “Documented deployment tradeoffs for an owned self-hosted service.”

Avoid:

- “Implemented production CI/CD.”

### 5. Logging, monitoring, backup, and rollback gap analysis

Write an honest operations gap note.

Sections:

- Current logs: what exists, where, how to inspect
- Current monitoring: what exists, what does not
- Current health checks: manual vs automated
- Current backups: what exists, what is untested
- Current rollback: what is possible today
- Current alerting: what exists, what does not
- Next small improvements

Deliverable:

- `docs/ops-gap-analysis.md`

Safe framing:

- “Documented operational gaps and next improvements for logs, monitoring, backups, and rollback.”

Avoid:

- “Implemented monitoring and disaster recovery” unless actually built and tested.

### 6. Security headers and TLS note

Review current HTTP response headers and document defensive improvements.

Required evidence:

- `curl -I https://machinageist.dev`
- current headers
- missing or weak headers
- rationale for each change
- before/after output if changed
- limits of header hardening

Potential headers to understand:

- HSTS
- CSP / `frame-ancestors`
- `X-Content-Type-Options`
- `Referrer-Policy`
- `Permissions-Policy`

Deliverable:

- `docs/security-headers.md`

Safe framing:

- “Reviewed HTTP security headers for an owned web service and documented hardening steps.”

Avoid:

- “Secured the web application.”

### 7. Basic service-health evidence

Build or document the first health-check path.

Minimum manual checks:

- HTTP status
- TLS certificate expiry
- local process/systemd status when on the host
- local listening port when on the host

Possible future tool:

- `mg-health` can consume these checks later and emit JSON/Markdown.

Deliverable:

- `docs/health-checks.md`
- optional sample output under `docs/examples/`

Safe framing:

- “Built or documented basic service-health checks for an owned site.”

Avoid:

- “Built observability platform.”

### 8. README cleanup

The README should make the repo easier for an employer to evaluate without overclaiming.

Recommended README structure:

1. What this is
2. Why it exists
3. Current architecture
4. Local development
5. Deployment overview
6. Operations/runbook links
7. Incident and troubleshooting evidence
8. Known limitations
9. Portfolio/resume-safe framing
10. Dangerous claims this repo does not support

README should point to:

- architecture doc
- systemd runbook
- incident report
- deployment tradeoffs
- ops gap analysis
- security headers note

### 9. Portfolio artifacts supported by this repo

First publishable artifacts:

1. Debugging a 502 into a systemd `203/EXEC` failure
2. How `machinageist.dev` is hosted
3. Caddy and Cloudflare Tunnel request path
4. Deployment script tradeoffs
5. Security headers on `machinageist.dev`
6. What is missing: logs, monitoring, backups, rollback
7. Basic service-health checks for `machinageist.dev`
8. systemd service runbook
9. journald/logging triage note
10. Backup and rollback gap analysis

Each artifact needs:

- real commands/output/logs/configs where possible
- mechanism explanation
- one safe failure/fix or failure-mode analysis
- verification
- cleanup notes if a lab was run
- safe claim
- dangerous overclaim to avoid
- claim-defense sheet

## Near-term work queue

### Slice 1: Document current architecture

- Create architecture diagram or sequence table.
- Capture DNS and HTTP evidence.
- Explain Caddy, Cloudflare Tunnel, systemd, and app boundary.
- Label unknowns.

Done when:

- A reviewer can trace request path from browser to app.
- The doc includes at least one real verification command.

### Slice 2: Create systemd runbook

- Identify unit name and service lifecycle commands.
- Explain healthy and unhealthy states.
- Include log commands.
- Do not expose secrets.

Done when:

- Jeff can explain how to inspect and restart the service without notes.

### Slice 3: Write the 502 incident report

- Reconstruct or reproduce the failure honestly.
- Include root cause, fix, and verification.
- Add prevention ideas.

Done when:

- The incident supports a narrow NOC/Linux troubleshooting claim.

### Slice 4: Document deployment and rollback gaps

- Describe current deployment flow.
- Identify manual steps and risks.
- Define smallest next rollback/health-check improvement.

Done when:

- The doc clearly separates what exists from what is planned.

### Slice 5: Security headers and health checks

- Capture current headers.
- Document missing headers and risks.
- Capture basic health-check commands.
- Avoid broad “secure” or “monitoring” claims.

Done when:

- Header and health-check notes have real command output.

## Suggested docs layout

```text
docs/
  architecture.md
  deployment.md
  systemd-runbook.md
  health-checks.md
  ops-gap-analysis.md
  security-headers.md
  incidents/
    502-systemd-203-exec.md
  examples/
    curl-headers.txt
    health-check-output.md
```

## Claim-defense sheet: mg-server

### Safe resume bullet

Documented and operated an owned self-hosted Rust/Axum website on a Debian VM behind Caddy and Cloudflare Tunnel, including service runbook notes, request-path documentation, and incident/gap analysis.

### Dangerous resume bullet to avoid

Built production-grade Rust cloud infrastructure with SRE-level deployment, monitoring, and high availability.

### What I can honestly claim

- self-hosted service ownership
- basic Linux service operations
- request-path documentation
- reverse-proxy/tunnel awareness
- troubleshooting and incident documentation
- deployment tradeoff analysis

### What I cannot yet claim

- HA
- enterprise deployment
- cloud engineering
- SRE-level reliability
- complete monitoring/alerting
- tested disaster recovery
- advanced Rust backend architecture

### Interview questions this triggers

- Explain the request path from browser to Axum app.
- Where does TLS terminate?
- What does Cloudflare Tunnel do?
- What does Caddy do?
- What happens if the Rust process dies?
- What does systemd `203/EXEC` mean?
- What logs did you check?
- How do you verify service health?
- What is the rollback path?
- What is missing from current monitoring/backups?

### Commands/concepts to know cold

- `dig`
- `curl -I`
- `systemctl status`
- `journalctl -u`
- `ss -tulpn`
- DNS
- TLS
- reverse proxy
- Cloudflare Tunnel
- systemd unit files
- HTTP 502
- backup vs rollback vs HA

### Weak areas to drill

- TLS termination boundaries
- Cloudflare Tunnel limits
- Caddy config details
- rollback strategy
- backup restore testing
- monitoring vs health checks vs alerting
- log retention

### Evidence links needed

- architecture doc
- incident report
- deployment doc
- ops gap analysis
- security headers note
- health-check output

### Claim label

Safe with narrower wording.

## Non-goals for this improvement phase

- Do not add broad new site features before the ops docs exist.
- Do not rewrite the app for architecture vanity.
- Do not claim production-grade reliability.
- Do not claim full observability until monitoring/alerting exists and is tested.
- Do not claim security maturity from headers alone.
- Do not turn this into a generic Rust backend portfolio.

## Definition of done

The improvement plan is successful when:

- README explains the repo clearly and conservatively.
- Request path is documented with real verification.
- systemd operations are documented.
- the 502/`203/EXEC` incident has a defensible report.
- deployment tradeoffs and rollback gaps are explicit.
- security headers are reviewed with real output.
- basic health checks are documented.
- public/resume claims are narrow and backed by evidence.
