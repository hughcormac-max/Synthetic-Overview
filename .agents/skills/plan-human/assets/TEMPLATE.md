---
id: PLAN-NNNN
title: "[Short, Descriptive Title]"
status: draft # draft | approved | in-progress | completed | superseded | abandoned
author: "[Author / Agent Name]"
created: YYYY-MM-DD
updated: YYYY-MM-DD
completed_at: "" # Populated upon completion by /archive-plan
branch: "[branch-name]"
---

# PLAN-NNNN: [Short, Descriptive Title]

> **Status:** `draft` | **Created:** YYYY-MM-DD | **Last Updated:** YYYY-MM-DD
> **Author:** [Author / Agent Name] | **Branch:** [branch-name]
> **Permanent Location:** `.agents/docs/plans/PLAN-NNNN.md`

---

# 🧠 PART 1: HUMAN PROBLEM ALIGNMENT
*(Authored via `/plan-human` — Aligns user-facing intent, boundaries, and acceptance criteria prior to technical design)*

## 🎯 1. Problem Statement & Context
[Describe the core problem, user requirement, or friction being addressed from the user or developer perspective.]

## 🚀 2. Core Objectives & Capabilities
- [Objective 1: What concrete capability will be unlocked?]
- [Objective 2: What user workflow is improved or enabled?]

## 🛑 3. Non-Goals & Exclusions
- [Non-Goal 1: What is explicitly out of scope for this plan?]
- [Non-Goal 2: What follow-ups are deferred to future plans?]

## 👥 4. User Stories & Interaction Journeys
- **As a:** [type of user]
- **I want to:** [perform an action]
- **So that:** [achieve a specific outcome]

*Key interaction walkthrough:*
1. User invokes...
2. System provides...
3. Result is verified by...

## ✅ 5. Acceptance Criteria
- [ ] [Criterion 1: Observable behavior defining success]
- [ ] [Criterion 2: Observable behavior defining success]

## ⚠️ 6. Edge Cases & Boundary Behaviors
- **Empty State / Zero Inputs:** [How does the system respond?]
- **Invalid Input / Unauthorized:** [What error message or recovery is presented?]
- **Boundary Thresholds:** [Upper limits, time-outs, rate limits]

---

# 🤖 PART 2: ROBOT TECHNICAL SPECIFICATIONS
*(Authored via `/plan-robot` — Defines contracts, interfaces, and pseudocode-driven atomic tasks)*

## 📚 7. Authoritative Domain References (Tier 0)
*Downlink to immutable domain specifications, formulas, constants, and truth tables in `.agents/docs/ssot/`:*
- [SSOT-NNNN: Domain SSOT Title](../../ssot/SSOT-NNNN.md) — *[Specify applicable formulas, constants, decision trees, or golden vectors cited from this SSOT]*

## 🏗️ 8. Domain Types & Data Contracts
```typescript
// Define explicit domain models, schemas, and contracts here
export interface ExampleDomainContract {
  readonly id: string;
  readonly createdAt: number;
  readonly status: 'pending' | 'active' | 'completed';
}
```

## 🔌 9. Public API / Service Signatures
```typescript
// Define service interfaces, function signatures, and Result types
export interface ExampleService {
  executeTask(payload: ExampleDomainContract): Promise<Result<void, DomainError>>;
}
```

## 📂 10. File-by-File Module Mapping
- **Domain Layer:** `src/core/...`
- **Application Layer:** `src/services/...`
- **Infrastructure Layer:** `src/adapters/...`
- **Presentation Layer:** `src/components/...`

## ⚙️ 11. Pseudocode-Driven Atomic Logical Tasks
*Ordered checklist broken down into logical tasks (e.g., specific functions, components). **MANDATORY INVARIANT: Every task MUST include explicit pseudocode detailing the algorithmic steps to be implemented.***

- [ ] **Task 1: Data Contracts & Schemas** *(Target: `src/core/types.ts`)*
  - **Pseudocode:**
    ```text
    Define interface ExampleDomainContract with fields id, createdAt, status.
    Ensure fields are readonly.
    ```
- [ ] **Task 2: Pure Domain Pipeline** *(Target: `src/core/calculator.ts`)*
  - **Pseudocode:**
    ```text
    Function calculate_trajectory(input: Data):
      If input is null, return Error
      result = apply SSOT-NNNN formula to input
      Return result
    ```
- [ ] **Task 3: Domain Unit Tests** *(Target: `tests/core/calculator.test.ts`)*
  - **Pseudocode:**
    ```text
    Test case 1: Nominal path using golden vector.
    Test case 2: Empty inputs return explicit error.
    ```
- [ ] **Task 4: Full Regression & Verification**
  - [ ] Run typecheck.
  - [ ] Run test suite with 100% pass rate.

## 🧪 12. Unit Test Targets & Verification Commands

### 12.1 Unit Test Coverage Targets
| Module / File | Test File | Key Scenarios Covered |
| :--- | :--- | :--- |
| `src/core/example.ts` | `tests/core/example.test.ts` | Nominal path, empty inputs, boundary thresholds, golden vector validation |

### 12.2 Verification Commands
```bash
# Typecheck
npm run typecheck

# Lint
npm run lint

# Test Suite
npm test
```

---

# 📝 PART 3: VERIFICATION & RETROSPECTIVE
*(Updated during execution and closed via `/archive-plan`)*

## 🔧 13. Runtime Deviations & Architectural Pivots
- *[None logged during drafting. Document any runtime adjustments here.]*

## 🏁 14. Retrospective & Follow-Up Items
- *[None logged during drafting. Document lessons learned and follow-ups here.]*
