# mg-server — agent context

Orientation for an agent starting cold on this repo. Everything here traces to a file in
the tree; where a claim comes from another repo the path is given.

**Written 2026-08-14.** If something below contradicts the code, the code wins — say so
and fix this file.

---

## 1. What this is

`mg-server` is the Rust/Axum application that serves **machinageist.dev**. The repo is the
development source for the site, **not** the deployment server (`IMPROVEMENT_PLAN.md`,
"Corrected scope").

The site is an **evidence-first portfolio** aimed at getting Jeff Cincoski hired as an
**infrastructure technician — Linux, networking, and virtualization** (`README.md` §Overview).
It is organized around **four pillars**: a Proxmox homelab, networking, Linux / SysAdmin, and
a small defensive-security section. A fifth "certification journey" pillar existed once and
was retired 2026-07-25 — do not rebuild it (`docs/public-portfolio-structure.md` header note).

Target readers, in the order the site is built for them:

1. A hiring manager or recruiter skimming for thirty seconds.
2. An engineer peer checking whether the writing survives scrutiny.
3. A self-directed learner using `/learn`.

The role targets themselves were re-ranked 2026-08-02 around a ~$117k comp floor — remote
Linux / infrastructure support first, on-site NOC and data-center roles kept warm at Tier 2.
See `IMPROVEMENT_PLAN.md` §"Target role support" and
`~/mg-coreforge/bootcamp/career/PUBLIC_FACE.md`.

The application is also, honestly, a learning artifact for Rust and the backend ecosystem.
It is deliberately small. It is not a platform.

---

## 2. Read these, in this order

| File | Why |
|---|---|
| `README.md` | Public-facing description, stack table, deployment request path, headers |
| `IMPROVEMENT_PLAN.md` | Claim posture, safe/dangerous claims, the ops-writeup backlog. Carries an amendment block at the top — read it |
| `docs/public-portfolio-structure.md` | Audience, pillars, evidence standard, claim discipline. Carries its own 2026-07-25 amendment header |
| `gauntlet-output/criteria.md` | The five quality lenses and the three auto-fail rules any new work is graded against |
| `gauntlet-output/feature-tree.md` | Every feature of the site, with current state (implemented / absent / model-only) |
| `src/router.rs` | The single source of truth for routes |
| `~/mg-coreforge/bootcamp/CERT_PLAN.md` | The live certification spine. Internal only — nothing from it goes on the site without checking §5 below |
| `~/mg-coreforge/bootcamp/career/PUBLIC_FACE.md` | Wording rules for anything public |

---

## 3. Stack and layout

Axum + Askama + flat-file Markdown. No database, no runtime template engine, no build step
for the frontend. Content is read from `content/` at request time.

```text
src/
  main.rs                 # startup, tracing init, bind
  router.rs               # ALL routes + middleware order — single source of truth
  state.rs                # AppState: request counters, uptime, RSS; Status snapshot
  errors.rs               # SiteError + the themed 404/500 fallbacks
  handlers/
    pages.rs              # home, about, portfolio
    blog.rs               # /blog list (grouped by pillar) + /blog/:slug
    wiki.rs               # /learn index + pages; hardcoded SIDEBAR lives here
    releases.rs           # /releases
    status.rs             # /status (human) + /status.json (machine)
    well_known.rs         # security.txt, robots.txt
  middleware/
    security_headers.rs   # CSP, HSTS, Permissions-Policy, Referrer-Policy, nosniff, DENY
    rate_limit.rs         # governor token bucket, 60 req/min, per-instance not per-IP
    vitals.rs             # request + per-route counters, inside the limiter
  models/
    markdown.rs           # pulldown-cmark rendering
    page.rs               # content/pages/*.md  -> Page
    post.rs               # content/posts/*.md  -> BlogPost
    project.rs            # hardcoded portfolio entries + the anti-overclaim test
    lab.rs                # curated lab list — TRACKED BUT NOT COMPILED, see §9
templates/                # Askama; base.html is the shell (nav, theme menu, footer, vitals)
static/css/style.css      # the whole stylesheet
static/js/                # ~80 lines total: theme selector only
content/posts/            # published blog posts
content/pages/            # the /learn wiki
content/drafts/           # unrouted, never served
docs/                     # planning docs, theme generator, solarcore brand spec
gauntlet-universal/       # the portable spec pipeline
gauntlet-output/          # this project's criteria, feature tree, specs, scorecards
tests/wiki_pages.rs       # drift guard for the /learn wiki
```

