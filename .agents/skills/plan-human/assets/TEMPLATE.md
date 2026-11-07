---
id: PLAN-<slug>
title: "[Short, Descriptive Title]"
project: "[[YYMMDD_ORG_ProjectName]]"
status: draft # draft | approved | in-progress | completed
author: "[Author / Agent Name]"
created: YYYY-MM-DD
updated: YYYY-MM-DD
branch: "[branch-name]"
---

# PLAN-<slug>: [Short, Descriptive Title]

> **Status:** `draft` | **Created:** YYYY-MM-DD | **Last Updated:** YYYY-MM-DD
> **Project Node:** `[[YYMMDD_ORG_ProjectName]]` | **Branch:** `[branch-name]`
> **Staged Location:** `.agents/work/PLAN-<slug>.md` *(Exported & purged via `/close-to-palace` upon completion)*

---

# 🧑‍💻 PART 1: HUMAN PROBLEM ALIGNMENT
*(Authored via `/plan-human` — Aligns user-facing intent, boundaries, and acceptance criteria prior to technical design)*

## 🎯 1. Problem Statement & Context
[Describe the core problem, user requirement, or friction being addressed from the user or developer perspective.]

## 🚀 2. Core Objectives & Capabilities
- [Objective 1: What concrete capability will be unlocked?]
- [Objective 2: What user workflow is improved or enabled?]

## 🚫 3. Non-Goals & Exclusions
- [Non-Goal 1: What is explicitly out of scope for this plan?]
- [Non-Goal 2: What follow-ups are deferred to future sessions?]

## 👤 4. User Stories & Interaction Journeys
- **As a:** [type of user]
- **I want to:** [perform an action]
- **So that:** [achieve a specific outcome]

*Key interaction walkthrough:*
1. User invokes...
2. System provides...
3. Result is verified by...

## ✅ 5. Acceptance Criteria
*Use EARS syntax: "WHEN [trigger], the system SHALL [behavior]"*
- [ ] WHEN [trigger], the system SHALL [observable behavior]
- [ ] IF [error condition], THEN the system SHALL [recovery behavior]

## ⚠️ 6. Edge Cases & Boundary Behaviors
- **Empty State / Zero Inputs:** [How does the system respond?]
- **Invalid Input / Unauthorized:** [What error message or recovery is presented?]
- **Boundary Thresholds:** [Upper limits, time-outs, rate limits]

---

# 🤖 PART 2: ROBOT TECHNICAL SPECIFICATIONS
*(Authored via `/plan-robot` — Defines Memory Palace spec citations, interfaces, and pseudocode-driven atomic implementation tasks)*

## 📐 7. Authoritative Memory Palace & Research References
*Cite standing specifications in `G:\My Drive\06_Memory Palace\Knowledge\Distilled\` (`Design_And_Development/`, `Research_And_Review/`) or staged `.agents/work/RESEARCH-*.md` spikes:*
- `[[Design_And_Development_Node_Name]]` — *[Specify applicable rules, plain-text formulas, constants, or invariants cited from this node]*
- `[[Research_And_Review_Node_Name]]` — *[Specify supporting domain derivations or trade-off conclusions]*

## 🧩 8. Domain Types & Data Contracts
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

## 🗺️ 10. File-by-File Module Mapping
- **Domain Layer:** `src/core/...`
- **Application Layer:** `src/services/...`
- **Infrastructure Layer:** `src/adapters/...`
- **Presentation Layer:** `src/components/...`

## 🛠️ 11. Atomic Logical Tasks (PSEUDOCODE DRIVEN)
*Ordered checklist broken down into logical components (e.g., a specific function, class, or schema). **MANDATORY INVARIANT: Each step must include strict pseudocode or algorithmic steps for the Implementer to follow.***

- [ ] **Step 1: Data Contracts & Schemas** *(Target: `src/core/types.ts`)*
  - **Pseudocode Spec:**
    ```text
    Define ExampleDomainContract interface
    Include id, createdAt, and status fields
    ```
- [ ] **Step 2: Pure Domain Pipeline** *(Target: `src/core/calculator.ts`)*
  - **Pseudocode Spec:**
    ```text
    Create function calculateTransformation(input: ExampleDomainContract)
    Apply formula from [[Design_And_Development_Node_Name]]
    Return Result object (success/failure)
    ```
- [ ] **Step 3: Domain Unit Tests** *(Target: `tests/core/calculator.test.ts`)*
  - **Pseudocode Spec:**
    ```text
    Test 1: Nominal path with valid input
    Test 2: Edge case where input status is 'pending'
    ```
- [ ] **Step 4: Full Regression & Verification**
  - [ ] Run typecheck (`strict: true`).
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

# 📋 PART 3: EXECUTION DEVIATIONS & PALACE DISTILLATION NOTES
*(Updated during `/execute` and packaged into `06_Memory Palace\Inbox\Context_Dumps\` via `/close-to-palace`)*

## 📝 13. Runtime Deviations & Architectural Pivots
- *[None logged during drafting. Document any runtime adjustments here during `/execute`.]*

## 🏛️ 14. Memory Palace Distillation Directives
- **New/Updated `Design_And_Development/` Specs:** *[Note whether this plan introduces a new spec node, accretes onto `[[Existing_Node]]`, or supersedes `[[Old_Node]]`]*
- **Follow-Up Action Items (`Episodic/Projects/`):** *[List remaining `- [ ]` next steps to record in the session project log]*
