---
id: PLAN-012
title: "Scaffold Initial Solar System & Surface Nodes"
status: completed
author: "Antigravity"
created: 2026-09-20
updated: 2026-09-20
completed_at: 2026-09-20
branch: "main"
---

# PLAN-012: Scaffold Initial Solar System & Surface Nodes

> **Status:** `completed` | **Created:** 2026-09-20 | **Completed:** 2026-09-20
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The game requires a deterministic representation of celestial bodies (`AstroNodes`) and their interactable surfaces (`SurfaceNodes`). Currently, the core domain engine is mostly empty. We need to define the data structures for astronomical bodies, a method to populate them from static configuration, and the procedural generation logic for their surface nodes using Poisson sphere sampling.

### 1.2 Core Objectives
- Define the Rust core domain models for `AstroNode` and `SurfaceNode` in `synthetic-core`.
- Implement a deterministic Poisson sphere sampling algorithm to generate `SurfaceNode` coordinates based on planetary radii.
- Create a static TOML/JSON configuration file containing the orbital elements and physical properties for the Sun, 8 major planets, Luna, and major moons (Titan, Europa, Ganymede, etc.).
- Scaffold the loading mechanism in the application layer (`synthetic-client`) to deserialize the config and insert these bodies into the Bevy ECS.

### 1.3 Non-Goals & Exclusions
- Do not implement actual in-game rendering of the planets in this plan (this relies on the data layer being built first).
- Do not implement dynamic orbital motion (time-stepping) yet; just load the static initial definitions and their epoch states.
- Do not add minor asteroids or comets yet, stick to the major bodies defined in the objective.

### 1.4 Simulation Invariants & Time Basis
- **Time Step Invariant:** For current development, time steps are fixed at 1 tick = 1.0 second (`delta_t = 1.0 s`).
- **Canonical Units:** Strictly canonical SI units across all models (`m`, `kg`, `s`, `rad`, `rad/s`).

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-PHY-001: Astrodynamics & Topography](../../../docs/ssot/SSOT-PHY-001-Astrodynamics.md) — Constants and Keplerian formulas.
- [SSOT-PHY-004: AstroNode Architecture](../../../docs/ssot/SSOT-PHY-004-AstroNode-Architecture.md) — AstroNode definitions and Poisson sphere distribution invariant.
- [SSOT-SYS-000: SI Units](../../../docs/ssot/SSOT-SYS-000-SI-Units.md) — f64 Determinism and SI baseline types.

### 2.2 Domain Types & Schemas
```rust
// synthetic-core/src/astronomy/models.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstroNodeConfig {
    pub name: String,
    pub parent_name: Option<String>,
    pub mass_kg: f64,
    pub radius_m: f64,
    pub semi_major_axis_m: f64,
    pub eccentricity: f64,
    pub true_anomaly_epoch_rad: f64,
    pub longitude_of_periapsis_rad: f64,
    pub mean_motion_rad_s: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct SurfaceNode {
    pub local_x: f64,
    pub local_y: f64,
    pub local_z: f64,
}

#[derive(Debug, Clone)]
pub struct AstroNode {
    pub config: AstroNodeConfig,
    pub surface_nodes: Vec<SurfaceNode>,
}
```

### 2.3 Public API / Service Signatures
```rust
// synthetic-core/src/astronomy/topography.rs
/// Generates Poisson sphere nodes for an AstroNode based on its radius and global density factor.
pub fn generate_poisson_surface_nodes(radius_m: f64, k_density: f64) -> Vec<SurfaceNode>;
```

### 2.4 Layer Boundary Mapping
- **Domain Layer:** `crates/synthetic-core/src/astronomy/` (Pure math, structs, Poisson generation)
- **Application/Infrastructure Layer:** `crates/synthetic-client/src/systems/` (Loading config, Bevy ECS initialization)
- **Configuration:** `assets/data/solar_system.toml` (or JSON)

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Define Core Models**
  - [x] Create `synthetic-core/src/astronomy/models.rs`.
  - [x] Define `AstroNodeConfig`, `SurfaceNode`, and `AstroNode` types with `Serialize`/`Deserialize` traits.
  - [x] Assert: Run `cargo check -p synthetic-core` to ensure types compile.

- [x] **Step 2: Create Static Solar System Config**
  - [x] Create `assets/data/solar_system.json`.
  - [x] Populate with baseline data for Sun, 8 planets, Luna, and major moons using canonical SI units (meters, kilograms, radians).
  - [x] Assert: Verify JSON syntax validity.

- [x] **Step 3: Implement Poisson Sphere Generator**
  - [x] Create `synthetic-core/src/astronomy/topography.rs`.
  - [x] Implement deterministic `generate_poisson_surface_nodes(radius_m: f64, k_density: f64) -> Vec<SurfaceNode>`.
  - [x] Assert: Write unit tests in `topography.rs` checking deterministic point counts and minimum separation distance.
  - [x] Assert: Run `cargo test -p synthetic-core`.

- [x] **Step 4: Implement Config Loader**
  - [x] Create `synthetic-client/src/systems/astronomy.rs`.
  - [x] Implement parsing logic to deserialize `solar_system.json` into a `Vec<AstroNodeConfig>`.
  - [x] Assert: Run `cargo check -p synthetic-client` and add a basic JSON parse test.

- [x] **Step 5: Bevy ECS Integration**
  - [x] Implement a Bevy startup system in `synthetic-client` to ingest the parsed configs.
  - [x] Iterate each body, generate surface nodes, and spawn Bevy entities for each.
  - [x] Assert: Run `cargo check -p synthetic-client` to verify ECS component registration and spawning.

- [x] **Step 6: Full Regression & Verification**
  - [x] Run `cargo check --workspace`, `cargo test --workspace`, and `cargo clippy`.
  - [x] Confirm no LaTeX usage in documentation or comments.
  - [x] Confirm strict downward dependency (client depends on core, core has no I/O).

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [x] Target 1: The solar system config successfully parses into the ECS.
- [x] Target 2: Poisson generation yields a deterministic number of points proportional to the body radius.
- [x] Target 3: Zero float variance (no hardware transcendentals used if soft-float is strictly enforced, or isolated properly).

### 4.2 Verification Commands
```bash
cargo check
cargo test
cargo clippy
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- None. Implementation strictly adheres to technical contracts in Section 2, SSOT-PHY-004, and SSOT-SYS-000.

### 5.2 Lessons Learned & Follow-Up Tasks
- The Poisson sphere topography generator implements uniform Marsaglia spherical projection and a deterministic XorShift64 pseudo-random generator with fixed seed hashing. This ensures identical surface node locations across runs and platforms without external crate dependencies.
- Follow-up work will wire these nodes to the 3D globe rendering pipeline and implement closed-form 2D Keplerian orbit motion in the ECS update loop.

