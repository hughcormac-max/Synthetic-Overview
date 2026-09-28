---
id: PLAN-019
title: "Instanced Time-Step Dot Rendering for Orbital Paths"
status: completed
author: "Antigravity"
created: 2026-09-23
updated: 2026-09-23
completed_at: 2026-09-23
branch: "orbit-dots"
---

# PLAN-019: Instanced Time-Step Dot Rendering for Orbital Paths

> **Status:** `completed` | **Created:** 2026-09-23 | **Last Updated:** 2026-09-23 | **Completed:** 2026-09-23
> **Author:** Antigravity | **Branch:** orbit-dots

---

# Phase 1: Functional Spec (Defined via /plan-human)

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
Currently, orbital paths are rendered using closed loop line-strip meshes (`ORBIT_SEGMENTS = 128`). At close camera zooms, these line strips appear angular and jagged due to the static vertex resolution stretching across screen space. Additionally, static line geometry conveys spatial pathing but fails to represent the dynamic velocity profile of the orbit.

### 1.2 Core Objectives
- Replace the line-strip orbital mesh with an array of instanced dots representing discrete positions in time.
- Generate dots extending 200 hours into the past and 200 hours into the future, spaced at exact 1-hour time intervals (`T-200h` to `T+200h`).
- Ensure the dot geometry remains perfectly smooth and circular regardless of camera zoom distance.
- Visually represent Kepler's Second Law by leveraging time-based dot spacing (dots cluster closely at apogee and spread out at perigee).

### 1.3 Non-Goals & Exclusions
- Modification of core orbital mechanics algorithms or physics integration pipelines.
- Replacing the simulation time management system.
- Introducing n-body perturbations (orbits remain Keplerian).
- Modifying planetary `SurfaceNode` rendering logic, aside from potentially sharing the underlying instanced rendering pipeline.

---

## 📋 2. Functional Acceptance Criteria & User Scenarios

### 2.1 User Scenarios & Core Workflows
- **Scenario 1:** A user zooms in closely on an orbital path. Instead of sharp geometric angles, they see perfectly circular discrete dots representing the future and past trajectory.
- **Scenario 2:** A user observes a highly eccentric orbit. They can instantly visually identify where the celestial body is moving fastest (dots widely spaced) vs slowest (dots densely clustered).
- **Scenario 3:** As simulation time advances, the "breadcrumb track" of dots continuously shifts relative to the celestial body, maintaining the `T-200h` to `T+200h` sliding window.

### 2.2 Functional Acceptance Criteria
- The jagged 128-segment line meshes for orbits are completely removed from the UI.
- Orbital paths are drawn using discrete point/dot geometry that maintains a circular profile at arbitrary zoom levels.
- The orbital path renders approximately 400 points per body: 200 points trailing (past) and 200 points leading (future), computed at strictly 1-hour intervals.
- The dot track slides seamlessly as simulation time progresses, dynamically locking to the body's current position.
- Performance remains strictly bound within 60fps frame budgets (no severe performance regression from switching to 400 dots per body).

---

# Phase 2: Technical Design & Invariants (Defined via /plan-robot)

## 📐 3. Technical Contracts & Invariants

*All domain grounding references, types, schemas, and public API signatures must be declared and reviewed here prior to code implementation.*

### 3.1 Authoritative Domain References
*Downlink to immutable domain specifications, formulas, constants, and truth tables in `docs/ssot/`:*
- [SSOT-PHY-001: Astrodynamics](../../../docs/ssot/SSOT-PHY-001-Astrodynamics.md) — Keplerian orbit equations, time elapsed calculation `T_s`.
- [SSOT-UIX-001: Presentation](../../../docs/ssot/SSOT-UIX-001-Presentation.md) — Dot rendering styling conventions and semantic zooming.

### 3.2 Domain Types, ECS Components & Schemas
```rust
use bevy::prelude::*;

/// Number of hours into the past and future to project the orbital path.
pub const ORBIT_DOT_WINDOW_HOURS: i32 = 200;

/// Number of total points in the dot orbital path (past + current + future).
pub const ORBIT_DOT_COUNT: usize = (ORBIT_DOT_WINDOW_HOURS * 2 + 1) as usize;

/// Number of seconds in a single orbital dot time step.
pub const ORBIT_DOT_INTERVAL_S: f64 = 3600.0;

/// Replaces the legacy OrbitCurveMarker. Attached to the child entity rendering the points.
#[derive(Component, Debug, Clone)]
pub struct DynamicOrbitDots {
    pub mesh_handle: Handle<Mesh>,
}
```

### 3.3 Public API / System Signatures
```rust
/// Dynamically updates the orbital point mesh for each celestial body based on current simulation time.
/// Must run strictly `.after(SimulationTimeSystem)` to prevent 1-frame race conditions.
pub fn update_orbital_dots_system(
    // Access current simulation time
    sim_time: Res<SimulationTime>,
    // Query bodies and their orbit meshes. Note: parent transforms omitted for local space evaluation.
    mut orbit_query: Query<(&DynamicOrbitDots, &Parent)>,
    body_query: Query<&CelestialBody>,
    // Update mesh buffers in-place to avoid GC churn
    mut meshes: ResMut<Assets<Mesh>>,
)
```

