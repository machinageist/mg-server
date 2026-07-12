# mg-server reorg — CLI agent handoff prompt

> Paste everything below the line into the Claude Code CLI agent running inside the
> `mg-server` repo. It is self-contained but expects read access to two sibling repos on
> the same machine: `~/tech-skill-up/` (career/curriculum source of truth) and
> `~/geistscope/` (the old security-tool codebase this site over-documents).
>
> This file was generated 2026-07-10 from a full inspection of the repo. If the code has
> moved on since then, trust the repo over this file and note the drift.

---

## Role and mission

You are reorganizing `machinageist.dev` (served by this `mg-server` Rust/Axum app) so it
reads as the portfolio of someone moving **into Systems Administrator / NOC Technician
work**, with a **homelab + networking** blog as the content engine and a **four-CompTIA-cert
journey** as the visible through-line. Today the site is dominated by GeistScope — an
earlier security-scanner project — which now works against that story. Your job is to
de-prioritize GeistScope, restructure the blog/projects/about, and align the public
identity with the refined goals, **without inventing evidence and without breaking the
build or tests.**

Work in **small, verified slices** and commit per phase. After every phase run
`cargo fmt`, `cargo clippy`, `cargo build`, and `cargo test` and keep them green.

## Read these first (source of truth)

In this repo:
- `IMPROVEMENT_PLAN.md`, `docs/public-portfolio-structure.md` — the *previous* pivot pass.
  Useful, but they aim at a "Data Center / Remote Hands / generic infrastructure-support"
  identity that is now **too broad**. You will update them (Phase 6).
- `src/router.rs`, `src/handlers/*.rs`, `src/models/*.rs`, `templates/*.html`,
  `content/posts/*.md`, `content/pages/*.md`.

In `~/tech-skill-up/` (the authoritative career plan — do not duplicate, link/point to it):
- `ROADMAP.md` — four-cert spine + SysAdmin/NOC goal, AI-infra pivot deferred.
- `CERT_PLAN.md` — dated Network+ → Security+ → Linux+ → Server+ schedule (Jan 2027).
- `HOMELAB_PROJECTS.md` — the **three homelab projects** that should anchor the blog and
  portfolio: (1) Internal DNS + network map [Network+], (2) Harden & monitor the homelab
  [Security+], (3) Proxmox backup/restore + monitoring + incident log [Server+/Linux+].
  Each has a "safe résumé claim" and an "overclaim to avoid" — reuse that discipline.

Also load the local skills if available: `mg-server`, `rust-security-patterns`.

## Current state (verified 2026-07-10 — re-confirm, don't trust blindly)

- **Stack:** Axum + Askama (compile-time templates) + pulldown-cmark + gray_matter, flat-file
  content, Caddy + Cloudflare Tunnel + Proxmox Debian VM. Tests live inline (`#[cfg(test)]`).
- **Routes** (`src/router.rs`): `/`, `/start-here`, `/about`, `/portfolio`, `/blog`,
  `/blog/:slug`, `/wiki`, `/wiki/:slug`, `/releases`, well-known. Nav (in `templates/base.html`)
  shows: Start Here · About · Portfolio · Writing · Reference. (`/releases` is intentionally
  not in nav; a test asserts that.)
- **Blog** (`content/posts/*.md`, `src/models/post.rs`): every post needs YAML frontmatter
  with **all** of `title`, `date` (YYYY-MM-DD), `summary`, `tags: [..]` or it fails to load.
  `blog.rs` loads all posts newest-first. There is **no category/tag routing yet** — tags are
  parsed but not displayed or filterable.
  - ~24 posts total. ~22 are `geistscope-*` devlogs. Non-geistscope: `blog-draft-mg-server.md`,
    `memory-safety-c-vs-rust.md`, `port-scanner-in-rust.md`.
- **Wiki** (`content/pages/*.md`, `src/handlers/wiki.rs`): ~85 pages, nearly all `mg-*`
  GeistScope security modules (mg-xss, mg-sqli, mg-ssrf, mg-recon, mg-fuzz, …). The left-nav
  **`SIDEBAR` is a hardcoded `const` in `wiki.rs`** enumerating these tools by section — pruning
  pages means editing that const too. `content/pages/index.md` is already reframed as a
  "GeistScope reference archive."
