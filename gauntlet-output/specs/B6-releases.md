# Spec: Releases

**Feature ID:** `B6` / `releases`
**Parent feature:** root (Content surfaces — existing)
**Spec author agent:** spec-agent-9 (Claude Opus 4.8)
**Date:** 2026-08-08
**Iteration:** 1

---

## 0. Reading notes and scope boundary

Everything asserted about current state was read from source, not docs. Citations
are `path:line` or `path:line-range`.

**In scope (B6 owns):** the `/releases` route and its handler
(`src/handlers/releases.rs`), the `releases.html` template, the artifact-scanning
and checksum mechanism, the release-list presentation and its CSS
(`style.css:1268-1323`, `:1554-1560`), the contents of `static/releases/`, the
page's `<title>`/`<meta description>`/`section()` contract, and the **admission
policy** that decides what may be published here.

**Out of scope, inherited:**

| Concern | Owner | What B6 assumes |
|---|---|---|
| Colour/type/spacing tokens, 23-theme correctness, contrast audit, `.releases-empty` reference empty-state shape | `A1` design-system | `--text/--text-muted/--text-faint/--accent/--accent-hover/--border-subtle/--surface`, the type scale, and `--measure` exist and are AA-validated per theme; `.releases-empty` (`style.css:1224-1227` in A1's numbering, `:1272-1275` in the shipped file) is the canonical empty-list pattern |
| Header, nav, footer, vitals strip, skip link, 404/500, `section()` return type, `<title>` convention, no-JS fallback | `A2` site-shell | `Section` enum (A2 §4.2) includes `Releases`; every page ends its title in `" — machinageist"`; the shell renders on every route incl. errors; `<meta description>` is user-visible copy under Lens 1 (A2 §4.3 S-1, test U-7) |
| Security headers (CSP `default-src 'self'`), rate limiting, `ServeDir` static serving, HTML cache policy | `A3` ops | Every response is rate-limited and CSP-stamped; `/static/*` is served by `ServeDir` (`router.rs:59`); tarball downloads inherit those headers |
| The GeistScope publication gate and portfolio claim discipline | `docs/public-portfolio-structure.md`, `docs/geistscope-page-triage.md`, `~/mg-coreforge` PUBLIC_FACE | The gate is: full pipeline + human **and** AI operation + sanitized evidence from an authorized engagement (`geistscope-page-triage.md:5-8`). B6 must obey it, not re-litigate it. |

Where B6 needs a change in another feature's territory (a nav/portfolio link, a
shell test), it is filed as a **cross-feature request** in §7.4.

---

## 1. Purpose

### 1.1 One-sentence job

Give a visitor who already has an artifact URL a **verifiable, provenance-honest
download** — filename, exact byte size, and a full SHA-256 — for the small set of
artifacts that Jeff has cleared for public distribution, so that "here is the
thing, and here is proof it is the thing" is a first-class page rather than a
raw file drop.

### 1.2 Why it matters

The releases page is the site's smallest surface and its highest-risk one, for a
reason that has nothing to do with design.

1. **Integrity as a demonstrated habit.** Publishing a checksum next to a download
   is a concrete, defensible ops behaviour — the same discipline as verifying a
   distro ISO before flashing it. The current handler already streams SHA-256 in
   64 KB chunks off the async executor (`releases.rs:49`, `:101-115`); that is real,
   interview-defensible engineering and is the part of this feature worth keeping.
   It maps directly onto the evidence standard's *verification* field
   (`public-portfolio-structure.md:87`).

