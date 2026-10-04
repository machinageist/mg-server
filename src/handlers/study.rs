// Author:      machinageist
// Date:        2026-08-14
// Description: Handlers for /study — a topic index, a quiz per topic, and a
//              graded result page. The first POST route on the site.
//              Also the flashcards, the typed-command scenarios, and the
//              generated drills under /study/drills.
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
use crate::models::drill::{self, GradedProblem, Problem, ProblemKind};
use crate::models::question::{self, GradedAnswer, Question, QuestionSet, STUDY_DIR};
use crate::models::scenario::{self, GradedStep, PBQ_DIR, Scenario};
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::{Form, Path as AxumPath, Query};
use axum::response::{Redirect, Response};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

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

// -----------------------------------------------------------------------
// Subnetting drill — /study/drills/subnetting
// -----------------------------------------------------------------------

const SUBNETTING_DRILL_PATH: &str = "/study/drills/subnetting";

// Query keys the drill reads
const SEED_PARAM: &str = "seed";
const KIND_PARAM: &str = "type";

// Seeds are kept short so a shared URL stays readable
const SEED_RANGE: u128 = 1_000_000;

// Which set a request is for. The seed and the kind are the whole state
#[derive(Clone, Copy)]
pub struct DrillSet {
    pub seed: u64,
    pub kind: Option<ProblemKind>,
}

impl DrillSet {
    // The URL for this exact set
    pub fn href(&self) -> String {
        format!(
            "{SUBNETTING_DRILL_PATH}?{SEED_PARAM}={}{}",
            self.seed,
            kind_suffix(self.kind, '&')
        )
    }

    // The URL that starts a fresh set of the same kind
    pub fn new_set_href(&self) -> String {
        format!("{SUBNETTING_DRILL_PATH}{}", kind_suffix(self.kind, '?'))
    }

    pub fn problems(&self) -> Vec<Problem> {
        drill::subnetting_set(self.seed, self.kind)
    }
}

// Write the kind as a query fragment, or nothing when every kind is in play
fn kind_suffix(kind: Option<ProblemKind>, separator: char) -> String {
    kind.map(|kind| format!("{separator}{KIND_PARAM}={}", kind.key()))
        .unwrap_or_default()
}

// Read the problem kind from the query. Absent means every kind
fn parse_kind(query: &HashMap<String, String>) -> Result<Option<ProblemKind>, SiteError> {
    match query.get(KIND_PARAM) {
        None => Ok(None),
        Some(key) => ProblemKind::from_key(key)
            .map(Some)
            .ok_or_else(|| SiteError::PageNotFound(SUBNETTING_DRILL_PATH.to_string())),
    }
}

// Read the set a request names. A seed or kind that does not parse is a 404,
// not a silently different set
fn parse_drill_set(query: &HashMap<String, String>) -> Result<Option<DrillSet>, SiteError> {
    let kind = parse_kind(query)?;
    let Some(raw) = query.get(SEED_PARAM) else {
        return Ok(None);
    };
    let seed = raw
        .parse::<u64>()
        .map_err(|_| SiteError::PageNotFound(SUBNETTING_DRILL_PATH.to_string()))?;
    Ok(Some(DrillSet { seed, kind }))
}

// Pick a seed for a fresh set. It only has to differ between visits
fn fresh_seed() -> u64 {
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_micros())
        .unwrap_or_default();
    (micros % SEED_RANGE) as u64
}

#[derive(Template)]
#[template(path = "study_drill.html")]
pub struct DrillTemplate {
    pub set: DrillSet,
    pub problems: Vec<Problem>,
}

impl DrillTemplate {
    pub fn title(&self) -> &str {
        "Subnetting drill — machinageist"
    }
    pub fn description(&self) -> &str {
        "Generated subnetting problems, graded with the working shown for each one."
    }
    pub fn section(&self) -> &str {
        "study"
    }

    pub fn kinds(&self) -> [ProblemKind; 3] {
        ProblemKind::ALL
    }
}

// Render a drill set, or send a request with no seed to a fresh one
pub async fn subnetting_drill(
    Query(query): Query<HashMap<String, String>>,
) -> Result<Response, SiteError> {
    let Some(set) = parse_drill_set(&query)? else {
        let fresh = DrillSet {
            seed: fresh_seed(),
            kind: parse_kind(&query)?,
        };
        return Ok(Redirect::to(&fresh.href()).into_response());
    };
    Ok(DrillTemplate {
        set,
        problems: set.problems(),
    }
    .into_response())
}

#[derive(Template)]
#[template(path = "study_drill_result.html")]
pub struct DrillResultTemplate {
    pub set: DrillSet,
    pub problems: Vec<GradedProblem>,
    pub correct: usize,
    pub total: usize,
}

impl DrillResultTemplate {
    pub fn title(&self) -> &str {
        "Subnetting drill results — machinageist"
    }
    pub fn description(&self) -> &str {
        "Subnetting drill results, with the working for every problem."
    }
    pub fn section(&self) -> &str {
        "study"
    }

    pub fn skipped(&self) -> usize {
        self.problems.iter().filter(|p| p.unanswered()).count()
    }
}

// Name the form field for one input of one problem
pub fn drill_field_name(index: &usize, key: &str) -> String {
    format!("p{index}-{key}")
}

