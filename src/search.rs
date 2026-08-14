// Author:      machinageist
// Date:        2026-08-14
// Description: In-memory search over the site's published content. Builds a
//              corpus from the same sources the routes serve (posts, plus the
//              sidebar-allowlisted learn pages), ranks case-insensitive term
//              matches with field weighting, and returns escaped, highlighted
//              snippets. No external index, no JavaScript, no database.
// Notes:       The corpus is defined by ROUTABLE content only, which is what
//              makes a result unable to 404 and unable to surface a draft:
//              content/drafts/ is read by no route and by no branch here.
//              The corpus is rebuilt per request, matching the rest of the
//              site's read-fresh-from-disk posture — a new post is searchable
//              the moment its file lands, with no restart.

use crate::handlers::blog::POSTS_DIR;
use crate::handlers::wiki::{PAGES_DIR, sidebar_slugs};
use crate::models::page::Page;
use crate::models::post::BlogPost;
use chrono::NaiveDate;
use std::path::Path;

// Field weights — a title hit says more about relevance than a body hit
const WEIGHT_TITLE: i32 = 6;
const WEIGHT_TAGS: i32 = 4;
const WEIGHT_SUMMARY: i32 = 3;
const WEIGHT_BODY: i32 = 1;

// Extra body occurrences add a little, but a long page must not outrank a title
const BODY_REPEAT_BONUS: i32 = 1;
const BODY_REPEAT_CAP: i32 = 5;

// A query longer than this is a paste, not a search
const MAX_QUERY_CHARS: usize = 128;

// Bound on results returned — larger than today's corpus, defined for growth
const MAX_RESULTS: usize = 20;

// Roughly how many words of context a snippet carries around its first match
const SNIPPET_WORDS: usize = 30;

// The overview page is the /learn route itself, not a topic hit
const OVERVIEW_SLUG: &str = "index";

// Which surface a hit came from — decides the URL and the human label
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocKind {
    Post,
    Page,
}

impl DocKind {
    // Name the surface in a word, never a colour alone
    fn label(self) -> &'static str {
        match self {
            DocKind::Post => "Writing",
            DocKind::Page => "Learn",
        }
    }

    // Build the route a hit lives at
    fn url(self, slug: &str) -> String {
        match self {
            DocKind::Post => format!("/blog/{slug}"),
            DocKind::Page => format!("/learn/{slug}"),
        }
    }
}

// One indexed document, flattened from a BlogPost or a Page
struct SearchDoc {
    kind: DocKind,
    slug: String,
    title: String,
    summary: String,
    tags: Vec<String>,
    category: Option<String>,
    date: NaiveDate,
    body: String,
}

// One ranked result handed to the template
pub struct SearchResult {
    pub url: String,
    pub title: String,
    pub kind_label: &'static str,
    pub date: NaiveDate,
    pub category: Option<String>,
    // Pre-escaped text carrying <mark> spans — rendered with |safe, sound only
    // because highlight_snippet escapes every character before marking
    pub snippet_html: String,
    pub score: i32,
}

pub struct SearchIndex {
    docs: Vec<SearchDoc>,
}

impl SearchIndex {
    // Build the corpus from routable content only
    pub fn build() -> Self {
        let mut docs = Vec::new();

        // Every file under POSTS_DIR is servable at /blog/:slug
        if let Ok(posts) = BlogPost::load_all(Path::new(POSTS_DIR)) {
            docs.extend(posts.into_iter().map(|post| SearchDoc {
                kind: DocKind::Post,
                slug: post.slug,
                title: post.title,
                summary: post.summary,
                tags: post.tags,
                category: post.category,
                date: post.date,
                body: post.content_text,
            }));
        }

        // Only the SIDEBAR allowlist is servable at /learn/:slug
        for slug in sidebar_slugs() {
            if slug == OVERVIEW_SLUG {
                continue;
            }
            if let Ok(page) = Page::find(Path::new(PAGES_DIR), slug) {
                docs.push(SearchDoc {
                    kind: DocKind::Page,
                    slug: slug.to_string(),
                    title: page.title,
                    summary: page.summary,
                    tags: page.tags,
                    category: None,
                    date: page.date,
                    body: page.content_text,
                });
            }
        }

        SearchIndex { docs }
    }