2. **It is a claim surface disguised as a file list.** A downloadable, checksummed,
   versioned `geistscope-0.4.0-source.tar.gz` is not neutral. Its contents
   (verified: `crates/mg-exploitgen`, `mg-recopilot`, `mg-takeover`, `mg-brute`,
   `mg-shodan`, `mg-breach`, `mg-leak-monitor`, `payload-engine`, …) are the exact
   crates the site's own triage archived as `Archive` / `Unsafe` / `Needs
   ownership walkthrough` (`geistscope-page-triage.md:40-88`). Distributing them
   **re-asserts the offensive-security identity** the entire 2026-07 reorg removed,
   and it does so more strongly than any tool *page* did — a page describes; a
   tarball hands over the working code. This is the pain B6 exists to resolve: the
   rest of the site is honest about GeistScope (one retrospective, gate-locked) and
   this one orphan route quietly undoes that.

3. **The mechanism outlives its current payload.** The right move is not to delete
   good code because its current input is wrong. It is to keep the mechanism, put
   an **admission policy** in front of it that is the same gate the rest of the
   site obeys, and let the page carry only what clears that gate — which today is
   nothing GeistScope, and at most a pinned snapshot of the one thing that *is* a
   claimed artifact: `mg-server` itself.

### 1.3 Success signal

**Primary (measurable):** `cargo test --all-targets` passes on a tree where a new
guard asserts that `static/releases/` contains **no artifact that fails the
publication gate** — concretely, no `geistscope-*` artifact absent a recorded,
committed gate-pass — and where the `<meta description>` names no un-cleared
capability (the A2 U-7 claim-audit vocabulary applied to this page). The four
`geistscope-*-source.tar.gz` files are gone from `static/releases/`.

**Secondary (observable):** with JavaScript fully disabled, a reader can load
`/releases`, read each artifact's name, size, and full SHA-256, and download it
via a plain `<a download>` link that hits a static file — no script anywhere on
the path. When there is nothing to publish, the page renders the designed empty
state, not a blank or a 404.

---

## 2. User Stories

> **Happy path — verifier.** As someone who was handed a link to a source
> snapshot, I want the filename, exact size, and full SHA-256 on the page next to
> the download, so that I can `sha256sum` the file I downloaded and confirm it
> matches before I trust it.

> **Happy path — engineer peer.** As an engineer who wandered to `/releases`, I
> want the page to tell me plainly what these artifacts are and that this is a
> provenance surface rather than a reviewer path, so that I read it as
> disciplined housekeeping and not as a product download page over-selling
> itself.

> **Empty state.** As a visitor arriving when nothing is cleared for
> distribution, I want a one-line "nothing here yet" rather than a blank page or
> an error, so that I know the page works and simply has no current contents.

> **Accessibility — screen reader.** As a screen-reader user, I want each release
> announced as "filename, download, size, checksum" in a sensible order with the
> long hex string flagged as a checksum, so that the SHA-256 is not read to me as
> 64 unlabelled characters mid-sentence.

> **Accessibility — no JavaScript.** As someone browsing with JS disabled, I want
> every download link to work as an ordinary link to a real file, so that the one
> functional thing on this page never depended on script.

> **Operator / claim discipline (Jeff).** As the site owner, I want it to be
> *impossible* to accidentally publish an artifact that fails the gate — a test
> should fail in CI if a `geistscope-*` tarball (or any un-cleared artifact)
> reappears in `static/releases/`, so that a stray `cp` cannot silently re-open
> the offensive-identity hole the reorg closed.

> **Operator — provenance.** As the site owner, I want the checksum shown on the
> page to be the checksum of the byte-identical file the download link serves, so
> that the integrity claim is true by construction, not by my remembering to
> regenerate a sidecar.

---

## 3. UX Specification

### 3.1 Screen / view inventory

| Surface | Path to reach | New / modified | Layout pattern |
|---|---|---|---|
| **Releases page** | `/releases` (`router.rs:48`) — **deliberately not in primary nav**; reached by direct URL or, target, a link from the `mg-server` portfolio entry (§7.4) | Modification (`templates/releases.html`) | Standard shell + single `<section>`: `h1`, intro paragraph, then either the empty-state paragraph or a divider list of release rows |
| **Release row** | Inside the list | Modification | `.release-item`: header row (filename ↔ download link) over a meta row (size · `sha256:` string), 1px `--border-subtle` dividers (`style.css:1281-1323`) |
| **Empty state** | `/releases` when the dir is empty/absent | Modification | Single `--text-muted` paragraph `.releases-empty` (`releases.html:13`, `style.css:1272-1275`) — the A1 reference empty state |
| **Artifact download** | `.release-download` `href` → `/static/releases/<file>` served by `ServeDir` (`router.rs:59`) | Modification (contents change) | Browser download of a static file; not an HTML view |

No modals, sheets, popovers, or drawers. The only overlay anywhere is the shell's
theme menu (A2), which this page does not touch. The page introduces **no new
screen** — it is one server-rendered document.

### 3.2 Interaction flows

**Primary flow — verify and download (fully JS-independent).**

1. Reader requests `/releases`. Axum routes to `releases::list` (`router.rs:48`).
2. Handler offloads the directory scan to `spawn_blocking` (`releases.rs:49`) so
   file I/O and hashing never block the async executor.
3. `scan_releases` (`releases.rs:56-98`) reads `static/releases/`, skips
   non-files (`:68`) and dotfiles (`:78`), and for each artifact computes size
   (`:83`) and a full streaming SHA-256 (`:84`, `:101-115`), sorted alphabetically
   (`:96`).
4. `ReleasesTemplate` renders the shell + the list (or the empty state).
5. Reader reads the size and SHA-256, clicks the `download` link
   (`releases.html:20`), receives the static file, and runs `sha256sum` locally
   to confirm the hex matches what the page showed.

**Branch — empty or absent directory.** If `static/releases/` does not exist,
`scan_releases` returns an empty vec and a **200** (`releases.rs:60-62`); if it
exists but holds only dotfiles, the same empty vec results. The template renders
`.releases-empty` (`releases.html:12-13`). This is the target default state once
the GeistScope tarballs are removed and nothing has yet cleared the gate.

**Branch — I/O failure.** A `read_dir`/`metadata`/`read` error, or a
`spawn_blocking` join error, becomes `std::io::Error` →
`SiteError::Io` (`errors.rs:56`) → the `other =>` arm → **themed 500**
(`errors.rs:113-116`). The shell (nav, footer, vitals) still renders on the 500
(A2 Flow D). No path, version, or stack leaks (`errors.rs:184-193`).

No haptics, no sound, no page-specific animation. The only motion in view is the
shell's (theme swap, cursor blink), all `prefers-reduced-motion`-gated by A1/A2.

### 3.3 Layout descriptions

Component hierarchy, top → bottom (`releases.html`):

1. `<h1>Releases</h1>` — the one page heading (`releases.html:5`), `--text`,
   article-scale per A1.
2. `p.section-intro` — the honest framing paragraph (`releases.html:6-10`,
   `style.css:1198-1203`, `--text-muted`, `max-width: 65ch`). **Target copy** in
   §6.3.
3. Either:
   - `p.releases-empty` "No releases posted yet." (`releases.html:13`), or
   - `ul.releases-list` (`releases.html:15-28`) of `li.release-item` rows.
4. Each `li.release-item` (`style.css:1281-1323`):
   - `.release-header` — flex row, `space-between`, baseline-aligned
     (`style.css:1288-1294`): `.release-filename` (`--text`, 700) leading; a
     `.release-download` link (`--accent`, `::before "↓ "`) trailing.
   - `.release-meta` — flex row, wrap, `--text-faint`, 0.78rem
     (`style.css:1312-1318`): human size, then `.release-sha` (`word-break:
     break-all`, `style.css:1320-1323`) carrying `sha256:` + the hex.

**Data sources.** Every row is a `ReleaseArtifact { filename, url, size_human,
sha256 }` (`releases.rs:21-26`) produced from a real file in `static/releases/`.
No literal artifact list exists in the template or handler — the filesystem **is**
the source of truth (see §5A discussion). Size and hash are computed, never typed.

**Empty state.** Copy: "No releases posted yet." rendered as one `--text-muted`
paragraph at body size — the A1 empty-state invariant (no placeholder card, no
spinner). This is the reference implementation A1 §3.3 cites, so it must not
regress into anything heavier.

### 3.4 Input & gestures

- **Pointer.** Click a `.release-download` link. Hover raises it to
  `--accent-hover` (`style.css:1309`) — additive colour only, no reflow.
- **Touch.** The download link is a normal inline anchor; on ≤640px the
  `.release-header` stacks to a column (`style.css:1554-1560`) so the filename and
  the download control are not squeezed onto one narrow line. **Target:** confirm
  the stacked download link's hit area is comfortable (it inherits the shell's
  link sizing; no page-specific target-size fix is needed because it is text, not
  an icon button).
- **Keyboard.** Each download link is a native `<a href>` — reachable by Tab,
  activated by Enter, in DOM order (filename is text, download link is the first
  and only interactive element per row). No page-specific shortcuts; none added
  (single-key accelerators are declined site-wide, A2 §3.4).
- **Specialised input.** N/A — text and links only.
- **Responsive.** One page-specific breakpoint: ≤640px stacks `.release-header`
  and `.project-header` together (`style.css:1554-1560`). The `.release-meta` row
  already wraps (`flex-wrap: wrap`, `style.css:1315`) and `.release-sha` already
  breaks anywhere (`word-break: break-all`, `:1321`), so a 64-char hash never
  forces horizontal page scroll even at 320px. This is the design-system rule
  "wide content scrolls inside its own container, the page body never scrolls
  sideways" satisfied by wrapping rather than scrolling — correct for a hash.

### 3.5 Transitions & animation

The page defines **no motion of its own**. The only animated properties in view
are the shell's colour transitions and the brand cursor blink, all inside
`@media (prefers-reduced-motion: no-preference)` (A1 §3.5, A2 §3.5). The
`.release-download` colour change on hover uses the global chrome transition
(`style.css` transition block, A1 `:713-720`) and is therefore already
reduced-motion-gated. **Reduced-motion alternative:** absence — the hover colour
still changes, it simply does not tween. Nothing on this page flashes, autoplays,
or moves in body content.

### 3.6 Error states

| # | Trigger | Presentation | Why that presentation | Recovery | Data loss |
|---|---|---|---|---|---|
| **E-1** | `static/releases/` absent or empty | **In-page** `.releases-empty` paragraph, HTTP **200** (`releases.rs:60-62`, `releases.html:12-13`) | An empty collection is a normal state, not an error — a 404 or banner would misrepresent "nothing published yet" as "page broken" | None needed; nav is present | No |
| **E-2** | `read_dir` / `metadata` / file `read` fails (permissions, disk) | **Full-page themed 500** (`errors.rs:113-116`) inside the shell | The page cannot honestly render a partial list; a half-scanned directory could hide or misreport an artifact, and integrity is the whole point | Header nav + "reboot → return home" | No |
| **E-3** | `spawn_blocking` task panics/cancels | Same as E-2 — join error mapped to `io::Error::other` then `SiteError::Io` (`releases.rs:50-51`) | Consistency; a hashing panic must not surface a wrong checksum | Header nav | No |
| **E-4** | Download link points at a file removed between render and click | `ServeDir` returns its bare 404 today (A2 E-04); **target:** the themed static-asset 404 that A2 §4.3 S-3 wires (`ServeDir::not_found_service`) | The requested file genuinely does not exist | Header nav + back | No |
| **E-5** | A checksum shown is stale relative to the served bytes | **No current signal** — checksums are recomputed per request from the same file `ServeDir` serves, so drift is structurally impossible *unless* a caching optimisation is added later (§4.7). If checksum caching lands, its cache key **must** include size+mtime and a test must pin it | Integrity is the product; a wrong hash is a claim-integrity bug, not a cosmetic one | Regenerate | No |

**Justification of choices.** E-1 is in-page-200 because absence is expected. E-2/
E-3 are full-page-500 because a releases page that cannot prove integrity must fail
closed, not render a lie. No toast/snackbar is used or proposed — toasts need JS to
appear and dismiss, which is below the no-JS floor (A1 §3.6). **Data-loss risk
across the feature: none** — the page stores nothing and reads only static files.

### 3.7 Accessibility

This section is a hard gate. Invariants first, shipped state after.

**A. No-JS (auto-fail).** The entire page — heading, intro, list, sizes, hashes,
and every download link — is server-rendered by Askama and reachable with JS
disabled. Download links are native `<a href download>` (`releases.html:20`) to
static files. There is **zero** page-specific JavaScript. ✅ shipped; §5.2 pins it.

**B. Contrast and colour independence.** All colours resolve to A1 tokens audited
AA per theme: `.release-filename` `--text` (`:1297`), `.release-download`
`--accent`/`--accent-hover` (`:1303`,`:1309`), `.release-meta`/`.release-sha`
`--text-faint` (`:1316`,`:1322`). **Finding:** `.release-meta` and `.release-sha`
render at **0.78rem** (`style.css:1317`) on `--text-faint` — small text that must
clear **4.5:1**, which is exactly the A1 §7.1.4 audit class (`--text-faint` at
sub-0.8rem). B6 inherits A1's requirement that `--text-faint` be re-tuned to 4.5:1
at ≤0.8rem across all 23 themes; until A1 lands, the size or token here is
non-compliant in the themes A1 enumerates. No state on this page is signalled by
hue alone — the download affordance carries the literal word "download" plus a
`↓` glyph (`style.css:1310`), and the checksum carries the literal `sha256:`
prefix (`releases.html:24`).

**C. Focus.** Each download link uses the global
`:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px }`
(A1 `:685`) — never removed. `--accent` clears 3:1 on `--bg` in all 23 themes
(A1 §7 C). ✅ inherited.

**D. Focus order.** Per row: filename (text, not focusable) → download link. Down
the page: intro → row 1 download → row 2 download → … . Natural DOM order; no
`tabindex`, no trap. ✅

**E. Semantics.** `ul.releases-list` is a real list, so AT announces item count
(`releases.html:15`). One `<h1>` (`:5`); no other headings (the page is a flat
list, so no `h2` is warranted — a heading over a single list would be noise).
**Target additions:**
- The download link's accessible name is currently the bare word "download"
  repeated N times — ambiguous in a rotor list. Give it context via the visually-
  hidden utility A2 defines (`.vh`): `download <span class="vh">{{ filename }}</span>`
  so each link announces "download geistscope-…" and the rotor is navigable.
  (Do **not** rely on the `download="{{filename}}"` attribute for this — it is not
  an accessible name.)
- Wrap the hash so the 64-char string is announced as a checksum, not read inline
  as characters: keep the visible `sha256:` label and add
  `aria-label="SHA-256 checksum {{ sha256 }}"` on `.release-sha`, or precede it
  with a `.vh` "checksum:". The `&#x20;` after `sha256:` (`releases.html:24`) is a
  deliberate non-breaking-context space and stays.
