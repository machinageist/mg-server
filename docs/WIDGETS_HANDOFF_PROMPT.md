# WIDGETS_HANDOFF_PROMPT.md
<!-- Paste everything below the line into a fresh Fable CLI session at the mg-server repo root. -->
<!-- Origin: Cowork brainstorm session 2026-07-12. -->

---

You are building a set of interactive widgets for **mg-server**, Jeff's personal Rust portfolio
site at machinageist.dev. Read this entire prompt before writing any code.

## Mission

Add ten small, content-first "gizmos" that demonstrate real engineering skill without being
gimmicky. The site's primary content is written artifacts (blog, wiki, portfolio); every widget
is a footnote that proves the writing is real. Aesthetic target: **"the 90s web imagined the
future correctly"** — terminal readouts, odometers, 88×31 badges, boot sequences — executed
with modern rigor (CSP-clean, accessible, tested, zero external assets).

Work in phases (defined below). Complete, verify, and commit each phase before starting the
next. Each phase ends with a working build, passing tests, and a clean `cargo audit`.

## Codebase orientation (verified 2026-07-12 — re-verify before you start)

- **Stack:** Axum 0.7, tokio (full), tower-http 0.5 (fs, trace), Askama 0.12 + askama_axum 0.4,
  pulldown-cmark, gray_matter, chrono, serde, thiserror, tracing, governor + axum-client-ip
  (rate limiting), sha2/hex. Rust edition 2024.
- **Layout:** `src/main.rs` (thin entry, binds 127.0.0.1:3000, `MG_BIND_ADDR` override),
  `src/router.rs` (single source of truth for all routes and middleware ordering),
  `src/handlers/` (blog, pages, releases, well_known, wiki), `src/middleware/` (rate_limit,
  security_headers), `src/models/` (page, post, project), `templates/` (Askama),
  `static/{css,js,assets}`, `content/{posts,pages}`, `tests/` (integration, see
  `tests/wiki_pages.rs` for the pattern).
- **No shared AppState exists yet.** `router::build()` takes no arguments. Introducing
  `AppState` is Phase 0 work.
- **CSP is `script-src 'self'`** (see `src/middleware/security_headers.rs`). No inline
  `<script>` or `<style>` anywhere, ever. All JS ships as external files under `static/js/`
  with `?v=YYYYMMDD-name` cache-busting query params (see `templates/base.html`).
- **A theme system already exists:** `static/js/theme-init.js` runs pre-paint in `<head>`,
  resolves `system | light | dark` from localStorage, sets `data-theme` on `<html>`; a menu
  button lives in `base.html`; CSS variables live under `:root` / `:root[data-theme="light"]`
  in `static/css/style.css`. **Extend it — do not rebuild it.**
- **Deployment:** Internet → managed edge → private connector → reverse proxy → application.
  Treat forwarded client identity as trusted only on the authenticated proxy path.

## Working agreement (non-negotiable)

1. **Explain before you implement.** At the start of each phase, present: an annotated list of
   files you will touch, the approach, and any trade-off you chose — then wait for approval
   before writing code. Jeff's learning preference is concept-first; this codebase is a
   teaching artifact.
2. **Code style.** Every logic-heavy file gets the header block
   (`// Author / Date / Description / Notes`). Section-divider comments above blocks.
   `// Verb + noun` comment above every function, plain-English fragments, no periods.
   `ALL_CAPS_SNAKE_CASE` constants — never inline magic strings or numbers. Declare and
   initialize together. 4-space indentation. Match the voice of `src/router.rs` exactly.
3. **Small verified slices.** One feature per commit minimum. `cargo build`, `cargo test`,
   `cargo clippy -- -D warnings`, and `cargo audit` must pass before any commit.
4. **No invented output.** Never claim a test passed or a page rendered without running it.
5. **Dependency discipline.** Prefer zero new dependencies. If one is unavoidable, justify it,
   pin minimal feature flags, and run `cargo audit` after adding it.
6. **Security posture is the brand.** Nothing may weaken the CSP, leak internal paths, log
   or store visitor PII, or open an unauthenticated write path (guestbook excepted, and it is
   heavily constrained below).
7. **Accessibility floor:** every widget honors `prefers-reduced-motion`, meets WCAG AA
   contrast in every theme, and is keyboard-operable. Decorative elements get
   `aria-hidden="true"`.
8. **No external requests from the browser.** No CDNs, no fonts, no analytics. Everything is
   served from this server.

## Phase 0 — Foundation: shared AppState

Everything downstream needs shared state. Do this first.

