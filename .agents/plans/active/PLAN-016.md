---
id: PLAN-016
title: "Pivot to Full 3D Solar System & Astrodynamics"
status: approved
author: "Antigravity"
created: 2026-09-21
updated: 2026-09-21
branch: "feature/3d-solar-system"
---

# PLAN-016: Pivot to Full 3D Solar System & Astrodynamics

> **Status:** `approved` | **Created:** 2026-09-21 | **Last Updated:** 2026-09-21
> **Author:** Antigravity | **Branch:** feature/3d-solar-system

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The solar system simulation currently relies on a 2D coplanar (ecliptic) lock to simplify orbital transfer calculations. However, obfuscating and abstracting inclined orbits complicates game logic design. Since orbital transfer pathfinding and calculations are infrequent (e.g., staggered across frames, calculated weekly), the performance constraint no longer necessitates a 2D simplification. The system must be updated to support full 3D Keplerian orbits (including inclination, longitude of the ascending node, and argument of periapsis) for all celestial bodies.

### 1.2 Core Objectives
- Refactor `AstroNodeConfig` and related models to support full 3D orbital elements.
- Update closed-form kinematic calculations in `synthetic-core` to resolve global positions in true 3D space.
- Modify rendering systems in `synthetic-client` to generate and display 3D orbital curves.
- Update existing data assets (`solar_system.json`) to include realistic 3D orbital parameters for all bodies.
- Remove 2D coplanar constraints from `SSOT-PHY-001` and `SSOT-PHY-004`.

### 1.3 Non-Goals & Exclusions
- Implementing the actual 3D Lambert or generalized 3D orbital transfer solvers is out of scope for this specific structural pivot; we are merely establishing the 3D domain space and definitions first.
- N-body gravitational integration remains explicitly prohibited (Invariant 2 of SSOT-PHY-001 stands).

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-PHY-001: 2D Keplerian Astrodynamics, Rocket Mechanics & Orbital Transfers](../../../docs/ssot/SSOT-PHY-001-Astrodynamics.md) — *Will be renamed and updated to remove 2D invariants and introduce 3D transformation matrices.*
- [SSOT-PHY-004: AstroNode Architecture & Spherical Fibonacci Surface Topography](../../../docs/ssot/SSOT-PHY-004-AstroNode-Architecture.md) — *Will be updated to remove 2D constraints.*

### 2.2 Domain Types & Schemas
```rust
// In crates/synthetic-core/src/astronomy/models.rs
pub struct AstroNodeConfig {
    pub name: String,
    pub parent_name: Option<String>,
    pub mass_kg: f64,
    pub radius_m: f64,
    pub semi_major_axis_m: f64,
    pub eccentricity: f64,
    pub inclination_rad: f64, // NEW
    pub longitude_of_ascending_node_rad: f64, // NEW
    pub argument_of_periapsis_rad: f64, // REPLACES longitude_of_periapsis_rad
    pub true_anomaly_epoch_rad: f64,
    pub mean_motion_rad_s: f64,
}
```

### 2.3 Public API / Service Signatures
```rust
// In crates/synthetic-client/src/systems/astronomy.rs
pub fn create_orbit_curve_mesh(
    semi_major_axis_m: f64,
    eccentricity: f64,
    inclination_rad: f64,
    longitude_of_ascending_node_rad: f64,
    argument_of_periapsis_rad: f64,
) -> Mesh;
```

### 2.4 Layer Boundary Mapping
- **Domain Layer:** `crates/synthetic-core/src/astronomy/...` (Models and Kinematics)
- **Presentation Layer:** `crates/synthetic-client/src/systems/astronomy.rs` (Rendering 3D Orbits)
- **Data Layer:** `assets/data/solar_system.json` (Configuration)

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: SSOT Documentation Updates**
  - [x] Modify `SSOT-PHY-001` and `SSOT-PHY-004` to eliminate the 2D coplanar constraints.
  - [x] Document the 3D Keplerian positional formulas.
- [x] **Step 2: Core Domain Logic & Data Types**
  - [x] Update `AstroNodeConfig` in `crates/synthetic-core/src/astronomy/models.rs`.
  - [x] Update `calculate_kepler_position` in `crates/synthetic-core/src/astronomy/kinematics.rs` to apply 3D transformation matrices.
  - [x] Update kinematic unit tests to assert 3D positional calculations and adapt existing 2D tests.
- [x] **Step 3: Presentation Integration**
  - [x] Update `create_orbit_curve_mesh` and its usages in `crates/synthetic-client/src/systems/astronomy.rs`.
  - [x] Update rendering tests.
- [x] **Step 4: Dataset Migration**
  - [x] Migrate `assets/data/solar_system.json` to replace `longitude_of_periapsis_rad` with `argument_of_periapsis_rad` and introduce `inclination_rad: 0.0` and `longitude_of_ascending_node_rad: 0.0`.
- [x] **Step 5: Full Regression & Verification**
  - [x] Execute project typecheck (`cargo check`).
  - [x] Execute full unit test suite (`cargo test`).

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [Target 1: 100% unit test pass rate across `synthetic-core` and `synthetic-client`.]
- [Target 2: Complete compilation with zero warnings (`cargo clippy`).]

### 4.2 Unit Test Targets
| Module / File | Test Module | Key Scenarios Covered |
| :--- | :--- | :--- |
| `synthetic-core/.../kinematics.rs` | `tests` module | Verify 3D transformation logic given a non-zero inclination |
| `synthetic-client/.../astronomy.rs` | `tests` module | Verify orbital curve mesh generation in 3D |

### 4.3 Verification Commands
```bash
# Typecheck & Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test Suite
cargo test --all
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- *[None logged during drafting. Update during/after implementation.]*

### 5.2 Lessons Learned & Follow-Up Tasks
- *[None logged during drafting. Update during/after implementation.]*

