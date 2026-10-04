# Handoff: point /learn at CCNA, and make the site interactive for learners

**Written:** 2026-10-04 (Cowork planning session with Jeff)
**For:** a coding agent working in this repo from the CLI
**Read first:** `CLAUDE.md` (if present), `docs/VOICE.md`, `src/handlers/wiki.rs`,
`src/models/question.rs`, `src/models/scenario.rs`, `src/middleware/security_headers.rs`

---

## 1. Context in five lines

- Jeff is studying for **CCNA 200-301 v1.1**. First attempt is planned for the week of
  2027-01-11, a retake the week of 2027-01-25. v1.1 retires 2027-02-02 and v2.0 starts 2027-02-03
  (`CCNA_V2_FIRST_DAY` in `wiki.rs` already handles the switch).
- He stopped working through the Network+ textbook. The /learn wiki (28 pages, ~47k words) stays
  and keeps growing, but **a page is now written after a topic is studied and labbed, not before.**
- New CCNA pages are drafted by Jeff from voice memos, inside a 45-minute timebox per topic.
- **geistscope is going private.** The site already demoted it; make sure nothing public links to it.
- The study plan lives in `~/mg-coreforge/ccna/course/` (syllabus + weekly lessons and labs).

## 2. Authorship policy (this decides what you may write)

| Surface | Who writes it | Why |
|---|---|---|
| Lesson prose on `/learn` pages, configs, `show` output, lab write-ups | **Jeff only.** You may scaffold headings and TODO markers. | He must be able to defend every word in an interview. |
| Quiz questions, PBQ scenarios, flashcards | Jeff writes or approves each one. You may draft into a `drafts/` path, never straight into `content/study/`. | They teach. Wrong ones teach errors. |
| Interactive tooling (Rust handlers, templates, JS, drill generators, calculators) | **You may build these.** | Jeff is not applying for web-dev roles. AI-built tooling is fine here. |
| Portfolio entries in `src/models/project.rs` | Jeff edits copy. You may add an authorship field (see 5.2). | Honest labeling. |

Add one line to `README.md` and the site footer or about page: *"Lessons are written by Jeff.
Interactive study tools on this site are AI-assisted."* Use Jeff's plain voice (`docs/VOICE.md`).

## 3. Hard constraints

- CSP is `script-src 'self'`. No CDNs, no inline scripts, no external fonts or trackers.
- **Progressive enhancement:** every learning surface must work with JS off. JS can only add to it.
- No accounts and no server-side learner state. Per-learner progress lives in `localStorage`,
  wrapped in try/catch, and the page must render correctly when storage is empty or throws.
- Keep the existing citation test: every question cites a real heading on a real /learn page.
  New interactive items that teach must cite a page the same way.
- Follow the repo's file header and comment conventions (see any file in `src/models/`).
- `cargo fmt`, `cargo clippy -- -D warnings`, `cargo test` green before each commit.
- Small slices, one conventional commit each. **Jeff reviews before anything is pushed.**

## 4. Workstream A: reshape /learn for CCNA (do first)

### A1. Draft pages that stay out of navigation
Check whether `src/models/page.rs` supports a `draft: true` frontmatter flag. If it does not,
add one. Draft pages: excluded from the sidebar, `/learn` index, search, and sitemap; reachable
by direct URL only when running locally (gate on a debug build or an env var, your call; say which).

### A2. Scaffold 14 CCNA pages, in Jeremy's IT Lab order
Create each as `content/pages/<slug>.md` with `draft: true`, frontmatter, and these headings only.
Under each heading put a single `TODO(jeff)` line. **Do not write lesson prose.**

```
## Overview
## How it works
## On the device        <- IOS config + verified show output, pasted by Jeff from his lab
## Troubleshooting it   <- symptoms -> commands -> cause
## Exam key points
## Lab                  <- link to the matching lab write-up
```

| Order | Slug | Title (working) |
|---|---|---|
| 1 | `ethernet-and-arp` | Ethernet frames, MAC learning, and ARP |
| 2 | `spanning-tree` | Spanning tree: STP and RSTP |
| 3 | `etherchannel` | EtherChannel |
| 4 | `ospf` | OSPF, single area |
| 5 | `first-hop-redundancy` | First-hop redundancy (HSRP, VRRP, GLBP) |
| 6 | `access-control-lists` | ACLs and wildcard masks |
| 7 | `dhcp-and-dns-services` | DHCP, DHCP relay, and DNS |
| 8 | `device-management-protocols` | NTP, SNMP, syslog, CDP and LLDP |
| 9 | `nat` | NAT and PAT |
| 10 | `securing-device-access` | Securing device access: passwords, SSH, AAA |
| 11 | `layer-2-security` | Port security, DHCP snooping, dynamic ARP inspection |
| 12 | `rest-json-and-config-management` | REST APIs, JSON, Ansible and Terraform |
| 13 | `ai-in-network-operations` | AI and machine learning in network operations |
| 14 | `troubleshooting-method` | A troubleshooting method, layer by layer |

