# Criteria Template — Spec Gauntlet

**Instructions for the orchestrator:** Generate this file as
`gauntlet-output/criteria.md` during Phase 0.3 based on the user interview.
The structure below is a skeleton — fill in every section from the user's
answers. The generated file becomes the binding grading standard for all
verification agents.

---

```markdown
# {Project Name} — Quality Criteria

**Generated:** {date}
**Based on interview with:** {user name or role}
**Criteria version:** 1

---

## Scoring

Each criterion is graded 0–3:
- **0 — Missing:** Not addressed.
- **1 — Inadequate:** Addressed but wrong, superficial, or contradicts constraints.
- **2 — Acceptable:** Correct and functional; minor gaps.
- **3 — Excellent:** Would ship in a best-in-class product; no meaningful gap.

**Pass threshold:** No criterion scores 0. Every lens average ≥ 2.0. No more
than two criteria at 1 per lens.

---

## Auto-Fail Rules

{List rules from interview Q4. These override all other scoring. Examples:}
- {e.g., "Any spec that describes storing PII without encryption → automatic fail"}
- {e.g., "Any spec missing accessibility support → automatic fail"}
- {If user specified none: "No auto-fail rules beyond the standard pass threshold."}

---

## Lens 1: {Platform} Design Excellence (weight: {N}%)

**Standard:** {e.g., Apple HIG for iPadOS / Material Design 3 / custom design system}
**Benchmark apps:** {from interview Q2, e.g., "Things 3 (task clarity), Bear (typography)"}

### 1A. {Category name, e.g., "Layout & Navigation"}
{2–4 bullet criteria derived from the chosen standard and benchmarks.
Each bullet should be specific enough to grade yes/no or on a spectrum.}

### 1B. {Category name, e.g., "Interaction Design"}
{...}

### 1C. {Category name, e.g., "Visual Design"}
{...}

### 1D. {Category name, e.g., "Accessibility"}
{Minimum: WCAG 2.1 AA. Add platform-specific requirements.}

### 1E. {Category name, e.g., "Performance"}
{Specific targets: load time, frame rate, memory, payload sizes.}

### 1F. {Category name, e.g., "Error Handling & Edge Cases"}
{...}

---

## Lens 2: Competitive Depth & Differentiation (weight: {N}%)

**Competitors analyzed:** {from interview Q3}

### Competitive baseline
{For each competitor, list what they do well that specs must match:}

| Competitor | Strengths to match | Gaps to exploit |
|---|---|---|
| {name} | {strengths} | {gaps} |
| ... | ... | ... |

### 2A. {Category, e.g., "Feature Completeness"}
{...}

### 2B. {Category, e.g., "Workflow Efficiency"}
{Specific targets: task completion time, number of steps, etc.}

### 2C. {Category, e.g., "Differentiation"}
{What does this product do that competitors can't? Specs must articulate this.}

### 2D. {Category, e.g., "Domain Fit"}
{Domain-specific terminology, workflows, mental models the product must respect.}

### 2E. {Category, e.g., "Data Model & Extensibility"}
{...}

### 2F. {Category, e.g., "Onboarding & First Value"}
{Time-to-value target, learning curve expectations.}

---

## Lens 3: {Domain} Compliance & Safety (weight: {N}%)

**Governing standards:** {from interview Q4, e.g., "HIPAA Security Rule 2026,
GDPR Article 25, internal SOUL.md"}

### 3A. {Highest-priority safety criterion — the auto-fail candidate}
{e.g., "PHI Boundary," "PII Handling," "Financial Data Protection"}
{If this criterion = 0, the entire spec fails.}

### 3B. {e.g., "Language Safety" / "Legal Disclaimers" / "Content Moderation"}
{...}

### 3C. {e.g., "Privacy by Design" / "Data Minimization"}
{...}

### 3D. {e.g., "Asset Provenance" / "Licensing" / "IP Compliance"}
{...}

### 3E. {e.g., "Truthful Representation" / "No Overclaiming"}
{...}

### 3F. {e.g., "Error States in {Domain} Context"}
{What happens when things go wrong in a high-stakes context?}

### 3G. {e.g., "Accessibility as {Domain} Requirement"}
{Why accessibility is non-optional in this specific domain.}

---

## {Lens 4+: Optional additional lenses}

{If the user requested additional lenses in Q5, add them here with the same
structure: weight, standard, benchmark, and lettered criteria.}

---

## Scoring Summary

| Lens | Criteria count | Weight | Auto-fail conditions |
|---|---|---|---|
| {Lens 1 name} | {count} | {weight}% | {or "—"} |
| {Lens 2 name} | {count} | {weight}% | {or "—"} |
| {Lens 3 name} | {count} | {weight}% | {e.g., "3A = 0 → entire spec fails"} |
| {Lens 4+ if any} | {count} | {weight}% | {or "—"} |

Weights must sum to 100%.
```

---

**Note to orchestrator:** After generating the criteria file, present it to the
user for review. They may want to adjust weights, add criteria, remove criteria,
or change auto-fail rules. The criteria file is the contract — once confirmed,
verification agents treat it as law.
