# HOC Project Template for Antigravity

> **Universal, Multi-Stack Agentic Coding Template for Google Antigravity & AI Assistants**

This repository provides a standardized, anti-hallucination development framework designed for autonomous and pair-programming AI agents. It establishes strict layer separation, contract-first technical planning, atomic task execution, and automated verification.

---

## 🏛️ Architecture & Governance

The workspace is governed by a 4-tier structure designed for context bounding and deterministic agent execution:

```text
HOC-project-template/
├── AGENTS.md                            # Primary AI agent entry point & grounding rules
├── README.md                            # Human-facing project overview & quick start
├── .agents/
│   ├── plans/                           # Plan specification framework
│   │   ├── active/                      # In-flight active plans
│   │   └── archive/                     # Completed/superseded plans & ledger
│   │       └── INDEX.md                 # Master historical plan ledger
│   ├── subagents/                       # Custom subagent definitions
│   │   ├── planner.md                   # Deep research & contract design (read-only)
│   │   ├── implementer.md               # Atomic step implementation (branch/inherit)
│   │   ├── ssot-writer.md               # Distills knowledge into Tier 0 SSOT docs
│   │   ├── reporter.md                  # Compiles research into markdown reports
│   │   ├── web-researcher.md            # Online documentation & bug research
│   │   ├── code-reviewer.md             # Static analysis & QA gatekeeper
│   │   └── doc-researcher.md            # Zero-hallucination SSOT/Research doc retriever
│   └── skills/                          # Actionable skills & slash commands
│       ├── plan/                        # /plan: Dependency scan & plan drafting
│       │   └── resources/TEMPLATE.md    # Unified PLAN-XXX blueprint template
│       ├── execute/                     # /execute: Sequential task implementation
│       ├── archive-plan/                # /archive-plan: QA verification & archival
│       ├── research/                    # /research: Deep online & codebase research
│       ├── define-ssot/                 # /define-ssot: Establish Single Source of Truth
│       └── ask-docs/                    # /ask-docs: Query internal truth & research docs
├── docs/                                # Living system documentation
│   ├── ssot/                            # Single Source of Truth vault (Tier 0)
│   ├── research/                        # Digested research reports
│   ├── ARCHITECTURE.md                  # Layer topology, data flows, core tenets
│   ├── ROADMAP.md                       # Long-term vision & milestone tracking
│   └── TASKS.md                         # Active sprint task board
└── .vscode/                             # Workspace editor settings & extensions
    ├── settings.json
    └── extensions.json
```

---

## ⚡ Quick Start: Agentic Workflow

### 1. Research & Knowledge Discovery

To perform deep online research, establish an SSOT, or query existing documentation:

```bash
/research "What are the latest best practices for OAuth 2.0 PKCE?"
/define-ssot "Distill the OAuth 2.0 research into a new domain truth document"
/ask-docs "What are our token expiration rules?"
```

- **`/research`**: Orchestrates web-researcher and reporter subagents to produce a digested markdown report in `docs/research/`.
- **`/define-ssot`**: Uses the ssot-writer subagent to format knowledge into a pure Tier 0 `SSOT-NNNN` specification in `docs/ssot/`.
- **`/ask-docs`**: Dispatches the doc-researcher subagent to retrieve answers strictly from `docs/research/` and `docs/ssot/` without hallucinating.

### 2. Planning a New Feature or Refactor

Invoke the `/plan` workflow to create an architectural blueprint:

```bash
/plan "Implement user authentication service"
```

- The **Planner Subagent** will inspect dependencies, design technical contracts, and draft `.agents/plans/active/PLAN-001.md`.
- Review the executive summary in chat and refine the technical contracts before approving.

### 3. Executing the Plan

Once approved, execute atomic tasks sequentially:

```bash
/execute
```

- The **Implementer Subagent** implements one task at a time, running unit test assertions at each step and marking progress.

### 4. Reviewing & Archiving

When all tasks and tests pass:

```bash
/archive-plan
```

- The **Code Reviewer Subagent** validates the 7-point QA checklist, logs runtime deviations, updates metadata, and archives the plan into `.agents/plans/archive/YYYY/`.

---

## 🛡️ Core Rules & Invariants

1. **Strict Type Safety:** No `any` types; all boundaries and domain models must have explicit types.
2. **Plain-Text & ASCII Math Formatting (CRITICAL: ZERO LATEX):** Never use LaTeX or dollar sign delimiters (`$...$`, `$$...$$`). Render formulas in clean plain text (e.g. `a = a0 + a_dot * T`, `P = P_base * (1 + k * (D - S) / S)`).
3. **Strict Layer Separation:** Domain logic is 100% pure and decoupled from UI, network, and file system I/O.
4. **Bounded Task Execution:** Implementation steps must remain small (under 150-200 lines) with tests asserting correctness before moving to the next step.

---

## 📚 Documentation Index

- [AGENTS.md](AGENTS.md) — Master Agent Guidelines & Invariants
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — System Topology & Layer Contracts
- [docs/ROADMAP.md](docs/ROADMAP.md) — Milestone Roadmap & Ideas Sandbox
- [docs/TASKS.md](docs/TASKS.md) — Active Sprint Task Board
- [docs/ssot/INDEX.md](docs/ssot/INDEX.md) — Single Source of Truth Vault (Tier 0)
- [docs/research/INDEX.md](docs/research/INDEX.md) — Research Reports Ledger