- The size string ("533.8 KB") is already a plain readable token; no change.

**F. Responsive / resilient.** Works 320px→wide: header stacks ≤640px
(`:1554-1560`), meta wraps, hash breaks anywhere. The empty state is a *designed*
state, not an accident (E-1). At 200% browser font / 400% zoom the row reflows via
the same rules; no horizontal scroll (§3.4). ✅ with the A1 `--text-faint` fix.

---

## 4. Implementation Specification

### 4.1 Architecture placement

```
src/
  handlers/releases.rs     ← handler, ReleaseArtifact, scan_releases, sha256_file, format_size
  router.rs                ← /releases route (:48); /static ServeDir (:59)
  errors.rs                ← SiteError::Io → 500 (already; :56, :113-116)
  shell.rs (A2, NEW)       ← Section::Releases variant that section() returns
templates/
  releases.html            ← the page body
static/
  releases/                ← the artifact directory — CONTENTS CHANGE (see §7.2)
  css/style.css            ← .releases-* / .release-* rules (:1268-1323, :1554-1560)
docs/
  public-portfolio-structure.md   ← the admission policy this feature enforces
  geistscope-page-triage.md       ← the gate wording (:5-8)
```

`releases.rs` is the only Rust module the feature owns. The single cross-module
change B6 needs is `section()` returning `Section::Releases` instead of the string
`"releases"` (`releases.rs:42-44`) — that is A2's enum migration, listed here as a
dependency, not re-specified.

