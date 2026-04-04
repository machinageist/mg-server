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

pub async fn post(Path(slug): Path<String>) -> Result<impl IntoResponse, SiteError> {
    if slug.contains('/') || slug.contains('\\') || slug.contains("..") {
        return Err(SiteError::PostNotFound(slug));
    }

    let posts_dir = PathBuf::from("content/posts");
    let post = BlogPost::find(&posts_dir, &slug)?;

    Ok(BlogPostTemplate { post })
}