- **Projects** are a **hardcoded `Vec` in `src/models/project.rs::all()`** (5 entries), already
  partly reframed toward infrastructure-support, still listing GeistScope.
- **Templates + handler `description()`/copy** were reframed toward "Data Center / NOC /
  Remote Hands / infrastructure-support." **Inline tests pin these strings** (see
  `src/handlers/pages.rs` and `src/models/project.rs` test modules, e.g. asserting
  `"infrastructure-support portfolio"`, `"Data Center"`, and asserting absence of `"red-team"`,
  `"offensive security"`, `"bug-bounty"`). When you change public copy you **must update these
  tests in the same commit**, keeping the anti-overclaim guard assertions.

## Target positioning (recommended — Jeff asked me to advise)

**Lead identity: "Systems Administrator / NOC Technician (in training)."** Rationale: it maps
exactly to the two primary target roles in `~/tech-skill-up/ROADMAP.md`, it's what the homelab
and cert work actually support, and it's broad enough to cover both without the older copy's
over-broad "Data Center / Remote Hands" co-lead (those become *secondary/fallback*, not the
headline). If Jeff prefers a NOC-first or Linux-first headline instead, that's a one-line hero
change plus the matching section headers — leave a comment noting the swap point.

**Positioning pillars (use as blog categories and About structure):**
1. **Homelab & Proxmox** — the operations lab (node baseline, VM fleet, backup/restore, monitoring).
2. **Networking** — DNS, subnetting/VLANs, request-path, CLI diagnostics (the Network+ lane).
3. **Linux / SysAdmin** — systemd, journald, users/permissions, service ops, small automation.
4. **Security (small, defensive)** — headers, SSH hardening, TLS, log/auth detection. A real but
   compact section, **built to grow later** (Jeff wants the door open past Security+).
5. **Certification journey** — an honest running thread: Network+ → Security+ → Linux+ → Server+
   by Jan 2027, each cert tied to a homelab project. Great narrative glue; don't claim a cert as
   passed until it actually is.

**Tone: honest, humble, evidence-first.** "In training / building toward" is a strength here, not
a weakness. Never fabricate command output, logs, dates, or a passed cert. Use project **status**
(`InProgress`/`Active`) honestly; if a homelab artifact isn't captured yet, mark it planned.

## Decisions locked by Jeff (do not re-litigate)

1. **GeistScope:** *Keep* the wiki pages that correspond to **still-active parts of
   `~/geistscope/`**; prune/archive the rest. **Compress the ~22 `geistscope-*` devlog posts into
   ONE honest retrospective post** that describes the whole project and frames it candidly as an
   **early experiment with AI-assisted coding** (what it was, what was real vs aspirational, why
   the scope was too broad, what you learned, and the pivot). Delete the individual devlog posts
   after the retrospective captures them.
2. **Wiki / "Reference" section:** keep it as a **small archive only** — the retained (active)
   GeistScope pages plus the reframed archive index. Homelab/networking content lives in the
   **blog**, not the wiki.
3. **Security:** a **small dedicated defensive section**, with structure that can expand later.
4. **Lead identity:** your call — use the recommendation above unless Jeff says otherwise.

## Phased work plan

Commit after each phase (conventional commits, e.g. `refactor(site): …`). Keep build+tests green.

### Phase 0 — Recon and inventory (no writes yet)
- Read `~/tech-skill-up/{ROADMAP,CERT_PLAN,HOMELAB_PROJECTS}.md`.
- In `~/geistscope/`, determine which tool modules are **actually implemented/active** vs
  aspirational stubs (check the source tree, `Cargo.toml`/subcommands, README, tests). Produce a
  short classification: for each `content/pages/mg-*.md`, mark **KEEP (active)** or **ARCHIVE/REMOVE
  (dead/aspirational)**. Save this as `docs/geistscope-page-triage.md` so the pruning is reviewable.
- List the current posts and classify: keep / rewrite / fold-into-retrospective / delete.
- Post the plan back to Jeff (or write it to `docs/`) before destructive edits if anything is ambiguous.