### 4.2 Data model

The in-memory descriptor is already correct and stays:

```rust
// Author:      machinageist
// Date:        2026-05-17
// Description: One downloadable artifact — its name, the URL ServeDir serves it
//              from, a human-readable size, and the full lowercase SHA-256 hex.
// Notes:       Every field is derived from a real file in static/releases/; the
//              filesystem is the source of truth, so there is no hand-maintained
//              artifact list that could drift from what actually downloads.
pub struct ReleaseArtifact {
    pub filename:   String,   // exact on-disk name; also the download= value
    pub url:        String,   // "/static/releases/<filename>", served by ServeDir
    pub size_human: String,   // "1.9 MB" — see units fix below
    pub sha256:     String,   // 64-char lowercase hex of the served bytes
}
```

**No new type, no database, no migration** — the site has no persistence layer;
`static/releases/` is the entire model.

**Target correctness fixes to the existing code:**

1. **Units label.** `format_size` (`releases.rs:118-126`) divides by 1024/1048576
   but labels the result "KB"/"MB". The site's vitals strip labels base-1024
   memory as "MiB" (A2 §3.3). Align: label these **"KiB"/"MiB"** (base-1024) or
   divide by 1000/1000000 for "KB"/"MB". Pick IEC ("KiB"/"MiB") to match the
   vitals strip and be technically correct. One-line change; a unit test pins it
   (§5.1 T-3).

2. **No functional change to the scan/hash path** — `spawn_blocking` + streaming
   64 KB SHA-256 is the right shape and stays.

### 4.3 API contracts

**Route.** `GET /releases` → `releases::list` (`router.rs:48`).

