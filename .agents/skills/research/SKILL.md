---
name: research
description: Use this skill when the user asks to perform deep online research on a specific goal. It orchestrates a planner, web-researchers, and a reporter subagent to produce a digested report.
---

# Deep Research Workflow

This skill defines the multi-agent workflow for conducting deep online research and producing a synthesized report without polluting the main agent's context window.

## Workflow Steps

When the user requests research, execute the following steps sequentially:

1. **Clarify the Goal**: If the user's research request is vague, ask clarifying questions to establish the specific objective, target domain, and constraints.
2. **Invoke Planner (`planner`)**: Read `.agents/subagents/planner.md` if necessary, and invoke the planner subagent. Instruct the planner to break down the main research goal into 2-4 independent, non-overlapping search objectives.
3. **Dispatch Web Researchers (`web-researcher`)**:
   - Read `.agents/subagents/web-researcher.md` to understand the subagent's role.
   - For each search objective defined by the planner, concurrently invoke a `web-researcher` subagent using the `invoke_subagent` tool.
   - Wait for all web researchers to report back with their raw findings in your messaging inbox.
4. **Compile Raw Data**: Gather all the raw findings provided by the web researchers.
5. **Invoke Reporter (`reporter`)**:
   - Read `.agents/subagents/reporter.md` and the template at `.agents/skills/research/resources/template.md`.
   - Invoke the `reporter` subagent. Send it the compiled raw data and instruct it to synthesize a cohesive report matching the exact format of the template.
6. **Save and Update Ledger**:
   - Parse the JSON metadata block provided at the end of the reporter's output.
   - Read `docs/research/INDEX.md` to find the highest existing `RESEARCH-NNNN` ID in the ledger table. Increment this number to determine the new ID. (If the table is empty, start with `RESEARCH-0001`).
   - Save the reporter's output to a new file: `docs/research/RESEARCH-NNNN.md`.
   - Append a new row to the table in `docs/research/INDEX.md` using the calculated ID and the JSON metadata provided by the reporter.
7. **Present Findings**: Notify the user that the research is complete and provide a link to the new `RESEARCH-NNNN.md` file.

## Invariants

- **Context Isolation**: Do not perform the `search_web` operations yourself. Always delegate them to `web-researcher` subagents so the massive HTML/Markdown output does not pollute your context.
- **Concurrent Execution**: Dispatch the web researchers at the same time to save real-world time.
- **Zero LaTeX**: Ensure the reporter subagent is explicitly instructed to adhere to the project's Zero-LaTeX ASCII math rules if the research involves formulas or equations.

