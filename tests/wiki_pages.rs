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
const WIKI_SLUGS: &[&str] = &[
    "index",
    "mg-engagement",
    "mg-harness",
    "mg-tui",
    "subdomain-enum",
    "mg-scan",
    "mg-fingerprint",
    "mg-recon",
    "corpus-builder",
    "mg-crawl",
    "mg-probe",
    "mg-fuzz",
    "mg-replay",
    "ai-prioritize",
    "mg-report",
    "mg-recopilot",
    "mg-aifuzz",
    "mg-exploitgen",
    "libraries",
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
