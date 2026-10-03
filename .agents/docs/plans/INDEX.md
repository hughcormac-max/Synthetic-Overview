# Master Plan Ledger

> **Location:** `.agents/docs/plans/INDEX.md`
> **Purpose:** Master ledger of all architectural and technical plans for this workspace across their entire lifecycle (draft, approved, in-progress, completed, superseded, abandoned).

---

## 🗂️ Master Plan Index

| Plan ID | Title | Status | Author | Created | Completed | Target Branch | Document Link |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| [PLAN-001](PLAN-001.md) | Define Tech Stack, Engine, and System Architecture | `completed` | Antigravity | 2026-09-12 | 2026-09-12 | `main` | [PLAN-001.md](PLAN-001.md) |
| [PLAN-002](PLAN-002.md) | Scaffold Core ECS and Node/Processor Network using bevy_ecs | `completed` | Antigravity | 2026-09-12 | 2026-09-12 | `main` | [PLAN-002.md](PLAN-002.md) |
| [PLAN-003](PLAN-003.md) | Scaffold H3 Spatial Grid for Astronomical Bodies | `completed` | Antigravity | 2026-09-12 | 2026-09-12 | `main` | [PLAN-003.md](PLAN-003.md) |
| [PLAN-004](PLAN-004.md) | Scaffold Initial Economic Simulation Scenario and IPC DTOs | `completed` | Antigravity | 2026-09-13 | 2026-09-13 | `main` | [PLAN-004.md](PLAN-004.md) |
| [PLAN-005](PLAN-005.md) | Scaffold Surface Layer Visualisation UI/UX Pipeline | `completed` | Antigravity | 2026-09-13 | 2026-09-13 | `main` | [PLAN-005.md](PLAN-005.md) |
| [PLAN-006](PLAN-006.md) | Pivot UI/Rendering Stack to Bevy | `completed` | Antigravity | 2026-09-13 | 2026-09-13 | `main` | [PLAN-006.md](PLAN-006.md) |
| [PLAN-007](PLAN-007.md) | Restructure and Standardize SSOT Library | `completed` | Antigravity | 2026-09-13 | 2026-09-16 | `main` | [PLAN-007.md](PLAN-007.md) |
| [PLAN-008](PLAN-008.md) | Re-architect SSOT for SI Units, Astrodynamics, and Topography | `completed` | Antigravity | 2026-09-13 | 2026-09-16 | `main` | [PLAN-008.md](PLAN-008.md) |
| [PLAN-009](PLAN-009.md) | Implement WebGPU Rendered Surface View with Backface Culling | `completed` | Antigravity | 2026-09-14 | 2026-09-16 | `main` | [PLAN-009.md](PLAN-009.md) |
| [PLAN-010](PLAN-010.md) | Implement Interactive Orbit & Zoom Camera Controls for Planetary Globe View | `completed` | Antigravity | 2026-09-16 | 2026-09-16 | `main` | [PLAN-010.md](PLAN-010.md) |
| [PLAN-011](PLAN-011.md) | Transition SSOT to f64 Math & Unified 3D Coordinates | `completed` | Antigravity | 2026-09-17 | 2026-09-17 | `main` | [PLAN-011.md](PLAN-011.md) |
| [PLAN-012](PLAN-012.md) | Scaffold Initial Solar System & Surface Nodes | `completed` | Antigravity | 2026-09-20 | 2026-09-20 | `main` | [PLAN-012.md](PLAN-012.md) |
| [PLAN-013](PLAN-013.md) | Switch Surface Topography to Spherical Fibonacci Lattice | `completed` | Antigravity | 2026-09-20 | 2026-09-20 | `main` | [PLAN-013.md](PLAN-013.md) |
| [PLAN-014](PLAN-014.md) | Solar System Viewer: Astrodynamics Positioning & Semantic Zoom | `completed` | Antigravity | 2026-09-20 | 2026-09-20 | `main` | [PLAN-014.md](PLAN-014.md) |
| [PLAN-015](PLAN-015.md) | Solar System Ecliptic Orientation Widget and Astrobody Selector | `completed` | Antigravity | 2026-09-21 | 2026-09-21 | `main` | [PLAN-015.md](PLAN-015.md) |
| [PLAN-016](PLAN-016.md) | Pivot to Full 3D Solar System & Astrodynamics | `completed` | Antigravity | 2026-09-21 | 2026-09-21 | `main` | [PLAN-016.md](PLAN-016.md) |
| [PLAN-017](PLAN-017.md) | Native 6-Step Spec-Driven Development Pipeline | `completed` | Antigravity | 2026-09-21 | 2026-09-21 | `main` | [PLAN-017.md](PLAN-017.md) |
| [PLAN-018](PLAN-018.md) | Improve Camera Navigation UI | `completed` | Antigravity | 2026-09-22 | 2026-09-22 | `main` | [PLAN-018.md](PLAN-018.md) |
| [PLAN-019](PLAN-019.md) | Instanced Time-Step Dot Rendering for Orbital Paths | `completed` | Antigravity | 2026-09-23 | 2026-09-23 | `orbit-dots` | [PLAN-019.md](PLAN-019.md) |
| [PLAN-020](PLAN-020.md) | Simulation Time Warp Controls & Keyboard Shortcut Help Popup | `completed` | Antigravity | 2026-09-29 | 2026-09-29 | `main` | [PLAN-020.md](PLAN-020.md) |
| [PLAN-021](PLAN-021.md) | Fixed Geometric Orbit Dot Rendering | `completed` | Antigravity | 2026-09-30 | 2026-09-30 | `orbit-dots` | [PLAN-021.md](PLAN-021.md) |

---

## 📝 Plan Lifecycle & Conventions

All plans are stored directly in this directory (`.agents/docs/plans/PLAN-NNNN.md`) and **do not move** between active and archive directories. Their status is tracked directly in YAML frontmatter:

```yaml
---
id: PLAN-NNNN
title: "[Short, Descriptive Title]"
status: draft # draft | approved | in-progress | completed | superseded | abandoned
author: "[Author / Agent Name]"
created: YYYY-MM-DD
updated: YYYY-MM-DD
completed_at: YYYY-MM-DD # populated when completed
branch: "[branch-name]"
---
```

### Lifecycle Progression:
1. **`draft`:** Initialized by `/plan-human` and technical contracts completed by `/plan-robot`.
2. **`approved`:** Audited and approved by human developer.
3. **`in-progress`:** Picked up by `/execute` for atomic implementation.
4. **`completed`:** Passed the 7-Point QA verification in `/archive-plan` with retrospective notes.
5. **`superseded` / `abandoned`:** Replaced by newer architecture or deprioritized.
