// Author:      machinageist
// Date:        2026-05
// Description: Defines Page — a single Markdown-backed standalone page.
//              from_file() reads a .md file, splits the YAML frontmatter block
//              from the Markdown body using gray_matter, deserializes metadata
//              into a typed Frontmatter struct, parses the date string into a
//              NaiveDate, and converts the Markdown body to HTML with pulldown-cmark.
//              find() locates one page by slug and delegates to from_file().

use crate::errors::SiteError;
use chrono::NaiveDate;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    summary: String,
    tags: Vec<String>,
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
    pub content_html: String,
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

        let md_parser = Parser::new_ext(&parsed.content, Options::all());
        let mut content_html = String::new();
        html::push_html(&mut content_html, md_parser);

        Ok(Page {
            slug,
            title: fm.title,
            date,
            summary: fm.summary,
            tags: fm.tags,
            content_html,
        })
    }

    pub fn find(dir: &Path, slug: &str) -> Result<Self, SiteError> {
        let path = dir.join(format!("{}.md", slug));
        if !path.exists() {
            return Err(SiteError::PageNotFound(slug.to_string()));
        }
        Page::from_file(&path)
    }
}
