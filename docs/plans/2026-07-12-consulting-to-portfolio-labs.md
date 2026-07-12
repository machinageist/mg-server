# Consulting Ideas to Portfolio Labs — Review Draft

> Planning only. Do not implement from this document until Jeff reviews the parallel Claude changes, revises this draft, and explicitly approves implementation.

## Goal

Turn a small, credible subset of the current `jeffcincoski-consulting` offer into repeatable labs that produce honest SysAdmin/NOC portfolio evidence while also rehearsing work Jeff may later deliver for clients.

This is not a plan to advertise untested services as finished capabilities. Each lab moves through: safe lab build → break/fix exercise → evidence capture → plain-language handoff → public writeup → only then a portfolio claim.

## Selection rules

A lab belongs in the first portfolio wave only when it:

- supports the current Homelab, Networking, Linux/SysAdmin, or defensive-security pillars;
- can run entirely on owned hardware, test accounts, or synthetic data;
- produces visible evidence beyond screenshots of a happy path;
- includes rollback, recovery, and maintenance notes;
- rehearses a current consulting deliverable without pretending a lab is client work;
- can be explained to a household or solo operator without jargon;
- avoids formal pentesting, compliance, legal advice, emergency response, and credential custody.

## Proposed lab collection

### Lab 1 — The Calm Home Network

**Consulting source:** Home Systems Reset; Network + Device Upgrade Plan; Router + Wi-Fi Tuneup.

**Portfolio fit:** Networking + Network+.

**Question:** Can Jeff inventory an accidental network, define trust zones, improve its configuration, and leave a map another person can understand?

**Safe environment:** A dedicated homelab router/AP or isolated test VLAN. Do not disrupt the household production network during the first run.

**Build:**

- capture hardware, firmware, uplink, subnet, DHCP, DNS, SSIDs, and representative test devices;
- draw before/after physical and logical maps;
- define trusted, guest, IoT, and administration zones, using the safest supported fallback when VLANs are unavailable;
- establish naming and configuration-backup conventions;
- validate DHCP, DNS, internet access, local access, and intended isolation.

**Break/fix drill:** Introduce one reversible DNS or DHCP fault, diagnose from a client outward, restore service, and record the decision path.

**Evidence:** Sanitized configuration excerpts, topology diagrams, test matrix, real command output, fault timeline, rollback proof, and a plain-language household handoff.

**Potential writeup:** “Turning an Accidental Home Network into a System.”

**Safe claim:** Built and documented an isolated home-network lab with trust zones, validation checks, and a reversible DNS/DHCP troubleshooting exercise.

**Do not claim:** Enterprise network design, zero trust, or a completed client engagement.

### Lab 2 — Smart Device Side Yard

**Consulting source:** Smart Device Side Yard; IoT safety; Pi-hole / DNS Filter Install.

**Portfolio fit:** Networking + defensive security + Security+.

**Question:** How much practical isolation and telemetry reduction can a small network achieve without breaking the household’s normal device flows?

**Safe environment:** Test VLAN/guest network with a streaming device, printer, or synthetic endpoints. Capture DNS only for owned test devices.

**Build:**

- place test IoT devices in the least-trusted supported segment;
- deploy Pi-hole or AdGuard Home with conservative lists;
- define an explicit bypass and recovery path;
- test DNS resolution, internet access, casting/discovery where relevant, administration boundaries, and allowlist behavior;
- record blocked-query counts only as operational observations, not proof that all tracking is prevented.

**Break/fix drill:** Block a dependency required by a test device, identify the responsible rule from logs, add the narrowest exception, and verify recovery.

**Evidence:** Zone diagram, DNS flow, sanitized logs, allowlist rationale, before/after query samples, functional test matrix, and disable procedure.

**Potential writeup:** “A Side Yard for Smart Devices: Isolation Without Pretending IoT Is Simple.”

**Safe claim:** Tested practical IoT segmentation and DNS filtering on owned devices, including a documented break/fix and bypass path.

**Do not claim:** Anonymity, complete tracker blocking, or universal IoT security.

### Lab 3 — Private Access, No Random Open Ports

**Consulting source:** Work-From-Anywhere Access; Private Access Quickstart.

**Portfolio fit:** Networking + Linux/SysAdmin + defensive security.

**Question:** Can Jeff provide remote access to one lab service while keeping inbound router exposure closed and preserving revocation/recovery controls?

**Safe environment:** A disposable Linux VM and one off-network test client. Use synthetic files and test identities.

**Build:**

- baseline listening services and external exposure;
- deploy Tailscale as the primary path, with WireGuard reserved for a later comparison;
- restrict access to one intended service and identity/device set;
- verify access on LAN, off-network, after logout/revocation, and after service restart;
- document enrollment, removal, recovery, and disable procedures.

**Break/fix drill:** Revoke a test device or introduce a reversible policy error, prove access fails, diagnose it, and restore only the intended path.

**Evidence:** Access-path diagram, `ss`/firewall checks, policy excerpts, off-network test results, revocation proof, and recovery runbook.

**Potential writeup:** “Remote Access Without a Port-Forwarding Habit.”

**Safe claim:** Built and validated identity-aware private access to a lab service, including device revocation and recovery tests.

**Do not claim:** A VPN product, enterprise IAM, or universally secure remote access.

### Lab 4 — The Restore Is the Product

**Consulting source:** Backup + Restore Drill; Account, Password & Backup Baseline; local media/personal cloud.

**Portfolio fit:** Homelab/Proxmox + Linux/SysAdmin + Server+.

**Question:** Can Jeff recover something meaningful under a stated recovery objective, rather than merely report that a backup job completed?

**Safe environment:** Disposable VM plus synthetic documents/photos. No personal or client data in the public artifact.

**Build:**

