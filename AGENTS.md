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

## 🔄 3. 6-Step Native Spec-Driven Development (SDD) Workflow

All significant features, algorithmic implementations, and architectural refactors follow the strict 6-step native Spec-Driven Development pipeline:

1. **Step 1: `/research` (Multi-Agent Knowledge Gathering)**
   - Orchestrates `planner`, `web-researcher`, and `reporter` subagents to perform deep online research and compile findings into `docs/research/RESEARCH-NNNN.md`.
2. **Step 2: `/define-ssot` (Immutable Truth Grounding)**
   - Distills raw findings and domain specifications via `ssot-writer` into an immutable Tier 0 domain specification in `docs/ssot/SSOT-NNNN.md` (defining mathematical models, constants, decision trees, and golden test vectors).
3. **Step 3: `/plan-human` (Functional Specification & Acceptance Criteria)**
   - Scaffolds Phase 1 of `PLAN-XXX.md` in `.agents/plans/active/`, defining user problem statements, scope boundaries, non-goals, user scenarios, and observable acceptance criteria.
4. **Step 4: `/plan-robot` (Technical Design, Invariants & Adversarial Review)**
   - Architects Phase 2 of `PLAN-XXX.md`, defining Rust ECS data structures, system signatures, precision boundaries, and Zero-LaTeX invariants.
   - Dispatches the `interrogator` subagent to conduct an adversarial technical review (probing ECS cache misses, float non-determinism, and logic gaps).
   - Refines the technical plan to resolve critical findings and halts at the **Human Review Gate** for developer approval.
5. **Step 5: `/execute` (Sequential Implementation & Invariant Testing)**
   - Dispatches `implementer` to execute implementation tasks sequentially, enforcing step-by-step invariant verification and property-based testing using native `cargo test`.
6. **Step 6: `/archive-plan` (QA Verification, Safety Audits & Ledger Archival)**
   - Dispatches `code-reviewer` to audit dead-code removal, code cleanliness, and blast-radius safety, and verify the Pre-Completion QA Checklist.
   - Updates plan frontmatter, logs architectural retrospectives, and archives the plan into `.agents/plans/archive/YYYY/PLAN-XXX.md` and `INDEX.md`.

Specific subagent delegations, workflows, and tools are defined within each individual skill under [`.agents/skills/`](.agents/skills/) and subagent specs under [`.agents/subagents/`](.agents/subagents/).

---

## 📁 4. Workspace Map & Navigation

| Path | Purpose | Key References |
| :--- | :--- | :--- |
| `AGENTS.md` | Master agent grounding & non-negotiable invariants | [AGENTS.md](AGENTS.md) |
| `.agents/plans/` | Plan specification framework | [active/](.agents/plans/active/), [archive/INDEX.md](.agents/plans/archive/INDEX.md) |
| `.agents/subagents/` | Custom subagent definitions | [planner.md](.agents/subagents/planner.md), [interrogator.md](.agents/subagents/interrogator.md), [ssot-writer.md](.agents/subagents/ssot-writer.md), [implementer.md](.agents/subagents/implementer.md), [code-reviewer.md](.agents/subagents/code-reviewer.md), [web-researcher.md](.agents/subagents/web-researcher.md), [reporter.md](.agents/subagents/reporter.md), [doc-researcher.md](.agents/subagents/doc-researcher.md) |
| `.agents/skills/` | Actionable skills & slash commands | [plan-human](.agents/skills/plan-human/SKILL.md), [plan-robot](.agents/skills/plan-robot/SKILL.md), [execute](.agents/skills/execute/SKILL.md), [archive-plan](.agents/skills/archive-plan/SKILL.md), [research](.agents/skills/research/SKILL.md), [define-ssot](.agents/skills/define-ssot/SKILL.md), [ask-docs](.agents/skills/ask-docs/SKILL.md) |
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
| **Typecheck / Check** | `cargo check --workspace --all-targets` | Verify strict type safety & compilation |
| **Lint** | `cargo clippy --workspace --all-targets -- -D warnings` | Verify code cleanliness, performance & style |
| **Unit & Invariant Tests** | `cargo test --workspace` | Verify hermetic unit & property-based invariant pass rate |
| **Build** | `cargo build --workspace` | Verify clean compilation & packaging |

---

## ✅ 6. Pre-Completion QA Checklist

Prior to marking any task or active plan as complete:

- [ ] **1. Compilation & Typecheck:** Clean compilation with zero errors (`cargo check --workspace --all-targets`).
- [ ] **2. Code Cleanliness & Dead-Code Removal:** Unused functions, dead code, orphan imports, and debug statements are purged.
- [ ] **3. Blast-Radius Safety Check:** Unintended side effects on neighboring systems and unapproved contract changes are verified absent.
- [ ] **4. Unit & Invariant Tests:** 100% pass rate for touched modules and invariant property tests (`cargo test --workspace`).
- [ ] **5. Regression Tests:** All existing regression suites pass with zero regressions.
- [ ] **6. Contract Adherence:** Code strictly satisfies technical contracts and invariants approved in `PLAN-XXX.md`.
- [ ] **7. Zero LaTeX Audit:** Zero unescaped dollar math delimiters or LaTeX macros in docs, code comments, or chat output.
- [ ] **8. Downward Reference Audit:** Zero upward references in `docs/ssot/` to source or build files.
- [ ] **9. Plan Checklist Sync:** All implementation checkboxes in `PLAN-XXX.md` are marked complete.
- [ ] **10. Clean Git Status:** No lingering temporary files, debug statements, or unsanctioned modifications.
