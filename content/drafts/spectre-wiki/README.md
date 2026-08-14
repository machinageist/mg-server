# Spectre-Seq wiki — unpublished draft set

> **DRAFT / UNPUBLISHED.** These files are source material, not part of the public
> education wiki. The server does not scan or route this directory.
>
> **Last synchronized:** 2026-08-06 against accepted Spectre product, requirements,
> architecture, roadmap, and status documents. R0 and R1 are exited; R2 remains the
> active milestone and is described as in progress until its full workspace gate is
> reviewed.

## Purpose

This directory stages an educational wiki about Spectre, an open-source digital audio
workstation in development. It is deliberately kept outside the site's publication
surfaces while the product, evidence, and explanatory material mature.

The draft set begins with:

- [Overview](index.md)
- [Architecture](architecture.md)
- [Roadmap](roadmap.md)
- [Quality and validation](quality-and-validation.md)
- [Clean-room references](clean-room-references.md)

## Publication boundary

Everything in this directory is unpublished. In the current site, wiki handlers read
only `content/pages`, and only slugs in the hard-coded wiki sidebar are accepted.
Drafts are not a fallback content source.

**Warning:** moving or copying one of these files into `content/pages/` and adding its
slug to the sidebar makes it reachable through the public Learn routes. Promotion must
therefore be an intentional, reviewed change. Do not add draft links to navigation,
templates, page lists, the sitemap, feeds, tests, or routing as a shortcut.

## Source and authority order

Wiki claims follow the Spectre repository's own authority order:

1. accepted product vision in `docs/00-product/`;
2. accepted requirements and decisions in `docs/01-requirements/`;
3. accepted architecture contracts in `docs/03-architecture/`;
4. accepted detailed specifications in `docs/04-specs/`, when present;
5. accepted quality contracts in `docs/05-quality/`, when present;
6. roadmap and current-milestone plans in `docs/06-plans/`;
7. dated implementation evidence in `docs/status/`;
8. draft reference research in `docs/02-reference-research/`.

Implementation can prove behavior, but it does not silently redefine accepted product
requirements. Reference research can inform a candidate, but only the requirements
ledger can turn it into a Spectre requirement.

Primary synchronization pointers:

- `docs/README.md`
- `docs/00-product/vision.md`
- `docs/01-requirements/requirements-ledger.md`
- `docs/01-requirements/traceability.md`
- `docs/01-requirements/decision-gates.md`
- `docs/03-architecture/`
- `docs/06-plans/rebuild-roadmap.md`
- `docs/06-plans/current-milestone.md`
- `docs/status/STATUS.md`
- `docs/status/NEXT.md`
- `docs/02-reference-research/methodology.md`

## Freshness policy

Every page carries a **Last synchronized** date. Product direction may be stated from
accepted documents without implying implementation. A current implementation claim
must also include its verification date or point to a dated status/evidence record.

Before editing a current-state paragraph:

1. read the authority map and accepted vision;
2. read the current milestone, status, and next-work documents;
3. compare the claim with traceability and the relevant architecture contract;
4. downgrade disputed, targeted-only, or not-yet-reviewed work to **in progress**;
5. preserve explicit gaps instead of inferring an unfinished feature;
6. update the synchronization date only after reviewing all affected sources.

A stale page remains a draft; it must not be promoted merely because its prose reads
smoothly.

## Promotion checklist

A page may move toward publication only when all of the following are true:

- [ ] Spectre's owner approves the page and its public timing.
- [ ] Every product and implementation claim follows the current authority order.
- [ ] Current-state claims have a date and pass the cited evidence gate.
- [ ] Planned, proposed, implemented, and verified behavior are visibly distinct.
- [ ] The page makes no parity, compatibility, prevalence, or release-readiness claim
      that the sources do not support.
- [ ] Reference-product material passes the clean-room methodology and source review.
- [ ] Links and terminology are reviewed for public context; repository-only pointers
      are either made useful or replaced with durable public citations.
- [ ] Accessibility, editorial, legal/licensing, and claim-accuracy reviews are complete
      for the page's scope.
- [ ] The destination filename, public slug, sidebar label, and tests are updated in one
      deliberate publication change.
- [ ] Route, sitemap, feed, and rendered-page checks pass after promotion.

Until every applicable box is closed, these files stay here and remain unrouted.
