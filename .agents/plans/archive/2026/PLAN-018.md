---
id: PLAN-018
title: "Improve Camera Navigation UI"
status: completed
author: "Antigravity"
created: 2026-09-22
updated: 2026-09-22
completed_at: 2026-09-22
branch: "feature/camera-navigation"
---

# PLAN-018: Improve Camera Navigation UI

> **Status:** `completed` | **Created:** 2026-09-22 | **Last Updated:** 2026-09-22
> **Author:** Antigravity | **Branch:** feature/camera-navigation

---

# Phase 1: Functional Spec (Defined via /plan-human)

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The current camera navigation interactions are suboptimal. The vertical orbiting axis when using the mouse does not feel intuitive to users who expect an inverted vertical axis. Additionally, navigating the scene is tedious because users cannot easily set the camera's focal point to a specific astronode by clicking on it in the UI.

### 1.2 Core Objectives
- Flip the axis for vertical orbiting when using mouse drag input.
- Enable screen-picking (raycasting) so that selecting a specific astronode on the screen moves the camera's focal point (target) to that object.

### 1.3 Non-Goals & Exclusions
- Adding new UI overlays or changing the visual representation of astronodes.
- Modifying keyboard or controller input mappings for the camera.
- Creating complex pathfinding or automated camera tours beyond the simple focal point transition.

---

## 📋 2. Functional Acceptance Criteria & User Scenarios

### 2.1 User Scenarios & Core Workflows
- **Scenario 1:** Given the user is viewing the simulation, when they click and drag the mouse vertically to orbit the camera, then the camera pitches in the opposite direction compared to the original behavior.
- **Scenario 2:** Given multiple astronodes on the screen, when the user clicks directly on one of the astronodes, then the camera's focal point updates to center on the selected astronode.

### 2.2 Functional Acceptance Criteria
- Vertical mouse drag input for orbiting is inverted.
- Clicking on an astronode correctly identifies the clicked object via raycasting or screen-to-world projection.
- The camera's target/focal point is updated to the position of the selected object.
- Deselecting or clicking empty space either does nothing or maintains the last focal point.

---

# Phase 2: Technical Design & Invariants (Defined via /plan-robot)

## 📐 3. Technical Contracts & Invariants

*All domain grounding references, types, schemas, and public API signatures must be declared and reviewed here prior to code implementation.*

### 3.1 Authoritative Domain References
*Downlink to immutable domain specifications, formulas, constants, and truth tables in `docs/ssot/`:*
- N/A - Camera interactions and raycasting do not have specific physics formulas, relying on Bevy's built-in camera projection math.

### 3.2 Domain Types, ECS Components & Schemas
```rust
// Existing components will be reused.
// No new data schemas required.
```

### 3.3 Public API / System Signatures
```rust
// In `crates/synthetic-client/src/systems/astronomy.rs`
pub fn mouse_pick_astronode_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<bevy::window::PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform, &Projection), With<GlobeOrbitCamera>>,
    body_query: Query<(&CelestialBody, &GlobalTransform)>,
    mut origin: ResMut<FloatingOrigin>,
    mut globe_camera: Query<&mut GlobeOrbitCamera>,
    mut click_start_pos: Local<Option<Vec2>>,
)
```

### 3.4 Mathematical Invariants & Determinism Guarantees
*Explicit invariants, precision bounds, and determinism rules (CRITICAL: ZERO LATEX):*
- Invariant 1: Vertical mouse drag must increase pitch when moving mouse up (negative Y delta) and decrease pitch when moving down, matching inverted controls.
- Invariant 2: Mouse picking must prioritize the celestial body closest to the camera in 3D world space (Z-depth) among all bodies that overlap the cursor in screen space.
- Invariant 3: The pick radius threshold must be the maximum of the projected 2D body radius (via `calculate_screen_radius_px` using `CelestialBody.config.radius_m`) and a minimum click radius (e.g., 10.0 px) to ensure small distant bodies are easily selectable.
- Invariant 4: Screen distance from cursor to body center must be calculated using squared distance `(x2-x1)^2 + (y2-y1)^2` avoiding unnecessary `sqrt` operations.
- Invariant 5: `Camera::world_to_viewport` returns an `Option`. Bodies returning `None` (behind the near clip plane) must be safely filtered out to prevent panics.
- Invariant 6: A pick is only registered if the mouse was pressed and released without significant cursor displacement, differentiating a click from a drag.

