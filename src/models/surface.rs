// Author:      machinageist
// Date:        2026-08-14
// Description: The registry of public surfaces — every part of the site a
//              reader can arrive at, with its nav label and its one-line
//              description. The header nav, the home page, and the about page
//              all render from this list rather than hardcoding their own.
// Notes:       This exists because home and about kept going stale. Labs and
//              search shipped and neither page mentioned them, because nothing
//              made them. Adding a route now means adding one entry here, and
//              every surface that lists the site updates with it.
//
//              What is NOT automatic is caught loudly instead: a test walks
//              src/router.rs and fails if a route exists with neither a
//              registry entry nor a place on the utility list below. That is
//              the drift guard criteria 5B asks for — the thing that cannot
//              silently fall out of sync.

// One public surface of the site
pub struct Surface {
    pub path: &'static str,
    // Nav label, and the link text on any page that lists surfaces
    pub label: &'static str,
    // One line, present tense, describing what a reader finds there
    pub blurb: &'static str,
    // The value the page's section() returns, for the active-nav check
    pub section: &'static str,
    // Whether it earns a slot in the header. The nav holds about six before it
    // stops being scannable; everything else is reachable from home and from
    // the surfaces it belongs to.
    pub in_nav: bool,
}

// Routes that are not reader-facing surfaces and so need no registry entry.
// Listed explicitly so the drift guard stays strict — a new route is either a
// surface or a deliberate exception, never an oversight.
//
// Test-scoped because nothing at runtime consults it: its only job is to be the
// allowlist the guard checks against, and keeping it here rather than buried in
// the test module puts the exceptions next to the rule they are exceptions to.
#[cfg(test)]
pub const UTILITY_ROUTES: &[&str] = &[
    "/",                         // the home page itself, reached via the brand
    "/status.json",              // machine-readable twin of /status
    "/wiki",                     // pre-rename redirect, kept so old links work
    "/wiki/:slug",               // ditto, per page
    "/robots.txt",               //
    "/security.txt",             //
    "/.well-known/security.txt", //
];

// Every surface, in the order a reader would meet them
pub const SURFACES: &[Surface] = &[
    Surface {
        path: "/about",
        label: "About",
        blurb: "Who I am, what I actually operate, and what each claim is backed by.",
        section: "about",
        in_nav: true,
    },
    Surface {
        path: "/portfolio",
        label: "Portfolio",
        blurb: "Finished work, with evidence. Deliberately short.",
        section: "portfolio",
        in_nav: true,
    },
    Surface {
        path: "/labs",
        label: "Labs",
        blurb: "What I plan to build next in the homelab, each with a step-by-step plan you could follow.",
        section: "labs",
        in_nav: true,
    },
    Surface {
        path: "/blog",
        label: "Writing",
        blurb: "Writeups of work on the lab, including the outages.",
        section: "writing",
        in_nav: true,
    },
    Surface {
        path: "/learn",
        label: "Learn",
        blurb: "A public education wiki on networking and Linux, built from my own study notes and checked against the primary sources.",
        section: "wiki",
        in_nav: true,
    },
    Surface {
        path: "/glossary",
        label: "Glossary",
        blurb: "Terms and commands from the wiki, each linked back to the page that explains it.",
        section: "glossary",
        in_nav: false,
    },
    Surface {
        path: "/search",
        label: "Search",
        blurb: "Full-text search across the writing and the wiki.",
        section: "search",
        in_nav: true,
    },
    Surface {
        path: "/status",
        label: "Status",
        blurb: "Live uptime and request counts for this server.",
        section: "status",
        in_nav: false,
    },
];

// The surfaces that appear in the header nav
pub fn nav() -> Vec<&'static Surface> {
    SURFACES.iter().filter(|surface| surface.in_nav).collect()
}

// Every surface, for pages that list what the site contains
pub fn all() -> &'static [Surface] {
    SURFACES
}

#[cfg(test)]
mod tests {
    use super::*;

    // The guard that makes the registry worth having. Shipping a route without
    // registering it is how /labs and /search ended up unmentioned on the home
    // page for a day — this fails the build instead.
    #[test]
    fn every_route_is_a_registered_surface_or_a_declared_utility() {
        let router = std::fs::read_to_string("src/router.rs").expect("router source");

        let mut unregistered = Vec::new();
        for (index, _) in router.match_indices(".route(\"") {
            let path: String = router[index + ".route(\"".len()..]
                .chars()
                .take_while(|c| *c != '"')
                .collect();

            if UTILITY_ROUTES.contains(&path.as_str()) {
                continue;
            }
            // A route belongs to a surface if it *is* that surface or lives
            // beneath it — /learn/:slug and /glossary/terms are both parts of
            // their parent, not separate places a reader arrives at
            let covered = SURFACES.iter().any(|surface| {
                path == surface.path || path.starts_with(&format!("{}/", surface.path))
            });
            if !covered {
                unregistered.push(path);
            }
        }

        assert!(
            unregistered.is_empty(),
            "these routes are neither a registered surface nor a declared utility: {unregistered:?}. \
             Add them to SURFACES so the nav, the home page, and /about pick them up, or to \
             UTILITY_ROUTES if they are not reader-facing."
        );
    }

    #[test]
    fn every_surface_says_something_specific() {
        for surface in all() {
            assert!(
                surface.path.starts_with('/'),
                "{} is not a path",
                surface.path
            );
            assert!(!surface.label.is_empty(), "{} has no label", surface.path);
            assert!(
                surface.blurb.len() > 20,
                "{}'s blurb is too short to be worth rendering",
                surface.path
            );
            assert!(
                !surface.section.is_empty(),
                "{} has no section key, so the nav cannot mark it active",
                surface.path
            );
        }
    }

    // The registry only helps if the pages that list the site actually render
    // from it. These assert the wiring, so removing the loop from a template
    // fails here rather than silently reverting to a hand-maintained list.
    #[test]
    fn the_home_and_about_pages_list_every_surface() {
        use askama::Template;

        let home = crate::handlers::pages::IndexTemplate {
            name: "machinageist".to_string(),
            posts: Vec::new(),
        }
        .render()
        .expect("home renders");

        let about = crate::handlers::pages::AboutTemplate {
            bio: "Test bio.".to_string(),
        }
        .render()
        .expect("about renders");

        for surface in all() {
            for (page, name) in [(&home, "home"), (&about, "about")] {
                assert!(
                    page.contains(&format!("href=\"{}\"", surface.path)),
                    "the {name} page does not link {} — is it still rendering from the registry?",
                    surface.path
                );
                assert!(
                    page.contains(surface.blurb),
                    "the {name} page does not carry {}'s description",
                    surface.path
                );
            }
        }
    }

    #[test]
    fn the_nav_stays_scannable() {
        assert!(
            nav().len() <= 6,
            "the header nav has {} entries; past six it stops being scannable and \
             something should move to in_nav: false",
            nav().len()
        );
    }
}
