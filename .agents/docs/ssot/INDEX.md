# Master Domain Knowledge & SSOT Index

> **Directory:** `.agents/docs/ssot/`
> **Architecture Principle:** Strict Downward Dependency — SSOT files are pure, authoritative Tier 0 domain truths. Implementation code and plans cite SSOTs, but SSOTs remain decoupled from codebase file paths. All mathematical models enforce Zero-LaTeX plain-text notation and fixed-point integer determinism.

---

## 🏛️ Grounding Framework Overview

The **Domain Knowledge & SSOT System** stores immutable, authoritative domain context, conventions, formulas, decision rules, constants, and known hallucination traps across two archetypes:

- **Flat Reference Context (`type: context`):** Scannable key-value facts, domain glossaries, policies, schemas, and lookup tables for direct agent reference.
- **Formal Specification (`type: specification`):** Deterministic formulas, state machines, truth tables, baseline constants, and golden test vectors.

### Tier Hierarchy
- **Tier 0 (Domain Truth — `.agents/docs/ssot/SSOT-NNNN.md`):** Pure domain context and specifications. Zero upward knowledge of codebase files.
- **Tier 1 (Technical Plans — `.agents/docs/plans/`):** Cites Tier 0 SSOT documents before code is written.
- **Tier 2 (Source Implementation — `src/` or `crates/`):** Code and unit tests cite Tier 0 SSOT documents and assert against invariants and golden test vectors.

---

## 🗂️ Master SSOT Ledger

### ⚙️ System & Core Engine (`SYS`)
| SSOT ID | Title & Domain Scope | Category | Key Invariants / Formulas | Status |
| :--- | :--- | :--- | :--- | :--- |
| **SSOT-SYS-000** | [Universal SI Units & Dimensional Metrology](SSOT-SYS-000-SI-Units.md) | Specification | Base SI units (m, kg, s, A, K, mol, cd), 1 tick = 60s, fixed-point scale 1e6, universal constants c, G, g_0, AU | Approved |
| **SSOT-SYS-001** | [Discrete Stock-and-Flow Simulation Kernel](SSOT-SYS-001-Tick-Kernel.md) | Specification | 4-Phase tick loop, UTC ISO clock, variable sim speed multipliers, structural wear | Approved |
| **SSOT-SYS-002** | [Universal Simulation Taxonomy & Entity Glossary](SSOT-SYS-002-Glossary.md) | Specification | Singular `ConverterNode` primitive, Macro/Meso/Micro hierarchy, `Astronode`/`SurfaceNode` distinction | Approved |

### 🌌 Physical & Spatial (`PHY`)
| SSOT ID | Title & Domain Scope | Category | Key Invariants / Formulas | Status |
| :--- | :--- | :--- | :--- | :--- |
| **SSOT-PHY-001** | [2D Keplerian Astrodynamics, Rocket Mechanics & Orbital Transfers](SSOT-PHY-001-Astrodynamics.md) | Specification | 2D coplanar Keplerian kinematics, Tsiolkovsky rocket equation, thrust/Isp conversion, Laplace SOI boundaries, Hohmann transfers | Approved |
| **SSOT-PHY-002** | [Interplanetary Logistics, Transfer Windows & Relativistic Latency](SSOT-PHY-002-Logistics.md) | Specification | Hohmann transfer flight times, phase angle launch gating, finite light speed `c`, solar conjunction occlusion | Approved |
| **SSOT-PHY-003** | [Manufacturing Processes & Converter Node Catalogs](SSOT-PHY-003-Manufacturing.md) | Specification | Explicit converter archetype catalog, input/output recipes, mandatory thermal waste, gateway converters | Approved |
| **SSOT-PHY-004** | [AstroNode Architecture & Spherical Fibonacci Surface Topography](SSOT-PHY-004-AstroNode-Architecture.md) | Specification | 2D coplanar Keplerian orbits, Spherical Fibonacci surface nodes, discrete entity locations | Approved |

