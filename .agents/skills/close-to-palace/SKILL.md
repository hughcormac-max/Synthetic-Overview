---
name: close-to-palace
description: >-
  Close out a coding, research, or design session in a project workspace by packaging the session's progress, domain research/trade-off reviews, and architectural decisions into a structured Context Dump in G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\, then purging transient research and scratch markdown files from the code repository.
---

# `/close-to-palace` Workflow Trajectory

> **Workflow Command:** `/close-to-palace`
> **Purpose:** Shift session progress, text-heavy domain research (`Research_And_Review`), and committed architectural specifications (`Design_And_Development`) from the code workspace into `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`, and purge transient research or completed scratch markdown files from the Git repository.

---

## 🎯 Workflow Execution Steps

### Phase 1: Resolve Project & Entity Context in Memory Palace

1. **Locate Project Entity:**
   - Check `G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\` to resolve the canonical project node for the current repository (e.g., `[[260830_HOC_Synthetic_Overview]]` for `C:\GitHub\Personal\Synthetic-Overview`).
2. **Inspect Existing Distilled Nodes:**
   - Check `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Design_And_Development\` and `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Research_And_Review\` for existing notes bound to this project so the context dump can reference them for accretion or superseding.

### Phase 2: Gather Session Artifacts & Ephemeral Repo Docs

1. **Collect Session Findings:**
   - Review the current session's code changes, architectural discussions, benchmarks, and any transient markdown documents (e.g., `.agents/docs/research/`, spike notes, literature reviews, trade-off matrices, or completed plans) in the project workspace.
2. **Classify Permanent vs. Ephemeral Files:**
   - **Permanent Repository Files:** `README.md`, `AGENTS.md`, source code, automated tests, build configurations, and active compiler/API contracts.
   - **Ephemeral Session Docs:** Raw research essays, multi-option trade-off explorations, and completed scratch plans that belong in the Memory Palace rather than accumulating in Git.

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

Session closeout from `<Repository Path>` for `[[YYMMDD_ORG_ProjectName]]`. Shifts raw research findings, trade-off analyses, and architectural decisions from the code workspace into the Memory Palace as the permanent human-facing Source of Truth.

---

## Raw Payload

### 1. Session Execution & Milestones (-> `Episodic/Projects/`)
* Summary of code implemented, refactored, or verified during the session.
* Completed deliverables (`[x]`) and remaining next-step action items (`[ ]`).

### 2. Domain Research, Derivations & Trade-Off Reviews (-> `Knowledge/Distilled/Research_And_Review/`)
* Preserve the complete technical synthesis, literature/crate reviews, plain-text formulas, and multi-option trade-off comparisons (e.g., Option A vs. Option B) explored during the spike.
* Explicitly note which downstream design node(s) this research `supports` or is a `prerequisite_of`.

### 3. Committed Design & Architecture Decisions (-> `Knowledge/Distilled/Design_And_Development/`)
* Document the chosen system specification, data layout, or architectural contract committed for `[[YYMMDD_ORG_ProjectName]]`.
* State whether this creates a new `Design_And_Development/` node, accretes onto an existing node (`[[Existing_Node]]`), or supersedes a previous design decision (`refutes: ["[[Old_Node]]"]`).
```

### Phase 4: Purge Ephemeral Docs & Verify Clean Repo State

1. **Verify Context Dump Integrity:**
   - Confirm that the Context Dump file exists and is non-empty in `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`.
2. **Purge Transient Markdown Docs:**
   - Delete the transient raw research markdown files and completed scratch docs from the project repository (with user confirmation if deleting tracked files) so the Git workspace stays lightweight and focused on executable code.
3. **Present Closeout Summary:**
   - Report the created Context Dump path in `06_Memory Palace` and the list of purged repository files, noting that running `/ingest-inbox` in `06_Memory Palace` will complete distillation into the knowledge graph.
