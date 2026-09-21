---
id: PLAN-014
title: "Solar System Viewer: Astrodynamics Positioning & Semantic Zoom"
status: completed
author: "Antigravity"
created: 2026-09-20
updated: 2026-09-20
completed_at: 2026-09-20
branch: "main"
---

# PLAN-014: Solar System Viewer: Astrodynamics Positioning & Semantic Zoom

> **Status:** `completed` | **Created:** 2026-09-20 | **Last Updated:** 2026-09-20 | **Completed:** 2026-09-20
> **Author:** Antigravity

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The current visualization focuses on a single isolated planetary globe at the origin. We need a cohesive solar system viewer that initializes all celestial bodies from their 2D Keplerian configurations at epoch (`t=0`), supports locked camera tracking with near-orthographic perspective, and implements semantic LOD zoom rules for surface nodes to prevent solid-mass rendering when zoomed out.

### 1.2 Core Objectives
- Initialize all `AstroNode` and `SurfaceNode` locations in memory at `t=0` using strict Keplerian mechanics (SSOT-PHY-001).
- Implement a floating-origin rendering system to preserve Bevy `f32` precision for interplanetary distances.
- Configure an orbit camera with ~1 degree FOV, locked focus to a specific `AstroNode`, and rotate/zoom capabilities.
- Render all nodes as dots (AstroNodes ~4x the size of SurfaceNodes).
- Implement semantic zoom (LOD) for `SurfaceNodes`: iteratively reveal points as the camera zooms in.
- Hide `SurfaceNodes` whose normals point away from the camera (backface culling).

### 1.3 Non-Goals & Exclusions
- Simulation ticks and active time progression (speed). This is purely for viewing the static initialization snapshot.
- Full UI overlays.
- Mesh textures or complex lighting. Nodes are rendered exclusively as solid off-white dots.

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-PHY-001: 2D Keplerian Astrodynamics](../../../docs/ssot/SSOT-PHY-001-Astrodynamics.md) — *Coplanar Keplerian position formulas.*
- [SSOT-PHY-004: AstroNode Architecture](../../../docs/ssot/SSOT-PHY-004-AstroNode-Architecture.md) — *Spherical Fibonacci Lattice.*
- [SSOT-UIX-001: Presentation Layer](../../../docs/ssot/SSOT-UIX-001-Presentation.md) — *Semantic zoom thresholds.*

### 2.2 Domain Types & Schemas
```rust
// In synthetic-core/src/astronomy/kinematics.rs
pub struct GlobalPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn calculate_epoch_position(config: &AstroNodeConfig, parent_pos: GlobalPosition) -> GlobalPosition {
    // Computes Keplerian position at t=0
}
```

### 2.3 Layer Boundary Mapping
- **Domain Layer:** `synthetic-core/src/astronomy/kinematics.rs` (f64 Kepler math)
- **Presentation Layer:** `synthetic-client/src/solar_system/...` (Floating origin render, Bevy point rendering, Camera controls)

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Domain Kinematics (synthetic-core)**
  - [x] Implement `calculate_epoch_position` in `synthetic_core::astronomy` using the Kepler formulas from SSOT-PHY-001.
  - [x] Write unit tests for epoch calculation (e.g. Earth at t=0).
- [x] **Step 2: Floating Origin System (synthetic-client)**
  - [x] Store `f64` global positions in Bevy ECS components for all bodies.
  - [x] Create a `FloatingOrigin` resource storing the ID of the currently focused body.
  - [x] Create a system that translates `f64` global positions to relative `f32` Bevy `Transform`s based on the focused body.
- [x] **Step 3: Camera Overhaul**
  - [x] Modify `GlobeOrbitCamera` (or create `SolarSystemCamera`) to use a `PerspectiveProjection` with `fov` set to ~1 degree (PI / 180).
  - [x] Adjust zoom ranges and sensitivities for interplanetary distances.
- [x] **Step 4: Dot Rendering & Semantic Zoom**
  - [x] Implement an instanced point renderer or custom `PointList` pipeline capable of distinct sizing (AstroNodes vs SurfaceNodes) and dot LOD.
  - [x] Apply semantic zoom thresholding (from SSOT-UIX-001 or dynamic angular sizing) to conditionally skip `SurfaceNode` rendering when the parent body is small in screen space.
  - [x] Implement dot-product backface culling for `SurfaceNodes` against the camera view vector.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- 100% unit test pass rate for Keplerian `t=0` position logic.
- Camera effectively views the solar system with no `f32` jitter at the focus point.
- Only visible dots are rendered, maintaining clear separation without merging into solid masses.

### 4.2 Verification Commands
```bash
cargo check
cargo clippy -- -D warnings
cargo test
```

---

## 🔍 5. Architectural Deviations & Retrospective

### 5.1 Deviations from Initial Plan
1. **Full-Window UI Viewport Transition:**
   - Instead of retaining the initial 4-panel docked interface (sidebars and bottom bar), the client transitioned to an unobstructed full-window viewport (`100%` width and height) with fallback hover support for mouse camera navigation.
2. **Surface Node Activation Rule:**
   - Shifted from a multi-tier semantic LOD curve to an instantaneous screen-radius threshold rule: all surface nodes for a body are rendered when the body's projected screen radius exceeds the 6px AstroNode dot radius (`radius_screen_px > ASTRO_DOT_RADIUS_PX`). Otherwise, only the single AstroNode dot is rendered.
3. **Orbital Curve Visualization:**
   - Added 128-segment Keplerian ellipse orbital loops in the 2D ecliptic plane (`z = 0`) for each orbiting body, parented to the parent attractor to move cohesively with the floating origin.
4. **Camera Coordinate Alignment & Zero-Roll Lock:**
   - Realigned the orbit camera spherical coordinates to match the solar system's ecliptic plane normal (`Vec3::Z`) rather than `Vec3::Y`. Orientation uses `look_at(focus, Vec3::Z)`, guaranteeing mathematical zero roll across all pitch and yaw angles.
5. **Perspective Field of View:**
   - Adjusted perspective camera FOV from 1.0 degree to 3.0 degrees (~0.05236 rad) to provide a slightly wider focal view while maintaining near-parallel projection.
6. **Surface Node Backface Culling:**
   - Re-introduced dot-product backface culling in the WGSL shaders (`dot(world_normal, view_vector) <= 0.0`), culling vertices outside the clip volume and discarding them in the fragment shader.
7. **Telemetry Overlay:**
   - Added a real-time smoothed FPS counter pinned to the top-right corner of the window.

### 5.2 Retrospective & Key Takeaways
- **Precision Floating Origin:** Anchoring Bevy's `Transform` relative to the focused celestial body completely eliminated `f32` vertex jitter across interplanetary distances (such as Neptune at 4.5e12 m).
- **Coordinate Conventions:** Ensuring the camera's up vector matches the orbital plane's normal (`Vec3::Z`) prevents gimbal and roll issues when orbiting coplanar systems.
