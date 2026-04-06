use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Path;
use std::path::PathBuf;
use crate::models::post::BlogPost;
use crate::errors::SiteError;

#[derive(Template)]
#[template(path =  "blog_list.html")]
pub struct BlogListTemplate {
    pub posts: Vec<BlogPost>,
}

impl BlogListTemplate {
    pub fn title(&self) -> &str {
        "writing"
    }
}

pub async fn list() -> Result<impl IntoResponse, SiteError> {
    let posts_dir = PathBuf::from("content/posts");

    let posts = BlogPost::load_all(&posts_dir)?;

    Ok(BlogListTemplate { posts })
}

#[derive(Template)]
#[template(path = "blog_post.html")]
pub struct BlogPostTemplate {
    pub post: BlogPost,
}

impl BlogPostTemplate {
    pub fn title(&self) -> &str {
        &self.post.title
    }
}

// Path<String> is an Axum extractor.
// Axum reads the :slug segment from the URL and injects it as a String.
// If the URL has no :slug, Axum returns 400 Bad Request before your handler runs.
// Destructuring syntax: Path(slug) binds the inner String directly to `slug`.
pub async fn post(Path(slug): Path<String>) -> Result<impl IntoResponse, SiteError> {
    // Slug validation before any filesystem access.
    // The slug will be used to construct a file path — we must ensure it cannot
    // escape the posts directory. This is path traversal prevention.
    //
    // Attack being prevented:
    //   GET /blog/../../etc/passwd
    //   → without this check, slug = "../../etc/passwd"
    //   → path = "content/posts/../../etc/passwd"
    //   → reads /etc/passwd
    //
    // Two layers of defense:
    //   1. This explicit check rejects anything containing / \ or ..
    //   2. BlogPost::find() constructs the path itself — user never controls the directory
    if slug.contains('/') || slug.contains('\\') || slug.contains("..") {
        return Err(SiteError::PostNotFound(slug));
    }

    let posts_dir = PathBuf::from("content/posts");
    // ? operator: if find() returns Err, this function returns that Err immediately.
    // The error flows to IntoResponse which renders the appropriate HTTP response.
    // No unwrap(), no panic — every failure becomes a graceful HTTP error.
    let post = BlogPost::find(&posts_dir, &slug)?;

    Ok(BlogPostTemplate { post })
}
