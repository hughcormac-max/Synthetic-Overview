---
name: execute
description: Sequentially implement an approved plan in .agents/work/PLAN-<slug>.md by dispatching the implementer subagent for single tasks and the code-reviewer subagent for Two-Stage Verification.
---

# `/execute` Workflow Trajectory

> **Workflow Command:** `/execute`
> **Purpose:** Act as the **Orchestrator** to sequentially implement an approved plan from `.agents/work/PLAN-<slug>.md`. Dispatch isolated subagents for atomic tasks and verify them via the Boomerang pattern.

---

## 🎯 Core Execution Invariants

1. **Orchestrator Ownership:**
   - The Orchestrator (Main Agent) owns the staged plan in `.agents/work/PLAN-<slug>.md`. Subagents NEVER edit the plan checkboxes themselves.
2. **Pseudocode-Driven Tasks:**
   - Every task dispatched must include the exact pseudocode defined in Section 11 of the plan.
3. **The Boomerang Pattern (Two-Stage Verification):**
   - Step N must be implemented, summarized via a Boomerang Summary, and pass `code-reviewer` validation (Spec Adherence + Quality) before the Orchestrator ticks the box and proceeds to Step N+1.

---

## 🛠️ Execution Protocol

### Phase 1: Ingest Approved Plan & Register Subagents

1. **Locate Target Plan:**
   - Read `.agents/work/PLAN-<slug>.md`.
   - Confirm frontmatter `status` is `approved` or `in-progress`.
   - If `approved`, update frontmatter to `status: in-progress`.
2. **Ensure Subagent Definitions are Registered:**
   - Read `.agents/subagents/implementer.md` and `.agents/subagents/code-reviewer.md`.
   - If `implementer` and `code-reviewer` have not yet been defined in this conversation via `define_subagent`, register them first using their system prompts and tool permissions (`EnableWriteTools: true` for `implementer`, `EnableWriteTools: false` for `code-reviewer`).

### Phase 2: Atomic Task Execution Loop

For each unchecked task (`- [ ]`) in Section 11 of `.agents/work/PLAN-<slug>.md`:

1. **Extract Pseudocode Spec:**
   - Extract the specific logical task, target file path(s), and pseudocode steps.
2. **Spawn Implementer Subagent:**
   - Invoke the `implementer` subagent using `invoke_subagent` (`TypeName: "implementer"`).
   - Provide it with the explicit pseudocode, target files, and project verification commands.
   - Wait for the `implementer` to return its structured **Boomerang Summary** (Files Modified, Commands Run, Exit Codes, Status).
3. **Spawn Code-Reviewer Subagent (Two-Stage Verification):**
   - Invoke the `code-reviewer` subagent (`TypeName: "code-reviewer"`).
   - Pass the `implementer`'s Boomerang Summary, modified file paths, and the original pseudocode.
   - Wait for the reviewer's Pass/Fail verdict.
   - *If Fail:* Pass the failure feedback back to the `implementer` for a single automatic fix attempt.
4. **Synchronize Checklist Progress:**
   - Mark the step complete (`- [x]`) in `.agents/work/PLAN-<slug>.md`.
   - If any runtime architectural pivot occurred, log it immediately in Section 13 (Runtime Deviations & Architectural Pivots).

### Phase 3: Final Verification & Pre-Closeout Readiness

1. **Run Full Verification:**
   - Run full typecheck, lint, and full regression test suite across the whole project.
2. **Confirm Plan Completion:**
   - Verify all implementation checkboxes in `.agents/work/PLAN-<slug>.md` are ticked (`- [x]`) and update frontmatter to `status: completed`.
   - Inform the developer that execution is complete and offer to run `/close-to-palace` when ready to close out the session.
