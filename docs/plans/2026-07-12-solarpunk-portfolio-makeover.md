# Solarpunk Portfolio Makeover — Review Draft

> Planning only. Do not implement from this document until Jeff reviews the parallel Claude changes, revises this draft, and explicitly approves implementation.

## Goal

Evolve `machinageist.dev` from a restrained terminal-themed portfolio into a warm, credible solarpunk operations fieldbook: practical infrastructure, repair, local control, low-power systems, and evidence of stewardship.

The makeover should not turn the site into botanical decoration over unchanged content. Solarpunk here means a systems ethic made visible:

- repair before replacement;
- right-sized infrastructure instead of maximal infrastructure;
- local-first and user-owned where practical;
- measured energy and resource use instead of vague “green” claims;
- resilience through understandable maps, backups, and exit paths;
- technology that households and small communities can maintain;
- optimism grounded in logs, tests, tradeoffs, and limits.

## Desired impression

A reviewer should feel they have opened a field notebook from a small, carefully tended network workshop — not a cyberpunk terminal, generic eco startup, cottagecore blog, or glossy enterprise dashboard.

Three traits should coexist:

1. **Operational:** diagrams, status, evidence, dates, tests, recovery notes.
2. **Human:** plain language, visible care, honest limits, household-scale outcomes.
3. **Regenerative:** repair, reuse, low power, local control, long service life, graceful exits.

## Design principles

### 1. Field station, not command center

Replace the “operator in a dark terminal” mood with a quiet field-station mood. Keep technical precision, but let paper, maps, labels, daylight, plant forms, and material textures soften it.

### 2. Evidence remains the visual hierarchy

The strongest visual elements should point to what was built, tested, measured, broken, restored, and learned. Decorative motifs must never overpower evidence or imply capability.

### 3. Solarpunk is a practice, not a claim

Use concrete labels such as “low-power host measured at…,” “existing router retained,” or “restore tested.” Avoid broad labels such as sustainable, carbon-neutral, green infrastructure, resilient, or sovereign unless the page contains evidence supporting that exact statement.

### 4. Nature cues stay restrained

Favor abstract leaf/branch/network geometry, contour lines, sun arcs, seed/patch motifs, and hand-drawn annotations. Avoid stock foliage, neon green gradients, fantasy eco-cities, and visual clutter.

### 5. Accessibility is part of the aesthetic

Maintain strong contrast, semantic HTML, keyboard focus, reduced-motion behavior, readable line lengths, and a fully coherent light/dark/system experience. “Organic” must not mean low contrast or irregular interaction behavior.

## Proposed visual direction

### Palette

Plan a palette study before selecting final tokens. Candidate roles:

- **Day background:** warm recycled-paper or limestone neutral, not pure white.
- **Night background:** deep forest-charcoal, not blue-purple Dracula.
- **Primary text:** ink/charcoal with WCAG-compliant contrast.
- **Growth accent:** moss or fern green for links and verified/active states.
- **Sun accent:** muted marigold/ochre for highlights, dates, and cautions.
- **Water accent:** restrained blue-green for networking/request paths.
- **Soil accent:** clay/rust for incidents, boundaries, and destructive-action warnings.
- **Surfaces:** subtle tonal layers rather than heavy card shadows.

Do not map “green = good, red = bad” as the only status signal. Preserve text labels and shape/border differences.

### Typography

Retire monospace as the universal body voice while preserving it for commands, metrics, dates, tags, and small technical labels.

Potential direction:

- humanist/system sans or readable serif for narrative body copy;
- sturdy display face or system serif for page titles;
- system monospace for evidence and technical metadata;
- no Google Fonts or third-party font requests;
- if self-hosted fonts are considered, review weight, privacy, licensing, payload, and fallback behavior first.

The preferred no-dependency starting point is a strong system-font stack with clearer role separation.

### Shape and texture

- soft but not bubbly corner radii;
- thin “mapped route” borders and connectors;
- subtle paper grain or contour-line SVG generated locally, with a no-texture fallback;
- occasional annotation marks that resemble pencil, survey, or garden labels;
- asymmetric section rhythm used sparingly, while cards and evidence tables remain aligned;
- no parallax, particle fields, autoplay, or heavy canvas effects.

### Illustration system

If illustrations are added later, use a consistent small set of original, local SVG motifs:

- sun + circuit horizon;
- root/branch network topology;
- home/mini-PC/router as a small habitat;
- water-cycle-like request paths;
- patch/repair mark for incident and restoration stories;
- seedling stages for planned → running → verified → maintained.

Illustrations should explain structure or state, not merely fill space.

