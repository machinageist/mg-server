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

---

## 2026-10-04: /learn pointed at CCNA

Branch: `ccna-learn-reshape` (off `main`). Source:
`docs/plans/2026-10-04-ccna-wiki-and-interactive-learning-HANDOFF.md`. One entry per
slice, each its own commit with `cargo fmt`, `clippy -D warnings`, and `test` green.

**Slice 1: draft flag (A1).** A page with `draft: true` in its frontmatter stays
registered in `SIDEBAR` but is left out of all three sidebar orderings and out of
search. Its URL is a 404 in a release build and loads in a debug build
(`SERVE_DRAFTS = cfg!(debug_assertions)` in `handlers/wiki.rs`). The gate is the
build profile, not an environment variable, so nothing can be left switched on in
production. The site has no sitemap, so there was nothing to exclude there.

**Slice 2: CCNA page shape in the lint.** `tests/content_lint.rs` now knows two page
shapes. A page with a `## Lab` section is a CCNA page and needs Overview, On the
device, Exam key points, Lab, Related pages, and Sources with an `https://` link, but
no Suggested practice and no textbook name. Draft pages skip the section contract. A
published page or post that links to a draft fails `internal_links_resolve`. The
existing textbook contract is unchanged, only moved into its own function.

**Slice 3: 14 CCNA draft pages (A2).** `content/pages/` gained `ethernet-and-arp`,
`spanning-tree`, `etherchannel`, `ospf`, `first-hop-redundancy`,
`access-control-lists`, `dhcp-and-dns-services`, `device-management-protocols`, `nat`,
`securing-device-access`, `layer-2-security`, `rest-json-and-config-management`,
`ai-in-network-operations`, and `troubleshooting-method`. Each has `draft: true`,
eight headings, and a `TODO(jeff)` line under each. No lesson prose. The summary is
also `TODO(jeff)`. All 14 are registered in `SIDEBAR` (folded into the existing topic
sections) and in `WIKI_SLUGS`. To publish one, write it and delete the `draft` line.

**Slice 4: drafts filed under their objectives (A3).** Each draft is listed once in
`CCNA_V1_1` and `CCNA_V2_0`, under the lowest objective in its main domain. The
numbers come from Cisco's exam topics PDFs, read on 2026-10-04
(`learningcontent.cisco.com/documents/marketing/exam-topics/200-301-CCNA-v1.1.pdf`
and `200-301_CCNA_v2.0_Exam_Topics_PDF.pdf`). Drafts do not render in either ordering
until `draft` is removed. Left out on purpose, for Jeff to confirm:
`troubleshooting-method` has no v1.1 objective (v2.0 files it at 2.4), and
`ethernet-and-arp` has no v2.0 objective (v1.1 files it at 1.13, switching concepts).
`dhcp-and-dns-services` is 1.7 in v2.0 for the DHCP half. Its DNS half is 4.4.
`rest-json-and-config-management` is 5.5 in v2.0, which covers Ansible only. v2.0
names neither REST nor JSON.

**Slice 5: hidden On the device placeholders (A4).** `switching-technologies`,
`routing-technologies`, `ipv4-addressing`, `ipv6-addressing`, `wireless-technologies`,
and `network-protocols` each gained a commented-out `## On the device` heading with a
`TODO(jeff)` line, placed above Suggested practice. Nothing else in those files
changed. The renderer used to escape an HTML comment and show it as text, so
`models/markdown.rs` now drops comments from the page, the outline, and search. All
other source HTML is still escaped. To use a placeholder, delete the `<!--` and `-->`
lines and paste the lab output.

**Slice 6: wiki backlog (A6).** `docs/wiki-backlog.md` holds the template for polish
skipped when the 45-minute timebox runs out. It has no entries yet.

**Slice 7: portfolio authorship label (C).** `Project` gained
`authorship: Option<Authorship>`, with two values that render as "hand-written" and
"AI-assisted, directed by me". A card shows the line only when the value is set. All
three entries are `None`, so `/portfolio` looks the same until Jeff fills them in.

