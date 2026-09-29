---
id: PLAN-010
title: "Implement Interactive Orbit & Zoom Camera Controls for Planetary Globe View"
status: completed
author: "Antigravity"
created: 2026-09-16
updated: 2026-09-16
completed_at: 2026-09-16
branch: "main"
---

# PLAN-010: Implement Interactive Orbit & Zoom Camera Controls for Planetary Globe View

> **Status:** `completed` | **Created:** 2026-09-16 | **Completed:** 2026-09-16
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The user has requested interactive camera controls for the 3D planetary globe view in `synthetic-client`. The operator must be able to rotate/orbit around the planet via mouse dragging (yaw and pitch) and zoom smoothly in and out via mouse wheel scrolling. Camera interaction must only trigger when interacting with or hovering over the "2: Main View" panel.

### 1.2 Core Objectives
- Create `crates/synthetic-client/src/globe/camera.rs` with `GlobeOrbitCamera` component and spherical coordinate transformation math.
- Implement `orbit_camera_input_system` to handle mouse drag rotation, mouse wheel zoom, and UI hover bounds gating.
- Implement `orbit_camera_transform_system` with frame-rate independent exponential damping interpolation.
- Wire camera systems into `GlobePlugin` in `crates/synthetic-client/src/globe/mod.rs`.
- Verify smooth mouse rotation and zooming with zero compilation warnings or lints.

### 1.3 Non-Goals & Exclusions
- Free-flight 6DOF WASD movement (this is strictly a focused spherical orbit camera).
- Changing planetary focus targets (stays centered at origin `Vec3::ZERO`).

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-UIX-001: UI Architecture, Viewport State Machine & Terminal Grammar](../ssot/SSOT-UIX-001-Presentation.md) — *Specifies the Viewport State Machine and semantic zoom altitude thresholds.*

### 2.2 Domain Types & Schemas
```rust
// crates/synthetic-client/src/globe/camera.rs
use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct GlobeOrbitCamera {
    pub focus: Vec3,
    pub distance: f32,
    pub target_distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub target_yaw: f32,
    pub target_pitch: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    pub rotate_sensitivity: f32,
    pub zoom_sensitivity: f32,
    pub damping: f32,
    pub is_dragging: bool,
}
```

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Implement `GlobeOrbitCamera` Component & Systems**
  - [x] Create `crates/synthetic-client/src/globe/camera.rs`.
  - [x] Implement spherical-to-Cartesian position calculation and exponential damping.
  - [x] Implement mouse drag and scroll input reading with Main View hover gating.

- [x] **Step 2: Scene & Plugin Wire-up**
  - [x] Update `crates/synthetic-client/src/globe/mod.rs` to register camera systems and attach `GlobeOrbitCamera` to `Camera3d`.
  - [x] Update `crates/synthetic-client/src/main.rs` to add `Interaction` component to `MainViewPanelMarker`.

- [x] **Step 3: Verification & Polish**
  - [x] Run `cargo check --workspace` and `cargo clippy --workspace -- -D warnings`.
  - [x] Test runtime execution and confirm smooth orbiting and zooming.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [x] Target 1: Clean compilation and clippy pass with zero warnings.
- [x] Target 2: Mouse drag smoothly orbits camera in yaw and pitch.
- [x] Target 3: Mouse wheel smoothly zooms between min_distance and max_distance.
- [x] Target 4: Hovering over sidebars ignores camera drag and zoom.

### 4.2 Verification Commands
```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo run -p synthetic-client
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- *[None. Implemented strictly to approved architecture.]*

### 5.2 Lessons Learned & Follow-Up Tasks
- Added dual hover checking combining `Interaction` and bounding rect validation to ensure seamless mouse drag tracking even during rapid cursor sweeps.

