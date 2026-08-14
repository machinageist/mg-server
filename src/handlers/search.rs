// Author:      machinageist
// Date:        2026-08-14
// Description: Handler for /search. Reads the `q` query parameter, ranks the
//              published corpus with search::SearchIndex, and renders the
//              results server-side.
// Notes:       No JavaScript is involved at any point: the form is a plain GET,
//              the results are HTML, and every result is a real URL. A missing
//              or blank `q` is the idle state, not an error — nothing here can
//              return a non-200 for a badly formed search.
//              `q` is a search term, never a path, so it reaches no filesystem
//              call and needs no traversal check. Its hardening is length
//              bounding (in search::normalize) and output escaping.

use crate::search::{SearchIndex, SearchResult};
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Query;
use serde::Deserialize;

// The query string of a search request — absent `q` deserialises to None
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    // Echoed back into the input so a refined search starts from the last one
    pub query: String,
    pub results: Vec<SearchResult>,
    // True once a real search ran, which separates "no results" from "not asked"
    pub searched: bool,
}

impl SearchTemplate {
    // Supply page title to base.html <title> slot
    pub fn title(&self) -> String {
        if self.searched {
            format!("Search: {} — machinageist", self.query)
        } else {
            "Search — machinageist".to_string()
        }
    }

    pub fn description(&self) -> &str {
        "Search the writing and education-wiki pages on machinageist.dev."
    }

    pub fn section(&self) -> &str {
        "search"
    }

    // Phrase the result count for the count line
    pub fn result_summary(&self) -> String {
        match self.results.len() {
            0 => "No results".to_string(),
            1 => "1 result".to_string(),
            n => format!("{n} results"),
        }
    }
}

// Build the search view for one query
//
// Split from the handler so tests can render it directly — a handler returning
// `impl IntoResponse` hides the template behind an opaque type.
fn search_view(raw_query: Option<String>) -> SearchTemplate {
    let query = raw_query.unwrap_or_default();
    let trimmed = query.trim();
    let searched = !trimmed.is_empty();

    // The corpus is rebuilt per request so a newly published file is findable
    // immediately, matching the rest of the site's read-fresh-from-disk posture
    let results = if searched {
        SearchIndex::build().query(trimmed)
    } else {
        Vec::new()
    };

    SearchTemplate {
        query: trimmed.to_string(),
        results,
        searched,
    }
}

// Rank the published corpus against ?q= and render the results
pub async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    search_view(params.q)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Render the page for one query, as the route would
    fn render(q: Option<&str>) -> String {
        search_view(q.map(|s| s.to_string()))
            .render()
            .expect("search template renders")
    }

    #[test]
    fn an_absent_query_renders_the_idle_state_not_an_error() {
        let html = render(None);
        assert!(html.contains("<form"), "the form is always offered");
        assert!(
            !html.contains("No results"),
            "not having asked is not the same as having found nothing"
        );
    }

    #[test]
    fn a_blank_query_is_idle_rather_than_a_failed_search() {
        let html = render(Some("   "));
        assert!(!html.contains("No results"));
    }

    #[test]
    fn a_matching_query_lists_real_routes() {
        let html = render(Some("subnet"));
        assert!(html.contains("result"), "a count line should render");
        assert!(
            html.contains("href=\"/learn/") || html.contains("href=\"/blog/"),
            "results must be real links: {html}"
        );
        assert!(html.contains("<mark>"), "matches should be highlighted");
    }

    #[test]
    fn a_query_matching_nothing_renders_a_designed_empty_state() {
        let html = render(Some("zzzzznotathing"));
        assert!(html.contains("No results"));
        // The empty state offers a way onward rather than a dead end
        assert!(html.contains("/blog"));
        assert!(html.contains("/learn"));
    }

    #[test]
    fn the_query_is_echoed_escaped_never_as_markup() {
        let html = render(Some("<script>alert(1)</script>"));
        assert!(
            !html.contains("<script>alert(1)</script>"),
            "the query must never round-trip as live markup"
        );
        assert!(html.contains("&lt;script&gt;"), "expected an escaped echo");
    }

    #[test]
    fn search_needs_no_javascript() {
        // Auto-fail rule 3. Strip every script tag and the page must still work.
        let html = render(Some("linux"));
        let mut stripped = String::new();
        let mut rest = html.as_str();
        while let Some(start) = rest.find("<script") {
            stripped.push_str(&rest[..start]);
            match rest[start..].find("</script>") {
                Some(end) => rest = &rest[start + end + "</script>".len()..],
                None => break,
            }
        }
        stripped.push_str(rest);

        assert!(stripped.contains("<form"), "the form survives without JS");
        assert!(
            stripped.contains("method=\"get\""),
            "submission is a plain GET"
        );
        assert!(
            stripped.contains("href=\"/learn/") || stripped.contains("href=\"/blog/"),
            "results survive without JS"
        );
    }

    // Drive a request through the real router and return status plus body
    async fn get(path: &str) -> (axum::http::StatusCode, String) {
        use tower::ServiceExt;
        let app = crate::router::build(crate::state::AppState::new());
        let response = app
            .oneshot(
                axum::http::Request::get(path)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        (status, String::from_utf8(bytes.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn the_route_is_wired_and_answers_every_query_shape() {
        // Idle, matching, empty-result, and hostile all return 200 — a search
        // that cannot be satisfied is a result, not an error
        for path in [
            "/search",
            "/search?q=",
            "/search?q=subnet",
            "/search?q=zzzzznotathing",
            "/search?q=%3Cscript%3Ealert(1)%3C%2Fscript%3E",
            "/search?q=%20%20%20",
        ] {
            let (status, _) = get(path).await;
            assert_eq!(status, axum::http::StatusCode::OK, "{path} must be 200");
        }
    }

    #[tokio::test]
    async fn every_result_link_the_page_offers_actually_resolves() {
        // The corpus is built from routable content, so this should hold by
        // construction — this is the test that proves the construction is right.
        let (_, body) = get("/search?q=network").await;

        let mut checked = 0;
        for marker in ["href=\"/learn/", "href=\"/blog/"] {
            for (index, _) in body.match_indices(marker) {
                let href: String = body[index + "href=\"".len()..]
                    .chars()
                    .take_while(|c| *c != '"')
                    .collect();
                let (status, _) = get(&href).await;
                assert_eq!(status, axum::http::StatusCode::OK, "{href} must resolve");
                checked += 1;
            }
        }
        assert!(checked > 0, "the query returned no links to check");
    }

    #[tokio::test]
    async fn the_search_page_is_reachable_from_every_page() {
        let (_, home) = get("/").await;
        assert!(
            home.contains("href=\"/search\""),
            "search must be offered in the site shell, not only at its own URL"
        );
    }
}
