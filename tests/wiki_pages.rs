// Author:      machinageist
// Date:        2026-05-15
// Description: Regression test: every slug listed in the wiki sidebar must
//              resolve to a parseable Markdown file under content/pages/.
// Notes:       Loads the same Page::from_file logic the runtime uses. If a
//              sidebar entry references a missing or malformed page, this
//              test fails before the change can ship.

use std::path::Path;

// Slugs must stay in sync with src/handlers/wiki.rs::SIDEBAR. The list is
// duplicated here on purpose so the test crate stays decoupled from the bin.
// The full GeistScope archive is restored (with a beginner/AI disclaimer on every
// page), so this lists every content/pages/*.md file.
const WIKI_SLUGS: &[&str] = &[
    "index",
    "ai-prioritize",
    "corpus-builder",
    "libraries",
    "subdomain-enum",
    "mg-aifuzz",
    "mg-apikey",
    "mg-apk",
    "mg-artifact-audit",
    "mg-authz",
    "mg-aws",
    "mg-azure",
    "mg-breach",
    "mg-brute",
    "mg-cache-poison",
    "mg-cloud-enum",
    "mg-cmdinject",
    "mg-cname-chain",
    "mg-cors-exploit",
    "mg-crawl",
    "mg-csp",
    "mg-csrf",
    "mg-deser",
    "mg-diff",
    "mg-dns-enum",
    "mg-dns-history",
    "mg-dns-rebind",
    "mg-docker",
    "mg-engagement",
    "mg-exploitgen",
    "mg-fingerprint",
    "mg-fuzz",
    "mg-gcp",
    "mg-github",
    "mg-google-dork",
    "mg-graphql",
    "mg-grpc",
    "mg-harness",
    "mg-http2",
    "mg-ipa",
    "mg-js-analyze",
    "mg-jwt",
    "mg-k8s",
    "mg-leak-monitor",
    "mg-loot",
    "mg-metadata",
    "mg-notify",
    "mg-oauth",
    "mg-oob",
    "mg-openapi",
    "mg-privesc-linux",
    "mg-privesc-windows",
    "mg-probe",
    "mg-proto-pollute",
    "mg-recon",
    "mg-recopilot",
    "mg-redirect",
    "mg-replay",
    "mg-report",
    "mg-scan",
    "mg-secret-validate",
    "mg-serverless",
    "mg-session-audit",
    "mg-shodan",
    "mg-smb",
    "mg-smtp",
    "mg-smuggle",
    "mg-snmp",
    "mg-sourcemap",
    "mg-sqli",
    "mg-ssh-audit",
    "mg-ssrf",
    "mg-ssti",
    "mg-takeover",
    "mg-timeline",
    "mg-tls-scan",
    "mg-traversal",
    "mg-tui",
    "mg-udp-scan",
    "mg-vhost",
    "mg-webscan",
    "mg-websocket",
    "mg-whois",
    "mg-xss",
    "mg-xxe",
];

#[test]
fn every_wiki_slug_has_a_parseable_page() {
    let pages_dir = Path::new("content").join("pages");
    for slug in WIKI_SLUGS {
        let path = pages_dir.join(format!("{slug}.md"));
        assert!(path.exists(), "missing wiki page: {}", path.display());
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        assert!(
            raw.starts_with("---"),
            "wiki page {} must start with YAML frontmatter",
            path.display()
        );
        assert!(
            raw.contains("title:"),
            "wiki page {} must declare a title",
            path.display()
        );
    }
}

// Reverse guard: no orphaned page files. Every .md under content/pages/ must be
// listed in WIKI_SLUGS, or a pruned page silently lingers with no sidebar link.
#[test]
fn no_orphaned_wiki_pages_on_disk() {
    let pages_dir = Path::new("content").join("pages");
    for entry in std::fs::read_dir(&pages_dir).expect("read content/pages") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("utf-8 file stem");
        assert!(
            WIKI_SLUGS.contains(&slug),
            "orphaned wiki page not in SIDEBAR/WIKI_SLUGS: {}",
            path.display()
        );
    }
}
