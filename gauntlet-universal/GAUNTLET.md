# Spec Gauntlet — Universal Edition

**What this is:** A portable multi-agent pipeline that produces AAA-quality
product specs for every feature in a project, verifies them blind against
user-defined quality criteria, and loops failures through remediation.

**Key difference from a project-specific gauntlet:** This version discovers the
project and interviews the user to build the quality criteria dynamically. It
works on any codebase, any platform, any domain.

**Platforms:** Claude Code CLI, Xcode Chat (26.3+), or any environment with
Claude's agent harness (subagents + file I/O).

**Invocation:**
> "Run the spec gauntlet. Read gauntlet-universal/GAUNTLET.md and follow it."

Or as a Claude Code slash command: `/gauntlet`

---

## Setup: First Run vs. Subsequent Runs

**First run:** The gauntlet runs Phase 0 (full discovery + criteria interview +
feature tree). This produces `gauntlet-output/criteria.md` and
`gauntlet-output/feature-tree.md`. These persist for future runs.

**Subsequent runs:** If `gauntlet-output/criteria.md` exists, ask the user:
> "I found existing quality criteria and a feature tree from a previous run.
> Use them as-is, update them, or start fresh?"

This lets the user iterate without repeating the interview.

---

## Phase 0: Discovery, Criteria, & Feature Tree

Phase 0 has three sub-steps that run sequentially. Each requires user input.

### Step 0.1 — Project Discovery

Scan the repo to understand what you're working with. Discover:

1. **Language & framework** — What's the stack? (Swift/SwiftUI, React, Rust,
   Python, etc.) Read config files: `Package.swift`, `package.json`,
   `Cargo.toml`, `pyproject.toml`, `build.gradle`, `*.xcodeproj`, etc.
2. **Architecture** — Where do views, models, services, and tests live? Map the
   directory structure.
3. **Existing docs** — Look for: README, AGENTS.md, CLAUDE.md, HANDOFF.md,
   CONTRIBUTING.md, architecture docs, product specs, PRDs. Read them.
4. **Current state** — Are there tests? CI? What's the build command? Any
   existing quality gates?

Summarize your findings to the user before proceeding:
> "Here's what I found: [stack], [architecture pattern], [N screens/routes/
> modules], [existing docs]. Does this look right? Anything I missed?"

### Step 0.2 — Criteria Interview

Build the project's quality criteria by interviewing the user. Ask these
questions in order, adapting based on what you learned in 0.1. Use a
conversational tone — this isn't a form, it's a dialogue.

#### Question 1: Platform & Design Standards

> "What platform design standards should specs target?"

Offer choices based on what you discovered:
- **iOS/iPadOS** → Apple Human Interface Guidelines
- **Android** → Material Design 3
- **Web** → No single standard — ask about framework conventions (e.g., Radix,
  shadcn/ui, Ant Design, custom design system)
- **Desktop (macOS)** → Apple HIG (macOS) or platform-specific
- **Desktop (Windows)** → Fluent Design / WinUI
- **Cross-platform** → Ask which platform is primary
- **CLI / API / backend-only** → Skip platform design lens; replace with API
  design best practices (REST maturity, error contracts, pagination, versioning)

If the user has a custom design system doc in the repo, offer to use it as the
standard.

#### Question 2: Quality Benchmarks

> "Name 2–5 apps or products you consider best-in-class for what you're
> building. These become the benchmark — when we say 'AAA quality,' these are
> the A's. What specifically makes each one good?"

Guide the user to be specific:
- Not just "Notion" but "Notion — keyboard shortcuts, slash commands, block
  flexibility, real-time collaboration"
- Not just "Linear" but "Linear — speed, keyboard-first, opinionated workflows,
  minimal UI"

If the user isn't sure, suggest well-known apps in their domain based on what
you discovered about the project. Research if needed.

#### Question 3: Competitive Landscape

> "Who are your direct competitors? For each, what do they do well and what do
> they miss?"

If the user names competitors, research them to build comparison criteria. If
the user doesn't have competitors or doesn't know, offer to research the space
based on the product description.

The output should be: for each competitor, 3–5 things they do well (that specs
should match or exceed) and 2–3 gaps (that represent differentiation
opportunities).

