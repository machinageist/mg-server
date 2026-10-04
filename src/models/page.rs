// Author:      machinageist
// Date:        2026-05
// Description: Defines Page — a single Markdown-backed standalone page.
//              from_file() reads a .md file, splits the YAML frontmatter block
//              from the Markdown body using gray_matter, deserializes metadata
//              into a typed Frontmatter struct, parses the date string into a
//              NaiveDate, and converts the Markdown body to HTML via models::markdown.
//              find() locates one page by slug and delegates to from_file().
//
// Notes:       draft: true in the frontmatter marks a page that is on disk but
//              not published. Page only reports the flag. What a draft is kept
//              out of is decided by handlers::wiki.

use crate::errors::SiteError;
use crate::models::markdown::{self, Heading};
use chrono::NaiveDate;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    summary: String,
    tags: Vec<String>,
    // Absent on every published page, so it defaults to false
    #[serde(default)]
    draft: bool,
}

// The one frontmatter field is_draft() needs, so it can skip the rest
#[derive(Debug, Deserialize)]
struct DraftFlag {
    #[serde(default)]
    draft: bool,
}

#[derive(Debug, Clone)]
pub struct Page {
    #[allow(dead_code)]
    pub slug: String,
    pub title: String,
    pub date: NaiveDate,
    #[allow(dead_code)]
    pub summary: String,
    pub tags: Vec<String>,
    // True while the page is being written and is not yet published
    #[allow(dead_code)]
    pub draft: bool,
    pub content_html: String,
    // Body as plain text — what search matches on and snippets are cut from
    pub content_text: String,
    // The h2/h3 outline, for pages that render an on-page contents list
    pub outline: Vec<Heading>,
}

impl Page {
    pub fn from_file(path: &Path) -> Result<Self, SiteError> {
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(SiteError::InvalidPath)?
            .to_string();

        let raw = fs::read_to_string(path)?;
        let matter = Matter::<YAML>::new();
        let parsed = matter.parse(&raw);
        let fm: Frontmatter = parsed
            .data
            .ok_or_else(|| SiteError::MissingFrontmatter(slug.clone()))?
            .deserialize()
            .map_err(|e| SiteError::FrontmatterParse(e.to_string()))?;
        let date = NaiveDate::parse_from_str(&fm.date, "%Y-%m-%d")
            .map_err(|e| SiteError::DateParse(e.to_string()))?;

        let content_html = markdown::to_html(&parsed.content);
        let content_text = markdown::to_text(&parsed.content);
        let outline = markdown::outline(&parsed.content);

        Ok(Page {
            slug,
            title: fm.title,
            date,
            summary: fm.summary,
            tags: fm.tags,
            draft: fm.draft,
            content_html,
            content_text,
            outline,
        })
    }

    pub fn find(dir: &Path, slug: &str) -> Result<Self, SiteError> {
        if !crate::models::slug::is_safe(slug) {
            return Err(SiteError::PageNotFound(slug.to_string()));
        }
        let path = dir.join(format!("{}.md", slug));
        if !path.exists() {
            return Err(SiteError::PageNotFound(slug.to_string()));
        }
        Page::from_file(&path)
    }

    // Read a page's h2/h3 outline without rendering its HTML. A page that is
    // missing or unreadable has no outline
    pub fn outline_of(dir: &Path, slug: &str) -> Vec<Heading> {
        if !crate::models::slug::is_safe(slug) {
            return Vec::new();
        }
        let Ok(raw) = fs::read_to_string(dir.join(format!("{}.md", slug))) else {
            return Vec::new();
        };
        markdown::outline(&Matter::<YAML>::new().parse(&raw).content)
    }

    // Report whether a page is marked draft, reading only its frontmatter.
    // A page that is missing or unreadable is not a draft, so the caller's
    // normal not-found path handles it
    pub fn is_draft(dir: &Path, slug: &str) -> bool {
        if !crate::models::slug::is_safe(slug) {
            return false;
        }
        let Ok(raw) = fs::read_to_string(dir.join(format!("{}.md", slug))) else {
            return false;
        };
        Matter::<YAML>::new()
            .parse(&raw)
            .data
            .and_then(|data| data.deserialize::<DraftFlag>().ok())
            .is_some_and(|flag| flag.draft)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_rejects_a_path_like_slug() {
        let error = Page::find(
            Path::new("content/pages"),
            "../posts/hosting-machinageist-dev",
        )
        .expect_err("a path-like slug must not leave the page directory");

        assert!(matches!(error, SiteError::PageNotFound(_)));
    }

    // Write one page into a scratch directory and return the directory
    fn scratch_page(name: &str, frontmatter_extra: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("mg-server-{name}-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("create scratch dir");
        let body = format!(
            "---\ntitle: \"Scratch\"\ndate: 2026-10-04\nsummary: \"A scratch page.\"\n\
             tags: [education]\n{frontmatter_extra}---\n\n## Overview\n\nText.\n"
        );
        fs::write(dir.join("scratch.md"), body).expect("write scratch page");
        dir
    }

    #[test]
    fn a_page_is_a_draft_only_when_its_frontmatter_says_so() {
        let draft = scratch_page("draft", "draft: true\n");
        assert!(Page::is_draft(&draft, "scratch"));
        assert!(Page::find(&draft, "scratch").expect("parses").draft);

        let published = scratch_page("published", "");
        assert!(!Page::is_draft(&published, "scratch"));
        assert!(!Page::find(&published, "scratch").expect("parses").draft);

        assert!(!Page::is_draft(&published, "no-such-page"));
        assert!(!Page::is_draft(&published, "../scratch"));

        fs::remove_dir_all(draft).ok();
        fs::remove_dir_all(published).ok();
    }
}
