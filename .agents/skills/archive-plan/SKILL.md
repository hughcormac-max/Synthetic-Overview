---
name: archive-plan
description: Verify completion criteria, perform dead-code removal, cleanliness, and blast-radius safety checks via code-reviewer, log architectural retrospective notes, update plan metadata, and move active plans into the permanent historical archive ledger.
---

# `/archive-plan` Workflow Trajectory

> **Workflow Command:** `/archive-plan`
> **Purpose:** Verify completion criteria, conduct code cleanliness, dead-code removal, and blast-radius safety audits via `code-reviewer`, log architectural retrospective notes, update plan metadata, and move active plans into the permanent historical archive ledger.

---

## 🎯 Workflow Execution Steps

### Phase 1: Verification, Cleanliness & Safety Gate (via `code-reviewer`)

Delegate comprehensive QA verification and safety auditing to the `code-reviewer` subagent ([.agents/subagents/code-reviewer.md](../../subagents/code-reviewer.md)) in read-only audit mode (`mode: inherit`):

1. **Invoke Code Reviewer Subagent (`code-reviewer`):**
   - Read `.agents/subagents/code-reviewer.md` to confirm QA audit parameters.
   - Dispatch the `code-reviewer` subagent via `invoke_subagent` (with `Workspace: "inherit"`).
   - Instruct the subagent to execute the following safety and QA checks:
     1. **Dead-Code Removal & Cleanliness Audit:** Confirm that all unused functions, obsolete structs/fields, orphan imports, commented-out code, and temporary debug statements have been removed. Verify code cleanliness and formatting.
     2. **Blast-Radius Safety Check:** Verify that modified code does not unintentionally break adjacent crates/modules, alter unapproved public API surfaces, introduce ECS schedule conflicts, or leak mutable state across system boundaries.
     3. **Compilation & Typecheck:** Confirm clean build with zero errors and zero warnings (`cargo check --workspace --all-targets`).
     4. **Linter & Clippy Cleanliness:** Confirm zero clippy warnings (`cargo clippy --workspace --all-targets -- -D warnings`).
     5. **Unit, Integration & Property Tests:** Confirm 100% pass rate across all suites, including invariant property-based tests (`cargo test --workspace`).
     6. **Contract Adherence:** Confirm implementation strictly satisfies the approved technical contracts and invariants in Section 3 of `PLAN-XXX.md`.
     7. **Zero-LaTeX Audit:** Grep touched files for unescaped dollar signs (`$...$`, `$$...$$`) or LaTeX macros (`\dot`, `\frac`, `\approx`, `\Omega`, etc.). Confirm pure ASCII/plain text notation.
     8. **Downward Reference Audit:** Confirm `docs/ssot/` maintains zero upward path references to source or build configs (`src/`, `tests/`, `crates/`, `Cargo.toml`, etc.).
     9. **Plan Checklist & Clean Git Status:** Verify all steps are ticked (`- [x]`) and no lingering scratch files remain.
2. **Review QA Report:**
   - Wait for `code-reviewer` to report back with explicit sign-off approval.
   - If any dead code, blast-radius risk, test failure, or invariant breach is reported, halt the archive workflow and surface actionable findings for correction.

### Phase 2: Retrospective & Frontmatter Update

1. **Log Deviations & Retrospective:**
   - Review Section 6 (Deviations & Retrospective) in `PLAN-XXX.md`.
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