#### Question 4: Compliance, Safety & Domain Constraints

> "Are there any regulatory, legal, safety, or domain-specific constraints that
> specs must respect?"

Offer categories based on the project domain:
- **Healthcare/medical:** HIPAA, FDA (SaMD), clinical language restrictions,
  PHI handling, BAA requirements
- **Finance/payments:** PCI-DSS, SOX, financial advice disclaimers
- **Education:** COPPA, FERPA, age-gating
- **General consumer:** GDPR, CCPA/CPRA, accessibility (WCAG 2.1 AA),
  data retention/deletion
- **Enterprise/B2B:** SOC 2, SSO/SAML requirements, audit logging, data
  residency
- **Government:** FedRAMP, Section 508, ITAR
- **None identified:** Still enforce basic accessibility (WCAG 2.1 AA) and
  data safety (encryption, no plaintext credentials)

Also ask:
> "Are there any 'hard lines' — things that should automatically fail a spec
> if violated, regardless of other scores?"

These become auto-fail criteria (like PHI boundary was for SomaTrace).

#### Question 5: Lens Weighting & Priorities

> "I'm going to build three quality lenses from your answers. Here's what
> I'm thinking — tell me if the weights feel right:
>
> 1. **{Platform} Design Excellence** ({weight}%) — {brief description}
> 2. **Competitive Depth & Differentiation** ({weight}%) — {brief description}
> 3. **{Domain} Compliance & Safety** ({weight}%) — {brief description}
>
> Should any lens weigh more heavily? Should I split or merge any of them?
> Are there additional lenses I'm missing?"

The user might want:
- A fourth lens (e.g., "Developer Experience" for a library, "Performance" for
  a game, "Accessibility" as its own lens rather than embedded)
- Different weights (e.g., compliance at 60% for a healthcare app)
- A lens removed (e.g., no competitors for a novel product)

Adapt. The system supports 2–5 lenses.

### Step 0.3 — Build Criteria File

From the interview, generate `gauntlet-output/criteria.md` using the structure
in `CRITERIA-TEMPLATE.md`. This file becomes the grading standard for all
verification agents.

Show the generated criteria to the user:
> "Here are the quality criteria I built from our conversation. Review them —
> I need your sign-off before generating specs."

### Step 0.4 — Feature Tree

Same as the SomaTrace-specific version, but discovered from whatever codebase
you're in:

1. Scan source directories for screens, routes, views, modules, components.
2. Read any existing feature docs, PRDs, or roadmaps.
3. Build a hierarchical tree.
4. Present to the user for confirmation.
5. Write confirmed tree to `gauntlet-output/feature-tree.md`.

**Do not proceed to Phase 1 until the user confirms BOTH the criteria file AND
the feature tree.**

---

## Phase 1: Spec Generation

Identical dispatch pattern to the project-specific version, but the spec agent
prompt adapts to the project:

```
You are a spec agent in the Spec Gauntlet.

YOUR ASSIGNMENT: Write a complete product spec for: {feature name}
Feature ID: {feature-id}
Parent: {parent-id or "root"}

CONTEXT FILES TO READ FIRST:
1. gauntlet-universal/SPEC-TEMPLATE.md — your output template
2. gauntlet-output/criteria.md — your spec will be graded against this
3. {project's entry-point docs discovered in Phase 0}
4. {specific source files relevant to this feature}

OUTPUT: Write your completed spec to:
  gauntlet-output/specs/{feature-id}.md

RULES:
- Fill EVERY section of the template. Mark N/A with justification if inapplicable.
- Describe TARGET state first (ideal product), then GAP from current state.
- Read actual source files, not just docs. Docs orient; source confirms.
- Distinguish: implemented / prototyped / planned / gated / absent.
- Do not spawn sub-agents. Report sub-feature needs in §8 (Open Questions).
- All examples must use synthetic/test data if the project handles sensitive data.

QUALITY TARGET: Your spec will be blind-reviewed against the project's custom
criteria in gauntlet-output/criteria.md. Aim for 3s on every criterion.
```

### Dispatch strategy