### 3.5 Layer Boundary Mapping
- **ECS Systems:** `crates/synthetic-client/src/globe/camera.rs` (Axis inversion)
- **ECS Systems:** `crates/synthetic-client/src/systems/astronomy.rs` (Mouse picking logic)

---

## 🛠️ 4. Implementation Steps

*Ordered checkbox checklist broken down into atomic, testable steps.*

- [x] **Step 1: Invert Vertical Orbiting Axis**
  - [x] Modify `orbit_camera_input_system` in `crates/synthetic-client/src/globe/camera.rs`.
  - [x] Change `motion_delta.y` subtraction to addition for `camera.target_pitch`.
- [x] **Step 2: Implement Screen-to-World Picking Logic**
  - [x] Implement `mouse_pick_astronode_system` in `crates/synthetic-client/src/systems/astronomy.rs`.
  - [x] Add an early return if `!mouse_button_input.just_released(MouseButton::Left)` and `!mouse_button_input.just_pressed(...)`.
  - [x] Track mouse press position in `Local<Option<Vec2>>` to differentiate clicks from drags (cancel if dragged > 5 pixels).
  - [x] On valid click release, project each `CelestialBody`'s 3D position to 2D viewport coordinates safely (handling `None`).
  - [x] Use `distance_squared` against the calculated squared threshold (max of actual 2D radius and 10px).
  - [x] From all bodies within their threshold, select the one with the smallest 3D distance to the camera.
- [x] **Step 3: Integrate Picking System with Floating Origin**
  - [x] On successful pick, use `focus_on_body` to update the `FloatingOrigin` resource and `GlobeOrbitCamera` target distance.
  - [x] Register `mouse_pick_astronode_system` in `AstronomyPlugin` inside the `Update` schedule.
- [x] **Step 4: Full Regression & Verification**
  - [x] Execute `cargo check`, `cargo clippy`, and `cargo test`.
  - [x] Implement tests for Click vs Drag differentiation, Behind-Camera projection, and Z-Depth priority.

---

## 🧪 5. Verification & Criteria

### 5.1 Measurable Benchmarks & Invariant Targets
- [Target 1: 100% unit test pass rate for camera modules]
- [Target 2: Zero compiler warnings and zero clippy warnings (`cargo clippy`)]

### 5.2 Unit & Property Test Targets
| Module / File | Test File | Key Scenarios & Invariants Covered |
| :--- | :--- | :--- |
| `crates/.../src/systems/astronomy.rs` | `crates/.../src/systems/astronomy.rs` | Ensure `mouse_pick_astronode_system` can be integrated without breaking existing focus tests. |
| `crates/.../src/systems/astronomy.rs` | `crates/.../src/systems/astronomy.rs` | Click vs. Drag Differentiation (no trigger on drag). |
| `crates/.../src/systems/astronomy.rs` | `crates/.../src/systems/astronomy.rs` | Behind-Camera Projection (handling `None` gracefully). |
| `crates/.../src/systems/astronomy.rs` | `crates/.../src/systems/astronomy.rs` | Z-Depth Priority (picks closest 3D body if multiple overlap in 2D). |

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
- *No major deviations. The picking logic efficiently uses a Z-depth priority and squared distance check to avoid performance hits, as suggested by the interrogator.*

### 6.2 Lessons Learned & Follow-Up Tasks
- *Handling `Option` safely in `world_to_viewport` proved critical to prevent unwrapping panics for objects behind the camera. Follow-up: Ensure all future viewport projections across the codebase do similar filtering.*
