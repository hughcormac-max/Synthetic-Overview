---
name: archive-plan
description: Verify completion criteria, log architectural retrospective notes, update plan metadata, and move active plans into the permanent historical archive ledger.
---
# `/archive-plan` Workflow Trajectory

> **Workflow Command:** `/archive-plan`
> **Purpose:** Verify completion criteria, log architectural retrospective notes, update plan metadata, and move active plans into the permanent historical archive ledger.

---

## 🎯 Workflow Execution Steps

### Phase 1: Verification & QA Gate (via `code-reviewer`)

Delegate the 7-Point QA verification to the `code-reviewer` subagent ([.agents/subagents/code-reviewer.md](../../subagents/code-reviewer.md)) in read-only audit mode (`mode: inherit`):

1. **Invoke Code Reviewer Subagent (`code-reviewer`):**
   - Read `.agents/subagents/code-reviewer.md` to confirm QA audit parameters.
   - Dispatch the `code-reviewer` subagent via `invoke_subagent` (with `Workspace: "inherit"`).
   - Instruct the subagent to execute the Pre-Completion 7-Point QA Checklist ([AGENTS.md](../../../AGENTS.md)):
     1. **Typecheck:** Confirm zero compilation/type errors (`npm run typecheck`).
     2. **Unit Tests:** Confirm 100% pass rate on touched and new modules (`npm test`).
     3. **Regression Tests:** Verify full test suite passes with zero regressions.
     4. **Contract Adherence:** Confirm implementation strictly matches approved interfaces in `PLAN-XXX.md`.
     5. **Zero-LaTeX Audit:** Grep touched files for unescaped dollar signs (`$...$`, `$$...$$`) or LaTeX macros (`\dot`, `\frac`, `\approx`, `\Omega`, etc.). Confirm pure ASCII/plain text notation.
     6. **Downward Reference Audit:** Confirm `docs/ssot/` maintains zero upward path references to source or build configs (`src/`, `tests/`, `package.json`, etc.).
     7. **Plan Checklist & Clean Git Status:** Verify all steps are ticked (`- [x]`) and no lingering scratch/debug artifacts remain.
2. **Review QA Report:**
   - Wait for `code-reviewer` to report back with explicit sign-off approval.
   - If any audit point fails, halt the archive workflow and surface actionable findings for correction.

### Phase 2: Retrospective & Frontmatter Update

1. **Log Deviations & Retrospective:**
   - Review Section 5 (Deviations & Retrospective) in `PLAN-XXX.md`.
   - Prompt the user or record any runtime pivots, trade-offs, or follow-ups.
2. **Update Frontmatter:**

   ```yaml
   ---
   id: PLAN-XXX
   title: "[Short, Descriptive Title]"
   status: completed # completed | superseded | abandoned
   author: "[Author / Agent Name]"
   created: YYYY-MM-DD
   updated: YYYY-MM-DD
   completed_at: YYYY-MM-DD
   branch: "[branch-name]"
   ---
   ```

### Phase 3: File Relocation & Ledger Update

1. **Create Year Folder:**
   - Ensure destination directory exists: `.agents/plans/archive/YYYY/` (e.g. `.agents/plans/archive/2026/`).
2. **Move Plan File:**
   - Move `.agents/plans/active/PLAN-XXX.md` to `.agents/plans/archive/YYYY/PLAN-XXX.md`.
3. **Update Archive Ledger:**
   - Append the plan entry to [.agents/plans/archive/INDEX.md](../../plans/archive/INDEX.md) table with ID, Title, Status, Author, Dates, and Link.
