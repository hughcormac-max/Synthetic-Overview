---
name: plan
description: Master architectural planning orchestrator that coordinates two-phase planning (/plan-human for problem alignment, followed by /plan-robot for technical contracts and <=50-line atomic tasks).
---

# `/plan` Workflow Trajectory

> **Workflow Command:** `/plan`
> **Purpose:** Master planning workflow orchestrating two distinct phases: human problem alignment ([`/plan-human`](../plan-human/SKILL.md)) and technical contract specification ([`/plan-robot`](../plan-robot/SKILL.md)).

---

## 🎯 Dual-Phase Architecture

Planning is divided into two distinct responsibilities to ensure complete alignment before committing to code architecture:

```text
User Request ──► [Phase 1: /plan-human] ──► Human Review Gate ──► [Phase 2: /plan-robot] ──► Approved PLAN-XXX.md
                 (Problem Alignment)        (Approval)             (Technical Contracts &
                                                                   <= 50 line slices)
```

---

## 📋 Execution Protocol

When `/plan` is invoked:

1. **Check for Existing Plan or Draft:**
   - Scan `.agents/docs/plans/INDEX.md` and read existing plans.
   - If the user provides a new feature or idea, initialize Phase 1 via [`/plan-human`](../plan-human/SKILL.md).
   - If an existing plan has Part 1 complete and approved, transition directly to Phase 2 via [`/plan-robot`](../plan-robot/SKILL.md).

2. **Phase 1: Human Problem Alignment (`/plan-human`):**
   - Copy [.agents/skills/plan/resources/TEMPLATE.md](resources/TEMPLATE.md) to `.agents/docs/plans/PLAN-XXX.md`.
   - Populate **Part 1 (Human Alignment)**: Problem statement, objectives, non-goals, user stories, acceptance criteria.
   - Present human summary to user and halt for approval.

3. **Phase 2: Robot Technical Contracts (`/plan-robot`):**
   - Following human approval, populate **Part 2 (Robot Technical Requirements)**: Cites `.agents/docs/ssot/`, types, schemas, function signatures, module mapping.
   - Break tasks into atomic checklist slices with **strict limit: max ~50 lines of code changes per task**.
   - Present technical blueprint and obtain final sign-off before implementation.
