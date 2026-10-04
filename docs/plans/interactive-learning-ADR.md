# ADR: interactive learning on /study

**Written:** 2026-10-04
**Status:** proposed. Nothing in here is built. Jeff approves it, then features 1 and 2
are built in that order.
**Source:** `docs/plans/2026-10-04-ccna-wiki-and-interactive-learning-HANDOFF.md`, section 5.

Features 1 and 2 are specified to build depth. Features 3 to 8 get one paragraph each,
enough to keep their routes and data shapes from colliding later.

---

## 1. Rules every feature follows

These come from the handoff and from how the site already works.

- **The server does the work.** Every surface is a normal page and a normal form. It
  renders, grades, and explains with JavaScript off. JavaScript only adds to a page
  that already works.
- **CSP is `default-src 'self'; script-src 'self'; style-src 'self'`.** No inline
  scripts, no inline `style` attributes, no CDN. Scripts are files under `static/js/`,
  loaded with the `?v=` asset version like `learn-order.js`. SVG is styled by class
  from `style.css`.
- **No learner state on the server.** No accounts, no cookies, no session. Progress is
  `localStorage` only, every read and write is in `try/catch`, and a page looks right
  when storage is empty or throws.
- **Anything that teaches cites the wiki.** A hand-written item cites a `/learn` page
  and heading, and a test resolves that against the real heading ids, the same as
  `models::question` and `models::scenario` do today. A generated item (a drill) cites
  one fixed page and heading per problem type, checked by the same kind of test.
- **A citation must point at a published page.** The citation tests in
  `models::question` and `models::scenario` reject a `draft: true` target, because the
  link would be a 404 in production. New citation tests do the same.
- **Who writes what.** I build handlers, templates, JS, and generators. Quiz questions,
  PBQ scenarios, and flashcards are written or approved by Jeff. When I draft one, it
  goes in `content/drafts/study/` (unrouted, never loaded) and Jeff moves it into
  `content/study/` himself.
- **Conventions.** File headers and comment style as in `src/models/`. `cargo fmt`,
  `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets` green before
  each commit. One slice, one commit. A `docs/REORG_CHANGELOG.md` entry per slice.

## 2. Route map

Existing routes are unchanged. New ones, with the feature that owns each:

| Route | Method | Feature |
|---|---|---|
| `/study/drills/subnetting` | GET, POST | 1 |
| `/study/pbq/:slug` | GET, POST (exists, matching extended) | 2 |
| `/study/drills/routes` | GET, POST | 3 |
| `/study/walks/:slug` | GET | 4 |
| none (button on `/learn/:slug`) | | 5 |
| none (section on `/study`) | | 6 |
| `/study/cards/:slug.csv` | GET | 7 |
| `/study/drills/wildcards` | GET, POST | 8 |

`/study/drills/...`, `/study/walks/...`, and `/study/cards/...` are registered above
`/study/:slug` in `router.rs`, as `/study/cards/:slug` already is. `drills`, `walks`,
`cards`, and `pbq` become reserved names that a question set cannot use as a slug. A
test will hold that.

Content directories: `content/study/pbq/` (exists), `content/study/walks/` (feature 4).
Drills have no content files. They are generated.

---

## 3. Feature 1: subnetting drill generator

### What the reader gets

A page of ten subnetting problems. The reader fills in the answers and submits. The
result page marks each answer, shows the correct one, and shows the working by the
block-size method. A "new set" link gives ten different problems. The URL carries the
seed, so a set can be bookmarked or shared and always shows the same problems.

### Routes

- `GET /study/drills/subnetting` with no seed: pick a seed and redirect (303) to
  `?seed=<n>`. The seed comes from the system clock's nanoseconds. It only has to
  differ between visits, so no random-number crate is added.
- `GET /study/drills/subnetting?seed=<n>&type=<t>`: render the set. `type` is optional
  and limits the set to one problem type. An unknown `type` or a `seed` that is not a
  `u64` is a 404 through the normal `SiteError` path.
