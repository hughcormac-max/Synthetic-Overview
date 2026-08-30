# Master Domain Knowledge & Reference Index

> **Directory:** `docs/references/`
> **Architecture Principle:** Strict Downward Dependency — Reference files are pure, authoritative domain truths. Implementation code and plans cite references, but references remain decoupled from codebase file paths.

---

## 🏛️ Grounding Framework Overview

The **Domain Knowledge & Reference System** stores immutable, authoritative facts, formulas, logic loops, constants, and known hallucination traps.

```
+-------------------------------------------------------------------------+
| Tier 0: Pure Domain Truth (docs/references/REF-XXX.md)                  |
|         - Formulas, physical constants, logic loops, truth tables       |
|         - Anti-hallucination traps & golden test vectors                |
|         - ZERO upward knowledge of source code or file paths            |
+-------------------------------------------------------------------------+
                                     ^
                                     | (Downward Citation Dependency)
+-------------------------------------------------------------------------+
| Tier 1: Plans & Specifications (.antigravity/plans/active/PLAN-XXX.md)  |
|         - Grounded in Tier 0 references before writing code             |
+-------------------------------------------------------------------------+
                                     ^
                                     | (Downward Implementation Dependency)
+-------------------------------------------------------------------------+
| Tier 2: Source Code Implementation (src/core/...)                       |
|         - Code docstrings & constants cite Tier 0 references            |
|         - Unit tests assert against Tier 0 golden benchmark tables      |
+-------------------------------------------------------------------------+
```

---

## 🗂️ Master Reference Ledger

| Reference ID | Title & Domain Scope | Category | Key Invariants / Formulas | Status |
| :--- | :--- | :--- | :--- | :--- |
| *No references registered yet* | — | — | — | — |

---

## 📝 Reference Authoring Workflow

1. When introducing complex domain logic, mathematical formulas, physical constants, or intricate decision trees, create a new reference file:
   - Copy `docs/references/TEMPLATE.md` to `docs/references/REF-XXX-[slug].md`.
2. Populate the 6 core grounding sections (Statement of Truth, Formulas, Logic Loops, Figures/Constants, Hallucination Traps, and Golden Test Vectors).
3. Ensure **zero upward references**: Do NOT mention specific source code files or directory paths in the reference document.
4. Register the new reference in the table above.
