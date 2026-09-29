---
name: archive-plan
description: Verify completion criteria, log architectural retrospective notes, update plan metadata, and finalize plan status in the master ledger without moving files.
---

# `/archive-plan` Workflow Trajectory

> **Workflow Command:** `/archive-plan`
> **Purpose:** Verify the 7-Point QA checklist, log architectural retrospective notes, and finalize plan status in `.agents/docs/plans/INDEX.md`. Plans remain permanently in `.agents/docs/plans/` without moving folders.

---

## 🎯 Workflow Execution Steps

### Phase 1: Verification & QA Gate (via `code-reviewer`)

1. **Invoke Code Reviewer Audit:**
   - Execute the Pre-Completion 7-Point QA Checklist ([AGENTS.md](../../../AGENTS.md)):
     1. **Typecheck:** Confirm zero compilation/type errors.
     2. **Unit Tests:** Confirm 100% pass rate on touched and new modules.
     3. **Regression Tests:** Verify full test suite passes with zero regressions.
     4. **Contract Adherence:** Confirm implementation matches approved contracts in `PLAN-XXX.md`.
     5. **Zero-LaTeX Audit:** Grep touched files for unescaped dollar signs (`$...$`, `$$...$$`) or LaTeX macros (`\dot`, `\frac`, `\approx`, `\Omega`, etc.). Confirm pure ASCII/plain text notation.
     6. **Downward Reference Audit:** Confirm `.agents/docs/ssot/` maintains zero upward path references to codebase files (`src/`, `package.json`, etc.).
     7. **Plan Checklist & Clean Git Status:** Verify all steps are ticked (`- [x]`) and no lingering scratch/debug artifacts remain.
2. **Review QA Report:**
   - If any audit point fails, halt the workflow and surface actionable findings for correction.

### Phase 2: Retrospective & Frontmatter Finalization

1. **Log Retrospective & Deviations:**
   - Review Part 3 (Sections 13 & 14) in `.agents/docs/plans/PLAN-XXX.md`.
   - Ensure all runtime pivots, lessons learned, and follow-up items are documented.
2. **Update Plan Frontmatter:**
   - Update YAML frontmatter in `.agents/docs/plans/PLAN-XXX.md`:
     ```yaml
     status: completed # completed | superseded | abandoned
     updated: YYYY-MM-DD
     completed_at: YYYY-MM-DD
     ```

### Phase 3: Synchronize Master Plan Ledger

1. **Update Master Ledger:**
   - In `.agents/docs/plans/INDEX.md`, locate the plan entry.
   - Update its `Status` to `completed` (or `superseded` / `abandoned`) and populate the `Completed` date column.
2. **Present Completion Summary:**
   - Provide the user with a concise completion summary and a link to the finalized plan.
