---
id: PLAN-009
title: "Implement WebGPU Rendered Surface View with Backface Culling"
status: completed
author: "Antigravity"
created: 2026-09-14
updated: 2026-09-16
completed_at: 2026-09-16
branch: "main"
---

# PLAN-009: Implement WebGPU Rendered Surface View with Backface Culling

> **Status:** `completed` | **Created:** 2026-09-14 | **Completed:** 2026-09-16
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The user has requested the implementation of the planetary globe view (as defined in SSOT-UIX-001) in the upper center UI panel using WebGPU/Bevy 0.15. Surface nodes must be rendered efficiently as discrete points on a sphere. To avoid seeing nodes through the back of the planetary body, a backface culling technique utilizing the dot product of the node's normal vector and the exact perspective view vector must be evaluated in the vertex shader.

### 1.2 Core Objectives
- Implement a 3D camera viewport integrated into the central "2: Main View" panel via dynamic sub-viewport coordinate synchronization.
- Implement a custom WGSL shader material (`GlobeMaterial`) in Bevy to render points (`PrimitiveTopology::PointList`).
- Implement exact perspective backface culling in the vertex shader: `view_vector = normalize(camera_pos - vertex_pos)`, culling when `dot(normal, view_vector) <= 0.0`.
- Implement a Spherical Fibonacci mesh generator adhering strictly to `SSOT-PHY-004`.
- Add an interactive orbit/rotation system so the globe rotates smoothly to visually demonstrate backface horizon culling.

### 1.3 Non-Goals & Exclusions
- Implementing the full 2D Interplanetary Ecliptic Plane mode (focusing purely on the 3D Planetary Globe view).
- Semantic zoom clustering logic (will be deferred; all nodes render as Tier 2 Micro Discrete Nodes for now).
- Fetching live node data from a simulation backend (will use generated Fibonacci coordinates on a sphere for prototyping).

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-UIX-001: UI Architecture, Viewport State Machine & Terminal Grammar](../../../docs/ssot/SSOT-UIX-001-Presentation.md) — *Specifies the Docked 4-Pane Workspace and the dual-instance viewport (specifically `VIEW_PLANETARY_GLOBE`).*
- [SSOT-PHY-004: Spherical Fibonacci Surface Topography & Spatial Indexing](../../../docs/ssot/SSOT-PHY-004-Surface-Topography.md) — *Specifies the Golden-Angle Spherical Fibonacci spiral generation.*

### 2.2 Domain Types & Schemas
```rust
// crates/synthetic-client/src/globe/material.rs
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlobeMaterial {
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(0)]
    pub atmosphere_color: LinearRgba,
}

impl Material for GlobeMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/globe_point.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/globe_point.wgsl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.topology = bevy::render::render_resource::PrimitiveTopology::PointList;
        descriptor.primitive.cull_mode = None;
        Ok(())
    }
}
```

### 2.3 Public API / Plugin Contract
```rust
// crates/synthetic-client/src/globe/mod.rs
use bevy::prelude::*;

pub struct GlobePlugin;

impl Plugin for GlobePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<GlobeMaterial>::default())
           .add_systems(Startup, setup_globe_scene)
           .add_systems(Update, (sync_globe_viewport, rotate_globe));
    }
}
```

### 2.4 Layer Boundary Mapping
- **Presentation Rust Layer:** `crates/synthetic-client/src/globe/`
- **Presentation Shader Layer:** `crates/synthetic-client/assets/shaders/globe_point.wgsl`

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Shader & Custom Material Implementation**
  - [x] Create `crates/synthetic-client/assets/shaders/globe_point.wgsl` with vertex and fragment entry points implementing perspective view-vector dot product culling.
  - [x] Implement `GlobeMaterial` in `crates/synthetic-client/src/globe/material.rs` using Bevy 0.15 `Material` trait with `PointList` specialization.

- [x] **Step 2: Fibonacci Mesh Generation**
  - [x] Implement `generate_fibonacci_globe_mesh(node_count: usize, radius: f32) -> Mesh` in `crates/synthetic-client/src/globe/mesh.rs` per `SSOT-PHY-004`.
  - [x] Populate vertex positions and surface normals on a sphere.

- [x] **Step 3: Sub-Viewport Synchronization & Scene Setup**
  - [x] Implement viewport synchronization in `crates/synthetic-client/src/globe/viewport.rs` to track the "2: Main View" UI panel bounds.
  - [x] Implement `setup_globe_scene` in `crates/synthetic-client/src/globe/mod.rs` to spawn `Camera3d`, light, rotating globe entity, and wire `GlobePlugin` into `main.rs`.
  - [x] Add gentle rotation system (`rotate_globe`) to clearly demonstrate dynamic horizon backface culling.

- [x] **Step 4: Compilation & Visual Verification**
  - [x] Run `cargo check -p synthetic-client` to verify zero type or compile errors.
  - [x] Run `cargo clippy -p synthetic-client` to ensure workspace lint compliance.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [x] Target 1: Clean compilation and clippy pass with zero warnings across synthetic-client.
- [x] Target 2: Vertex shader culls all points facing away from camera based on perspective view vector.
- [x] Target 3: Points render using PointList topology within the Main View panel rect.

### 4.2 Verification Commands
```bash
# Cargo Check
cargo check -p synthetic-client

# Clippy Validation
cargo clippy -p synthetic-client -- -D warnings
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- **Exact Perspective View Vector**: Updated vertex shader from orthographic camera forward vector approximation to true perspective view vector calculation (`normalize(camera_position - world_position)`).
- **SSOT-PHY-004 Fibonacci Integration**: Replaced arbitrary mock points with deterministic Spherical Fibonacci spiral generator with Golden Angle stepping.
- **Bevy 0.15 UI Viewport Synchronization**: Implemented dynamic `Viewport` synchronization tracking `ComputedNode` layout bounds to render `Camera3d` directly within the "2: Main View" panel.

### 5.2 Lessons Learned & Follow-Up Tasks
- **Follow-up Completed in PLAN-010**: Implemented interactive mouse orbit and zoom controls.

