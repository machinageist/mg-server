// Author:      machinageist
// Date:        2026-08-14
// Description: Mechanical quality floor for everything under content/. Enforces
//              the frontmatter schema, the tag vocabulary, the page-authoring
//              contract, internal link resolution, and public claim boundaries.
// Notes:       This is the automated half of the output gate. It checks that the
//              scaffolding is present and the links resolve — deliberately not
//              whether the prose is any good, which is the /review-page command's
//              job. A lint that tried to grade pedagogy would either be useless
//              or would constrain the author's voice.
//              mg-server has no lib target, so this crate cannot import from the
//              binary. Checks needing the real Markdown renderer live in
//              src/handlers/wiki.rs instead, where models::markdown is in scope.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const PAGES_DIR: &str = "content/pages";
const POSTS_DIR: &str = "content/posts";
const LABS_DIR: &str = "content/labs";
const TEMPLATES_DIR: &str = "templates";

// The overview page introduces the wiki rather than teaching a topic, so it is
// exempt from the per-topic section contract
const CONTRACT_EXEMPT_PAGES: &[&str] = &["index"];

// Frontmatter keys every content file must declare
const REQUIRED_FRONTMATTER_KEYS: &[&str] = &["title", "date", "summary", "tags"];

// A summary is rendered into <meta name="description"> and og:description, where
// search engines and link unfurls truncate past roughly this length
const SUMMARY_MAX_CHARS: usize = 200;

// Sections the page-authoring contract requires on every topic page
const REQUIRED_PAGE_SECTIONS: &[&str] = &["## Related pages", "## Sources and further reading"];

// Certification slugs must never return as tags. A tag pill reads as a claim of
// credential rather than a citation of a textbook; criteria.md 1D scores a stale
// cert claim at zero, and these were removed from every learn page on 2026-08-14.
const BANNED_TAGS: &[&str] = &[
    "network-plus",
    "networkplus",
    "security-plus",
    "securityplus",
    "ccna",
    "rhcsa",
    "comptia",
    "linux-plus",
    "server-plus",
];

// Claim language criteria.md auto-fail rule 1 and Lens 1E forbid the site to use
// about itself. Checked against frontmatter and templates — not against page
// bodies, because a topic page explaining what penetration testing *is* should
// be free to name it. The distinction is describing versus claiming.
const BANNED_CLAIM_STRINGS: &[&str] = &[
    "CompTIA stack",
    "production-grade",
    "enterprise-grade",
    "penetration test",
    "pentest",
    "red team",
    "red-team",
    "offensive security",
];

// One parsed content file
struct ContentFile {
    slug: String,
    path: PathBuf,
    frontmatter: BTreeMap<String, String>,
    body: String,
}

// Collect and parse every .md file in a content directory
fn load_dir(dir: &str) -> Vec<ContentFile> {
    let mut files: Vec<ContentFile> = fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("read {dir}: {err}"))
        .filter_map(|entry| {
            let path = entry.expect("dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                return None;
            }
            Some(parse(&path))
        })
        .collect();
    files.sort_by(|a, b| a.slug.cmp(&b.slug));
    assert!(!files.is_empty(), "{dir} has no Markdown files");
    files
}