| Aspect | Contract |
|---|---|
| Signature | `async fn list() -> Result<impl IntoResponse, SiteError>` (`releases.rs:48`) |
| Success | 200 with the rendered page (populated or empty) |
| Error | 500 (themed) on any I/O or join failure via `SiteError::Io` (`errors.rs:56`,`:113-116`) |
| Auth | None — public, read-only |
| Rate limit | Inherited from the global limiter (`router.rs:72-75`); important here because the handler does work proportional to total artifact bytes (§4.7) |
| Pagination | N/A — the artifact set is tiny by policy (a handful, gate-limited) |

**Template contract (A2 S-1), target values:**

| Method | Current (`releases.rs:36-44`) | Target |
|---|---|---|
| `title()` | `"Releases — machinageist"` | unchanged — already ends in `" — machinageist"` ✅ |
| `description()` | `"GeistScope source tarballs and compiled binaries."` **← claim defect** | rewrite: names no un-cleared capability, no "GeistScope", no "binaries" (none ship) — see §6.3 |
| `section()` | `"releases"` (`&str`) | `Section::Releases` once A2's enum lands |

**Static download contract.** `/static/releases/<file>` is served by `ServeDir`
(`router.rs:59`) with path sanitisation (traversal → 404, `router.rs:12`) and the
site's security headers (A3). **Target:** the `.not_found_service` themed-404 wiring
(A2 §4.3 S-3) so a removed-mid-session file returns the themed 404 (E-4).

**Admission policy (the load-bearing contract this spec adds).** An artifact may
exist in `static/releases/` **only if** it clears the site-wide publication gate:

- It is Jeff's owned work, defensible against the evidence standard
  (`public-portfolio-structure.md:76-94`).
- If it is GeistScope-derived, it has passed the gate verbatim: *full pipeline +
  human and AI operation + sanitized evidence from an authorized engagement*
  (`geistscope-page-triage.md:5-8`), recorded in a committed decision.
- It carries no capability the site's claim discipline forbids leading with
  (pentest/red-team/offensive, "production-grade", "enterprise", SRE —
  `public-portfolio-structure.md:106-113`).

This policy is **enforced by a test**, not by hoping (§5.1 T-1), because the
current state proves the honour system failed.

### 4.4 State management

| State | Owner | Lifetime | Sync |
|---|---|---|---|
| Artifact list | `static/releases/` on disk, read per request | Process/disk | Server only; never client state |
| Per-request descriptors | Local `Vec<ReleaseArtifact>` in `list()` | Per request | None |
| Theme (only client state on the page) | `localStorage.theme` (shell) | Browser | Client only |

**No new state container.** The page is stateless request→scan→render. There is
deliberately no cache in the shipped code (checksums recomputed each request);
adding one is an optional perf improvement with a correctness obligation (§4.7,
E-5). **Offline/draft persistence:** N/A — nothing is authored in the browser.

### 4.5 Dependencies

- **New packages:** none. `sha2 = "0.10"` and `hex = "0.4"` are already in
  `Cargo.toml:54-55`, added for exactly this feature.
- **Assets removed:** `static/releases/geistscope-0.1.0-source.tar.gz`,
  `-0.2.0-`, `-0.3.0-`, `-0.4.0-source.tar.gz` (total ~1.9 MB; verified present,
  `ls static/releases/`). They remain in git history and in the GeistScope repo —
  removal is from the *public distribution surface*, not destruction of the work.
- **Assets added:** none required. Optionally, one gate-cleared artifact (the
  strongest candidate is a pinned `mg-server` source snapshot — the one Active
  portfolio entry, already public at `github.com/machinageist/mg-server`), but the
  page is fully valid empty.
- **Infrastructure:** none. No CDN (CSP `default-src 'self'`), no new services.

### 4.6 Platform-specific considerations

- **`spawn_blocking`** keeps hashing off the async executor (`releases.rs:49`) —
  correct and portable.
