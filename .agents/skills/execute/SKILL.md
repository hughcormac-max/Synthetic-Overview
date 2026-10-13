---
name: execute
description: Sequentially implement an approved plan by dispatching the implementer subagent for single tasks and the code-reviewer subagent for Two-Stage Verification.
---

# `/execute` Workflow Trajectory

> **Workflow Command:** `/execute`
> **Purpose:** Act as the **Orchestrator** to sequentially implement an approved plan from `.agents/docs/plans/PLAN-NNNN.md`. Dispatch isolated subagents for atomic tasks and verify them via the Boomerang pattern.

---

## 🎯 Core Execution Invariants

1. **Orchestrator Ownership:**
   - The Orchestrator (Main Agent) owns the plan. Subagents NEVER edit the plan checkboxes themselves.
2. **Pseudocode-Driven Tasks:**
   - Every task dispatched must include the exact pseudocode defined in the plan.
3. **The Boomerang Pattern (Two-Stage Verification):**
   - Step N must be implemented, summarize via a Boomerang Summary, and perfectly pass `code-reviewer` validation (Spec Adherence + Quality) before the Orchestrator ticks the box and proceeds to Step N+1.

---

## 🛠️ Execution Protocol

### Phase 1: Ingest Approved Plan

1. **Locate Target Plan:**
   - Read `.agents/docs/plans/PLAN-NNNN.md`.
   - Confirm frontmatter `status` is `approved` or `in-progress`.
   - If `approved`, update frontmatter to `status: in-progress`.

### Phase 2: Atomic Task Execution Loop

For each unchecked task (`- [ ]`) in Section 11 of the plan:

1. **Read Pseudocode Spec:**
   - Extract the specific logical task and its pseudocode.
2. **Spawn Implementer Subagent:**
   - Read `.agents/subagents/implementer.md` to understand its capabilities.
   - Invoke the `implementer` subagent using `invoke_subagent` (TypeName: "implementer", Workspace: "branch").
   - Provide it with the explicit pseudocode and target files.
   - Wait for the `implementer` to return its structured **Boomerang Summary** (Files Modified, Commands Run, Exit Codes).
3. **Spawn Code-Reviewer Subagent (Two-Stage Verification):**
   - Read `.agents/subagents/code-reviewer.md`.
   - Invoke the `code-reviewer` subagent (TypeName: "code-reviewer", Workspace: "inherit" or matching the branch).
   - Pass the `implementer`'s Boomerang Summary and the original pseudocode.
   - Wait for the reviewer's Pass/Fail verdict.
   - *If Fail:* Pass the failure feedback back to the `implementer` for a single automatic fix attempt.
4. **Synchronize Checklist Progress:**
   - Once verified, merge the branch (if applicable).
   - Mark the step complete (`- [x]`) in `.agents/docs/plans/PLAN-NNNN.md`.
   - If any runtime architectural pivot occurred, log it immediately in Section 13 (Deviations).

### Phase 3: Final Verification & Pre-Archive Readiness

1. **Run Full Verification:**
   - Inform the user that all tasks are complete.
   - Run full typecheck, lint, and full regression test suite across the whole project.
2. **Confirm Plan Completion:**
   - Verify all implementation checkboxes in the plan are ticked.
   - Inform developer that execution is complete and ready for `/plan-archive`.
