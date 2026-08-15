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
use crate::models::question::{self, GradedAnswer, Question, QuestionSet, STUDY_DIR};
use crate::models::scenario::{self, GradedStep, PBQ_DIR, Scenario};
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::{Form, Path as AxumPath, Query};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

// -----------------------------------------------------------------------
// Index — /study
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "study_index.html")]
pub struct StudyIndexTemplate {
    pub sets: Vec<QuestionSet>,
    pub scenarios: Vec<Scenario>,
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
        scenarios: scenario::all(&PathBuf::from(PBQ_DIR)),
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

// -----------------------------------------------------------------------
// Flashcards — GET /study/cards/:slug
// -----------------------------------------------------------------------

// Which card, and whether its answer is showing. Both live in the URL so the
// whole flow is navigable, bookmarkable, and survives the back button.
#[derive(Debug, Deserialize)]
pub struct CardPosition {
    #[serde(default)]
    pub i: Option<usize>,
    #[serde(default)]
    pub show: Option<u8>,
}

#[derive(Template)]
#[template(path = "study_cards.html")]
pub struct CardsTemplate {
    pub set: QuestionSet,
    pub index: usize,
    pub revealed: bool,
}

impl CardsTemplate {
    pub fn title(&self) -> String {
        format!("{} flashcards — machinageist", self.set.topic)
    }
    pub fn description(&self) -> &str {
        "Flashcards drawn from the education wiki."
    }
    pub fn section(&self) -> &str {
        "study"
    }

    pub fn card(&self) -> &Question {
        &self.set.questions[self.index]
    }

    pub fn total(&self) -> usize {
        self.set.questions.len()
    }

    // 1-based for display; the URL stays 0-based to match the answer indices
    pub fn position(&self) -> usize {
        self.index + 1
    }

    pub fn has_previous(&self) -> bool {
        self.index > 0
    }

    pub fn has_next(&self) -> bool {
        self.index + 1 < self.total()
    }

    pub fn previous_index(&self) -> usize {
        self.index.saturating_sub(1)
    }

    pub fn next_index(&self) -> usize {
        self.index + 1
    }
}

// Render one flashcard, prompt-side or answer-side
//
// A card is a question rendered without its distractors — the bank is shared
// rather than duplicated, so a corrected explanation fixes both surfaces.
pub async fn cards(
    AxumPath(slug): AxumPath<String>,
    Query(position): Query<CardPosition>,
) -> Result<impl IntoResponse, SiteError> {
    let set = question::load(&PathBuf::from(STUDY_DIR), &slug)?;
    if set.questions.is_empty() {
        return Err(SiteError::PageNotFound(slug));
    }

    // Clamp rather than 404 — a hand-edited or stale index should land on a
    // real card, not an error page
    let index = position.i.unwrap_or(0).min(set.questions.len() - 1);

    Ok(CardsTemplate {
        set,
        index,
        revealed: position.show.is_some_and(|show| show == 1),
    })
}

// -----------------------------------------------------------------------
// Performance-based scenarios — /study/pbq/:slug
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "study_pbq.html")]
pub struct ScenarioTemplate {
    pub scenario: Scenario,
}

impl ScenarioTemplate {
    pub fn title(&self) -> String {
        format!("{} — machinageist", self.scenario.title)
    }
    pub fn description(&self) -> &str {
        "A performance-based scenario worked one command at a time."
    }
    pub fn section(&self) -> &str {
        "study"
    }
}

// Render a scenario's steps as a single form
pub async fn scenario_page(
    AxumPath(slug): AxumPath<String>,
) -> Result<impl IntoResponse, SiteError> {
    let scenario = scenario::load(&PathBuf::from(PBQ_DIR), &slug)?;
    Ok(ScenarioTemplate { scenario })
}

#[derive(Template)]
#[template(path = "study_pbq_result.html")]
pub struct ScenarioResultTemplate {
    pub title: String,
    pub slug: String,
    pub steps: Vec<GradedStep>,
    pub correct: usize,
    pub total: usize,
}

impl ScenarioResultTemplate {
    pub fn title(&self) -> String {
        format!("{} results — machinageist", self.title)
    }
    pub fn description(&self) -> &str {
        "Scenario results, with the accepted command and an explanation for every step."
    }
    pub fn section(&self) -> &str {
        "study"
    }

    pub fn skipped(&self) -> usize {
        self.steps.iter().filter(|s| s.unanswered()).count()
    }
}

// Grade a submitted scenario
pub async fn grade_scenario(
    AxumPath(slug): AxumPath<String>,
    Form(submitted): Form<HashMap<String, String>>,
) -> Result<impl IntoResponse, SiteError> {
    let scenario = scenario::load(&PathBuf::from(PBQ_DIR), &slug)?;

    let steps: Vec<GradedStep> = scenario
        .steps
        .iter()
        .enumerate()
        .map(|(index, step)| GradedStep {
            step: step.clone(),
            typed: submitted
                .get(&format!("s{index}"))
                .cloned()
                .unwrap_or_default(),
        })
        .collect();

    let correct = steps.iter().filter(|s| s.correct()).count();
    let total = steps.len();

    Ok(ScenarioResultTemplate {
        title: scenario.title,
        slug,
        steps,
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

    fn cards_view(set: QuestionSet, index: usize, revealed: bool) -> CardsTemplate {
        CardsTemplate {
            set,
            index,
            revealed,
        }
    }

    #[test]
    fn a_card_hides_its_answer_until_the_url_says_otherwise() {
        let Some(set) = any_topic() else {
            return;
        };
        let answer = set.questions[0].answer_html();

        let hidden = cards_view(set.clone(), 0, false)
            .render()
            .expect("card renders");
        assert!(
            !hidden.contains(&answer),
            "the answer must not be in the prompt-side HTML at all — hiding it \
             with CSS would leak it to anyone reading the source"
        );
        assert!(
            hidden.contains("show=1"),
            "there must be a way to reveal it"
        );

        let shown = cards_view(set, 0, true).render().expect("card renders");
        assert!(shown.contains(&answer));
    }

    #[test]
    fn card_navigation_is_links_and_bounded_at_both_ends() {
        let Some(set) = any_topic() else {
            return;
        };
        let last = set.questions.len() - 1;

        let first = cards_view(set.clone(), 0, false).render().expect("renders");
        assert!(
            !first.contains("Previous"),
            "no previous before the first card"
        );
        assert!(first.contains("Next"));

        let end = cards_view(set, last, false).render().expect("renders");
        assert!(end.contains("Previous"));
        assert!(
            end.contains("Take the quiz"),
            "the last card should offer somewhere to go"
        );
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