- **`/proc`-style assumptions:** none — this handler reads ordinary files, so it
  behaves identically on the macOS dev box and the Debian VM (unlike the vitals
  strip's Linux-only RSS).
- **Browser support:** the `download` attribute (`releases.html:20`) is universally
  supported; where unsupported the link still navigates to the file, which the
  browser then downloads or displays — graceful degradation, no break.
- **CSP:** the page needs no script and no external resource, so it is trivially
  inside `default-src 'self'`. Any future enhancement must not reach for inline
  script (A2 §4.6).
- **Feature flags / rollout:** N/A — single binary, single deploy.

### 4.7 Performance budget

| Dimension | Current | Note / target |
|---|---|---|
| Per-request CPU | **O(total artifact bytes)** — every hit re-hashes every file (`releases.rs:84`, `:101-115`). Today ~1.9 MB across 4 files ≈ negligible, but it is unbounded in artifact size and count | **Risk:** an unauthenticated endpoint that forces N MB of SHA-256 per request is a small amplification vector; the global rate limiter (`router.rs:72-75`) caps it. **Target (optional):** cache `sha256` keyed by `(path, len, mtime)`, or read a committed sidecar `<file>.sha256`. **Correctness obligation:** any cache key must include size+mtime or the shown hash can go stale (E-5), pinned by a test |
| Memory | Streaming 64 KB buffer per file (`releases.rs:104`); files never fully loaded | Correct; keep. Do not switch to `fs::read` + hash |
| Network (page) | Tiny HTML: N rows × ~120 B. Empty state is ~1 line | Negligible |
| Network (download) | The artifact size itself | Served by `ServeDir` with range support; unaffected by this spec |
| Startup | None — directory scanned lazily per request, not at boot | Keep lazy; a boot-time scan would add nothing |
| Storage (server) | The artifact files. **Target reduces it by ~1.9 MB** (tarballs removed) | — |
| Storage (client) | None beyond the shell's one `localStorage` key | — |

**Rejected optimisation, recorded:** precomputing all checksums at startup would
move the cost from per-request to per-boot but re-introduces staleness if a file
changes without a restart, and the site restarts rarely. The `(path,len,mtime)`
cache is strictly better and is the recommended form if hashing ever becomes hot.

---

## 5. Test Specification

All tests run under `cargo test --all-targets` and gate CI
(`.github/workflows/ci.yml`: `fmt → clippy -D warnings → test → build --release`).
`releases.rs` currently has **no `#[cfg(test)]` module** (verified) — this is the
largest test gap of any shipped content surface and the one most coupled to claim
integrity.

### 5.1 Unit tests — new `#[cfg(test)] mod tests` in `src/handlers/releases.rs`

| # | Name | Setup | Assertion | Covers |
|---|---|---|---|---|
| T-1 | `no_uncleared_artifact_is_published` | Read the real `static/releases/` directory entries | **No filename matches `geistscope-*`** (nor any name on an `UNCLEARED` denylist) unless a committed `static/releases/CLEARED.txt` records an explicit gate-pass for it | **The core claim guard** — auto-fail rule 1 / criterion 1C. Fails today (4 tarballs); passes after removal |
| T-2 | `description_carries_no_retired_or_uncleared_claim` | `ReleasesTemplate{artifacts:vec![]}.description()` | Does not contain `"GeistScope"`, `"binaries"`, `"offensive"`, `"red-team"`, `"pentest"`, `"bug-bounty"`, `"production-grade"`, `"enterprise"`, `"SRE"` | **Fails today** — `description()` says "GeistScope source tarballs and compiled binaries." (`releases.rs:39-41`). Mirrors A2 U-7 for this page's `<meta>` |
| T-3 | `format_size_uses_iec_units` | `format_size(0)`, `1023`, `1024`, `1_048_576`, `1_600_000` | Returns `"0 B"`, `"1023 B"`, `"1.0 KiB"`, `"1.0 MiB"`, `"1.5 MiB"` (or the chosen decimal-unit variant) — the label matches the divisor | The MB/KB-with-1024-divisor mismatch (`releases.rs:118-126`) |
| T-4 | `dotfiles_and_nonfiles_are_skipped` | A temp dir with `.gitkeep`, `.DS_Store`, a subdir, and one real file | Result contains exactly the one real file | `releases.rs:68`, `:78` behaviour pinned |
| T-5 | `missing_dir_is_empty_not_error` | Point the scan at a non-existent path | `Ok(vec![])`, not `Err` | `releases.rs:60-62` — the 200-empty contract (E-1) |
| T-6 | `sha256_matches_known_vector` | Hash a temp file containing `"abc"` | Equals `ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad` | The streaming hasher (`:101-115`) is correct — the integrity claim depends on this being true |

T-1 and T-5 read the real directory, so T-1 fails until the tarballs are removed
and passes after — it is the executable form of this spec's success signal.

### 5.2 Integration tests — router-level (`tower::ServiceExt::oneshot`, the pattern at `errors.rs:171-182`)

| # | Name | Assertion |
|---|---|---|
| I-1 | `releases_renders_full_shell` | `GET /releases` → 200; body has the skip link, `<header class="site-header"`, `aria-label="Primary"`, `<main id="content"`, `<footer class="site-footer"`, `vitals-strip` (extends A2 I-1 to this route) |
| I-2 | `releases_needs_no_javascript` | `GET /releases`; strip all `<script>` elements; body still contains every download `href` present, the `sha256:` labels, and the intro copy | The machine-checkable no-JS floor (auto-fail rule 3) |
| I-3 | `releases_is_not_in_primary_nav` | `GET /` (or any page) → the rendered `.nav-links` block contains no `/releases` href | **Makes real the guard `REORG_CHANGELOG.md:90` and `REORG_HANDOFF_PROMPT.md:57` claim exists but which is absent** (verified: no test in `src/` references releases) — criterion 5C |
| I-4 | `empty_releases_shows_designed_empty_state` | With `static/releases/` empty, `GET /releases` → 200 body contains "No releases posted yet." and no `.release-item` | E-1; A1 empty-state invariant |
| I-5 | `download_link_targets_static_and_escapes` | For any artifact, its `href` starts `/static/releases/` and the filename is HTML-escaped by Askama | Path/URL correctness; XSS via a crafted filename |

### 5.3 UI / E2E tests

**Absent, and deliberately so** — consistent with A1/A2. There is no browser
harness in the repo, and this page has no JavaScript to drive. The behaviours an
E2E suite would cover (link works, focus ring visible) are covered by I-2
(served bytes) and the shell's own focus rules. Stated as a decision, not an
omission.

### 5.4 Visual / manual verification

Per A1's tiered matrix. **Tier 1 (six themes: Lunarcore, Solarcore, Paper, Cloud,
Solarized, CRT)** on:

- **Populated** `/releases` (temporarily drop one gate-cleared file, e.g. an
  `mg-server` snapshot) — check filename/size/hash row wrapping, the `↓` download
  affordance, and `--text-faint` legibility of the hash on both `--bg` and (n/a
  here — the list sits on `--bg`) at 0.78rem.