### Routes

`src/router.rs` is the authority. As of this writing:

`/` · `/about` · `/portfolio` · `/blog` · `/blog/:slug` · `/learn` · `/learn/:slug` ·
`/wiki` and `/wiki/:slug` (permanent redirects to `/learn`, kept so old links work) ·
`/releases` · `/status` · `/status.json` · `/.well-known/security.txt` · `/security.txt` ·
`/robots.txt` · `/static/*` (ServeDir) · fallback 404.

Middleware order matters and is documented in the file header: `TraceLayer` outermost, then
the rate limiter, then vitals counting (deliberately inside the limiter so a throttled flood
cannot inflate counters), then `add_security_headers` on the way out.

### Themes

23 themes, generated from one Python file. `docs/themes/generate_themes.py` is the single
source of truth for the CSS token blocks, the JS `MODES` array and icon map, and the
`base.html` menu buttons. Editing any of those four registries by hand is how they drift.
`--check` runs a WCAG contrast audit and is the first step in CI.

---

## 4. Claim discipline — the part that actually matters

The site's core asset is that everything on it is true and defensible in an interview
(`gauntlet-output/criteria.md`, Lens 1). A single overclaim costs more than a hundred design
nits. Four boundaries:

1. **No unearned certification claims.** No cert claim without a booked exam voucher. None is
   booked. `README.md:14-16` records the 2026-07-25 removal.
2. **No offensive-security identity.** No red-team, pentest, bug-bounty, or SOC-analyst
   framing. GeistScope is gated behind a publication requirement (full pipeline + human and AI
   operation + sanitized evidence from an authorized engagement) and currently has exactly one
   public artifact, a retrospective.
3. **No production-grade / SRE / enterprise / high-availability language.** See
   `IMPROVEMENT_PLAN.md` §"Claim posture" for the explicit dangerous-claims list.
4. **No planned thing written as a shipped thing.** Distinguish
   implemented / prototyped / planned / gated / absent in user-visible copy.

**These are encoded as tests, not just prose.** Do not weaken or delete them to make a change
pass:

| Test | File | What it pins |
|---|---|---|
| `portfolio_only_carries_entries_with_verifiable_status_and_evidence` | `src/models/project.rs` | `all().len() == 1`; rejects "Homelab", "GeistScope", "Certification track", "bug-bounty", "red-team", "offensive security" |
| `home_page_shows_concrete_work_without_strategy_narration` | `src/handlers/pages.rs` | Rejects "infrastructure-support", "in training", "evidence-first", "security engineer", "offensive security", "red-team" |
| `about_page_describes_work_plainly_without_disclaimers` | `src/handlers/pages.rs` | Rejects a "What I am not claiming yet" section and the same identity strings |
| `labs_never_claim_offensive_or_unearned_identity` | `src/models/lab.rs` | Rejects "SOC analyst", "penetration test", "red team", "HackerOne", "bug bounty", "Hack The Box", "HTB" — **but see §9, this file is not compiled** |

The site copy voice follows from this: quiet, show-don't-tell. No strategy narration, no
disclaimer sections. The tests enforce the absence of both.

---

## 5. The /learn wiki: three-place registration

A wiki page must be registered in **three** places or a test fails:

1. `content/pages/<slug>.md` — the file itself.
2. `SIDEBAR` in `src/handlers/wiki.rs` — the navigation entry, inside the right
   `SidebarSection`.
3. `WIKI_SLUGS` in `tests/wiki_pages.rs` — the drift guard's copy of the list.

The duplication between 2 and 3 is deliberate: it keeps the test crate decoupled from the
bin. `tests/wiki_pages.rs` enforces it in both directions —
`every_wiki_slug_has_a_parseable_page` catches a sidebar entry with no file, and
`no_orphaned_wiki_pages_on_disk` catches a file with no sidebar entry. **Adding a Markdown
file to `content/pages/` and nothing else breaks the build.**

