---
name: plan-robot
description: Use this skill after /plan-human and human-in-the-loop review to fill in technical specifications, contracts, file-by-file mapping, unit test targets, and atomic steps (<= 50 lines each) in an existing plan.
---

# `/plan-robot` Workflow Trajectory

> **Workflow Command:** `/plan-robot`
> **Purpose:** Ingest a human-reviewed and approved plan to formulate technical contracts, schemas, module mappings, and atomic tasks with a strict limit: **any step larger than approximately 50 lines must be decomposed further**.

---

## 🎯 Workflow Execution Steps

### Phase 1: Ingest Aligned Plan & Domain Truth

1. **Locate Target Plan:**
   - Read `.agents/docs/plans/PLAN-XXX.md`.
   - Verify that Part 1 (Human Problem Alignment) is populated and has been explicitly reviewed and approved by the human developer.
2. **Scan Codebase & Domain References:**
   - Query `.agents/docs/ssot/` for applicable domain truths, formulas, constants, or golden vectors.
   - Inspect existing codebase interfaces, data structures, and caller sites.
   - Enforce master architectural rules from [AGENTS.md](../../../AGENTS.md).

### Phase 2: Populate Part 2 (Robot Technical Requirements)

1. **Section 2.1 Authoritative Domain References:**
   - Cite relevant `.agents/docs/ssot/SSOT-NNNN.md` documents.
2. **Section 2.2 Data Types & Schemas:**
   - Declare exact, strictly-typed public schemas, interfaces, and immutable data models.
3. **Section 2.3 Public API / Service Signatures:**
   - Define exact function signatures, argument types, and explicit Result/Error types.
4. **Section 2.4 File-by-File Module Mapping:**
   - Group by layer (Domain, Application, Infrastructure, Presentation) with exact target file paths.
5. **Section 2.5 Atomic Implementation Slices (STRICT <= ~50 LINES RULE):**
   - Break implementation tasks into an ordered checklist (`- [ ]`).
   - **MANDATORY INVARIANT:** Each atomic step must target a single file or isolated unit and represent no more than ~50 lines of code changes. Any task estimated or drafted at > 50 lines must be split into sub-steps before finalizing.
6. **Section 2.6 Unit Test Targets & Verification Commands:**
   - List each module alongside its test file and test scenarios (including edge cases and golden vectors).
   - Configure active verification commands matching project stack (typecheck, lint, test, build).

### Phase 3: Technical Summary & Developer Approval

1. **Update Plan Metadata:**
   - Set `updated: YYYY-MM-DD`.
2. **Present Technical Blueprint in Chat:**
   - Summarize key types, public APIs, affected files, and atomic task breakdowns.
3. **Approval Gate (HALT):**
   - **STRICT HALT:** Await explicit developer approval. Upon approval, update frontmatter:
     ```yaml
     status: approved
     ```
   - Update status in `.agents/docs/plans/INDEX.md` to `approved`.
