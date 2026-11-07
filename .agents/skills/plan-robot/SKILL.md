---
name: plan-robot
description: Use this skill after /plan-human to fill in the technical specifications, Memory Palace references, contracts, file-by-file mapping, unit test targets, and atomic pseudocode steps in .agents/work/PLAN-<slug>.md.
---

# `/plan-robot` Workflow Trajectory

> **Workflow Command:** `/plan-robot`
> **Purpose:** Ingest a human-aligned plan in `.agents/work/PLAN-<slug>.md`, ground it against `G:\My Drive\06_Memory Palace\Knowledge\Distilled\`, and formulate technical contracts, schemas, module mappings, and atomic tasks defined by strict pseudocode.

---

## 🎯 Workflow Execution Steps

### Phase 1: Ingest Aligned Plan & Memory Palace Truth

1. **Locate Target Plan & Verify Human Approval:**
   - Read `.agents/work/PLAN-<slug>.md`.
   - **PREREQUISITE CHECK:** Verify that Part 1 (Human Problem Alignment) is complete and has been explicitly approved by the human developer. If Part 1 is unpopulated or pending human review, halt and request the user to review or invoke `/plan-human` first.
2. **Scan Memory Palace & Codebase Interfaces:**
   - Query `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Design_And_Development\` and `Research_And_Review\` (plus any staged `.agents/work/RESEARCH-*.md` spikes) for applicable domain rules, plain-text formulas, constants, or architectural contracts.
   - Inspect existing codebase interfaces, data structures, and caller sites.
   - Enforce master architectural rules from [AGENTS.md](../../../AGENTS.md).

### Phase 2: Populate Part 2 (Robot Technical Specifications)

1. **Section 7 Authoritative Memory Palace & Research References:**
   - Cite relevant `[[Design_And_Development_Node]]` and `[[Research_And_Review_Node]]` documents in `06_Memory Palace` (or staged `.agents/work/RESEARCH-*.md` files).
2. **Section 8 Domain Types & Data Contracts:**
   - Declare exact, strictly-typed public schemas, interfaces, and immutable data models.
3. **Section 9 Public API / Service Signatures:**
   - Define exact function signatures, argument types, and explicit Result/Error types.
4. **Section 10 File-by-File Module Mapping:**
   - Group by layer (Domain, Application, Infrastructure, Presentation) with exact target file paths.
5. **Section 11 Atomic Logical Tasks (PSEUDOCODE DRIVEN):**
   - Break implementation tasks into an ordered checklist (`- [ ]`).
   - **MANDATORY INVARIANT:** For each task, define explicit **pseudocode** or algorithmic steps. This pseudocode acts as the strict rubric for the `implementer` and `code-reviewer`. Avoid vague requirements.
6. **Section 12 Unit Test Targets & Verification Commands:**
   - List each module alongside its test file and test scenarios (including edge cases and golden vectors).
   - Configure active verification commands matching the project stack (typecheck, lint, test, build).
7. **Section 14 Memory Palace Distillation Directives:**
   - Note whether completing this plan will create a new `Design_And_Development/` node, accrete onto an existing node, or supersede/refute an older design decision when `/close-to-palace` runs.

### Phase 3: Technical Summary & Developer Approval

1. **Update Plan Metadata:**
   - Set `updated: YYYY-MM-DD`.
2. **Present Technical Blueprint in Chat:**
   - Summarize key types, public APIs, affected files, and atomic task breakdowns with a clickable link to `.agents/work/PLAN-<slug>.md`.
3. **Approval Gate (HALT):**
   - **STRICT HALT:** Await explicit developer approval. Upon approval, update frontmatter:
     ```yaml
     status: approved
     ```
