---
name: execute
description: Sequentially implement an approved active plan, enforcing property-based testing of invariants at every step using native cargo test and maintaining synchronized checklist progress.
---

# `/execute` Workflow Trajectory

> **Workflow Command:** `/execute`
> **Purpose:** Sequentially implement an approved active plan from `.agents/plans/active/`, enforcing property-based testing of invariants at every step using native `cargo test` and maintaining synchronized checklist progress.

---

## 🎯 Workflow Execution Steps

### Phase 1: Ingest Active Plan

1. **Load Active Plan:**
   - Locate the target approved plan in `.agents/plans/active/PLAN-XXX.md`.
   - Verify that frontmatter status is `approved` or `in-progress`.
   - Update frontmatter status to `in-progress` if not already set.

### Phase 2: Sequential Step Execution with Invariant Testing (via `implementer`)

Delegate code implementation to the `implementer` subagent ([.agents/subagents/implementer.md](../../subagents/implementer.md)) in an isolated branch workspace (`mode: branch`):

1. **Invoke Implementer Subagent (`implementer`):**
   - Read `.agents/subagents/implementer.md` to ground implementation rules.
   - Dispatch the `implementer` subagent via `invoke_subagent` (with `Workspace: "branch"`).
   - Instruct the subagent to:
     - Ingest `.agents/plans/active/PLAN-XXX.md`.
     - Sequentially implement each atomic unchecked step (`- [ ]`) adhering strictly to Section 3 contracts, invariants, and [AGENTS.md](../../../AGENTS.md).
     - **Enforce Property-Based & Invariant Testing:** Author and run hermetic unit and property-based tests verifying mathematical invariants using native `cargo test` after every single step before progressing to the next.
     - Never advance to Step N+1 if any invariant assertions or unit tests fail in Step N.
     - Synchronize the plan checklist by ticking off completed steps (`- [x]`).
2. **Await Completion & Branch Merge:**
   - Wait for `implementer` to complete all steps and report back.
   - Ensure changes are merged cleanly into the working branch.

### Phase 3: Final Verification Triad

1. **Run Full Native Verification:**
   - Run typecheck / compilation: `cargo check --workspace --all-targets`
   - Run linter: `cargo clippy --workspace --all-targets -- -D warnings`
   - Run full test suite & property invariant assertions: `cargo test --workspace`
2. **Review Checklist:**
   - Ensure all steps are ticked before preparing to trigger `/archive-plan`.
