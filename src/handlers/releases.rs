// Author:      machinageist
// Date:        2026-05-17
// Description: Handler for /releases — scans static/releases/, computes
//              SHA-256 checksums for each artifact, and renders the page.
// Notes:       File I/O runs in spawn_blocking to keep the async executor free.
//              sha256_file streams in 64 KB chunks so large binaries never load
//              fully into memory.

use askama::Template;
use askama_axum::IntoResponse;
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use crate::errors::SiteError;

const RELEASES_DIR: &str = "static/releases";

// ── Release artifact descriptor ────────────────────────────────────────
pub struct ReleaseArtifact {
    pub filename: String,
    pub url: String,
    pub size_human: String,
    pub sha256: String,
}

// ── Template struct ────────────────────────────────────────────────────
#[derive(Template)]
#[template(path = "releases.html")]
pub struct ReleasesTemplate {
    pub artifacts: Vec<ReleaseArtifact>,
}

impl ReleasesTemplate {
    pub fn title(&self) -> &str {
        "Releases — machinageist"
    }
    pub fn description(&self) -> &str {
        "GeistScope source tarballs and compiled binaries."
    }
    pub fn section(&self) -> &str {
        "releases"
    }
}

// ── Handler — offloads blocking I/O to threadpool ─────────────────────
pub async fn list() -> Result<impl IntoResponse, SiteError> {
    let artifacts = tokio::task::spawn_blocking(scan_releases)
        .await
        .unwrap_or_else(|e| Err(std::io::Error::other(e.to_string())))?;
    Ok(ReleasesTemplate { artifacts })
}

// ── Scan releases directory and build artifact list ────────────────────
fn scan_releases() -> Result<Vec<ReleaseArtifact>, std::io::Error> {
    let dir = Path::new(RELEASES_DIR);
    let mut artifacts = Vec::new();

    if !dir.exists() {
        return Ok(artifacts);
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let filename = match path.file_name() {
            Some(n) => n.to_string_lossy().into_owned(),
            None => continue,
        };

        // Skip dotfiles (.gitkeep, .DS_Store, etc.)
        if filename.starts_with('.') {
            continue;
        }

        let metadata = fs::metadata(&path)?;
        let size_human = format_size(metadata.len());
        let sha256 = sha256_file(&path)?;
        let url = format!("/static/releases/{}", filename);

        artifacts.push(ReleaseArtifact {
            filename,
            url,
            size_human,
            sha256,
        });
    }

    // Alphabetical order — most recent version names sort last
    artifacts.sort_by(|a, b| a.filename.cmp(&b.filename));
    Ok(artifacts)
}

// ── Stream file through SHA-256 in 64 KB chunks ───────────────────────
fn sha256_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 65_536];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    Ok(hex::encode(hasher.finalize()))
}

// ── Human-readable file size ───────────────────────────────────────────
fn format_size(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.1} KB", bytes as f64 / 1_024.0)
    } else {
        format!("{} B", bytes)
    }
}