## Information architecture proposal

Do not add all of this automatically. Review against Claude’s completed work first.

### Navigation

Keep the current routes initially. Consider relabeling or restructuring only after content review:

- Start / Home
- Labs
- Field Notes (current Writing)
- About
- Archive

“Labs” should only become a route when at least one lab has real evidence. Until then, planned labs can remain private planning documents; do not launch an empty promise shelf.

### Homepage

Proposed narrative order:

1. **Posture:** junior SysAdmin/NOC trajectory expressed through systems stewardship.
2. **Current fieldwork:** two or three verified/in-progress items with precise status.
3. **Evidence trail:** latest lab, incident, restore, or operating note.
4. **Working principles:** map it, reduce exposure, test recovery, leave a handoff.
5. **Learning path:** certs and skills as support, not the emotional center.
6. **Clear next clicks:** Labs/Portfolio and Field Notes/Writing.

Avoid turning the hero into a manifesto. One grounded sentence is stronger than multiple paragraphs of ecological branding.

### Portfolio / future Labs

Organize artifacts by lifecycle rather than a flat project list:

- **Observe** — inventories, baselines, maps, measurements.
- **Tend** — updates, hardening, maintenance, monitoring.
- **Repair** — incidents, break/fix drills, restore exercises.
- **Share** — handoffs, runbooks, explainers, reusable tools.

Each card could eventually show:

- status: planned / building / verified / maintained / retired;
- environment: owned lab / owned production / synthetic exercise;
- question being tested;
- evidence types captured;
- last verified date;
- resource note where meaningful (power, hardware reused, storage, complexity);
- explicit limitation.

The lifecycle labels are editorial framing, not a replacement for direct technical tags.

### Writing / Field Notes

Preserve pillar categories but add article-type cues:

- Field report
- Incident note
- Restore drill
- Build log
- Decision record
- Handoff/runbook
- Retrospective

An article should make the evidence path obvious: context → map → change → test → failure/limit → recovery/exit → claim.

### About

Connect the role transition to a practical philosophy without overstating identity:

- systems should be understandable by the people who depend on them;
- resilience includes recovery and documentation;
- privacy and local control are practical choices, not purity tests;
- the site records what Jeff can actually defend in an interview.

Do not present Jeff as an environmental engineer, sustainability expert, community-network organizer, or established solarpunk practitioner without corresponding work.

## Content voice pass

### Prefer

- “tend,” “repair,” “map,” “restore,” “right-size,” “keep,” “hand off,” and “leave an exit” when they describe real work;
- concrete household-scale outcomes;
- short annotations explaining tradeoffs;
- visible “what remains fragile” sections;
- invitations to inspect evidence rather than claims of mastery.

### Avoid

- “revolutionary,” “future-proof,” “planet-friendly,” “eco,” “green,” “sustainable,” or “resilient” without measurement;
- treating self-hosting as inherently ethical or efficient;
- implying local-first is always safer, cheaper, or simpler;
- generic AI language;
- mystical nature metaphors around security or reliability;
- replacing established technical terms with cute garden vocabulary inside commands, diagnostics, or interviews.

### Candidate framing (for later copy review, not approved copy)

- “Small systems, carefully tended.”
- “Map what exists. Repair what is fragile. Test the way back.”
- “Infrastructure practice at household and homelab scale.”
- “Field notes from learning to operate systems people can understand.”

These are direction markers, not final headlines.

## Solarpunk evidence layer

The makeover becomes substantive when selected artifacts can include honest resource and stewardship notes.

Possible fields, only when measured or known:

- hardware retained/reused versus newly purchased;
- idle/load power measurement and measurement method;
- service consolidation decision and tradeoff;
- expected maintenance cadence;
- replacement/repair availability;
- backup destination and restore evidence;
- export/decommission path;
- dependency on external cloud/vendor services;
- household usability impact;
- what would make the local option worse than a hosted option.

Do not require every artifact to report every field. Use only the fields relevant to the decision.

## Planned implementation slices after review

### Slice 0 — Reconcile parallel work

- inspect Claude’s final diff and branch state;
- identify new widgets, route changes, and style tokens;
- resolve conflicts between this direction and the completed implementation;
- decide what to keep, revise, or discard before writing an engineering plan.

**Gate:** No implementation plan until the review is complete.

### Slice 1 — Visual language prototypes

- create two or three static design studies outside production templates;
- compare “field notebook,” “civic infrastructure atlas,” and “repair workshop” directions;
- test day/night palettes and type roles;
- review at desktop and mobile sizes;
- select one direction and record rejected choices.