---

## 6. The page-authoring contract

Observable in every file under `content/pages/` except `index.md`, which is the overview page
and exempt.

**Frontmatter** (YAML, between `---` fences, first thing in the file):

```yaml
---
title: "The OSI model"
date: 2026-07-23
summary: "One sentence a reader can scan."
tags: [education, networking, osi, tcp-ip, troubleshooting]
---
```

`title:` is asserted by the test. Then, in order:

- `## Overview` — what this is and why it exists, in ordinary language.
- Body sections — prose first. Concepts are built from the ground up before jargon and
  connected to the larger system. Tables and lists support the prose; they do not replace it.
  Bullet-dumping a source note scores ≤ 1 against `criteria.md` 2C.
- `## Suggested practice: <specific thing>` — something the reader can actually do, on
  hardware they already own, with FOSS tools.
- `## Related pages` — cross-links to other `/learn/<slug>` pages, each with a clause saying
  why you would follow it.
- `## Sources and further reading` — names the source textbook, then the primary sources that
  were checked against it.

**The source line is a fixed form.** Networking pages:

> This page was edited from my own study notes, taken from Ian Neil's CompTIA Network+
> certification guide, and checked against the primary sources:

Linux pages use Brian Ward's *How Linux Works* the same way. Then the primary sources — RFCs
(linked to `rfc-editor.org`), ISO/IEC standards, man pages, kernel docs — each with a short
gloss on what it actually specifies. The textbook is the study source; the primary source is
the check. Both get named.

**On citing a Network+ book while not sitting Network+.** Network+ was dropped from the
certification spine 2026-08-02. The `/learn` networking pages still cite Ian Neil's Network+
guide and should keep doing so — it is the book the notes come from, and roughly 60% of that
material is CCNA content, which is on the live spine. Citing a textbook is not claiming a
credential. What is forbidden is copy that presents Network+ as an exam Jeff is sitting.
(The `network-plus` *tags* that used to sit on those pages were a different matter — a tag
pill reads as a credential claim rather than a citation — and were removed in `9664566`.)

---

## 7. Verification

Exactly what CI runs (`.github/workflows/`), in order:

```sh
python3 docs/themes/generate_themes.py --check   # WCAG contrast audit across all 23 themes
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo build --release
```

Run locally with `RUST_LOG=info cargo run` — binds `127.0.0.1:3000`, no database or external
service needed. `MG_BIND_ADDR` overrides the bind address.

`criteria.md` 5D requires any spec to name these commands. Name them.

---

## 8. The gauntlet

Two directories, different jobs:

- **`gauntlet-universal/`** — the portable pipeline itself. `GAUNTLET.md` is the procedure:
  Phase 0 discovers the project and interviews the user to build quality criteria, then spec
  agents write one document per feature, blind verification agents grade them against the
  criteria, and failures loop through up to three remediation passes. Plus three templates.
  Nothing in here is mg-server-specific. Invoked via the `gauntlet` skill or by reading
  `gauntlet-universal/GAUNTLET.md`.
- **`gauntlet-output/`** — this project's instance. `criteria.md` (five weighted lenses, three
  auto-fail rules), `feature-tree.md` (13 features, A/B/C tiers), `manifest.md` (per-feature
  status and score), `specs/`, `scorecards/`, `gap-reports/`.

A scorecard passing does not mean its Priority 1 items were applied — the manifest tracks
"Applied" and "Outstanding" separately, and several features still carry outstanding items.

---

## 9. Known stale documents and open conflicts

Do not trust these blindly.

**Stale or superseded**

- `docs/public-portfolio-structure.md` — the body still describes **five** pillars and a
  Network+ → Security+ → Linux+ → Server+ spine. The amendment header at the top corrects the
  pillar count, but the header's own cert line ("Current plan: Network+ then RHCSA") is itself
  stale: it predates the 2026-08-02 re-lock. Live spine: **RHCSA → CCNA → Security+**.
