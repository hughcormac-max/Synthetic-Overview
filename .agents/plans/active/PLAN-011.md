---
id: PLAN-011
title: "Transition SSOT to f64 Math & Unified 3D Coordinates"
status: in-progress
author: "Antigravity"
created: 2026-09-17
updated: 2026-09-17
branch: "main"
---

# PLAN-011: Transition SSOT to f64 Math & Unified 3D Coordinates

> **Status:** `in-progress` | **Created:** 2026-09-17 | **Last Updated:** 2026-09-17
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The current simulation relies on 64-bit integer fixed-point math to maintain deterministic ECS execution. However, this causes massive scale disparities when attempting to unify 2D interplanetary heliocentric coordinates with 3D planetary surface meshes. Squaring interplanetary distances at surface-level precision overflows 64-bit integers.

Following the success of the `Keplerian-Elements` reference architecture, we will transition the simulation kernel and astrodynamics engines to `f64` (64-bit IEEE-754 floating point) to enable a unified 3D coordinate space. This eliminates the coordinate shifting fudges across the Karman line while maintaining deterministic math via strict IEEE-754 WebAssembly / soft-float constraints.

### 1.2 Core Objectives
- Unify the interplanetary coordinate space and surface coordinate space into a single global `f64` 3D coordinate system.
- Transition all mathematical specifications from fixed-point (e.g. `1_000_000` multipliers, `sqrt_int64`, `cos_table`) to `f64` deterministic standard math operations.
- Enforce the "Zero-Inclination 2D Interplanetary" invariant: keep interplanetary orbits strictly coplanar (`z = 0`) to prevent the utility AI from needing to solve complex 3D Lambert problems.
- Maintain Patched Conics physics transitions at the Laplace Sphere of Influence.

### 1.3 Non-Goals & Exclusions
- Do not introduce N-body physics (Patched Conics boundary remains).
- Do not migrate any actual game code in this plan; this plan strictly covers amending the SSOT documentation and master architecture documents to reflect the new paradigms.

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-PHY-001: Astrodynamics & Topography](../../../docs/ssot/SSOT-PHY-001-Astrodynamics.md) — Needs updating to remove fixed-point limits and introduce f64.
- [SSOT-PHY-004: Surface Topography](../../../docs/ssot/SSOT-PHY-004-Surface-Topography.md) — Needs updating to remove Trap 2 (32-bit jitter) and permit unified 3D space with f64.
- [SSOT-SYS-001: Tick Kernel](../../../docs/ssot/SSOT-SYS-001-Tick-Kernel.md) — Update determinism constraints to enforce IEEE-754 exactness rather than integer-only logic.
- [SSOT-SYS-000: SI Units](../../../docs/ssot/SSOT-SYS-000-SI-Units.md) — Update base scalar types.

### 2.2 Domain Types & Schemas
```rust
// New f64-based State Vector
pub struct StateVector {
    pub x: f64, // meters
    pub y: f64, // meters
    pub z: f64, // meters
    pub vx: f64, // m/s
    pub vy: f64, // m/s
    pub vz: f64, // m/s
}
```

### 2.3 Layer Boundary Mapping
- **Documentation Layer:** `docs/ssot/*`
- **Master README/Architecture:** `README.md`, `docs/ARCHITECTURE.md`

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Amend SSOT-PHY-001 (Astrodynamics)**
  - [x] Rewrite formulas to use `f64` (remove `mul_fixed`, `TWO_PI_MICRO`, `sqrt_int64`).
  - [x] Add the "Zero-Inclination 2D Interplanetary" invariant to keep AI logistics coplanar while allowing 3D coordinates.
  - [x] Update Traps to explicitly enforce `f64` and ban `f32`.
- [x] **Step 2: Amend SSOT-PHY-004 (Surface Topography)**
  - [x] Remove Invariant 2 (Barycentric Isolation) and rewrite to support Unified 3D Coordinate Space.
  - [x] Update formulas to standard `f64` math.
  - [x] Remove Trap 2 and 3 concerning floating-point usage, replacing with rules on how to safely use `f64` deterministically.
- [x] **Step 3: Amend SSOT-SYS-000 and SSOT-SYS-001 (Kernel & Units)**
  - [x] Update SSOT-SYS-000 to define `f64` as the base standard for distance, mass, and time calculations.
  - [x] Update SSOT-SYS-001 to mandate strict IEEE-754 WebAssembly / soft-float execution to maintain cross-platform ECS determinism.
- [x] **Step 4: Update ARCHITECTURE.md and README.md**
  - [x] Update references of "fixed-point math" to "f64 IEEE-754 deterministic math".
  - [x] Refine the description of the 3D/2D hybrid space (3D coordinate space, 2D planar interplanetary orbits).

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [Target 1: All documents reflect the new f64 unified 3D architecture.]
- [Target 2: Fixed-point nomenclature (e.g. `_MICRO`, `fixed_mul`, `sqrt_int64`) is entirely purged from SSOT formulas.]
- [Target 3: Zero-LaTeX rule strictly maintained across all document edits.]

### 4.2 Verification Commands
```bash
# Agentic Review
ripgrep "fixed-point" docs/ssot/
ripgrep "mul_fixed" docs/ssot/
ripgrep "\\$" docs/ssot/
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- *[None logged during drafting. Update during/after implementation.]*

### 5.2 Lessons Learned & Follow-Up Tasks
- *[None logged during drafting. Update during/after implementation.]*

