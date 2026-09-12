# Historical Plan Ledger

> **Location:** `.agents/plans/archive/INDEX.md`
> **Purpose:** Master index and historical record of all completed, superseded, and abandoned architectural plans.

---

## 📚 Archive Directory Structure

Archived plans are categorized by year under `.agents/plans/archive/YYYY/PLAN-XXX.md`.

```text
.agents/plans/archive/
├── INDEX.md                 # Master historical ledger (this file)
└── 2026/
    └── PLAN-001.md          # Completed / archived plan record
```

---

## 🗂️ Master Plan Index

| Plan ID | Title | Status | Author | Created | Completed | Archive Path |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| *No archived plans yet* | — | — | — | — | — | — |

---

## 📝 Archival Workflow Instructions

When completing, superseding, or abandoning an active plan:

1. Ensure all test criteria in the plan are met and verified.
2. Prompt for and log any architectural deviations in Section 5 of the plan.
3. Update the YAML frontmatter:

   ```yaml
   status: completed # completed | superseded | abandoned
   completed_at: YYYY-MM-DD
   ```

4. Move the file from `.agents/plans/active/PLAN-XXX.md` to `.agents/plans/archive/YYYY/PLAN-XXX.md`.
5. Add a new entry to the table above.