**geistscope check (C).** Nothing to change. No file under `content/posts`,
`content/pages`, `content/projects`, `content/labs`, `templates`, `src`, or `static`
links to the geistscope repository, and neither does `~/machinageist/README.md`. The
remaining mentions are in unrouted `content/drafts/` files, the guard comment and test
in `src/models/project.rs`, and a filename in the README tree.

**Slice 8: who writes what.** `README.md` and the `/study` index each say: "I write
the lessons on this site. The interactive study tools are AI-assisted." The handoff
worded it in the third person. It is in the first person here because the README and
the site already are. Jeff to confirm the wording. The about page and footer are
unchanged.

**Slice 9: interactive learning ADR (B, design only).**
`docs/plans/interactive-learning-ADR.md` covers routes, data formats, no-JS behavior,
and what JavaScript adds. The subnetting drill and IOS PBQ matching are specified to
build depth. Features 3 to 8 get a paragraph each. Nothing is built. It waits for
Jeff's approval, and it ends with four open questions for him.

**Slice 10: follow-ups from Jeff's review.** The disclosure line now reads "The
lessons on this site are AI-written with my direction and approval. The interactive
study tools are AI-assisted." (Jeff's correction, 2026-10-04). The citation tests for
questions and scenarios reject a draft page as a target. The handoff document is
tracked.

---

## 2026-10-04: interactive learning, feature 1 (subnetting drill)

Built from `docs/plans/interactive-learning-ADR.md` section 3 after Jeff's go-ahead.

**Drill model, routes, and grading.** `src/models/drill.rs` generates ten subnetting
problems from a seed (SplitMix64, output pinned by a test), solves them, matches typed
answers as values, and writes the block-size working for each. Three kinds ship:
network and host range, hosts per prefix, and prefix for a host count. Wildcard
problems wait for a published page to cite. `GET /study/drills/subnetting` redirects to
a seeded URL, `?seed=` shows that set, and a POST to the same URL grades it. The server
stores nothing. A seed or `type` that does not parse is a 404. Every kind links to the
`/learn/subnetting` heading that teaches it, and a test resolves the links. **The
working text in `Problem::working` and `network_working` is teaching copy for Jeff to
read and approve.**

**Drill timer and index link.** `/study` lists the subnetting drill under a new
Drills heading. `static/js/drill.js` shows a count-up timer from the first answer
typed. The timer is hidden in the HTML, is never sent to the server, and the page
works the same without it.

## 2026-10-04: interactive learning, feature 2 (Cisco IOS PBQ matching)

**IOS abbreviation in scenario grading.** A scenario with `dialect: ios` accepts any
unambiguous prefix of a keyword, checked against keyword groups the scenario lists
itself. A step sets `mode` when it is not typed at the exec prompt. Interface names
abbreviate with or without a space (`g0/1`, `gig 0/1`). Arguments match exactly. The
accepted list is still checked first, so the published shell scenario grades as before.
No route, template, or handler signature changed. `content/drafts/study/pbq/ios-fixture.md`
shows the format and is loaded only by tests. **No IOS scenario is published. Jeff
writes those, and checks each keyword list against a real device.**

## 2026-10-04: disclosures and terms pages, footer links

Jeff asked for the disclosures to live on their own page. `/disclosures` says who
writes what (lessons, study tools, projects), that the site is not affiliated with
Cisco or CompTIA, and what the site stores. `/terms` is a short plain-language terms of
use. **It was drafted by an agent and has not had legal review.** The footer on every
page now links Disclosures, Terms, Contact (a mailto to the address `security.txt`
already publishes), and Source. The authorship line was removed from `/study`. The
per-project disclosure sections stay where they are, because a test requires them on
each project page. Both new routes are registered in `UTILITY_ROUTES`.

---

## 2026-10-04: wiki refactored around the CCNA objectives

Jeff asked for the wiki to reflect the CCNA objectives. Plan agreed in session: sidebar
rebuilt around the CCNA domains, every networking page reordered so CCNA material leads
with Network+-only material kept below it, gaps against v1.1 filled, Linux pages
untouched, the 14 scaffolds left empty. This lifts the handoff's freeze on the
Network+-only pages.

