---
name: define-ssot
description: Establish a new Single Source of Truth (SSOT-NNNN.md) in .agents/docs/ssot/ as either a flat reference context document or a formal algorithmic specification.
---

# `/define-ssot` Workflow Trajectory

> **Workflow Command:** `/define-ssot`
> **Purpose:** Distill domain knowledge, conventions, reference context, or algorithmic rules into an immutable Tier 0 Single Source of Truth document (`SSOT-NNNN.md`) in `.agents/docs/ssot/`.

---

## 🎯 Workflow Execution Steps

1. **Gather Domain Knowledge & Select SSOT Archetype:**
   - Ingest source materials (e.g., from `.agents/docs/research/RESEARCH-NNNN.md`, official specifications, or user directives).
   - Choose the appropriate SSOT archetype based on the content:
     - **Archetype A — Flat Reference Context (`type: context`):** Use [.agents/skills/define-ssot/assets/TEMPLATE-CONTEXT.md](assets/TEMPLATE-CONTEXT.md) when capturing domain glossaries, architectural boundaries, product rules, schemas, conventions, or flat lookup tables. Keep headings flat (`##` only) with zero artificial formulas or test vectors.
     - **Archetype B — Formal Specification (`type: specification`):** Use [.agents/skills/define-ssot/assets/TEMPLATE-SPEC.md](assets/TEMPLATE-SPEC.md) (or the optional modules in [.agents/skills/define-ssot/assets/TEMPLATE.md](assets/TEMPLATE.md)) when capturing mathematical formulas, state machines, decision truth tables, physical/financial constants, and deterministic golden test vectors.
2. **Determine Sequential SSOT ID:**
   - Read `.agents/docs/ssot/INDEX.md` to identify the highest existing `SSOT-NNNN` (or domain-prefixed `SSOT-[DOM]-NNN`) ID.
   - Increment the number to determine the next sequential ID.
3. **Format Specification Cleanly:**
   - Populate the selected template and **omit any optional sections that do not apply** (never leave empty placeholder sections or `N/A` tables).
   - **Enforce Invariant 1 (Strict Downward Reference):** The document must remain 100% self-contained domain truth. NEVER reference specific codebase files (`src/...`, `crates/...`) in an SSOT.
   - **Enforce Invariant 2 (Zero-LaTeX):** All formulas, variables, and units must be formatted in clean ASCII / Unicode plain text (e.g., `a = a0 + a_dot * T`).
4. **Save Document & Update Ledger:**
   - Save the document to `.agents/docs/ssot/`.
   - Append a new row to `.agents/docs/ssot/INDEX.md` with ID, title & scope, type (`context` or `specification`), core truths summary, status, and document link.
5. **Notify User:**
   - Present a concise domain summary in chat with a clickable link to the new SSOT document.