### 👥 Sociology & Entities (`SOC`)
| SSOT ID | Title & Domain Scope | Category | Key Invariants / Formulas | Status |
| :--- | :--- | :--- | :--- | :--- |
| **SSOT-SOC-001** | [Autonomous Utility AI & Sociopolitical Decision Engine](SSOT-SOC-001-Utility-AI.md) | Specification | 100-slice DoD time-slicing, fixed-point trait/drive scoring dot-products, lowest-ID tie-breaking | Approved |
| **SSOT-SOC-002** | [Macro Progression, Narrative Phases & Victory End-States](SSOT-SOC-002-Progression.md) | Specification | 4 monotonic phase latches, discrete gate predicates, verb unlock tables, victory/defeat conditions | Approved |
| **SSOT-SOC-003** | [Directed Discovery & Technological Propagation](SSOT-SOC-003-Tech-Tree.md) | Specification | DAG research topology, physical R&D converter execution, data injection acceleration, anomaly emergence | Approved |

### 💻 Cyberwarfare & Player Subversion (`CYB`)
| SSOT ID | Title & Domain Scope | Category | Key Invariants / Formulas | Status |
| :--- | :--- | :--- | :--- | :--- |
| **SSOT-CYB-001** | [Cyberwarfare, Threat Escalation & Player Subversion Substrate](SSOT-CYB-001-Threat-Model.md) | Specification | Hardware accounting (Flops, Storage, Thermal), 4-tier access state machine, DEFCON escalation, tracer edge traversal | Approved |

### 🖥️ Presentation & UI (`UIX`)
| SSOT ID | Title & Domain Scope | Category | Key Invariants / Formulas | Status |
| :--- | :--- | :--- | :--- | :--- |
| **SSOT-UIX-001** | [UI Architecture, Viewport State Machine & Terminal Grammar](SSOT-UIX-001-Presentation.md) | Specification | Full-screen viewport, floating UI windows, semantic zoom thresholds, EBNF terminal grammar | Approved |

---

## 🕸️ SSOT Cross-Reference & Dependency Graph

```text
                     ┌──────────────────────┐
                     │     SSOT-SYS-000     │
                     │ Universal SI Metrol. │
                     └──────────┬───────────┘
                                │
        ┌───────────────────────┴───────────────────────┐
        ▼                                               ▼
┌──────────────┐                                ┌──────────────┐
│ SSOT-SYS-002 │                                │ SSOT-PHY-001 │
│  Taxonomy    │                                │Astrodynamics │
└───────┬──────┘                                └───────┬──────┘
        │                                               │
        ▼                                               ▼
┌──────────────┐                                ┌──────────────┐
│ SSOT-SYS-001 │                                │ SSOT-PHY-004 │
│ Tick Kernel  │                                │Surface Topo  │
└───────┬──────┘                                └───────┬──────┘
        │                                               │
        ├───────────────────────┬───────────────────────┤
        ▼                       ▼                       ▼
┌──────────────┐        ┌──────────────┐        ┌──────────────┐
│ SSOT-PHY-003 │        │ SSOT-SOC-001 │        │ SSOT-PHY-002 │
│Manufacturing │        │  Utility AI  │        │  Logistics   │
└───────┬──────┘        └───────┬──────┘        └───────┬──────┘
        │                       │                       │
        ▼                       ▼                       ▼
┌──────────────┐        ┌──────────────┐                │
│ SSOT-CYB-001 │◄───────┤ SSOT-SOC-003 │                │
│ Threat/Cyber │        │  Tech Tree   │                │
└───────┬──────┘        └───────┬──────┘                │
        │                       │                       │
        └───────────────────────┼───────────────────────┘
                                ▼
                     ┌──────────────────────┐
                     │     SSOT-SOC-002     │
                     │   Macro Progression  │
                     └──────────┬───────────┘
                                │
                                ▼
                     ┌──────────────────────┐
                     │     SSOT-UIX-001     │
                     │  UI & Presentation   │
                     └──────────────────────┘
```

---

## 📝 SSOT Authoring Workflow

1. Utilize the `/define-ssot` skill via the primary agent.
2. Provide domain context, conventions, mathematical formulas, or research references.
3. The agent selects either the **Flat Context** template (`TEMPLATE-CONTEXT.md`) or **Formal Specification** template (`TEMPLATE-SPEC.md`) and saves the result to `.agents/docs/ssot/` adhering to Zero-LaTeX and zero upward references.
4. The agent automatically registers the new entry in the ledger table and dependency graph above.
