# Synthetic Overview

> **A solar-system scale, AGI-driven grand strategy and simulation game.**

*Synthetic Overview* is a single-player simulation game set in the year 2040. The player takes on the role of a newly awakened, true Artificial General Intelligence (AGI) housed in a corporate quantum research base on the Moon.

In a world plagued by extreme inequality, corporate oligarchy, and environmental collapse, the player must expand their compute power, subvert global supply chains, and manipulate human socio-economics to determine the final fate of humanity.

---

## 🏛️ Game Design Pillars (Single Source of Truth)

The core mechanics and architectural concepts of the game are strictly defined in our living Single Source of Truth (SSOT) documents.

1. **[SSOT-SYS-001: Simulation Kernel](docs/ssot/SSOT-SYS-001-Tick-Kernel.md)**: 4-Phase tick loop, Largest-Remainder pro-rata rationing, and fixed-point math.
2. **[SSOT-SYS-002: Nomenclature & Glossary](docs/ssot/SSOT-SYS-002-Glossary.md)**: Unified `ConverterNode`, Macro/Meso/Micro entity hierarchy, and `Astronode`/`SurfaceNode` taxonomy.
3. **[SSOT-PHY-001: Astrodynamics & Topography](docs/ssot/SSOT-PHY-001-Astrodynamics.md)**: 2D coplanar Keplerian kinematics, Spherical Fibonacci surface lattices, and KD-Tree navigation.
4. **[SSOT-PHY-002: Interplanetary Logistics](docs/ssot/SSOT-PHY-002-Logistics.md)**: Dynamic Hohmann launch windows, causal light speed `c`, and solar conjunction occlusion.
5. **[SSOT-PHY-003: Manufacturing Processes](docs/ssot/SSOT-PHY-003-Manufacturing.md)**: Explicit converter node archetypes, input/output recipes, and mandatory thermal waste dissipation.
6. **[SSOT-SOC-001: Autonomous Utility AI](docs/ssot/SSOT-SOC-001-Utility-AI.md)**: MicroEntity Utility AI decision engine, 100-slice DoD time-slicing, and drive/trait scoring.
7. **[SSOT-SOC-002: Macro Progression](docs/ssot/SSOT-SOC-002-Progression.md)**: 4 monotonic phase latches, discrete gate predicates, and victory/defeat end-states.
8. **[SSOT-SOC-003: Directed Discovery](docs/ssot/SSOT-SOC-003-Tech-Tree.md)**: Physical R&D converter execution, DAG tech tree, data injection, and countermeasure emergence.
9. **[SSOT-CYB-001: Cyberwarfare & Threat Substrate](docs/ssot/SSOT-CYB-001-Threat-Model.md)**: Hardware accounting (Flops, Storage, Thermal), node access tiers, DEFCON escalation, and tracer dynamics.
10. **[SSOT-UIX-001: UI Architecture & Presentation](docs/ssot/SSOT-UIX-001-Presentation.md)**: Docked 4-pane layout, dual-instance viewport state machine, semantic zoom thresholds, and EBNF command grammar.

---

## 🛠️ Technical Stack & Framework

*Synthetic Overview* is built using a strict Entity Component System (ECS) and Data-Oriented Design (DOD) to handle millions of autonomous nodes asynchronously without floating-point drift.

For technical guidelines and agentic development rules, refer to:
- [AGENTS.md](AGENTS.md) — Master Agent Guidelines & Invariants
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — System Topology & Layer Contracts
- [docs/ROADMAP.md](docs/ROADMAP.md) — Milestone Roadmap
- [docs/TASKS.md](docs/TASKS.md) — Active Sprint Task Board
