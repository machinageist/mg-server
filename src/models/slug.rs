// Author:      machinageist
// Date:        2026-08-20
// Description: Central validation for URL slugs used to select Markdown files.
// Notes:       A slug is one lowercase ASCII path component. Validate before
//              joining it to any content root so encoded separators and parent
//              directory components cannot escape that root.

pub fn is_safe(slug: &str) -> bool {
    !slug.is_empty()
        && slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_site_slug_shape() {
        assert!(is_safe("security-review-2026"));
    }

    #[test]
    fn rejects_path_syntax_and_non_ascii() {
        for slug in [
            "",
            "../post",
            "post/name",
            "post\\name",
            ".",
            "UPPER",
            "café",
        ] {
            assert!(!is_safe(slug), "accepted unsafe slug {slug:?}");
        }
    }
}
