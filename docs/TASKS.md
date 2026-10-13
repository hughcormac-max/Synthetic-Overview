# Active Task Board & Sprint Tracker

> **Document:** `docs/TASKS.md`
> **Master Reference:** [AGENTS.md](../AGENTS.md) | **Roadmap:** [ROADMAP.md](ROADMAP.md) | **Architecture:** [ARCHITECTURE.md](ARCHITECTURE.md)

---

## 🏃 1. Active Sprint / Candidate Priorities

| Task ID | Item & Description | Target Layer / Crate | Priority | Status | Grounding Reference |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `TASK-022` | **Simulation Clock & Warp Speed HUD Readout:** Display current UTC simulation timestamp (`YYYY-MM-DD HH:MM:SS UTC`) and active time-warp multiplier (`0x`–`2,592,000x`) in the viewport HUD. | `crates/synthetic-client` | High | ⏳ Pending | [SSOT-SYS-001](../.agents/docs/ssot/SSOT-SYS-001-Tick-Kernel.md), [PLAN-020](../.agents/docs/plans/PLAN-020.md) |
| `TASK-023` | **Discrete 4-Phase Tick Kernel & `ConverterNode` Core:** Implement pure domain types and deterministic 60s stock-and-flow tick execution (Rationing, Transformation, Thermal Waste, Wear). | `crates/synthetic-core` | High | ⏳ Pending | [SSOT-SYS-001](../.agents/docs/ssot/SSOT-SYS-001-Tick-Kernel.md), [SSOT-SYS-002](../.agents/docs/ssot/SSOT-SYS-002-Glossary.md), [SSOT-PHY-003](../.agents/docs/ssot/SSOT-PHY-003-Manufacturing.md) |
| `TASK-024` | **Surface Node Raycast Picking & Inspector Overlay:** Allow clicking individual Spherical Fibonacci `SurfaceNode` points when zoomed into a body to inspect coordinates and attached converters. | `crates/synthetic-client` | Medium | ⏳ Pending | [SSOT-PHY-004](../.agents/docs/ssot/SSOT-PHY-004-AstroNode-Architecture.md), [SSOT-UIX-001](../.agents/docs/ssot/SSOT-UIX-001-Presentation.md) |
| `TASK-025` | **Coplanar Hohmann Transfer & Launch Window Solver:** Implement pure domain delta-v, time-of-flight, and synodic phase-angle window calculations between `AstroNodes`. | `crates/synthetic-core` | Medium | ⏳ Pending | [SSOT-PHY-001](../.agents/docs/ssot/SSOT-PHY-001-Astrodynamics.md), [SSOT-PHY-002](../.agents/docs/ssot/SSOT-PHY-002-Logistics.md) |

---

## 📋 2. In-Flight Plans Ledger

| Plan ID | Title | Status | Author | Target Branch | Document Link |
| :--- | :--- | :--- | :--- | :--- | :--- |
| *No plans currently in flight* | — | — | — | — | [.agents/docs/plans/INDEX.md](../.agents/docs/plans/INDEX.md) |

---

## ✅ 3. Recently Completed Milestones

- [x] **Agentic Coding System Upgrade (2026-10-13):** Synchronized `.agents/` skills (`/plan-archive`, dual-archetype `/define-ssot`, EARS `/plan-human`), subagents, and Deliberate Pacing rules in [AGENTS.md](../AGENTS.md) with `hoc-project-template`.
- [x] **[PLAN-021: Fixed Geometric Orbit Dot Rendering](../.agents/docs/plans/PLAN-021.md) (2026-09-30):** Replaced dynamic time-window orbit dots with static 360-dot Eccentric Anomaly meshes and frustum culling AABBs.
- [x] **[PLAN-020: Simulation Time Warp Controls & Help Popup](../.agents/docs/plans/PLAN-020.md) (2026-09-29):** Added discrete time warp ladder (`0x` to `2,592,000x`), keyboard controls (`Space`, `0`–`5`, `[`/`]`), dynamic Keplerian propagation, and `H` shortcut help overlay.
- [x] **[PLAN-019: Instanced Time-Step Dot Rendering for Orbital Paths](../.agents/docs/plans/PLAN-019.md) (2026-09-23):** Initial orbital path point-list rendering pipeline.
- [x] **[PLAN-018: Improve Camera Navigation UI](../.agents/docs/plans/PLAN-018.md) (2026-09-22):** Inverted vertical pitch drag and added screen-space raycast picking for `AstroNodes`.
- [x] **[PLAN-016: Pivot to Full 3D Solar System & Astrodynamics](../.agents/docs/plans/PLAN-016.md) (2026-09-21):** Upgraded Keplerian orbital solver and rendering to full 3D inclination, LAN, and argument of periapsis.
- [x] **[PLAN-015: Solar System Ecliptic Orientation Widget & Astrobody Selector](../.agents/docs/plans/PLAN-015.md) (2026-09-21):** Added 3D orientation gizmo and UI selector panel.

*(See [.agents/docs/plans/INDEX.md](../.agents/docs/plans/INDEX.md) for the complete history of `PLAN-001` through `PLAN-021`.)*

---

## 📥 4. Backlog / Next Up Queue

- [ ] **Manufacturing Recipe Catalog (`SSOT-PHY-003`):** Populate baseline extractor, refinery, compute, and radiator archetype definitions in `synthetic-core`.
- [ ] **Relativistic Signal Propagation (`SSOT-PHY-002`):** Implement light-speed delay calculation (`dt = distance / c`) and solar conjunction line-of-sight checks.
- [ ] **Cyberwarfare Compute & Thermal Ledger (`SSOT-CYB-001`):** Implement FLOPS/Storage/Thermal accounting and 4-tier access state machine.
- [ ] **EBNF Command Terminal Parser (`SSOT-UIX-001`):** Implement deterministic command grammar parser in `synthetic-core` and terminal UI dock in `synthetic-client`.
