---
id: PLAN-015
title: "Solar System Ecliptic Orientation Widget and Astrobody Selector"
status: completed
author: "Antigravity"
created: 2026-09-21
updated: 2026-09-21
completed_at: 2026-09-21
branch: "main"
---

# PLAN-015: Solar System Ecliptic Orientation Widget and Astrobody Selector

> **Status:** `completed` | **Created:** 2026-09-21 | **Last Updated:** 2026-09-21
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The solar system visualization operates in 3D, allowing users to navigate around individual bodies. However, users currently lack a visual reference for the solar ecliptic orientation and an intuitive UI method to select which astrobody to focus the camera on (currently relying solely on keyboard shortcuts).

### 1.2 Core Objectives
- Implement an orientation widget (similar to a view cube) floating in the top-right corner of the UI that displays the solar ecliptic orientation (Z up, with a visual circle representing the ecliptic plane).
- Implement a floating UI selection mechanism (e.g., a list or dropdown) to allow the user to select a specific astrobody and snap the camera focus to it.

### 1.3 Non-Goals & Exclusions
- Changing the underlying physics or planetary orbital models.
- Interactive clicking/dragging on the orientation widget to rotate the camera (read-only orientation display is sufficient for this plan, unless explicitly requested).
- Re-architecting the existing 3D camera controls, aside from adding the new programmatic focus target trigger.

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-UIX-001: UI Architecture, Viewport State Machine & Terminal Grammar](../ssot/SSOT-UIX-001-Presentation.md) — *Invariant 3: Floating Overlay Constraints. UI components must never block or rigidly divide the fullscreen viewport. Data and controls must float over the spatial view.*

### 2.2 Domain Types & Schemas
```rust
// New UI components for the orientation widget and selector
#[derive(Component)]
pub struct EclipticOrientationWidget;

#[derive(Component)]
pub struct AstrobodySelectorUI;
```

### 2.3 Public API / Service Signatures
```rust
// Systems to be added/modified in UI layer
pub fn setup_orientation_widget(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut orbit_materials: ResMut<Assets<OrbitMaterial>>,
    windows: Query<&Window>,
);
pub fn sync_orientation_widget_camera(
    orbit_camera_query: Query<&GlobeOrbitCamera>,
    mut widget_camera_query: Query<&mut Transform, With<EclipticOrientationCameraMarker>>,
);
pub fn spawn_astrobody_selector(
    mut commands: Commands,
    origin: Res<FloatingOrigin>,
    ui_camera_query: Query<Entity, With<crate::UiCameraMarker>>,
);
pub fn handle_astrobody_selector_interaction(
    mut button_query: Query<(&Interaction, &AstrobodyButton, &mut BackgroundColor, &mut BorderColor), With<Button>>,
    mut origin: ResMut<FloatingOrigin>,
    body_query: Query<&CelestialBody>,
    mut camera_query: Query<&mut GlobeOrbitCamera>,
);
```

### 2.4 Layer Boundary Mapping
- **Presentation Layer:** `crates/synthetic-client/src/systems/ui.rs` to hold the floating widget and selector logic.

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: UI Scaffolding**
  - [x] Create a new UI module/plugin if necessary, or expand `setup_ui` in `main.rs`.
  - [x] Set up the UI layout containers (NodeBundles) floating on the right side of the screen per Invariant 3.
- [x] **Step 2: Ecliptic Orientation Widget**
  - [x] Render a 3D widget (e.g., using a separate `Camera3d` with `ClearColorConfig::None` and higher `order`, restricted to a specific viewport/render target, OR using a 2D projection approach). Given Bevy's capabilities, a secondary UI-rendered 3D camera layered on top is the standard approach for view cubes.
  - [x] Create the visual representation (Z-up axis indicator and ecliptic plane circle).
  - [x] Sync the widget's rotation inversely with the main `GlobeOrbitCamera` to reflect current orientation.
- [x] **Step 3: Astrobody Selector UI**
  - [x] Query the `FloatingOrigin` resource to build a list of selectable buttons for each loaded celestial body.
  - [x] Implement interaction systems to update the `FloatingOrigin::focused_index` when a button is clicked.
- [x] **Step 4: Integration and Styling**
  - [x] Apply styling consistent with existing floating windows (e.g., FPS counter).
  - [x] Ensure seamless interoperability with existing keyboard shortcuts for camera switching.
- [x] **Step 5: Full Regression & Verification**
  - [x] Build and run the client to verify visual placement and correct logic.
  - [x] Execute `cargo clippy` and `cargo test`.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- The orientation widget accurately reflects the 3D camera's orientation relative to the solar ecliptic (Z-up).
- Clicking an astrobody in the selector successfully shifts the camera focus to the new target.
- Zero type errors and linter warnings from `cargo clippy`.

### 4.2 Unit Test Targets
| Module / File | Key Scenarios Covered |
| :--- | :--- |
| `crates/synthetic-client/src/systems/ui.rs` | Circle and line strip vertex generator validation |
| `crates/synthetic-client/src/systems/astronomy.rs` | `focus_on_body` boundary and no-op condition checks |

### 4.3 Verification Commands
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- Unified body focus switching logic into a shared `focus_on_body` function in `astronomy.rs` to avoid divergence between keyboard shortcuts and UI button clicks.
- Leveraged Bevy 0.15 `RenderLayers::layer(1)` with a secondary `Camera3d` (order 2) and viewport anchoring to render the orientation widget without interfering with the primary 3D spatial camera or 2D UI overlay.

### 5.2 Lessons Learned & Follow-Up Tasks
- Future enhancements may add click interactions to the orientation widget to snap the camera to top-down (+Z) or cardinal views (+X, +Y).