- Create `src/state.rs` with `AppState` (cheap-clone: `Arc` internals):
  - `started_at: std::time::Instant` — process start, for uptime
  - `requests_total: Arc<AtomicU64>` — every non-static request
  - `page_hits: Arc<...>` — per-route counters (see Feature 4 for persistence; choose
    `dashmap` ONLY if you can justify it, otherwise `Mutex<HashMap<String, u64>>` is fine at
    this traffic level)
  - build metadata: `version` from `env!("CARGO_PKG_VERSION")`, plus a compile-time UTC
    build timestamp emitted by a small `build.rs` (`println!("cargo:rustc-env=BUILD_TS=...")`)
- Change `router::build()` to `build(state: AppState)`, thread it via `.with_state(state)`;
  `main.rs` constructs it. Keep `main.rs` thin.
- Add a counting middleware (`src/middleware/vitals.rs`): increments `requests_total` and the
  per-route counter, skipping `/static/*`. Place it inside the middleware stack so it runs
  after rate limiting (rejected floods should not inflate counts).
- Acceptance: existing tests still pass; new unit test proves the counter increments and
  `/static` is excluded.

## Phase 1 — Always-visible chrome

### Feature 1: Engine-room vitals strip + `/status`

The signature widget. A one-line rack-mount-style readout in the site footer on every page,
plus a full `/status` page.

- **Footer strip (server-rendered, zero JS):** Askama partial included from `base.html`
  showing: `UP <dd:hh:mm>` · `REQ <n>` · `MEM <n> MiB` · `v<version> · built <BUILD_TS>`.
  Values render at request time — no polling, no JS. This is a status *stamp*, not a live feed.
- **Memory:** read `VmRSS` from `/proc/self/status` on Linux; return `None` gracefully on
  other platforms (dev is macOS) and omit the field. Never panic on parse failure.
- **`/status` page:** full readout — uptime, request total, RSS, version, build timestamp,
  bind mode, and a short paragraph explaining that this is a real Rust process on a homelab
  box (that paragraph is content; write it well).
- **`/status.json`:** same data as JSON (serde). This is the API the terminal mode (Feature 8)
  consumes. Add `serde_json` if not already present (it is listed as a Phase-3 dependency in
  `Cargo.toml` comments — uncommenting it is pre-approved).
- **Hard rules:** no hostname, no internal IPs, no path disclosure, no per-core detail. Uptime,
  counts, RSS, and version only.
- **Styling:** monospace, LED-segment feel via CSS only (letter-spacing, subtle text-shadow in
  CRT themes). Must degrade to plain text.
- Acceptance: integration test asserts `/status` is 200 and contains `UP`, `/status.json`
  parses, and the footer strip appears on `/` and `/blog`.

### Feature 2: Display modes (retro theme expansion)

Extend the existing three-mode theme system into "display modes."

- New `data-theme` values: `crt` (green phosphor on near-black), `amber` (P3 phosphor),
  `paper` (light, ink-on-newsprint). Keep `system | light | dark` working unchanged.
- All colors defined as CSS variables in `style.css` under `:root[data-theme="crt"]` etc. —
  the existing pattern. Audit every variable the site uses and define all of them per theme;
  a half-themed page is worse than no theme.
- `crt` gets an optional pure-CSS scanline overlay (a fixed-position pseudo-element with a
  repeating-linear-gradient at very low opacity). It must be disabled entirely under
  `prefers-reduced-motion: reduce` and must never affect text contrast below WCAG AA.
- Update `theme-init.js` validation list and the `base.html` menu (keep the existing ARIA
  menu semantics — `aria-haspopup`, `aria-expanded`, `role="menu"`).
- Bump the `?v=` cache-busting params on `style.css`, `theme-init.js`, `main.js`.
- Acceptance: manual matrix — every page × every theme renders with no unstyled elements;
  contrast spot-checks pass; no console errors; localStorage round-trips across pages.

### Feature 3: Boot-sequence error pages

Restyle `templates/error_404.html` and `templates/error_500.html`.

- 404: retro disk-read flavor — `SECTOR NOT FOUND — 0 pages read at <path>` with the requested
  path HTML-escaped (Askama auto-escape; verify with a `<script>` probe test). Link home.
- 500: kernel-panic framing — monospace panic block, zero internal detail. The joke is the
  aesthetic; the content stays generic. Verify nothing from the real error reaches the page.
- Pure CSS "type-in" animation is allowed (steps() on a width or a staged opacity), fully
  static under `prefers-reduced-motion`. No JS.
- Acceptance: integration tests — unknown route returns 404 with escaped path; 500 body
  contains no `src/`, no `.rs`, no panic text.

## Phase 2 — Transparent instrumentation

### Feature 4: Odometer hit counter + `/stats` transparency page