- define the dataset, backup boundary, schedule, retention, RPO target, and RTO target;
- run a Proxmox VM backup and a file-level backup path;
- record destination capacity, permissions, and failure visibility;
- restore one file and one disposable VM or service;
- verify content integrity and service usability after restore.

**Break/fix drill:** Remove or corrupt a synthetic source file, recover it from the intended restore point, and record elapsed time and surprises.

**Evidence:** Sanitized job configuration, timestamps, checksums, restore transcript, measured RPO/RTO, capacity notes, and rollback/cleanup record.

**Potential writeup:** “A Green Backup Job Is Not a Restore Test.”

**Safe claim:** Designed and executed file and VM restore drills in a Proxmox lab with measured recovery objectives.

**Do not claim:** Disaster recovery, business continuity, or guaranteed data protection.

### Lab 5 — A Personal Cloud with an Exit Door

**Consulting source:** Media + Personal Cloud That You Control.

**Portfolio fit:** Linux/SysAdmin + storage + service operations.

**Question:** Can a locally controlled file/media service remain understandable, maintainable, and exportable rather than becoming a new dependency trap?

**Safe environment:** Synthetic media and documents on a disposable VM/container with test storage.

**Build:**

- compare Jellyfin plus simple file sync/share against a heavier all-in-one cloud stack;
- choose the smallest architecture that meets a defined test household’s needs;
- configure storage layout, permissions, service lifecycle, private access, backup, and update notes;
- prove files can be exported and the service removed without losing the canonical data.

**Break/fix drill:** Stop the service or detach a disposable data path, diagnose the failure, restore service, and verify media/file integrity.

**Evidence:** Decision record, architecture diagram, systemd/container status, permissions notes, restore/export test, maintenance calendar, and decommission steps.

**Potential writeup:** “Local-First Without Building a Tiny Cloud Company at Home.”

**Safe claim:** Evaluated and operated a maintainable local file/media service with tested backup, export, and decommission paths.

**Do not claim:** A production cloud platform or maintenance-free self-hosting.

### Lab 6 — Quiet Home Ops Box

**Consulting source:** Home Ops Copilot (Hermes).

**Portfolio fit:** Linux/SysAdmin + automation, after the first five labs.

**Question:** Can a low-power automation host do a few useful jobs with bounded permissions, visible logs, explicit approvals, and a physical/logical off switch?

**Safe environment:** Dedicated test mini-PC or VM; synthetic reminders and read-only checks; no client credentials or sensitive household control.

**Build:**

- baseline OS, service account, updates, storage, time sync, and resource use;
- configure three narrow workflows: one reminder, one read-only service-health report, and one backup-status summary;
- deny side-effecting tools by default and document any approval gate added later;
- document logs, update procedure, failure notification, pause, disable, and recovery.

**Break/fix drill:** Stop one dependency or provide malformed test input, confirm the workflow fails visibly and safely, then recover it.

**Evidence:** Permission matrix, process/service status, resource measurements, scheduler records, redacted logs, failure behavior, and kill-switch test.

**Potential writeup:** “A Home Ops Box That Knows Its Boundaries.”

**Safe claim:** Operated a low-power automation host with constrained workflows, visible failure handling, and tested disable/recovery controls.

**Do not claim:** Autonomous household management, safe handling of every sensitive workflow, or a client-ready product.

## Recommended sequence

1. **Calm Home Network** — establishes the map and network baseline.
2. **Smart Device Side Yard** — adds segmentation and DNS filtering.
3. **Private Access** — reaches the services without new public exposure.
4. **The Restore Is the Product** — proves recovery before adding more services.
5. **Personal Cloud with an Exit Door** — applies the baseline to a user-facing service.
6. **Quiet Home Ops Box** — automates only after the underlying operations are understood.

The order is deliberate: map → isolate → access → recover → serve → automate.

## Standard lab packet

Every lab should eventually produce these private working artifacts before a public article exists:

- `README` with purpose, boundary, and prerequisites;
- before-state inventory;
- architecture or request-path diagram;
- change plan and rollback trigger;
- validation matrix with expected and actual results;
- break/fix incident note;
- sanitized evidence index;
- plain-language operator/household handoff;
- maintenance and decommission notes;
- resume-safe claim and dangerous overclaim;
- public-writeup outline with redaction checklist.

No fabricated terminal output, timestamps, client scenarios, or performance numbers. If a result is reconstructed, label it as a reconstruction; if it is synthetic, label it as synthetic.

## Proposed future portfolio shape

After evidence exists, the portfolio could distinguish:

- **Labs** — controlled exercises on owned systems;
- **Operating artifacts** — real runbooks, incident notes, and site operations;
- **Builds** — tools/services that remain in active use;
- **Client work** — only with permission and appropriately anonymized evidence.

A lab card should show: status, pillar, question, environment, evidence captured, last verified date, and link to the writeup. “Planned” cards should not be visually confused with verified work.

## Review gates before implementation

After Claude finishes the parallel work:

1. Review the current branch diff and reconcile any new portfolio/widget model with this proposed collection.
2. Confirm which consulting modules remain current; the consulting repo is actively changing and its docs currently disagree with parts of the live services page.
3. Select only the first two labs for the initial implementation cycle.
4. Confirm available owned hardware and which network can be safely disrupted.
5. Decide what evidence may be public and what must remain sanitized/private.
6. Convert the approved labs into exact implementation plans with paths, commands, tests, and rollback steps.
7. Implement one lab at a time; publish only after the real evidence review.

## Explicit non-goals for this draft

- No changes to existing plans, application code, templates, styles, routes, project cards, or consulting copy.
- No claim that any proposed lab has been completed.
- No promise that every consulting module belongs on the portfolio.
- No implementation sequencing beyond the review-gated roadmap above.
