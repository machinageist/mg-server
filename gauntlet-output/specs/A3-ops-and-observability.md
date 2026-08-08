# Spec: Ops and Observability

**Feature ID:** ops-and-observability
**Parent feature:** root
**Spec author agent:** Spec Gauntlet agent 3 (A3)
**Date:** 2026-08-07
**Iteration:** 1

---

## 0. How to read this spec (and how its claims were checked)

Every "current state" claim below was checked against source I read, and the
runtime claims were checked against a running binary. The runtime evidence was
gathered by starting a second instance of the debug binary on `127.0.0.2:3000`
(the port is hardcoded at `src/main.rs:45`, but `MG_BIND_ADDR` accepts any
loopback address, so a second instance can run beside a live one) and probing it
with `curl`. Raw captures appear inline where a claim depends on them.

Vocabulary used strictly, per criterion 1B:

| Word | Meaning here |
|---|---|
| **implemented** | Code exists, runs, and I observed the behavior |
| **prototyped** | Code exists but is unwired, untested, or unreachable |
| **planned** | This spec proposes it; nothing exists yet |
| **gated** | Deliberately blocked by a recorded decision |
| **absent** | Does not exist and is not proposed |

---

## 1. Purpose

### 1.1 One-sentence job

Make the single Rust process behind machinageist.dev **observable to its
operator, defensible to a reviewer, and boring to attack** — by stamping
defensive response headers on every response, throttling floods, counting its own
work, publishing a small honest readout of that work at `/status`, and answering
scanners at the RFC-mandated well-known paths.

### 1.2 Why it matters

Three distinct pains, in the order they bite.

**Operator pain.** Jeff runs one Axum process on a Debian VM inside his Proxmox
box, reached through a Cloudflare Tunnel and Caddy. When it misbehaves he has
`systemctl`, `journalctl`, and a browser. There is no monitoring stack, no
alerting, and no tested restore — the "How machinageist.dev Is Hosted" post says
so plainly (`content/posts/hosting-machinageist-dev.md:101-113`). `/status` is
the cheapest possible answer to "is the process I think is running the process
that is actually running, and how long has it been up?" — answerable from a phone
without SSH.

**Reviewer pain.** Criterion 4B is the whole competitive thesis: junior homelab
portfolios "rarely show verification, failure, or recovery." A live readout that
resets to `UP 00:00:00` after a restart is an admission, not a trophy. It is one
of the few things on a personal site that can be *falsified by the reader* —
refresh it and the request counter moves, or the claim was a lie. That
falsifiability is the differentiator.

