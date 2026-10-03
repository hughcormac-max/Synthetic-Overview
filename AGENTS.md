# AGENTS.md — Workspace Rules & Context for AI Agents

> **Project Template:** Agentic Development Framework
> **Master Agent Grounding Document** — Read on every session initialization.

This document establishes the repository guidelines, architectural boundaries, core tenets, context-bounding rules, and human-in-the-loop interaction protocols for all AI coding agents working in this workspace.

---

## 🏛️ 1. Core Tenets & Non-Negotiable Invariants

1. **Strict Type Safety:**
   - Write clean, modular, and strictly-typed code across all languages (e.g., Rust strong typing with zero unvetted `unsafe`, TypeScript `strict: true` with no `any` types, Python type hints with `mypy`/`pyright`).
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
   - **Downward Reference Rule:** `.agents/docs/ssot/SSOT-NNNN.md` acts as Tier 0 immutable domain truth (invariable logic, decisions, formulas, constants, decision trees, golden vectors). Plans and source code cite downward to SSOT references; SSOT references must NEVER contain upward references to codebase file paths.

4. **Code Quality & Immutability Standards:**
   - **Immutability by Default:** Prefer immutable data structures (`readonly`, `const`, Rust immutable bindings). Transform data by creating new state instances rather than mutating in place.
   - **Explicit Error Handling:** Use explicit domain error or result types (`Result<T, E>`). Never write empty catch blocks or silently swallow errors. Fail fast with input validation at boundary layers.
   - **Focused Units:** Adhere to the Single Responsibility Principle. Keep individual functions small (aim for under 40 lines) and files under 250-300 lines.

5. **Anti-Hallucination, Bounded Context & Subagent Isolation:**
   - **Contract-First:** Never write implementation code until public interfaces, data types, and function signatures are specified in an approved plan.
   - **Pseudocode-Driven Tasks:** The strict <= 50-line rule has been replaced. Instead, work must be broken down into atomic logical tasks (e.g., specific functions, components, or files), and explicit **pseudocode** must be drafted for each step.
   - **Subagent Task Isolation:** Implementation tasks targeting isolated files are executed by focused subagent contexts, preventing cross-file pollution and bounding blast radius. The `implementer` strictly translates pseudocode to code.
   - **Boomerang Summary & Verification:** After implementation, the orchestrator awaits a Boomerang Summary and dispatches a `code-reviewer` to verify pseudocode adherence and test passes.

6. **Static Plan Storage (No Active/Archive File Movement):**
   - All plans reside permanently in `.agents/docs/plans/` (e.g. `PLAN-001.md`).
   - Plan lifecycle status (`draft`, `approved`, `in-progress`, `completed`, `superseded`, `abandoned`) is tracked exclusively in the plan's YAML frontmatter and synchronized in `.agents/docs/plans/INDEX.md`. Files never move between folders upon completion.

7. **Dependency Discipline:**
   - Unauthorized third-party packages and external dependencies are strictly prohibited without prior architectural approval.

---

## 🤝 2. Human-in-the-Loop Safety Protocol

1. **Explain Reasoning & Architectural Diffs:** Before and during code modifications, clearly summarize the intent, file changes, and non-obvious design rationale.
2. **No Destructive Commands Without Confirmation:** Never execute destructive or irreversible commands (e.g. `rm -rf`, git hard resets, deleting production directories, dropping database schemas) without explicit user confirmation.
3. **API & Contract Integrity:** Do not alter established schemas or public interfaces without checking and updating all dependent invocation sites.
4. **Automated Verification:** Always run the project typecheck, lint, and test suite before declaring a task complete.

---

## 🔄 3. Agentic Workflow Trajectory

All domain research, specifications, planning, and implementations follow this trajectory:

1. **`/research` (Grounded Reality Research):**
   - Conducts deep web research to gather non-hallucinated facts, documentation, and external standards.
   - Saves findings to `.agents/docs/research/RESEARCH-NNNN.md` (sequential 4-digit ID) and updates the ledger in `.agents/docs/research/INDEX.md`.

2. **`/define-ssot` (Invariable Domain Truth):**
   - Formulates project-specific invariable logic, formulas, decision rules, constants, and golden vectors.
   - Saves to `.agents/docs/ssot/SSOT-NNNN.md` (sequential 4-digit ID) and updates the ledger in `.agents/docs/ssot/INDEX.md`.

