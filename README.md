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
├── .antigravity/
│   ├── rules/                           # Persistent architectural & coding constraints
│   │   ├── 00-core-architecture.md      # Layer separation, contracts, pure pipelines
│   │   ├── 01-coding-standards.md       # Naming, immutability, explicit error handling
│   │   └── 02-verification.md           # Testing mandate & 7-point QA gate
│   ├── plans/                           # Plan specification framework
│   │   ├── TEMPLATE.md                  # Unified PLAN-XXX blueprint template
│   │   ├── active/                      # In-flight active plans
│   │   └── archive/                     # Completed/superseded plans & ledger
│   │       └── index.md                 # Master historical plan ledger
│   ├── agents/                          # Custom subagent definitions
│   │   ├── planner.md                   # Deep research & contract design (read-only)
│   │   ├── implementer.md               # Atomic step implementation (branch/inherit)
│   │   └── code-reviewer.md             # Static analysis & QA gatekeeper
│   └── workflows/                       # Slash-command workflow playbooks
│       ├── plan.md                      # /plan: Dependency scan & plan drafting
│       ├── execute.md                   # /execute: Sequential task implementation
│       └── archive-plan.md              # /archive-plan: QA verification & archival
├── docs/                                # Living system documentation
│   ├── ARCHITECTURE.md                  # Layer topology, data flows, core tenets
│   ├── ROADMAP.md                       # Long-term vision & milestone tracking
│   └── TASKS.md                         # Active sprint task board
└── .vscode/                             # Workspace editor settings & extensions
    ├── settings.json
    └── extensions.json
```

---

## ⚡ Quick Start: Agentic Workflow

### 1. Planning a New Feature or Refactor
Invoke the `/plan` workflow to create an architectural blueprint:
```bash
/plan "Implement user authentication service"
```
- The **Planner Subagent** will inspect dependencies, design technical contracts, and draft `.antigravity/plans/active/PLAN-001.md`.
- Review the executive summary in chat and refine the technical contracts before approving.

### 2. Executing the Plan
Once approved, execute atomic tasks sequentially:
```bash
/execute
```
- The **Implementer Subagent** implements one task at a time, running unit test assertions at each step and marking progress.

### 3. Reviewing & Archiving
When all tasks and tests pass:
```bash
/archive-plan
```
- The **Code Reviewer Subagent** validates the 7-point QA checklist, logs runtime deviations, updates metadata, and archives the plan into `.antigravity/plans/archive/YYYY/`.

---

## 🛡️ Core Rules & Invariants

1. **Strict Type Safety:** No `any` types; all boundaries and domain models must have explicit types.
2. **Plain-Text & ASCII Math Formatting (CRITICAL: ZERO LATEX):** Never use LaTeX or dollar sign delimiters (`$...$`, `$$...$$`). Render formulas in clean plain text (e.g. `a = a0 + a_dot * T`, `P = P_base * (1 + k * (D - S) / S)`).
3. **Strict Layer Separation:** Domain logic is 100% pure and decoupled from UI, network, and file system I/O.
4. **Bounded Task Execution:** Implementation steps must remain small (under 150-200 lines) with tests asserting correctness before moving to the next step.

---

## 📚 Documentation Index

- [AGENTS.md](file:///AGENTS.md) — Master Agent Guidelines & Invariants
- [docs/ARCHITECTURE.md](file:///docs/ARCHITECTURE.md) — System Topology & Layer Contracts
- [docs/ROADMAP.md](file:///docs/ROADMAP.md) — Milestone Roadmap & Ideas Sandbox
- [docs/TASKS.md](file:///docs/TASKS.md) — Active Sprint Task Board
- [.antigravity/rules/](file:///.antigravity/rules/) — Workspace Rule Specifications
