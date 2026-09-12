# AGENTS.md — Workspace Rules & Context for AI Agents

> **Project Template:** Agentic Development Framework
> **Master Agent Grounding Document** — Read on every session initialization.

This document establishes the repository guidelines, architectural boundaries, core tenets, context-bounding rules, and human-in-the-loop interaction protocols for all AI coding agents working in this workspace.

---

## 🏛️ 1. Core Tenets & Non-Negotiable Invariants

1. **Strict Type Safety:**
   - Write clean, modular, and strictly-typed code across all languages (e.g., TypeScript `strict: true` with no `any` types, Python type hints with `mypy`/`pyright`, Rust strong typing).
   - Public APIs, data models, and service boundaries must declare explicit type contracts.

2. **Plain-Text & ASCII Math Formatting (CRITICAL: ZERO LATEX):**
   - **STRICT PROHIBITION:** NEVER use single dollar signs (`$...$`) or double dollar signs (`$$...$$`) for inline or block math in chat responses, explanations, markdown files, code comments, or documentation.
   - NEVER use LaTeX macros, symbols, or escape sequences (e.g. do NOT use `\dot{}`, `\ddot{}`, `\cdot`, `\varpi`, `\Omega`, `\approx`, `\times`, `\frac{}{}`, `\text{}`, `\sum`, `\int`, `_0` subscript tags, etc.).
   - Always render formulas, variables, and rates using clean ASCII/Unicode plain text or inline code formatting:
     - Write `a = a0 + a_dot * T` (NEVER LaTeX)
     - Write `P = P_base * (1 + k * (D - S) / S)`
     - Write `eps = v^2 / 2 - mu / r`
     - Render units as clean text: `250 km`, `1.5e11 m`, `m/s`, `AU`, `deg/cy`.

3. **Strict Layer Separation & Pure Function Pipelines:**
   - Decouple Domain / Core Logic, Application / Services, Infrastructure / I/O, and Presentation / UI.
   - Core domain models and calculation pipelines must have **zero external I/O or UI dependencies**. State transformations and calculation pipelines must be pure, deterministic functions without side effects.
   - **Downward Reference Rule:** `docs/ssot/SSOT-NNNN.md` acts as Tier 0 immutable domain truth (formulas, constants, logic loops, anti-hallucination traps). Plans and source code cite downward to references; references must NEVER contain upward references to codebase file paths.

4. **Code Quality & Immutability Standards:**
   - **Immutability by Default:** Prefer immutable data structures (`readonly`, `const`). Transform data by creating new state instances rather than mutating in place.
   - **Explicit Error Handling:** Use explicit domain error or result types. Never write empty catch blocks or silently swallow errors. Fail fast with input validation at boundary layers.
   - **Focused Units:** Adhere to the Single Responsibility Principle. Keep individual functions small (aim for under 40 lines) and files under 250-300 lines.

5. **Anti-Hallucination & Bounded Context Protocol:**
   - **Contract-First:** Never write implementation code until public interfaces, data types, and function signatures are specified in an approved plan.
   - **Atomic Task Sizing:** Keep individual implementation steps small (under 150-200 lines of code changes per step).
   - **Step-by-Step Assertion:** Run and pass unit tests for Step N before proceeding to Step N+1.

6. **Dependency Discipline:**
   - Unauthorized third-party packages and external dependencies are strictly prohibited without prior architectural approval.

---

## 🤝 2. Human-in-the-Loop Safety Protocol

1. **Explain Reasoning & Architectural Diffs:** Before and during code modifications, clearly summarize the intent, file changes, and non-obvious design rationale.
2. **No Destructive Commands Without Confirmation:** Never execute destructive or irreversible commands (e.g. `rm -rf`, git hard resets, deleting production directories, dropping database schemas) without explicit user confirmation.
3. **API & Contract Integrity:** Do not alter established schemas or public interfaces without checking and updating all dependent invocation sites.
4. **Automated Verification:** Always run the project typecheck, lint, and test suite before declaring a task complete.

---

## 🔄 3. Plan-Driven Development Workflow

