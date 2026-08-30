# `/archive-plan` Workflow Trajectory

> **Workflow Command:** `/archive-plan`
> **Purpose:** Verify completion criteria, log architectural retrospective notes, update plan metadata, and move active plans into the permanent historical archive ledger.

---

## 🎯 Workflow Execution Steps

```
+---------------------+     +--------------------------+     +------------------------+
| 1. Subagent QA &    | --> | 2. Prompt Retrospective  | --> | 3. Relocate to Archive |
|    Regression Gate  |     |    & Update Metadata     |     |    & Update Ledger     |
+---------------------+     +--------------------------+     +------------------------+
```

### Phase 1: Verification & QA Gate

1. **Execute 7-Point Verification Checklist:**
   - Confirm all unit tests and full regression test suites pass cleanly.
   - Confirm zero typecheck errors and zero lint warnings.
   - Confirm all checklist boxes (`- [x]`) are checked in `.antigravity/plans/active/PLAN-XXX.md`.
   - Verify zero LaTeX math delimiters (`$...$`, `$$...$$`, `\dot{}`, etc.) across touched files.

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
   - Ensure destination directory exists: `.antigravity/plans/archive/YYYY/` (e.g. `.antigravity/plans/archive/2026/`).
2. **Move Plan File:**
   - Move `.antigravity/plans/active/PLAN-XXX.md` to `.antigravity/plans/archive/YYYY/PLAN-XXX.md`.
3. **Update Archive Ledger:**
   - Append the plan entry to [.antigravity/plans/archive/index.md](file:///.antigravity/plans/archive/index.md) table with ID, Title, Status, Author, Dates, and Link.
