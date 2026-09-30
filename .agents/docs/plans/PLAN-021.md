---
id: PLAN-021
title: "Fixed Geometric Orbit Dot Rendering"
status: completed
author: "Antigravity"
created: 2026-09-30
updated: 2026-09-30
completed_at: 2026-09-30
branch: "orbit-dots"
---

# PLAN-021: Fixed Geometric Orbit Dot Rendering

> **Status:** `completed` | **Created:** 2026-09-30 | **Last Updated:** 2026-09-30 | **Completed:** 2026-09-30
> **Author:** Antigravity | **Branch:** orbit-dots
> **Permanent Location:** `.agents/docs/plans/PLAN-021.md`

---

# 🧑‍💻 PART 1: HUMAN PROBLEM ALIGNMENT
*(Authored via `/plan-human` — Aligns user-facing intent, boundaries, and acceptance criteria prior to technical design)*

## 🎯 1. Problem Statement & Context
The current orbit renderer relies on a time-based solver projecting points +/- 200 hours into the future/past. Attempting to render a complete orbital path using time-based spacing across the entire orbit is computationally prohibitive or visually jagged due to Kepler's Second Law. The goal is to completely scrap the existing 200hr point calculation logic and replace it with a system that renders a complete, visually smooth, and statically fixed orbit composed of 360 discrete dots using pure geometric parametric rendering.

## 🚀 2. Core Objectives & Capabilities
- **Scrap Legacy Forecasting:** Remove the existing time-based +/- 200 hours point rendering logic entirely.
- **Geometric Orbit Coverage:** Render exactly 360 evenly spaced points covering 100% of the orbital path.
- **Pure Geometric Rendering (Solver Bypass):** Generate these points by linearly sweeping Eccentric Anomaly (`E`) from 0 to 2*PI. Sweeping `E` (as opposed to True Anomaly) maps to the circumscribing auxiliary circle, guaranteeing a much more visually pleasing and spatially even distribution of dots on the physical ellipse, and completely bypasses any physics time-solving.
- **Statically Fixed Points:** Ensure that the generated points are entirely static and fixed in space for the given orbit, rather than crawling along the path over time.

## 🚫 3. Non-Goals & Exclusions
- **Line/Mesh Rendering:** We are strictly sticking to dot-based rendering for now. No continuous lines or meshes will be drawn between these points.
- **Spacecraft Forecasting:** Rendering dynamic spacecraft maneuvers or handling open trajectories (hyperbolas) will be deferred entirely to a future plan.

## 👤 4. User Stories & Interaction Journeys
- **As a** player/user viewing the solar system map
- **I want to** see 360 perfectly distributed dots mapping out an object's entire orbit
- **So that** I can easily visualize the full static geometric path without stuttering the simulation.

## ✅ 5. Acceptance Criteria
- [ ] 360 individual point entities are rendered covering the full 360 degrees of closed orbits.
- [ ] The points are geometrically evenly spaced (by sweeping Eccentric Anomaly `E`).
- [ ] The generated points are statically fixed in space for the given orbit; they do not move or crawl along the path over time.
- [ ] No continuous line meshes are drawn between these points.
- [ ] The game runs smoothly with no framerate drops when instantiating these orbits.
- [ ] The legacy +/- 200 hours time-based point calculation logic is successfully removed.

## ⚠️ 6. Edge Cases & Boundary Behaviors
- **Hyperbolic Orbits (e >= 1.0):** The generator must detect `e >= 1.0` and gracefully abort mesh generation (returning `None`) rather than generating mathematical NaNs.
- **Perfect Circular Orbits (e = 0.0) / Equatorial Orbits (i = 0.0):** The 3D matrix math must securely handle collapsed angles without resulting in NaN projection vectors.

---

# 🤖 PART 2: ROBOT TECHNICAL SPECIFICATIONS
*(Authored via `/plan-robot` — Defines contracts, interfaces, and <= ~50 line atomic implementation tasks)*

## 📐 7. Authoritative Domain References (Tier 0)
*   **SSOT-PHY-001 (Astrodynamics)**: Pure domain kinematics bounding coplanar and 3D rotations, enforcing zero upward references and zero I/O.

## 🧩 8. Domain Types & Data Contracts
```rust
// synthetic-client/src/systems/astronomy.rs
pub const ORBIT_DOT_COUNT: usize = 360;

// DynamicOrbitDots and ORBIT_DOT_WINDOW_HOURS will be DELETED.
```

## 🔌 9. Public API / Service Signatures
```rust
// synthetic-core/src/astronomy/kinematics.rs
pub fn calculate_position_from_eccentric_anomaly(
    config: &AstroNodeConfig,
    parent_pos: GlobalPosition,
    eccentric_anomaly: f64,
) -> GlobalPosition;

// synthetic-client/src/systems/astronomy.rs
pub fn generate_static_orbit_mesh(config: &AstroNodeConfig) -> Option<(Mesh, Aabb)>;
```

