---
id: PLAN-005
title: "Scaffold Surface Layer Visualisation UI/UX Pipeline"
status: completed
author: "Antigravity"
created: 2026-09-13
updated: 2026-09-13
completed_at: 2026-09-13
branch: "surface-layer-viz"
---

# PLAN-005: Scaffold Surface Layer Visualisation UI/UX Pipeline

> **Status:** `completed` | **Created:** 2026-09-13 | **Last Updated:** 2026-09-13
> **Author:** Antigravity | **Branch:** surface-layer-viz

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The current frontend UI consists of metric cards and network tables but lacks a spatial or geographical representation of the simulation. To support the visual integration of facilities on planetary surfaces (reminiscent of the Rimworld world map), we need a 3D surface layer visualization pipeline that renders astronomical bodies as spheres with lat/long lines and overlays the H3 hex grid outlines. 

### 1.2 Core Objectives
- Integrate a 3D rendering pipeline into the React frontend using **Deck.gl** (configured for **WebGPU**) as specified in our architecture (`PLAN-001`).
- Render a basic spherical proxy for astronomical bodies (a globe view), stylized with rough latitude/longitude wireframes.
- Leverage Deck.gl's `H3HexagonLayer` to render H3 hex outlines over the surface of the sphere to visualize the spatial grid directly on the GPU.
- Expand the Tauri IPC `SimulationStateDto` and Rust backend payload to expose `AstroNode` (planets/bodies) and `SurfaceNode` (facility placements) components.

### 1.3 Non-Goals & Exclusions
- Fully textured or procedurally generated terrain (deferred to later UI passes).
- Advanced interactivity, such as placing new facilities via clicking on hexes (this plan is strictly visualization).
- Visualizing orbital mechanics or deep space node networks (focus is entirely on the surface layer grid).

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [PLAN-001: Define Tech Stack, Engine, and System Architecture](../../archive/2026/PLAN-001.md) — *Specifies WebGPU and Deck.gl.*
- Continues to build on the foundations laid by `PLAN-003: Scaffold H3 Spatial Grid for Astronomical Bodies` which introduced `h3o` in the Rust backend.

### 2.2 Domain Types & Schemas

**Frontend Payload Additions (`src/types/simulation.ts`):**
```typescript
export interface AstroNodeDto {
  readonly entity_id: number;
  readonly body_id: number;
  readonly radius_km: number;
  readonly h3_resolution: number;
}

export interface SurfaceNodeDto {
  readonly entity_id: number;
  readonly parent_body_id: number;
  readonly h3_cell_index: string; // 64-bit hex string from H3
}

export interface SimulationStateDto {
  // ... existing fields ...
  readonly astro_nodes: readonly AstroNodeDto[];
  readonly surface_nodes: readonly SurfaceNodeDto[];
}
```

**Rust Backend Equivalents (`crates/synthetic-core/src/network/types.rs` or similar DTO module):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AstroNodeDto {
    pub entity_id: u32,
    pub body_id: u32,
    pub radius_km: u32,
    pub h3_resolution: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceNodeDto {
    pub entity_id: u32,
    pub parent_body_id: u32,
    pub h3_cell_index: String,
}
```

### 2.3 Layer Boundary Mapping
- **Domain Layer (Rust):** Serializes `AstroNode` and `SurfaceNode` components into DTOs in the Bevy ECS tick system.
- **Presentation Layer (React):** Introduces `@deck.gl/react`, `@deck.gl/core`, and `@deck.gl/geo-layers` to render the WebGPU canvas. Uses the `H3HexagonLayer` to automatically project the H3 cell indices provided by the backend onto the 3D globe.

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Frontend Dependencies & Tooling**
  - [x] Run `npm install @deck.gl/core @deck.gl/react @deck.gl/geo-layers @deck.gl/layers h3-js`
  - [x] Install typings if necessary (`h3-js` ships with bundled types in dist/types.d.ts).
- [x] **Step 2: DTO & IPC Expansion**
  - [x] Update `SimulationStateDto` in `src/types/simulation.ts` to include `AstroNodeDto` and `SurfaceNodeDto`.
  - [x] Update the Rust DTO structures (`SimulationStateDto` equivalent) and serialization logic in `src-tauri` / `synthetic-core` to extract `AstroNode` and `SurfaceNode` components from the ECS.
- [x] **Step 3: WebGPU Deck.gl Canvas & Globe Component**
  - [x] Create `src/components/SurfaceMap.tsx` containing a `<DeckGL>` canvas component.
  - [x] Configure the Viewport to use `GlobeView` or equivalent 3D geospatial projection supported by Deck.gl.
- [x] **Step 4: H3 Hex Overlay Rendering**
  - [x] Use `h3-js` on the frontend (e.g. via `getRes0Cells` and `getChildren`) to generate the full set of H3 resolution 3 cells for the planet.
  - [x] Feed these resolution 3 cell indices into Deck.gl's `H3HexagonLayer` to draw the global grid wireframe, highlighting specific hexes occupied by `surface_nodes`.
- [x] **Step 5: App Integration & Verification**
  - [x] Integrate the `<SurfaceMap>` component into the main layout in `App.tsx`.
  - [x] Ensure the application compiles without type errors.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- The UI displays a Deck.gl WebGPU-powered 3D sphere upon launching the application.
- Hex grid lines are visible and match the H3 resolution defined by the simulated astronomical body, visualized via `H3HexagonLayer`.
- Zero typecheck errors for the newly added Deck.gl integrations.

### 4.2 Verification Commands
```bash
npm run typecheck
npm run lint
cargo clippy --workspace
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- **WebGPU Deck.gl over Three.js:** Realigned from Three.js to Deck.gl with WebGPU support and `_GlobeView` as mandated by `PLAN-001` and `RESEARCH-0009`.
- **Pre-computed H3 Res-3 Cache:** Rather than querying H3 indices on every render frame, `generateH3Grid(3)` is computed once and memoized, efficiently providing all 41,162 hex indices directly to the GPU layer.

### 5.2 Lessons Learned & Follow-Up Tasks
- Deck.gl's `H3HexagonLayer` combined with `_GlobeView` renders 41,162 resolution-3 hexagonal cells in real time with high frame rates and interactive rotation/pitch/zoom.
- DTO serialization cleanly isolated in `crates/synthetic-core/src/dto.rs` keeps serialization boundaries decoupled from internal ECS component state.

