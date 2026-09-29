---
name: research
description: Conduct online research grounded in reality through web search and non-hallucinated knowledge, generating sequential RESEARCH-0001.md reports in .agents/docs/research/.
---

# `/research` Workflow Trajectory

> **Workflow Command:** `/research`
> **Purpose:** Perform deep research grounded in reality using web search, extract non-hallucinated findings with primary citations, and compile a structured report in `.agents/docs/research/RESEARCH-NNNN.md`.

---

## 🎯 Workflow Execution Steps

1. **Clarify Objective:**
   - Establish the specific research goal, target domain, and scope from the user prompt.
2. **Determine Sequential Research ID:**
   - Inspect `.agents/docs/research/INDEX.md` to find the highest existing `RESEARCH-NNNN` ID.
   - Increment the number to determine the new 4-digit zero-padded sequential ID (e.g. `RESEARCH-0001`, `RESEARCH-0002`).
3. **Execute Grounded Research (via `research` Subagent or Search Tools):**
   - Delegate to the native `research` subagent (or execute targeted web search using `search_web` and `read_url_content`).
   - Extract raw, non-hallucinated facts, documentation excerpts, and verifiable domain data.
   - Collect exact primary source URLs for all findings.
4. **Synthesize Report using Template:**
   - Ingest findings into the template at `.agents/skills/research/resources/template.md`.
   - Adhere strictly to the project's Zero-LaTeX plain-text math rules.
   - Include the JSON `Index Metadata` block at the end of the report.
5. **Save Report & Update Ledger:**
   - Save to `.agents/docs/research/RESEARCH-NNNN.md`.
   - Append a new row to `.agents/docs/research/INDEX.md` with ID, title, date, objective, conclusions, and link.
6. **Notify User:**
   - Provide a concise summary of the key findings in chat and a direct link to the new report.
