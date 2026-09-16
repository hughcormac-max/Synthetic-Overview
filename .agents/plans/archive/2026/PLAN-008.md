---
id: PLAN-008
title: "Re-architect SSOT for SI Units, Astrodynamics, and Topography"
status: completed
author: "Antigravity"
created: 2026-09-13
updated: 2026-09-16
completed_at: 2026-09-16
branch: "main"
---

# PLAN-008: Re-architect SSOT for SI Units, Astrodynamics, and Topography

> **Status:** `completed` | **Created:** 2026-09-13 | **Completed:** 2026-09-16
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The project currently lacks a universal grounding document for its units of measurement, which can lead to inconsistencies in formulas across the SSOT library. Additionally, `SSOT-PHY-001-Astrodynamics.md` currently bundles two distinct domains: 2D interplanetary astrodynamics and 3D surface topography (Fibonacci Spheres). This coupling complicates the logic loops and creates context fragmentation.

### 1.2 Core Objectives
- Create a new authoritative Tier 0 SSOT document (`SSOT-SYS-000-SI-Units.md`) to establish the baseline SI units (m, s, m/s, kg, etc.) for the entire project. All other SSOTs will cite this document for physical and temporal constants.
- Split `SSOT-PHY-001-Astrodynamics.md` into two separate, focused specifications:
  - **`SSOT-PHY-001-Astrodynamics.md` (Refactored):** Will strictly govern 2D top-down system math, location tracking, orbital mechanics, Keplerian trajectories, and be the source of truth for thrust, delta-V, the rocket equation, and Sphere of Influence (SOI) transfers.
  - **`SSOT-PHY-004-Surface-Topography.md` (New):** Will strictly govern the 3D local planetary sphere math, including the Fibonacci spiral lattice, KD-Tree indexing, Karman line boundaries, and coordinate space transformations on planetary surfaces.
- Update `INDEX.md` to reflect the new file hierarchy and dependency graph.

### 1.3 Non-Goals & Exclusions
- This plan *does not* implement any Rust or TypeScript game code.
- This plan *does not* touch socio-economic or cyberwarfare SSOTs, except where unit consistency checks might be required in the future (deferred).

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-SYS-000-SI-Units.md](../../../docs/ssot/SSOT-SYS-000-SI-Units.md) — *Will serve as the master reference for base units (m, kg, s), derived units (N, J, W, m/s, m/s^2), scaling factors, and integer/fixed-point standard conversions.*
- [SSOT-PHY-001-Astrodynamics.md](../../../docs/ssot/SSOT-PHY-001-Astrodynamics.md) — *Keplerian mechanics, Tsiolkovsky rocket equation (`dv = v_e * ln(m0/mf)` represented without LaTeX), and coplanar 2D vectors.*
- [SSOT-PHY-004-Surface-Topography.md](../../../docs/ssot/SSOT-PHY-004-Surface-Topography.md) — *Golden ratio math, spherical surface distribution.*

### 2.2 Domain Contracts & Formulas (Zero-LaTeX)
- **Rocket Equation (Tsiolkovsky):** `delta_v = v_e * ln(m_initial / m_final)`
- **Gravitational Parameter:** `mu = G * M`
- **Orbital Velocity:** `v = sqrt(mu * (2/r - 1/a))`
- **Unit Scale (SI):** Distance primarily evaluated in meters (`m`) or kilometers (`km`) with clear fixed-point integer conversions, time in seconds (`s`), and mass in kilograms (`kg`).

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Draft SI Units Root Document**
  - [x] Write `SSOT-SYS-000-SI-Units.md` detailing SI base units, derived units, and scaling conventions (e.g., fixed-point representations for `m`, `kg`, `s`).
- [x] **Step 2: Isolate and Refactor Astrodynamics**
  - [x] Refactor `SSOT-PHY-001-Astrodynamics.md`. Remove all Fibonacci sphere logic.
  - [x] Add explicit math for thrust, delta-V, Tsiolkovsky rocket equation, and SOI boundary formulas.
- [x] **Step 3: Extract Surface Topography**
  - [x] Create `SSOT-PHY-004-Surface-Topography.md`.
  - [x] Move Fibonacci spiral lattice algorithms, Z-coordinate fraction formulas, KD-Tree resolution tables, and surface node distributions into this file.
- [x] **Step 4: Dependency Graph & Index Update**
  - [x] Update `docs/ssot/INDEX.md` to map `SSOT-SYS-000` as the root dependency for `SSOT-SYS-001` and `SSOT-PHY-001`.
  - [x] Register `SSOT-PHY-004` and update the visual cross-reference graph.
- [x] **Step 5: Zero-LaTeX and Invariant Verification**
  - [x] Verify that all equations in the new/modified documents strictly use ASCII plain text formats (e.g., no `$...$`).
  - [x] Ensure all 3 documents follow the standard `define-ssot` schema.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [x] Target 1: Zero LaTeX macros across all generated files.
- [x] Target 2: Astrodynamics contains explicit delta-V and rocket equation math without overlapping with 3D terrain logic.
- [x] Target 3: Topography contains explicit Fibonacci logic without overlapping with interplanetary orbits.
- [x] Target 4: `INDEX.md` successfully integrates `SSOT-SYS-000-SI-Units.md` at the top of the tree.

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- **Strict SI Grounding Root:** Introduced `SSOT-SYS-000-SI-Units.md` at the very apex of the SSOT dependency hierarchy, explicitly binding discrete simulation tick rates (`1 tick = 60s`) and fixed-point representations (`1e6` scale) to SI base and derived dimensions (`[L]`, `[M]`, `[T]`, `[I]`, `[Theta]`, `[N]`, `[J]`).
- **Clean Astrodynamics / Topography Decoupling:** Successfully isolated 2D interplanetary Keplerian orbits, Laplace SOI patched conics, and Tsiolkovsky delta-V mechanics into `SSOT-PHY-001`, and planetary 3D spherical Fibonacci lattice geometry and KD-Tree site queries into `SSOT-PHY-004`.

### 5.2 Lessons Learned & Follow-Up Tasks
- Citing `SSOT-SYS-000` down into `SSOT-PHY-001` and `SSOT-PHY-004` prevents conversion ambiguity (such as confusing specific impulse in seconds with effective exhaust velocity in m/s).
- All new files passed zero-LaTeX validation audits with zero math delimiters.

