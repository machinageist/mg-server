# Scorecard Template — Spec Gauntlet (Universal)

**Instructions for verification agents:** You are performing a BLIND review.
You have not seen the spec agent's reasoning — only the finished spec.

Before grading, read:
1. The spec file
2. `gauntlet-output/criteria.md` — the grading standard
3. This template — your output format

Grades must cite specific spec sections. Vague justifications are rejected.

---

```markdown
# Scorecard: {Feature Name}

**Feature ID:** {matching the spec}
**Spec file:** gauntlet-output/specs/{feature-id}.md
**Reviewer agent:** {agent identifier}
**Date:** {ISO date}
**Spec iteration reviewed:** {matches spec's iteration}

---

## Verdict: {PASS | FAIL}

**Summary:** {2–3 sentences. Strongest quality and most critical gap. If FAIL,
the single most important fix.}

---

## {Lens 1 Name} (weight: {N}%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| {1A name} | | {cite spec section + finding} | {specific fix or "—"} |
| {1B name} | | | |
| {1C name} | | | |
| {1D name} | | | |
| {1E name} | | | |
| {1F name} | | | |

**Lens average:** {calculated}
**Lens pass:** {Yes/No — avg ≥ 2.0, ≤ two 1s, no 0s}

---

## {Lens 2 Name} (weight: {N}%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| {2A name} | | | |
| {2B name} | | | |
| {2C name} | | | |
| {2D name} | | | |
| {2E name} | | | |
| {2F name} | | | |

**Lens average:** {calculated}
**Lens pass:** {Yes/No}

---

## {Lens 3 Name} (weight: {N}%)

| Criterion | Score (0–3) | Evidence from spec | Remediation needed |
|---|---|---|---|
| {3A name} | | | |
| {3B name} | | | |
| {3C name} | | | |
| {3D name} | | | |
| {3E name} | | | |
| {3F name} | | | |
| {3G name} | | | |

**Lens average:** {calculated}
**Lens pass:** {Yes/No}
**Auto-fail triggered:** {Yes/No — check criteria.md for auto-fail rules}

---

{## Lens 4+ (if criteria.md defines additional lenses, add them here)}

---

## Feasibility Check

Read the actual source files referenced in the spec before filling this table.

| Check | Status | Notes |
|---|---|---|
| Types/models exist or are clearly specified | {✓/✗} | |
| API/interface changes are feasible with current architecture | {✓/✗} | |
| Views/screens fit current navigation pattern | {✓/✗} | |
| Dependencies are available and version-compatible | {✓/✗} | |
| Platform/renderer requirements are realistic | {✓/✗} | |
| Test strategy is executable with current infrastructure | {✓/✗} | |
| Performance budget is realistic for target hardware | {✓/✗} | |
| No undeclared dependency on unbuilt features | {✓/✗} | |

**Feasibility verdict:** {Feasible / Feasible with caveats / Infeasible}
**Caveats:** {if any}

---

## Composite Score

| Lens | Average | Weight | Weighted |
|---|---|---|---|
| {Lens 1} | {avg} | {weight} | {avg × weight} |
| {Lens 2} | {avg} | {weight} | {avg × weight} |
| {Lens 3} | {avg} | {weight} | {avg × weight} |
| {Lens 4+ if any} | {avg} | {weight} | {avg × weight} |
| **Composite** | | | **{sum}** |

**Pass conditions (from criteria.md):**
- [ ] Composite ≥ 2.0
- [ ] All lens averages ≥ 2.0
- [ ] No criterion scores 0
- [ ] No more than two criteria at 1 per lens
- [ ] All auto-fail rules pass
- [ ] Feasibility ≠ Infeasible

**All conditions met:** {Yes → PASS / No → FAIL}

---

## Remediation Brief (FAIL only)

### Priority 1 — Must fix to pass
{Numbered list. Specific changes referencing spec sections. A different agent
must be able to act on these without clarification.}

### Priority 2 — Should fix for quality
{Would raise scores but not blocking.}

### Priority 3 — Consider for excellence
{Polish items: 2 → 3.}
```

---

**End of template.** Return the completed scorecard to the orchestrator.