**Objective map.** `docs/plans/ccna-wiki-refactor-MAP.md` lists every v1.1 objective
with the page that covers it, the gap, and the action, then the planned section order
for each of the 21 networking pages. It ends with open questions: objectives 5.1 and 5.2
have no page or scaffold.

**Reading list.** `docs/ccna-reading-list.md` gives Jeff, for each of the 14 scaffolds,
the v1.1 objectives verbatim, the Jeremy's IT Lab days and Odom chapters from his own
syllabus, and primary sources to check against. Every link was loaded on 2026-10-04.
His notes go in `content/drafts/ccna-notes/<slug>.md`.

**Sidebar rebuilt around CCNA.** The sidebar has two views: CCNA, which is the default
and shows with JavaScript off, and Network+. The Topic view is gone. Each view lists the
exam's domains in objective order, then "Not on this exam", then "Linux foundations", so
either view reaches every published page once. `SIDEBAR` is now a flat registry with a
group per page. A stored choice of `topic` falls back to CCNA.

**Pilot page: `switching-technologies`.** Reordered to follow v1.1 objectives 2.1, 2.2,
2.4, and 2.5: VLANs, inter-VLAN routing, trunks, 802.1Q, link aggregation, spanning
tree. Private VLANs and frame size moved under a new `## Beyond the CCNA blueprint`
heading with their text unchanged. Added: the default VLAN and VLAN ranges, trunk
negotiation (DTP), Rapid PVST+ with the root bridge and port roles, and PortFast with
the four guards. No published sentence was removed. Four were reworded because they
pointed at a section's old position or listed what the page did not cover. The
additions cite Cisco's exam topics and four Cisco spanning-tree documents, each
fetched and read on 2026-10-04. Heading ids are unchanged, so existing links, glossary
references, and question citations still resolve.

**Sidebar entries expand to their sections.** Each page in the sidebar is a native
`<details>`: the marker expands it to links for the page's second-level headings, and
the page name is still a link. The page being read starts expanded. No script. The
headings are read from each page on every request by `page_anchors` in
`handlers/wiki.rs`.

**Sidebar groups collapse.** Each group in the sidebar (an exam domain, "Not on this
exam", "Linux foundations") is also a native `<details>`. The group holding the page
being read starts open, and so does Overview. The rest start
collapsed, so the sidebar opens as a short list of headings.

**Merged to `main`.** `main` had gained `physical-installations` (published
2026-10-02) since the branch was cut. The merge keeps it: registered in the flat
`SIDEBAR` as a networking page, with its exam ordering entries as published.

**Fix: four pages were in the wrong sidebar group.** When `SIDEBAR` was flattened,
`osi-model`, `network-topologies`, `traffic-types`, and `troubleshooting-method` were
marked as the overview group. Under v1.1 that had no visible effect. After the v2.0
cutover the first three would have been listed beside the front page instead of under
"Not on this exam". They are networking pages now, and a test holds that only the
front page is in the overview group.

**`physical-installations` reordered.** Same pattern as the pilot. Cable pinouts and
physical access control, the parts v1.1 lists under 1.3, 1.4, and 5.2, lead the page.
Distribution frames, racks, building cabling, power, and environment moved under
`## Beyond the CCNA blueprint` with their text unchanged. No sentence was removed and
no lesson content was added, apart from one paragraph saying how the page is ordered
and a two-line lead-in to the pinouts. Heading ids are unchanged.

---

## 2026-10-05: CCNA reading list and notes moved to the course

The reading list and the place notes go moved to `~/mg-coreforge/ccna/course/`, so
note taking runs from one folder. `docs/ccna-reading-list.md` is now a pointer.
`notes/` there has one blank note file per topic, `ORDER.md`, and `READING-LIST.md`.
Each week folder has a `reading.md`, and the sources for the 14 draft pages are in
those. The empty `content/drafts/ccna-notes/` folder is gone. The 14 draft pages are
unchanged and stay in `content/pages/`.