- **Persistence:** counters from Phase 0 flush to `data/hits.json` (create `data/`, gitignore
  its contents) on a tokio interval task (every 60s, `DEFAULT_FLUSH_SECS` const) and on
  graceful shutdown (`axum::serve(...).with_graceful_shutdown(...)` on SIGTERM/ctrl-c — this
  also future-proofs systemd restarts). Load on boot; corrupt/missing file starts at zero
  with a `tracing::warn!`, never a panic.
- **What is counted:** page views per route pattern (`/blog/:slug` bucketed per slug), total
  requests. **What is never stored:** IPs, user agents, referrers, timestamps per hit,
  cookies. No fingerprinting of any kind.
- **Widget:** classic odometer on the home page footer — server-rendered digits in individual
  `<span>`s styled as wheels. Zero JS.
- **`/stats` page:** the counts, plus a methodology section stating exactly what is and is not
  collected and why. This transparency page is the actual portfolio piece.
- Acceptance: unit test for flush/load round-trip including corrupt-file recovery; integration
  test that a request to `/` increments the home counter; kill -TERM test (manual) confirms
  flush-on-shutdown.

### Feature 5: "Your connection" panel — `/you`

Show visitors their own request as a one-page networking lesson.

- Read from request headers/metadata: `CF-Ray` (render the trailing colo code and link the
  concept, e.g. `SJC — Cloudflare edge, San Jose`), `CF-IPCountry`, negotiated HTTP version
  (`axum::http::Version`), scheme via `X-Forwarded-Proto`, and the visitor's IP from
  `CF-Connecting-IP` **masked** (IPv4: last octet zeroed, e.g. `203.0.113.0`; IPv6: /48).
- Add a Caddy snippet to the deployment docs injecting `X-Tls-Version` and `X-Tls-Cipher`
  via Caddy placeholders (`{tls_version}`, `{tls_cipher}`) so the panel can display the TLS
  line when live; render `not available (direct dev connection)` when headers are absent.
  Every field must handle absence — local dev has no Cloudflare headers.
- Layout: a request "trace" — `You → Cloudflare edge (<colo>) → tunnel → Caddy → this Rust
  process`, each hop annotated with the evidence header. Diagram in semantic HTML/CSS, not an
  image.
- **Privacy rules:** everything is echoed back to the requester only — nothing on this route
  is logged (suppress or downgrade TraceLayer for `/you` if it would log the IP), nothing is
  stored, and the page says so. Add `Cache-Control: no-store` on this response.
- Acceptance: integration test with synthetic CF headers asserts masking is applied and the
  full IP appears nowhere in the body; test without headers asserts graceful fallbacks.

### Feature 6: Baud-rate read time

- In the post model, compute rendered-HTML byte size and derive transfer time at 56 kbit/s
  (`BAUD_BITS_PER_SEC` const, bytes × 8 / 56000). Also keep a words/200-wpm minutes estimate.
- Blog list and post header render: `~4 min read · 38s at 56k`. One `<span>` with a `title`
  attribute explaining the joke for the curious.
- Pure Rust, no JS. Acceptance: unit test with a fixture post of known size.

## Phase 3 — Destination pages

### Feature 7: Hardened guestbook — `/guestbook`

The only unauthenticated write path on the site. Treat it as hostile input by default; the
hardening itself becomes a blog post.

- **Model:** flat-file JSONL at `data/guestbook.jsonl`, append-only, one entry per line:
  `{ts, name, message, published: bool}`. No database.
- **Input constraints (consts):** name ≤ 40 chars, message ≤ 280 chars, both trimmed,
  reject empty, reject any message containing `http://`, `https://`, or `[url` patterns
  (kills 90% of spam), strip control characters. Askama auto-escape on render — verify with
  an XSS probe test, and never mark guestbook fields `|safe`.
- **Bot defenses (layered, no CAPTCHA, no third parties):** a honeypot field hidden via CSS
  (any value → silently accept and discard), a minimum-time trap (form embeds a signed
  timestamp — HMAC over ts using a server secret from env `MG_FORM_KEY`; reject submissions
  faster than 3s or older than 1h; sha2 is already a dependency, use HMAC-SHA256 — add the
  `hmac` crate, justify it), and a strict per-IP governor rate limit on the POST route
  (e.g. 2/hour) keyed on `CF-Connecting-IP` via the existing axum-client-ip setup. IPs used
  for rate limiting live in memory only and are never written to disk.
- **Moderation:** entries publish immediately but the JSONL `published` flag plus a
  documented one-liner (`scripts/guestbook-mod.sh` using jq, or a tiny `--unpublish <ts>`
  admin subcommand) makes takedown trivial. No admin web UI — SSH is the admin UI.
- **UX:** page shows newest 50, older behind `?page=`. Empty state written with care. On
  success redirect (PRG pattern) so refresh can't double-post.
