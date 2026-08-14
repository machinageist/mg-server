// Author:      machinageist
// Date:        2026-08-14
// Description: Handler for /labs — the homelab progress surface. Groups the
//              lab list from models::lab by phase and renders it in dependency
//              order.
// Notes:       This is a PROGRESS page, not a portfolio page. criteria.md 1C
//              permits work in progress to appear here and forbids it implying
//              portfolio status, so the template leads with status and the
//              model refuses to call anything Done without a published writeup.
//              Grouping is derived from the list rather than hardcoded, so a
//              new phase cannot be added to the model and silently fail to
//              render.

use crate::errors::SiteError;
use crate::models::lab::{self, Lab, Phase};
use crate::models::page::Page;
use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::Path as AxumPath;
use std::path::PathBuf;

// Procedures live as Markdown beside the wiki's, so they get the same renderer,
// the same heading anchors, and code blocks that survive a narrow viewport
pub(crate) const LABS_DIR: &str = "content/labs";

// One phase and the labs inside it, in dependency order
pub struct PhaseGroup {
    pub label: &'static str,
    pub blurb: &'static str,
    pub labs: Vec<Lab>,
}

#[derive(Template)]
#[template(path = "labs.html")]
pub struct LabsTemplate {
    pub groups: Vec<PhaseGroup>,
}

impl LabsTemplate {
    pub fn title(&self) -> &str {
        "Labs — machinageist"
    }

    pub fn description(&self) -> &str {
        "The homelab work in progress — recovery, network segmentation, and services, in the order it has to happen."
    }

    pub fn section(&self) -> &str {
        "labs"
    }

    // Count every entry across the phases, for the intro line
    pub fn total(&self) -> usize {
        self.groups.iter().map(|group| group.labs.len()).sum()
    }
}

// Describe what each phase is for, so a group heading is not just a label
fn blurb(phase: Phase) -> &'static str {
    match phase {
        Phase::Segmentation => {
            "Divide the flat network into VLANs, one change domain at a time, lowest-risk zone \
             first and the management network last."
        }
        Phase::Services => {
            "Put each service on its target zone and prove its configuration, rather than \
             assuming the VM that exists is the VM that was intended."
        }
    }
}

// Group the lab list by phase, preserving dependency order within each
fn grouped() -> Vec<PhaseGroup> {
    let labs = lab::all();
    let mut groups: Vec<PhaseGroup> = Vec::new();

    for entry in labs {
        match groups.last_mut() {
            Some(group) if group.label == entry.phase.label() => group.labs.push(entry),
            _ => groups.push(PhaseGroup {
                label: entry.phase.label(),
                blurb: blurb(entry.phase),
                labs: vec![entry],
            }),
        }
    }
    groups
}

// Build the labs view
fn labs_view() -> LabsTemplate {
    LabsTemplate { groups: grouped() }
}

// Render the homelab progress page
pub async fn labs() -> impl IntoResponse {
    labs_view()
}

// -----------------------------------------------------------------------
// Detail page — /labs/:slug
// -----------------------------------------------------------------------

#[derive(Template)]
#[template(path = "lab_page.html")]
pub struct LabPageTemplate {
    pub lab: Lab,
    // The step-by-step procedure, rendered from content/labs/<slug>.md
    pub page: Page,
}

impl LabPageTemplate {
    pub fn title(&self) -> String {
        format!("{} — machinageist", self.lab.name)
    }

    pub fn description(&self) -> &str {
        &self.page.summary
    }

    pub fn section(&self) -> &str {
        "labs"
    }
}

