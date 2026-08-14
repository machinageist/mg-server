// Author:      machinageist
// Date:        2026-08-14
// Description: Single Markdown → HTML conversion used by both Page and BlogPost.
//              Headings gain a stable `id` derived from their text so that
//              cross-links can address a section directly, e.g.
//              /learn/linux-streams#redirection-and-pipes.
// Notes:       pulldown-cmark's Tag::Heading already carries an `id` field, so
//              the ids are set on the event stream rather than spliced into the
//              rendered HTML. An explicit `{#id}` in the source always wins.

use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use std::collections::HashMap;

// Characters a heading slug may contain; everything else becomes a separator
const SLUG_SEPARATOR: char = '-';

// Fallback id for a heading whose text is entirely punctuation
const EMPTY_SLUG_FALLBACK: &str = "section";

// Convert a Markdown body to HTML, giving every heading a unique anchor id
pub fn to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all());
    let mut events: Vec<Event> = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();

    // Index into `events` of the heading currently being read, if any
    let mut open_heading: Option<usize> = None;
    let mut heading_text = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                open_heading = Some(events.len());
                heading_text.clear();
                events.push(event);
            }
            Event::End(TagEnd::Heading(level)) => {
                if let Some(index) = open_heading.take() {
                    // Only generate an id when the author did not write one
                    let anchor_id = match &mut events[index] {
                        Event::Start(Tag::Heading { id, .. }) => {
                            if id.is_none() {
                                *id = Some(unique_slug(&heading_text, &mut seen).into());
                            }
                            id.as_ref().map(|id| id.to_string())
                        }
                        _ => None,
                    };

                    // h1 is rendered from frontmatter by the template, so a
                    // permalink only makes sense from h2 down
                    if let Some(anchor_id) = anchor_id.filter(|_| level != HeadingLevel::H1) {
                        events.push(Event::Html(heading_anchor(&anchor_id).into()));
                    }
                }
                events.push(Event::End(TagEnd::Heading(level)));
            }
            // Heading text arrives as several events; inline code and emphasis
            // contribute their text, and everything else is ignored
            Event::Text(ref text) | Event::Code(ref text) if open_heading.is_some() => {
                heading_text.push_str(text);
                events.push(event);
            }
            _ => events.push(event),
        }
    }

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    html_output
}

// One heading in a document's outline, for building an on-page contents list
#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

// Extract the h2/h3 outline of a document
//
// Runs the same slug generation as to_html, so the ids here are the ids the
// rendered page actually carries. Deriving the contents list from the document
// rather than maintaining it by hand is what stops the two drifting apart.
pub fn outline(markdown: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut open: Option<(u8, Option<String>)> = None;
    let mut text = String::new();

    for event in Parser::new_ext(markdown, Options::all()) {
        match event {
            Event::Start(Tag::Heading { level, ref id, .. }) => {
                open = Some((level as u8, id.as_ref().map(|id| id.to_string())));
                text.clear();
            }
            Event::Text(ref chunk) | Event::Code(ref chunk) if open.is_some() => {
                text.push_str(chunk);
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some((level, explicit)) = open.take() {
                    // The slug counter must advance for every heading, not just
                    // the listed ones, or ids here stop matching the page
                    let generated = unique_slug(&text, &mut seen);
                    if (2..=3).contains(&level) {
                        headings.push(Heading {
                            level,
                            id: explicit.unwrap_or(generated),
                            text: text.trim().to_string(),
                        });
                    }
                }
            }
            _ => {}
        }
    }
    headings
}

