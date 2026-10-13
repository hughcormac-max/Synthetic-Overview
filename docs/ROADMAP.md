# Project Roadmap & Vision

> **Document:** `docs/ROADMAP.md`
> **Status:** Living Master Roadmap | **Active Tasks:** [TASKS.md](TASKS.md) | **Master Reference:** [AGENTS.md](../AGENTS.md)

---

## 🎯 1. Vision & Core Objectives

*Synthetic Overview* is a solar-system scale, AGI-driven grand strategy and simulation game set in 2040. This roadmap tracks the progression from the foundational 3D astrodynamics and spherical surface engine to the complete stock-and-flow economy, autonomous utility AI, cyberwarfare substrate, and diegetic UI.

Milestones from this roadmap are decomposed into formal [PLAN-NNNN](../.agents/skills/plan-human/assets/TEMPLATE.md) specifications in [`.agents/docs/plans/`](../.agents/docs/plans/INDEX.md) and tracked on the active [TASKS.md](TASKS.md) board.

---

## 🗺️ 2. Milestone Phases

### 🧱 Phase 1: Foundation, Engine Architecture & 3D Spatial Viewer (Completed)

- [x] **Agentic Framework & Tier 0 SSOT Library:** Established 12 authoritative Tier 0 SSOT specifications (`SYS`, `PHY`, `SOC`, `CYB`, `UIX`), 10 grounded engine research reports, and the two-phase planning + Boomerang execution pipeline (`PLAN-001`, `PLAN-007`, `PLAN-008`, `PLAN-011`, `PLAN-017`).
- [x] **Rust Workspace & Bevy Rendering Stack:** Decoupled pure domain logic (`crates/synthetic-core`) from the Bevy 3D presentation client (`crates/synthetic-client`) (`PLAN-006`).
- [x] **Spherical Fibonacci Surface Topography:** Implemented deterministic Spherical Fibonacci surface lattices (`SSOT-PHY-004`) with WebGPU backface culling and globe camera controls (`PLAN-009`, `PLAN-010`, `PLAN-012`, `PLAN-013`).
- [x] **Full 3D Keplerian Solar System & Floating Origin:** Implemented 64-bit Keplerian astrodynamics (`SSOT-PHY-001`), camera-relative floating origin translation, semantic zoom, mouse raycast picking, and ecliptic orientation HUD (`PLAN-014`, `PLAN-015`, `PLAN-016`, `PLAN-018`).
- [x] **Static Geometric Orbit Rendering & Time Warp Controls:** Implemented 360-dot Eccentric Anomaly static orbital meshes with frustum culling and discrete simulation time warp controls (`0x` to `2,592,000x`) with a keyboard shortcut help modal (`PLAN-019`, `PLAN-020`, `PLAN-021`).

### ⚙️ Phase 2: Discrete Stock-and-Flow Kernel & Converter Economy

- [ ] **4-Phase Discrete Tick Kernel (`SSOT-SYS-001`):** Implement the deterministic 60-second tick loop in `synthetic-core` (Phase 1: Intent & Demand Aggregation, Phase 2: Pro-Rata Input Rationing, Phase 3: Recipe Transformation & Mandatory Thermal Waste, Phase 4: Structural Wear & State Latch).
- [ ] **Universal `ConverterNode` & Resource Taxonomy (`SSOT-SYS-002`, `SSOT-PHY-003`):** Define pure domain structs and catalogs for raw extraction, refining, manufacturing, compute racks, power generation, and radiator nodes.
- [ ] **Surface Node Attachment & Local Grids (`SSOT-PHY-004`):** Bind `ConverterNode` installations and resource stockpiles to discrete `SurfaceNode` and `AstroNode` locations.
- [ ] **Simulation Clock & Telemetry HUD (`SSOT-UIX-001`):** Surface UTC epoch clock, active warp speed readout, and node stock/flow inspection in `synthetic-client`.

### 🚀 Phase 3: Interplanetary Logistics, Transfers & Relativistic Latency

- [ ] **Hohmann Transfer & Launch Window Solver (`SSOT-PHY-001`, `SSOT-PHY-002`):** Implement coplanar Hohmann delta-v calculations, synodic phase angle gating, and Tsiolkovsky propellant mass-fraction checks.
- [ ] **Spacecraft Trajectory Forecasting:** Render active transit arcs, departure/arrival markers, and Sphere of Influence (SOI) transitions in the 3D solar system view.
- [ ] **Causal Light-Speed (`c`) Latency & Conjunction Occlusion (`SSOT-PHY-002`):** Model distance-delayed telemetry/command packets and solar conjunction line-of-sight blackouts.

### 🧠 Phase 4: Utility AI, Cyberwarfare Substrate & Diegetic Terminal

- [ ] **100-Slice Autonomous Utility AI (`SSOT-SOC-001`):** Implement deterministic time-sliced faction/agent evaluation using trait-drive dot products and lowest-ID tie-breaking.
- [ ] **Cyberwarfare & Hardware Accounting (`SSOT-CYB-001`):** Implement AGI compute accounting (FLOPS, storage, thermal load), 4-tier node access infiltration, DEFCON threat escalation, and active tracer traversal.
- [ ] **Directed Discovery Tech Tree & Macro Progression (`SSOT-SOC-002`, `SSOT-SOC-003`):** Implement DAG research progression driven by physical R&D converters, monotonic narrative phase latches, and end-state victory/defeat predicates.
- [ ] **Floating Window UI & EBNF Command Terminal (`SSOT-UIX-001`):** Build floating inspector windows and the deterministic EBNF command-line parser for headless/tactical AGI control.

---

## 💡 3. Ideas Sandbox & Future Explorations

*Unscheduled features, experimental ideas, and research spikes.*

- **Surface Node Visual Heatmap Modes:** Shader-driven coloring of Spherical Fibonacci surface nodes by thermal signature, resource density, or faction control.
- **Deterministic Replay & Snapshot Verification:** Save/load state hashing across tick boundaries to verify zero drift across platforms.
- **Interactive Porkchop / Phase-Angle Indicator:** Visual launch window arc indicator between origin and destination orbits when planning interplanetary transfers.