    // Rank the corpus against a raw query string
    pub fn query(&self, raw: &str) -> Vec<SearchResult> {
        let terms = normalize(raw);
        if terms.is_empty() {
            return Vec::new();
        }

        let mut hits: Vec<SearchResult> = self
            .docs
            .iter()
            .filter_map(|doc| score(doc, &terms).map(|score| (doc, score)))
            .map(|(doc, score)| SearchResult {
                url: doc.kind.url(&doc.slug),
                title: doc.title.clone(),
                kind_label: doc.kind.label(),
                date: doc.date,
                category: doc.category.clone(),
                snippet_html: highlight_snippet(&doc.body, &terms),
                score,
            })
            .collect();

        // Score first, then newest, then slug — the last key only so that two
        // otherwise-identical documents never swap order between requests
        hits.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then(b.date.cmp(&a.date))
                .then(a.url.cmp(&b.url))
        });
        hits.truncate(MAX_RESULTS);
        hits
    }
}

// Reduce a raw query to lowercase search terms
fn normalize(raw: &str) -> Vec<String> {
    raw.chars()
        .take(MAX_QUERY_CHARS)
        .collect::<String>()
        .to_lowercase()
        .split_whitespace()
        .map(|term| {
            term.trim_matches(|c: char| !c.is_alphanumeric())
                .to_string()
        })
        .filter(|term| !term.is_empty())
        .collect()
}

// Score one document, or None when it does not match every term
//
// AND semantics across terms: a second word narrows the result set rather than
// widening it, which is what a reader typing two words expects.
fn score(doc: &SearchDoc, terms: &[String]) -> Option<i32> {
    let title = doc.title.to_lowercase();
    let summary = doc.summary.to_lowercase();
    let tags = doc.tags.join(" ").to_lowercase();
    let body = doc.body.to_lowercase();

    let mut total = 0;
    for term in terms {
        let mut best = 0;
        if title.contains(term) {
            best = best.max(WEIGHT_TITLE);
        }
        if tags.contains(term) {
            best = best.max(WEIGHT_TAGS);
        }
        if summary.contains(term) {
            best = best.max(WEIGHT_SUMMARY);
        }
        let occurrences = body.matches(term.as_str()).count() as i32;
        if occurrences > 0 {
            best = best.max(WEIGHT_BODY);
            total += (occurrences - 1).clamp(0, BODY_REPEAT_CAP) * BODY_REPEAT_BONUS;
        }
        // One term missing everywhere disqualifies the document
        if best == 0 {
            return None;
        }
        total += best;
    }
    Some(total)
}

// Build a highlighted, fully-escaped snippet around the first match
//
// Contract: the caller renders this with |safe. Every character of `text` and of
// each matched run is HTML-escaped BEFORE any <mark> is introduced, so the only
// live tags in the output are the ones emitted here. Escaping after marking
// would double-escape the marks; matching against escaped text would miscount
// offsets. Both orderings are wrong, and this is the one that is not.
fn highlight_snippet(text: &str, terms: &[String]) -> String {
    let lowered = text.to_lowercase();
    let first = terms
        .iter()
        .filter_map(|term| lowered.find(term.as_str()))
        .min()
        .unwrap_or(0);

    // Take a window of whole words centred on the first match, so a snippet never
    // begins or ends mid-word
    let mut words: Vec<(usize, &str)> = Vec::new();
    let mut cursor = 0;
    for word in text.split_whitespace() {
        let start = text[cursor..]
            .find(word)
            .map(|at| cursor + at)
            .unwrap_or(cursor);
        words.push((start, word));
        cursor = start + word.len();
    }

    let center = words
        .iter()
        .position(|(start, _)| *start >= first)
        .unwrap_or(0);
    let begin = center.saturating_sub(SNIPPET_WORDS / 2);
    let end = (begin + SNIPPET_WORDS).min(words.len());
    let Some((window_start, _)) = words.get(begin) else {
        return String::new();
    };
    let window_end = words
        .get(end.saturating_sub(1))
        .map(|(start, word)| start + word.len())
        .unwrap_or(text.len());
    let window = &text[*window_start..window_end];

    let mut out = String::new();
    if begin > 0 {
        out.push('…');
    }
    out.push_str(&mark_terms(window, terms));
    if end < words.len() {
        out.push('…');
    }
    out
}

// Split a run of text on term boundaries, escaping each piece before marking
fn mark_terms(window: &str, terms: &[String]) -> String {
    let lowered = window.to_lowercase();
    let mut out = String::with_capacity(window.len());
    let mut cursor = 0;

    while cursor < window.len() {
        // The earliest term match at or after the cursor wins
        let next = terms
            .iter()
            .filter_map(|term| lowered[cursor..].find(term.as_str()).map(|at| (at, term)))
            .min_by_key(|(at, _)| *at);

        match next {
            Some((offset, term)) => {
                let start = cursor + offset;
                let end = start + term.len();
                out.push_str(&escape_html(&window[cursor..start]));
                out.push_str("<mark>");
                out.push_str(&escape_html(&window[start..end]));
                out.push_str("</mark>");
                cursor = end;
            }
            None => {
                out.push_str(&escape_html(&window[cursor..]));
                break;
            }
        }
    }
    out
}