// Grade a submitted drill
//
// The set is rebuilt from the seed in the URL, so the server holds nothing
// between showing the problems and grading them. A field that is missing from
// the form is unanswered, not an error.
pub async fn grade_subnetting_drill(
    Query(query): Query<HashMap<String, String>>,
    Form(submitted): Form<HashMap<String, String>>,
) -> Result<impl IntoResponse, SiteError> {
    let set = parse_drill_set(&query)?
        .ok_or_else(|| SiteError::PageNotFound(SUBNETTING_DRILL_PATH.to_string()))?;
    Ok(grade_drill(set, &submitted))
}

// Pair each problem in a set with the submitted answers
fn grade_drill(set: DrillSet, submitted: &HashMap<String, String>) -> DrillResultTemplate {
    let problems: Vec<GradedProblem> = set
        .problems()
        .into_iter()
        .enumerate()
        .map(|(index, problem)| {
            GradedProblem::new(problem, |key| {
                submitted
                    .get(&drill_field_name(&index, key))
                    .cloned()
                    .unwrap_or_default()
            })
        })
        .collect();
    let correct = problems.iter().filter(|p| p.correct()).count();
    let total = problems.len();
    DrillResultTemplate {
        set,
        problems,
        correct,
        total,
    }
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

    // Build a query map from pairs
    fn query(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }

    #[test]
    fn the_drill_needs_no_javascript() {
        let set = DrillSet {
            seed: 42,
            kind: None,
        };
        let html = DrillTemplate {
            set,
            problems: set.problems(),
        }
        .render()
        .expect("drill renders");
        let content = html
            .split("<main")
            .nth(1)
            .and_then(|rest| rest.split("</main>").next())
            .expect("the page has a main element");

        assert!(!content.contains("<script"), "the drill loads no script");
        assert!(content.contains("method=\"post\""));
        assert!(content.contains("action=\"/study/drills/subnetting?seed=42\""));
        assert!(content.contains("<button type=\"submit\""));
        // Every input is a native field with a label pointing at it
        let inputs = content.matches("<input type=\"text\"").count();
        assert_eq!(inputs, content.matches("<label for=\"p").count());
        assert!(inputs >= drill::SET_SIZE);
    }

    #[test]
    fn a_drill_url_names_one_set() {
        let plain = parse_drill_set(&query(&[("seed", "42")]))
            .expect("parses")
            .expect("has a seed");
        assert_eq!(plain.href(), "/study/drills/subnetting?seed=42");
        assert_eq!(plain.new_set_href(), "/study/drills/subnetting");

        let hosts = parse_drill_set(&query(&[("seed", "42"), ("type", "hosts")]))
            .expect("parses")
            .expect("has a seed");
        assert_eq!(hosts.href(), "/study/drills/subnetting?seed=42&type=hosts");
        assert_eq!(hosts.new_set_href(), "/study/drills/subnetting?type=hosts");
        assert!(
            hosts
                .problems()
                .iter()
                .all(|problem| problem.kind() == ProblemKind::Hosts)
        );

        // No seed is not an error. The handler redirects to a fresh set
        assert!(parse_drill_set(&query(&[])).expect("parses").is_none());
    }

    #[test]
    fn a_bad_seed_or_kind_is_not_found() {
        for bad in [
            query(&[("seed", "abc")]),
            query(&[("seed", "-1")]),
            query(&[("seed", "1"), ("type", "nonsense")]),
            query(&[("type", "nonsense")]),
        ] {
            assert!(matches!(
                parse_drill_set(&bad),
                Err(SiteError::PageNotFound(_))
            ));
        }
    }

    #[tokio::test]
    async fn a_request_with_no_seed_redirects_to_a_seeded_set() {
        let response = subnetting_drill(Query(query(&[("type", "vlsm")])))
            .await
            .expect("redirects");
        assert_eq!(response.status(), axum::http::StatusCode::SEE_OTHER);
        let location = response.headers()["location"].to_str().expect("ascii");
        assert!(location.starts_with("/study/drills/subnetting?seed="));
        assert!(location.ends_with("&type=vlsm"));
    }

    #[test]
    fn a_submitted_drill_is_graded_from_the_seed_alone() {
        let set = DrillSet {
            seed: 42,
            kind: None,
        };
        // Answer the first problem correctly and leave the rest blank
        let mut submitted = HashMap::new();
        for field in set.problems()[0].fields() {
            submitted.insert(drill_field_name(&0, field.key), field.answer.display());
        }
        let result = grade_drill(set, &submitted);
        assert_eq!(result.total, drill::SET_SIZE);
        assert_eq!(result.correct, 1);
        assert_eq!(result.skipped(), drill::SET_SIZE - 1);

        let html = result.render().expect("result renders");
        assert!(html.contains("1 of 10 correct"));
        assert!(html.contains("9 skipped"));
        assert!(html.contains("/learn/subnetting#"));
        assert!(html.contains("Block size is") || html.contains("octet boundary"));
    }

    // /study/:slug would be shadowed by these fixed path segments, so a
    // question set may not take one as its slug
    #[test]
    fn no_question_set_uses_a_reserved_path_name() {
        for set in question::all(&PathBuf::from(STUDY_DIR)) {
            assert!(
                !["drills", "walks", "cards", "pbq"].contains(&set.slug.as_str()),
                "{} is a reserved /study path",
                set.slug
            );
        }
    }
}
