use chrono::NaiveDate;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use pulldown_cmark::{Parser, Options, html};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use crate::errors::SiteError;

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
            .ok_or(SiteError::MissingFrontmatter(slug.clone()))?
            .deserialize()
            .map_err(|e| SiteError::FrontmatterParse(e.to_string()))?;
        
        let date = NaiveDate::parse_from_str(&fm.date, "%Y-%m-%d")
            .map_err(|e| SiteError::DateParse(e.to_string()))?;

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

/*
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn test_load_all_posts() {
        let posts = BlogPost::load_all(Path::new("content/posts"))
            .expect("should load posts wihtout error");

        assert_eq!(posts.len(), 2);

        assert_eq!(posts[0].slug, "port-scanner-in-rust");
        assert_eq!(posts[1].slug, "memory-safety-c-vs-rust");

        assert!(posts[0].content_html.contains("<h2>"));
    }
}
*/