3. **Deliberate Two-Phase Planning Workflow (Human-in-the-Loop):**
   - **`/plan-human` (Problem Alignment):**
     - Aligns the human-facing problem: problem statement, background, core objectives, non-goals, user stories, acceptance criteria, and edge cases.
     - Scaffolds Part 1 of `.agents/docs/plans/PLAN-NNNN.md`.
   - **Review Gate (Mandatory Human Review):** Developer audits, refines, and explicitly approves the human problem alignment before proceeding.
   - **`/plan-robot` (Technical Specification):**
     - Completes technical contracts: SSOT citations, types, schemas, interfaces, module mapping, unit test targets, and **Pseudocode-Driven Atomic Tasks**.
     - Fills Part 2 of `.agents/docs/plans/PLAN-NNNN.md`.
   - **Review Gate (Mandatory Human Sign-off):** Developer audits and signs off on the technical specification before execution begins.

4. **`/execute` (Orchestrator-Worker Implementation):**
   - Implements the approved plan using the Boomerang Pattern.
   - Spawns the `implementer` subagent to translate pseudocode into code, and waits for a Boomerang Summary.
   - Dispatches the `code-reviewer` to run hermetic tests, verify pseudocode adherence, and update checklist progress.

5. **`/archive-plan` (QA Verification & Lifecycle Closure):**
   - Validates the 7-Point QA checklist.
   - Logs retrospective notes and runtime architectural deviations.
   - Updates plan frontmatter (`status: completed`, `completed_at: YYYY-MM-DD`) and ledger status in `.agents/docs/plans/INDEX.md`.

Specific skills and subagent specs are located under [`.agents/skills/`](.agents/skills/) and [`.agents/subagents/`](.agents/subagents/).

---

## 📁 4. Workspace Map & Navigation

| Path | Purpose | Key References |
| :--- | :--- | :--- |
| `AGENTS.md` | Master agent grounding & non-negotiable invariants | [AGENTS.md](AGENTS.md) |
| `.agents/docs/plans/` | Consolidated plan specifications & ledger | [INDEX.md](.agents/docs/plans/INDEX.md) |
| `.agents/docs/research/` | Grounded reality research reports | [INDEX.md](.agents/docs/research/INDEX.md) |
| `.agents/docs/ssot/` | Single Source of Truth vault (Tier 0 invariable domain truth) | [INDEX.md](.agents/docs/ssot/INDEX.md) |
| `.agents/skills/` | Actionable skills & slash commands | [plan-human](.agents/skills/plan-human/SKILL.md), [plan-robot](.agents/skills/plan-robot/SKILL.md), [execute](.agents/skills/execute/SKILL.md), [archive-plan](.agents/skills/archive-plan/SKILL.md), [research](.agents/skills/research/SKILL.md), [define-ssot](.agents/skills/define-ssot/SKILL.md), [ask-docs](.agents/skills/ask-docs/SKILL.md) |
| `.agents/subagents/` | Subagent role system prompts | [implementer.md](.agents/subagents/implementer.md), [code-reviewer.md](.agents/subagents/code-reviewer.md), [researcher.md](.agents/subagents/researcher.md) |
| `docs/` | Human-facing living system documentation | [ARCHITECTURE.md](docs/ARCHITECTURE.md), [ROADMAP.md](docs/ROADMAP.md), [TASKS.md](docs/TASKS.md) |
| `.vscode/` | Workspace editor & sandbox configuration | [settings.json](.vscode/settings.json), [extensions.json](.vscode/extensions.json) |

---

## ⚡ 5. Standard Verification Commands

*Active verification commands for this Rust workspace:*

| Check | Command / Mechanism | Purpose |
| :--- | :--- | :--- |
| **Standards Audit** | Agentic Review (`code-reviewer` / native ripgrep) | Verify Zero-LaTeX & Downward Reference integrity |
| **Typecheck / Check** | `cargo check --workspace --all-targets` | Verify strict type safety & compilation |
| **Lint** | `cargo clippy --workspace --all-targets -- -D warnings` | Verify code cleanliness, performance & style |
| **Unit & Invariant Tests** | `cargo test --workspace` | Verify hermetic unit & property-based invariant pass rate |
| **Build** | `cargo build --workspace` | Verify clean compilation & packaging |

---

## ✅ 6. Pre-Completion 7-Point QA Checklist

Prior to marking any task or plan as complete (`/archive-plan`):

- [ ] **1. Type Check:** Clean compilation with zero type errors (`cargo check --workspace --all-targets`).
- [ ] **2. Unit Tests:** 100% pass rate for touched and newly created modules (`cargo test --workspace`).
- [ ] **3. Regression Tests:** All existing regression suites pass with zero regressions.
- [ ] **4. Contract Adherence:** Code strictly satisfies technical contracts in `PLAN-NNNN.md`.
- [ ] **5. Zero LaTeX Audit:** Zero unescaped dollar signs (`$...$`, `$$...$$`) or LaTeX macros in docs, code comments, or chat output.
- [ ] **6. Plan Checklist Sync:** All implementation checkboxes in `PLAN-NNNN.md` are marked complete.
- [ ] **7. Clean Git Status:** No lingering temporary files, debug statements, or unsanctioned modifications.
