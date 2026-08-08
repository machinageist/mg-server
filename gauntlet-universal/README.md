# Spec Gauntlet — Universal

A portable multi-agent pipeline for AAA product spec generation, blind
verification, and remediation. Works on any project, any platform, any domain.

## How it works

1. **You drop this directory** into a project repo.
2. **You invoke it** (`/gauntlet` or "Run the spec gauntlet").
3. **The agent interviews you** to build quality criteria tailored to your
   project — what platforms standards to follow, which competitors to benchmark,
   what compliance rules to enforce, which criteria are auto-fail.
4. **It discovers features** from your codebase and docs, presents a tree.
5. **Subagents write specs** for every feature against your criteria.
6. **Blind verification agents** grade each spec independently.
7. **Failures loop back** with specific remediation notes, up to 3 times.
8. **You get a final report** with scores, patterns, and escalations.

## Files

| File | Purpose | Who reads it |
|---|---|---|
| `GAUNTLET.md` | Master orchestration prompt | The orchestrating agent |
| `CRITERIA-TEMPLATE.md` | Skeleton for building project-specific criteria | Orchestrator (Phase 0) |
| `SPEC-TEMPLATE.md` | Output format for spec agents | Spec agents |
| `SCORECARD-TEMPLATE.md` | Output format for verification agents | Verification agents |

## Output

The pipeline produces `gauntlet-output/` in your project root:

```
gauntlet-output/
├── criteria.md          ← Your quality standard (persists across runs)
├── feature-tree.md      ← Confirmed feature tree
├── manifest.md          ← Status of every feature
├── summary.md           ← Final report
├── specs/               ← One spec per feature
├── scorecards/          ← One scorecard per spec
└── gap-reports/         ← Escalations (3x failures)
```

## Quick start

```bash
# Copy into your project
cp -r gauntlet-universal/ your-project/gauntlet-universal/

# Install as Claude Code slash command
cd your-project
mkdir -p .claude/commands
echo 'Read and follow gauntlet-universal/GAUNTLET.md.' > .claude/commands/gauntlet.md

# Run it
claude
> /gauntlet
```

## Adapts to

- **iOS / iPadOS** — Apple HIG, SwiftUI, Xcode Previews verification
- **Android** — Material Design 3, Jetpack Compose
- **Web** — Any framework; uses your design system if present
- **Backend / API** — Swaps design lens for API design quality
- **CLI tools** — Swaps design lens for UX conventions (help text, flags, exit codes)
- **Libraries** — Swaps design lens for developer experience (API surface, docs, types)
- **Games** — Adds performance lens; swaps design for game-feel benchmarks
- **Healthcare, finance, education, government** — Compliance lens adapts to domain

## vs. the SomaTrace-specific version

The `SomaTraceApp/gauntlet/` directory contains a pre-configured version with
SomaTrace's criteria baked in (Apple HIG, Jane App/Noterro benchmarks, HIPAA
compliance). Use that for SomaTrace work. Use this universal version for
everything else — or as a starting point for building another project-specific
gauntlet after the first run.
