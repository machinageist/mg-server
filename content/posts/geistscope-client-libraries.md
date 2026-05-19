---
title: "GeistScope Part 6: The Shared Client Layer"
date: 2026-05-11
summary: "Two library crates that every tool in the pipeline depends on: http-client handles UA rotation, rate limiting, and retry with jittered backoff. llm-client gives a single interface over local Ollama and remote Anthropic models."
tags: [rust, security, bug-bounty, geistscope, networking, ai]
---

## Libraries Beneath the Tools

Parts 1–5 of this series covered the user-facing binaries: workspace setup, recon,
crawling, fuzzing, and the terminal dashboard. But six of those binaries call the
same two library crates on every request: `http-client` and `llm-client`.
Neither is glamorous. Both are load-bearing.

---

## `http-client`: Controlled HTTP at the Foundation

Every tool that touches the network — `subdomain-enum`, `mg-scan`, `mg-crawl`,
`corpus-builder`, `mg-fuzz` — imports `http-client` instead of calling `reqwest`
directly. Three reasons:

**UA rotation.** Tools that make hundreds of requests to the same target need to
avoid looking like a scanner. The client cycles through six realistic browser
strings on every request — Chrome on Windows, Chrome on Mac, Chrome on Linux,
Firefox on Windows, Firefox on Mac, Safari on Mac — in a deterministic round-robin
using an `AtomicUsize`:

```rust
static USER_AGENTS: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 ...",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 ...",
    // ...
];
```

The counter is atomic so concurrent tasks each advance it without locking.

**Rate limiting.** Every client instance holds a `Mutex<Instant>` for the last
request time. The `throttle()` method computes elapsed time and sleeps the
difference before allowing the next request:

```rust
async fn throttle(&self) {
    let Some(min_ms) = self.config.rate_limit_ms else { return; };
    let min = Duration::from_millis(min_ms);
    let mut last = self.last_req.lock().await;
    let elapsed = last.elapsed();
    if elapsed < min {
        tokio::time::sleep(min - elapsed).await;
    }
    *last = Instant::now();
}
```

Rate limiting is per-instance, so a tool can create separate clients with different
budgets — one conservative client for target probing, one faster client for public
APIs like crt.sh.

**Retry with jittered backoff.** Transient failures — timeouts, TCP resets, brief
502s — shouldn't abort a long recon run. The client retries up to `max_retries`
times with exponential base delay and 200ms of uniform jitter:

```rust
let base_ms = 300u64 * (1u64 << attempt);
let jitter_ms = fastrand::u64(0..200);
tokio::time::sleep(Duration::from_millis(base_ms + jitter_ms)).await;
```

The jitter matters when many concurrent tasks hit the same transient failure.
Without it, all retries wake up at the same instant and flood the target again.

The public surface is small: `get`, `post_json`, `get_json`, `get_text`,
`post_json_text`. Callers configure a `ClientConfig` struct and get a `Client`.
That's it.

---

## `llm-client`: One Interface, Two Backends

Several tools need to call a language model — `ai-prioritize`, `mg-report`,
`mg-recopilot`, `mg-exploitgen`. The choice of model (local vs. remote, which
provider) is an operator decision that shouldn't be baked into each tool.

`llm-client` exposes a single enum with two variants:

```rust
pub enum LlmClient {
    Ollama(OllamaClient),
    Anthropic(AnthropicClient),
}

impl LlmClient {
    pub fn ollama(model: impl Into<String>) -> Result<Self, LlmError> { ... }
    pub fn anthropic(api_key: impl Into<String>, model: impl Into<String>) -> Result<Self, LlmError> { ... }
    pub async fn complete(&self, system: &str, user: &str) -> Result<String, LlmError> { ... }
}
```

Every tool that calls the LLM checks `ANTHROPIC_API_KEY` at startup. If it's set,
you get `AnthropicClient` pointing at Claude Sonnet 4.6. If not, you get
`OllamaClient` pointing at a local Llama model. The same `complete(system, user)`
call works either way.

This matters for the offline mode that every AI tool supports. When you pass
`--offline`, the tool skips the LLM call entirely and generates a deterministic
placeholder. No API key, no network, reproducible output for testing. The
offline/online branching happens in each tool's lib; `llm-client` doesn't need
to know about it.

---

## Why Shared Rather Than Inline

The alternative was duplicating the retry logic in each binary or calling `reqwest`
directly from `mg-crawl`, `corpus-builder`, etc. That would work until it didn't:
the first time a target started returning 429s and one tool handled it differently
from another, or UA rotation was only in some tools and not others.

Shared libraries make policy uniform. If rate limiting needs to be tightened,
one change in `http-client` applies everywhere. If a new Anthropic model replaces
the current one, one change in `llm-client` is enough.

---

*Part 7 covers `corpus-builder`: mining certificate transparency logs and the
Wayback Machine to reconstruct a target's historical attack surface before any
active probing begins.*
