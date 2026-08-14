# mg-server Improvement Plan

> ## ⚠️ AMENDED 2026-08-14 — the certification spine and the pillar count
>
> Two claims in this document went stale. Both are corrected inline below; the amendment
> is recorded here so the corrections are not silently reverted.
>
> - **The cert spine.** This plan used to call a four-CompTIA spine (Network+ → Security+ →
>   Linux+ → Server+, targeted January 2027) the site's "visible through-line." The live
>   spine, re-locked 2026-08-02 in `~/mg-coreforge/bootcamp/CERT_PLAN.md`, is
>   **RHCSA (EX200) → CCNA (200-301) → Security+ (SY0-701)**, and **RHCSA is the only
>   pre-employment exam** (targeted ~Nov–Dec 2026). Network+ was dropped, Linux+ was
>   superseded by RHCSA, and Server+ was dropped.
> - **The pillar count.** Pillar 5, the certification journey, was retired from the site
>   2026-07-25 (`docs/public-portfolio-structure.md` header note). The site has **four**
>   pillars, which is what `README.md` says. Do not rebuild a cert route or a cert card.
>
> **No voucher is booked for any exam.** `gauntlet-output/criteria.md` auto-fail rule 1
> rejects any spec that introduces a certification claim without a booked voucher, and
> criterion 1D scores stale cert copy 0. `README.md:14-16` records the 2026-07-25 removal
> of cert claims from the site. Public wording rules live in
> `~/mg-coreforge/bootcamp/career/PUBLIC_FACE.md` (loosened 2026-08-03): RHCSA may be named
> as *actively studying*, provided the copy also says it is not earned and no date is
> booked; CCNA and Security+ may be named only as planned or next; nothing may appear in a
> format that implies possession.
>
> **Studying from a Network+ book is not a Network+ claim.** The `/learn` networking pages
> cite Ian Neil's CompTIA Network+ certification guide as their source textbook, and they
> should keep doing so — it is the book the networking notes are taken from, and
> `CERT_PLAN.md` puts roughly 60% of that material inside CCNA, which is on the live spine.
> Citing a textbook is not claiming a credential. What is forbidden is presenting Network+
> as an exam Jeff is sitting, which he is not.
>
> **Path change:** `~/tech-skill-up/` no longer exists. The curriculum, labs, and evidence
> system live at `~/mg-coreforge/bootcamp/`. Paths below have been updated.

Copied into this repository from the `tech-skill-up` portfolio/curriculum planning context
(now `~/mg-coreforge/bootcamp/`).

Current status: development repository for the public `machinageist.dev` portfolio site.

## Corrected scope

This repository is not the deployment server. Treat it as the source for a public-facing professional portfolio. The immediate work is site structure, homepage/about/portfolio copy, article taxonomy, and publishable artifact paths that showcase the skills being built in `~/mg-coreforge/bootcamp/`.

Operational/deployment writeups still matter, but only as public portfolio artifacts when they are written in a hiring-manager-readable way with evidence, limits, and claim-defense notes. Do not spend the first pass on private deployment-server runbooks unless they directly improve public structure or content.

## Purpose

`mg-server` should become the professional public hub for the `~/mg-coreforge/bootcamp/` evidence system, organized around four pillars: a homelab & Proxmox operations lab, networking, Linux / SysAdmin, and a small defensive-security section. The homelab + networking writeups are the content engine, and the visible through-line is the evidence itself — an operated system explained, broken, and verified — not a list of exams.

The certification spine is real but internal. It is **RHCSA → CCNA → Security+** (re-locked 2026-08-02), with RHCSA the only pre-employment exam. It shapes *what gets studied and labbed*, which is why the ops writeups below still map to cert lanes; it does not get its own site pillar, route, or card, and no exam is named on the site while no voucher is booked.

Do not position this repo as proof of advanced Rust backend engineering, production SRE maturity, high availability, enterprise cloud architecture, or DevOps seniority. The strongest honest story is:

> Building and publishing evidence-backed Linux/infrastructure portfolio artifacts: a Proxmox homelab, Linux service operations, DNS/HTTP/request-path troubleshooting, incident documentation, and a small defensive-security section — each one leaving behind real commands, real output, and a defensible claim.

## Target role support

Re-ranked 2026-07-25 and amended 2026-08-02 by the ~$117k comp floor — see
`~/mg-coreforge/bootcamp/career/PUBLIC_FACE.md`. Remote-at-market is the only funnel that
clears the floor, which is what moved the on-site operations roles down a tier.

Primary:

- Remote Linux / Infrastructure Support Engineer at an all-remote or remote-first company
- Infrastructure Technical Support / Escalation Engineer at an infrastructure or dev-tools vendor
- Systems Administrator (Linux-leaning), remote preferred — local sysadmin pay mostly fails the comp floor

Tier 2, kept warm:

- Data Center Operations Technician
- NOC Technician / Network Operations

Secondary later:

- Security Product Support as the defensive section (headers, TLS, SSH hardening, log/auth detection) matures — gated on capability and evidence, not on an exam
- Systems-to-AI infrastructure learning only after Linux/networking/monitoring/virtualization evidence exists, and only after the first stable role

## How the mg-server ops docs map to the study lanes and labs

The operational writeups below are not generic "infrastructure-support" busywork; each one
feeds a study lane and one of the labs under `~/mg-coreforge/bootcamp/subjects/*/labs/`
(the old single `HOMELAB_PROJECTS.md` was split into per-subject lab files). The whole
mg-server series is itself tracked as
`subjects/web-services/labs/mg-server-evidence-series.md`.

- Architecture / request-path + DNS/HTTP triage (§1) → the **networking** lane, now aimed at **CCNA** rather than Network+, and `subjects/networking/labs/proxmox-network-foundation.md`.
- Security headers + TLS + SSH hardening + auth-log detection (§6) → the **defensive-security** lane and **Security+** (employed-time, last on the spine), and `subjects/security-defensive/labs/hardening-and-monitoring.md`.
- systemd runbook, 502→`203/EXEC` incident, deployment/rollback, ops-gap and backup analysis (§2–§5, §7) → the **Linux systems** lane and **RHCSA**, and `subjects/linux-systems/labs/fleet-backup-automation.md`.

One honest limit on that last mapping: mg-server runs on Debian, so the systemd, journald,
and service-operations work transfers conceptually to RHCSA but is not RHEL-toolchain
practice. `CERT_PLAN.md` is explicit that the lab set does not yet cover SELinux, LVM,
podman, or `dnf`, and that the RHCSA rep loop must be run separately on a scratch
Rocky/AlmaLinux VM. Do not let mg-server writeups stand in for that.

The mg-server artifacts are the narrow, already-underway slice; the fuller evidence for
each lane comes from the homelab labs themselves.

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
