---
name: discuss
description: >-
  Freeform ideation, architectural discussion, and iterative research backed by a live, editable session scratchpad in .agents/work/SCRATCHPAD-<slug>.md.
---

# `/discuss` — Freeform Ideation, Research & Live Scratchpad

> **Workflow Command:** `/discuss`
> **Purpose:** Collaborate with the user to explore ideas, weigh trade-offs, run grounded research spikes, and converge on architectural choices, roadmap items, or implementation plans while maintaining a live scratchpad in `.agents/work/SCRATCHPAD-<slug>.md`.

---

## 🎯 Workflow Behavior

### 1. Default Memory Palace Grounding
- Check [AGENTS.md](../../../AGENTS.md) for the bound `Project Entity Node` (`[[YYMMDD_ORG_ProjectName]]`).
- Consult relevant standing notes in `G:\My Drive\06_Memory Palace\Knowledge\` (`Entities/Projects/`, `Distilled/Design_And_Development/`, `Distilled/Research_And_Review/`, `Distilled/Standards_And_Guidance/`) so the discussion builds on existing project truth without hallucinating.

### 2. Initialize & Maintain the Live Scratchpad (`.agents/work/SCRATCHPAD-<slug>.md`)
- Create or open `.agents/work/SCRATCHPAD-<slug>.md` (or reuse an active scratchpad in `.agents/work/`).
- Treat the scratchpad as a **living document** that you can edit, reorganize, and refine at will throughout the conversation as ideas evolve:

```markdown
# Session Scratchpad: <Topic>
> **Date:** YYYY-MM-DD | **Project:** [[YYMMDD_ORG_ProjectName]]

## 1. Active Discussion & Working Ideas
- [Current problem framing, candidate approaches, and evolving thoughts]

## 2. Research Findings & Trade-Off Context
- [Grounded facts, primary source URLs, plain-text formulas, and option comparisons]

## 3. Decided Outcomes & Architectural Choices (SSOT)
- [Agreed architectural rules, contracts, or rejected alternatives + rationale]

## 4. Roadmap Items, Parked Ideas & Next Steps
- [ ] [Immediate next steps or deferred ideas to track in the Memory Palace]
```

### 3. Seamless In-Place Research (`research` Subagent & Web/Code Lookups)
- Whenever the conversation requires deeper context, library comparisons, or mathematical derivations:
  - Run targeted searches or invoke Antigravity's built-in `research` subagent.
  - **Append or edit findings directly into Section 2 of the active `.agents/work/SCRATCHPAD-<slug>.md`** (never spawn separate rigid research files unless requested).
  - Strictly enforce Zero-LaTeX plain-text math notation (`a = a0 + a_dot * T`, never dollar-sign delimiters).

### 4. Convergence & Hand-Off
- As the back-and-forth loop reaches decisions on how to proceed, update Sections 3 & 4 of `.agents/work/SCRATCHPAD-<slug>.md` so nothing is lost in chat history.
- From here, the user can seamlessly:
  - Continue iterating and researching in chat,
  - Trigger Antigravity's built-in `/plan` command to implement a decided feature (using the scratchpad as context), or
  - Run `/push-to-memory` to commit the session's ideas, research, roadmap items, and architectural choices to `06_Memory Palace`.
