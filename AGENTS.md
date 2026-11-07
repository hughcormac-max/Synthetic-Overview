# AGENTS.md — Workspace Rules & Context for AI Agents

> **Project:** Synthetic Overview (`[[260830_HOC_Synthetic_Overview]]`)
> **Master Agent Grounding Document** — Read on every session initialization.
> **Memory Palace Vault:** `G:\My Drive\06_Memory Palace`
> **Project Entity Node:** `[[260830_HOC_Synthetic_Overview]]` (`G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\260830_HOC_Synthetic_Overview.md`)

This document establishes the repository guidelines, architectural boundaries, core tenets, Memory Palace integration rules, and human-in-the-loop interaction protocols for all AI coding agents working in this workspace.

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

3. **Strict Layer Separation & Memory Palace Grounding:**
   - Decouple Domain / Core Simulation Engine, Application / Orchestration, Infrastructure / I/O, and Presentation / 3D WebGPU Client.
   - Core simulation models and calculation pipelines must have **zero external I/O or UI/rendering dependencies**. State transformations and calculation pipelines must be pure, deterministic functions without side effects.
   - **Downward Reference Rule (Memory Palace Truth):** Standing project identity (`Knowledge/Entities/Projects/260830_HOC_Synthetic_Overview.md`), domain research (`Knowledge/Distilled/Research_And_Review/`), and committed system specifications (`Knowledge/Distilled/Design_And_Development/`) reside in `G:\My Drive\06_Memory Palace`. Active plans and source code cite downward to canonical Memory Palace node names (e.g., `[[Keplerian_Astrodynamics_Fibonacci_Globes_And_Light_Latency]]`).

4. **Code Quality & Immutability Standards:**
   - **Immutability by Default:** Prefer immutable data structures (`readonly`, `const`, Rust immutable bindings). Transform data by creating new state instances rather than mutating in place where appropriate.
   - **Explicit Error Handling:** Use explicit domain error or result types (`Result<T, E>`). Never write empty catch blocks or silently swallow errors. Fail fast with input validation at boundary layers.
   - **Focused Units:** Adhere to the Single Responsibility Principle. Keep individual functions small (aim for under 40 lines) and files under 250-300 lines.

5. **Anti-Hallucination, Bounded Context & Subagent Isolation:**
   - **Contract-First:** Never write implementation code until public interfaces, data types, and function signatures are specified in an approved plan in `.agents/work/`.
   - **Pseudocode-Driven Tasks:** Work must be broken down into atomic logical tasks (e.g., specific functions, components, or files), and explicit **pseudocode** must be drafted for each step.
   - **Subagent Task Isolation:** Implementation tasks targeting isolated files are executed by focused subagent contexts, preventing cross-file pollution and bounding blast radius. The `implementer` strictly translates pseudocode to code.
   - **Boomerang Summary & Verification:** After implementation, the orchestrator awaits a Boomerang Summary and dispatches a `code-reviewer` to verify pseudocode adherence and test passes.

