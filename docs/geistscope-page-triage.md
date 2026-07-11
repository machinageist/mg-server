# GeistScope wiki-page triage

Reviewable classification for the wiki prune (Phase 4 of the site reorg). Every
`content/pages/*.md` file is marked **KEEP** or **REMOVE**. The rule, per Jeff's
locked decision, is: keep only the pages that correspond to the *still-active,
safe-core* parts of `~/geistscope/`; archive/remove the rest.

The source of truth for "active vs aspirational" is GeistScope's own pruning
inventory: `~/geistscope/PRUNING_INVENTORY.md` (dated 2026-07-02) and
`~/geistscope/PRUNING_PLAN.md`. That inventory labels each crate one of
`Keep now`, `Keep later`, `Needs ownership walkthrough`, `Archive`, `Unsafe`.

## KEEP set (the small archive)

These map to the GeistScope safe core (`Keep now` + `Keep later`) — the scope/
workspace, passive checks, reporting, and operator-surface crates that survive
the pruning pass. Ten pages total.

| Wiki page | GeistScope crate | Inventory label | Why kept |
|---|---|---|---|
| `index` | — | — | Reference-archive index (reframed) |
| `mg-engagement` | `engagement` | Keep now | Scope/workspace manager; default-deny core |
| `mg-report` | `mg-report` | Keep now | Evidence/report generation |
| `mg-fingerprint` | `fingerprint` | Keep now | Passive HTTP fingerprinting |
| `mg-probe` | `mg-probe` | Keep now | Passive/report-first posture checks |
| `mg-artifact-audit` | `mg-artifact-audit` | Keep now | Passive static artifact analysis |
| `mg-csp` | `mg-csp` | Keep now | Passive CSP header analysis (defensively relevant) |
| `mg-harness` | `mg-harness` | Keep later | Operator/dispatch surface (trails the core) |
| `mg-tui` | `mg-tui` | Keep later | Operator TUI (trails the core) |
| `libraries` | `session`, `security-graph` | Keep now (libs) | Shared library notes for kept-core crates |

## REMOVE set (archived — pages deleted, slugs dropped from SIDEBAR)

Everything below is labeled `Needs ownership walkthrough`, `Archive`, or `Unsafe`
in the GeistScope inventory. None is public-claim-ready, so none stays as a wiki
page. Link-rot on these archived slugs is acceptable per the reorg decision.

**Active recon/crawl/scan (Needs ownership walkthrough):** `mg-scan`,
`mg-recon`, `mg-crawl`, `subdomain-enum`, `corpus-builder`, `ai-prioritize`,
`mg-probe` is the *only* one of this cluster kept (it's passive).

**Recon expansion / OSINT (Needs walkthrough / PassiveRemote, not core):**
`mg-whois`, `mg-shodan`, `mg-dns-enum`, `mg-dns-history`, `mg-cloud-enum`,
`mg-cname-chain`, `mg-udp-scan`, `mg-github`, `mg-breach`, `mg-google-dork`,
`mg-leak-monitor`.

**Web vuln scanning (Archive — HighActive):** `mg-webscan`, and the retired
single-purpose pages that already redirect to it: `mg-xss`, `mg-sqli`,
`mg-ssrf`, `mg-ssti`, `mg-xxe`, `mg-traversal`, `mg-redirect`, `mg-csrf`,
`mg-cmdinject`, `mg-cors-exploit`, `mg-cache-poison`, `mg-proto-pollute`,
`mg-deser`, `mg-smuggle`.

**Auth & session (Archive/Needs walkthrough — HighActive):** `mg-jwt`,
`mg-authz`, `mg-oauth`, `mg-session-audit`, `mg-brute`.

**OOB & cloud SSRF (Archive — HighActive):** `mg-oob`, `mg-aws`, `mg-gcp`,
`mg-azure`, `mg-serverless`.

**Network services (Archive/Needs walkthrough):** `mg-tls-scan`, `mg-ssh-audit`,
`mg-smtp`, `mg-snmp`, `mg-smb`, `mg-http2`.

**API surface (Needs walkthrough):** `mg-graphql`, `mg-openapi`, `mg-grpc`,
`mg-websocket`.

**Cloud & infra (Archive — HighActive):** `mg-k8s`, `mg-docker`, `mg-vhost`,
`mg-takeover`.

**Mobile/static retired pages (fold into `mg-artifact-audit`, which is kept):**
`mg-apikey`, `mg-apk`, `mg-ipa`, `mg-js-analyze`, `mg-metadata`, `mg-sourcemap`,
`mg-secret-validate`.

**Post-access (Unsafe — Destructive):** `mg-privesc-linux`, `mg-privesc-windows`,
`mg-loot`.

**Workflow (Needs walkthrough):** `mg-diff`, `mg-notify`, `mg-timeline`,
`mg-dns-rebind`.

**Analysis & exploit dev (Archive — AI-assisted exploit surface):**
`mg-recopilot`, `mg-aifuzz`, `mg-exploitgen`.

> Note on Phase 5: the reorg prompt floated reframing `mg-tls-scan`,
> `mg-ssh-audit`, and `mg-session-audit` as *defensive* wiki notes. They are not
> in the GeistScope safe core (all labeled Needs-walkthrough / Archive), so they
> are **not** kept as wiki pages. The defensive-security surface instead lives in
> the blog (the `security` pillar), seeded with the real mg-server security-headers
> writeup. `mg-csp` is the one CSP-analysis page kept because it is passive and
> report-first (`Keep now`), and it doubles as a defensive reference.

## Blog post triage

24 posts today. Classification:

**Fold into one retrospective, then delete (22 `geistscope-*` devlogs):**
`geistscope-engagement-workspace`, `geistscope-recon-pipeline`,
`geistscope-crawl-and-probe`, `geistscope-fuzz-replay-ai`, `geistscope-tui`,
`geistscope-client-libraries`, `geistscope-corpus-builder`,
`geistscope-session-and-payloads`, `geistscope-security-graph`,
`geistscope-harness`, `geistscope-ai-analysis`, `geistscope-injection-testing`,
`geistscope-auth-session`, `geistscope-osint`, `geistscope-recon-expansion`,
`geistscope-network-services`, `geistscope-ssrf-cloud`,
`geistscope-api-surface`, `geistscope-mobile-static`, `geistscope-workflow`,
`geistscope-state-update-may-2026`, `geistscope-webscan-consolidation`.

**Replace them with:** `geistscope-retrospective.md` — one honest post framing
GeistScope as an early AI-assisted-coding experiment that over-scoped and got
pruned; what was real vs aspirational; what was learned; the pivot to SysAdmin/
NOC + homelab evidence.

**Keep (general / security-adjacent, lightly reframe tags/intro only):**
`memory-safety-c-vs-rust`, `port-scanner-in-rust`.

**Finish and publish:** `blog-draft-mg-server` → a real "How machinageist.dev is
hosted" post grounded in the actual stack (Axum + Caddy + Cloudflare Tunnel +
Proxmox Debian VM) with the request-path table; no fabricated command output —
mark any not-yet-captured evidence as pending.

**Seed as drafts/planned (only where evidence is real or imminent):** the three
homelab projects from `~/tech-skill-up/HOMELAB_PROJECTS.md` (internal DNS +
network map / Network+; harden & monitor / Security+; Proxmox backup-restore +
monitoring + incidents / Server+). Published only when the artifact exists.