### A3. Add them to the exam orderings in `wiki.rs`
Add each page to `CCNA_V1_1` and `CCNA_V2_0` under the right section and objective number.
**Verify objective numbers against Cisco's official exam topics PDFs for 200-301 v1.1 and
v2.0. Do not guess.** If you cannot fetch them, leave `objective: "?"` and list the open
items for Jeff. Draft pages must not render in the ordering until `draft` is false.

### A4. Add an "On the device" layer to existing CCNA pages
For `switching-technologies`, `routing-technologies`, `ipv4-addressing`, `ipv6-addressing`,
`wireless-technologies`, and `network-protocols`: insert an empty `## On the device` heading with
a `TODO(jeff)` line in a sensible spot. Nothing else.

### A5. Freeze list (do not touch content)
Network+-only pages: `content-delivery-networks`, `zero-trust-architecture`, `network-topologies`,
`transceivers`, `transmission-media`, `wired-media`, `cloud-computing`, `traffic-types`.
All `linux-*` pages are frozen until Jeff starts RHCSA.

### A6. Backlog file
Create `docs/wiki-backlog.md`: a dated list where Jeff parks polish he skipped because the
45-minute timebox ran out. Seed it with the template only.

## 5. Workstream B: interactive learning (explore, then build)

**First deliverable is a short design note, not code:** `docs/plans/interactive-learning-ADR.md`.
It covers routes, data formats (YAML in `content/study/...`, following the existing frontmatter
pattern), no-JS behavior, and what JS adds. Jeff approves it, then build in the order below.

| Rank | Feature | Why it helps learners | Notes |
|---|---|---|---|
| 1 | **Subnetting drill generator** `/study/drills/subnetting` | Unlimited fresh problems with instant feedback. Subnetting is the most-tested arithmetic. | Server-side, seeded by `?seed=` so a problem set is shareable and works without JS. Problem types: network/broadcast/range, hosts per prefix, VLSM fit, wildcard for a prefix. Explanation shows the block-size method. Optional JS timer. |
| 2 | **Cisco IOS PBQs** | Typing commands from a blank prompt is the CCNA skill Network+ never taught. | Extend `scenario.rs` matching with IOS abbreviation rules: a token matches if it is an unambiguous prefix of the keyword (`sh ip int br` = `show ip interface brief`). Keep the accepted-forms list and add a per-scenario keyword dictionary. Write unit tests for this matching first. |
| 3 | **Route selection trainer** | Fixes a common misconception: longest prefix first, then administrative distance, then metric. | Given a routing table and a destination, pick the route. The explanation names which rule decided it. |
| 4 | **Packet-walk stepper** | Shows ARP, frame rewrite at each hop, VLAN tags, and NAT one step at a time. | Data-driven from a YAML scenario. Server renders all steps as an ordered list (no JS). JS turns it into a stepper with an SVG diagram highlighting the current hop. |
| 5 | **Listen to this page** | Audio learners, including Jeff. | Browser `speechSynthesis` only, no external service. A button that appears only when the API exists. Reads the main article, skips code blocks. |
| 6 | **Progress and review** | Learners see what they have done and what is due. | `localStorage` record of quiz/drill results per slug. A "due for review" list on `/study` using a simple spaced schedule (1, 3, 7, 14 days). |
| 7 | **Flashcard export** | Lets learners take the cards into Anki. | `/study/cards/:slug.csv` (front, back, tags) built from the existing card data. |
| 8 | **Wildcard and ACL trainer** | Wildcard math is a common error. | Can reuse the generator from feature 1. |

## 6. Workstream C: hygiene

- **geistscope going private:** `grep -rni geistscope content templates src static`. Remove or
  unlink any public link to the GitHub repo. `content/drafts/geistscope-retrospective.md` is
  unpublished. Leave it, but do not link to the private repo from it. The comment and test in
  `src/models/project.rs` are fine.
- **Portfolio authorship labels:** add an optional `authorship` field to portfolio entries
  (`"hand-written"`, `"AI-assisted, directed by me"`) and render it as a small line on the card.
  Jeff fills in the values.
- `~/machinageist/README.md` (separate repo): check it for any geistscope link. There was none at
  the time of writing.

## 7. Non-goals

- No new blog posts, no visual redesign, no theme work.
- No lesson prose, quiz questions, or lab write-ups authored for Jeff.
- No accounts, analytics, comments, or third-party embeds.
- Do not touch the Network+ ordering beyond keeping it compiling.

## 8. Definition of done

- A1 to A6 merged locally, tests green, the 14 drafts invisible in production navigation.
- The ADR is written and approved by Jeff, then features 1 and 2 are shipped with tests.
- A short `docs/REORG_CHANGELOG.md` entry per slice.
- A final message to Jeff listing: files changed, objective numbers that still need checking,
  and anything you were unsure about.
