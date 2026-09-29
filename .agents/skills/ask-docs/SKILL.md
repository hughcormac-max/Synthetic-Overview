---
name: ask-docs
description: Query internal project documentation exclusively within .agents/docs/research/ and .agents/docs/ssot/, refusing to answer from pre-trained knowledge if the answer is missing.
---

# `/ask-docs` Workflow Trajectory

> **Workflow Command:** `/ask-docs`
> **Purpose:** Query project documentation exclusively within `.agents/docs/research/` and `.agents/docs/ssot/` with zero hallucination.

---

## 🎯 Workflow Execution Steps

1. **Clarify Query Scope:**
   - Extract the specific domain concept, constant, formula, or rule the user is inquiring about.
2. **Execute Scoped Search:**
   - Search exclusively within `.agents/docs/research/` and `.agents/docs/ssot/`.
   - Do not search outside these directories.
   - Do not draw upon pre-trained knowledge to answer domain questions.
3. **Verify Citations & Sourcing:**
   - Every factual answer MUST cite the source document directly (e.g. `[.agents/docs/ssot/SSOT-0001.md](...)`).
   - Quote exact definitions, formulas, or constants from the document.
4. **Anti-Hallucination Fallback:**
   - If the requested information is absent or ambiguous in `.agents/docs/research/` and `.agents/docs/ssot/`, state clearly:
     *"Information not available in SSOT/Research docs."*
   - Suggest running `/research` to gather knowledge or `/define-ssot` to establish domain truth.
5. **Format Response:**
   - Present answer in clean markdown with clickable links, maintaining strict Zero-LaTeX plain-text math notation.
