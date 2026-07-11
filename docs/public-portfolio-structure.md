# Public portfolio structure

This repository is the development source for `machinageist.dev`. The site should read as a professional, evidence-first portfolio for the skills being built in `~/tech-skill-up/`, not as a claim that this checkout is the deployment server.

## Primary audience

- Systems Administrator (junior Linux/Windows) — primary
- NOC Technician / Network Operations — primary

Fallback audiences (only if the primary funnel stalls past the certs): Data Center Technician / Operations, Remote Hands / Infra Delivery, infrastructure-heavy IT Support. Security Product Support later, after the defensive section grows past Security+.

## Positioning pillars

The site is organized around five pillars — these double as the blog categories and the About structure:

1. Homelab & Proxmox — the operations lab (node baseline, VM fleet, backup/restore, monitoring).
2. Networking — DNS, subnetting/VLANs, request-path, CLI diagnostics (the Network+ lane).
3. Linux / SysAdmin — systemd, journald, users/permissions, service ops, small automation.
4. Security (small, defensive) — headers, SSH hardening, TLS, log/auth detection; built to grow after Security+.
5. Certification journey — Network+ → Security+ → Linux+ → Server+ by January 2027, each cert tied to a homelab project. A cert is claimed only once it is passed.

## Homepage job

The homepage should answer, within one screen:

1. What kind of role is Jeff aiming at? (Systems Administrator / NOC, in training.)
2. What evidence is being built? (The five pillars, homelab-first.)
3. Where should a reviewer click first? (Portfolio and Writing.)
4. What is intentionally not being claimed yet?

Current homepage direction:

- lead with "Systems Administrator / NOC Technician (in training)" plus homelab/networking and the cert journey;
- connect the site to `~/tech-skill-up/` without exposing private local detail beyond the curriculum name/path;
- point to Portfolio and Writing first, with the wiki demoted to an Archive;
- avoid leading with GeistScope, offensive security, red-team, AI, or senior backend identity.

## Core public sections

Recommended public-facing navigation/content groups, mapped to the five pillars:

1. Start Here / About This Portfolio
2. Homelab & Proxmox operations
3. Networking (DNS, request-path, CLI diagnostics)
4. Linux / SysAdmin service operations, runbooks, and incident reports
5. Security (defensive) notes
6. Certification journey
7. Resume Claim Defense

These do not all need to be top-level routes. The first pass uses the existing homepage, About, Start Here, Portfolio, and Blog routes, with the blog grouped by pillar via a `category` frontmatter field, while the wiki is kept only as a small GeistScope archive.

## First content slices

1. Reframe homepage/about/portfolio copy around infrastructure support.
2. Add or revise a Start Here page that explains the portfolio, target roles, evidence standard, and non-claims.
3. Convert the strongest public artifacts into blog/report pages:
   - How `machinageist.dev` is hosted.
   - 502 -> systemd `203/EXEC` incident.
   - DNS/HTTP triage from the CLI.
   - Security headers on `machinageist.dev`.
4. Add a claim-defense block to each artifact.
5. Decide which GeistScope pages remain as reference, which are demoted, and which need pruning from primary navigation.

## Evidence standard

Every resume-facing artifact should include:

- why it matters for the target role;
- starting state;
- target state;
- tools/components;
- commands/configs/scripts used;
- real evidence;
- what broke or could break;
- verification;
- cleanup or rollback notes;
- what Jeff understands now;
- what Jeff still needs to learn;
- resume-safe summary;
- dangerous overclaim to avoid.

## Public claim discipline

Say:

- Systems Administrator / NOC Technician (in training);
- Proxmox homelab operations;
- Linux service operations, systemd/journald;
- networking, DNS, request-path, and CLI diagnostics;
- defensive security fundamentals on owned scope;
- a certification journey (a cert claimed only once passed);
- owned/self-hosted scope.

Do not lead with:

- senior DevOps/SRE;
- production-grade infrastructure;
- enterprise cloud/networking;
- pentesting/red-team identity;
- AI infrastructure engineer;
- advanced Rust backend engineer.

## Current implementation notes

The first corrected slice updates:

- homepage hero and current-focus copy;
- About page role focus, technical range, approach, and non-claims;
- Portfolio intro and project list;
- page metadata descriptions;
- tests that pin the new public positioning and prevent accidental return to offensive/security-engineer-first copy.

## Resolved structure decisions (site reorg, 2026-07)

- Nav label `Reference` renamed to `Archive`; the wiki is now a 10-page GeistScope safe-core archive (see `docs/geistscope-page-triage.md`).
- A dedicated `/start-here` route exists and is the reviewer orientation page (About covers role focus and pillars).
- The GeistScope pages outside the safe core were pruned; old URLs to them 404, an accepted tradeoff (honesty over preservation).
- First polished artifacts published: "How machinageist.dev Is Hosted" and "Security Headers on machinageist.dev", plus the "GeistScope retrospective" replacing 22 devlogs.
- Blog is grouped by the five pillars via a `category` frontmatter field.

## Still open

- When each homelab project accrues real captured evidence, publish its writeup and flip the matching portfolio card from in-progress to a claimed artifact.
- Whether to add per-pillar landing pages or tag-filtered routes later; the current grouped blog list is sufficient for now.