All significant features, refactors, and bug fixes follow the **Plan Specification Framework**:

1. **`/plan`**: Inspect dependencies, define technical contracts, and draft `PLAN-XXX.md`.
2. **Review Gate**: Developer audits and explicitly approves the plan.
3. **`/execute`**: Implement atomic steps sequentially with tests asserted at each step.
4. **`/archive-plan`**: Pass 7-point QA verification and archive to the permanent ledger.

Specific subagent delegations, workflows, and tools are defined within each individual skill under [`.agents/skills/`](.agents/skills/) and subagent specs under [`.agents/subagents/`](.agents/subagents/).

---

## 📁 4. Workspace Map & Navigation

| Path | Purpose | Key References |
| :--- | :--- | :--- |
| `AGENTS.md` | Master agent grounding & non-negotiable invariants | [AGENTS.md](AGENTS.md) |
| `.agents/plans/` | Plan specification framework | [active/](.agents/plans/active/), [archive/INDEX.md](.agents/plans/archive/INDEX.md) |
| `.agents/subagents/` | Custom subagent definitions | [planner.md](.agents/subagents/planner.md), [ssot-writer.md](.agents/subagents/ssot-writer.md), [implementer.md](.agents/subagents/implementer.md), [code-reviewer.md](.agents/subagents/code-reviewer.md), [web-researcher.md](.agents/subagents/web-researcher.md), [reporter.md](.agents/subagents/reporter.md), [doc-researcher.md](.agents/subagents/doc-researcher.md) |
| `.agents/skills/` | Actionable skills & slash commands | [plan](.agents/skills/plan/SKILL.md), [execute](.agents/skills/execute/SKILL.md), [archive-plan](.agents/skills/archive-plan/SKILL.md), [research](.agents/skills/research/SKILL.md), [define-ssot](.agents/skills/define-ssot/SKILL.md), [ask-docs](.agents/skills/ask-docs/SKILL.md) |
| `docs/ssot/` | Domain knowledge & truth vault | [INDEX.md](docs/ssot/INDEX.md) |
| `docs/research/` | Digested research reports vault | [INDEX.md](docs/research/INDEX.md) |
| `docs/` | Living system documentation | [ARCHITECTURE.md](docs/ARCHITECTURE.md), [ROADMAP.md](docs/ROADMAP.md), [TASKS.md](docs/TASKS.md) |
| `.vscode/` | Workspace editor & sandbox configuration | [settings.json](.vscode/settings.json), [extensions.json](.vscode/extensions.json) |

---

## ⚡ 5. Standard Verification Commands

*Configure the active commands below to match the initialized project stack:*

| Check | Command / Mechanism | Purpose |
| :--- | :--- | :--- |
| **Standards Audit** | Agentic Review (`code-reviewer` / native ripgrep) | Verify Zero-LaTeX & Downward Reference integrity |
| **Typecheck** | `npm run typecheck` / `mypy .` / `cargo check` | Verify strict type safety |
| **Lint** | `npm run lint` / `ruff check .` / `cargo clippy` | Verify code quality & style |
| **Unit Tests** | `npm test` / `pytest` / `cargo test` | Verify hermetic unit test pass rate |
| **Build** | `npm run build` / `cargo build` | Verify clean compilation & packaging |

---

## ✅ 6. Pre-Completion 7-Point QA Checklist

Prior to marking any task or active plan as complete:

- [ ] **1. Type Check:** Clean compilation with zero type errors.
- [ ] **2. Unit Tests:** 100% pass rate for touched and newly created modules.
- [ ] **3. Regression Tests:** All existing regression suites pass with zero regressions.
- [ ] **4. Contract Adherence:** Code strictly satisfies the technical contracts approved in `PLAN-XXX.md`.
- [ ] **5. Zero LaTeX Audit:** Zero `$...$` or `$$...$$` or LaTeX macros in docs, code comments, or chat output.
- [ ] **6. Plan Checklist Sync:** All implementation checkboxes in `PLAN-XXX.md` are marked complete.
- [ ] **7. Clean Git Status:** No lingering temporary files, debug statements, or unsanctioned modifications.
