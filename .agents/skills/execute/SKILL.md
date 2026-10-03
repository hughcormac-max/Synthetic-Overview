---
name: execute
description: Sequentially implement an approved plan using the Orchestrator-Worker (Boomerang) pattern. Dispatches the implementer with pseudocode, waits for a Boomerang Summary, and spawns the code-reviewer for verification.
---

# `/execute` Workflow Trajectory

> **Workflow Command:** `/execute`
> **Purpose:** Sequentially implement an approved plan from `.agents/docs/plans/PLAN-NNNN.md` by orchestrating the implementer and code-reviewer subagents using the Boomerang pattern.

---

## 🛑 Core Execution Invariants

1. **Pseudocode Translation:**
   - The orchestrator owns the plan and delegates only **one pseudocode task** at a time to the implementer.
   - The implementer translates pseudocode directly into code.
2. **Boomerang Summary Protocol:**
   - The orchestrator must wait for the implementer to return a strict **Boomerang Summary** (Files Modified, Commands Run, Exit Codes) before proceeding.
3. **Two-Stage Verification Gate:**
   - Before ticking off a task, the orchestrator MUST spawn the `code-reviewer` to validate the Boomerang Summary against the pseudocode and ensure all project tests pass.
   - Checklist checkboxes (`- [x]`) are marked in `.agents/docs/plans/PLAN-NNNN.md` only after the `code-reviewer` approves.

---

## ⚙️ Execution Protocol

### Phase 1: Ingest Approved Plan

1. **Locate Target Plan:**
   - Read `.agents/docs/plans/PLAN-NNNN.md`.
   - Confirm frontmatter `status` is `approved` or `in-progress`.
   - If `approved`, update frontmatter to `status: in-progress`.

### Phase 2: Orchestrator Loop (Boomerang Pattern)

For each unchecked task (`- [ ]`) in Section 11 of the plan:

1. **Identify Task:**
   - Read the next unchecked logical task and its explicit pseudocode.
2. **Spawn Implementer:**
   - Invoke the `implementer` subagent.
   - Pass the specific target file and the **pseudocode task**.
   - Instruct the subagent to write the implementation and return the Boomerang Summary.
3. **Wait for Boomerang Summary:**
   - Receive the summary containing Files Modified, Commands Run, and Exit Codes.
4. **Spawn Code-Reviewer:**
   - Invoke the `code-reviewer` subagent.
   - Pass the original pseudocode and the implementer's Boomerang Summary.
   - Instruct the reviewer to perform the Two-Stage Verification Gate.
5. **Tick the Checkbox / Handle Rejection:**
   - If the reviewer **APPROVES**: Mark the step complete (`- [x]`) in `.agents/docs/plans/PLAN-NNNN.md`.
   - If the reviewer **REJECTS**: Re-dispatch the `implementer` with the reviewer's feedback.

### Phase 3: Final Verification & Pre-Archive Readiness

1. **Run Full Verification:**
   - Run typecheck, lint, and full regression test suite.
2. **Confirm Plan Completion:**
   - Verify all implementation checkboxes in the plan are ticked.
   - Inform developer that execution is complete and ready for `/archive-plan`.
