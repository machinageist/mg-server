# Blog outline — "A status line with no JavaScript"

Draft outline only — Jeff writes the prose. Phase 1 written artifact from the
widgets plan (docs/WIDGETS_HANDOFF_PROMPT.md).

## Angle

Every page on this site carries a one-line readout of the process serving it —
uptime, request count, memory, build stamp — and it ships zero JavaScript.
The piece is about how much observability you get from server-side rendering
plus two atomics, and why "a stamp, not a feed" is the honest design.

## Outline

1. **The widget** — what the footer strip shows; screenshot in CRT mode.
   Values are rendered at request time; refresh and they move. No polling,
   no websocket, no client code.
2. **AppState in ~50 lines** — `Instant` for uptime, `AtomicU64` for the
   request total (`Ordering::Relaxed` and why that's enough for a counter),
   `Mutex<HashMap>` for per-route hits and why dashmap wasn't justified at
   this traffic level. Cheap-clone state: every field Copy or Arc.
3. **The middleware** — `from_fn_with_state`, placed inside the rate limiter
   so rejected floods never inflate the numbers; static assets and 404s
   excluded so the counts mean something.
4. **Reading your own memory** — parsing `VmRSS` from `/proc/self/status`,
   returning `Option` instead of panicking off-Linux; dev is macOS so the
   field simply disappears. What RSS actually measures.
5. **The OnceLock trick** — the footer renders on every page, but threading
   state into every Askama template struct would touch every handler. A
   process-global `OnceLock<AppState>` clone shares the same Arcs; the
   template calls `Status::current()` directly. Trade-off discussion:
   globals vs. plumbing, and why read-only snapshot access makes this safe.
6. **Build stamps** — a five-line `build.rs` emitting epoch seconds via
   `cargo:rustc-env`; version from `CARGO_PKG_VERSION`. The strip proves
   which binary is live.
7. **What it deliberately doesn't say** — no hostname, no addresses, no
   paths, no per-core detail. The Status struct is the allowlist; verbose
   status pages are recon data.
8. **Close** — /status.json exists for machines; the terminal mode that will
   consume it is coming. Total cost: one middleware, one struct, no JS.

## Evidence to include

- `src/state.rs`, `src/middleware/vitals.rs`, `build.rs` excerpts
- `curl /status.json` output
- The vitals-strip test asserting /static is excluded from counts
