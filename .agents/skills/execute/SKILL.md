---
name: execute
description: Sequentially implement an approved plan by spawning isolated subagents for file-level tasks, enforcing <= 50 line change thresholds and step-by-step test assertions.
---

# `/execute` Workflow Trajectory

> **Workflow Command:** `/execute`
> **Purpose:** Sequentially implement an approved plan from `.agents/docs/plans/PLAN-XXX.md` by dispatching isolated subagents to implement focused tasks across different code files.

---

## 🎯 Core Execution Invariants

1. **Strict ~50-Line Slicing Threshold:**
   - Any implementation task involving more than approximately 50 lines of code changes MUST be split into smaller sub-tasks before code generation begins.
2. **Subagent Task Isolation:**
   - Tasks targeting different files or modules are executed by isolated subagents (using `self` with branch/share workspace or focused prompt scopes).
   - This bounds the blast radius and prevents hallucinated cross-file pollution.
3. **Step-by-Step Test Assertion:**
   - Unit tests for step N must pass before proceeding to step N+1.
   - Checklist checkboxes (`- [x]`) are marked in `.agents/docs/plans/PLAN-XXX.md` as each step is verified.

---

## 🛠️ Execution Protocol

### Phase 1: Ingest Approved Plan

1. **Locate Target Plan:**
   - Read `.agents/docs/plans/PLAN-XXX.md`.
   - Confirm frontmatter `status` is `approved` or `in-progress`.
   - If `approved`, update frontmatter to `status: in-progress`.

### Phase 2: Atomic Task Execution Loop

For each unchecked task (`- [ ]`) in Section 11 of the plan:

1. **Task Size Verification:**
   - Review the planned task scope. If estimated changes exceed ~50 lines, split the task into multiple discrete sub-tasks in the plan checklist before proceeding.
2. **Spawn Isolated Subagent:**
   - Invoke an isolated subagent (`TypeName: "self"` with `Workspace: "branch"` or focused task prompt):
     - Pass the approved technical contracts from Section 8 & 9.
     - Pass the specific target file and task instructions.
     - Enforce the 50-line maximum constraint.
     - Instruct the subagent to write the implementation code and corresponding unit tests.
3. **Assert Hermetic Tests & Typechecks:**
   - Run the project test suite and typechecker on the modified code.
   - If tests fail, resolve failures within the isolated context before merging.
4. **Synchronize Checklist Progress:**
   - Mark the step complete (`- [x]`) in `.agents/docs/plans/PLAN-XXX.md`.
   - If any runtime architectural pivot occurred, log it immediately in Section 13 (Deviations).

### Phase 3: Final Verification & Pre-Archive Readiness

1. **Run Full Verification:**
   - Run typecheck, lint, and full regression test suite.
2. **Confirm Plan Completion:**
   - Verify all implementation checkboxes in the plan are ticked.
   - Inform developer that execution is complete and ready for `/archive-plan`.
