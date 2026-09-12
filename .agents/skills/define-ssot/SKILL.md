---
name: define-ssot
description: Use this skill when the user asks to establish a new Single Source of Truth (SSOT) from raw research or domain knowledge. It orchestrates the ssot-writer subagent to format the knowledge and saves it to docs/ssot/.
---

# Define SSOT Workflow

This skill defines the workflow for converting raw domain knowledge, research reports, or specs into an immutable Tier 0 `SSOT-NNNN` document.

## Workflow Steps

When the user requests to define an SSOT, execute the following steps sequentially:

1. **Gather Raw Knowledge**:
   - Identify the source material. The user may point you to an existing `RESEARCH-NNNN.md` file, a web URL, or provide raw text.
   - If you need to read a file, use `read_file` to ingest its contents.
2. **Invoke SSOT Writer (`ssot-writer`)**:
   - Read `.agents/subagents/ssot-writer.md` and the template at `.agents/skills/define-ssot/resources/template.md`.
   - Invoke the `ssot-writer` subagent. Send it the raw knowledge you gathered and instruct it to synthesize a cohesive SSOT matching the exact format of the template.
3. **Save and Update Ledger**:
   - Parse the JSON metadata block provided at the end of the `ssot-writer` output.
   - Read `docs/ssot/INDEX.md` to find the highest existing `SSOT-NNNN` ID in the ledger table. Increment this number to determine the new ID. (If the table is empty, start with `SSOT-0001`).
   - Save the subagent's output to a new file: `docs/ssot/SSOT-NNNN.md`.
   - Append a new row to the table in `docs/ssot/INDEX.md` using the calculated ID and the JSON metadata.
4. **Present Findings**: Notify the user that the SSOT is defined and provide a link to the new file.

## Invariants

- **Context Preservation**: Pass all necessary raw data directly to the `ssot-writer` in your initial message, as it does not have tools to read files itself.
- **Zero LaTeX**: Ensure the subagent is explicitly instructed to adhere to the project's Zero-LaTeX ASCII math rules.