// Split a content file into its frontmatter map and its Markdown body
fn parse(path: &Path) -> ContentFile {
    let slug = path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("utf-8 file stem")
        .to_string();
    let raw =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()));

    assert!(
        raw.starts_with("---\n"),
        "{}: must open with a YAML frontmatter fence",
        path.display()
    );
    let (front, body) = raw[4..]
        .split_once("\n---\n")
        .unwrap_or_else(|| panic!("{}: frontmatter fence is never closed", path.display()));

    // Flat key: value pairs only — the schema has no nesting, so a full YAML
    // parser would be a dependency bought for nothing
    let mut frontmatter = BTreeMap::new();
    for line in front.lines() {
        if let Some((key, value)) = line.split_once(':') {
            if !key.starts_with(char::is_whitespace) {
                frontmatter.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }

    ContentFile {
        slug,
        path: path.to_path_buf(),
        frontmatter,
        body: body.to_string(),
    }
}

// Split a frontmatter list value — tags: [a, b, c] — into its members
fn tag_list(raw: &str) -> Vec<String> {
    raw.trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

// Every content file declares the full frontmatter schema, with a usable summary
#[test]
fn frontmatter_is_complete_and_within_budget() {
    for file in load_dir(PAGES_DIR)
        .into_iter()
        .chain(load_dir(POSTS_DIR))
        .chain(load_dir(LABS_DIR))
    {
        for key in REQUIRED_FRONTMATTER_KEYS {
            assert!(
                file.frontmatter.contains_key(*key),
                "{}: frontmatter is missing `{key}`",
                file.path.display()
            );
        }

        let summary = file.frontmatter["summary"].trim_matches('"');
        assert!(
            !summary.is_empty(),
            "{}: summary is empty — it is the page's <meta description>",
            file.path.display()
        );
        assert!(
            summary.chars().count() <= SUMMARY_MAX_CHARS,
            "{}: summary is {} chars, over the {SUMMARY_MAX_CHARS} budget — it will be \
             truncated in search results and link unfurls",
            file.path.display(),
            summary.chars().count()
        );

        let date = &file.frontmatter["date"];
        assert!(
            date.len() == 10 && date.split('-').count() == 3,
            "{}: date `{date}` is not YYYY-MM-DD — Page::from_file will fail to parse it",
            file.path.display()
        );
    }
}

// Tags stay lowercase-kebab, and no certification slug returns as a tag
#[test]
fn tags_use_the_agreed_vocabulary() {
    for file in load_dir(PAGES_DIR)
        .into_iter()
        .chain(load_dir(POSTS_DIR))
        .chain(load_dir(LABS_DIR))
    {
        let tags = tag_list(&file.frontmatter["tags"]);
        assert!(
            !tags.is_empty(),
            "{}: declares no tags",
            file.path.display()
        );

        for tag in tags {
            assert!(
                !BANNED_TAGS.contains(&tag.as_str()),
                "{}: `{tag}` is a certification tag. A tag pill reads as a credential claim \
                 rather than a citation — name the textbook in Sources instead \
                 (criteria.md 1D)",
                file.path.display()
            );
            assert!(
                tag.chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "{}: tag `{tag}` is not lowercase-kebab",
                file.path.display()
            );
        }
    }
}

// Every topic page carries the scaffolding the authoring contract requires
#[test]
fn every_topic_page_follows_the_authoring_contract() {
    for file in load_dir(PAGES_DIR) {
        if CONTRACT_EXEMPT_PAGES.contains(&file.slug.as_str()) {
            continue;
        }

        assert!(
            file.body.contains("## Overview"),
            "{}: no `## Overview` — every topic page opens by saying what it is",
            file.path.display()
        );
        for section in REQUIRED_PAGE_SECTIONS {
            assert!(
                file.body.contains(section),
                "{}: missing `{section}`",
                file.path.display()
            );
        }
        assert!(
            file.body.contains("## Suggested practice"),
            "{}: no `## Suggested practice` section. Understand → Practice → Evidence is \
             the wiki's contract (content/pages/index.md); a page that only explains is \
             half a page",
            file.path.display()
        );

        // The source textbook is named, not merely gestured at. This is what keeps
        // "edited from my study notes" from being an unattributed paraphrase.
        let sources = file
            .body
            .split("## Sources and further reading")
            .nth(1)
            .expect("section presence already asserted");
        assert!(
            sources.contains("Ian Neil") || sources.contains("Brian Ward"),
            "{}: Sources names no source textbook. Networking pages cite Ian Neil's \
             Network+ guide, Linux pages cite Brian Ward's How Linux Works",
            file.path.display()
        );
        assert!(
            sources.contains("https://"),
            "{}: Sources cites no primary document. The textbook is the source; an RFC, \
             standard, or man page is the check",
            file.path.display()
        );
    }
}

// Every internal /learn/ and /blog/ link points at content that exists
#[test]
fn internal_links_resolve() {
    let pages: Vec<String> = load_dir(PAGES_DIR).into_iter().map(|f| f.slug).collect();
    let posts: Vec<String> = load_dir(POSTS_DIR).into_iter().map(|f| f.slug).collect();

    let mut checked = 0;
    for file in load_dir(PAGES_DIR).into_iter().chain(load_dir(POSTS_DIR)) {
        for (prefix, known) in [("/learn/", &pages), ("/blog/", &posts)] {
            for (index, _) in file.body.match_indices(prefix) {
                let rest = &file.body[index + prefix.len()..];
                let slug: String = rest
                    .chars()
                    .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
                    .collect();
                // `/learn` on its own is the index route, not a page reference
                if slug.is_empty() {
                    continue;
                }
                assert!(
                    known.contains(&slug),
                    "{}: links to {prefix}{slug}, which does not exist",
                    file.path.display()
                );
                checked += 1;
            }
        }
    }
    assert!(checked > 0, "link checker matched nothing — it has broken");
}

// Site copy claims nothing criteria.md's auto-fail rules forbid
#[test]
fn site_copy_makes_no_forbidden_claim() {
    // Page bodies are exempt on purpose: a topic page describing offensive
    // security is teaching, not claiming. Frontmatter and templates are the
    // surfaces that speak for the site in its own voice.
    let mut surfaces: Vec<(String, String)> = Vec::new();

    for file in load_dir(PAGES_DIR)
        .into_iter()
        .chain(load_dir(POSTS_DIR))
        .chain(load_dir(LABS_DIR))
    {
        let front = file
            .frontmatter
            .values()
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");
        surfaces.push((file.path.display().to_string(), front));
    }

    // Lab procedures describe work on real infrastructure, so their bodies are
    // the site speaking in its own voice — unlike /learn, which teaches and
    // must stay free to name the things it explains
    for file in load_dir(LABS_DIR) {
        surfaces.push((file.path.display().to_string(), file.body));
    }

    for entry in fs::read_dir(TEMPLATES_DIR).expect("read templates") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("html") {
            continue;
        }
        let raw = fs::read_to_string(&path).expect("read template");
        // Askama comments {# … #} carry the decision record explaining why a
        // claim was removed, and naming it there is the opposite of claiming it
        let visible = strip_template_comments(&raw);
        surfaces.push((path.display().to_string(), visible));
    }

    for (name, text) in surfaces {
        let haystack = text.to_lowercase();
        for banned in BANNED_CLAIM_STRINGS {
            assert!(
                !haystack.contains(&banned.to_lowercase()),
                "{name}: contains {banned:?}. criteria.md auto-fail rule 1 and Lens 1E \
                 forbid this framing in the site's own voice"
            );
        }
    }
}

// Directories that describe real operated infrastructure rather than teach a
// concept. The distinction is the whole rule: /learn cannot explain RFC 1918
// without naming 10.0.0.0/8, and a writeup about my own lab has no reason to
// publish the address it actually uses.
const INFRASTRUCTURE_DIRS: &[&str] = &[POSTS_DIR, LABS_DIR];

// Site-wide sanitization standard, enforced rather than remembered.
//
// Nothing describing real infrastructure may publish the identifiers that
// locate it: no private address literals, no lab hostnames, no VM IDs. Zones
// are named (MGMT, LAB, SERVERS), roles are named ("the firewall VM"), and the
// addressing scheme is described rather than enumerated. A reader can still
// follow the procedure against their own lab — arguably more easily.
//
// The checks below deliberately match *shapes* rather than listing the real
// values. This repository is public, so a test that enumerated the hostnames
// and subnets it was protecting would disclose exactly what it exists to keep
// off the site.
#[test]
fn infrastructure_writing_publishes_no_private_addressing() {
    for dir in INFRASTRUCTURE_DIRS {
        for file in load_dir(dir) {
            for (line_no, line) in file.body.lines().enumerate() {
                if let Some(found) = first_private_address(line) {
                    panic!(
                        "{}:{}: publishes the private address {found:?}. Describe the zone \
                         or the scheme instead — see docs/agent-context/README.md \
                         §Sanitization.",
                        file.path.display(),
                        line_no + 1
                    );
                }
            }
        }
    }
}

// Lab hostnames and VM IDs locate a machine as precisely as an address does
#[test]
fn no_content_publishes_a_host_or_vm_identifier() {
    for dir in [PAGES_DIR, POSTS_DIR, LABS_DIR] {
        for file in load_dir(dir) {
            for (line_no, line) in file.body.lines().enumerate() {
                // `mg-server` is the public name of this site and its repository,
                // so it is the one `mg-` name that is not an internal hostname
                for word in line.split(|c: char| !(c.is_alphanumeric() || c == '-')) {
                    assert!(
                        !(word.starts_with("mg-") && word != "mg-server"),
                        "{}:{}: publishes the host name {word:?}",
                        file.path.display(),
                        line_no + 1
                    );
                }
                assert!(
                    !line.contains("VM 1")
                        && !line.contains("VM 2")
                        && !line.contains("VM 3")
                        && !line.contains("VMID"),
                    "{}:{}: publishes a VM identifier — name the role instead",
                    file.path.display(),
                    line_no + 1
                );
            }
        }
    }
}

// Public technical writing should teach the method without doubling as a
// current-state reconnaissance brief. These phrases previously exposed exact
// topology, service identifiers, or missing controls with little teaching value.
#[test]
fn public_copy_avoids_high_value_operational_recon() {
    const BANNED_DISCLOSURES: &[&str] = &[
        "mg-server.service",
        "three-node Proxmox cluster",
        "No automated monitoring or alerting",
        "No tested backup/restore",
        "The network is still flat",
        "transcribed from the runbooks I actually work from",
    ];

    let mut public_text = String::new();
    for dir in INFRASTRUCTURE_DIRS {
        for file in load_dir(dir) {
            public_text.push_str(&file.body);
            public_text.push('\n');
        }
    }
    public_text
        .push_str(&std::fs::read_to_string("templates/labs.html").expect("read labs template"));

    for disclosure in BANNED_DISCLOSURES {
        assert!(
            !public_text.contains(disclosure),
            "public copy still contains high-value operational detail {disclosure:?}"
        );
    }
}

// Find the first RFC 1918 address literal in a line, if any
fn first_private_address(line: &str) -> Option<String> {
    for token in line.split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '/')) {
        let octets: Vec<&str> = token
            .split('/')
            .next()
            .unwrap_or_default()
            .split('.')
            .collect();
        if octets.len() != 4 || octets.iter().any(|o| o.parse::<u8>().is_err()) {
            continue;
        }
        let first: u8 = octets[0].parse().unwrap_or(0);
        let second: u8 = octets[1].parse().unwrap_or(0);
        let private = first == 10
            || (first == 192 && second == 168)
            || (first == 172 && (16..=31).contains(&second));
        if private {
            return Some(token.to_string());
        }
    }
    None
}

// Remove {# … #} Askama comments so decision records are not read as page copy
fn strip_template_comments(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(start) = rest.find("{#") {
        out.push_str(&rest[..start]);
        match rest[start..].find("#}") {
            Some(end) => rest = &rest[start + end + 2..],
            None => return out,
        }
    }
    out.push_str(rest);
    out
}
