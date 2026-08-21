# Site reorg changelog — SysAdmin/NOC retarget

> **Later change (2026-07-23):** the wiki became a curated education section. All
> GeistScope tool pages were removed, leaving the retrospective as the public record.
> Git retains the complete experiment; tools return only after meeting the current
> pipeline, human/AI operation, and authorized-engagement evidence gate.

Branch: `site-reorg-sysadmin-noc` (off `main`). Done 2026-07-10.
Every phase kept `cargo fmt`/`clippy -D warnings`/`build`/`test` green; each phase
is its own conventional commit.

## What changed, by phase

**Baseline** — committed the pre-existing uncommitted "Data Center / Remote Hands /
infrastructure-support" pivot as a labeled baseline so the retarget reads as clean
deltas.

**Phase 0 — recon.** Added `docs/geistscope-page-triage.md`: every wiki page marked
KEEP/REMOVE from GeistScope's own `PRUNING_INVENTORY.md`, plus blog-post
classification. KEEP = the 10-page safe core (`Keep now` + `Keep later`).

**Phase 1 — identity & copy.** Hero, About, Start Here, and page `description()`
strings now lead with "Systems Administrator / NOC Technician (in training)", the
five pillars, and the Network+ → Server+ cert spine. Location metadata was
normalized. Nav label "Reference" → "Archive."
Pinned-copy tests updated; anti-overclaim guards kept.

**Phase 2 — portfolio.** `project.rs::all()` reordered to lead with the three
`HOMELAB_PROJECTS.md` projects (honest InProgress, safe-claim wording marked
evidence-pending), then mg-server, the cert track, and GeistScope demoted to one
archived-reference line. Test rewritten to assert the new lead + demotion.

**Phase 3 — blog.** Deleted the 22 `geistscope-*` devlogs; added one honest
`geistscope-retrospective.md`. Finished the mg-server draft into
`hosting-machinageist-dev.md` (real `dig` + `curl -I` evidence). Added an optional
`category` frontmatter field and grouped the blog list by the five pillars (unit
tested). Kept + lightly reframed `memory-safety-c-vs-rust` (Security) and
`port-scanner-in-rust` (Networking).

**Phase 4 — wiki archive.** Deleted ~75 pages, kept the 10-page safe core. Rewrote
the `SIDEBAR` const. Resynced `tests/wiki_pages.rs` and added a reverse
no-orphaned-pages guard. Reframed `content/pages/index.md`. Converted dangling
in-page links to removed slugs into plain "archived, no page" text.

**Phase 5 — security.** Added `security-headers-on-machinageist-dev.md` (Security
pillar) with real `curl -I` evidence and the actual middleware source; defensive,
owned-scope framing; resolves the hosting post's forward link.

**Phase 6 — docs.** Retargeted `IMPROVEMENT_PLAN.md` and
`docs/public-portfolio-structure.md` to SysAdmin/NOC-primary + five pillars + cert
spine (kept claim-defense discipline). Rewrote `README.md` overview/structure/
deployment/security.

**Phase 7 — verify.** `fmt --check`, `clippy -D warnings`, `build`, `test` all
green (12 tests). Clicked through every route: all core routes, 5 posts, and 10
wiki pages return 200; removed slugs 404. No dangling internal links. No banned
affirmative tokens on public pages.

## Deleted

- 22 `content/posts/geistscope-*.md` devlogs (folded into the retrospective).
- `content/posts/blog-draft-mg-server.md` (finished as `hosting-machinageist-dev.md`).
- ~75 `content/pages/mg-*.md` (all but the 10-page safe core).

## Evidence captured (real, not fabricated)

- `dig +short machinageist.dev A/NS` and `curl -sSI https://machinageist.dev`,
  captured live 2026-07-08/09. Used in the hosting and security-headers posts.
  The live headers match `src/middleware/security_headers.rs`.

## Open questions / decisions for Jeff

1. **Lead identity headline.** I used "Systems Administrator / NOC Technician (in
   training)" per the roadmap's two primary roles. If you want NOC-first or
   Linux-first instead, it's a one-line hero change in `templates/index.html`
   plus the matching About/Start-Here headers.

2. **Homelab project statuses.** All three homelab cards are `InProgress` with
   "evidence not yet captured" wording. As each artifact lands, publish its blog
   writeup and flip the card. None is claimed as done.

3. **Empty pillars.** "Homelab & Proxmox" currently has no published post, so it
   doesn't render a blog group yet (empty groups are dropped). The first homelab
   writeup (internal DNS + network map, Network+) will populate it.

4. **Cert résumé lines.** Add "Network+ (in progress, exam scheduled)" style lines
   to the cert-track card only when true; add a passed cert the day you pass it.
   The site currently claims no cert as passed.

5. **`releases` route.** Left as-is (not in nav; a test asserts that). Nothing
   points at it.

6. **Retrospective tone.** `geistscope-retrospective.md` deliberately uses
   "red-team/offensive" in *negation* ("presenting owned-scope code as an offensive
   platform was the overclaim"). That's the honest self-critique, and it's in the
   post body only — the identity/positioning templates stay clean. Reword if you'd
   rather avoid the words entirely.

7. **Merge.** This is on `site-reorg-sysadmin-noc`, not merged to `main`. Review
   the per-phase commits, then merge when you're happy.
