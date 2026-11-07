---
name: close-to-palace
description: >-
  Run the Pre-Closeout 7-Point QA Checklist, package the session's progress, staged research spikes (.agents/work/RESEARCH-*.md), and committed architectural decisions (.agents/work/PLAN-*.md) into a structured Context Dump in G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\, and purge transient files from .agents/work/.
---

# `/close-to-palace` Workflow Trajectory

> **Workflow Command:** `/close-to-palace`
> **Purpose:** Verify the 7-Point QA Checklist, export session progress (`Episodic/Projects/`), domain research (`Research_And_Review/`), and committed architectural specifications (`Design_And_Development/`) into `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`, and purge transient `.agents/work/*.md` files from the Git repository.

---

## 🎯 Workflow Execution Steps

### Phase 1: Pre-Closeout 7-Point QA Gate (When Code Was Modified)

If source code or tests were modified during the session or via `/execute`, verify the **7-Point QA Checklist** ([AGENTS.md](../../../AGENTS.md)):
1. **Typecheck:** Confirm zero compilation or type errors.
2. **Unit Tests:** Confirm 100% pass rate on touched and newly created modules.
3. **Regression Tests:** Verify full test suite passes with zero regressions.
4. **Contract Adherence:** Confirm implementation matches approved contracts in `.agents/work/PLAN-<slug>.md` and cited Memory Palace `Design_And_Development/` specs.
5. **Zero-LaTeX Audit:** Audit touched files and staged markdown for unescaped dollar signs (`$...$`, `$$...$$`) or LaTeX macros (`\dot`, `\frac`, `\approx`, `\Omega`, etc.). Confirm pure ASCII/Unicode plain-text notation.
6. **Plan Checklist & Retrospective Sync:** Ensure all completed steps in `.agents/work/PLAN-<slug>.md` are ticked (`- [x]`) and any runtime deviations or follow-up items are recorded in Sections 13 & 14.
7. **Clean Workspace Check:** Confirm no lingering debug statements or unintended scratch files exist in source directories.

*(If any QA check fails on modified code, halt and surface actionable findings before exporting.)*

---

### Phase 2: Resolve Project & Entity Context in Memory Palace

1. **Locate Project Entity:**
   - Check [AGENTS.md](../../../AGENTS.md) or scan `G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\` to resolve the canonical project node for the current repository (e.g., `[[260830_HOC_Synthetic_Overview]]`).
2. **Inspect Existing Distilled Nodes:**
   - Check `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Design_And_Development\` and `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Research_And_Review\` for existing notes bound to this project so the Context Dump can explicitly specify whether to create a new node, accrete onto `[[Existing_Node]]`, or supersede/refute `[[Old_Node]]`.
3. **Gather Session Artifacts from `.agents/work/`:**
   - Collect all staged `.agents/work/RESEARCH-*.md` reports and `.agents/work/PLAN-*.md` blueprints alongside the session's git diff and commit summary.

---

### Phase 3: Write Structured Context Dump to `06_Memory Palace`

1. **Generate Context Dump File:**
   - Write `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\YYYY_MM_DD_<ProjectSlug>_<Topic>.md` using the canonical schema below (strictly enforcing Zero-LaTeX—render all formulas and units in clean ASCII/Unicode plain text or inline code):

```markdown
---
title: "YYYY-MM-DD <Project Name> — <Session Topic>"
type: context-dump
ingest_status: pending
date: "YYYY-MM-DD"
target_domain:
  - creative
related_hints:
  - "[[YYMMDD_ORG_ProjectName]]"
  - "[[HOC]]"
action_directive: "Generate project episodic log, distill domain research into Research_And_Review/, and distill or update committed specs in Design_And_Development/."
---

## Context & Framing

Session closeout from `<Repository Path>` for `[[YYMMDD_ORG_ProjectName]]`. Shifts raw research findings, trade-off analyses, and committed architectural decisions from the code workspace into the Memory Palace as the permanent human-facing Source of Truth.

---

## Raw Payload

### 1. Session Execution & Milestones (-> `Episodic/Projects/`)
* Summary of code implemented, refactored, or verified during the session (including any runtime deviations from Section 13 of the plan).
* Completed deliverables (`[x]`) and remaining next-step action items (`[ ]`).

### 2. Domain Research, Derivations & Trade-Off Reviews (-> `Knowledge/Distilled/Research_And_Review/`)
* Preserve the complete technical synthesis, literature/crate reviews, plain-text formulas, and multi-option trade-off comparisons from `.agents/work/RESEARCH-*.md` (or state "No new domain research spikes in this session" if none).
* Explicitly note which downstream design node(s) this research `supports` or is a `prerequisite_of`.

### 3. Committed Design & Architecture Decisions (-> `Knowledge/Distilled/Design_And_Development/`)
* Document the chosen system specification, data layout, or architectural contract committed for `[[YYMMDD_ORG_ProjectName]]` during this session.
* State whether this creates a new `Design_And_Development/` node, accretes onto an existing node (`[[Existing_Node]]`), or supersedes a previous design decision (`refutes: ["[[Old_Node]]"]`).
```

---

### Phase 4: Purge `.agents/work/` & Verify Clean Repo State

1. **Verify Context Dump Integrity:**
   - Confirm that the Context Dump file exists and is non-empty in `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`.
2. **Purge Transient Staging Docs:**
   - Delete the exported `.agents/work/*.md` files (preserving `.agents/work/.gitkeep`) so the Git workspace stays lightweight and focused strictly on executable code, tests, and build configs.
3. **Present Closeout Summary:**
   - Report the created Context Dump path in `06_Memory Palace` and the list of purged `.agents/work/` files, noting that running `/ingest-inbox` in `06_Memory Palace` will complete distillation into the knowledge graph.
