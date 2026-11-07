---
name: plan-human
description: Use this skill when initiating a new feature, refactor, or bugfix to align the user-facing problem, objectives, non-goals, user stories, and acceptance criteria in .agents/work/PLAN-<slug>.md before technical specification.
---

# `/plan-human` Workflow Trajectory

> **Workflow Command:** `/plan-human`
> **Purpose:** Align the human-facing problem statement, user requirements, user stories, and acceptance criteria in Part 1 of a staged plan specification in `.agents/work/PLAN-<slug>.md`.

---

## 🎯 Workflow Execution Steps

### Phase 1: Context Gathering & Memory Palace Grounding

1. **Understand Problem & Project Context:**
   - Check the project's entity node in `G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\` and any relevant `Design_And_Development/` pillars to ground the feature in the project's standing scope.
   - Engage with the user to clarify the core problem, desired workflow, edge cases, and explicit out-of-scope boundaries.
2. **Determine Plan Slug:**
   - Choose a concise, descriptive kebab-case filename in `.agents/work/PLAN-<slug>.md` (e.g., `.agents/work/PLAN-hohmann-transfer.md`).
   - If updating an existing draft in `.agents/work/`, reuse that file.

### Phase 2: Draft Part 1 (Human Problem Alignment)

1. **Initialize or Update Staged Plan File:**
   - Copy [.agents/skills/plan-human/assets/TEMPLATE.md](assets/TEMPLATE.md) to `.agents/work/PLAN-<slug>.md` (if new).
   - Set YAML frontmatter:
     ```yaml
     ---
     id: PLAN-<slug>
     title: "[Short, Descriptive Title]"
     project: "[[YYMMDD_ORG_ProjectName]]"
     status: draft
     author: "[Author / Agent Name]"
     created: YYYY-MM-DD
     updated: YYYY-MM-DD
     branch: "[branch-name]"
     ---
     ```
2. **Populate Part 1 Sections Exclusively:**
   - **Section 1 Problem Statement:** Clear description of user problem, friction point, or opportunity.
   - **Section 2 Core Objectives:** Concrete capabilities unlocked from the user or developer perspective.
   - **Section 3 Non-Goals & Exclusions:** Explicit out-of-scope boundaries and deferred work.
   - **Section 4 User Stories & Interaction Journeys:** Walkthrough of key scenarios and workflows.
   - **Section 5 Acceptance Criteria (EARS Syntax):** Use `WHEN [trigger], the system SHALL [behavior]`.
   - **Section 6 Edge Cases & Boundary Behaviors:** User-facing error messaging, empty states, limits.
3. **Leave Part 2 as Placeholder:**
   - Leave Part 2 (Robot Technical Specifications) intact for `/plan-robot`.

### Phase 3: Present Summary & Request Human Review

1. **Present Human Summary to User:**
   - Present a concise summary in chat with a clickable link to `.agents/work/PLAN-<slug>.md`, highlighting problem statement, core objectives, non-goals, and acceptance criteria.
2. **Review Gate (MANDATORY HALT):**
   - **PAUSE & STOP:** Deliberately halt execution. Do not proceed to technical specification.
   - Await explicit user feedback, revisions, and approval on Part 1.
   - Inform the user that `/plan-robot` can be invoked once Part 1 is approved.
