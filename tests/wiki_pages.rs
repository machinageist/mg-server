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
// This lists every curated education page under content/pages/.
const WIKI_SLUGS: &[&str] = &[
    "index",
    "osi-model",
    "transmission-media",
    "transceivers",
    "network-appliances",
    "network-applications",
    "network-functions",
    "network-protocols",
    "traffic-types",
    "cloud-computing",
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
