---
id: PLAN-NNNN
title: "[Short, Descriptive Title]"
status: draft # draft | approved | in-progress | completed | superseded | abandoned
author: "[Author / Agent Name]"
created: YYYY-MM-DD
updated: YYYY-MM-DD
completed_at: "" # Populated upon completion by /plan-archive
branch: "[branch-name]"
---

# PLAN-NNNN: [Short, Descriptive Title]

> **Status:** `draft` | **Created:** YYYY-MM-DD | **Last Updated:** YYYY-MM-DD
> **Author:** [Author / Agent Name] | **Branch:** [branch-name]
> **Permanent Location:** `.agents/docs/plans/PLAN-NNNN.md`

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
- [Non-Goal 2: What follow-ups are deferred to future plans?]

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
*(Authored via `/plan-robot` — Defines contracts, interfaces, and pseudocode-driven atomic implementation tasks)*

## 📐 7. Authoritative Domain References (Tier 0)
*Downlink to immutable domain specifications, formulas, constants, and truth tables in `.agents/docs/ssot/`:*
- [SSOT-NNNN: Domain SSOT Title](../../ssot/SSOT-NNNN.md) — *[Specify applicable formulas, constants, decision trees, or golden vectors cited from this SSOT]*

## 🧩 8. Domain Types & Data Contracts
```rust
// Define explicit domain models, ECS components, schemas, and contracts here
#[derive(Debug, Clone, PartialEq)]
pub struct ExampleDomainContract {
    pub id: u64,
    pub created_at: u64,
}
```

## 🔌 9. Public API / Service Signatures
```rust
// Define service interfaces, pure function signatures, and Result types
pub fn execute_transformation(payload: &ExampleDomainContract) -> Result<(), DomainError>;
```

## 🗺️ 10. File-by-File Module Mapping
- **Domain Layer:** `crates/...`
- **Application Layer:** `crates/...`
- **Infrastructure Layer:** `crates/...`
- **Presentation Layer:** `crates/...`

## 🛠️ 11. Atomic Logical Tasks (PSEUDOCODE DRIVEN)
*Ordered checklist broken down into logical components (e.g., a specific function, struct, or system). **MANDATORY INVARIANT: Each step must include strict pseudocode or algorithmic steps for the Implementer to follow.***

- [ ] **Step 1: Data Contracts & Schemas** *(Target: `crates/...`)*
  - **Pseudocode Spec:**
    ```text
    Define ExampleDomainContract struct
    Include id and created_at fields
    ```
- [ ] **Step 2: Pure Domain Pipeline** *(Target: `crates/...`)*
  - **Pseudocode Spec:**
    ```text
    Create function calculate_transformation(input: &ExampleDomainContract)
    Apply SSOT-NNNN constant K
    Return Result (Ok / Err)
    ```
- [ ] **Step 3: Domain Unit Tests** *(Target: `crates/...`)*
  - **Pseudocode Spec:**
    ```text
    Test 1: Nominal path with valid input and golden vector assertion
    Test 2: Edge case boundary validation
    ```
- [ ] **Step 4: Full Regression & Verification**
  - [ ] Run typecheck & clippy (`cargo check --workspace --all-targets`, `cargo clippy --workspace --all-targets -- -D warnings`).
  - [ ] Run test suite with 100% pass rate (`cargo test --workspace`).

## 🧪 12. Unit Test Targets & Verification Commands

### 12.1 Unit Test Coverage Targets
| Module / File | Test Location | Key Scenarios Covered |
| :--- | :--- | :--- |
| `crates/...` | `crates/...` | Nominal path, empty inputs, boundary thresholds, golden vector validation |

### 12.2 Verification Commands
```bash
# Typecheck
cargo check --workspace --all-targets

# Lint
cargo clippy --workspace --all-targets -- -D warnings

# Test Suite
cargo test --workspace
```

---

# 📋 PART 3: VERIFICATION & RETROSPECTIVE
*(Updated during execution and closed via `/plan-archive`)*

## 📝 13. Runtime Deviations & Architectural Pivots
- *[None logged during drafting. Document any runtime adjustments here.]*

## 💡 14. Retrospective & Follow-Up Items
- *[None logged during drafting. Document lessons learned and follow-ups here.]*
