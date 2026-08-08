# mg-server / machinageist.dev — Feature Tree

**Generated:** 2026-08-07
**Scope decision (Jeff):** full site — existing routes are spec'd alongside new work.
**Status:** awaiting confirmation

Discovered from `src/router.rs` (the single source of truth for routes),
`src/handlers/`, `src/models/`, `templates/`, and the repo's planning docs.

---

## A. Foundation

Cross-cutting concerns every content surface depends on. Spec'd first because the
others reference their decisions.

| ID | Feature | Current state | Notes |
|---|---|---|---|
| `A1` | **design-system** | implemented, spec stale | Token architecture, type scale, 23-theme roster, brand mark. **Blocking job:** rewrite `SOLARCORE_SPEC.md` to match shipped reality, then layer best practices. See criteria 2A. |
| `A2` | **site-shell** | implemented | `base.html`, nav, theme selector (`main.js`, `theme-init.js`), footer, vitals strip, skip link, 404/500 pages. |
| `A3` | **ops-and-observability** | implemented | Security headers, rate limiting, vitals middleware, `/status` + `/status.json`, `security.txt`, `robots.txt`. |

## B. Content surfaces (existing)

| ID | Feature | Current state | Notes |
|---|---|---|---|
| `B1` | **home** | implemented | Hero, Lately, Latest writing teaser, Learn pointer. |
| `B2` | **about** | implemented | Bio, "What I work with", Further out. Carries the stale CompTIA copy (criteria 1D). |
| `B3` | **portfolio** | implemented, deliberately minimal | One entry (`mg-server`). Anti-overclaim test pins `len() == 1`. |
| `B4` | **writing** | implemented | `/blog` grouped by pillar, `/blog/:slug`. Four posts. |
| `B5` | **learn** | implemented | `/learn`, `/learn/:slug`, hardcoded `SIDEBAR` allowlist, 12 pages across Networking and Linux Foundations. |
| `B6` | **releases** | implemented | `/releases` + source tarballs under `static/releases/`. |

## C. New capabilities

| ID | Feature | Current state | Notes |
|---|---|---|---|
| `C1` | **search** | absent | No index exists. Server-rendered `/search?q=` over `content/posts/` + `content/pages/` fits the no-JS floor. |
| `C2` | **glossary** (branch) | absent | Two children, spec'd in one document. |
| `C2a` | glossary-terms | absent | Definitions of terms across networking and Linux. |
| `C2b` | glossary-commands | absent | Common commands with usage and context. |
| `C3` | **study-tools** (branch) | absent | Three children. Progressive enhancement is an auto-fail gate here. |
| `C3a` | flashcards | absent | Server-rendered card flow first; JS sharpens it. |
| `C3b` | multiple-choice | absent | Form POST → graded result page as the no-JS baseline. |
| `C3c` | pbq-simulations | absent | Performance-based question simulations. Hardest no-JS problem in the tree. |
| `C4` | **progress** | model only, unwired | `src/models/lab.rs` exists untracked: 12 curated labs, `LabStatus`, three tests. No handler, route, or template. Surfaces spectre-seq and rhelings as *progress*, not portfolio claims. |

---

## Dispatch plan

Per `GAUNTLET.md`: leaf → one agent; branch with ≤ 3 children → one agent covers
parent and children in one document.

| Spec agent | Covers |
|---|---|
| 1 | `A1` design-system |
| 2 | `A2` site-shell |
| 3 | `A3` ops-and-observability |
| 4 | `B1` home |
| 5 | `B2` about |
| 6 | `B3` portfolio |
| 7 | `B4` writing |
| 8 | `B5` learn |
| 9 | `B6` releases |
| 10 | `C1` search |
| 11 | `C2` glossary (parent + 2 children) |
| 12 | `C3` study-tools (parent + 3 children) |
| 13 | `C4` progress |

**13 specs.** Each gets a blind verification pass, and any failure gets up to
three remediation loops. Concurrency capped at 2–3 per `GAUNTLET.md`'s
usage-limited guidance.

**Ordering:** A before B and C — `A1`'s design-system reconciliation is an input
to every visual decision downstream, and `A2`'s shell decisions constrain every
page spec.

---

## Explicitly out of scope

- `content/drafts/` — staged, unpublished writing.
- Legacy `/wiki` redirects — covered under `B5`, not their own feature.
- Deploy pipeline (Caddy, Cloudflare Tunnel, systemd) — infrastructure, not a
  product feature. Referenced by `A3` where it affects observable behavior.
