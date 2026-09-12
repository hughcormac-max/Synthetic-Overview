---
name: plan
description: Inspect codebase dependencies, design technical contracts, and draft a structured PLAN-XXX.md specification in active plans before any code changes occur.
---
# `/plan` Workflow Trajectory

> **Workflow Command:** `/plan`
> **Purpose:** Inspect codebase dependencies, design technical contracts, and draft a structured `PLAN-XXX.md` specification in `.agents/plans/active/` before any code changes occur.

---

## 🎯 Workflow Execution Steps

### Phase 1: Research & Codebase Inspection

1. **Explore Existing Modules & Dependencies:**
   - Scan relevant source files, types, and existing tests in the target domain.
   - Check [AGENTS.md](../../../AGENTS.md) for layer constraints and coding standards.
2. **Determine Plan ID:**
   - Check `.agents/plans/active/` and `.agents/plans/archive/INDEX.md` to determine the next sequential ID (e.g., `PLAN-001`).

### Phase 2: Draft Plan Specification

1. **Initialize Plan File:**
   - Copy [.agents/skills/plan/resources/TEMPLATE.md](resources/TEMPLATE.md) to `.agents/plans/active/PLAN-XXX.md`.
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
