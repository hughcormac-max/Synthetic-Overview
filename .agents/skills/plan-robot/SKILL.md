---
name: plan-robot
description: Architect technical contracts, Rust ECS structs, and invariants for Phase 2 of PLAN-XXX.md, orchestrate adversarial review via the interrogator subagent, refine the specification, and request human approval.
---

# `/plan-robot` Workflow Trajectory

> **Workflow Command:** `/plan-robot`
> **Purpose:** Architect technical contracts, Rust ECS data structures, and mathematical invariants (Phase 2 of `PLAN-XXX.md`), orchestrate an adversarial technical critique via the `interrogator` subagent, refine the plan, and present it for human approval.

---

## 🎯 Workflow Execution Steps

### Phase 1: Functional Plan Ingestion & Codebase Research

1. **Ingest Active Plan:**
   - Locate and read the target active plan in `.agents/plans/active/PLAN-XXX.md`.
   - Confirm that Phase 1 (Sections 1 & 2) has defined the problem statement, core objectives, exclusions, and functional acceptance criteria.
2. **Ground Against Domain SSOT & Codebase:**
   - Review relevant immutable truth documents in `docs/ssot/SSOT-NNNN.md`.
   - Inspect existing Rust crates, modules, ECS queries, and data models to map dependencies and avoid architectural collisions.

### Phase 2: Technical Design & Invariants Formulation

Populate Phase 2 of `PLAN-XXX.md`:

1. **Section 3: Technical Contracts & Invariants:**
   - **3.1 Authoritative Domain References:** Link cited SSOT documents, formulas, and golden vectors.
   - **3.2 Domain Types, ECS Components & Schemas:** Specify exact Rust structs, enums, component packing, and resource definitions adhering to Data-Oriented Design.
   - **3.3 Public API / System Signatures:** Define Bevy ECS system signatures, query filters, and schedule placements.
   - **3.4 Mathematical Invariants & Determinism Guarantees:** Define all precision tolerances, determinism rules, and invariant formulas using plain-text ASCII math (Zero-LaTeX).
   - **3.5 Layer Boundary Mapping:** Map crates and module boundaries.
2. **Section 4: Implementation Steps:**
   - Break down implementation into atomic, testable steps with checkable boxes (`- [ ]`).
   - Sequence tasks: contracts/types first, domain/property tests second, systems/adapters third, integration fourth, and full regression fifth.
3. **Section 5: Verification & Criteria:**
   - Define measurable targets, property-based test assertions, and exact verification commands (`cargo check`, `cargo clippy`, `cargo test`).

### Phase 3: Adversarial Review Gate (via `interrogator`)

Delegate an adversarial technical audit to the `interrogator` subagent ([.agents/subagents/interrogator.md](../../subagents/interrogator.md)) in read-only audit mode (`mode: inherit`):

1. **Invoke Interrogator Subagent (`interrogator`):**
   - Dispatch the `interrogator` subagent via `invoke_subagent`.
   - Provide the drafted Phase 2 sections of `PLAN-XXX.md`.
   - Instruct the `interrogator` to scrutinize:
     - **ECS Cache-Misses & Layouts:** Suboptimal component packing, pointer indirection, archetypal churn, and query bottlenecks.
     - **Floating-Point Determinism:** Order-of-operations drift, non-associative float accumulations, missing epsilon bounds.
     - **Logic Gaps & Edge Cases:** Boundary singularities, zero division, NaN propagation, unhandled failure paths.
     - **Zero-LaTeX Compliance:** Ensure pure ASCII math formatting.
2. **Evaluate Adversarial Critique:**
   - Await the Interrogator's adversarial report.
   - Refine and update `PLAN-XXX.md` Sections 3, 4, and 5 to address all Critical Blockers and valid critiques.

### Phase 4: Executive Summary & Human Approval Gate

1. **Present Technical Architecture in Chat:**
   - Present a concise executive summary highlighting:
     - Core technical contracts and ECS component layouts.
     - Key mathematical invariants and property test plans.
     - Interrogator findings and resolutions made.
2. **Await Explicit Human Approval:**
   - **STRICT HALT:** Do not proceed to `/execute` until the developer reviews and explicitly approves the finalized plan.
   - Upon developer approval, update frontmatter status to `approved`.
