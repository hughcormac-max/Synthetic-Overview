---
name: define-ssot
description: Establish a new Single Source of Truth (SSOT-0001.md) in .agents/docs/ssot/ capturing project-specific invariable logic, formulas, decision rules, and constants.
---

# `/define-ssot` Workflow Trajectory

> **Workflow Command:** `/define-ssot`
> **Purpose:** Distill raw research, algorithm specs, or domain requirements into an immutable Tier 0 Single Source of Truth document (`SSOT-NNNN.md`) in `.agents/docs/ssot/`.

---

## 🎯 Workflow Execution Steps

1. **Gather Invariable Domain Knowledge:**
   - Ingest raw source materials (e.g. from `.agents/docs/research/RESEARCH-NNNN.md`, official specifications, or domain directives).
   - Identify core invariable logic, mathematical equations, state transitions, domain invariants, and baseline constants.
2. **Determine Sequential SSOT ID:**
   - Read `.agents/docs/ssot/INDEX.md` to identify the highest existing `SSOT-NNNN` ID.
   - Increment the number to determine the new 4-digit zero-padded sequential ID (e.g. `SSOT-0001`, `SSOT-0002`).
3. **Format Specification using Template:**
   - Format the domain knowledge using the template at `.agents/skills/define-ssot/resources/template.md`.
   - **Enforce Invariant 1 (Strict Downward Reference):** The document must remain 100% self-contained domain truth. NEVER reference specific codebase files (`src/...`) in an SSOT.
   - **Enforce Invariant 2 (Zero-LaTeX):** All formulas must be formatted in clean ASCII / Unicode plain text (e.g. `a = a0 + a_dot * T`).
   - Formulate explicit golden test vectors with deterministic inputs and exact outputs.
4. **Save Specification & Update Ledger:**
   - Save the document to `.agents/docs/ssot/SSOT-NNNN.md`.
   - Append a new row to `.agents/docs/ssot/INDEX.md` with ID, title, category, key invariants, status, and link.
5. **Notify User:**
   - Present a concise domain summary in chat with a clickable link to the new SSOT specification.