### Phase 1 — Identity & copy
- `templates/index.html` (hero): lead with SysAdmin/NOC-in-training + homelab/networking + cert
  journey. CTA to Portfolio and Writing.
- `templates/about.html` + `src/handlers/pages.rs::about()` bio: rewrite role focus to the five
  pillars; security as a **supporting** competency; honest "what I'm not claiming yet." Keep the
  location line current and city-only; do not publish relocation plans.
- `templates/start_here.html`: reorient reviewer paths + evidence standard to the new pillars and
  the cert spine; keep the non-claims block.
- Handler `description()` strings in `pages.rs`, `blog.rs`: update to the new framing.
- `templates/base.html` nav: keep it lean (Start Here · About · Portfolio · Writing · Reference).
  Consider renaming "Reference" label to "Archive" to signal its demoted role (optional).
- **Update the pinned-string tests** in `pages.rs` to the new copy; keep the guard assertions that
  block `red-team` / `offensive security` / `security engineer` first-person framing.

### Phase 2 — Portfolio / projects (`src/models/project.rs`)
- Reorder and rewrite `all()` to lead with the work that matters now:
  1. **Homelab operations** (the three `HOMELAB_PROJECTS.md` projects — can be one card or three;
     use honest `InProgress`/`Active` status and the safe-claim wording from that file).
  2. **mg-server** (this site — narrow Linux-service / self-hosting artifact; keep it modest).
  3. **Certification track** (optional card: Network+ → Server+ by Jan 2027, links to writeups).
  4. **GeistScope** — demote to a **single honest line**: "early AI-assisted-coding security-tooling
     experiment; scope narrowed, archived as reference." Status `Complete` or `InProgress` (pruning),
     not a lead artifact.
- Drop the older "Data Center / Remote Hands" generic cards unless backed by real homelab evidence.
- Update the `project.rs` test to assert the new lead framing (homelab/NOC/SysAdmin/cert) and keep
  the anti-overclaim guards.

### Phase 3 — Blog restructure (`content/posts/`, `templates/blog_list.html`, `blog.rs`)
- **Write the GeistScope retrospective** (`content/posts/geistscope-retrospective.md`) per Decision
  1 — one honest post; then **delete the ~22 `geistscope-*` devlog files**. Preserve any genuinely
  useful technical nugget by folding it into the retrospective, not as a separate post.
- Keep `memory-safety-c-vs-rust.md` and `port-scanner-in-rust.md` (general/security-adjacent, fine);
  lightly reframe tags/intro if needed. Turn `blog-draft-mg-server.md` into a finished "How
  machinageist.dev is hosted" post if evidence supports it (see `IMPROVEMENT_PLAN.md` §1/§9).
- **Seed the new pillars** with post stubs *only where real evidence exists or is imminent* — do not
  fabricate. Good first real posts map to the homelab projects: internal DNS + network map
  (Network+), harden & monitor (Security+). Mark anything not yet done as draft/planned rather than
  publishing empty claims.
- Introduce lightweight **categorization**: the frontmatter already has `tags`. Either (a) surface
  tags in `blog_list.html` and group/label by the five pillars, or (b) add a simple `category`
  frontmatter field + optional `/blog?category=` or section headings. Keep it simple; compile-time
  templates + flat files, no DB. Update `post.rs` frontmatter struct + `load_all` if you add a field
  (remember: a missing required field breaks loading of every existing post — make new fields
  `Option<_>` or backfill all files).
- Rewrite the `blog_list.html` intro and `blog.rs::description()` to the homelab/networking/cert
  framing.

### Phase 4 — Wiki → small archive (`content/pages/`, `src/handlers/wiki.rs`)
- Prune `content/pages/mg-*.md` to the **KEEP (active)** set from Phase 0. Delete or move the rest.
- **Edit the hardcoded `SIDEBAR` const in `wiki.rs`** to list only the retained pages — every removed
  slug must also be removed from `SIDEBAR`, or the nav links 404. Add a test or a build-time check
  that every `SIDEBAR` slug has a matching file in `content/pages/` (cheap guard against drift).