// Neutralize every character that could begin live markup
fn escape_html(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc(title: &str, summary: &str, tags: &[&str], body: &str) -> SearchDoc {
        SearchDoc {
            kind: DocKind::Page,
            slug: "test-page".to_string(),
            title: title.to_string(),
            summary: summary.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
            category: None,
            date: NaiveDate::from_ymd_opt(2026, 8, 14).unwrap(),
            body: body.to_string(),
        }
    }

    #[test]
    fn a_missing_term_disqualifies_the_document() {
        let d = doc("Subnetting", "Masks and CIDR", &["networking"], "prefix");
        assert!(score(&d, &["subnetting".into()]).is_some());
        // AND semantics: the second term narrows rather than widens
        assert!(score(&d, &["subnetting".into(), "kernel".into()]).is_none());
    }

    #[test]
    fn a_title_hit_outranks_a_body_hit() {
        let titled = doc("Subnetting", "", &[], "unrelated words");
        let bodied = doc("Something else", "", &[], "subnetting appears only here");
        let terms = vec!["subnetting".to_string()];
        assert!(score(&titled, &terms) > score(&bodied, &terms));
    }

    #[test]
    fn repeated_body_hits_help_but_cannot_beat_a_title() {
        let titled = doc("Subnetting", "", &[], "");
        let repeated = doc("Other", "", &[], &"subnetting ".repeat(50));
        let terms = vec!["subnetting".to_string()];
        assert!(
            score(&titled, &terms) >= score(&repeated, &terms),
            "a long page must not outrank a page that is actually about the term"
        );
    }

    #[test]
    fn an_empty_or_punctuation_only_query_matches_nothing() {
        assert!(normalize("").is_empty());
        assert!(normalize("   ").is_empty());
        assert!(normalize("!!! ???").is_empty());
        assert!(
            SearchIndex { docs: Vec::new() }
                .query("anything")
                .is_empty()
        );
    }

    #[test]
    fn a_query_is_length_bounded() {
        let terms = normalize(&"a".repeat(MAX_QUERY_CHARS * 4));
        assert_eq!(terms.len(), 1);
        assert_eq!(terms[0].len(), MAX_QUERY_CHARS);
    }

    #[test]
    fn snippets_escape_markup_before_they_highlight() {
        // The security-critical contract: the snippet is the one thing rendered
        // with |safe, so neither content nor query may become live markup
        let snippet = highlight_snippet(
            "a <script>alert(1)</script> tag in the body",
            &["script".to_string()],
        );
        assert!(
            !snippet.contains("<script"),
            "live script tag survived: {snippet}"
        );
        assert!(snippet.contains("&lt;"), "angle bracket was not escaped");
        assert!(
            snippet.contains("<mark>script</mark>"),
            "the match should still be highlighted: {snippet}"
        );
    }

    #[test]
    fn snippet_marks_every_occurrence_not_only_the_first() {
        let snippet = highlight_snippet("alpha beta alpha", &["alpha".to_string()]);
        assert_eq!(snippet.matches("<mark>").count(), 2);
    }

    #[test]
    fn the_real_corpus_is_searchable_and_excludes_drafts() {
        let index = SearchIndex::build();
        assert!(
            index.docs.len() >= 20,
            "expected the published corpus, got {}",
            index.docs.len()
        );

        // Every result URL must be a route the site actually serves
        for result in index.query("network") {
            assert!(
                result.url.starts_with("/blog/") || result.url.starts_with("/learn/"),
                "unroutable result: {}",
                result.url
            );
        }

        // content/drafts/ is read by no route, so it must be unreachable here.
        // "spectre" appears only in the drafts tree.
        assert!(
            index.query("spectre").is_empty(),
            "a draft leaked into the search corpus"
        );
    }

    #[test]
    fn a_body_only_term_is_findable() {
        // The difference between a real search and a menu of page titles
        let index = SearchIndex::build();
        let results = index.query("trilateration");
        assert!(
            !results.is_empty(),
            "a term appearing only in a page body should be findable"
        );
        assert!(results[0].url.starts_with("/learn/"));
    }
}
