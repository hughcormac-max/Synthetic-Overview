---
name: ask-docs
description: Query authoritative project documentation in G:\My Drive\06_Memory Palace\Knowledge\ and local staging files in .agents/work/, refusing to answer from pre-trained knowledge if the answer is missing.
---

# `/ask-docs` Workflow Trajectory

> **Workflow Command:** `/ask-docs`
> **Purpose:** Query project domain truth, research syntheses, and system specifications across `G:\My Drive\06_Memory Palace\Knowledge\` and active staging files in `.agents/work/` with zero hallucination.

---

## 🎯 Workflow Execution Steps

1. **Clarify Query & Resolve Project Node:**
   - Extract the specific domain concept, constant, formula, or architectural rule the user is inquiring about.
   - Check [AGENTS.md](../../../AGENTS.md) for the bound Memory Palace `Project Entity Node` (or locate the matching project in `G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\`).

2. **Execute Scoped Search:**
   - Delegate to the `researcher` subagent (or read directly for targeted lookups) across:
     1. `G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\` (Project Map of Content & scope)
     2. `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Design_And_Development\` (Committed project specs & ADRs)
     3. `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Research_And_Review\` (Domain syntheses, derivations & trade-off analyses)
     4. `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Standards_And_Guidance\` (Cross-project engineering/coding standards)
     5. `.agents/work/` (Any in-flight `RESEARCH-*.md` or `PLAN-*.md` staged in the current session)
   - Ignore notes marked `status: superseded` unless the user asks about historical design evolution.
   - Do not draw upon pre-trained knowledge to invent project-specific rules.

3. **Verify Citations & Sourcing:**
   - Every factual answer MUST cite the exact Memory Palace node (`[[Node_Name]]`) or staged file path with a clickable link.
   - Quote exact definitions, plain-text formulas, or constants from the source note.

4. **Anti-Hallucination Fallback:**
   - If the requested information is absent or ambiguous in both `06_Memory Palace\Knowledge\` and `.agents/work/`, state clearly:
     *"Information not available in Memory Palace Knowledge or local `.agents/work/` staging docs."*
   - Suggest running `/research` to conduct a grounded research spike.

5. **Format Response:**
   - Present the answer in clean markdown with clickable `file:///` links, maintaining strict Zero-LaTeX plain-text math notation.