**Gate:** Jeff chooses a direction; no production CSS changes in the exploration step.

### Slice 2 — Token-only foundation

- translate the approved palette, type roles, spacing, border, and shadow decisions into existing theme tokens;
- preserve current component structure;
- verify contrast and system/light/dark behavior;
- compare screenshots before/after.

**Gate:** Visual change works without route or content restructuring.

### Slice 3 — Shared shell and homepage

- apply the approved motif to header, footer, background, focus states, and hero;
- update homepage hierarchy using only defensible existing content;
- add no new lab claims;
- update the footer date in the same site-update slice.

**Gate:** Mobile navigation, reduced motion, and no-JavaScript reading remain sound.

### Slice 4 — Portfolio evidence cards

- extend the data model only after the approved labs and status vocabulary are settled;
- distinguish planned from verified artifacts;
- expose evidence and limitations without bloating every card;
- add tests that prevent planned work from rendering as complete.

**Gate:** Every public status maps to a defined evidence requirement.

### Slice 5 — Article templates

- add optional article-type and evidence metadata only where current content supports it;
- style command output, diagrams, decisions, limits, and recovery notes consistently;
- migrate one strong article as the reference implementation before touching the archive.

**Gate:** Existing posts remain readable and old frontmatter remains valid.

### Slice 6 — Original illustration and texture pass

- add only the motifs proven useful in the prototypes;
- optimize local SVGs and provide accessible labels only when semantically meaningful;
- verify payload and rendering with images disabled;
- avoid decorating the GeistScope archive as current work.

**Gate:** The site still feels complete without illustration assets.

### Slice 7 — Whole-site claim and accessibility audit

- review ecological, sovereignty, privacy, reliability, and skill claims;
- run tests/build, HTML/accessibility checks, and responsive visual review;
- test light/dark/system, keyboard navigation, reduced motion, and print/readability;
- capture final before/after evidence and known limitations.

**Gate:** No release until claims and statuses agree with the real artifacts.

## Verification plan for the later implementation

Exact commands must be confirmed against the post-Claude repository state. At minimum, the finalized engineering plan should include:

- Rust unit/integration tests;
- formatting and linting gates already used by the repo;
- full application build;
- representative runtime checks for homepage, portfolio/labs, writing, and archive;
- automated or manual contrast checks for both themes;
- keyboard-only and reduced-motion review;
- mobile and desktop screenshots at stable widths;
- payload comparison for fonts, CSS, JS, SVG, and textures;
- assertion that planned labs are not presented as completed evidence;
- footer date check for any public site update.

Expected outputs should be recorded only after the actual commands and tooling are selected.

## Risks and countermeasures

| Risk | Countermeasure |
|---|---|
| Solarpunk becomes cosmetic greenwashing | Tie the design to measured stewardship fields and claim limits. |
| The site looks whimsical rather than employable | Keep evidence, technical terms, and operational status dominant. |
| Nature motifs reduce contrast or readability | Build palette/contrast checks into the first production slice. |
| New status vocabulary hides what is complete | Define evidence requirements and retain explicit text labels. |
| Self-hosting is implied to be universally better | Publish tradeoffs, maintenance burden, dependencies, and exit paths. |
| The makeover conflicts with Claude’s widgets work | Reconcile the completed diff before finalizing any implementation plan. |
| Scope expands into a full redesign | Approve one vertical slice at a time; tokens and shell before data-model changes. |
| Archive material appears re-endorsed | Keep archive styling and disclaimers clearly historical. |

## Decisions needed after Claude finishes

1. Should the visual anchor be field notebook, civic atlas, or repair workshop?
2. Should “Writing” remain literal or become “Field Notes”?
3. Should “Portfolio” remain, or become “Labs” only after the first verified lab?
4. How much of the current terminal/monospace identity should remain in evidence components?
5. Is the current Dracula/Catppuccin lineage worth preserving, or should the approved palette fully replace it?
6. Which two consulting-derived labs will actually be built first?
7. What owned hardware can be measured for power/resource notes?
8. Are illustrations desired, and should they be hand-authored SVG, generated then redrawn, or omitted?
9. Should ecological/resource notes appear on every eligible artifact or only in dedicated decision records?
10. What parts of Claude’s completed work become foundations versus candidates for revision?

## Explicit non-goals for this draft

- No changes to existing plans, code, templates, content, styles, routes, assets, tests, or project data.
- No final copy, token values, font choice, illustration assets, or route names.
- No representation that proposed labs or environmental benefits are already verified.
- No implementation before the parallel work is reviewed and this plan is revised and approved.
