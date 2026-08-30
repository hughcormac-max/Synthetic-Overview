# `/execute` Workflow Trajectory

> **Workflow Command:** `/execute`
> **Purpose:** Sequentially implement an approved active plan from `.antigravity/plans/active/`, executing test assertions at each atomic step and maintaining synchronized checklist progress.

---

## 🎯 Workflow Execution Steps

```
+-------------------+     +-------------------------+     +-----------------------+
| 1. Ingest Plan    | --> | 2. Sequential Step      | --> | 3. Verify & Mark      |
|    & Verify State |     |    Implementation       |     |    Checklist Done     |
+-------------------+     +-------------------------+     +-----------------------+
                                       ^                              |
                                       +------------------------------+
                                            (Repeat for all steps)
```

### Phase 1: Ingest Active Plan

1. **Load Active Plan:**
   - Locate the target approved plan in `.antigravity/plans/active/PLAN-XXX.md`.
   - Verify that frontmatter status is `approved` or `in-progress`.
   - Update frontmatter status to `in-progress` if not already set.

### Phase 2: Sequential Step Execution

For each unchecked step (`- [ ]`) in Section 3 of `PLAN-XXX.md`:

1. **Implement Atomic Code Change:**
   - Write or update code strictly conforming to the interfaces defined in Section 2.
   - Respect layer boundaries ([.antigravity/rules/00-core-architecture.md](file:///.antigravity/rules/00-core-architecture.md)).
   - Follow naming and immutability standards ([.antigravity/rules/01-coding-standards.md](file:///.antigravity/rules/01-coding-standards.md)).
2. **Author & Run Associated Tests:**
   - Write corresponding unit tests in accordance with [.antigravity/rules/02-verification.md](file:///.antigravity/rules/02-verification.md).
   - Execute the test runner (e.g. `npm test`) to confirm tests pass hermetically.
3. **Synchronize Checklist:**
   - Mark the step as complete (`- [x]`) in `.antigravity/plans/active/PLAN-XXX.md`.

### Phase 3: Final Verification Triad

1. **Run Full Verification:**
   - Run typechecker (`npm run typecheck`).
   - Run linter (`npm run lint`).
   - Run all regression tests (`npm test`).
2. **Review Checklist:**
   - Ensure all steps are ticked before preparing to trigger `/archive-plan`.
