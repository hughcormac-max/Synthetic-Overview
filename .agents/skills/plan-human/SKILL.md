---
name: plan-human
description: Define functional intent, scope boundaries, user stories, and acceptance criteria (Phase 1 of PLAN-XXX.md) prior to technical design.
---

# `/plan-human` Workflow Trajectory

> **Workflow Command:** `/plan-human`
> **Purpose:** Establish the functional specification, problem boundaries, user experience scenarios, and acceptance criteria (Phase 1 of `PLAN-XXX.md`) before initiating technical architecture.

---

## 🎯 Workflow Execution Steps

### Phase 1: Scope Discovery & Identification

1. **Clarify Functional Goals:**
   - Review developer requirements, user requests, or referenced research documents (`docs/research/` or `docs/ssot/`).
   - Identify user workflows, expected behavioral outcomes, and boundary conditions.
2. **Determine Plan ID:**
   - Inspect `.agents/plans/active/` and `.agents/plans/archive/INDEX.md` to assign the next sequential plan identifier (e.g., `PLAN-018`).

### Phase 2: Scaffold & Draft Phase 1 Specification

1. **Initialize Plan File:**
   - Copy [.agents/skills/plan-human/resources/TEMPLATE.md](resources/TEMPLATE.md) to `.agents/plans/active/PLAN-XXX.md`.
2. **Draft Functional Specification (Sections 1 & 2):**
   - **Section 1: Intent & Boundaries:**
     - **1.1 Problem Statement:** Articulate the exact problem, user pain point, or capability gap being addressed.
     - **1.2 Core Objectives:** List discrete, measurable goals and capabilities to be unlocked.
     - **1.3 Non-Goals & Exclusions:** Explicitly enumerate items out of scope to bound development.
   - **Section 2: Functional Acceptance Criteria & User Scenarios:**
     - **2.1 User Scenarios & Core Workflows:** Detail step-by-step user interaction stories and behavioral sequences.
     - **2.2 Functional Acceptance Criteria:** Define clear, measurable acceptance criteria from an end-user / caller perspective.
3. **Preserve Phase 2 Placeholders:**
   - Leave Sections 3, 4, 5, and 6 unpopulated for technical design by `/plan-robot`.

### Phase 3: Alignment & Review Gate

1. **Present Functional Summary in Chat:**
   - Highlight the problem statement, objectives, exclusions, and acceptance criteria.
2. **Handoff to Technical Planning:**
   - Prompt the user to confirm the functional scope, then proceed to run `/plan-robot` to architect the technical design, data types, and invariants.