- `POST /study/drills/subnetting?seed=<n>&type=<t>`: regenerate the same set from the
  seed, grade the form, render results. The server holds nothing between the two
  requests. The seed is the whole state.

### Generator

New file `src/models/drill.rs`. Pure functions, no I/O.

- A small seeded generator (SplitMix64, about ten lines) so the same seed gives the
  same problems on every build and platform. This is not security. It only has to be
  repeatable. Its output is pinned by a test, because changing it would change what
  every shared link shows.
- `pub fn subnetting_set(seed: u64, kind: Option<ProblemKind>) -> Vec<Problem>`.
- Addresses are drawn from RFC 1918 space and the RFC 5737 documentation ranges only.

Four problem kinds:

| Kind | Given | Reader answers | Fields |
|---|---|---|---|
| `network` | a host address and a prefix, for example `172.16.37.201/20` | network address, broadcast address, first and last usable host | 4 text inputs |
| `hosts` | a prefix | usable hosts per subnet | 1 text input |
| `vlsm` | a host count needed | the longest prefix that fits it | 1 text input |
| `wildcard` | a prefix | the wildcard mask | 1 text input |

Prefix ranges: `/8` to `/30` for `network`, `hosts`, and `wildcard`. `/31` and `/32`
are left out of the generator because their host rules are special cases the subnetting
page does not teach yet.

### Grading

- An answer is normalized before comparing: trimmed, a leading `/` on a prefix is
  accepted or omitted, and an address is parsed into four octets so `010.0.0.1` style
  padding does not matter. An address that does not parse is wrong, not an error.
- Blank is skipped, not wrong, and the result page counts it separately. This is the
  same rule the quiz and PBQ graders already follow.
- A `network` problem is four graded fields, each marked on its own.

### Explanations

Each result shows the working, generated from the numbers, in a fixed form:

> Prefix /20 puts the boundary in the third octet. Block size is 256 - 240 = 16. The
> multiples of 16 around 37 are 32 and 48. Network 172.16.32.0, broadcast 172.16.47.255,
> hosts 172.16.32.1 to 172.16.47.254.

These sentences are templates in Rust with numbers filled in. They are teaching copy,
so **Jeff reviews the four templates before they ship**, and each one links to the
heading that teaches it:

| Kind | Cites |
|---|---|
| `network` | `/learn/subnetting#subnet-masks` |
| `hosts` | `/learn/subnetting#counting-hosts-and-networks` |
| `vlsm` | `/learn/subnetting#variable-length-subnet-masking` |
| `wildcard` | open. The subnetting page has no wildcard section yet |

The wildcard kind is held back until a published page teaches wildcard masks
(`access-control-lists` is the planned one). A test resolves each citation against the
rendered heading ids.

### Templates and JavaScript

- `templates/study_drill.html` (the form) and `templates/study_drill_result.html`.
  Inputs use `inputmode="numeric"` or plain text, `autocomplete="off"`, and a `<label>`
  each.
- `/study` gains a "Drills" section linking here.
- JavaScript, optional, in `static/js/drill.js`: a count-up timer that starts on first
  input and shows the elapsed time. It is not sent to the server and does not affect
  grading. The timer element is `hidden` in the HTML and the script reveals it, so
  nothing shows with JavaScript off.

### Tests, written first

- Same seed, same set. Different seed, different set. The first values for seed 1 are
  pinned.
- For every prefix from /8 to /30 and a sample of addresses: network, broadcast, and
  host range agree with a direct bitmask calculation.
- `hosts` and `vlsm` are inverses: the prefix chosen for N hosts holds at least N, and
  the next longer prefix does not.
- Grading: correct, wrong, blank, padded octets, prefix with and without the slash.
- Generated addresses stay inside the allowed ranges.
- The page has a `<form method="post">` and a submit button and loads no script it
  needs (same style as `the_quiz_needs_no_javascript`).
- Each explanation citation resolves.

### Slices

