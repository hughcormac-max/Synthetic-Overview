# `/plan` Workflow Trajectory

> **Workflow Command:** `/plan`
> **Purpose:** Inspect codebase dependencies, design technical contracts, and draft a structured `PLAN-XXX.md` specification in `.antigravity/plans/active/` before any code changes occur.

---

## 🎯 Workflow Execution Steps

```
+-------------------+     +-------------------------+     +-----------------------+
| 1. Research &     | --> | 2. Draft PLAN-XXX.md    | --> | 3. Executive Summary  |
|    Inspection     |     |    in plans/active/     |     |    & Human Approval   |
+-------------------+     +-------------------------+     +-----------------------+
```

### Phase 1: Research & Codebase Inspection

1. **Explore Existing Modules & Dependencies:**
   - Scan relevant source files, types, and existing tests in the target domain.
   - Check [.antigravity/rules/00-core-architecture.md](file:///.antigravity/rules/00-core-architecture.md) for layer constraints.
2. **Determine Plan ID:**
   - Check `.antigravity/plans/active/` and `.antigravity/plans/archive/index.md` to determine the next sequential ID (e.g., `PLAN-001`).

### Phase 2: Draft Plan Specification

1. **Initialize Plan File:**
   - Copy [.antigravity/plans/TEMPLATE.md](file:///.antigravity/plans/TEMPLATE.md) to `.antigravity/plans/active/PLAN-XXX.md`.
2. **Populate Core Sections:**
   - **Section 1 (Intent & Boundaries):** Define problem statement, explicit goals, and non-goals.
   - **Section 2 (Technical Contracts & Interfaces):** Write exact TypeScript/language interfaces, schemas, and public signatures.
   - **Section 3 (Implementation Steps):** Break down tasks into ordered, atomic checklist items with checkboxes (`- [ ]`).
   - **Section 4 (Verification & Criteria):** Define target unit tests, regression commands, and acceptance benchmarks.
   - **Section 5 (Deviations):** Leave placeholder for runtime pivots.

### Phase 3: Executive Summary & Review Gate

1. **Present in Chat:**
   - Summarize the intent, key contracts, affected files, and open questions directly to the user.
2. **Pause for Review:**
   - **STRICT HALT:** Do not begin any code implementation until the developer reviews and approves the plan.