// Flatten a Markdown body to plain text for search matching and snippets
//
// Matching against content_html would match inside tags and produce snippets
// containing markup, so the searchable form of a document is built here from the
// same parse the HTML comes from — one definition of what a document says.
pub fn to_text(markdown: &str) -> String {
    let mut text = String::with_capacity(markdown.len());
    for event in Parser::new_ext(markdown, Options::all()) {
        match event {
            Event::Text(chunk) | Event::Code(chunk) => {
                text.push_str(&chunk);
                text.push(' ');
            }
            // Block boundaries become spaces so words either side never fuse
            Event::End(_) | Event::HardBreak | Event::SoftBreak => text.push(' '),
            _ => {}
        }
    }
    // Collapse the runs of whitespace the folding above produces
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

// Render the permalink that trails a heading, letting a reader cite one section
fn heading_anchor(id: &str) -> String {
    format!(
        r##"<a class="heading-anchor" href="#{}" aria-label="Permalink to this section">#</a>"##,
        escape_attribute(id)
    )
}

// Escape the characters that would break out of a double-quoted HTML attribute
fn escape_attribute(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

// Build a slug from heading text, suffixing repeats so ids stay unique per document
fn unique_slug(text: &str, seen: &mut HashMap<String, usize>) -> String {
    let base = slugify(text);
    let count = seen.entry(base.clone()).or_insert(0);
    *count += 1;
    if *count == 1 {
        base
    } else {
        format!("{base}{SLUG_SEPARATOR}{count}")
    }
}

// Lowercase the text and reduce every run of non-alphanumerics to one separator
fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    let mut pending_separator = false;

    for ch in text.chars() {
        if ch.is_alphanumeric() {
            if pending_separator && !slug.is_empty() {
                slug.push(SLUG_SEPARATOR);
            }
            pending_separator = false;
            slug.extend(ch.to_lowercase());
        } else {
            pending_separator = true;
        }
    }

    // A heading of pure punctuation would otherwise produce an empty id
    if slug.is_empty() {
        slug.push_str(EMPTY_SLUG_FALLBACK);
    }
    slug
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn heading_gets_an_id_slugged_from_its_text() {
        let html = to_html("## Redirection and pipes\n");
        assert!(
            html.contains(r#"id="redirection-and-pipes""#),
            "expected a slugged id, got: {html}"
        );
    }

    #[test]
    fn punctuation_and_case_collapse_to_single_separators() {
        assert_eq!(slugify("The OSI model"), "the-osi-model");
        assert_eq!(slugify("Reading `ls -l`"), "reading-ls-l");
        assert_eq!(
            slugify("Suggested practice: work the streams"),
            "suggested-practice-work-the-streams"
        );
        assert_eq!(slugify("IPv4 / IPv6"), "ipv4-ipv6");
        assert_eq!(slugify("???"), "section");
    }

    #[test]
    fn repeated_headings_get_distinct_ids() {
        let html = to_html("## Overview\n\ntext\n\n## Overview\n");
        assert!(
            html.contains(r#"id="overview""#),
            "first heading keeps the base slug"
        );
        assert!(
            html.contains(r#"id="overview-2""#),
            "second heading is suffixed, got: {html}"
        );
    }

    #[test]
    fn an_explicit_id_in_the_source_wins() {
        let html = to_html("## Redirection and pipes {#pipes}\n");
        assert!(html.contains(r#"id="pipes""#), "explicit id should survive");
        assert!(
            !html.contains(r#"id="redirection-and-pipes""#),
            "generated id should not override the author's"
        );
    }

    #[test]
    fn inline_code_in_a_heading_contributes_to_the_slug() {
        let html = to_html("## Default permissions and `umask`\n");
        assert!(
            html.contains(r#"id="default-permissions-and-umask""#),
            "code spans should count toward the slug, got: {html}"
        );
    }

    #[test]
    fn headings_carry_a_permalink_pointing_at_their_own_id() {
        let html = to_html("## Redirection and pipes\n");
        assert!(
            html.contains(r##"href="#redirection-and-pipes""##),
            "permalink should target the heading's own id, got: {html}"
        );
        assert!(
            html.contains(r#"class="heading-anchor""#),
            "permalink needs the class the stylesheet hooks, got: {html}"
        );
        assert!(
            html.contains("aria-label=\"Permalink to this section\""),
            "a bare # is meaningless to a screen reader without a label"
        );
    }

    #[test]
    fn h1_gets_an_id_but_no_permalink() {
        // The template renders the page title from frontmatter, so a body h1 is
        // already the top of the document and has nothing to link back to.
        let html = to_html("# Page title\n");
        assert!(html.contains(r#"id="page-title""#), "h1 still gets an id");
        assert!(
            !html.contains("heading-anchor"),
            "h1 should not carry a permalink, got: {html}"
        );
    }

    #[test]
    fn body_content_is_unchanged_by_the_heading_pass() {
        let html = to_html("Some **bold** text and a [link](/learn/osi-model).\n");
        assert!(html.contains("<strong>bold</strong>"));
        assert!(html.contains(r#"href="/learn/osi-model""#));
    }

    #[test]
    fn the_outline_ids_match_the_ids_the_page_renders() {
        let source = "# Title\n\n## First section\n\ntext\n\n### A sub\n\n## First section\n";
        let html = to_html(source);
        let outline = outline(source);

        // h1 is excluded — it is the page title, not a section
        assert_eq!(outline.len(), 3);
        assert_eq!(outline[0].text, "First section");
        assert_eq!(outline[1].level, 3);

        // The duplicate must carry the same suffix the renderer gave it, or the
        // contents link would point at nothing
        assert_eq!(outline[2].id, "first-section-2");
        for heading in &outline {
            assert!(
                html.contains(&format!(r#"id="{}""#, heading.id)),
                "outline id {:?} is not in the rendered page",
                heading.id
            );
        }
    }

    #[test]
    fn an_explicit_heading_id_is_used_by_the_outline_too() {
        let outline = outline("## Redirection and pipes {#pipes}\n");
        assert_eq!(outline[0].id, "pipes");
    }

    #[test]
    fn plain_text_extraction_drops_markup_and_keeps_words() {
        let text = to_text("## A heading\n\nSome **bold** text and `code`.\n\n- a list item\n");
        assert_eq!(text, "A heading Some bold text and code . a list item");
        assert!(!text.contains('<'), "no markup may survive into the index");
    }

    #[test]
    fn plain_text_never_fuses_words_across_blocks() {
        // Without a separator at block boundaries "first" and "second" would run
        // together and stop matching either term
        let text = to_text("first\n\nsecond\n");
        assert_eq!(text, "first second");
    }

    #[test]
    fn tables_and_other_extensions_still_render() {
        let html = to_html("| a | b |\n|---|---|\n| 1 | 2 |\n");
        assert!(html.contains("<table>"), "Options::all() must stay enabled");
    }
}