- `docs/REORG_HANDOFF_PROMPT.md`, `docs/REORG_CHANGELOG.md` — written against the five-pillar,
  four-CompTIA world. Historical record, not instructions.
- `~/.claude/security/CERTS.md` — flagged stale in the global CLAUDE.md. Do not act on it.
- `~/tech-skill-up/` — referenced by `README.md` and older docs. **The directory no longer
  exists.** The curriculum and labs are at `~/mg-coreforge/bootcamp/`.
- `docs/solarcore/SOLARCORE_SPEC.md` — contradicts the shipped site in at least five places
  (theme polarity, the CRT/scanline ban, the split wordmark, `--sc-*` token names, the magenta
  role). Resolved direction, Jeff 2026-08-07: **the shipped site wins**, and the spec's job is
  to be rewritten to match. See `criteria.md` 2A.

**Live conflicts, unresolved**

- **`src/models/lab.rs` is tracked by git but never declared in `src/models/mod.rs`.** It does
  not compile and its three tests — including the anti-overclaim guard — never run. The
  feature tree calls `C4 progress` "model only, unwired"; this is the concrete shape of that.
  Anyone wiring up a `/labs` route inherits the test suite the moment they add `pub mod lab;`.
- **`gauntlet-output/manifest.md` currently contains unresolved git conflict markers**
  (`<<<<<<< HEAD` / `=======` / `>>>>>>>`) in both the status table and the correction-pass
  table. Read the lower half (the `fc3da33…` side) as current.
- **Open decision, `manifest.md` §"Open decision for Jeff":** may the home page name RHCSA?
  `criteria.md` auto-fail rule 1 says no cert claim without a voucher; `PUBLIC_FACE.md:15-23`
  was loosened 2026-08-03 to allow naming RHCSA *with its status attached*. The B1 spec took
  the conservative path (name no exam on `/`). Overrulable, nothing blocked on it.
- **Outstanding scorecard Priority 1 items** for A2 (site-shell), A3 (ops), and B3
  (portfolio). Citations in those scorecards are known-stale and have been re-verified
  against source in `gauntlet-output/REMEDIATION-BRIEF.md` — read that rather than the
  scorecards' own line numbers. B5 (learn) is closed.

**Recently closed** — listed because older docs and scorecards still describe them as open:

- The stale `network-plus` / `ccna` / `rhcsa` tags on the `/learn` pages were dropped
  (`9664566`). Learn pages carry topic tags only; the source textbook is named in the
  Sources section instead, which is a citation rather than a credential claim. This closed
  B5's only Priority 1 item.
- The "working through the CompTIA stack" bio and both "CompTIA study" meta descriptions were
  replaced (`0ef3568`), and the `criteria.md` 5C reference case — `home_page_shows_concrete_work…`
  asserting `html.contains("CompTIA")` when "CompTIA" appeared only in `<meta description>` —
  was fixed in the same commit by splitting on `<main>` and asserting against the body.

---

## 10. Conventions

**Rust file headers.** Every source file opens with a four-field block comment, and the
`Description`/`Notes` fields are expected to explain the non-obvious:

```rust
// Author:      machinageist
// Date:        YYYY-MM
// Description: What this file does and why it exists.
//
// Notes:       Non-obvious context — invariants, tradeoffs, what would break.
```

**Section dividers** sit above the block they describe, never trailing:

```rust
// -----------------------------------------------------------------------
// Data types
// -----------------------------------------------------------------------
```

**Function comments** are a `// Verb + noun` fragment above the function, no period.
4-space indent. `ALL_CAPS_SNAKE_CASE` consts instead of inline magic strings — `PAGES_DIR`,
`OVERVIEW_SLUG`, `HOME_POST_COUNT`, `STATIC_PREFIX` are the pattern.

**Docs.** Plain, specific, no marketing. When a long-lived document's claims stop being true,
amend it in place with a dated header block rather than quietly rewriting it — that is what
`docs/public-portfolio-structure.md` and `IMPROVEMENT_PLAN.md` both do, and it is how the next
agent can tell what changed and when.

**Commits.** Small working commits, split by logical change; push once at the end.
