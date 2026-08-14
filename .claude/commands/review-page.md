Grade shipped output against `gauntlet-output/criteria.md`.

This is the judgment half of the output gate. `tests/content_lint.rs` already
enforces what is mechanically checkable — frontmatter, tag vocabulary, section
scaffolding, link resolution, banned claim strings — and it runs in CI on every
commit. **Do not repeat those checks.** Your job is the part a test cannot do:
pedagogy, claim integrity, restraint, and whether the page is actually good.

## Target

`$ARGUMENTS` is one of:

- a slug (`osi-model`) — review `content/pages/<slug>.md`
- a path (`content/posts/hosting-machinageist-dev.md`)
- `--diff` — review every content file changed against `origin/main`
- empty — review every content file changed in the working tree, and if there
  are none, ask which page to review rather than reviewing all 23

## Before grading

1. Read `gauntlet-output/criteria.md` in full. It is the standard; nothing here
   restates it.
2. Read the target page in full.
3. Read `docs/agent-context/README.md` §6 for the page-authoring contract.
4. Run `cargo test --all-targets`. If the lint is already failing, say so and
   stop — mechanical defects get fixed before judgment is worth spending.

## Grade these criteria only

Shipped content cannot violate most of the architecture-facing criteria, so
grading them produces noise. Score 0–3 on exactly these, using `criteria.md`'s
own definitions:

| Lens | Criteria | What you are actually looking for on a page |
|---|---|---|
| 1. Claim integrity | 1A, 1B, 1C, 1D, 1E, 1F | Every capability claimed is backed by published evidence; planned work never reads as done; no stale cert or role framing |
| 2. Design & craft | 2B, 2C, 2D, 2E | **2C is the one that matters most here.** Concept before jargon, built from the ground up, connected to the larger system, practice on hardware the reader owns. Bullet-dumping a source note scores ≤ 1 |
| 3. Accessibility | 3B, 3D, 3F | Heading outline is real; tables and code blocks survive a narrow viewport; nothing depends on colour alone |
| 4. Competitive depth | 4A, 4C | Does this teach, or restate? Would a working engineer respect the explanation? |
| 5. Accuracy | 5E | If the page changes shipped behaviour, does it say which long-lived document must change with it |

Apply all three auto-fail rules. Rule 1 (unearned claims) is the one that fires
on content.

## Verify before you score

Claim integrity is not a reading-comprehension exercise. For every capability,
tool, or result the page asserts:

- find the evidence in `content/posts/`, and cite it `file:line`;
- if the page claims something a post calls *planned* or *absent*, that is
  auto-fail rule 1, not a Priority 2 nit;
- check technical assertions against the primary sources the page itself cites.
  A page that cites RFC 4291 and then states the multicast prefix wrong is worse
  than one that cites nothing.

An overclaim shipped on this site in exactly this way until 2026-08-14: `/about`
named five capabilities the blog posts recorded as absent or planned. The
contradiction was one click away. Look for that shape.

## Output

Follow `gauntlet-universal/SCORECARD-TEMPLATE.md` so the result is comparable
with the gauntlet's own scorecards. Per criterion: the score, the evidence
(quote the page, cite `file:line`), and — where the score is below 3 — a
remediation note specific enough for someone else to act on without rereading
the page.

Then:

- **Verdict:** PASS / FAIL against `criteria.md`'s pass threshold.
- **Priority 1** — auto-fail violations and factual errors. These block.
- **Priority 2** — pedagogy and craft gaps worth fixing.
- **Priority 3** — polish.

Write the scorecard to `gauntlet-output/scorecards/page-<slug>-scorecard.md`
only if asked; otherwise report inline.

## Rules

- Cite or do not claim. Every score names the specific passage it is about.
- Do not soften a Priority 1 into a suggestion. An unearned claim on a portfolio
  site aimed at getting hired is the most expensive defect available.
- Do not rewrite the page unless asked. Report, then wait.
- Voice is the author's. "I would phrase this differently" is not a finding;
  "this states a capability the evidence does not support" is.
