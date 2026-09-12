---
name: execute
description: Sequentially implement an approved active plan, executing test assertions at each atomic step and maintaining synchronized checklist progress.
---
# `/execute` Workflow Trajectory

> **Workflow Command:** `/execute`
> **Purpose:** Sequentially implement an approved active plan from `.agents/plans/active/`, executing test assertions at each atomic step and maintaining synchronized checklist progress.

---

## 🎯 Workflow Execution Steps

### Phase 1: Ingest Active Plan

1. **Load Active Plan:**
   - Locate the target approved plan in `.agents/plans/active/PLAN-XXX.md`.
   - Verify that frontmatter status is `approved` or `in-progress`.
   - Update frontmatter status to `in-progress` if not already set.

### Phase 2: Sequential Step Execution (via `implementer`)

Delegate code implementation to the `implementer` subagent ([.agents/subagents/implementer.md](../../subagents/implementer.md)) in an isolated branch workspace (`mode: branch`):

1. **Invoke Implementer Subagent (`implementer`):**
   - Read `.agents/subagents/implementer.md` to ground implementation rules.
   - Dispatch the `implementer` subagent via `invoke_subagent` (with `Workspace: "branch"`).
   - Instruct the subagent to:
     - Ingest `.agents/plans/active/PLAN-XXX.md`.
     - Sequentially implement each atomic unchecked step (`- [ ]`) adhering strictly to Section 2 contracts and [AGENTS.md](../../../AGENTS.md).
     - Author and run hermetic unit tests (`npm test` / project test runner) after each step before progressing to the next.
     - Synchronize the plan checklist by ticking off completed steps (`- [x]`).
2. **Await Completion & Branch Merge:**
   - Wait for `implementer` to complete all steps and report back.
   - Ensure changes are merged cleanly into the working branch.

### Phase 3: Final Verification Triad

1. **Run Full Verification:**
   - Run typechecker (`npm run typecheck`).
   - Run linter (`npm run lint`).
   - Run all regression tests (`npm test`).
2. **Review Checklist:**
   - Ensure all steps are ticked before preparing to trigger `/archive-plan`.