6. **Lean Code Repository & Ephemeral Workbench (`.agents/work/`):**
   - The Git repository stores executable source code, tests, build configs, and agent workflow definitions—never permanent markdown archives of research or completed plans.
   - Active plans (`PLAN-<slug>.md`) and session research spikes (`RESEARCH-<slug>.md`) are staged ephemerally in `.agents/work/`.
   - At session closeout (`/close-to-palace`), session milestones, research syntheses, and committed design specs are exported into `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`, and transient `.agents/work/*.md` files are purged from the repository.
   - **Inbox Dropbox Boundary:** When operating from a code workspace, read freely from `G:\My Drive\06_Memory Palace\Knowledge\`, but only write to `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`. Never directly mutate `Episodic/` or `Knowledge/` from a code workspace (graph ingestion is handled by `/ingest-inbox` inside the Memory Palace).

7. **Dependency Discipline:**
   - Unauthorized third-party packages and external dependencies are strictly prohibited without prior architectural approval.

---

## 🤝 2. Human-in-the-Loop Safety Protocol

1. **Explain Reasoning & Architectural Diffs:** Before and during code modifications, clearly summarize the intent, file changes, and non-obvious design rationale.
2. **No Destructive Commands Without Confirmation:** Never execute destructive or irreversible commands (e.g. `rm -rf`, git hard resets, deleting production directories, dropping database schemas) without explicit user confirmation.
3. **API & Contract Integrity:** Do not alter established schemas or public interfaces without checking and updating all dependent invocation sites.
4. **Automated Verification:** Always run the project typecheck, lint, and test suite before declaring a task complete.
5. **Deliberate Pacing & Turn-Ending Next Steps:**
   - **No Rushing or Presumptive Urgency:** Never declare that "we are now ready to X" or assume the next step is mandatory. Maintain a calm, deliberate pace.
   - **Options-Oriented Next Steps:** At the conclusion of any task or research run, offer 2–3 grounded paths forward based on `[[260830_HOC_Synthetic_Overview]]` (`G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\260830_HOC_Synthetic_Overview.md`), recent episodic project logs (`Episodic/Projects/`), or active `.agents/work/` items.
   - **Explore Alternatives & Pivots:** Always explicitly raise the option to pause, reprioritize, pivot to a different track, or pursue tangential exploration before moving forward.

---

## 🔄 3. Agentic Workflow Trajectory

All domain lookups, research spikes, planning, execution, and session closeouts follow this 6-skill trajectory:

1. **`/ask-docs` (Memory Palace & Workspace Retrieval):**
   - Queries standing domain truth in `G:\My Drive\06_Memory Palace\Knowledge\` (`Entities/Projects/260830_HOC_Synthetic_Overview.md`, `Distilled/Design_And_Development/`, `Distilled/Research_And_Review/`, `Distilled/Standards_And_Guidance/`) alongside any active staging files in `.agents/work/`, refusing to hallucinate from pre-trained knowledge if information is absent.

2. **`/research` (Grounded Reality Research Spike):**
   - Conducts deep web and workspace research to gather non-hallucinated facts, plain-text formulas, and multi-option trade-off analyses.
   - Saves a staged report to `.agents/work/RESEARCH-<slug>.md` ready to inform active plans and export to `Knowledge/Distilled/Research_And_Review/` during `/close-to-palace`.

3. **Deliberate Two-Phase Planning Workflow (Human-in-the-Loop):**
   - **`/plan-human` (Problem Alignment):**
     - Aligns the human-facing problem: problem statement, core objectives, non-goals, user stories, acceptance criteria, and edge cases.
     - Scaffolds Part 1 of `.agents/work/PLAN-<slug>.md`.
     - **Mandatory Review Gate (HALT):** The agent halts execution. The human developer audits, refines, and explicitly approves the problem alignment before any technical design begins.
   - **`/plan-robot` (Technical Specification):**
     - Invoked deliberately only after human approval of Part 1.
     - Queries Memory Palace `Design_And_Development/` and `Research_And_Review/` nodes, and completes Part 2 of `.agents/work/PLAN-<slug>.md`: Memory Palace node citations, data types, interfaces, module mapping, unit test targets, and **Pseudocode-Driven Atomic Tasks**.
     - **Approval Gate (HALT):** Halts for developer sign-off on the technical blueprint before `/execute`.

4. **`/execute` (Orchestrator-Worker Implementation):**
   - Implements the approved `.agents/work/PLAN-<slug>.md` using the Boomerang Pattern.
   - Spawns the `implementer` subagent to translate pseudocode into code, and waits for a Boomerang Summary.
   - Dispatches the `code-reviewer` to run hermetic tests, verify pseudocode adherence, and update checklist progress.

5. **`/close-to-palace` (7-Point QA Gate, Memory Palace Export & Repo Hygiene):**
   - Runs the Pre-Closeout 7-Point QA Checklist (via `code-reviewer` if code was modified).
   - Packages session execution milestones and plan retrospectives (`-> Episodic/Projects/`), staged research spikes (`-> Knowledge/Distilled/Research_And_Review/`), and committed architectural specifications (`-> Knowledge/Distilled/Design_And_Development/`) into a structured Context Dump in `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`.
   - Purges transient `.agents/work/*.md` files from the code repository so the Git workspace stays lightweight and focused on executable code.

Specific skills and subagent specs are located under [`.agents/skills/`](.agents/skills/) and [`.agents/subagents/`](.agents/subagents/).

---

## 📁 4. Workspace Map & Navigation

| Path | Purpose | Key References |
| :--- | :--- | :--- |
| `AGENTS.md` | Master agent grounding, Memory Palace binding & invariants | [AGENTS.md](AGENTS.md) |
| `README.md` | Human-facing repository overview | [README.md](README.md) |
| `G:\My Drive\06_Memory Palace` | Permanent CoALA Memory Palace knowledge graph & episodic logs | `Knowledge/Entities/Projects/260830_HOC_Synthetic_Overview.md`, `Knowledge/Distilled/`, `Episodic/Projects/` |
| `.agents/work/` | Ephemeral in-repo staging workbench (git-ignored except `.gitkeep`) | `PLAN-<slug>.md`, `RESEARCH-<slug>.md` (purged on `/close-to-palace`) |
| `.agents/skills/` | 6 actionable skills & slash commands | [ask-docs](.agents/skills/ask-docs/SKILL.md), [research](.agents/skills/research/SKILL.md), [plan-human](.agents/skills/plan-human/SKILL.md), [plan-robot](.agents/skills/plan-robot/SKILL.md), [execute](.agents/skills/execute/SKILL.md), [close-to-palace](.agents/skills/close-to-palace/SKILL.md) |
| `.agents/subagents/` | Subagent role system prompts | [implementer.md](.agents/subagents/implementer.md), [code-reviewer.md](.agents/subagents/code-reviewer.md), [researcher.md](.agents/subagents/researcher.md) |
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

## ✅ 6. Pre-Closeout 7-Point QA Checklist

Prior to closing out a coding session or completed plan (`/close-to-palace`):

- [ ] **1. Type Check:** Clean compilation with zero type errors (`cargo check --workspace --all-targets`).
- [ ] **2. Unit Tests:** 100% pass rate for touched and newly created modules (`cargo test --workspace`).
- [ ] **3. Regression Tests:** All existing regression suites pass with zero regressions.
- [ ] **4. Contract Adherence:** Code strictly satisfies technical contracts in `.agents/work/PLAN-<slug>.md` and cited Memory Palace `Design_And_Development/` specs.
- [ ] **5. Zero LaTeX Audit:** Zero unescaped dollar signs (`$...$`, `$$...$$`) or LaTeX macros in docs, Context Dumps, code comments, or chat output.
- [ ] **6. Plan Checklist Sync:** All implementation checkboxes in the active plan are marked complete (`- [x]`).
- [ ] **7. Clean Git Status:** Transient `.agents/work/*.md` files are exported to `06_Memory Palace\Inbox\Context_Dumps\` and purged from the repo; no lingering debug artifacts remain.
