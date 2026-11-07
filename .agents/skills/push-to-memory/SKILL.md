---
name: push-to-memory
description: >-
  Commit the active conversation context, live .agents/work/ scratchpad, and any session code/design outcomes into G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\, then clean up .agents/work/.
---

# `/push-to-memory` — Commit Session & Scratchpad to Memory Palace

> **Workflow Command:** `/push-to-memory`
> **Purpose:** Synthesize the session's chat discussion, `.agents/work/` scratchpad, research findings, parked ideas, and architectural/code outcomes into a structured Context Dump in `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`, and clean up `.agents/work/`.

---

## 🎯 Workflow Execution Steps

### Phase 1: Pre-Push Verification (Only When Code Was Modified)
If source code or tests were modified during the session:
1. Run the project's standard verification commands defined in [AGENTS.md](../../../AGENTS.md) (typecheck, lint, unit tests).
2. Audit modified files and scratchpad notes for strict **Zero-LaTeX compliance** (zero `$` or `$$` math delimiters or LaTeX macros).

### Phase 2: Gather Context & Resolve Memory Palace Nodes
1. **Harvest Session Sources:**
   - Combine insights from **both** the active chat conversation and any `.agents/work/*.md` scratchpad files (plus `git status` / `git diff` if code was written).
2. **Resolve Project & Existing Distilled Nodes:**
   - Identify the project node in `G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\`.
   - Scan `Knowledge/Distilled/Design_And_Development/` and `Knowledge/Distilled/Research_And_Review/` so the Context Dump can state whether new decisions create a node, accrete onto `[[Existing_Node]]`, or supersede `[[Old_Node]]`.

### Phase 3: Write Context Dump to `06_Memory Palace\Inbox\Context_Dumps\`
Write `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\YYYY_MM_DD_<ProjectSlug>_<Topic>.md` (strictly enforcing Zero-LaTeX plain-text notation):

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
action_directive: "Update project episodic log and entity roadmap/backlog, distill domain research into Research_And_Review/, and distill or update committed specs in Design_And_Development/."
---

## Context & Framing

Session push from `<Repository Path>` for `[[YYMMDD_ORG_ProjectName]]`. Captures conversational ideation, scratchpad research, roadmap decisions, and committed architectural/code outcomes.

---

## Raw Payload

### 1. Session Summary, Milestones & Immediate Next Steps (-> `Episodic/Projects/`)
* Summary of topics discussed, decisions reached, or code implemented/verified during the session.
* Completed items (`[x]`) and immediate next-step action items (`[ ]`).

### 2. Roadmap Items, Parked Ideas & Project Scope Updates (-> `Knowledge/Entities/Projects/`)
* New feature ideas, future tracks, backlog items, or scope adjustments discussed during the session to record on `[[YYMMDD_ORG_ProjectName]]`.

### 3. Domain Research, Derivations & Trade-Off Reviews (-> `Knowledge/Distilled/Research_And_Review/`)
* Full technical synthesis, literature/library evaluations, plain-text formulas, primary URLs, and multi-option trade-off analyses gathered during the session (or state "No new domain research in this session").

### 4. Committed Design & Architecture Decisions / SSOT (-> `Knowledge/Distilled/Design_And_Development/`)
* Standing architectural choices, system contracts, data schemas, or rejected alternatives + rationale decided during the session.
* Note whether this creates a new `Design_And_Development/` node, accretes onto `[[Existing_Node]]`, or supersedes `[[Old_Node]]`.
```

### Phase 4: Clean Up `.agents/work/`
1. Confirm the Context Dump file was written and is non-empty in `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`.
2. Delete the exported `.agents/work/*.md` scratchpad files (preserving `.agents/work/.gitkeep`), unless the user explicitly asks to keep the scratchpad open for the next session.
3. Report the created Context Dump path and remind the user that `/ingest-inbox` in `06_Memory Palace` will distill it into the knowledge graph.
