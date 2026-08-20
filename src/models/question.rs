// Author:      machinageist
// Date:        2026-08-14
// Description: The study question bank — multiple-choice questions loaded from
//              frontmatter-only Markdown under content/study/, one file per
//              /learn topic.
// Notes:       Every question cites the wiki page and heading that teaches its
//              answer. That citation is not decoration: a test resolves each
//              one against the real rendered heading ids, and a question that
//              cannot point at material on this site does not ship. It is what
//              separates this from a generic question bank.
//
//              The explanation is the product. A score teaches nothing; the
//              reason a distractor is wrong, with a link to the page that
//              explains it, is the whole reason the surface exists.

use crate::errors::SiteError;
use crate::models::markdown;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::Deserialize;
use std::fs;
use std::path::Path;

pub const STUDY_DIR: &str = "content/study";

// Where a question's answer is taught
#[derive(Debug, Clone, Deserialize)]
pub struct LearnCitation {
    pub slug: String,
    pub anchor: String,
}

impl LearnCitation {
    // The deep link a reader follows to see why the answer is what it is
    pub fn href(&self) -> String {
        format!("/learn/{}#{}", self.slug, self.anchor)
    }
}

// One multiple-choice question
#[derive(Debug, Clone, Deserialize)]
pub struct Question {
    pub stem: String,
    pub options: Vec<String>,
    // Zero-based index into options
    pub answer: usize,
    pub explanation: String,
    pub learn: LearnCitation,
}

impl Question {
    // Stems, options, and explanations carry commands, flags, and addresses in
    // inline code spans. Rendered so they reach the reader as monospace rather
    // than as literal backticks.
    pub fn stem_html(&self) -> String {
        markdown::to_inline_html(&self.stem)
    }

    pub fn explanation_html(&self) -> String {
        markdown::to_inline_html(&self.explanation)
    }

    // Every option rendered, so a template can iterate them directly rather
    // than indexing back into the raw strings
    pub fn options_html(&self) -> Vec<String> {
        self.options
            .iter()
            .map(|option| markdown::to_inline_html(option))
            .collect()
    }

    // The correct option, rendered. A dedicated accessor because a template
    // cannot pass self.answer into option_html without Askama taking it by
    // reference.
    pub fn answer_html(&self) -> String {
        self.option_html(self.answer)
    }

    pub fn option_html(&self, index: usize) -> String {
        self.options
            .get(index)
            .map(|option| markdown::to_inline_html(option))
            .unwrap_or_default()
    }
}

// One topic's worth of questions
#[derive(Debug, Clone, Deserialize)]
pub struct QuestionSet {
    pub topic: String,
    pub questions: Vec<Question>,
    // Filled from the filename, not the frontmatter — the file name is the URL
    #[serde(skip)]
    pub slug: String,
}

// Load one topic's question set by slug
pub fn load(dir: &Path, slug: &str) -> Result<QuestionSet, SiteError> {
    if !crate::models::slug::is_safe(slug) {
        return Err(SiteError::PageNotFound(slug.to_string()));
    }
    let path = dir.join(format!("{slug}.md"));
    if !path.exists() {
        return Err(SiteError::PageNotFound(slug.to_string()));
    }
    let raw = fs::read_to_string(&path)?;
    let parsed = Matter::<YAML>::new().parse(&raw);
    let mut set: QuestionSet = parsed
        .data
        .ok_or_else(|| SiteError::MissingFrontmatter(slug.to_string()))?
        .deserialize()
        .map_err(|err| SiteError::FrontmatterParse(err.to_string()))?;
    set.slug = slug.to_string();
    Ok(set)
}

// Every topic that has questions, sorted for a stable index
pub fn all(dir: &Path) -> Vec<QuestionSet> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut sets: Vec<QuestionSet> = entries
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                return None;
            }
            let slug = path.file_stem().and_then(|s| s.to_str())?;
            load(dir, slug).ok()
        })
        .collect();
    sets.sort_by(|a, b| a.topic.cmp(&b.topic));
    sets
}

// -----------------------------------------------------------------------
// Grading — stateless, and nothing is recorded
// -----------------------------------------------------------------------

// What a reader answered for one question, and whether it was right
pub struct GradedAnswer {
    pub question: Question,
    // None when the question was left blank, which is not the same as wrong
    pub chosen: Option<usize>,
}

impl GradedAnswer {
    pub fn correct(&self) -> bool {
        self.chosen == Some(self.question.answer)
    }

    pub fn unanswered(&self) -> bool {
        self.chosen.is_none()
    }

