use chrono::NaiveDate;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use pulldown_cmark::{Parser, Options, html};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use crate::errors::SiteError;

// Deserialize is a serde trait — it lets gray_matter fill this struct from YAML.
// The field names must match the YAML keys in frontmatter exactly.
// If a key is missing from the file, deserialization returns an error.
#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    date: String,
    summary: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct BlogPost {
    pub slug: String,
    pub title: String,
    pub date: NaiveDate,
    pub summary: String,
    pub tags: Vec<String>,
    pub content_html: String,
}
impl BlogPost {
    pub fn from_file(path: &Path) -> Result<Self, SiteError> {
        // file_stem() strips the extension: "port-scanner-in-rust.md" → "port-scanner-in-rust"
        // to_str() converts OsStr (OS-native string) to &str (Rust string slice)
        // ok_or() converts Option to Result — returns Err if either step produces None
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or(SiteError::InvalidPath)?
            .to_string();
        
        // fs::read_to_string returns Result<String, io::Error>
        // ? converts io::Error to SiteError::Io via the #[from] impl in errors.rs
        // This is the ? operator's full power: read file, propagate any error, continue
        let raw = fs::read_to_string(path)?;

        // gray_matter splits the --- YAML block from the Markdown body.
        // parsed.data contains the YAML fields.
        // parsed.content contains everything after the closing ---.
        let matter = Matter::<YAML>::new();
        let parsed = matter.parse(&raw);

        let fm: Frontmatter = parsed
            .data
            .ok_or(SiteError::MissingFrontmatter(slug.clone()))?
            .deserialize()
            .map_err(|e| SiteError::FrontmatterParse(e.to_string()))?;
        
        // Parsing the date string into a typed NaiveDate means bad dates are caught
        // when the server starts (or when the post is first loaded), not when a user
        // requests the page. Fail early, fail loudly, fail at the boundary.    
        let date = NaiveDate::parse_from_str(&fm.date, "%Y-%m-%d")
            .map_err(|e| SiteError::DateParse(e.to_string()))?;

        // pulldown-cmark parses CommonMark Markdown and produces an event stream.
        // Options::all() enables extensions: tables, footnotes, strikethrough, task lists.
        // html::push_html writes the HTML output into our String buffer.
        let parser = Parser::new_ext(&parsed.content, Options::all());
        let mut content_html = String::new();
        html::push_html(&mut content_html, parser);

        Ok(BlogPost {
            slug,
            title: fm.title,
            date,
            summary: fm.summary,
            tags: fm.tags,
            content_html,
        })
    }

    pub fn load_all(dir: &Path) -> Result<Vec<Self>, SiteError> {
        let mut posts = fs::read_dir(dir)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()?.to_str()? == "md" {
                    Some(path)
                } 
                else {
                    None
                }
            })

            .map(|path| BlogPost::from_file(&path))
            .collect::<Result<Vec<_>, _>>()?;
        posts.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(posts)
    }

    pub fn find(dir: &Path, slug: &str) -> Result<Self, SiteError> {
        let path = dir.join(format!("{}.md", slug));

        if !path.exists() {
            return Err(SiteError::PostNotFound(slug.to_string()));
        }
        BlogPost::from_file(&path)
    }
}