- **Empty** `/releases` — the reference empty state renders as one quiet line.
- **500** (force an I/O error) — the shell survives, no leak.

Configuration extremes: browser default font 24px + 200% zoom (hash must wrap, not
overflow); viewport 320px (header stacks, no horizontal scroll); `reduce` motion
(no change — page has no motion); JS disabled + `prefers-color-scheme: light`
(page renders in Solarcore, download works).

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **No sensitive data involvement** *for the page itself.*

The page stores nothing, sets no cookie, reads no user input, and transmits no
personal data. **However**, the *artifacts it distributes* are a
capability-exposure concern, not a data-privacy one: the current GeistScope
tarballs ship working offensive-security tooling source. That is handled as a
claim-integrity and admission-policy problem (§6.3, §4.3), which is where it
belongs, rather than as a PII question.

### 6.2 Asset provenance

- [x] **Uses third-party / mixed-provenance assets** — the distributed files.

| Artifact (current) | Origin | Status |
|---|---|---|
| `geistscope-0.1.0…0.4.0-source.tar.gz` | Jeff's own GeistScope repo, AI-assisted | **Fails the publication gate** (`geistscope-page-triage.md:5-8`); contents are the archived/unsafe crate set (`mg-exploitgen`, `mg-recopilot`, `mg-takeover`, …, verified via `tar tzf`). **Remove.** They are Jeff's work (no external-licence issue) but they are gate-blocked, and bundled third-party crate deps inside the tarball would additionally need a licence pass before any public redistribution |
| Target: `mg-server` source snapshot (optional) | This repo, Jeff's own | Gate-clearable — it is the one Active portfolio entry, already public. If shipped, note its bundled Rust crate licences are permissive (MIT/Apache-2.0) as with any Cargo project |

The releases *mechanism* (streaming SHA-256, spawn_blocking) is original work in
this repo. No fonts, images, or third-party UI assets are involved.

### 6.3 Language / claims audit

- [x] **Makes a claim not supported by evidence — YES, in current shipped copy.**
  `description()` = "GeistScope source tarballs and compiled binaries."
  (`releases.rs:39-41`) is doubly defective: it leads with **GeistScope** (a
  capability pulled from the public surface by the reorg) and claims **compiled
  binaries** that do not exist (only source tarballs ship). This is a Lens 1B/1D
  violation and it is *invisible* because it lives in a `<meta>` tag no page-body
  test guards — exactly the A2 U-7 gap. **Target copy** below fixes it.
- [x] **Promises a capability not built — NO** (after fix).
- [x] **Uses domain-restricted language — the tarball *contents* do**, by handing
  over pentest tooling; removal resolves it.

**Target `description()`:** e.g. *"Checksum-verified downloads for artifacts
cleared for public distribution."* (names the mechanism, no capability claim, no
"GeistScope", no "binaries"). Passes T-2.

**Target `.section-intro` copy** (already softened by `edaef94` but keep it and
tighten): *"Downloadable artifacts with published SHA-256 checksums for
verification. This is a provenance page, not a primary reviewer path — start at
Portfolio or Writing. Verify checksums before running anything."* — quiet,
show-don't-tell, no strategy narration (honours the mg-server copy-voice memory).

### 6.4 Regulatory alignment — Lens 3 (Accessibility & Progressive Enhancement)

| Criterion | Addressed |
|---|---|
| **3A works without JS** | ✅ zero page JS; server-rendered list; native `<a download>`; pinned by I-2 (§3.7 A) |
| **3B contrast / colour independence** | Inherits A1 tokens; **carries the `--text-faint` @0.78rem 4.5:1 obligation** (A1 §7.1.4) for `.release-meta`/`.release-sha`; state carried by the words "download"/`sha256:`, never hue (§3.7 B) |
| **3C keyboard & focus** | Native links, global `:focus-visible`, natural order (§3.7 C/D) |
| **3D semantics & AT** | Real `<ul>`; one `<h1>`; **target** `.vh` filename context on each download link and a checksum aria-label so the hash is not read as loose characters (§3.7 E) |
| **3E motion & sensory** | No page motion; shell motion is `reduce`-gated (§3.5) |
| **3F responsive & resilient** | 320px→wide via header stack + meta wrap + hash break; designed empty state (§3.7 F) |

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**State: implemented, but distributing gate-failing content.**

- **Route:** `/releases` → `releases::list` (`router.rs:48`). ✅ implemented.
- **Handler:** full scan + streaming SHA-256 + human size, `spawn_blocking`,
  graceful empty-dir → 200, dotfile skip, alpha sort (`releases.rs:48-98`). ✅
  well-built.
- **Template + CSS:** complete, themed, responsive, with a reference empty state
  (`releases.html`, `style.css:1268-1323`,`:1554-1560`). ✅ implemented.
- **Content:** four `geistscope-*-source.tar.gz` (0.1.0–0.4.0) in
  `static/releases/` whose contents are the archived/unsafe GeistScope crate set.
  ❌ **fails the publication gate (1C) and trips auto-fail rule 1.**
- **`<meta description>`:** "GeistScope source tarballs and compiled binaries."
  ❌ **claim defect** (1B/1D): un-cleared capability + non-existent "binaries".
- **Discoverability:** orphan — not in nav (`base.html:23-27`), not linked from
  any page or content file (verified: `grep -rn releases templates/ content/`
  returns only self-references). Deliberate per the reorg, but the guarding test
  the reorg docs claim exists (`REORG_CHANGELOG.md:90`) **does not exist**
  (verified). ❌ documented drift guard is absent (5C).
