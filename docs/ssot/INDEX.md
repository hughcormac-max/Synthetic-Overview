# Master Domain Knowledge & SSOT Index

> **Directory:** `docs/ssot/`
> **Architecture Principle:** Strict Downward Dependency — SSOT files are pure, authoritative domain truths. Implementation code and plans cite SSOTs, but SSOTs remain decoupled from codebase file paths.

---

## 🏛️ Grounding Framework Overview

The **Domain Knowledge & SSOT System** stores immutable, authoritative facts, formulas, logic loops, constants, and known hallucination traps.

- **Tier 0 (Domain Truth — `docs/ssot/SSOT-NNNN.md`):** Pure domain truth, formulas, constants, and golden test vectors. Zero upward knowledge of codebase files.
- **Tier 1 (Technical Plans — `.agents/plans/active/`):** Cites Tier 0 SSOT specifications before code is written.
- **Tier 2 (Source Implementation — `src/`):** Code and unit tests cite Tier 0 SSOT documents and assert against golden test vectors.

---

## 🗂️ Master SSOT Ledger

| SSOT ID | Title & Domain Scope | Category | Key Invariants / Formulas | Status |
| :--- | :--- | :--- | :--- | :--- |
| *No SSOTs registered yet* | — | — | — | — |

---

## 📝 SSOT Authoring Workflow

1. Utilize the `define-ssot` skill via the primary agent.
2. Provide the agent with raw domain logic, mathematical formulas, physical constants, or research.
3. The agent will orchestrate the `ssot-writer` subagent to generate a new `SSOT-NNNN.md` document, ensuring zero upward references.
4. The agent will automatically register the new SSOT in the table above.