- Acceptance: integration tests for length rejection, URL rejection, honeypot discard,
  time-trap rejection, XSS escape, rate-limit 429, and PRG redirect.

### Feature 8: Terminal mode — `/tty`

An alternate navigation, not the primary one. Link it subtly from the footer.

- Single Askama page + one external JS file (`static/js/tty.js`, CSP-safe). Renders a prompt
  `visitor@machinageist:~$` and implements a tiny parser: `help`, `ls [posts|wiki|pages]`,
  `cat <slug>` (fetches the real page and prints a text extract), `open <slug>` (navigates),
  `theme <mode>` (drives the existing theme system), `uptime` (fetches `/status.json`),
  `whoami`, `clear`. Unknown command → `command not found: <x>` (escaped).
- **Manifest endpoint** `/tty/manifest.json`: handler enumerates `content/posts` and
  `content/pages` slugs + titles (reuse existing model loaders — do not re-parse markdown in
  JS). `cat` uses each page's existing HTML route and extracts text client-side.
- Progressive enhancement: `<noscript>` block renders a plain link list of all content.
- All DOM insertion via `textContent`, never `innerHTML`. Command history (up-arrow) in
  memory only.
- Acceptance: manifest test (valid JSON, slugs match content dir); manual script of every
  command; XSS probe via `cat "<img onerror>"`-style input stays inert.

### Feature 9: Auto-generated route map — `/map`

- Refactor `router.rs` so routes are declared in a `const`-style table:
  `SITE_ROUTES: &[RouteEntry]` where `RouteEntry { path, kind, description }` — the table
  both registers routes (iterated in `build()` where signatures allow; document exceptions
  like the ServeDir nest) and renders `/map`. **Single source of truth is preserved by
  construction, not convention.**
- `/map` page: the route table rendered as an annotated tree with one-line descriptions,
  styled like a circuit diagram / directory listing hybrid. Include middleware order as a
  footnote — it teaches the request lifecycle.
- Add a test asserting every entry in `SITE_ROUTES` returns non-404 — the parity guarantee.
- Acceptance: that parity test, plus `/map` renders all entries.

### Feature 10: 88×31 self-audit badge

- A handler-generated SVG at `/badge.svg`, 88×31, webring-era styling: site name + the
  security posture the middleware actually enforces (e.g. `CSP ✓ HSTS ✓`). Derive the
  claims from the same constants `security_headers.rs` uses — the badge cannot drift from
  reality because it reads the config, not a hardcoded string.
- Serve with `Content-Type: image/svg+xml` and long cache. Escape nothing dynamic into it
  beyond those constants. Offer it on `/about` with copyable embed HTML pointing at
  machinageist.dev, as a nod to link-badge culture.
- No external grade fetching (SSL Labs / Observatory) in v1 — self-attested claims derived
  from code only. Note the external-grade version as future work.
- Acceptance: test asserts SVG parses, dimensions are 88×31, and claims flip if a header
  const is removed (compile-coupled).

## Cross-cutting verification (every phase)

- `cargo build && cargo test && cargo clippy -- -D warnings && cargo audit` — all clean.
- CSP check: no inline scripts/styles introduced (`grep -rn "<script>" templates/` should
  show only `src=` external references; same for `style=` attributes).
- Response-header check: `curl -I` each new route — security headers present, no
  cache-control mistakes (`/you` is `no-store`, `/badge.svg` is long-cache).
- Theme matrix: each new page in all six themes.
- Accessibility pass: keyboard-only walk of every new interactive element;
  `prefers-reduced-motion` verified via devtools emulation.
- Update `/map`'s route table and the README route list with every new route.
- Deployment note per phase: anything Caddy/tunnel-related (Feature 5's TLS headers,
  `MG_FORM_KEY` env for Feature 7, `data/` directory permissions) goes in the deploy docs.

## Written artifacts (the actual point)

Each phase ends with a blog post draft outline (not full prose — Jeff writes those):

- Phase 1: "A status line with no JavaScript" — AppState, atomics, /proc/self/status.
- Phase 2: "Analytics you can read in one JSON file" + "Showing you your own request."
- Phase 3: "Hardening a guestbook in 2026" and "Making the router draw its own map."

## Suggested order & sizing

| Phase | Features | Est. sessions |
|-------|----------|---------------|
| 0 | AppState foundation | 1 |
| 1 | Vitals, display modes, error pages | 2–3 |
| 2 | Hit counter/stats, /you, baud read-time | 2 |
| 3 | Guestbook, /tty, /map, badge | 3–4 |

Stop after any phase — each leaves the site better and shippable. Do not start Phase 3 in the
same session as Phase 2. If a spec conflicts with something you find in the code, say so and
propose the fix before proceeding; the code is the ground truth, this document is intent.
