# AGENTS.md — Workspace Rules & Context for AI Agents

> **Project:** Synthetic Overview (`[[260830_HOC_Synthetic_Overview]]`)
> **Master Agent Grounding Document** — Read on every session initialization.
> **Memory Palace Vault:** `G:\My Drive\06_Memory Palace`
> **Project Entity Node:** `[[260830_HOC_Synthetic_Overview]]` (`G:\My Drive\06_Memory Palace\Knowledge\Entities\Projects\260830_HOC_Synthetic_Overview.md`)

This document establishes the repository guidelines, architectural boundaries, core tenets, default Memory Palace integration rules, and human-in-the-loop interaction protocols for all AI coding agents working in this workspace.

---

## 🏛️ 1. Core Tenets & Non-Negotiable Invariants

1. **Default Memory Palace Grounding (Always-On Truth Retrieval):**
   - Standing project identity (`Knowledge/Entities/Projects/260830_HOC_Synthetic_Overview.md`), declarative models, domain research, committed system specifications, and engineering standards (`Knowledge/Distilled/Concepts/`), repeatable workflows (`Knowledge/Distilled/Processes/`), and actionable implementation milestones (`Knowledge/Distilled/Tasks/`) reside in `G:\My Drive\06_Memory Palace`.
   - **Consult by Default:** Before answering project-specific questions, proposing architecture, or drafting plans, check the relevant nodes in `G:\My Drive\06_Memory Palace\Knowledge\` and any active scratchpad in `.agents/work/`—never hallucinate project rules from pre-trained knowledge.
   - **Inbox Dropbox Boundary:** When operating from a code workspace, read freely from `G:\My Drive\06_Memory Palace\Knowledge\`, but only write to `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`. Never directly mutate `Episodic/` or `Knowledge/` from a code workspace (graph ingestion is handled by `/ingest-inbox` inside the Memory Palace).

2. **Plain-Text & ASCII Math Formatting (CRITICAL: ZERO LATEX):**
   - **STRICT PROHIBITION:** NEVER use single dollar signs (`$...$`) or double dollar signs (`$$...$$`) for inline or block math in chat responses, explanations, markdown files, code comments, or documentation.
   - NEVER use LaTeX macros, symbols, or escape sequences (e.g. do NOT use `\dot{}`, `\ddot{}`, `\cdot`, `\varpi`, `\Omega`, `\approx`, `\times`, `\frac{}{}`, `\text{}`, `\sum`, `\int`, `_0` subscript tags, etc.).
   - Always render formulas, variables, and rates using clean ASCII/Unicode plain text or inline code formatting:
     - Write `a = a0 + a_dot * T` (NEVER LaTeX)
     - Write `P = P_base * (1 + k * (D - S) / S)`
     - Write `eps = v^2 / 2 - mu / r`
     - Render units as clean text: `250 km`, `1.5e11 m`, `m/s`, `AU`, `deg/cy`.

3. **Strict Type Safety:**
   - Write clean, modular, and strictly-typed code across all languages (e.g., Rust strong typing with zero unvetted `unsafe`, TypeScript `strict: true` with no `any` types, Python type hints with `mypy`/`pyright`).
   - Public APIs, data models, and service boundaries must declare explicit type contracts.

4. **Strict Layer Separation:**
   - Decouple Domain / Core Simulation Engine, Application / Orchestration, Infrastructure / I/O, and Presentation / 3D WebGPU Client.
   - Core simulation models and calculation pipelines must have **zero external I/O or UI/rendering dependencies**. State transformations and calculation pipelines must be pure, deterministic functions without side effects, citing downward to canonical Memory Palace node names (e.g., `[[Keplerian_Astrodynamics_Fibonacci_Globes_And_Light_Latency]]`).

5. **Code Quality & Immutability Standards:**
   - **Immutability by Default:** Prefer immutable data structures (`readonly`, `const`, Rust immutable bindings). Transform data by creating new state instances rather than mutating in place where appropriate.
   - **Explicit Error Handling:** Use explicit domain error or result types (`Result<T, E>`). Never write empty catch blocks or silently swallow errors. Fail fast with input validation at boundary layers.
   - **Focused Units:** Adhere to the Single Responsibility Principle. Keep individual functions small (aim for under 40 lines) and files under 250-300 lines.

6. **Lean Code Repository & Live Scratchpad (`.agents/work/`):**
   - The Git repository stores executable source code, tests, build configs, and agent workflow definitions—never permanent markdown archives of research or completed plans.
   - During discussion and research sessions (`/discuss`), a live scratchpad (`SCRATCHPAD-<slug>.md`) is maintained in `.agents/work/`.
   - At session closeout (`/push-to-memory`), conversational decisions, scratchpad research, roadmap items, and committed architectural specs are exported into `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\`, and transient `.agents/work/*.md` files are cleaned up.

7. **Dependency Discipline:**
   - Unauthorized third-party packages and external dependencies are strictly prohibited without prior architectural approval.

---

## 🤝 2. Human-in-the-Loop Safety Protocol

1. **Explain Reasoning & Architectural Diffs:** Before and during code modifications, clearly summarize the intent, file changes, and non-obvious design rationale.
2. **No Destructive Commands Without Confirmation:** Never execute destructive or irreversible commands (e.g. `rm -rf`, git hard resets, deleting production directories, dropping database schemas) without explicit user confirmation.
3. **API & Contract Integrity:** Do not alter established schemas or public interfaces without checking and updating all dependent invocation sites.
4. **Automated Verification:** Always run the project typecheck, lint, and test suite before declaring a coding task complete.
5. **Deliberate Pacing & Turn-Ending Next Steps:**
   - **No Rushing or Presumptive Urgency:** Never declare that "we are now ready to X" or assume the next step is mandatory. Maintain a calm, deliberate pace.
   - **Options-Oriented Next Steps:** At the conclusion of any task or discussion turn, offer 2–3 grounded paths forward based on `[[260830_HOC_Synthetic_Overview]]`, recent episodic project logs (`Episodic/Projects/`), or active `.agents/work/` items.
   - **Explore Alternatives & Pivots:** Always explicitly raise the option to pause, reprioritize, pivot to a different track, or pursue tangential exploration before moving forward.

---

## 🔄 3. Agentic Workflow Trajectory

The workspace combines **always-on Memory Palace grounding** and **Antigravity's native `/plan` workflow** with two custom skills:

1. **Default Memory Palace Lookup (Automatic):**
   - On any query or task, the agent consults `G:\My Drive\06_Memory Palace\Knowledge\` (`Entities/Projects/260830_HOC_Synthetic_Overview.md`, `Distilled/Concepts/`, `Distilled/Processes/`, `Distilled/Tasks/`) and any active scratchpad in `.agents/work/` by default.

2. **`/discuss` (Freeform Ideation, Iterative Research & Live Scratchpad):**
   - Used to brainstorm solutions, explore trade-offs, debate architecture, and decide on next steps or roadmap items.
   - Maintains a live, freely editable session scratchpad at `.agents/work/SCRATCHPAD-<slug>.md`.
   - Any research tasks (web lookups, codebase inspection, or native `research` subagent runs) append or edit findings, citations, and plain-text formulas directly into the active `.agents/work/SCRATCHPAD-<slug>.md`.

3. **Native Antigravity `/plan` (Implementation Planning & Execution):**
   - When ready to implement code, rely on Antigravity's built-in `/plan` command, using the Memory Palace specs and `.agents/work/SCRATCHPAD-<slug>.md` as grounding context.
   - Enforce contract-first design, pure domain separation, and automated verification (`cargo check`, `cargo clippy`, `cargo test`).

4. **`/push-to-memory` (Commit Session & Scratchpad to Memory Palace):**
   - Runs pre-push verification if code was modified.
   - Synthesizes both the **active conversation context** and the **`.agents/work/SCRATCHPAD-*.md` file** into a structured Context Dump in `G:\My Drive\06_Memory Palace\Inbox\Context_Dumps\YYYY_MM_DD_<ProjectSlug>_<Topic>.md` across target buckets:
     - `-> Episodic/Projects/` (Session summary, completed milestones & immediate next steps)
     - `-> Knowledge/Entities/Projects/` (Standing project scope & MOC updates)
     - `-> Knowledge/Distilled/Concepts/` (Domain research, derivations, trade-off analyses & committed architectural specs/SSOTs)
     - `-> Knowledge/Distilled/Processes/` & `-> Knowledge/Distilled/Tasks/` (Repeatable workflows & actionable milestones/tasks)
   - Purges exported `.agents/work/*.md` scratchpads so the Git repository stays clean.

Specific skill definitions are located under [`.agents/skills/`](.agents/skills/).

---

## 📁 4. Workspace Map & Navigation

| Path | Purpose | Key References |
| :--- | :--- | :--- |
| `AGENTS.md` | Master agent grounding, Memory Palace binding & invariants | [AGENTS.md](AGENTS.md) |
| `README.md` | Human-facing repository overview | [README.md](README.md) |
| `G:\My Drive\06_Memory Palace` | Permanent CoALA Memory Palace knowledge graph & episodic logs | `Knowledge/Entities/Projects/260830_HOC_Synthetic_Overview.md`, `Knowledge/Distilled/`, `Episodic/Projects/` |
| `.agents/work/` | Ephemeral in-repo scratchpad directory (git-ignored except `.gitkeep`) | `SCRATCHPAD-<slug>.md` (exported & purged on `/push-to-memory`) |
| `.agents/skills/` | Custom Memory Palace bridge skills | [discuss](.agents/skills/discuss/SKILL.md), [push-to-memory](.agents/skills/push-to-memory/SKILL.md) |
| `.vscode/` | Workspace editor & sandbox configuration | [settings.json](.vscode/settings.json), [extensions.json](.vscode/extensions.json) |

---

## ⚡ 5. Standard Verification Commands

*Active verification commands for this Rust workspace:*

| Check | Command / Mechanism | Purpose |
| :--- | :--- | :--- |
| **Typecheck / Check** | `cargo check --workspace --all-targets` | Verify strict type safety & compilation |
| **Lint** | `cargo clippy --workspace --all-targets -- -D warnings` | Verify code cleanliness, performance & style |
| **Unit & Invariant Tests** | `cargo test --workspace` | Verify hermetic unit & property-based invariant pass rate |
| **Build** | `cargo build --workspace` | Verify clean compilation & packaging |

---

## ✅ 6. Pre-Push QA Checklist (When Code Is Modified)

Prior to pushing a coding session (`/push-to-memory`):

- [ ] **1. Type Check:** Clean compilation with zero type errors (`cargo check --workspace --all-targets`).
- [ ] **2. Unit & Regression Tests:** 100% pass rate across unit and regression test suites (`cargo test --workspace`).
- [ ] **3. Contract Adherence:** Code strictly satisfies cited Memory Palace `Knowledge/Distilled/Concepts/` specs.
- [ ] **4. Zero LaTeX Audit:** Zero unescaped dollar signs (`$...$`, `$$...$$`) or LaTeX macros in docs, Context Dumps, code comments, or chat output.
- [ ] **5. Clean Git Status:** Transient `.agents/work/*.md` scratchpads are exported to `06_Memory Palace\Inbox\Context_Dumps\` and cleaned from the repo; no lingering debug artifacts remain.
