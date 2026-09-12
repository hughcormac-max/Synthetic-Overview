---
id: PLAN-003
title: "Scaffold H3 Spatial Grid for Astronomical Bodies"
status: in-progress
author: Antigravity
created: 2026-09-12
updated: 2026-09-12
branch: main
---

# PLAN-003: Scaffold H3 Spatial Grid for Astronomical Bodies

> **Status:** `in-progress` | **Created:** 2026-09-12 | **Last Updated:** 2026-09-12
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The user wants to implement the surface-level geographic abstractions of astronomical bodies (planets, moons, asteroids) as outlined in the physical architecture. This allows entities to exist not just in the abstract network, but in a physical geographic location with spatial relations.

### 1.2 Core Objectives
- Integrate the `h3o` crate (pure Rust H3 implementation) into the `synthetic-core` domain layer.
- Define `AstroNode` component to represent distinct astronomical bodies (Earth, Luna, Mars).
- Define `SurfaceNode` ECS component to attach entities (like mines/refineries) to specific H3 cells on an `AstroNode`.
- Ensure strict adherence to the zero-floating-point domain rules.

### 1.3 Non-Goals & Exclusions
- Macro-scale Keplerian orbit integration (to be handled in a separate plan).
- Orbital components and tracking (these will be developed in the 2D solar scale layer).
- UI/Rendering logic for the H3 map (Tier 0 Domain only).
- Dynamic movement and pathfinding across H3 cells (only structural representation is in scope for this plan).

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-003: Physical Architecture & Interplanetary Logistics](../../../docs/ssot/SSOT-003-Physical-Architecture.md) — *Physical surface clustering.*

### 2.2 Domain Types & Schemas
```rust
use h3o::CellIndex;
use bevy_ecs::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct AstroNode {
    pub body_id: u32, // e.g., Earth, Mars
    pub radius_km: u32,
    pub h3_resolution: u8, // Base simulation resolution (e.g., 3 for Earth, 2 for Mars)
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct SurfaceNode {
    pub parent_body: Entity,
    pub cell: CellIndex,
}
```

### 2.3 Layer Boundary Mapping
- **Domain Layer:** `crates/synthetic-core/src/spatial/`
- **Application Layer:** N/A for this plan.
- **Infrastructure Layer:** N/A for this plan.

---

## 🛠️ 3. Implementation Steps

- [ ] **Step 1: Dependency Integration**
  - [ ] Add `h3o` crate as a dependency in `crates/synthetic-core/Cargo.toml`.
- [ ] **Step 2: Spatial Components Definitions**
  - [ ] Create `crates/synthetic-core/src/spatial/mod.rs` and `components.rs`.
  - [ ] Implement `AstroNode` and `SurfaceNode` components.
- [ ] **Step 3: State Initialization Tests**
  - [ ] Write hermetic unit tests in `crates/synthetic-core/src/spatial/components.rs` or `tests/` verifying entity attachment to H3 cells.
  - [ ] Prove entities can be queried by their `SurfaceNode`.
- [ ] **Step 4: Full Regression & Verification**
  - [ ] Run `cargo check` and `cargo test` to ensure zero regressions across the workspace.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- 100% unit test pass rate for all spatial components.
- Zero type errors.
- Entities can successfully compose `Storage`, `Converter`, and `SurfaceNode`.

### 4.3 Verification Commands
```bash
cargo check
cargo clippy -- -D warnings
cargo test
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)
- *[None logged during drafting. Update during/after implementation.]*
