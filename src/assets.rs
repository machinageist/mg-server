// Author:      machinageist
// Date:        2026-08-15
// Description: The cache-busting version stamped onto every static asset URL,
//              derived from the bytes actually being served.
// Notes:       This replaces a hand-written string that was set on 2026-07-19
//              and never touched again, while style.css changed in twenty-two
//              commits behind it. Every one of those changes was invisible to
//              any browser that had already cached the stylesheet — the site
//              rendered correctly from a cold cache and served month-old CSS to
//              everyone else, which is a defect you cannot see by looking at
//              your own screen.
//
//              One version for all of static/ rather than one per file. It
//              over-invalidates — a JS edit re-fetches the CSS too — but the
//              whole directory is small, and a single number that is always
//              right beats four that have to be maintained.

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

const STATIC_DIR: &str = "static";

// Computed once on first render, then reused for the life of the process
static ASSET_VERSION: LazyLock<String> = LazyLock::new(build_version);

// The value templates stamp into ?v=
pub fn version() -> &'static str {
    ASSET_VERSION.as_str()
}

// Hash every served file's path and contents into one short token
//
// Not a cryptographic digest and does not need to be — the only question it
// answers is "are these the same bytes as last time".
fn build_version() -> String {
    let mut hasher = DefaultHasher::new();

    for path in asset_files() {
        path.hash(&mut hasher);
        match fs::read(&path) {
            Ok(bytes) => bytes.hash(&mut hasher),
            // An unreadable file still changes the version when it appears or
            // disappears, which is the useful half of the signal
            Err(_) => "unreadable".hash(&mut hasher),
        }
    }

    format!("{:016x}", hasher.finish())
}

// Every file under static/, sorted so the version does not depend on the order
// the filesystem happens to hand them back
fn asset_files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect(Path::new(STATIC_DIR), &mut files);
    files.sort();
    files
}

// Walk a directory into the accumulator
fn collect(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, files);
        } else {
            files.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_version_covers_every_served_asset() {
        let files = asset_files();
        assert!(
            files.len() > 3,
            "only {} files found under {STATIC_DIR}/ — is the walk working?",
            files.len()
        );
        for asset in ["static/css/style.css", "static/js/main.js"] {
            assert!(
                files.iter().any(|path| path == Path::new(asset)),
                "{asset} is served but not hashed into the asset version"
            );
        }
    }

    // The guard that matters: the stamp has to follow the bytes. A hardcoded
    // string passes every other test on this site and still ships stale CSS to
    // everyone with a warm cache.
    #[test]
    fn the_version_changes_when_an_asset_changes() {
        let before = build_version();

        let scratch = Path::new(STATIC_DIR).join("__version_probe.tmp");
        fs::write(&scratch, "probe").expect("write probe");
        let during = build_version();
        fs::remove_file(&scratch).expect("remove probe");

        assert_ne!(
            before, during,
            "adding a file did not change the asset version, so browsers would \
             keep serving whatever they cached"
        );
        assert_eq!(
            before,
            build_version(),
            "the version is not stable for identical content"
        );
    }

    // Nothing in the templates may go back to a literal. This walks the source
    // rather than the render so it catches a new asset link too.
    #[test]
    fn no_template_hardcodes_an_asset_version() {
        for entry in fs::read_dir("templates").expect("templates dir").flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "html") {
                continue;
            }
            let source = fs::read_to_string(&path).expect("read template");
            for (index, _) in source.match_indices("?v=") {
                let after = &source[index + "?v=".len()..];
                assert!(
                    after.starts_with("{{"),
                    "{} stamps a literal asset version ({}...) — use \
                     {{{{ crate::assets::version() }}}} so it tracks the bytes",
                    path.display(),
                    after.chars().take(12).collect::<String>()
                );
            }
        }
    }
}