    // Rendered forms for the result page
    pub fn chosen_option_html(&self) -> String {
        self.chosen
            .map(|index| self.question.option_html(index))
            .unwrap_or_default()
    }

    pub fn correct_option_html(&self) -> String {
        self.question.option_html(self.question.answer)
    }

    // Stated in a word so the result never depends on colour alone
    pub fn verdict(&self) -> &'static str {
        if self.unanswered() {
            "skipped"
        } else if self.correct() {
            "correct"
        } else {
            "incorrect"
        }
    }

    pub fn verdict_class(&self) -> &'static str {
        if self.unanswered() {
            "is-skipped"
        } else if self.correct() {
            "is-correct"
        } else {
            "is-wrong"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dir() -> PathBuf {
        PathBuf::from(STUDY_DIR)
    }

    #[test]
    fn traversal_slug_is_rejected_before_any_file_is_read() {
        let error = load(&dir(), "../posts/hosting-machinageist-dev")
            .expect_err("a path-like slug must not reach an out-of-directory Markdown file");

        assert!(matches!(error, SiteError::PageNotFound(_)));
    }

    // Every question is a promise that the site teaches its answer. A citation
    // pointing at a page that does not exist, or a heading the renderer never
    // generates, breaks that promise silently — the link still looks fine.
    #[test]
    fn every_question_cites_a_real_page_and_heading() {
        let pages = PathBuf::from(crate::handlers::wiki::PAGES_DIR);
        let mut checked = 0;

        for set in all(&dir()) {
            for question in &set.questions {
                let page = crate::models::page::Page::find(&pages, &question.learn.slug)
                    .unwrap_or_else(|err| {
                        panic!(
                            "{}: question {:?} cites /learn/{}, which does not exist: {err:?}",
                            set.slug, question.stem, question.learn.slug
                        )
                    });
                let ids: Vec<&str> = page
                    .outline
                    .iter()
                    .map(|heading| heading.id.as_str())
                    .collect();
                assert!(
                    ids.contains(&question.learn.anchor.as_str()),
                    "{}: question {:?} cites /learn/{}#{}, which has no such heading. \
                     Available: {ids:?}",
                    set.slug,
                    question.stem,
                    question.learn.slug,
                    question.learn.anchor
                );
                checked += 1;
            }
        }

        if !all(&dir()).is_empty() {
            assert!(checked > 0, "topics loaded but no questions were checked");
        }
    }

    // A question with a wrong answer index is worse than no question — it
    // teaches the wrong thing with the site's authority behind it
    #[test]
    fn every_question_is_well_formed() {
        for set in all(&dir()) {
            assert!(!set.topic.is_empty(), "{}: no topic name", set.slug);
            assert!(
                !set.questions.is_empty(),
                "{}: a topic file with no questions",
                set.slug
            );
            for question in &set.questions {
                assert!(
                    question.options.len() >= 2,
                    "{}: {:?} needs at least two options",
                    set.slug,
                    question.stem
                );
                assert!(
                    question.answer < question.options.len(),
                    "{}: {:?} has answer index {} but only {} options",
                    set.slug,
                    question.stem,
                    question.answer,
                    question.options.len()
                );
                assert!(
                    question.explanation.len() > 40,
                    "{}: {:?} has no real explanation, which is the part worth having",
                    set.slug,
                    question.stem
                );
                let mut sorted = question.options.clone();
                sorted.sort();
                sorted.dedup();
                assert_eq!(
                    sorted.len(),
                    question.options.len(),
                    "{}: {:?} repeats an option",
                    set.slug,
                    question.stem
                );
            }
        }
    }

    #[test]
    fn a_skipped_question_is_not_scored_as_wrong() {
        let question = Question {
            stem: "Test".to_string(),
            options: vec!["a".to_string(), "b".to_string()],
            answer: 1,
            explanation: "x".repeat(50),
            learn: LearnCitation {
                slug: "osi-model".to_string(),
                anchor: "overview".to_string(),
            },
        };

        let skipped = GradedAnswer {
            question: question.clone(),
            chosen: None,
        };
        assert!(skipped.unanswered());
        assert!(!skipped.correct());
        assert_eq!(skipped.verdict(), "skipped");

        let wrong = GradedAnswer {
            question: question.clone(),
            chosen: Some(0),
        };
        assert_eq!(wrong.verdict(), "incorrect");

        let right = GradedAnswer {
            question,
            chosen: Some(1),
        };
        assert_eq!(right.verdict(), "correct");
    }
}
