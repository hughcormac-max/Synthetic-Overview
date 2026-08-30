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

3. **Strict Layer Separation & Downward Knowledge Grounding:**
   - Decouple Domain / Core Logic, Application / Services, Infrastructure / I/O, and Presentation / UI.
   - Core domain models and computational pipelines must have **zero external I/O or UI dependencies**. See [.antigravity/rules/00-core-architecture.md](file:///.antigravity/rules/00-core-architecture.md).
   - **Downward Reference Rule:** `docs/references/REF-XXX.md` acts as Tier 0 immutable domain truth (formulas, constants, logic loops, anti-hallucination traps). Plans and source code cite downward to references; references must NEVER contain upward references to codebase file paths.

4. **Anti-Hallucination & Bounded Context Protocol:**
   - **Contract-First:** Never write implementation code until public interfaces, data types, and function signatures are specified in an approved plan.
   - **Atomic Task Sizing:** Keep individual implementation steps small (under 150-200 lines of code changes per step).
   - **Step-by-Step Assertion:** Run and pass unit tests for Step N before proceeding to Step N+1.

5. **Dependency Discipline:**
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

```
+------------------+     +-------------------+     +--------------------+
|  1. /plan        | --> |  2. Review Gate   | --> |  3. /execute       |
|  Draft PLAN-XXX  |     |  Human Approval   |     |  Sequential Steps  |
+------------------+     +-------------------+     +--------------------+
                                                             |
                                                             v
                                                   +--------------------+
                                                   |  4. /archive-plan  |
                                                   |  QA Gate & Ledger  |
                                                   +--------------------+
```

### Specialized Subagent Delegation

- **Planner Subagent ([.antigravity/agents/planner.md](file:///.antigravity/agents/planner.md)):** Performs read-only research, dependency analysis, and drafts `PLAN-XXX.md`.
- **Knowledge Grounder Subagent ([.antigravity/agents/knowledge-grounder.md](file:///.antigravity/agents/knowledge-grounder.md)):** Distills literature and domain truth into pure Tier 0 `REF-XXX` specifications.
- **Implementer Subagent ([.antigravity/agents/implementer.md](file:///.antigravity/agents/implementer.md)):** Executes atomic checklist items sequentially in an isolated workspace branch, testing at each step.
- **Code Reviewer Subagent ([.antigravity/agents/code-reviewer.md](file:///.antigravity/agents/code-reviewer.md)):** Audits diffs for rule adherence, static analysis, edge-case coverage, and executes the 7-point QA verification gate.

---

## 📁 4. Workspace Map & Navigation

| Path | Purpose | Key References |
| :--- | :--- | :--- |
| `.antigravity/rules/` | Persistent workspace constraints | [00-core-architecture.md](file:///.antigravity/rules/00-core-architecture.md), [01-coding-standards.md](file:///.antigravity/rules/01-coding-standards.md), [02-verification.md](file:///.antigravity/rules/02-verification.md) |
| `.antigravity/plans/` | Plan specification framework | [TEMPLATE.md](file:///.antigravity/plans/TEMPLATE.md), [active/](file:///.antigravity/plans/active/), [archive/index.md](file:///.antigravity/plans/archive/index.md) |
| `.antigravity/agents/` | Custom subagent definitions | [planner.md](file:///.antigravity/agents/planner.md), [knowledge-grounder.md](file:///.antigravity/agents/knowledge-grounder.md), [implementer.md](file:///.antigravity/agents/implementer.md), [code-reviewer.md](file:///.antigravity/agents/code-reviewer.md) |
| `.antigravity/workflows/` | Slash-command playbooks | [plan.md](file:///.antigravity/workflows/plan.md), [execute.md](file:///.antigravity/workflows/execute.md), [archive-plan.md](file:///.antigravity/workflows/archive-plan.md) |
| `docs/references/` | Domain knowledge & truth vault | [INDEX.md](file:///docs/references/INDEX.md), [TEMPLATE.md](file:///docs/references/TEMPLATE.md) |
| `docs/` | Living system documentation | [ARCHITECTURE.md](file:///docs/ARCHITECTURE.md), [ROADMAP.md](file:///docs/ROADMAP.md), [TASKS.md](file:///docs/TASKS.md) |
| `scripts/` | Standards verification scripts | [audit-standards.ps1](file:///scripts/audit-standards.ps1) |
| `.vscode/` | Workspace editor & sandbox configuration | [settings.json](file:///.vscode/settings.json), [extensions.json](file:///.vscode/extensions.json) |

---

## ⚡ 5. Standard Verification Commands

*Configure the active commands below to match the initialized project stack:*

| Check | Command | Purpose |
| :--- | :--- | :--- |
| **Standards Audit** | `powershell -ExecutionPolicy Bypass -File ./scripts/audit-standards.ps1` | Verify Zero-LaTeX & Downward Reference integrity |
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