// Render one lab's procedure, selected by URL slug
//
// The slug is checked against the model rather than passed to the filesystem,
// so an unknown slug is a 404 from the allowlist and never a path read — the
// same shape /learn/:slug uses.
pub async fn lab_page(AxumPath(slug): AxumPath<String>) -> Result<impl IntoResponse, SiteError> {
    let lab = lab::all()
        .into_iter()
        .find(|entry| entry.slug == slug)
        .ok_or_else(|| SiteError::PageNotFound(slug.clone()))?;
    let page = Page::find(&PathBuf::from(LABS_DIR), lab.slug)?;
    Ok(LabPageTemplate { lab, page })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render() -> String {
        labs_view().render().expect("labs template renders")
    }

    #[test]
    fn every_lab_in_the_model_reaches_the_page() {
        let rendered = render();
        for entry in lab::all() {
            assert!(
                rendered.contains(entry.name),
                "{} is in the model but not on the page",
                entry.name
            );
        }
    }

    #[test]
    fn grouping_covers_every_phase_without_dropping_entries() {
        let view = labs_view();
        assert_eq!(
            view.total(),
            lab::all().len(),
            "grouping lost or duplicated entries"
        );
        // Phases are contiguous in the model, so each appears exactly once
        let mut labels: Vec<&str> = view.groups.iter().map(|group| group.label).collect();
        let count = labels.len();
        labels.sort_unstable();
        labels.dedup();
        assert_eq!(labels.len(), count, "a phase was split across two groups");
    }

    // The page must name the off-list precondition, or every entry reads as
    // blocked for no stated reason
    #[test]
    fn the_page_states_the_precondition_and_links_its_writeup() {
        let rendered = render();
        assert!(
            rendered.contains(lab::RECOVERY_GATE),
            "the page must say what everything is waiting on"
        );
        assert!(
            rendered.contains("/blog/management-layer-first-network-migration"),
            "the precondition must link the post that covers it"
        );
    }

    // Same three-place registration the wiki has: a lab needs a model entry and
    // a procedure file, and neither may exist without the other
    #[test]
    fn every_lab_has_a_procedure_and_every_procedure_has_a_lab() {
        let dir = PathBuf::from(LABS_DIR);
        let slugs: Vec<&str> = lab::all().iter().map(|entry| entry.slug).collect();

        for slug in &slugs {
            let path = dir.join(format!("{slug}.md"));
            assert!(
                path.exists(),
                "{} is on the list with no procedure at {}",
                slug,
                path.display()
            );
            Page::find(&dir, slug).unwrap_or_else(|err| panic!("{slug} does not parse: {err:?}"));
        }

        for entry in std::fs::read_dir(&dir).expect("read content/labs") {
            let path = entry.expect("dir entry").path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }
            let stem = path.file_stem().and_then(|s| s.to_str()).expect("utf-8");
            assert!(
                slugs.contains(&stem),
                "orphaned procedure with no lab entry: {}",
                path.display()
            );
        }
    }

    #[test]
    fn no_recovery_work_appears_on_the_page() {
        let rendered = render();
        for stage in ["R0 —", "R1 —", "R2 —", "R3 —", "R4 —"] {
            assert!(
                !rendered.contains(stage),
                "{stage} is recovery work — covered by the blog post, not queued here"
            );
        }
    }

    // criteria.md 1C: a progress surface may show work in progress and may not
    // imply portfolio status. The page must say what these are, not let a
    // reader assume they are finished work.
    #[test]
    fn the_page_reads_as_progress_not_portfolio() {
        let rendered = render();
        let body = rendered
            .split_once("<main")
            .map(|(_, rest)| rest)
            .expect("layout always renders a <main>");

        // Status is stated in words, never carried by colour alone
        assert!(
            body.contains("blocked"),
            "blocked status must be spelled out"
        );
        assert!(body.contains("next"), "the next action must be labelled");
        // Nothing here is a portfolio claim
        assert!(!body.contains("Completed project"));
        assert!(!body.contains("case study"));
        // The anti-overclaim vocabulary the model bans must not arrive via the
        // template's own copy either
        for banned in [
            "penetration test",
            "red team",
            "offensive security",
            "SOC analyst",
        ] {
            assert!(!body.contains(banned), "page copy claims {banned:?}");
        }
    }

    #[test]
    fn no_certification_is_named_on_the_page() {
        let rendered = render();
        for cert in [
            "Network+",
            "Security+",
            "Server+",
            "CompTIA",
            "RHCSA",
            "CCNA",
        ] {
            assert!(
                !rendered.contains(cert),
                "the labs page names {cert:?} — no cert claim without a booked voucher"
            );
        }
    }
}
