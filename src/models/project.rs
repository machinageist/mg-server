// Author:      machinageist
// Date:        2026-04
// Description: Defines Project and ProjectStatus — the data model for portfolio
//              entries. Projects are hardcoded in all() as a static Vec rather
//              than loaded from files because they are structured, typed, and
//              change rarely. The compiler validates every field at build time.
//              ProjectStatus implements Display so templates can render
//              {{ project.status }} directly without a helper method.
//
// Notes:       &'static str fields have zero runtime allocation cost — the data
//              is embedded in the binary at compile time and lives for the entire
//              program. Appropriate for values that never change at runtime.
//              Adding a new ProjectStatus variant without updating the Display
//              match is a compile error — exhaustive matching enforced by Rust.
//              url is Option<&'static str> — None projects render without a link.

// -----------------------------------------------------------------------
// Data types
// -----------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Project {
    pub name:        &'static str,
    pub description: &'static str,
    // Fixed-size slice of static string slices — zero allocation
    pub tags:        &'static [&'static str],
    // None = not yet published or no public repo
    pub url:         Option<&'static str>,
    pub status:      ProjectStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Active,
    #[allow(dead_code)]
    InProgress,
    Complete,
}

// Allow {{ project.status }} in Askama templates — renders the display string directly
impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProjectStatus::Active     => write!(f, "active"),
            ProjectStatus::InProgress => write!(f, "in progress"),
            ProjectStatus::Complete   => write!(f, "complete"),
        }
    }
}

// -----------------------------------------------------------------------
// Project list — add new entries here as projects are built
// -----------------------------------------------------------------------

// Return canonical project list — called by portfolio handler on each request
pub fn all() -> Vec<Project> {
    vec![
        Project {
            name:        "GeistScope",
            description: "Automated bug bounty toolchain for human + AI collaboration. \
                          11 Rust binaries covering the full recon-to-submission pipeline: \
                          subdomain enumeration (CT logs + DNS brute force), async port scanning, \
                          HTTP tech stack fingerprinting, BFS web crawling with JS secret extraction, \
                          passive security posture checking (headers, CORS, cookies, exposed paths), \
                          Burp Intruder-style payload fuzzing with four attack modes, \
                          Burp Repeater-style finding verification, LLM-ranked attack surface \
                          via Anthropic or local Ollama, and a Ratatui terminal dashboard. \
                          Every tool writes to a shared file layout — no custom IPC needed \
                          for AI co-operation.",
            tags:        &["rust", "security", "bug-bounty", "async", "tokio", "ratatui", "ai"],
            url:         Some("https://github.com/machinageist/geistscope"),
            status:      ProjectStatus::Active,
        },
        Project {
            name:        "mg-server",
            description: "This site. Personal portfolio and blog server built from scratch in Rust. \
                          Axum routing, Askama compile-time templates (broken template = build error, \
                          not runtime 500), flat-file Markdown blog with YAML frontmatter, \
                          security headers middleware, and a 60 req/min rate limiter. \
                          Deployed behind Caddy and Cloudflare Tunnel — no open inbound ports. \
                          Verified with gobuster, nmap, curl traversal payloads, and SSL Labs.",
            tags:        &["rust", "axum", "web", "askama", "security"],
            url:         Some("https://github.com/machinageist/mg-server"),
            status:      ProjectStatus::Active,
        },
        Project {
            name:        "mg-scan",
            description: "Async TCP port scanner with banner grabbing, randomised scan order, \
                          configurable delay and jitter for rate-based IDS evasion, and optional \
                          source port binding for firewall bypass testing. \
                          Concurrency managed with Tokio JoinSet — no semaphore allocations \
                          per task. Part of the GeistScope toolchain.",
            tags:        &["rust", "networking", "security", "tokio", "port-scanning"],
            url:         Some("https://github.com/machinageist/geistscope"),
            status:      ProjectStatus::Complete,
        },
        Project {
            name:        "mg-fuzz",
            description: "Burp Intruder-equivalent HTTP fuzzer. Reads raw HTTP request templates \
                          with §marker§ injection positions. Four attack modes: sniper, battering-ram, \
                          pitchfork, cluster-bomb. Built-in payload sets for SQLi, XSS, SSTI, \
                          path traversal, SSRF, and more. Diffs each response against a baseline \
                          (status, body hash, length delta, timing anomaly) to surface interesting \
                          responses automatically.",
            tags:        &["rust", "security", "fuzzing", "web-security", "bug-bounty"],
            url:         Some("https://github.com/machinageist/geistscope"),
            status:      ProjectStatus::Complete,
        },
        Project {
            name:        "mg-tui",
            description: "Ratatui terminal dashboard for the GeistScope toolchain. \
                          Five tabs: engagements, hosts, findings (with severity filter), \
                          fuzz results, and live audit log tail. Full mouse support — \
                          scroll, click tab bar, click links in the built-in terminal browser. \
                          The browser renders HTML to styled terminal spans with Unicode \
                          half-block image rendering and a navigation history stack. \
                          Refreshes from disk on a 2-second timer with no extra processes.",
            tags:        &["rust", "ratatui", "tui", "security", "terminal"],
            url:         Some("https://github.com/machinageist/geistscope"),
            status:      ProjectStatus::Complete,
        },
    ]
}
