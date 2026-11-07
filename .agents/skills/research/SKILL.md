---
name: research
description: Conduct online and workspace research grounded in reality, generating a staged RESEARCH-<slug>.md report in .agents/work/ formatted for downstream distillation into the Memory Palace.
---

# `/research` Workflow Trajectory

> **Workflow Command:** `/research`
> **Purpose:** Perform deep research grounded in reality using web search and codebase inspection, extract non-hallucinated findings with primary citations, and stage a structured report in `.agents/work/RESEARCH-<slug>.md` ready for export to `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\` via `/close-to-palace`.

---

## 🎯 Workflow Execution Steps

1. **Clarify Objective & Check Existing Palace Research:**
   - Establish the specific research goal, target domain, and trade-off questions from the user prompt.
   - Check `G:\My Drive\06_Memory Palace\Knowledge\Distilled\Research_And_Review\` to see if a standing note already covers part of this topic so the spike can build on or refine it.

2. **Execute Grounded Research:**
   - Delegate to the `researcher` subagent (reads `.agents/subagents/researcher.md` role rules) or execute targeted web and workspace searches.
   - Extract raw, non-hallucinated facts, documentation excerpts, plain-text mathematical derivations, and multi-option trade-off comparisons.
   - Collect exact primary source URLs for all external findings.

3. **Synthesize Staged Report using Template:**
   - Populate the template from [.agents/skills/research/assets/TEMPLATE.md](assets/TEMPLATE.md) (whose sections align directly with `TPL_Research_And_Review.md` in `06_Memory Palace`).
   - Adhere strictly to the project's Zero-LaTeX plain-text math rules (never use `$` or `$$` delimiters).

4. **Save to Ephemeral Workbench (`.agents/work/`):**
   - Save the report to `.agents/work/RESEARCH-<slug>.md` (using a concise kebab-case `<slug>`, e.g., `.agents/work/RESEARCH-keplerian-solver.md`).
   - Note that this file serves as working context during the active coding session and will be packaged into `06_Memory Palace\Inbox\Context_Dumps\` and purged from the repo when `/close-to-palace` runs.

5. **Notify User:**
   - Provide a concise summary of the key findings and trade-offs in chat with a clickable link to `.agents/work/RESEARCH-<slug>.md`.