**Defensive pain.** The site has no login, no database, and no user input, so the
realistic threat is not a targeted attacker — it is background internet noise:
scanners walking `/.env`, `/wp-login.php`, and `/admin`, and crawlers pulling
assets. Response headers, a flood throttle, an error page that leaks nothing, and
a `security.txt` that gives a real human a real inbox are the proportionate
answer. That is the honest framing, and it is the framing the two published posts
already use ("header hardening on a personal site, not a claim that the
application is 'secured'" — `content/posts/security-headers-on-machinageist-dev.md:9-14`).

### 1.3 Success signal

**Primary, measurable:** every response the server emits — `200`, `404`, `405`,
`429`, `500`, static hit, static miss — carries the full six-header defensive set
and no `Server` header, proven by one table-driven test over the real router and
reproducible on the live site with a single `curl -I`.

Today that is **false for exactly one case**: a throttled `429` carries *zero*
security headers. See §7.1 finding F1.

**Secondary, observable:** after a deploy, `curl -s https://machinageist.dev/status.json | jq .build`
returns the timestamp of the binary Jeff just built. If it returns the old one,
the deploy did not take — which is exactly the class of failure that produced the
`203/EXEC` incident.

---

## 2. User Stories

> As **the operator (Jeff, on his phone, away from the desk)**, I want to load
> `/status` and see uptime, request count, memory, and build stamp, so that I can
> tell "restarted an hour ago" apart from "been up for nine days" without
> SSH-ing into the VM.

> As **a hiring manager skimming for thirty seconds**, I want the footer strip to
> show real, moving numbers from the process rendering the page, so that I can
> tell this is an operated system rather than a screenshot of somebody else's
> dashboard.

> As **an engineer peer who does not trust portfolio sites**, I want to run
> `curl -sSI https://machinageist.dev` and see the headers the blog post claims
> are set, so that the writing is checkable rather than assertable.

> As **a security researcher who found something**, I want
> `/.well-known/security.txt` to give me a monitored contact and a non-expired
> `Expires` field, so that I can report it without hunting for an email address —
> and so that my scanner does not reject the file as stale.

> As **a screen reader user**, I want the `/status` readout to be a real
> description list whose terse labels (`UP`, `REQ`, `MEM`) are disambiguated by
> the text that follows them, so that "up" is not announced as a bare preposition
> with a number after it.

> As **a visitor with JavaScript disabled**, I want `/status`, `/status.json`, the
> footer vitals strip, and the 404 page to render completely, so that the site's
> claim to be a server-rendered no-JS site holds on its most "dashboard-looking"
> page. *(Verified: `curl -sS http://127.0.0.2:3000/status` returns the full
> readout and the strip with values populated; total site JS is 95 lines across
> `static/js/theme-init.js` and `static/js/main.js`, theme selection only.)*

> As **a badly-behaved crawler**, I want to be told plainly in `robots.txt` that
> `/static/` is off limits to AI training agents, and I want to be throttled with
> a `429` and a `Retry-After` when I ignore that, so that backing off is the
> obvious behavior rather than a guess.

---

## 3. UX Specification

**Ownership boundary.** Per the A3 assignment, this spec owns the **data,
guarantees, copy, and semantics** of these surfaces. It does **not** own the
visual design of `/status` (A1/A2) or the footer strip markup (A2). Where a
requirement can only be satisfied by a change in an A1/A2 file, it is stated as a
binding requirement here and listed as a handoff in §7.4.

### 3.1 Screen / view inventory

| Surface | Path | New / modified | Layout pattern | Owner of visuals |
|---|---|---|---|---|
| Status page | `/status` | modified | Full-width prose column, 700px, one `<dl>` readout panel | A1/A2 |
| Status JSON | `/status.json` | modified | No UI — `application/json` body | A3 |
| Footer vitals strip | every page, `templates/vitals_strip.html` | modified (data + a11y semantics only) | Single flex row in `<footer>` | A2 |
| 404 page | any unmatched path (router fallback) | modified | Boot-log column, staged reveal | A1/A2 |
| 404 for missing static asset | `/static/<missing>` | **modified — currently a blank page** | Same as above once fixed | A1/A2 |
| 500 page | any handler error | modified (title only) | Boot-log column | A1/A2 |
| 429 throttle response | any path once the bucket empties | **modified** | `text/plain` body, no HTML | A3 |
| 405 response | non-GET on a known route | unchanged (documented, accepted) | Empty body + `Allow` | A3 |
| `security.txt` | `/.well-known/security.txt`, `/security.txt` | modified | `text/plain` | A3 |
| `robots.txt` | `/robots.txt` | unchanged (guard added) | `text/plain` | A3 |

No new navigable screen is introduced. **No dashboard, `/stats` route, nav item,
or homepage section is proposed** — `docs/plans/deferred-dashboard-notes.md`
gates that concept until Jeff has answered its six revisit questions in his own
terms, and this spec respects the gate.

### 3.2 Interaction flows

**Flow A — operator checks the process.**
1. Visitor requests `/status`. Request enters `TraceLayer` → security headers →
   rate limiter → vitals counter → `status::page`.
2. `status::page` takes `Status::current()` — one snapshot read at request time
   (`src/handlers/status.rs:46-53`). It is a stamp, not a feed.
3. Response returns `Cache-Control: no-store` so no intermediary (Cloudflare,
   Caddy, browser) serves a stale readout. *Already implemented and tested —
   `src/handlers/status.rs:20` and the test at `:126-135`.*
4. Page renders. The reader refreshes; `REQ` increments by at least one. That
   increment is the proof.

**Branch A1 — RSS unavailable.** `state::rss_mib()` reads `/proc/self/status`
and returns `None` off Linux or on any read/parse failure
(`src/state.rs:262-272`). The page renders `MEM — not available / RSS readout
requires Linux` (`templates/status.html:28-29`); the footer strip omits the MEM
item and its separator entirely (`templates/vitals_strip.html:11-16`); the JSON
serializes `"rss_mib": null`. No error, no panic, no empty box. A failed sample
is cached for the full 5 s TTL so a broken `/proc` does not become a syscall
storm — proven by the test at `src/state.rs:393-408`.

**Branch A2 — global state not published.** `Status::current()` reads a
process-global `OnceLock` (`src/state.rs:137`, `:239-254`). If `init_global` was
never called it degrades to zeros and `bind: "unknown"` rather than panicking. In
the real binary `main.rs:50` publishes before `router::build`, so this branch is
only reachable in tests.

**Flow B — flood and recovery.**
1. Client exceeds the bucket. `rate_limit` returns `429` **without** running the
   handler chain (`src/middleware/rate_limit.rs:66-72`).
2. `tracing::warn!("rate limit exceeded")` fires. This is the one thing that
   *does* appear in journald at the default log level today.
3. **Target:** the `429` carries `Content-Type: text/plain; charset=utf-8`,
   `Retry-After: <seconds>` computed from governor's `NotUntil`,
   `Cache-Control: no-store`, and the full security header set.
4. Client waits `Retry-After` and retries. Tokens refill at 1/s.

**Flow C — scanner finds the disclosure contact.** `GET /.well-known/security.txt`
→ `text/plain`, `Cache-Control: public, max-age=3600`, four RFC 9116 fields.
`/security.txt` serves the identical body (`src/router.rs:54-55`). **Target adds**
a second `Canonical` line so the root URI is also declared canonical, per RFC 9116
§2.5.2 — a strict validator that fetches `/security.txt` today sees a `Canonical`
that does not include the URI it fetched.

**Flow D — visitor hits a dead link.** Unmatched path → `errors::fallback_404`
→ themed boot-log 404 echoing the requested path, HTML-escaped
(`src/errors.rs:139-141`, escaping proven at `src/errors.rs:160-169`).

**Branch D1 — the dead link is under `/static/`.** `nest_service` hands the
request to `ServeDir`, whose own not-found response wins; the router fallback
never runs. **Verified:**

```console
$ curl -sS -D- -o body.txt http://127.0.0.2:3000/static/nope.css
HTTP/1.1 404 Not Found
content-security-policy: ...
content-length: 0
$ wc -c < body.txt
0
```

A completely blank page. Criterion 3F requires empty states to be designed rather
than accidental; this one is accidental. **Target:** attach the themed 404 as
`ServeDir`'s not-found service.

**Cues.** No haptics, no sound. The only animation anywhere in this feature is
the 404/500 staged boot-line reveal, covered in §3.5.

### 3.3 Layout descriptions

**`/status`** (`templates/status.html`) — vertical stack in a 700px column:

1. `<h1>Status</h1>`
2. Intro paragraph explaining what the numbers are and where they come from.
3. `<dl class="status-readout">` — a two-column grid (`max-content 1fr`,
   `style.css:858-868`) of six `<dt>`/`<dd>` pairs. `<dt>` is the terse label;
   `<dd>` is the value plus a `<span class="status-note">` gloss.
4. Outro paragraph linking `/status.json` and stating the retention posture.

Data source: one `Status` struct (`src/state.rs:226-235`), which is the
**allowlist** — nothing renders here that is not a field on that struct.

| `<dt>` | `<dd>` value | Gloss (target wording) | Source |
|---|---|---|---|
| `UP` | `dd:hh:mm` | `uptime — dd:hh:mm since process start` | `started_at.elapsed()` |
| `REQ` | integer | `requests served this process lifetime` | `AtomicU64` total |
| `MEM` | `N MiB` **or** `not available` | `resident set size, read from /proc/self/status` / `memory readout requires Linux` | `/proc` + 5 s cache |
| `VER` | `v0.1.0` | `crate version` | `CARGO_PKG_VERSION` |
| `BUILT` | `YYYY-MM-DD HH:MM UTC` | `build time, stamped at compile time by build.rs` | `BUILD_TS` |
| `BIND` | e.g. `loopback (IPv4)` | `resolved listener exposure; custom addresses stay private` | `BindMode` |

The gloss rewrites are the a11y fix in §3.7: each note now **leads with the noun**
the abbreviation stands for, so a screen reader announcing "UP, 00:04:12, uptime,
dd:hh:mm since process start" recovers the meaning without any new markup, new
CSS utility class, or `aria-label`. This is the restrained fix (criterion 2E):
copy, not chrome.

**Empty state.** `/status` has no empty state — every field always has a value or
an explicit "not available". The MEM `None` branch *is* the designed empty state.

**Footer vitals strip** (`templates/vitals_strip.html`, A2's markup) — one flex
row: `UP {{uptime}} · REQ {{requests}} · [MEM {{n}} MiB ·] v{{version}} · built
{{build}}`, the last item a link to `/status`.

**429 body.** Plain text, one line: `too many requests`. **Deliberate:** rendering
a themed HTML 429 would burn the exact CPU and bytes the limiter exists to
protect, and would grow a ~10 KB response into the flood path. The reader who
sees it is nearly always a script. A human who sees it gets a `Retry-After` their
browser understands. Stated as a tradeoff, not an oversight.

### 3.4 Input & gestures

Every surface in this feature is **read-only**. There are no forms, no buttons,
no drag targets, no stylus/voice/camera input, and no feature-specific keyboard
shortcuts to define.

- **Click/touch:** two links total — `/status.json` from the status outro, and
  `/status` from the footer strip. Both are ordinary `<a href>`; both work with
  JS disabled; both are standard browser targets (context menu, middle-click,
  copy-link all behave natively because nothing intercepts them).
- **Keyboard:** the two links are in DOM order and reachable by Tab. Nothing in
  this feature adds, removes, or reorders focusable elements, with one exception
  fixed in §3.7 (the 404 recovery link).
- **Responsive:** `.status-readout` is `grid-template-columns: max-content 1fr`,
  which holds down to a narrow viewport because the `<dt>` column is at most six
  characters. `.vitals-strip` sets `flex-wrap: wrap` (`style.css:816-828`), so
  the strip wraps rather than overflowing. Requirement: no horizontal body scroll
  at 320 px at 200 % zoom; the `<dd>` values are short enough that this holds
  without a media query.

### 3.5 Transitions & animation

`/status`, the vitals strip, `security.txt`, `robots.txt`, `status.json`, and the
`429` have **no animation of any kind**, by design (criterion 2E: spectacle is
budgeted to chrome; a readout is body content).

The 404 and 500 pages animate a staged boot-log reveal: five lines fade in at
0.15 s / 0.55 s / 1.0 s / 1.35 s / 1.8 s (`style.css:1279-1290`).

**Reduced motion — already correct, verified.** The entire animation block sits
inside `@media (prefers-reduced-motion: no-preference)` and the keyframe uses
`animation-fill-mode: both` with the hidden state living *inside* the animation.
The consequence, which the source comment at `style.css:1275-1278` states
explicitly: if animations never run, the page is simply fully visible. That is
the correct construction — no opacity:0 stranded in the base rule.

**One defect, and it is mine.** The 404's recovery link `(A)bort → return home`
is `.boot-line-5` (`templates/error_404.html:9`) with `animation-delay: 1.8s`.
A keyboard user who lands on the 404 and presses Tab immediately focuses a link
that is at `opacity: 0` for up to 1.8 seconds. Focus is technically correct but
invisible.

**Requirement:** focusing anything inside `.error-page` must cancel the reveal for
that element. Implementation (A1's file, one rule):

```css
.error-page a:focus-visible { animation: none; opacity: 1; }
```

### 3.6 Error states

| # | Trigger | Presentation | Justification | Recovery | Data loss |
|---|---|---|---|---|---|
| E1 | Unmatched route | Full-page themed 404, echoes the requested path (escaped) | The user asked for a page; a page is the right unit of response | Link home; site nav present via `base.html` | No |
| E2 | Missing file under `/static/` | **Today: blank 404 body. Target: same themed 404** | Consistency — every 404 on the site should look like a 404 | Link home | No |
| E3 | Handler error (`SiteError`) | Full-page themed 500, generic copy, zero internals | Verbose errors are recon data (`src/errors.rs:12-16`); the test at `:185-193` asserts no path, no `.rs`, no inner message reaches the body | Link home; details go to journald via `tracing::error!` | No |
| E4 | Template render fails | Plain-text `404 not found` / `500 internal server error` | Last resort; must not recurse into templating | Reload | No |
| E5 | Rate limit exceeded | `429` plain text + `Retry-After` | Response must stay cheap under flood | Wait and retry | No |
| E6 | Non-GET method on a known route | Framework `405` with `Allow: GET,HEAD`, empty body | **Accepted as-is.** The site is GET-only with no forms; a 405 is reached only by a script. Faking a themed 405 would add per-route fallbacks across 14 routes for no reader | Use GET | No |
| E7 | `/proc/self/status` unreadable | `MEM not available` / `"rss_mib": null` / strip omits MEM | A missing metric is not an error (`src/state.rs:16-17`) | None needed | No |
| E8 | Listener bind fails at startup | **Today: raw Rust panic.** Target: one `tracing::error!` line + `exit(1)` | An operator reading journald should see a sentence, not a backtrace | Free the port / fix `MG_BIND_ADDR` | No |

**Verified E8:**

```
thread 'main' panicked at src/main.rs:58:62:
called `Result::unwrap()` on an `Err` value: Os { code: 98, kind: AddrInUse, ... }
```

**No data-loss risk exists anywhere in this feature.** Nothing here writes to
disk, and the only mutable state is two in-memory counters that are *intended* to
reset on restart — a reset uptime is information, not a loss.

### 3.7 Accessibility

**Auto-fail floors — both satisfied and verified.**

*No-JS floor (rule 3).* Every surface is server-rendered. `curl` — a client with
no JavaScript engine at all — retrieves the complete `/status` readout, the
populated footer strip, both text endpoints, and the 404/500 bodies. Site-wide JS
is 95 lines (`theme-init.js` 15, `main.js` 80), theme selection only. No surface
in this feature reads, writes, or depends on it.

*Accessibility floor (rule 2).* Addressed item by item below, including one
measured contrast failure that this spec does not paper over.

**Semantics and assistive technology (3D).**

- `/status` uses `<dl>`/`<dt>`/`<dd>` — the correct structure for a name/value
  readout. `<h1>Status</h1>` is the only heading; the page outline is one level
  deep and correct. No change needed.
- **Terse labels.** `UP`, `REQ`, `MEM`, `VER`, `BUILT`, `BIND` are announced as
  written. Fixed by the gloss rewrite in §3.3 — each `<dd>` note leads with the
  expanded noun. Deliberately *not* fixed with `<abbr title>` (announcement is
  inconsistent across screen readers and the tooltip is unreachable on touch) or
  with a `.visually-hidden` span (no such utility exists in `style.css` today;
  introducing one to solve a copy problem is the wrong tool).
- **Vitals strip naming (A2 handoff, binding requirement).** The strip is
  `<div class="vitals-strip" aria-label="Server vitals">`
  (`templates/vitals_strip.html:7`). `aria-label` on a `<div>` with no role is
  dropped by most assistive tech, so the strip currently has no accessible name.
  Requirement: the strip must expose an accessible name, via a role-bearing
  element (`role="group"` or a semantic `<p>`/`<ul>`). The `·` separators are
  already correctly `aria-hidden="true"` (`:9`, `:13`, `:17`).
- **404 path echo.** `{{ path }}` is auto-escaped by Askama; the test at
  `src/errors.rs:160-169` asserts `<script>alert(1)</script>` renders inert. Keep
  that test; it is test-encoded policy under criterion 1F.
- **Page titles.** Every page follows `X — machinageist` except the 404, whose
  `title()` returns bare `"404"` (`src/errors.rs:73`). A screen reader announces
  the title first, so a user hears "404" with no context. **Target:**
  `"404 — Page not found — machinageist"`. The 500's `"500"` (`:92`) →
  `"500 — Server error — machinageist"`.

**Contrast and color independence (3B) — one measured failure.**

State is never communicated by hue in this feature: `.status-readout dt` uses
`var(--accent)` *and* `font-weight: 700` (`style.css:870-873`), so the label/value
distinction survives monochrome; the `MEM` unavailable state is the literal words
"not available", not a color.

Contrast is a different story. `style.css:10-13` claims "23 themes, all WCAG-AA
validated." I computed WCAG 2.1 ratios for the two pairings this feature actually
renders, across all 23 theme blocks in `static/css/style.css`:

| Pairing | Where it renders | Themes failing 4.5:1 |
|---|---|---|
| `--text-faint` on `--surface` | `.status-note` inside `.status-readout` (panel bg is `var(--surface)`, `style.css:864`) | **7 of 23** — lunarcore 4.47, cloud 4.28, gameboy 3.43, c64 3.27, nes 4.47, solarized 3.15, blueprint 4.41 |
| `--text-faint` on `--bg` | `.vitals-strip` and `.site-footer` (footer sets no background) | **3 of 23** — gameboy 3.97, c64 3.93, solarized 3.64 |

Lunarcore is the **default** theme, and the `.status-note` text renders at
`0.8rem` — nowhere near the 18.66 px large-text threshold, so 4.5:1 is the
binding floor. The generator validated `--text-faint` against `--bg`, and seven
themes fail once the same token lands on a `--surface` panel. That is precisely
the `generate_themes.py` drift class criterion 5B names.

**Binding requirement:** every text/background pair rendered by `/status`, the
vitals strip, and the error pages must meet **4.5:1 in all 23 themes**.

**Resolution path:**
- *Correct fix (A1, palette concern — legitimately a per-theme edit under 2F):*
  raise `--text-faint` so it clears 4.5:1 against **both** `--bg` and `--surface`
  in all 23 themes.
- *Interim fallback if A1 does not move first:* switch `.status-note` and
  `.vitals-strip` from `--text-faint` to `--text-muted`. Measured: `--text-muted`
  passes on `--bg` in **all 23** themes, and on `--surface` in **22 of 23** (only
  solarized fails, at 4.13). That reduces failures from 7 to 1 and leaves a
  single named palette bug for A1.
- *Drift guard (§5.1 T8):* a contrast test that fails CI, so this cannot regress
  silently and the "all WCAG-AA validated" comment becomes enforceable rather
  than aspirational.

**Keyboard and focus (3C).** Two links, DOM order, no interception, no custom
widget, no roving focus. Global focus visibility is A1's. The one defect — the
404 recovery link focusable while invisible — is fixed in §3.5.

**Text scaling / dynamic type (3F).** Type is set in `rem`, so browser text
scaling applies. `.status-readout`'s `max-content` first column grows with the
text. Verify at 200 % browser zoom and at a 24 px root size: the readout must not
introduce horizontal body scroll at 320 px.

**Motion (3E).** Covered in §3.5. No autoplay, no flashing, no body-content
animation, no polling — the readout is a stamp, so nothing on the page moves on
its own.

---

## 4. Implementation Specification

### 4.1 Architecture placement

| Concern | File | Status |
|---|---|---|
| Route + layer order (single source of truth) | `src/router.rs` | implemented, **order defect F1** |
| Response headers | `src/middleware/security_headers.rs` | implemented, untested |
| Throttling | `src/middleware/rate_limit.rs` | implemented, untested |
| Request/route counting | `src/middleware/vitals.rs` | implemented + tested (`:64-97`) |
| State, snapshot, `/proc` read, bind classification | `src/state.rs` | implemented + tested (`:295-408`) |
| `/status`, `/status.json` handlers | `src/handlers/status.rs` | implemented + tested (`:91-162`) |
| `security.txt`, `robots.txt` | `src/handlers/well_known.rs` | implemented + tested (`:123-159`) |
| Error type, error pages, fallback | `src/errors.rs` | implemented + tested (`:160-193`) |
| Build stamp | `build.rs` | implemented |
| Startup, bind, logging config | `src/main.rs` | implemented, **F7/F8** |
| CI gates | `.github/workflows/ci.yml` | implemented |

**Structural constraint that shapes the whole test plan.** `mg-server` is a
**binary-only crate** — `Cargo.toml` declares no `[lib]`, and `main.rs:16-21`
declares the modules privately. An integration test under `tests/` therefore
*cannot* reach `crate::router` or `crate::state`. This is why every router-level
test today lives in a `#[cfg(test)]` module inside `src/` (`status.rs`,
`errors.rs`, `vitals.rs`), and why `tests/wiki_pages.rs:13-15` duplicates
`WIKI_SLUGS` on purpose. **Every test this spec adds goes in `src/`.** Proposing
`tests/ops_headers.rs` would not compile.

### 4.2 Data model

`Status` is the **privacy allowlist**. Anything added to it ships to the public
internet (`src/handlers/status.rs:9-11`). Fields unchanged; only `bind`'s
*content* is corrected.

```rust
// The complete set of process facts published at /status and /status.json.
// This struct is an allowlist, not a convenience bag: adding a field is a
// publication decision. status_json_key_set_is_exactly_the_allowlist fails
// on any change, forcing the decision to be deliberate.
#[derive(Serialize)]
pub struct Status {
    pub uptime_secs: u64,        // seconds since process start
    pub uptime: String,          // same value formatted dd:hh:mm for display
    pub requests: u64,           // non-static requests that passed the limiter
    pub rss_mib: Option<u64>,    // resident set size; null off Linux or on read failure
    pub version: &'static str,   // CARGO_PKG_VERSION
    pub build: String,           // build stamp, "YYYY-MM-DD HH:MM UTC" or "unknown"
    pub bind: String,            // listener exposure class — never a custom address
}
```

`uptime_secs` / `uptime` are deliberate duplication: one derived from the other
by `format_uptime` (`src/state.rs:275-281`), so machines get arithmetic and humans
get a readout. Single source of truth is preserved because `uptime` is *computed
from* `uptime_secs`, not stored beside it.

**Modified — `BindMode::description` (`src/state.rs:209-220`).** Today every
loopback address reports the literal string `"loopback (127.0.0.1)"`. **Verified
inaccuracy:** an instance bound to `127.0.0.2` reports `"bind":"loopback (127.0.0.1)"`.
Safe, but wrong. Target:

```rust
// Return a useful exposure class without publishing a specific address
fn description(self) -> &'static str {
    match self {
        Self::LoopbackV4 => "loopback (IPv4)",
        Self::LoopbackV6 => "loopback (IPv6)",
        Self::AllIpv4    => "all IPv4 interfaces",
        Self::AllIpv6    => "all IPv6 interfaces",
        Self::Custom     => "custom interface",
    }
}
```

**New — extracted pure parser, for testability.** `rss_mib()` currently reads a
`const` path and parses in one function (`src/state.rs:262-272`), so the parse
logic can only be exercised on a Linux host with a readable `/proc`. Split it:

```rust
// Extract VmRSS in MiB from the text of a /proc/<pid>/status file
fn parse_vm_rss(text: &str) -> Option<u64> { /* existing loop */ }

// Read resident memory in MiB; None off Linux or on any read/parse failure
pub fn rss_mib() -> Option<u64> {
    parse_vm_rss(&std::fs::read_to_string(PROC_STATUS_PATH).ok()?)
}
```

No behavior change; the `None`-on-any-failure contract is preserved exactly.

**No database, no migrations, no persisted schema.** All state is two in-memory
counters plus a cached `u64`, and all of it is meant to reset on restart.

### 4.3 API contracts

#### `GET /status` → `200 text/html; charset=utf-8`
`Cache-Control: no-store`. No auth, no params, no pagination. Subject to the
global limiter. Cannot fail except via E3/E4.

#### `GET /status.json` → `200 application/json`
`Cache-Control: no-store`. **This is a public contract.** Verified live shape:

```json
{"uptime_secs":1361,"uptime":"00:00:22","requests":10,"rss_mib":11,
 "version":"0.1.0","build":"2026-08-07 23:45 UTC","bind":"loopback (127.0.0.1)"}
```

Stability policy: `rss_mib` may be `null` and consumers **must** handle it; other
keys are always present. Fields are added only by an explicit decision that also
updates the allowlist test. No CORS header is sent, so cross-origin JS cannot read
it — intended, and consistent with `connect-src 'self'`.

#### `GET /.well-known/security.txt` and `GET /security.txt` → `200 text/plain; charset=utf-8`
`Cache-Control: public, max-age=3600`. Target body:

```
Contact: mailto:machinageist@proton.me
Expires: 2027-05-16T00:00:00Z
Preferred-Languages: en
Canonical: https://machinageist.dev/.well-known/security.txt
Canonical: https://machinageist.dev/security.txt
```

The second `Canonical` is the fix: RFC 9116 §2.5.2 permits multiple `Canonical`
fields, and both URIs are served (`src/router.rs:54-55`). No `Policy` or
`Encryption` field is added — a `Policy:` URI pointing at a page that does not
exist would be a promise without a page (criterion 6.3), and no PGP key is
published.

#### `GET /robots.txt` → `200 text/plain; charset=utf-8`
Unchanged body. `Allow: /` for `*`; eighteen named AI agents get
`Disallow: /static/`. **The policy is assets-only by design** — search indexing of
prose is wanted; bulk asset scraping is not.

#### `429 Too Many Requests` — the contract that changes most
**Current, verified:**
```console
HTTP/1.1 429 Too Many Requests
content-length: 17
date: Sat, 08 Aug 2026 00:14:02 GMT

too many requests
```
No `Content-Type`. No `Retry-After`. No `Cache-Control`. **No security headers at
all.**

**Target:**
```
HTTP/1.1 429 Too Many Requests
content-type: text/plain; charset=utf-8
retry-after: <seconds, from governor NotUntil::wait_time_from>
cache-control: no-store
content-security-policy: <full policy>
strict-transport-security: max-age=63072000; includeSubDomains; preload
x-content-type-options: nosniff
x-frame-options: DENY
referrer-policy: strict-origin-when-cross-origin
permissions-policy: camera=(), microphone=(), geolocation=(), payment=()
```

#### Rate limiting — the actual algorithm, stated precisely

| Property | Value | Evidence |
|---|---|---|
| Library | `governor` 0.6 | `Cargo.toml` |
| Keying | **`NotKeyed` — one global bucket for the whole process** | `src/middleware/rate_limit.rs:38-39` |
| **Not** per-IP | The source says so outright: "This implementation is per-server-instance, not per-IP" | `src/middleware/rate_limit.rs:12` |
| Algorithm | GCRA token bucket | `RateLimiter::direct(quota)`, `:49` |
| Quota | `Quota::per_minute(60)` → burst 60, refill 1 token/s | `:48` |
| Storage | `InMemoryState`, per-process, lost on restart | `:38-39` |
| Rejected request | `429`, handler never runs, `WARN` logged | `:66-72` |
| Scope | **Every** request including `/static/*` | `src/router.rs:72-75` wraps the whole router |

**Observed:** 62 consecutive `GET /about` succeeded against a full bucket, then
`429` for the remainder of a 70-request flood — 60 burst plus ~2 refilled during
the ~1.4 s window. Matches the documented quota.

**Verified: throttled requests never reach the counters.** After the flood,
`/status.json` reported `requests: 65` = 2 setup requests + 62 successes + the
`status.json` read itself. The 9 rejected requests contributed **zero**. The claim
at `src/router.rs:67-68` and `src/middleware/vitals.rs:10-12` is accurate, and
§4.4 keeps it accurate after the reorder.

**Per-IP is `gated`, not `planned`, and the gate is recorded here.** The app binds
loopback (`src/main.rs:41-45`), so its socket peer is always `127.0.0.1` — Caddy.
A real client IP is only obtainable from a forwarded header
(`CF-Connecting-IP` / `X-Forwarded-For`), and trusting a client-settable header
is a spoofing vector unless Caddy is configured to *overwrite* rather than append
it. That configuration lives outside this repo. Until it is done and verified,
per-IP limiting would be **worse than none** — it would key a bucket on an
attacker-chosen string.

Consequence: `axum-client-ip = "0.5"` is declared in `Cargo.toml` but **has zero
references in `src/`** (verified by grep). An unused dependency is supply-chain
surface plus a false signal that per-IP limiting exists. **Target: remove it**,
and re-add it in the same change that lands the Caddy trusted-hop config.

#### Method handling
`GET`/`HEAD` only. Verified: `POST /` → `405`, `allow: GET,HEAD`, empty body,
security headers present. Accepted as-is (§3.6 E6).

### 4.4 State management

**Owner:** `AppState` (`src/state.rs:48-55`) — `Instant` start time,
`Arc<AtomicU64>` request total, `Arc<Mutex<HashMap>>` per-route hits,
`Arc<Mutex<RssCache>>`, and a `Copy` `BindMode`. Cheap to clone; every clone
shares the same `Arc`s (proven at `src/state.rs:295-306`).

**Injection points.** Two, deliberately different:
- *Write path:* `mw::from_fn_with_state(state.clone(), vitals::count)`
  (`src/router.rs:69`) bakes state into the layer, keeping the surrounding
  `Router` state-free.
- *Read path:* a process-global `OnceLock` (`src/state.rs:137`), published once by
  `main.rs:50` before the router is built. `Status::current()` reads it.

**Why the global exists, and what it costs.** `src/state.rs:8-12` gives the
reason: the footer strip renders on *every* page, and the alternative is threading
a `State` extractor through every handler and adding a `vitals` field to all
twelve templates. The global is the smaller change.

The cost is real and must be named (criterion 5C, no hidden coupling):

1. `templates/vitals_strip.html:6` calls `crate::state::Status::current()` **from
   inside a template**. A template reaching into a process global is invisible
   coupling — you cannot render the strip with a synthetic `Status`, so its
   `rss_mib == None` branch is not unit-testable.
2. `init_global` is effectively one-shot: it succeeds only for clones of the
   already-published runtime and otherwise returns `Err`
   (`src/state.rs:140-157`). Because `cargo test` runs all tests of a target in
   **one process**, exactly one test in the entire crate may publish. The comment
   at `src/handlers/status.rs:137-139` says so: *"The one test allowed to publish
   to the process-global OnceLock."* A future test that calls `init_global` with
   its own state fails with a confusing message about "a different router
   runtime."
3. On `/status`, the `<dl>` values come from the handler's snapshot while the
   footer strip takes a **second, independent** snapshot during template render.
   Under concurrent traffic the same HTML document can print two different request
   counts. (Measured under no load: both showed `REQ 7` in a single response.)

**Decision: keep the global; do not thread state through twelve templates.** The
threading cost is real and the drift risk is low. But the coupling must be
*documented at the point of use* rather than discovered:
- Add a doc comment on `init_global` stating the one-publisher-per-process rule
  and naming `status_json_reflects_requests_counted_through_the_router` as the
  designated publisher.
- Add a comment in `vitals_strip.html` recording that the strip's snapshot is
  independent of the page handler's, and that on `/status` the two may differ
  under load.
- Cover the `None` branch through `StatusTemplate` with a hand-built `Status`
  (which *is* possible — only the strip is unreachable), and cover the parse and
  cache logic as pure units.

**Layer ordering — the core correctness question.**

`src/router.rs:63` documents "applied bottom-up on request, top-down on response."
Checked against axum 0.7's documented semantics (successive `.layer()` calls nest
outward; the last-applied layer is outermost) and against observed behavior:
**the documentation is accurate.** Source-listing order top→bottom is
`security_headers`, `vitals`, `rate_limit`, `TraceLayer`; a request enters at the
bottom and travels up.

Current effective order:

```
request  →  TraceLayer  →  rate_limit  →  vitals::count  →  security_headers  →  route
response ←  TraceLayer  ←  rate_limit  ←  vitals::count  ←  security_headers  ←  route
```

Two of the three rationales in the comments hold; one does not.

- ✅ "TraceLayer sees every request first and last" (`:7`) — it is outermost.
- ✅ "vitals inside the limiter so throttled floods never reach the counters"
  (`:67-68`) — confirmed by the flood measurement in §4.3.
- ❌ "security_headers stamps every outgoing response" (`:8`, `:65`) — **false for
  the throttled path.** `security_headers` is the *innermost* layer, and
  `rate_limit` short-circuits without calling `next.run()`
  (`src/middleware/rate_limit.rs:66-72`), so the `429` is constructed *outside*
  the header layer and never passes through it. Verified: the `429` capture in
  §4.3 has no CSP, no HSTS, no `nosniff`, and no `Content-Type` — which means a
  browser is free to MIME-sniff a body that arrived with no declared type.

**Target order** — move `security_headers` from innermost to just inside
`TraceLayer`:

```rust
// Middleware layers — applied bottom-up on request, top-down on response
// Count requests and per-route hits — innermost, so only requests that
// survived the limiter are ever counted
.layer(mw::from_fn_with_state(state.clone(), vitals::count))
// Check rate limit before the request reaches any handler
.layer(mw::from_fn(move |req, next| { /* rate_limit */ }))
// Stamp security headers onto every outgoing response, including the 429
// the limiter produces below — this layer must sit OUTSIDE the limiter
.layer(mw::from_fn(add_security_headers))
// Log every request: method, path, status code, response time
.layer(TraceLayer::new_for_http() /* ...leveled, see below */)
```

Resulting order:

```
request  →  TraceLayer  →  security_headers  →  rate_limit  →  vitals  →  route
response ←  TraceLayer  ←  security_headers  ←  rate_limit  ←  vitals  ←  route
```

Both preserved properties, now provable: the `429` passes back through
`security_headers`; `vitals` is still inside `rate_limit`, so rejected floods
still never touch the counters. Two lines move. The comments move with them —
a comment that survives a reorder while its claim does not is exactly the drift
criterion 5E targets.

### 4.5 Dependencies

**Added crates:** none.

**Added feature on an existing crate: `tower-http/set-header` — required, and the
manifest edit is stated here so it is not discovered at compile time.** §4.7
Phase 1 uses `SetResponseHeaderLayer`, which lives in `tower_http::set_header`
behind `#[cfg(feature = "set-header")]` (tower-http 0.5.2 `src/lib.rs:232`).
`Cargo.toml:14` enables only `["fs", "trace"]`, so the type does not exist in
this build today. Exact edit:

```toml
# Cargo.toml:13-14 — before
# "fs" = ServeDir static file serving  "trace" = TraceLayer request logging
tower-http = { version = "0.5", features = ["fs", "trace"] }

# after
# "fs" = ServeDir static file serving  "trace" = TraceLayer request logging
# "set-header" = SetResponseHeaderLayer, the /static Cache-Control layer (§4.7)
tower-http = { version = "0.5", features = ["fs", "trace", "set-header"] }
```

**Why enable the feature rather than hand-roll the header.** `set-header = []` in
tower-http 0.5.2's manifest — it is a pure code-gating feature that pulls in **no
transitive crates**, so the supply-chain surface added is zero and `Cargo.lock`
does not change. `cargo tree -e features -i tower-http` confirms `set-header` is
not currently enabled (`fs` transitively enables `set-status`, which is a
different feature). The considered alternative was a `mw::from_fn` layer on the
nested `/static` service, matching the shape of `add_security_headers` — that
would need no manifest edit, but it means writing and testing bespoke middleware
to set one constant header that the dependency already ships, which §2E-style
restraint argues against. **Decision: enable the feature.** If a future reviewer
prefers zero manifest churn, the `mw::from_fn` variant is a drop-in substitute
and nothing else in this spec depends on which one is chosen.

**Removed:** `axum-client-ip = "0.5"` — declared in `Cargo.toml`, zero references
in `src/` (verified by grep). See §4.3 for the gate and the re-add condition.

**Already present and sufficient:**
- `governor` 0.6 — `NotUntil::wait_time_from` supplies `Retry-After`; no new dep.
- `chrono` — parses `SECURITY_TXT_EXPIRES` for the expiry drift guard.
- `tokio` with `features = ["full"]` — includes `signal`, so graceful shutdown
  needs no manifest change.
- `tower-http` with `["fs", "trace"]` — `ServeDir::not_found_service` is in `fs`;
  `TraceLayer` level configuration (`DefaultMakeSpan`, `DefaultOnRequest`,
  `DefaultOnResponse`) is in `trace`. **`SetResponseHeaderLayer` is not** — see
  the feature note above.

**Assets:** none. **Infrastructure:** none inside this repo. Cloudflare, Caddy,
the Tunnel, and the systemd unit are referenced but not modified — the feature
tree lists the deploy pipeline as out of scope.

### 4.6 Platform-specific considerations

**Linux vs. macOS is the live compatibility axis.** Production is a Debian VM;
`src/state.rs:16-17` records that dev has been macOS. `/proc/self/status` does not
exist on macOS, so `rss_mib()` returns `None` and every surface degrades as
described in §3.2 branch A1. **This must never become a panic or a hard
dependency.** CI runs `ubuntu-latest`, so the `Some` branch is what CI exercises
and the `None` branch is covered by unit tests of the extracted parser.

**No feature flags, no gradual rollout.** One binary, one process, one host;
deployment is a rebuild and a `systemctl restart`. There is no canary and no
rollback automation — the hosting post says so at
`content/posts/hosting-machinageist-dev.md:106-108`, and this spec does not
pretend otherwise.

**Browser support:** every response in this feature is HTML or plain text with no
JavaScript. `Permissions-Policy` and `Referrer-Policy` are ignored by browsers
that do not implement them, which is the correct failure mode. HSTS `preload`
is belt-and-suspenders on a `.dev` TLD (already preload-only) and the source
explains why the directive is kept anyway (`security_headers.rs:57-60`).

**Graceful shutdown (`planned`).** `axum::serve(listener, app).await.unwrap()`
(`src/main.rs:61`) has no `with_graceful_shutdown`, so `systemctl restart` sends
SIGTERM and in-flight requests die mid-response. Target: `with_graceful_shutdown`
on SIGTERM/SIGINT with a bounded drain. This is the smallest change that makes
the systemd runbook artifact honest — "restarting drops in-flight requests" is a
sentence a reviewer will ask about.

### 4.7 Performance budget

All figures measured on the running debug binary.

| Dimension | Budget | Measured / basis |
|---|---|---|
| Process RSS | ≤ 30 MiB steady state | **10–11 MiB** observed via `/status.json` |
| `/status.json` payload | ≤ 512 B | **146 B** |
| `/status` payload | ≤ 15 KB | ~12 KB (home page 12 551 B, 404 page 10 173 B) |
| `429` payload | ≤ 256 B | **17 B** body today; target ~17 B + a few headers |
| Per-request CPU (counting) | one relaxed atomic add + one uncontended `Mutex` on a small `HashMap` | `src/state.rs:75-83`; source notes dashmap is unjustified at this traffic level (`:14-16`) |
| `/proc` reads | ≤ 1 per 5 s regardless of render rate | `RSS_CACHE_TTL` (`src/state.rs:42`), proven at `src/state.rs:373-390` |
| Startup delta | 0 | Limiter built once (`src/router.rs:31`); build stamp is a compile-time `env!` |
| Network, steady state | unchanged | No polling, no websocket, no JSON fetch — the strip is server-rendered |
| Client storage | 0 bytes | No cookie, no `localStorage`, no client state in this feature |
| Server storage | 0 bytes | Nothing in this feature writes to disk |

**One budget risk worth naming.** The limiter's 60-token bucket covers **all**
requests including `/static/*`. `base.html` pulls four subresources
(`favicon.svg`, `theme-init.js`, `style.css`, `main.js`), so a cold-cache page
view costs ~5 tokens. `ServeDir` sets `Last-Modified` but **no `Cache-Control`**,
so even warm revalidations spend tokens. A single genuine reader browsing quickly
can approach the bucket — a self-DoS, not an attack.

**Target (phased, because the second phase has a footgun):**
- *Phase 1 (safe now, but not free):* add `Cache-Control: public, max-age=3600`
  to `/static` responses via `SetResponseHeaderLayer` on the nested service.
  **This requires enabling tower-http's `set-header` feature** — the type is
  `#[cfg(feature = "set-header")]`-gated and `Cargo.toml:14` does not enable it,
  so this phase does not compile without the one-word manifest edit spelled out
  in §4.5. It is the only manifest change in this spec besides the
  `axum-client-ip` removal.
- *Phase 2 (requires the version-token fix):* raise to
  `max-age=31536000, immutable`. This is only safe once the `?v=` cache-buster is
  derived from the build stamp instead of hand-typed — `templates/base.html`
  currently hard-codes the literal `v=20260719-spectrum` in **four places**
  (verified by count), so editing `style.css` without editing four strings would
  strand clients on a stale file for a year. Server side, this spec provides an
  `asset_version()` value from `BUILD_TS`; the template edit is an **A2 handoff**
  (§7.4), and Phase 2 must not land before it.

---

## 5. Test Specification

All tests live in `#[cfg(test)]` modules under `src/` — see the binary-crate
constraint in §4.1. All run under `cargo test --all-targets` in CI.

### 5.1 Unit tests

**New module `src/middleware/security_headers.rs::tests`** (the file has **no
test module at all** today):

- **T1 `every_response_carries_the_defensive_header_set`**
  *Setup:* `router::build(AppState::new())`, driven by `oneshot` over
  `["/", "/about", "/blog", "/status", "/status.json", "/robots.txt",
  "/.well-known/security.txt", "/no-such-page", "/static/css/style.css",
  "/static/nope.css"]`.
  *Assert:* all six headers present with exact values; `server` absent.
  *Edge covered:* `404`, static hit, static miss, JSON, and plain text all take
  the same path as `200 text/html`.

- **T2 `throttled_responses_carry_headers_and_a_retry_after`** — the regression
  test for finding F1.
  *Setup:* one router; issue 61+ requests until a `429` appears.
  *Assert:* status is `429`; `content-security-policy` and `x-content-type-options`
  present; `content-type` starts with `text/plain`; `retry-after` parses as a
  positive integer; `cache-control` is `no-store`.
  *Edge covered:* the short-circuit path that bypasses the header layer today.

**New module `src/middleware/rate_limit.rs::tests`** (also has **no test module**):

- **T3 `burst_is_allowed_then_requests_are_throttled`**
  *Setup:* minimal router wrapping only `rate_limit` with a fresh limiter.
  *Assert:* the first 60 requests are not `429`; a later request in the same
  burst is `429`. Asserted as "at least 60 succeed, then a 429 occurs" rather
  than an exact count — refill during the test window makes an exact number
  flaky, and a flaky test is worse than a looser one.
  *Edge covered:* quota arithmetic.

- **T4 `throttled_requests_are_not_counted`**
  *Setup:* full `router::build` with a known `AppState`; flood past the bucket;
  count `2xx` responses.
  *Assert:* `state.requests_total()` equals the number of non-`429` responses.
  *Edge covered:* the layer-order invariant that keeps counters honest — this test
  fails if someone later reorders `vitals` outside `rate_limit`.

**`src/state.rs::tests`** (extend the existing module):

- **T5 `parse_vm_rss_reads_kib_and_converts_to_mib`** — fixture with
  `VmRSS:\t   12345 kB` → `Some(12)`.
- **T6 `parse_vm_rss_returns_none_when_the_field_is_missing_or_malformed`** —
  fixture without `VmRSS`, and one with `VmRSS: not-a-number`, both → `None`.
  *Edge covered:* the `None` degradation path, on any OS, in CI. This is the
  compensating control for the fact that CI runs on Linux and never naturally
  exercises the unavailable branch.
- **T7 `bind_description_never_contains_a_specific_address`** — over all five
  `BindMode` variants plus `127.0.0.2`, `192.0.2.10`, `::1`, `0.0.0.0`: assert no
  output contains a `.`-separated quad other than the documented interface words,
  and that `127.0.0.2` no longer reports `127.0.0.1`.

**`src/handlers/status.rs::tests`** (extend; keep all existing tests — they are
test-encoded policy under 1F):

- **T8 `status_json_key_set_is_exactly_the_allowlist`**
  *Setup:* serialize `Status::current()`; collect top-level keys into a sorted
  `Vec`.
  *Assert:* equals the literal
  `["bind","build","requests","rss_mib","uptime","uptime_secs","version"]`.
  *Edge covered:* **the drift guard for the privacy allowlist.** Adding *any*
  field to `Status` fails this test, forcing the author to make a publication
  decision instead of leaking a field by reflex. This is the mechanism that turns
  the comment at `src/handlers/status.rs:9-11` into policy.

- **T9 `status_surfaces_leak_no_paths_hostnames_or_addresses`**
  *Setup:* fetch both `/status` and `/status.json` through the real router.
  *Assert:* neither body contains `/home/`, `/Users/`, `/proc/`, `0.0.0.0`,
  `::`, the output of `hostname`, or any `192.168.` / `10.` / `172.16.`–`172.31.`
  octet sequence.
  *Edge covered:* strengthens the existing pair of assertions
  (`src/handlers/status.rs:98-100`, `src/state.rs:337`), which only check
  `0.0.0.0`, `/Users/`, and `/home/` on one surface. **Note the deliberate
  tension:** `templates/status.html:26` displays the literal string
  `/proc/self/status` as a gloss. The test must assert on the *values*, not the
  page's explanatory prose — so this test runs against `/status.json` for the
  path patterns and against `/status` for address patterns, and the gloss is
  reworded (§3.3) to `read from /proc/self/status` so the intent is unambiguous
  to a future reader of the test.

- **T10 `status_template_renders_the_unavailable_memory_branch`**
  *Setup:* construct `StatusTemplate { status: Status { rss_mib: None, .. } }`
  directly and `render()`.
  *Assert:* body contains `not available` and does not contain `MiB`.
  *Edge covered:* the template's `{% when None %}` arm, which no router-driven
  test can reach on Linux.
  *Known gap, stated rather than hidden:* the **footer strip's** `None` arm
  remains untested because `vitals_strip.html:6` calls the process global
  directly (§4.4). Covered by inspection, not by CI.

**`src/handlers/well_known.rs::tests`** (extend):

- **T11 `security_txt_expires_is_at_least_thirty_days_out`**
  *Setup:* parse `SECURITY_TXT_EXPIRES` with `chrono`; compare to `Utc::now()`.
  *Assert:* more than 30 days remain.
  *Edge covered:* **the yearly-renewal drift guard.** `src/handlers/well_known.rs:7-8`
  says "Renew it by updating `SECURITY_TXT_EXPIRES` at least once a year" — a
  comment, which is exactly the kind of instruction that silently rots. This turns
  it into a red CI run 30 days before scanners start rejecting the file.

- **T12 `security_txt_declares_every_uri_it_is_served_from`**
  *Assert:* the body contains a `Canonical:` line for both
  `https://machinageist.dev/.well-known/security.txt` and
  `https://machinageist.dev/security.txt`.
  *Edge covered:* the RFC 9116 §2.5.2 mismatch, and the coupling to the two
  routes at `src/router.rs:54-55`.

- **T13 `every_named_robots_group_disallows_static`**
  *Setup:* parse `robots.txt` into `(User-agent, directives)` groups.
  *Assert:* the `*` group has `Allow: /`; every other group has
  `Disallow: /static/`; group count ≥ 18.
  *Edge covered:* a hand-edited list losing a `Disallow` line.
  **Stated limit:** no test can know about a crawler that does not exist yet.
  The source gets a dated `// AI agent list last reviewed: YYYY-MM-DD` comment;
  currency is a human review task, not a testable property. Saying so is the
  "what is still unknown" field of the evidence standard.

**`src/errors.rs::tests`** (extend; keep both existing tests):

- **T14 `missing_static_asset_renders_the_themed_404`** — `/static/nope.css` →
  `404` whose body contains `SECTOR NOT FOUND`.
- **T15 `internal_error_response_carries_security_headers`** — build a test
  router with one always-failing handler plus the real `add_security_headers`
  layer; assert the `500` carries CSP and `nosniff`. The existing
  `internal_error_page_leaks_nothing` calls `into_response()` directly and
  therefore proves nothing about the middleware stack.
- **T16 `error_page_titles_follow_the_site_convention`** — `title()` for 404 and
  500 both end in `— machinageist`.

**New — the contrast drift guard (criterion 5B, and the fix for §3.7's measured
failure):**

- **T17 `theme_tokens_meet_wcag_aa_on_every_surface_this_feature_renders`**
  *Setup:* parse `static/css/style.css` for every `:root[data-theme=...]` block;
  extract `--bg`, `--surface`, `--text`, `--text-muted`, `--text-faint`.
  *Assert:* for all 23 themes, `--text-faint` and `--text-muted` each meet 4.5:1
  against **both** `--bg` and `--surface`.
  *Placement:* this is a whole-site palette guard, so it belongs to **A1** if A1
  specifies one; A3 asserts it because A3's surfaces are where the failure
  renders. **This test fails today** — 7 themes for `faint`-on-`surface`, 3 for
  `faint`-on-`bg`, 1 for `muted`-on-`surface` (solarized). It must be introduced
  *with* the palette fix, in the same commit, or CI goes red on an unrelated
  change. Coordinate with A1 before landing.

### 5.2 Integration tests

`cargo test --all-targets` already drives the real router end to end via
`tower::ServiceExt::oneshot` (`src/handlers/status.rs:69-89`), which is the
integration layer available to a binary-only crate. T1, T2, T4, T9, T14 all run
through `router::build`, so they cover routing, layer order, handler, template
render, and serialization in one pass.

The existing round-trip test
`status_json_reflects_requests_counted_through_the_router`
(`src/handlers/status.rs:141-162`) is the strongest test in the repo — it proves
the middleware's writes and the global snapshot's reads share the same `Arc`s.
**Keep it, and keep its comment**, which records that it is the crate's single
authorized `init_global` caller.

**Out-of-process verification** (not automated; run before a deploy is trusted):

```sh
# Start a second instance beside a running one — the port is fixed at 3000,
# but any loopback address works
MG_BIND_ADDR=127.0.0.2 RUST_LOG=mg_server=info,tower_http=info cargo run

curl -sSI http://127.0.0.2:3000/                                  # six headers, no `server`
curl -sSI http://127.0.0.2:3000/no-such-page                      # 404 + six headers
curl -sS -o /dev/null -w '%{http_code} %{size_download}\n' \
     http://127.0.0.2:3000/static/nope.css                        # expect 404 + non-zero body
for i in $(seq 1 70); do
  curl -s -o /dev/null -w '%{http_code} ' http://127.0.0.2:3000/about
done; echo
curl -sSI http://127.0.0.2:3000/about                             # 429 + headers + retry-after
curl -sS http://127.0.0.2:3000/status.json                        # allowlist fields only
curl -sS http://127.0.0.2:3000/.well-known/security.txt           # two Canonical lines
curl -sS -X POST -o /dev/null -w '%{http_code}\n' http://127.0.0.2:3000/   # 405
```

**Live-site verification** (the same command the security-headers post publishes,
which is the point — the artifact is reproducible by the reader):

```sh
curl -sSI https://machinageist.dev |
  grep -iE 'content-security|strict-transport|x-frame|x-content|referrer|permissions|^server'
curl -sS https://machinageist.dev/status.json | jq .build   # must match the deploy
```

### 5.3 UI / E2E tests

**N/A — no browser automation harness exists in this repo, and this spec does not
introduce one.** There is no Playwright/Cypress/WebDriver setup, no `package.json`,
and CI is four `cargo` commands (`.github/workflows/ci.yml:26-36`). Adding a
browser runner to test six static text surfaces would be a large, permanently
maintained dependency for near-zero marginal coverage over T1–T16, which already
assert on the exact bytes returned.

The behaviors a browser runner would normally catch are covered otherwise:
navigation and links by T1's status-code table; error recovery by T14; rendering
by the Askama compile-time template check (a broken template is a **compile
error**, caught by `cargo build --release`); and no-JS behavior by the fact that
`curl` — a JS-free client — is the test client throughout.

### 5.4 Visual / manual verification

| Configuration | What to check | Why |
|---|---|---|
| All 23 themes on `/status` | `.status-note` legible against the `--surface` panel; `<dt>` labels distinguishable in grayscale | The seven measured 4.5:1 failures in §3.7 — check lunarcore (default), cloud, gameboy, c64, nes, solarized, blueprint, which are all seven of them |
| All 23 themes, footer strip | Strip text legible against page `--bg` | gameboy, c64, solarized fail today |
| Light and dark extremes | `paper` / `teletext` vs `lunarcore` / `matrix` | Widest luminance span in the roster |
| 200 % browser zoom, 24 px root | `.status-readout` grid does not force horizontal body scroll | `max-content` first column |
| 320 px viewport | Vitals strip wraps rather than overflowing | `flex-wrap: wrap` |
| MEM present vs absent | `MEM 10 MiB` vs `MEM not available`; strip omits MEM **and its separator** | The `None` branch is the designed empty state |
| Fresh restart vs long uptime | `UP 00:00:00` vs `UP 04:11:07`; `REQ` climbs on refresh | The falsifiability that makes the page worth having |
| `prefers-reduced-motion: reduce` on `/404` | All five boot lines visible immediately, no fade | `style.css:1279` gates on `no-preference` |
| Tab immediately on `/404` with motion enabled | Recovery link visible on focus, not invisible for 1.8 s | The §3.5 fix |
| Screen reader (Orca/VoiceOver) on `/status` | Each `<dt>`/`<dd>` pair announces a full noun; strip has an accessible name | The §3.7 gloss rewrite and the A2 strip-role handoff |
| `/status` with JS disabled | Full readout renders; strip populated | The no-JS floor |

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification

- [x] **Handles sensitive data — describe protection measures**

Not user data — **operator infrastructure data**, which is the sensitive class
here. Protections, all verifiable:

1. **The `Status` struct is a hard allowlist.** Only seven fields serialize
   (`src/state.rs:226-235`); the intent is recorded at
   `src/handlers/status.rs:9-11`; T8 makes it fail loudly on change.
2. **The bind address is classified, never published.** `BindMode`
   (`src/state.rs:188-220`) maps any address to one of five fixed strings; a
   custom address becomes `"custom interface"`. Verified: an instance on
   `127.0.0.2` publishes no trace of `.2`.
3. **No hostname, no filesystem path, no client IP** appears in any response
   body. Enforced by T9.
4. **Errors log internally, disclose nothing externally.** `src/errors.rs:104-126`
   logs the full error and renders a generic page; proven at `:185-193` (no
   `secret-internal-detail`, no `src/`, no `.rs`).
5. **The `Server` header is removed** (`security_headers.rs:101`) so passive
   fingerprinting yields nothing.
6. **No per-visitor data is stored.** The counters are process-wide totals; no
   cookie, no session, no log file written by this application.

**Access logging is where this classification gets sharp.** `/status`'s outro
currently reads: *"No per-visitor data is retained to produce this page — the
counters are process-wide totals, not a record of who visited"*
(`templates/status.html:44-46`). That is true today. Turning on access logging
(§7.2 T5) would not make it false — the counters still are not a visitor record —
but it would make it **incomplete**, because journald would then hold a per-request
line. Copy that is technically true and materially incomplete is exactly what
criterion 1D punishes.

**Requirement, binding on T5:** the access log records **method, path, status,
latency** and nothing else. Explicitly **not** the query string (`DefaultMakeSpan`
records the full `uri`, which would put a future `/search?q=` term in the system
journal), **not** the User-Agent, **not** any client IP. And the `/status` copy is
updated in the same change to say what is kept, including the part Jeff does not
control:

> No per-visitor data is retained to produce this page — the counters are
> process-wide totals, not a record of who visited. The server does keep an
> access log in the system journal: method, path, status, and latency. No IP
> address, no query string, no user agent. Cloudflare, which sits in front of
> this server, keeps its own edge logs on its own terms.

That last sentence is the difference between an honest ops writeup and a
comfortable one.

### 6.2 Asset provenance

- [x] **No third-party assets**

This feature ships no images, fonts, models, or datasets. Fonts are the system
monospace stack (`--font-body: ui-monospace, SFMono-Regular, Menlo, Consolas,
monospace`), so nothing is downloaded — consistent with `font-src 'self'` and
with the "no CDNs" claim in the security-headers post.

Third-party **code** is Cargo crates already vetted in `Cargo.toml`. This spec
adds none and **removes one** (`axum-client-ip`, unused). The eighteen crawler
user-agent strings in `robots.txt` are public identifiers, not licensed assets.

### 6.3 Language / claims audit

- [ ] Makes claims not supported by evidence — **No.** Every claim in §7.1 cites a
      file and line range I read, and the runtime claims cite a capture I took.
- [ ] Promises capabilities not yet built — **No.** Everything proposed is marked
      `planned`; everything existing is marked `implemented` or `prototyped`; per-IP
      limiting is marked `gated` with the gate written down.
- [ ] Uses language restricted by domain regulations — **No.**

**Words this spec does not use about the system, and why:**
*production-grade*, *enterprise*, *SRE*, *DevOps*, *high availability*,
*zero-trust*, *complete observability*, *monitoring stack*, *secured the
application*, *hardened infrastructure*, *SLA*, *uptime guarantee*. Each appears
on the forbidden list at `IMPROVEMENT_PLAN.md:56-66` and `:409-417`, and criterion
1E forbids the posture directly. This is one Rust process on a mini-PC in a house,
behind a tunnel, with no alerting and no tested restore. The spec says that
repeatedly and on purpose.

**No security-engineer identity is claimed.** The defensible framing, which the
two published posts already use, is: *reviewed and documented the HTTP security
headers for an owned web service, with reproducible `curl` evidence*
(`content/posts/security-headers-on-machinageist-dev.md:92-94`). This spec does
not upgrade it.

**Copy audit of every user-visible string this feature owns:**

| String | Location | Verdict |
|---|---|---|
| "rendered by the same Rust process that serves the rest of the site" | `status.html:8-9` | ✅ True — `status::page` is a handler in the same binary |
| "a single Axum binary on a homelab box, reached through a Cloudflare Tunnel and a Caddy reverse proxy" | `status.html:9-11` | ✅ True and appropriately modest — "homelab box", not "infrastructure" |
| "read from the live process at the moment you requested this page" | `status.html:11-12` | ✅ True — `Status::current()` at request time |
| "There is no dashboard service behind them and no JavaScript in front of them; refresh and they move." | `status.html:12-13` | ✅ True, and verified by `curl`. This is the single best sentence on the page: it is an invitation to falsify |
| "No per-visitor data is retained…" | `status.html:44-46` | ⚠️ True today; **becomes incomplete** the moment access logging lands. Rewrite specified in §6.1 |
| "resolved listener exposure; custom addresses stay private" | `status.html:39` | ✅ True — `BindMode` enforces it |
| "RSS readout requires Linux" | `status.html:29` | ⚠️ **Slightly wrong.** `None` also results from a read or parse failure *on* Linux. Target: "memory readout requires Linux" plus, in the JSON contract docs, the full `None` conditions. A small accuracy fix, and precisely the kind criterion 1B is about |
| "Machine-readable version at /status.json" | `status.html:43` | ✅ True |
| `Contact: mailto:machinageist@proton.me` | `well_known.rs:23` | ✅ Real, monitored inbox |
| "too many requests" | `rate_limit.rs:70` | ✅ Accurate |
| "registers dumped to the operator log — nothing useful to see here" | `error_500.html` | ✅ True — `tracing::error!` fires at `errors.rs:114` |
| `// consumed by the /stats page in Phase 2` | `src/state.rs:91` | ❌ **Promises a page that is gated.** `docs/plans/deferred-dashboard-notes.md` blocks any such route until Jeff answers its six questions. A code comment is not user-visible, but under 1B it is still a planned thing written as a scheduled thing. Target: `// no non-test consumer today; a stats surface is gated — see docs/plans/deferred-dashboard-notes.md` |

**Nothing in this feature touches certification claims**, so criterion 1D's live
spine (RHCSA → CCNA → Security+, re-locked 2026-08-02) is not restated on any A3
surface. **But this spec must flag a stale document it depends on:**
`IMPROVEMENT_PLAN.md:15` describes a "four-CompTIA-cert spine (Network+ →
Security+ → Linux+ → Server+, targeted January 2027)", and `:41-47` maps the ops
artifacts onto it. That spine is superseded. The ops *artifacts* remain correct
and worth building; their cert mapping is stale. Flagged as an §8 open question —
this spec does not silently propagate it.

### 6.4 Regulatory alignment

No statutory regime applies (no PII, no payments, no health data, no minors, no
EU cookie consent — there are no cookies). "Regulatory" here means the project's
own binding standards.

| Criterion | How this spec addresses it |
|---|---|
| **1A Evidence standard** | §7.5 maps every proposed public artifact to all thirteen fields of `docs/public-portfolio-structure.md`'s evidence standard, and refuses to publish the ones that cannot fill them |
| **1B State honesty** | Vocabulary table in §0; every capability labeled; per-IP limiting labeled `gated` with the reason; the `/stats` comment flagged |
| **1C GeistScope gate** | N/A — no GeistScope surface is touched. `/status` shows process facts only, no tool claims |
| **1D Copy currency** | §6.3 audits all eleven user-visible strings; two need edits; the stale cert spine in `IMPROVEMENT_PLAN.md` is flagged rather than reused |
| **1E Role posture** | §6.3 forbidden-word list; the framing is owned-scope self-hosting, and §1.2 explicitly calls the threat model "background internet noise" rather than a targeted adversary |
| **1F Test-encoded policy** | Every existing anti-leak test is kept and **strengthened** (T8, T9, T15); none is weakened. One is *updated* — `bind_description_comes_from_the_resolved_listener_address` (`src/state.rs:348-370`) asserts the literals `BindMode::description` currently returns, so §4.2's rewrite necessarily changes them. That edit is recorded in §7.2 and strictly *reduces* disclosure (`"loopback (IPv4)"` names less than `"loopback (127.0.0.1)"`), so it tightens the anti-leak boundary rather than relaxing it |
| **2E Restraint** | No dashboard, no gauges, no sparklines, no badge; the a11y fix is a copy edit rather than new markup; `429` stays plain text |
| **2F Theme integrity** | This feature adds no per-theme rule. §3.7 identifies a *palette* defect, which 2F explicitly allows to be a per-theme concern, and hands it to A1 with measured numbers |
| **3A No-JS floor** | Verified with `curl` on every surface |
| **3B/3C/3D/3E/3F** | §3.7, with one measured failure surfaced rather than glossed |
| **4B Evidence over enthusiasm** | §7.5 — the gap analysis ships *as* an artifact; failures are the content |
| **5A/5B/5C/5D/5E** | §7.2 T8/T11/T17 (guards), §4.4 (coupling named), §5.2 (commands), §7.6 (doc updates) |

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today

**Implemented and working:**

| Capability | Evidence |
|---|---|
| Six security headers + `Server` removal on the normal path | `src/middleware/security_headers.rs:29-104`; verified on the wire |
| Global token-bucket throttle, 60 burst / 1 per second | `src/middleware/rate_limit.rs:46-73`; verified: 62 through, then `429` |
| Request + per-route counting, static excluded, 404s excluded from the route map | `src/middleware/vitals.rs:25-46`, tested `:64-97` |
| Counters unreachable by throttled requests | `src/router.rs:69-75`; verified by flood measurement |
| `/status` HTML and `/status.json`, both `no-store` | `src/handlers/status.rs:46-58`, tested `:126-135` |
| Snapshot shares the live runtime's `Arc`s | tested `src/handlers/status.rs:141-162` |
| RSS from `/proc` with a 5 s cache; `None` off Linux; failed reads cached | `src/state.rs:106-112`, `:262-272`, tested `:373-408` |
| Bind exposure classified, never published verbatim | `src/state.rs:188-220`, tested `:348-370` |
| RFC 9116 `security.txt` at both URIs | `src/handlers/well_known.rs:20-42`, `src/router.rs:54-55`, tested `:123-140` |
| `robots.txt` with 18 AI agents blocked from `/static/` | `src/handlers/well_known.rs:45-115`, tested `:142-159` |
| Themed 404 with escaped path echo; themed 500 leaking nothing | `src/errors.rs:104-141`, tested `:160-193` |
| Reduced-motion-safe boot animation | `static/css/style.css:1275-1290` |
| CI runs fmt, clippy `-D warnings`, tests, release build | `.github/workflows/ci.yml:26-36` |
| Two published ops posts with reproducible evidence and honest limits | `content/posts/hosting-machinageist-dev.md`, `…/security-headers-on-machinageist-dev.md` |

**Findings — defects and gaps, all verified:**

- **F1 — `429` responses carry no security headers and no `Content-Type`.**
  `add_security_headers` is the *innermost* layer (`src/router.rs:66`) while
  `rate_limit` short-circuits at `src/middleware/rate_limit.rs:66-72`. Verified:
  the `429` capture in §4.3 has none of the six headers and no declared content
  type, so a browser may MIME-sniff it. The comment at `src/router.rs:8`
  ("security_headers stamps every outgoing response") is inaccurate for exactly
  this path. **Severity: highest in this spec** — it is the one place where the
  claim made in a published blog post does not hold on the wire.

- **F2 — no access log at the default log level.** `TraceLayer::new_for_http()`
  (`src/router.rs:77`) uses tower-http's defaults, which emit the span and the
  request/response events at **DEBUG**, while `main.rs:34` defaults the filter to
  `mg_server=info,tower_http=info`. **Verified:** 70 requests produced zero
  request lines under the default filter, and re-running with
  `RUST_LOG=tower_http=debug` produced
  `DEBUG request{method=GET uri=/about ...}: started processing request` and
  `... finished processing request latency=1 ms status=200`. The comment at
  `src/router.rs:76` ("Log every request: method, path, status code, response
  time") describes behavior that does not occur as configured. Today the only
  journald output is the startup line and `WARN rate limit exceeded`.

- **F3 — `/static/<missing>` returns a completely blank 404.** Verified:
  `content-length: 0`. `nest_service` (`src/router.rs:59`) means `ServeDir`
  handles its own misses and the router fallback never runs.

- **F4 — no test covers security headers or rate limiting.** Verified by grep:
  the strings `content-security-policy`, `nosniff`, `429`, and `TOO_MANY` appear
  in `src/**` only inside the two middleware files themselves. Neither
  `security_headers.rs` nor `rate_limit.rs` has a `#[cfg(test)]` module. The two
  most security-relevant files in the crate are the two with no tests.

- **F5 — `axum-client-ip = "0.5"` is declared and unused.** Verified by grep: zero
  references in `src/`. It implies per-IP limiting exists. It does not
  (`src/middleware/rate_limit.rs:12`).

- **F6 — `--text-faint` fails WCAG AA on this feature's surfaces in 7 of 23
  themes** (on `--surface`) and 3 of 23 (on `--bg`), including the **default**
  theme. Computed from the token blocks in `static/css/style.css`; full table in
  §3.7. The header comment at `style.css:10-13` asserts all 23 are AA-validated.

- **F7 — no graceful shutdown.** `src/main.rs:61`. `systemctl restart` drops
  in-flight requests.

- **F8 — bind failure panics.** `src/main.rs:58`. Verified: `AddrInUse` produced a
  raw panic with a `RUST_BACKTRACE` hint rather than an operator-readable line.

- **F9 — the `Canonical` field omits `/security.txt`**, which is served
  (`src/router.rs:55`). A strict RFC 9116 validator fetching the root URI sees a
  canonical set that excludes it.

- **F10 — `SECURITY_TXT_EXPIRES` has no drift guard.** `src/handlers/well_known.rs:17`
  hard-codes `2027-05-16T00:00:00Z`; `:7-8` says to renew yearly. Nothing fails
  when it lapses; the file simply becomes invalid to scanners, silently.

- **F11 — hidden coupling via the process global.** `templates/vitals_strip.html:6`
  calls `crate::state::Status::current()` from inside a template; `init_global` is
  one-shot per process so exactly one test may publish
  (`src/handlers/status.rs:137-139`); and `/status` renders the request count from
  two independent snapshots in one document. Analyzed in §4.4.

- **F12 — `src/state.rs:91` promises a `/stats` page that is gated** by
  `docs/plans/deferred-dashboard-notes.md`.

- **F13 — the `?v=20260719-spectrum` cache-buster is hand-maintained in four
  places** in `templates/base.html`, and `ServeDir` sets no `Cache-Control`, so
  every asset revalidates and spends a rate-limit token.

- **F14 — `bind` reports `loopback (127.0.0.1)` for any loopback address.**
  Verified on `127.0.0.2`. Safe but inaccurate.

- **F15 — documentation drift.** `README.md`'s project tree omits `state.rs`,
  `middleware/vitals.rs`, and `handlers/status.rs` (`handlers/releases.rs` **is**
  present, at `README.md:72` — an earlier draft of this spec said otherwise); the
  Security section lists the six headers but not the rate limiter's actual
  (global, non-per-IP) behavior. `IMPROVEMENT_PLAN.md:15,41-47` maps ops artifacts
  onto a superseded four-CompTIA cert spine. `docs/agent-context/README.md` is
  referenced by the global `CLAUDE.md` but **does not exist** in this repo.

**Prototyped:** `AppState::hits()` (`src/state.rs:92-94`) — collected on every
resolved request, read by nothing but tests, `#[allow(dead_code)]` with a comment
pointing at a gated page.

**Absent (and not proposed):** alerting, uptime monitoring, external health
checks, metrics export (Prometheus/OTel), structured JSON logging, log shipping,
tested backup/restore, CI/CD, automated rollback, per-IP limiting, a dashboard,
a `/stats` route, request tracing across the Cloudflare/Caddy hops.

### 7.2 Delta to spec

**Modified files**

| File | Change | Fixes |
|---|---|---|
| `src/router.rs` | Move `add_security_headers` from innermost to just inside `TraceLayer`; update the three ordering comments; configure `TraceLayer` levels + path-only span; attach `not_found_service` and a `Cache-Control` layer to the `/static` service | F1, F2, F3, F13 |
| `src/middleware/rate_limit.rs` | Add `Content-Type`, `Retry-After` (from `NotUntil::wait_time_from`), `Cache-Control: no-store` to the `429`; rewrite the header comment so per-IP reads as gated, not imminent; **add a `#[cfg(test)]` module** (T2–T4) | F1, F4, F5 |
| `src/middleware/security_headers.rs` | **Add a `#[cfg(test)]` module** (T1) | F4 |
| `src/state.rs` | Extract `parse_vm_rss`; correct `BindMode::description` to family-only; **update `bind_description_comes_from_the_resolved_listener_address` (`:348-370`) to the new literals — `"loopback (IPv4)"`, `"loopback (IPv6)"`, `"all IPv4 interfaces"`, `"all IPv6 interfaces"`, `"custom interface"`** — it asserts the old strings and would otherwise fail; rewrite the `/stats` comment on `hits()`; doc-comment `init_global`'s one-publisher rule; add T5–T7 | F12, F14, F11 |
| `src/handlers/status.rs` | Add T8–T10; keep all existing tests | F4, 1F |
| `src/handlers/well_known.rs` | Second `Canonical` line; dated "AI list last reviewed" comment; add T11–T13 | F9, F10 |
| `src/errors.rs` | Fix 404/500 `title()` to site convention; add T14–T16 | a11y, F3 |
| `src/main.rs` | `with_graceful_shutdown` on SIGTERM/SIGINT; replace the bind `unwrap()` with a logged error + `exit(1)` | F7, F8 |
| `Cargo.toml` | Remove `axum-client-ip` | F5 |
| `templates/status.html` | Gloss rewrites (noun-leading); "memory readout requires Linux"; retention-paragraph rewrite per §6.1 | a11y, 1D |
| `static/css/style.css` | `.error-page a:focus-visible { animation: none; opacity: 1; }`; the `--text-faint` palette fix **(A1)** | a11y, F6 |
| `templates/vitals_strip.html` | Role-bearing element for the accessible name; comment recording the independent snapshot **(A2)** | a11y, F11 |
| `templates/base.html` | `?v={{ asset_version }}` in all four places **(A2)**, gating Phase 2 caching | F13 |
| `README.md` | Correct the project tree; describe the limiter accurately as global rather than per-IP | F15 |

**New files:** none in `src/`. Optionally
`docs/incidents/502-systemd-203-exec.md` and `docs/ops-gap-analysis.md` per §7.5,
plus their published post counterparts under `content/posts/`.

**Migrations / schema changes:** N/A — no database, no persisted state, no stored
schema anywhere in this feature.

**New dependencies:** none. **Removed:** one.

### 7.3 Estimated scope

**M**, split into four independently shippable slices.

| Slice | Content | Size | Why now |
|---|---|---|---|
| **S1 — Correctness** | Layer reorder; `429` headers/`Retry-After`/`Content-Type`; T1–T4 | S | F1 is the only place a published claim fails on the wire. Two lines move; the tests are the work |
| **S2 — Guards** | T5–T16; `Canonical` fix; expiry guard; `bind` fix; title fixes; comment corrections; drop `axum-client-ip` | M | Turns comments into CI failures. Largest slice by line count, lowest by risk |
| **S3 — Operability** | `TraceLayer` levels + path-only span; `/status` retention copy; graceful shutdown; bind-failure message; `/static` themed 404 + `Cache-Control` phase 1 | S–M | F2 is the precondition for every ops artifact in §7.5 |
| **S4 — Cross-feature** | Contrast fix + T17 (**A1**); strip role (**A2**); `asset_version` (**A2**) + caching phase 2 | S each | Blocked on A1/A2; T17 must land with the palette fix or CI goes red |

Not XL: no new route, no new template, no new dependency, no schema, no
migration, no infrastructure change. The bulk is tests for code that already
works.

### 7.4 Blocking dependencies

| Blocker | Owner | Blocks | Notes |
|---|---|---|---|
| `--text-faint` palette fix across 23 themes | **A1** | S4, T17 | Measured failures in §3.7. Interim: switch to `--text-muted` (7 failures → 1) |
| T17 landing **with** the palette fix | **A1** | S4 | Landing the test first turns CI red on unrelated work |
| Vitals strip role-bearing element | **A2** | a11y sign-off | `aria-label` on a bare `<div>` is dropped by most AT |
| `asset_version` in `base.html` (4 sites) | **A2** | Caching phase 2 | `immutable` before this lands is a year-long stale-asset footgun |
| Caddy trusted-hop config (`CF-Connecting-IP` overwrite) | **External / infra** | per-IP limiting | Outside this repo. Until verified, per-IP is worse than global |
| Real journald evidence, or a safe local reproduction | **External / Jeff** | 502 incident artifact | The hosting post already refuses to reconstruct timestamps from memory (`content/posts/hosting-machinageist-dev.md:88-90`) — that refusal is the credibility, and this spec preserves it |
| Six revisit questions answered | **Jeff** | any dashboard concept | `docs/plans/deferred-dashboard-notes.md`. **Not proposed here** |

Nothing blocks S1, S2, or S3. Nothing in this feature is blocked by B or C
features; A3 is a leaf on the foundation tier.

### 7.5 What ships publicly vs. what stays a private runbook

Criterion 4B rewards surfacing verification, failure, and recovery. It does not
reward publishing a filesystem map. The rule this spec proposes:

> **A public ops artifact may name a mechanism, a command, and a unit name. It
> may not name a path, host, IP, or identifier that exists only on Jeff's
> machine.**

That is the same information-disclosure principle `src/errors.rs:12-16` already
applies to error responses, extended to prose. Both published posts already
satisfy it: the hosting post shows `systemctl status mg-server.service` — a unit
name, fine — and never shows an `ExecStart` path.

**Ships publicly:**

1. **`/status` and `/status.json`** — already live, already honest, no claim
   attached. Keep.
2. **"What This Server Does Not Tell Me"** — the observability gap analysis
   (`IMPROVEMENT_PLAN.md:178-203`) written as a post. Content: journald is the
   only log and its retention is systemd's default; there is no alerting, so
   discovery is "I looked"; there is no external health check; backups are
   untested; rollback is manual. *This is the criterion-4B artifact* — it is the
   post the competitor set does not write, because absence is not flattering.
   Evidence-standard fields: **all thirteen fillable today**, because the evidence
   *is* the absence, and the verification is "here is the command that shows you
   nothing is there."
3. **Access-log posture note** — a paragraph, not a post: what the server records,
   what it does not, and that Cloudflare's edge logs are outside Jeff's control.
   Lands with S3 in the `/status` copy.

**Ships publicly only when the evidence exists:**

4. **502 → systemd `203/EXEC` incident report.** All thirteen evidence-standard
   fields are fillable **except real logs**. `IMPROVEMENT_PLAN.md:142-146` sets
   the rule: use real output, or label it a reconstruction, or reproduce it safely
   in a local test service — never invent timestamps. The hosting post already
   defers it on exactly those grounds. **Publish only with real journald output or
   a labeled local reproduction.** A reconstruction dressed as a log would be the
   single most damaging thing this site could publish, because the whole asset is
   that everything on it is true.

**Stays private (runbook only):**

- Unit-file contents, `ExecStart`, service user, binary path — recon data.
- Caddy config, tunnel ID, tunnel credentials, hostname, any real IP.
- journald retention settings and backup schedules.
- Proxmox node names, VM IDs, storage layout.

**Explicitly not proposed, with the reason recorded:** a dashboard, a `/stats`
route, an uptime badge, a status-history graph, a "monitoring" claim, alerting
copy, or any SLA language. `docs/plans/deferred-dashboard-notes.md` gates the
concept; §1.2 and §6.3 forbid the posture; and a personal site that displays a
99.9 % uptime figure it cannot measure has spent its credibility on decoration.

### 7.6 Documentation that must be updated in the same change (criterion 5E)

| Document | Trigger | Update |
|---|---|---|
| `README.md` "Security" section | S1/S2 land | Describe the limiter accurately: **global** token bucket, 60 burst, 1/s, not per-IP; add the `429` contract |
| `README.md` project tree | Any | Add `state.rs`, `middleware/vitals.rs`, `handlers/status.rs`, `handlers/releases.rs` |
| `content/posts/security-headers-on-machinageist-dev.md` | S1 lands | Add a short "what I found reviewing my own middleware" note: the `429` bypassed the header layer, here is the ordering fix. **This is a better artifact than the original post** — a header audit that found a real hole in the author's own code, with a before/after `curl`, is precisely criterion 4B |
| `content/posts/hosting-machinageist-dev.md` | S3 lands | Update the "no automated monitoring" section if access logging changes what is knowable; the bullet stays true either way |
| `IMPROVEMENT_PLAN.md` §5, §7 | S3 lands | Mark the logging gap as partially closed; keep monitoring/alerting/backup marked absent |
| `docs/solarcore/SOLARCORE_SPEC.md` | S4 lands | **A1 owns this**; the `--text-faint` change is a token change and must land in A1's rewrite, not silently in CSS |
| `src/router.rs` header comment | S1 lands | The layer-order rationale must move with the layers |

---

## 8. Open Questions

- **Q1 — Access logging: on by default, or opt-in?** Leveling `TraceLayer` to
  INFO gives journald a real access log (F2) at the cost of a line per request
  forever, in a journal whose retention Jeff has not configured. Alternative:
  leave it at DEBUG and document `RUST_LOG=tower_http=debug` as the triage switch.
  This spec recommends **INFO with a path-only span** (no query string, no UA, no
  IP) because "I could not tell what happened" is the recurring theme of the ops
  backlog. — *blocks:* §4.4, §6.1, S3.

- **Q2 — Does the `/status` retention copy need to ship before or with access
  logging?** If logging lands first, the current sentence is briefly incomplete.
  Recommendation: same commit, non-negotiable. — *blocks:* §6.1, S3.

- **Q3 — Remove `axum-client-ip` now, or keep it as a bookmark?** Removing is
  cleaner and honest; keeping signals intent. Recommendation: **remove**, and note
  the intent in `rate_limit.rs`'s comment instead, where a reader will actually
  find it. — *blocks:* §4.5, S2.

- **Q4 — Is a themed 404 the right response for a missing static asset?** It fixes
  the blank page (F3) but sends ~10 KB of HTML to what is usually a subresource
  fetch. Alternative: a one-line `text/plain` 404 for `/static/*` only.
  Recommendation: **themed**, for consistency of the site's designed empty state;
  misses are rare and indicate a bug. — *blocks:* §3.6 E2, S3.

- **Q5 — Should `robots.txt` gain a `Sitemap:` line?** There is no sitemap today.
  Adding the line without the file is a broken promise; adding a generated sitemap
  is a new capability that overlaps `C1 search`. Recommendation: **defer to C1**,
  which will need a content index anyway. — *blocks:* §4.3.

- **Q6 — Does `security.txt` need a `Policy:` field?** RFC 9116 recommends it, but
  it must point at a real disclosure-policy page. Writing one is small; writing
  one that overclaims a response SLA is easy. Recommendation: add the field only
  alongside a page that says plainly "this is a personal site, I will respond when
  I see it, there is no bounty." — *blocks:* §4.3.

- **Q7 — `IMPROVEMENT_PLAN.md` maps every ops artifact onto the superseded
  four-CompTIA spine** (`:15`, `:41-47`). The artifacts are still right; the cert
  mapping is stale under criterion 1D. Should this spec's remediation include
  re-anchoring that mapping to RHCSA → CCNA → Security+, or is `IMPROVEMENT_PLAN.md`
  frozen as a historical planning document? — *blocks:* §6.3, §7.6.

- **Q8 — `docs/agent-context/README.md` is referenced by the global `CLAUDE.md`
  but does not exist in this repo.** Was it never created, or lost in the reorg
  (`docs/REORG_CHANGELOG.md`)? Future sessions will look for it. — *blocks:*
  nothing in this feature; flagged because it will cost every future agent time.

- **Q9 — Sub-feature candidate: a `mg-health` external check.**
  `IMPROVEMENT_PLAN.md:248-250` floats a tool consuming HTTP status, TLS expiry,
  and `systemctl` state into JSON/Markdown. That is a **separate feature**, not
  part of A3 — it runs *outside* this process and would be the first thing that
  could legitimately be called monitoring. Recommend a its own feature ID in a
  future tree revision rather than expanding A3. — *blocks:* nothing; recorded per
  the no-sub-agent rule.

- **Q10 — Should the contrast guard (T17) live in A1's spec instead of A3's?** It
  is a whole-site palette property; A3 only measured it because A3's surfaces are
  where it fails. Recommendation: **A1 owns the test**, A3 keeps the measured
  table as the evidence that motivated it. — *blocks:* §5.1 T17, S4.