## 🗺️ 10. File-by-File Module Mapping
- **Domain Layer:** `crates/synthetic-core/src/astronomy/kinematics.rs` (Math extraction)
- **Application Layer:** `crates/synthetic-client/src/systems/astronomy.rs` (Setup and mesh generation)
- **Plugin Layer:** `crates/synthetic-client/src/plugins/astronomy.rs` (Remove system registration)

## 🛠️ 11. Atomic Implementation Steps (STRICT <= ~50 LINES RULE)
- [x] **Step 1: Extract Core Geometry Math** *(Target: `synthetic-core/src/astronomy/kinematics.rs` ~45 lines)*
  - [x] Extract the 3D rotation logic from `calculate_kepler_position` into a new `calculate_position_from_eccentric_anomaly` function.
  - [x] Update `calculate_kepler_position` to solve for `E` and then call the new function to avoid duplication.
- [x] **Step 2: Core Math Unit Tests** *(Target: `synthetic-core/src/astronomy/kinematics.rs` ~40 lines)*
  - [x] Add hermetic tests for `calculate_position_from_eccentric_anomaly` verifying valid 3D coordinates given 0, PI, 2PI, PI/2, and 3PI/2.
  - [x] Verify `e=0` and `i=0` securely resolve without NaN.
- [x] **Step 3: Cleanup Legacy Forecasting Logic** *(Target: `synthetic-client/src/systems/astronomy.rs` ~50 lines)*
  - [x] Delete `ORBIT_DOT_WINDOW_HOURS` and `DynamicOrbitDots`.
  - [x] Delete the `update_orbital_dots_system` entirely.
  - [x] Change `ORBIT_DOT_COUNT` to 360.
- [x] **Step 4: Implement Static Mesh Generator** *(Target: `synthetic-client/src/systems/astronomy.rs` ~40 lines)*
  - [x] Implement `generate_static_orbit_mesh(config) -> Option<(Mesh, Aabb)>`. 
  - [x] If `config.eccentricity >= 1.0`, immediately return `None`.
  - [x] Iterate `0..360`, calculating `E = i * 2PI / 360`, calls `calculate_position_from_eccentric_anomaly(config, ZERO, E)`, and constructs a `PointList` mesh and an `Aabb`.
- [x] **Step 5: Integrate Static Generator into Setup** *(Target: `synthetic-client/src/systems/astronomy.rs` ~30 lines)*
  - [x] In `setup_solar_system`, call `generate_static_orbit_mesh`. 
  - [x] If it returns `Some((mesh, aabb))`, spawn the child orbit entity with `(Mesh3d, MeshMaterial3d(orbit_material), aabb, VisibilityBundle::default(), TransformBundle::default())` to ensure frustum culling functions properly.
- [x] **Step 6: Update Client Tests** *(Target: `synthetic-client/src/systems/astronomy.rs` ~50 lines)*
  - [x] Delete obsolete dynamic time-window tests (`test_orbital_time_window_bounds`, `test_dynamic_orbital_dots_aabb_update`).
  - [x] Add `test_generate_static_orbit_mesh` to verify the mesh has exactly 360 vertices, a correctly sized AABB, and correctly returns `None` for `e=1.5`.
- [x] **Step 7: Full Regression & Verification**
  - [x] Run `cargo check --workspace --all-targets`
  - [x] Run `cargo clippy --workspace --all-targets -- -D warnings`
  - [x] Run `cargo test --workspace`

## 🧪 12. Unit Test Targets & Verification Commands
```bash
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

---

# 📋 PART 3: VERIFICATION & RETROSPECTIVE
*(Updated during execution and closed via `/archive-plan`)*

## 📝 13. Runtime Deviations & Architectural Pivots
- **Eccentric vs True Anomaly Pivot:** During the planning phase, an adversarial review proved that sweeping True Anomaly (`nu`) creates severe spatial clustering at periapsis due to massive arc lengths on highly eccentric orbits. We pivoted back to sweeping Eccentric Anomaly (`E`), which maps to the circumscribing auxiliary circle and produces a much more visually pleasing, evenly distributed geometric ring.
- **Dynamic to Static Refactor:** We completely decoupled orbital paths from the time solver. Instead of calculating dots +/- 200 hours every frame in a system, we now generate a static 360-dot `PointList` mesh exactly once during `setup_solar_system`.

## 💡 14. Retrospective & Follow-Up Items
- **Spacecraft Forecasting:** By ripping out the time-based solver, planets now render beautifully at zero CPU cost per frame. However, future dynamic spacecraft maneuvering, SOI transitions, and open trajectory intercepts will require a totally separate time-based forecasting architecture (which was deliberately deferred from this plan).
- **Frustum Culling Safety:** Discovered that manually generating an `Aabb` is useless for Bevy's frustum culling unless the entity is also spawned with a complete spatial hierarchy (`VisibilityBundle` and `TransformBundle`). This was successfully integrated into the static mesh generator.