- **Tests:** none (`releases.rs` has no test module). ❌ absent.
- **Units:** "MB"/"KB" labels on base-1024 divisors (`releases.rs:118-126`). ⚠️
  accuracy nit.

### 7.2 Delta to spec

**Content / claim (the blocking work):**
- Remove `static/releases/geistscope-0.1.0…0.4.0-source.tar.gz` (4 files).
- Rewrite `description()` (`releases.rs:39-41`) per §6.3.
- Tighten `.section-intro` copy (`releases.html:6-10`) per §6.3.

**Guards (make the policy self-enforcing):**
- Add `#[cfg(test)] mod tests` to `releases.rs` with T-1…T-6 (§5.1).
- Add I-1…I-5 (§5.2), including I-3 — the never-implemented "not in nav" guard.

**Correctness / a11y:**
- Fix `format_size` units → IEC (`releases.rs:118-126`).
- Add `.vh` filename context + checksum aria-label in `releases.html:20`,`:24`.
- Inherit A1's `--text-faint` 4.5:1 fix (no local change; dependency).
- Adopt A2's `section() -> Section::Releases` and the `ServeDir`
  `not_found_service` themed-404 (both A2-owned; §7.4).

**Optional (only if hashing becomes hot):**
- `(path,len,mtime)` checksum cache with a staleness test (§4.7, E-5).
- Ship one gate-cleared `mg-server` snapshot + link it from the portfolio entry.

**Docs (5E — documentation follows behaviour):**
- Update `docs/public-portfolio-structure.md` "Still open" to record that
  `/releases` is now gate-governed and empty by default.
- Correct `REORG_CHANGELOG.md:90` / `REORG_HANDOFF_PROMPT.md:57` from "a test
  asserts that" to reference the now-real I-3, or note the claim was aspirational
  until this change.

### 7.3 Estimated scope

**S–M.** The removals and copy fixes are trivial (an afternoon). The test module
(T-1…T-6, I-1…I-5) is the bulk and is straightforward Rust in the established
in-file `#[cfg(test)]` pattern. The a11y `.vh` additions are template one-liners.
The only judgment cost is the product decision in §8 (empty vs. mg-server
snapshot vs. retire the route), which is Jeff's, not the implementer's. No new
dependency, no migration, no infra.

### 7.4 Blocking dependencies

- **A2 site-shell** — `Section::Releases` enum variant (A2 §4.2) and the
  `ServeDir::not_found_service` themed-404 (A2 §4.3 S-3). B6's `section()` and E-4
  target both live in A2. **Cross-feature requests:** (1) include `Releases` in
  the `Section` enum; (2) if/when a gate-cleared `mg-server` snapshot ships, add a
  "Download source snapshot (sha256 verified)" link from the `mg-server` portfolio
  entry (`B3` portfolio / `src/models/project.rs`) so the page is discoverable
  honestly without entering primary nav.
- **A1 design-system** — the `--text-faint` ≥4.5:1-at-small-size re-tune (A1
  §7.1.4). Until it lands, the hash/meta rows are non-compliant in the themes A1
  enumerates. No B6-local CSS change; the fix is a token change A1 owns.
- **External gate** — publishing *any* GeistScope-derived artifact here is blocked
  by the publication gate itself (`geistscope-page-triage.md:5-8`); nothing
  GeistScope currently clears it, so the correct near-term content is *none*.

---

## 8. Open Questions

- **Q1 — What does `/releases` carry after the tarballs are removed?** Options:
  (a) empty by default until something clears the gate (safest, fully honest);
  (b) ship a pinned, checksummed `mg-server` source snapshot (the one Active
  portfolio entry — turns the page into a live integrity demo);
  (c) retire the route entirely (delete handler, template, route, CSS) if it will
  realistically never carry compliant content.
  **Blocks:** §4.5, §7.2 optional work, §7.4 cross-feature request. **Recommendation:**
  (b) — it keeps the good engineering *and* gives it honest, gate-cleared content,
  and the mg-server snapshot is the strongest possible demonstration of the
  verification discipline the criteria reward. Falls back cleanly to (a).

- **Q2 — Discoverability if kept.** If (a)/(b), stay out of primary nav (aligns
  with the reorg and A2's 4-item nav) and link only from the mg-server portfolio
  entry? Or leave it a pure orphan reachable by URL? **Blocks:** §7.4 request 2.
  **Recommendation:** link from the portfolio entry under option (b); pure orphan
  under option (a).

- **Q3 — Checksum caching.** Adopt the `(path,len,mtime)` cache now, or defer
  until the artifact set is large enough to matter? **Blocks:** §4.7. The set is
  tiny under the gate, so this is genuinely deferrable; recorded so it is a
  decision, not an oversight.

- **Q4 — Disposition of the removed tarballs.** Confirmed they leave the public
  surface but stay in git history and the GeistScope repo — is that the intended
  archival, or should they be scrubbed from history too? **Blocks:** nothing in
  this spec (history is out of scope), raised for completeness.

- **Q5 — Sub-feature note (no sub-agents spawned, per rules).** None of the above
  requires decomposing B6 into child features. If Q1 resolves to (b), the
  "generate + pin an `mg-server` release snapshot" task is a small build/release
  chore, not a product sub-feature, and would live in the deploy pipeline
  (explicitly out of scope per the feature tree), not under B6.