- Reframe `content/pages/index.md` as a concise honest archive index ("GeistScope — archived
  reference; active tooling lives in `~/geistscope`"). Point primary reviewers back to Portfolio/Writing.
- Optional: keep a flat list of removed slugs (or a catch-all note) so old links degrade gracefully;
  link-rot on a personal archive is acceptable — prioritize honesty over preservation.

### Phase 5 — Security section (small, extensible)
- Add a compact defensive-security surface: e.g. a `security` tag/pillar in the blog plus a short
  "Security notes" grouping. Seed with what's real: the mg-server **security-headers** writeup
  (headers already exist in `src/middleware/security_headers.rs` — document them with real
  `curl -I` output), and reframe the handful of defensively-relevant old wiki pages (e.g.
  `mg-tls-scan`, `mg-ssh-audit`, `mg-csp`, `mg-session-audit`) as **defensive** notes if kept.
- Structure it so it can grow after Security+ without a redesign (a pillar/tag, not a bespoke route).
- Keep everything defensive and owned-scope; no offensive/red-team first-person framing.

### Phase 6 — Update the planning docs
- Rewrite `IMPROVEMENT_PLAN.md` and `docs/public-portfolio-structure.md` so their stated audience is
  **SysAdmin / NOC (primary)**, homelab + networking content engine, cert spine, small security
  section — replacing the older "Data Center / Remote Hands generic infrastructure-support" framing.
  Keep the excellent claim-defense discipline; just retarget it.
- Update `README.md` to describe the site's purpose in the new terms (it currently says "primarily a
  learning platform for backend development and the Rust ecosystem" — fine to keep the learning
  framing, but add the portfolio/identity purpose and the honest scope).

### Phase 7 — Verify
- `cargo fmt --check`, `cargo clippy -- -D warnings` (or repo's lint bar), `cargo build`, `cargo test`.
- Run locally and click through `/`, `/start-here`, `/about`, `/portfolio`, `/blog`, each kept post,
  `/wiki`, each kept wiki page. Confirm **no internal link points at a deleted slug** (grep the
  templates, `index.md`, `SIDEBAR`, and post bodies for removed `geistscope-*` / `mg-*` slugs).
- Confirm the pinned-copy tests reflect the new identity and the anti-overclaim guards still pass.
- Summarize what changed, what was deleted, and any TODOs/decisions left for Jeff.

## Non-negotiables (guardrails)

- **Never fabricate** command output, logs, timestamps, metrics, screenshots, or a passed
  certification. If evidence isn't captured yet, mark the artifact planned/in-progress.
- **Honest GeistScope framing:** an early AI-assisted-coding experiment that over-scoped and got
  pruned — that self-awareness is the value. Don't quietly erase it; don't inflate it.
- **Claim discipline:** reuse the safe-claim / overclaim-to-avoid pattern from
  `~/tech-skill-up/HOMELAB_PROJECTS.md` and the existing `IMPROVEMENT_PLAN.md` claim-defense sheet.
  No "production-grade," "SRE," "HA," "enterprise," "secured the app," or offensive-identity language.
- **Keep the build and tests green after every phase.** Askama is compile-time: a template/field
  mismatch is a build error. A missing required frontmatter field breaks *all* posts.
- **Small commits**, one phase each, conventional-commit messages.
- **When genuinely unsure** (which geistscope parts are active, whether a homelab artifact exists,
  identity headline), **write the question into `docs/` and ask Jeff** rather than guessing or
  fabricating.

## Definition of done

- Homepage, About, Start Here, Portfolio lead with SysAdmin/NOC + homelab/networking + cert journey;
  security present but compact.
- Blog is organized around the five pillars; the 22 geistscope devlogs are replaced by one honest
  retrospective; remaining posts are real.
- Wiki is a small archive of only the active/kept GeistScope pages; `SIDEBAR` matches files; index
  reframed.
- `project.rs` leads with homelab/cert work; GeistScope demoted to one line.
- Planning docs + README retargeted; pinned-copy tests updated; `fmt`/`clippy`/`build`/`test` green;
  no dangling internal links.
- A short changelog + open-questions list handed back to Jeff.
