---
name: ask-docs
description: Use this skill when the user asks a domain question or uses the /ask-docs command to query internal documentation. It orchestrates the doc-researcher subagent to search exclusively within docs/research/ and docs/ssot/, refusing to answer from pre-trained knowledge if the answer is missing.
---

# Ask Docs Workflow

This skill defines the workflow for querying project documentation (`docs/research/` and `docs/ssot/`) using a strictly bounded, anti-hallucinatory subagent.

## Workflow Steps

When the user asks a question via `/ask-docs` or requests information from internal documents, execute the following steps:

1. **Clarify Query Context**:
   - Extract the core domain question, technical concepts, constants, or formulas the user is inquiring about.

2. **Invoke Document Researcher (`doc-researcher`)**:
   - Read `.agents/subagents/doc-researcher.md` to ensure the subagent's rules and constraints are understood.
   - Invoke the `doc-researcher` subagent using `invoke_subagent` (or `define_subagent` if registering dynamically in the active session).
   - Instruct the subagent with the user's specific query.
   - Explicitly remind it:
     - Only search within `docs/research/` and `docs/ssot/`.
     - Do not rely on pre-trained knowledge or guess.
     - Cite exact documents (`[SSOT-NNNN.md](...)` or `[RESEARCH-NNNN.md](...)`) for every claim.
     - If the information is not present or ambiguous, state: *"Information not available in SSOT/Research docs."*

3. **Receive and Validate Findings**:
   - Wait for `doc-researcher` to report back.
   - Verify that all cited paths exist within `docs/research/` or `docs/ssot/`.
   - Verify that Zero-LaTeX rules are maintained (plain-text/ASCII math only).

4. **Present Answer to User**:
   - Deliver the retrieved information with clickable markdown links to the source documentation files.
   - If the information was missing, clearly explain what was searched and suggest either running deep research (`/research`) or establishing a new SSOT (`/define-ssot`) if domain truth needs to be added.

## Invariants

- **Zero Hallucination:** The agent and subagent must never fabricate details or draw conclusions unsupported by the documentation.
- **Strictly Bounded Scope:** Searches must remain strictly within `docs/research/` and `docs/ssot/`.
- **Zero LaTeX:** All formulas or mathematical terms must use clean plain-text/ASCII notation.

