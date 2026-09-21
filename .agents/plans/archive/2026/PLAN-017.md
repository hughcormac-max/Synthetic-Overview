---
id: PLAN-017
title: "Native 6-Step Spec-Driven Development Pipeline"
status: completed
author: "Antigravity"
created: 2026-09-21
updated: 2026-09-21
completed_at: 2026-09-21
branch: "chore/native-sdd-pipeline"
---

# PLAN-017: Native 6-Step Spec-Driven Development Pipeline

> **Status:** `draft` | **Created:** 2026-09-21 | **Last Updated:** 2026-09-21
> **Author:** Antigravity | **Branch:** chore/native-sdd-pipeline

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The current agentic development workflow previously relied on a mix of workspace-specific skills and external, heavy plugins (like `agystack`). To tighten and natively enforce a pure 6-step Spec-Driven Development (SDD) pipeline, we must refactor our `.agents/skills` to explicitly support a native, self-contained pipeline: `research` -> `define-ssot` -> `plan-human` -> `plan-robot` -> `execute` -> `archive-plan`.

### 1.2 Core Objectives
- Refactor the existing `/plan` skill into two explicitly separated skills: `/plan-human` (Functional Specification) and `/plan-robot` (Technical Architecture).
- Implement a native `interrogator` subagent to provide adversarial technical review during the `/plan-robot` phase (replacing the external `agystack/interrogate` dependency).
- Update the `/execute` skill to enforce step-by-step TDD verification of mathematical invariants natively.
- Update `/archive-plan` to subsume final cleanliness, dead-code removal, and blast-radius checks before archival.
- Update `AGENTS.md` to officially establish the 6-step native SDD workflow as the project's Tier 0 rule.

### 1.3 Non-Goals & Exclusions
- We will not rely on or call out to any global plugins. All logic must be contained within `.agents/skills/` and `.agents/subagents/`.
- We are not changing the physics or logic of the actual game; this is purely an architectural update to the AI agent framework.

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [AGENTS.md](../../../AGENTS.md) — Master Agent Guidelines & Invariants (To be updated with Section 3: 6-Step SDD Workflow).

### 2.2 Domain Types & Schemas
N/A (Markdown and Prompt structures).

### 2.3 Public API / Service Signatures
New Skill Commands to be registered:
- `/plan-human`: Generates Sections 1 & 2 of `PLAN-XXX.md` (Intent, User Experience, Boundaries).
- `/plan-robot`: Generates Sections 3, 4, 5 of `PLAN-XXX.md` (Technical Contracts, Rust Structs, Verification Invariants), triggers the `interrogator` subagent, refines the plan, and requests human approval.

### 2.4 Layer Boundary Mapping
- **Skills Layer:** `.agents/skills/plan-human/`, `.agents/skills/plan-robot/`
- **Subagents Layer:** `.agents/subagents/interrogator.md`
- **Rules Layer:** `AGENTS.md`

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Create the Interrogator Subagent**
  - [x] Create `.agents/subagents/interrogator.md` designed to act as an adversarial technical reviewer, checking for ECS cache-misses, floating point non-determinism, and logic gaps in technical plans.
- [x] **Step 2: Refactor the Plan Skills**
  - [x] Rename `.agents/skills/plan/` to `.agents/skills/plan-human/` and update its `SKILL.md` to focus strictly on defining functional intent and boundaries.
  - [x] Create `.agents/skills/plan-robot/SKILL.md` to focus on technical design, orchestrating the `interrogator`, and finalizing the plan for human review.
  - [x] Update `resources/TEMPLATE.md` (shared by both) to explicitly demarcate "Phase 1: Functional Spec" and "Phase 2: Technical Design & Invariants".
- [x] **Step 3: Update Execution & Archival Skills**
  - [x] Update `.agents/skills/execute/SKILL.md` to enforce property-based testing of invariants at every step using native `cargo test`.
  - [x] Update `.agents/skills/archive-plan/SKILL.md` to instruct `code-reviewer` to perform dead-code removal and blast-radius safety checks prior to archiving.
- [x] **Step 4: Update Global Workspace Rules**
  - [x] Modify `AGENTS.md` Section 3 to document the explicit 6-step native SDD workflow.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [Target 1: `/plan-human` and `/plan-robot` can be invoked successfully without plugin dependencies.]
- [Target 2: `interrogator` subagent correctly critiques drafted plans.]

### 4.2 Verification Commands
```bash
# Verify skills folder structure
Get-ChildItem -Path ".agents\skills"
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- None. Implementation strictly adhered to the native 6-step SDD pipeline design.

### 5.2 Lessons Learned & Follow-Up Tasks
- Updated references across workspace documentation (`docs/ROADMAP.md`, `.agents/subagents/planner.md`, `code-reviewer.md`, `implementer.md`) to point to `.agents/skills/plan-human/resources/TEMPLATE.md` and harmonize native `cargo` verification workflows.