1. `models/drill.rs`: generator and solver, with tests.
2. Routes, handler, the two templates, grading. No JS.
3. Explanations and citations, after Jeff has read the templates.
4. Timer script and the `/study` link.

---

## 4. Feature 2: Cisco IOS PBQs

### What the reader gets

The existing scenario page, with steps that expect IOS commands. `sh ip int br` is
marked correct where `show ip interface brief` is the answer, because IOS itself accepts
it.

### The matching rule

IOS accepts any prefix of a keyword that is unambiguous among the keywords valid at that
position. The site cannot know the full IOS grammar, so each scenario carries the
keyword lists it needs.

A typed answer matches an accepted command when:

1. Both have the same number of tokens after whitespace is collapsed.
2. For each position, the typed token equals the accepted token (ignoring case), or
   - the accepted token is a keyword, and
   - the typed token is a prefix of it (ignoring case), and
   - it is a prefix of no other keyword in the same group.
3. A token that is not a keyword is an argument: an interface number, an address, a
   name, a VLAN id. Arguments must match exactly, ignoring case only for interface
   names. `g0/1` for `GigabitEthernet0/1` is handled in rule 4.
4. Interface names: a typed `g0/1`, `gi0/1`, or `gig 0/1` matches
   `GigabitEthernet0/1`. The type and the number are split and the type is matched as a
   keyword against a fixed list of interface types. A space between type and number is
   accepted.

The existing `accept` list stays and is checked first, with the existing normalization,
so every current scenario grades exactly as it does now. Short-flag sorting (`-Rv` equals
`-vR`) applies to shell steps only.

### Data format

Two additions to `content/study/pbq/<slug>.md`, both optional, so the existing file is
valid unchanged:

```yaml
title: "..."
situation: >
  ...
# New. Says the steps are IOS commands. Default is "shell"
dialect: ios
# New. Keywords grouped by where they compete. A prefix must be unambiguous
# inside its group
keywords:
  exec: [show, configure, copy, ping, traceroute, write, enable, disable, debug]
  show: [ip, interfaces, vlan, running-config, startup-config, version, mac, spanning-tree]
  show-ip: [interface, route, ospf, protocols, nat, dhcp, access-lists]
  show-ip-interface: [brief]
steps:
  - prompt: >
      ...
    accept:
      - "show ip interface brief"
    explanation: >
      ...
    learn_slug: "..."
    learn_anchor: "..."
```

A group is named by the keywords before it, joined with `-`, with `exec` for the first
token. A keyword in an accepted command that is in no group is matched exactly. That
means a scenario with no `keywords` still works. It only loses abbreviation.

Why per scenario and not one global list: ambiguity depends on which commands exist at
that prompt. `sh` is unambiguous in exec mode on most images and `s` is not. A global
table would be a partial copy of the IOS parser that is wrong in ways a learner cannot
see. A per-scenario list is small and says exactly what the scenario grades against.

The keyword lists are tooling data, not lesson prose, but they decide what is marked
right, so **Jeff checks each scenario's list against a real device**. The scenarios
themselves are Jeff's to write.

### Code

- `src/models/scenario.rs`: `Scenario` gains `dialect` (an enum, default `Shell`) and
  `keywords` (a map of group to list, default empty). `Step::accepts` takes the
  scenario's dialect and keywords. A new private `ios_matches` holds the rule above.
- No route, template, or handler signature changes. `grade_scenario` passes the
  scenario through.
- The result page already shows the first accepted form as the canonical answer. For
  IOS steps that is the full command, which is what a learner should see.

### Tests, written first

Unit tests for `ios_matches`, before any scenario exists:

- `sh ip int br` matches `show ip interface brief`.
- A full command matches itself, in any letter case.
- `s ip int br` fails when `exec` holds both `show` and another `s` keyword.
- `sh ip in br` fails when `show-ip` holds two keywords starting `in`.
- A wrong argument fails: `int g0/2` does not match `interface GigabitEthernet0/1`.
- `int g0/1`, `interface gi0/1`, and `int gig 0/1` match `interface GigabitEthernet0/1`.
- A token count mismatch fails. An extra trailing keyword fails.
- A blank answer is skipped, not wrong.
- A keyword with no group is matched exactly and is not abbreviated.
- A shell scenario with no `dialect` grades exactly as before. The existing tests stay
  as they are.

