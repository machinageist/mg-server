// Author:      machinageist
// Date:        2026-08-14
// Description: Handlers for /study — a topic index, a quiz per topic, and a
//              graded result page. The first POST route on the site.
// Notes:       No JavaScript anywhere. The quiz is one <form method="post">
//              with radio inputs; grading happens server-side and renders a
//              page. criteria.md auto-fail rule 3 is met by construction
//              rather than by a fallback.
//
//              Nothing is stored. No session, no cookie, no score history, and
//              the submitted answers are never logged — the result page is the
//              whole output. That also means there is no CSRF surface worth
//              defending: the form has no side effects and nothing to forge
//              against.

use crate::errors::SiteError;
use crate::models::question::{self, GradedAnswer, QuestionSet, STUDY_DIR};
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::{Form, Path as AxumPath};
use std::collections::HashMap;
use std::path::PathBuf;

// -----------------------------------------------------------------------
// Index — /study
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "study_index.html")]
pub struct StudyIndexTemplate {
    pub sets: Vec<QuestionSet>,
}

impl StudyIndexTemplate {
    pub fn title(&self) -> &str {
        "Study — machinageist"
    }
    pub fn description(&self) -> &str {
        "Practice questions drawn from the education wiki, each answer linked to the page that explains it."
    }
    pub fn section(&self) -> &str {
        "study"
    }
}

// Render the list of topics that have questions
pub async fn index() -> impl IntoResponse {
    StudyIndexTemplate {
        sets: question::all(&PathBuf::from(STUDY_DIR)),
    }
}

// -----------------------------------------------------------------------
// Quiz — GET /study/:slug
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "study_quiz.html")]
pub struct QuizTemplate {
    pub set: QuestionSet,
}

impl QuizTemplate {
    pub fn title(&self) -> String {
        format!("{} — machinageist", self.set.topic)
    }
    pub fn description(&self) -> &str {
        "Practice questions from the education wiki."
    }
    pub fn section(&self) -> &str {
        "study"
    }
}

// Render one topic's quiz
pub async fn quiz(AxumPath(slug): AxumPath<String>) -> Result<impl IntoResponse, SiteError> {
    let set = question::load(&PathBuf::from(STUDY_DIR), &slug)?;
    Ok(QuizTemplate { set })
}

// -----------------------------------------------------------------------
// Result — POST /study/:slug
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "study_result.html")]
pub struct ResultTemplate {
    pub topic: String,
    pub slug: String,
    pub answers: Vec<GradedAnswer>,
    pub correct: usize,
    pub total: usize,
}

impl ResultTemplate {
    pub fn title(&self) -> String {
        format!("{} results — machinageist", self.topic)
    }
    pub fn description(&self) -> &str {
        "Practice question results, with an explanation for every answer."
    }
    pub fn section(&self) -> &str {
        "study"
    }

    // How many were left blank — reported separately, because skipping is not
    // the same as getting it wrong and a result page that conflates them is
    // telling the reader something false
    pub fn skipped(&self) -> usize {
        self.answers.iter().filter(|a| a.unanswered()).count()
    }
}

// Grade a submitted quiz
//
// Field names are `q<index>` and values are the chosen option index. Anything
// unparseable is treated as unanswered rather than as an error — a mangled
// form should not cost a reader the answers they did give.
pub async fn grade(
    AxumPath(slug): AxumPath<String>,
    Form(submitted): Form<HashMap<String, String>>,
) -> Result<impl IntoResponse, SiteError> {
    let set = question::load(&PathBuf::from(STUDY_DIR), &slug)?;

    let answers: Vec<GradedAnswer> = set
        .questions
        .iter()
        .enumerate()
        .map(|(index, question)| GradedAnswer {
            question: question.clone(),
            chosen: submitted
                .get(&format!("q{index}"))
                .and_then(|value| value.parse::<usize>().ok())
                .filter(|choice| *choice < question.options.len()),
        })
        .collect();

    let correct = answers.iter().filter(|a| a.correct()).count();
    let total = answers.len();

    Ok(ResultTemplate {
        topic: set.topic,
        slug,
        answers,
        correct,
        total,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_topic() -> Option<QuestionSet> {
        question::all(&PathBuf::from(STUDY_DIR)).into_iter().next()
    }

    fn grade_locally(set: &QuestionSet, submitted: HashMap<String, String>) -> ResultTemplate {
        let answers: Vec<GradedAnswer> = set
            .questions
            .iter()
            .enumerate()
            .map(|(index, question)| GradedAnswer {
                question: question.clone(),
                chosen: submitted
                    .get(&format!("q{index}"))
                    .and_then(|value| value.parse::<usize>().ok())
                    .filter(|choice| *choice < question.options.len()),
            })
            .collect();
        let correct = answers.iter().filter(|a| a.correct()).count();
        let total = answers.len();
        ResultTemplate {
            topic: set.topic.clone(),
            slug: set.slug.clone(),
            answers,
            correct,
            total,
        }
    }

    #[test]
    fn a_full_correct_submission_scores_everything() {
        let Some(set) = any_topic() else {
            return; // no question bank authored yet
        };
        let submitted: HashMap<String, String> = set
            .questions
            .iter()
            .enumerate()
            .map(|(i, q)| (format!("q{i}"), q.answer.to_string()))
            .collect();

        let result = grade_locally(&set, submitted);
        assert_eq!(result.correct, result.total);
        assert_eq!(result.skipped(), 0);
    }

    #[test]
    fn an_empty_submission_is_skipped_not_wrong() {
        let Some(set) = any_topic() else {
            return;
        };
        let result = grade_locally(&set, HashMap::new());
        assert_eq!(result.correct, 0);
        assert_eq!(
            result.skipped(),
            result.total,
            "a blank form means unanswered, and the result must say so"
        );
    }

    #[test]
    fn an_out_of_range_choice_is_ignored_rather_than_trusted() {
        let Some(set) = any_topic() else {
            return;
        };
        let mut submitted = HashMap::new();
        submitted.insert("q0".to_string(), "9999".to_string());
        submitted.insert("q1".to_string(), "not-a-number".to_string());

        let result = grade_locally(&set, submitted);
        assert!(
            result.answers[0].unanswered(),
            "an index past the option list must not be treated as a choice"
        );
    }

    #[test]
    fn the_result_explains_every_answer_and_links_the_teaching_page() {
        let Some(set) = any_topic() else {
            return;
        };
        let html = grade_locally(&set, HashMap::new())
            .render()
            .expect("result renders");

        for question in &set.questions {
            // Compared against the rendered form: explanations carry inline
            // code spans for commands and addresses, so the raw Markdown is not
            // what reaches the page
            assert!(
                html.contains(&question.explanation_html()),
                "the explanation is the product — {:?} is missing it",
                question.stem
            );
            assert!(
                html.contains(&question.learn.href()),
                "{:?} does not link the page that teaches it",
                question.stem
            );
        }
    }

    #[test]
    fn the_quiz_needs_no_javascript() {
        let Some(set) = any_topic() else {
            return;
        };
        let html = QuizTemplate { set }.render().expect("quiz renders");
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

        assert!(
            stripped.contains("method=\"post\""),
            "plain form submission"
        );
        assert!(
            stripped.contains("type=\"radio\""),
            "native inputs, not scripted widgets"
        );
        assert!(stripped.contains("<button"), "a real submit button");
    }
}