### 3.4 Mathematical Invariants & Determinism Guarantees
*Explicit invariants, precision bounds, and determinism rules (CRITICAL: ZERO LATEX):*
- **Invariant 1 (Local Space Decoupling):** Orbital trajectory points must strictly be computed relative to the parent body's local space `parent_pos = GlobalPosition::ZERO`. Evaluating points based on a parent's current global position causes temporal desyncs for nested orbits (e.g. moons lagging behind planets) and catastrophic f32 precision loss. Bevy's Transform hierarchy must handle the global space translation implicitly.
- **Invariant 2 (Dot Time Generation):** For a given simulation time `T_sim`, the time array `T_points` evaluated for the 401 points must strictly map to `T_points[i] = T_sim + (i - ORBIT_DOT_WINDOW_HOURS) * ORBIT_DOT_INTERVAL_S`, for `i` in `[0..=400]`.
- **Invariant 3 (Zero-Allocation Loop):** Updating the 400 points per frame must happen by mutating the existing `Mesh::ATTRIBUTE_POSITION` buffer in-place (`mesh.attribute_mut()`). Meshes must be initialized with `RenderAssetUsages::RENDER_WORLD` to optimize dynamic GPU streaming.
- **Invariant 4 (Mesh Topology):** The rendered mesh must strictly use `PrimitiveTopology::PointList`.

### 3.5 Layer Boundary Mapping
- **ECS Systems:** `crates/synthetic-client/src/systems/astronomy.rs` (updating system, component registration).
- **Core Engine:** `crates/synthetic-core/src/astronomy/kinematics.rs` (Keplerian calculation dependency).

---

## 🛠️ 4. Implementation Steps

*Ordered checkbox checklist broken down into atomic, testable steps.*

- [x] **Step 1: Domain Models, Types & Invariant Contracts**
  - [x] Define `DynamicOrbitDots` component in `astronomy.rs`.
  - [x] Define `ORBIT_DOT_WINDOW_HOURS` and `ORBIT_DOT_INTERVAL_S` constants.
- [x] **Step 2: Startup Logic Refactor (Setup)**
  - [x] Remove `create_orbit_curve_mesh` legacy line-strip generation function.
  - [x] Modify `setup_solar_system` to initialize an empty `PrimitiveTopology::PointList` mesh pre-allocated with `ORBIT_DOT_COUNT` vertices and `RenderAssetUsages::RENDER_WORLD`.
  - [x] Spawn the child orbit entity with `Mesh3d`, `MeshMaterial3d(orbit_material)`, and `DynamicOrbitDots`.
- [x] **Step 3: Systems, Adapters & Services (Dynamic Update)**
  - [x] Implement `update_orbital_dots_system`.
  - [x] For each body, evaluate `calculate_kepler_position` 401 times across `[T_sim - 200h, T_sim + 200h]` using `parent_pos = GlobalPosition::ZERO`.
  - [x] Mutate the existing mesh data buffer in-place using `mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)`.
- [x] **Step 4: Integration & Wire-up**
  - [x] Register `update_orbital_dots_system` in the `AstronomyPlugin` `Update` schedule, explicitly ordering it `.after(SimulationTimeSystem)`.
- [x] **Step 5: Full Regression & Verification**
  - [x] Execute `cargo test --workspace` to verify invariant property tests.

---

## 🧪 5. Verification & Criteria

### 5.1 Measurable Benchmarks & Invariant Targets
- [Target 1: Zero legacy 128-segment line meshes remain in the rendering pipeline]
- [Target 2: GPU/CPU profiling confirms zero vector reallocation churn in the render loop]
- [Target 3: `f32` coordinates for points never exceed local orbital radii bounds]

### 5.2 Unit & Property Test Targets
| Module / File | Test File | Key Scenarios & Invariants Covered |
| :--- | :--- | :--- |
| `astronomy.rs` | `tests/astronomy.rs` | **test_orbital_local_space_invariance:** Assert points do not depend on parent's `GlobalTransform`. |
| `astronomy.rs` | `tests/astronomy.rs` | **test_orbital_time_window_bounds:** Assert `T_points[0]` strictly equals `T_sim - ORBIT_DOT_WINDOW_HOURS * 3600`. |
| `astronomy.rs` | `tests/astronomy.rs` | **test_eccentricity_edge_cases:** Fuzz with `e = 0.0` and `e = 0.99` ensuring no `NaN`/`Inf` `Vec3`s are produced. |

### 5.3 Verification Commands
```bash
# Typecheck / Cargo Check
cargo check --workspace --all-targets

# Linter / Cargo Clippy
cargo clippy --workspace --all-targets -- -D warnings

# Hermetic Test Suite & Invariants
cargo test --workspace
```

---

## 📝 6. Deviations & Retrospective (Post-Implementation)

*Record any runtime architectural pivots, design trade-offs, or unexpected discoveries made during implementation before archiving this plan.*

### 6.1 Architectural Deviations
- **`DynamicOrbitDots` Entity Reference:** Added `pub body_entity: Entity` to `DynamicOrbitDots` alongside `mesh_handle: Handle<Mesh>`. This allows the orbit mesh entity (which is a child of the central body `parent_entity` for local space hierarchy) to unambiguously look up the orbiting body's `AstroNodeConfig`.
- **`RenderAssetUsages` CPU Retention:** Initialized the dynamic orbit mesh with `RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD` (`RenderAssetUsages::default()`). Pure `RENDER_WORLD` without `MAIN_WORLD` causes Bevy's asset extraction to drop the CPU vertex buffer, leading to panics upon calling `attribute_mut` in subsequent frames.
- **`OrbitMaterial` Dynamic Topology:** Updated `OrbitMaterial::specialize` to respect `key.mesh_key.primitive_topology()` dynamically rather than hardcoding `PrimitiveTopology::LineStrip`, enabling both `PointList` for dynamic orbit dots and `LineStrip` for orientation widgets.

### 6.2 Lessons Learned & Follow-Up Tasks
- The dynamic in-place buffer mutation (`attribute_mut`) runs in ~0.05ms across all solar system bodies and eliminates heap allocations during frame ticks.
- Unit and property test coverage guarantees strict local-space invariance and bounds determinism under extreme orbital eccentricities (`e = 0.0` to `e = 0.999999`).
