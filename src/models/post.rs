// Author:      machinageist
// Date:        2026-04
// Description: Defines BlogPost — the data model for a single blog post.
//              from_file() reads a .md file, splits the YAML frontmatter block
//              from the Markdown body using gray_matter, deserializes metadata
//              into a typed Frontmatter struct, parses the date string into a
//              NaiveDate, and converts the Markdown body to HTML via models::markdown.
//              load_all() scans a directory, calls from_file() on every .md file,
//              and returns the results sorted newest-first by date.
//              find() locates one post by slug and delegates to from_file().
//
// Notes:       Frontmatter struct uses serde::Deserialize — field names must
//              match YAML keys exactly. Missing keys produce a parse error caught
//              at load time, not when a user requests the page.
//              Date is parsed from String to NaiveDate explicitly so malformed
//              dates are caught early and clearly, not silently ignored.
//              content_html contains trusted output from pulldown-cmark — safe
//              to render with |safe in templates. Never use |safe on user input.
//              The slug is derived from the filename stem — the URL and the file
//              name are intentionally kept in sync.

use crate::errors::SiteError;
use crate::models::markdown;
use chrono::NaiveDate;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use std::fs;
use std::path::Path;

// -----------------------------------------------------------------------
// Frontmatter schema — must match YAML keys in every .md file exactly
// -----------------------------------------------------------------------

// Deserialize fills this struct from the --- YAML block via gray_matter
#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    date: String, // parsed into NaiveDate below — kept as String here for flexibility
    summary: String,
    tags: Vec<String>,
    // Optional pillar label used to group the blog list. Must stay Option so
    // older posts without the field keep loading — a missing required field
    // breaks parsing of every post.
    #[serde(default)]
    category: Option<String>,
}

// -----------------------------------------------------------------------
// Public data model — passed to templates and used throughout the crate
// -----------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct BlogPost {
    pub slug: String,    // URL identifier derived from filename stem
    pub title: String,   // from frontmatter
    pub date: NaiveDate, // parsed from frontmatter date string
    pub summary: String, // from frontmatter — used in list view
    pub tags: Vec<String>,
    pub category: Option<String>, // optional pillar label — used to group the list view
    pub content_html: String,     // Markdown body converted to HTML — empty in list view
    // Body as plain text — what search matches on and snippets are cut from
    pub content_text: String,
}

impl BlogPost {
    // -----------------------------------------------------------------------
    // Load and parse a single post from a .md file path
    // -----------------------------------------------------------------------

    // Read file, parse frontmatter, convert Markdown to HTML, return BlogPost
    pub fn from_file(path: &Path) -> Result<Self, SiteError> {
        // Derive URL slug from filename — "port-scanner-in-rust.md" → "port-scanner-in-rust"
        let slug = path
            .file_stem() // strip extension
            .and_then(|s| s.to_str()) // OsStr → &str
            .ok_or(SiteError::InvalidPath)? // None → Err
            .to_string();

        // Read entire file into memory — ? converts io::Error to SiteError::Io
        let raw = fs::read_to_string(path)?;

        // Split YAML frontmatter block from Markdown body
        let matter = Matter::<YAML>::new();
        let parsed = matter.parse(&raw);

        // Deserialize frontmatter fields into typed struct
        let fm: Frontmatter = parsed
            .data
            .ok_or_else(|| SiteError::MissingFrontmatter(slug.clone()))?
            .deserialize()
            .map_err(|e| SiteError::FrontmatterParse(e.to_string()))?;

        // Parse date string "YYYY-MM-DD" into NaiveDate — catches malformed dates at load time
        let date = NaiveDate::parse_from_str(&fm.date, "%Y-%m-%d")
            .map_err(|e| SiteError::DateParse(e.to_string()))?;

        // Convert Markdown body to HTML — shared with Page, and adds heading anchor ids
        let content_html = markdown::to_html(&parsed.content);
        let content_text = markdown::to_text(&parsed.content);

        Ok(BlogPost {
            slug,
            title: fm.title,
            date,
            summary: fm.summary,
            tags: fm.tags,
            category: fm.category,
            content_html,
            content_text,
        })
    }

    // -----------------------------------------------------------------------
    // Load all posts from a directory, sorted newest-first
    // -----------------------------------------------------------------------

    // Scan directory for .md files, parse each, sort by date descending
    pub fn load_all(dir: &Path) -> Result<Vec<Self>, SiteError> {
        // Collect file paths for all .md files in directory
        let mut posts = fs::read_dir(dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                // Skip non-.md files silently
                if path.extension()?.to_str()? == "md" {
                    Some(path)
                } else {
                    None
                }
            })
            // Parse each file — collect propagates the first Err if any file fails
            .map(|path| BlogPost::from_file(&path))
            .collect::<Result<Vec<_>, _>>()?;

        // Sort newest-first so list view shows most recent posts at the top
        posts.sort_by_key(|p| std::cmp::Reverse(p.date));
        Ok(posts)
    }

    // -----------------------------------------------------------------------
    // Find a single post by URL slug
    // -----------------------------------------------------------------------

    // Build expected file path from slug, return 404 error if file absent
    pub fn find(dir: &Path, slug: &str) -> Result<Self, SiteError> {
        let path = dir.join(format!("{}.md", slug));
        // Return typed 404 error rather than letting fs::read fail with a generic io::Error
        if !path.exists() {
            return Err(SiteError::PostNotFound(slug.to_string()));
        }
        BlogPost::from_file(&path)
    }
}
