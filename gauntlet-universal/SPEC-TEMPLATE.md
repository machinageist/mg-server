# Spec Template — Spec Gauntlet (Universal)

**Instructions for spec agents:** Copy this template into
`gauntlet-output/specs/{feature-id}.md` and fill every section. If a section is
not applicable, write "N/A — {reason}" rather than deleting it.

Before writing, read `gauntlet-output/criteria.md` to understand what your spec
will be graded against.

---

```markdown
# Spec: {Feature Name}

**Feature ID:** {kebab-case-id}
**Parent feature:** {parent-id or "root"}
**Spec author agent:** {agent identifier}
**Date:** {ISO date}
**Iteration:** {1 = first draft, incremented on remediation}

---

## 1. Purpose

### 1.1 One-sentence job
What does this feature do for the user, stated as a job-to-be-done?

### 1.2 Why it matters
Why does this feature exist in this product specifically? What user pain does
it address?

### 1.3 Success signal
How would you know this feature is working? One measurable or observable outcome.

---

## 2. User Stories

3–7 user stories covering:
- Happy path
- Edge case / error state
- Accessibility scenario
- (If applicable) Admin / secondary persona scenario

Format:
> As a {role}, I want {action}, so that {outcome}.

---

## 3. UX Specification

### 3.1 Screen / view inventory
List every screen, modal, sheet, popover, drawer, or panel this feature
introduces or modifies. For each:
- Name and navigation path to reach it
- New vs. modification of existing
- Layout pattern (sidebar, full-screen, modal, etc.)

### 3.2 Interaction flows
Primary flow step by step. Include branching for errors and edge cases.
Note haptic/sound/animation cues.

### 3.3 Layout descriptions
For each new view, describe component placement in enough detail to build
without ambiguity:
- Component hierarchy (top → bottom, leading → trailing)
- Component types (list, form, canvas, map, toolbar, etc.)
- Data sources (which model/store/state drives each component)
- Empty state appearance and copy

### 3.4 Input & gestures
- Touch / click / keyboard interactions
- Specialized input (stylus, game controller, voice, camera)
- Keyboard shortcuts (desktop/laptop)
- Responsive behavior across screen sizes

### 3.5 Transitions & animation
- Navigation transitions
- In-view state change animations
- Reduced-motion alternatives

### 3.6 Error states
For every error condition:
- Trigger
- User-visible presentation (inline, banner, toast, modal — justify the choice)
- Recovery path
- Data loss risk (yes/no/partial)

### 3.7 Accessibility
- Screen reader labels, hints, and traits for every interactive element
- Custom actions for complex interactions
- Text scaling / dynamic type behavior
- Color-independent state communication
- Focus order and keyboard navigability

---

## 4. Implementation Specification

### 4.1 Architecture placement
Where does this feature live in the project's module structure? Reference actual
directories and modules from the codebase.

### 4.2 Data model
New or modified types. Write them in the project's language with doc comments.
Include database migrations if applicable.

### 4.3 API contracts
New or modified endpoints / functions / interfaces:
- Signature (method, path, params, return type)
- Error cases
- Auth / permission requirements
- Pagination / rate limiting if applicable

### 4.4 State management
- Which store / controller / view model owns this state?
- New state container needed? Responsibilities and injection point.
- Local vs. server-synced state boundaries.
- Offline / draft persistence strategy.

### 4.5 Dependencies
- New packages / libraries / frameworks
- New assets or resources
- Infrastructure changes (database, CDN, third-party services)

### 4.6 Platform-specific considerations
- Renderer, engine, or framework migration concerns
- Version compatibility (OS versions, browser support, etc.)
- Feature flags or gradual rollout needs

### 4.7 Performance budget
- Memory impact
- CPU / render-time impact
- Network payload sizes
- Storage impact (client + server)
- Startup time impact

---

## 5. Test Specification

### 5.1 Unit tests
Test cases for core logic. For each: name, setup, assertion, edge case covered.

### 5.2 Integration tests
End-to-end or API round-trip tests.

### 5.3 UI / E2E tests
Automated UI test scenarios: navigation, happy path, error recovery.

### 5.4 Visual / manual verification
Configurations to check visually:
- Theme variants (light/dark)
- Text size extremes
- Screen size extremes
- Empty vs. populated states

---

## 6. Compliance & Safety Gate

### 6.1 Sensitive data classification
Does this feature touch, store, transmit, or display sensitive data?
- [ ] No sensitive data involvement
- [ ] Handles sensitive data — describe protection measures
- [ ] Uses synthetic/test data only until compliance gate clears

### 6.2 Asset provenance
Does this feature use third-party assets (models, images, data, fonts, etc.)?
- [ ] No third-party assets
- [ ] Uses third-party assets — list each with source, license, and rights status

### 6.3 Language / claims audit
Does any user-visible text in this spec:
- [ ] Make claims not supported by evidence? (MUST NOT unless flagged)
- [ ] Promise capabilities not yet built? (MUST NOT)
- [ ] Use language restricted by domain regulations? (MUST NOT)

### 6.4 Regulatory alignment
Reference specific criteria from `gauntlet-output/criteria.md` Lens 3 and
confirm each is addressed.

---

## 7. Gap Analysis vs. Current State

### 7.1 What exists today
Current implementation state. Reference specific files, commits, or tests.
Use correct state: implemented / prototyped / planned / gated / absent.

### 7.2 Delta to spec
Itemized list of changes needed:
- New files / modules
- Modified files
- Migrations / schema changes
- New dependencies

### 7.3 Estimated scope
T-shirt size (S/M/L/XL) with justification.

### 7.4 Blocking dependencies
What must land first? Reference other feature IDs or external gates.

---

## 8. Open Questions

Unresolved items for the user to decide:
- **Q1:** {question} — blocks: {section}
- **Q2:** ...
```

---

**End of template.** Verification agents grade against
`gauntlet-output/criteria.md`. Empty sections (without N/A) score 0.
