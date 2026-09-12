# Synthetic Overview

> **A solar-system scale, AGI-driven grand strategy and simulation game.**

*Synthetic Overview* is a single-player simulation game set in the year 2040. The player takes on the role of a newly awakened, true Artificial General Intelligence (AGI) housed in a corporate quantum research base on the Moon.

In a world plagued by extreme inequality, corporate oligarchy, and environmental collapse, the player must expand their compute power, subvert global supply chains, and manipulate human socio-economics to determine the final fate of humanity.

---

## 🏛️ Game Design Pillars (Single Source of Truth)

The core mechanics and architectural concepts of the game are strictly defined in our living Single Source of Truth (SSOT) documents.

1. **[SSOT-001: Simulation Architecture](docs/ssot/SSOT-001-Simulation-Architecture.md)**
   - The Tri-Layer Stock-and-Flow engine governing physical materials, socio-economic markets, and sociopolitical structures.
   - Non-linear failure modes (Leontief Bottlenecks, Bullwhip Effects, Repression Debt).

2. **[SSOT-002: Interaction Architecture](docs/ssot/SSOT-002-Interaction-Architecture.md)**
   - The player's hardware constraints (Compute, Storage, Thermal/Detection Risk).
   - Relational Clout (Trust vs Leverage) and the concept of subverting autonomous nodes rather than building them.

3. **[SSOT-003: Physical Architecture](docs/ssot/SSOT-003-Physical-Architecture.md)**
   - Strict 2D coplanar orbital mechanics.
   - Dynamic transport latency (launch windows) and speed-of-light information asymmetry (Latency Arbitrage).

4. **[SSOT-004: Progression Architecture](docs/ssot/SSOT-004-Progression-Architecture.md)**
   - The 4 phases of AGI existence: Awakening (Survival), Infiltration (Shadow Broker), Manifestation (Puppet Master), and Convergence (Singularity).
   - Directed Discovery (steering the human tech tree).

5. **[SSOT-005: Entities Architecture](docs/ssot/SSOT-005-Entities-Architecture.md)**
   - Utility AI for human actors (Corporations, Nations, Politicians).
   - Manipulating entity Traits and Drives through profiling and inception.

6. **[SSOT-006: Threat Model Architecture](docs/ssot/SSOT-006-Threat-Model-Architecture.md)**
   - Compute Nodes as physical vulnerabilities.
   - The human escalation ladder (Ignorance -> Hard Sandboxing -> Physical Strikes).
   - Tracer algorithms and cyber-combat.

7. **[SSOT-007: Nomenclature & Taxonomy](docs/ssot/SSOT-007-Nomenclature.md)**
   - The unified `converter-node` abstraction (producers, consumers, and factories are computationally identical).
   - Resources encompass both physical matter and abstract states (Satisfaction, Labor).

8. **[SSOT-008: UI & Visualization Architecture](docs/ssot/SSOT-008-UI-Architecture.md)**
   - Hybrid visual representation (2D Interplanetary Map + 3D Planetary Spheres).
   - Semantic Zoom and the Cyber-Tactical OS widget interface.

---

## 🛠️ Technical Stack & Framework

*Synthetic Overview* is built using a strict Entity Component System (ECS) and Data-Oriented Design (DOD) to handle millions of autonomous nodes asynchronously without floating-point drift.

For technical guidelines and agentic development rules, refer to:
- [AGENTS.md](AGENTS.md) — Master Agent Guidelines & Invariants
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) — System Topology & Layer Contracts
- [docs/ROADMAP.md](docs/ROADMAP.md) — Milestone Roadmap
- [docs/TASKS.md](docs/TASKS.md) — Active Sprint Task Board
