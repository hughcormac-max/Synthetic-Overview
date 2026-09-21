---
id: PLAN-XXX
title: "[Short, Descriptive Title]"
status: draft # draft | approved | in-progress | completed | superseded | abandoned
author: "[Author / Agent Name]"
created: YYYY-MM-DD
updated: YYYY-MM-DD
branch: "[branch-name]"
---

# PLAN-XXX: [Short, Descriptive Title]

> **Status:** `draft` | **Created:** YYYY-MM-DD | **Last Updated:** YYYY-MM-DD
> **Author:** [Author / Agent Name] | **Branch:** [branch-name]

---

# Phase 1: Functional Spec (Defined via /plan-human)

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
[Describe the core problem, user requirement, or bug being addressed.]

### 1.2 Core Objectives
- [Objective 1: What must this plan achieve?]
- [Objective 2: What capability will be unlocked?]

### 1.3 Non-Goals & Exclusions
- [Non-Goal 1: What is explicitly out of scope for this plan?]
- [Non-Goal 2: What follow-ups are deferred to future plans?]

---

## 📋 2. Functional Acceptance Criteria & User Scenarios

### 2.1 User Scenarios & Core Workflows
- Scenario 1: [Given / When / Then user or caller interaction flow]
- Scenario 2: [Nominal and boundary user experience]

### 2.2 Functional Acceptance Criteria
- [Criterion 1: Observable behavior or user-facing outcome]
- [Criterion 2: Functional constraint or boundary condition]

---

# Phase 2: Technical Design & Invariants (Defined via /plan-robot)

## 📐 3. Technical Contracts & Invariants

*All domain grounding references, types, schemas, and public API signatures must be declared and reviewed here prior to code implementation.*

### 3.1 Authoritative Domain References
*Downlink to immutable domain specifications, formulas, constants, and truth tables in `docs/ssot/`:*
- [SSOT-NNNN: Domain SSOT Title](../../../docs/ssot/SSOT-NNNN.md) — *[Specify applicable formulas, constants, decision trees, or golden vectors cited from this SSOT]*

### 3.2 Domain Types, ECS Components & Schemas
```rust
// Define domain models, ECS components, resources, or contracts here
pub struct ExampleComponent {
    pub id: u64,
    pub created_at: u64,
    pub status: ExampleStatus,
}

pub enum ExampleStatus {
    Pending,
    Active,
    Completed,
}
```

### 3.3 Public API / System Signatures
```rust
// Define system interfaces, service functions, or traits
pub fn example_system(
    query: Query<&ExampleComponent>,
) {
    // Deterministic system implementation
}
```

### 3.4 Mathematical Invariants & Determinism Guarantees
*Explicit invariants, precision bounds, and determinism rules (CRITICAL: ZERO LATEX):*
- Invariant 1: [Define invariant using plain-text ASCII math, e.g. mass_total == sum(component_masses)]
- Invariant 2: [Floating point epsilon tolerances: abs(v_actual - v_target) < 1e-6]
- Invariant 3: [ECS cache locality and component access constraints: Zero pointer indirection inside tight loops]

### 3.5 Layer Boundary Mapping
- **Domain / Core:** `crates/.../src/core/...`
- **ECS Systems:** `crates/.../src/systems/...`
- **Adapters / Infrastructure:** `crates/.../src/adapters/...`

---

## 🛠️ 4. Implementation Steps

*Ordered checkbox checklist broken down into atomic, testable steps.*

- [ ] **Step 1: Domain Models, Types & Invariant Contracts**
  - [ ] Define Rust structs, enums, and ECS component data models.
  - [ ] Implement boundary schema validation.
- [ ] **Step 2: Core Logic & Invariant Property Tests**
  - [ ] Implement deterministic transformation functions citing `[SSOT-NNNN]`.
  - [ ] Add property-based tests verifying mathematical invariants using native `cargo test`.
- [ ] **Step 3: Systems, Adapters & Services**
  - [ ] Implement ECS systems or service layer orchestration.
  - [ ] Ensure query filters avoid archetypal thrashing and minimize cache misses.
- [ ] **Step 4: Integration & Wire-up**
  - [ ] Connect systems to the scheduler / plugin pipeline.
  - [ ] Verify end-to-end data flow across layers.
- [ ] **Step 5: Full Regression & Verification**
  - [ ] Verify against golden test vectors in `[SSOT-NNNN]`.
  - [ ] Execute `cargo check`, `cargo clippy`, and `cargo test`.

---

## 🧪 5. Verification & Criteria

### 5.1 Measurable Benchmarks & Invariant Targets
- [Target 1: 100% unit and property-based test pass rate for all modified modules]
- [Target 2: Zero compiler warnings and zero clippy warnings (`cargo clippy`)]
- [Target 3: Zero regressions across existing test suites (`cargo test`)]
- [Target 4: Verified against golden benchmark vectors from cited SSOT documents]

### 5.2 Unit & Property Test Targets
| Module / File | Test File | Key Scenarios & Invariants Covered |
| :--- | :--- | :--- |
| `crates/.../src/example.rs` | `crates/.../tests/example_test.rs` | Nominal path, boundary thresholds, property-based invariant fuzzing, golden vector validation |

### 5.3 Verification Commands
```bash
# Typecheck / Cargo Check
cargo check --workspace --all-targets

# Linter / Cargo Clippy
cargo clippy --workspace --all-targets -- -D warnings

# Hermetic Test Suite & Invariants
cargo test --workspace
```

---

## 📝 6. Deviations & Retrospective (Post-Implementation)

*Record any runtime architectural pivots, design trade-offs, or unexpected discoveries made during implementation before archiving this plan.*

### 6.1 Architectural Deviations
- *[None logged during drafting. Update during/after implementation.]*

### 6.2 Lessons Learned & Follow-Up Tasks
- *[None logged during drafting. Update during/after implementation.]*