- **Leaf feature:** One agent, one spec.
- **Branch with ≤ 3 children:** One agent covers parent + children in one doc.
- **Branch with > 3 children:** One agent per child; parent gets an umbrella spec.
- **Parallelism:** Up to 5 concurrent. Reduce to 2–3 on usage-limited plans.

### Manifest

Track in `gauntlet-output/manifest.md`:

```markdown
| Feature ID | Name | Status | Spec | Scorecard | Score | Iterations |
|---|---|---|---|---|---|---|
```

Status flow: `pending` → `spec-in-progress` → `spec-complete` →
`verify-in-progress` → `pass` / `fail` → `remediation-{n}` → `pass` / `escalated`

---

## Phase 2: Blind Verification

Same pattern — fresh agent, no access to spec agent's reasoning:

```
You are a verification agent in the Spec Gauntlet.

YOUR ASSIGNMENT: Blind-review this spec against the project's quality criteria.

SPEC TO REVIEW: gauntlet-output/specs/{feature-id}.md
GRADING CRITERIA: gauntlet-output/criteria.md
OUTPUT TEMPLATE: gauntlet-universal/SCORECARD-TEMPLATE.md

Read all three files now.

FEASIBILITY CHECK:
- Read the actual source files the spec references.
- Verify implementation claims are accurate.
- Check that dependencies exist and are version-compatible.

OUTPUT: gauntlet-output/scorecards/{feature-id}-scorecard.md

RULES:
- You are BLIND. Grade only what's in the spec file.
- Every score must cite a specific spec section or absence.
- Remediation notes must be specific enough for a different agent to act on.
- Apply all auto-fail rules from the criteria file.
```

---

## Phase 3: Remediation Loop

Same as project-specific version:

1. Read scorecard's remediation brief.
2. Spawn remediation agent targeting Priority 1 items.
3. Fresh blind verification of revised spec.
4. Max 3 loops, then escalate to user.

Escalations go to `gauntlet-output/gap-reports/{feature-id}-escalation.md`.

---

## Phase 4: Final Report

Generate `gauntlet-output/summary.md`:

```markdown
# Gauntlet Summary

**Project:** {name}
**Run date:** {date}
**Criteria version:** {hash or date of criteria.md}
**Features in tree:** {count}
**Specs generated:** {count}
**Passed first attempt:** {count} ({%})
**Passed after remediation:** {count}
**Escalated:** {count}

## Score Distribution
| Lens | Min | Max | Mean | Median |
|---|---|---|---|---|

## Common Failure Patterns
{Top 3–5 failing criteria with pattern analysis}

## Escalated Features
{List with one-line reason}

## Recommended Next Steps
{Pattern-based recommendations}
```

---

## User Checkpoints

The gauntlet requires user confirmation at exactly three points:

1. **Phase 0.3** — Criteria file sign-off.
2. **Phase 0.4** — Feature tree sign-off.
3. **Phase 3 escalations** — What to do with specs that fail 3 times.

Everything else is autonomous.

---

## Output Structure

```
gauntlet-output/
├── criteria.md              # Generated from interview (persists across runs)
├── feature-tree.md          # Confirmed feature tree
├── manifest.md              # Status tracker
├── summary.md               # Final report
├── specs/
│   └── {feature-id}.md      # One per feature
├── scorecards/
│   └── {feature-id}-scorecard.md
└── gap-reports/
    └── {feature-id}-escalation.md  # Only for 3x failures
```

---

## Installation

### Claude Code slash command
```bash
mkdir -p .claude/commands
cat > .claude/commands/gauntlet.md << 'EOF'
Read and follow gauntlet-universal/GAUNTLET.md. This is the Spec Gauntlet — a
multi-agent pipeline for AAA product spec generation with user-defined quality
criteria, blind verification, and remediation. Start with Phase 0 and proceed
sequentially. If gauntlet-output/criteria.md exists, ask whether to reuse or
rebuild it.
EOF
```

### Portable
Copy the `gauntlet-universal/` directory into any project repo. The gauntlet
adapts to whatever it finds.

### Partial runs
After confirming the tree, tell the orchestrator:
> "Run only on the {subtree name} branch."

### Incremental runs
Existing `pass` specs in the manifest are skipped. Re-run after changes without
repeating everything.