Content tests:

- Every keyword used in an `accept` line of an IOS scenario that appears in a group is
  an unambiguous member of it. A list where the full word is itself ambiguous is a data
  error.
- Citations must resolve to a published page (the rule from section 1).

### Slices

1. `ios_matches` and its unit tests. No content, no behavior change on the site.
2. `dialect` and `keywords` on `Scenario`, wired into grading, with the content tests.
3. A fixture scenario under `content/drafts/study/pbq/` for Jeff to rewrite or replace.
   Nothing ships to `content/study/pbq/` from me.

---

## 5. Features 3 to 8, in brief

**3. Route selection trainer, `/study/drills/routes`.** Generated like feature 1 and
seeded the same way, reusing the generator in `models/drill.rs`. Each problem shows a
routing table and a destination address. The reader picks the route with radio buttons.
The explanation names the rule that decided it: longest prefix first, then
administrative distance, then metric. It cites the route selection section of
`/learn/routing-technologies`. No content files.

**4. Packet-walk stepper, `/study/walks/:slug`.** Hand-written data in
`content/study/walks/<slug>.md`, frontmatter only: a title, a list of nodes, and an
ordered list of steps. Each step has the hop it is at, the frame and packet fields at
that point (source and destination MAC, source and destination IP, VLAN tag, NAT state),
a sentence saying what changed, and a citation. The server renders every step as an
ordered list with a table per step. `static/js/walk.js` turns the list into a stepper
and draws the nodes as inline SVG generated by the server, highlighting the current hop
by class. Walks are teaching content, so Jeff writes or approves each one.

**5. Listen to this page.** No route. A button in `wiki_page.html`, `hidden` in the
HTML, revealed by `static/js/listen.js` only when `window.speechSynthesis` exists. It
reads the article text and skips `pre` and `code` blocks. Play, pause, and stop. No
network request and no external voice service. Nothing is stored.

**6. Progress and review.** No route. Result pages for quizzes, PBQs, and drills carry
`data-` attributes with the slug, the score, and the date. `static/js/progress.js`
writes one `localStorage` key, `mg.study.v1`, holding a map of slug to a short history.
On `/study` it reads the key and shows a "due for review" list using intervals of 1, 3,
7, and 14 days. The section is `hidden` in the HTML, so with JavaScript off or storage
unavailable `/study` looks as it does now. A "clear my progress" button removes the key.

**7. Flashcard export, `/study/cards/:slug.csv`.** A GET that returns `text/csv` built
from the same `QuestionSet` the card view uses. Columns: front, back, tags. Fields are
quoted and inner quotes doubled. Axum will not match `:slug.csv` as a pattern, so the
route is `/study/cards/:file` and the handler strips a `.csv` suffix, falling through
to the existing card view when there is none. A link on the card page points at it.

**8. Wildcard and ACL trainer, `/study/drills/wildcards`.** The `wildcard` kind from
feature 1 under its own route, plus a second kind: given an ACL line and an address,
does the line match. It waits on a published `access-control-lists` page to cite.

---

## 6. Open questions for Jeff

1. **Explanation templates in feature 1.** They are short and generated, but they
   teach. The plan is that I write the four templates and you read them before slice 3
   ships. Say if you would sooner write them.
2. **Wildcard problems** wait for a page to cite. The alternative is a short wildcard
   section on the `subnetting` page, which you would write.
3. **IOS keyword lists** are checked by you against a real device, per scenario. Say if
   you want a shared starter list in `content/study/pbq/` that scenarios can extend,
   instead of each one carrying its own.
4. **Set size.** Ten problems per drill set. Easy to change.
