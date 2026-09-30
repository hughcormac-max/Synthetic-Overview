---
name: plan-human
description: Use this skill when initiating a new feature, refactor, or bugfix to align the user-facing problem, objectives, non-goals, user stories, and acceptance criteria before technical specification.
---

# `/plan-human` Workflow Trajectory

> **Workflow Command:** `/plan-human`
> **Purpose:** Align the human-facing problem statement, user requirements, user stories, and acceptance criteria in Part 1 of a new or existing plan specification.

---

## 🎯 Workflow Execution Steps

### Phase 1: Context Gathering & Clarification

1. **Understand Problem & User Context:**
   - Engage with the user to clarify the core user problem, desired experience, and specific workflows.
   - Clarify edge cases, user pain points, and explicit boundaries.
2. **Determine Plan ID:**
   - Read `.agents/docs/plans/INDEX.md` to find existing plans.
   - Determine the next sequential plan ID (e.g., `PLAN-001`). If targeting an existing draft, reuse that ID.

### Phase 2: Draft Part 1 (Human Problem Alignment)

1. **Initialize or Update Plan File:**
   - Copy `.agents/skills/plan-human/resources/TEMPLATE.md` to `.agents/docs/plans/PLAN-XXX.md` (if new).
   - Set YAML frontmatter:
     ```yaml
     ---
     id: PLAN-XXX
     title: "[Short, Descriptive Title]"
     status: draft
     author: "[Author / Agent Name]"
     created: YYYY-MM-DD
     updated: YYYY-MM-DD
     branch: "[branch-name]"
     ---
     ```
2. **Populate Part 1 Sections Exclusively:**
   - **Section 1.1 Problem Statement:** Clear description of user problem, friction point, or opportunity.
   - **Section 1.2 Core Objectives:** Concrete capabilities unlocked from the user or developer perspective.
   - **Section 1.3 Non-Goals & Exclusions:** Explicit out-of-scope boundaries and deferred work.
   - **Section 1.4 User Stories & Interaction Journeys:** Walkthrough of key scenarios and workflows.
   - **Section 1.5 Acceptance Criteria:** Black-box observable criteria that define user success.
   - **Section 1.6 Edge Cases & Boundary Behaviors:** User-facing error messaging, empty states, limits.
3. **Leave Part 2 as Placeholder:**
   - Leave Part 2 (Robot Technical Requirements) marked as `[To be populated by /plan-robot]`.

### Phase 3: Synchronize Ledger & Request Review

1. **Register or Update in Ledger:**
   - Ensure an entry exists in `.agents/docs/plans/INDEX.md` with status `draft`.
2. **Present Human Summary to User:**
   - Present a concise summary in chat highlighting problem statement, core objectives, non-goals, and acceptance criteria.
3. **Review Gate (Mandatory Human In The Loop HALT):**
   - **STRICT HALT:** Deliberately pause here for human review. Do not automatically proceed to `/plan-robot`.
   - The user must review, refine if necessary, and explicitly instruct the agent to run `/plan-robot` once aligned.
