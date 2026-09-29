---
id: PLAN-004
title: "Scaffold Initial Economic Simulation Scenario and IPC DTOs"
status: completed
author: "Antigravity"
created: 2026-09-13
updated: 2026-09-13
completed_at: 2026-09-13
branch: "main"
---

# PLAN-004: Scaffold Initial Economic Simulation Scenario and IPC DTOs

> **Status:** `completed` | **Created:** 2026-09-13 | **Last Updated:** 2026-09-13 | **Completed:** 2026-09-13
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
We need to stress-test the Bevy ECS node and flow network architecture with a realistic multi-node mock setup, restricted to a single body (Earth). The frontend currently only displays orbital states and lacks insight into the economic network.

### 1.2 Core Objectives
- Create a deterministic programmatic generator in Rust to instantiate a 200-node economy (Mines, Factories, Population Centers).
- Define a 40-resource tiered economy (Raw, Intermediate, Consumer Goods) and algorithmically wire nodes together based on supply/demand.
- Extend `SimulationStateDto` to serialize `Storage`, `Converter`, and `FlowEdge` state across the Tauri IPC boundary.
- Implement a paginated/filterable table in the React frontend to inspect the 200-node graph.

### 1.3 Non-Goals & Exclusions
- Space-based transfers or multi-body topologies (reserved for future systems).
- Spatial distance heuristics or latency calculations (random/fixed latency based on connection).
- Full canvas/graph-based node visualization in the UI (tables will suffice).

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Domain Types & Schemas

**Rust DTOs (crates/synthetic-core/src/simulation.rs):**
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceDto {
    pub id: u32,
    pub name: String, // e.g., "Raw Resource 1", "Consumer Good 5"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StorageDto {
    pub entity_id: u32,
    pub resource_id: u32,
    pub amount: i64,      // raw micro-units
    pub capacity: i64,    // raw micro-units
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConverterDto {
    pub entity_id: u32,
    pub recipe_id: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FlowEdgeDto {
    pub edge_id: u32,
    pub source_id: u32,
    pub destination_id: u32,
    pub in_transit: i64, // total micro-units in packets
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationStateDto {
    pub tick: u64,
    pub timestamp_seconds: f64,
    pub delta_time_seconds: f64,
    pub entities: Vec<OrbitalState>, // Kept for legacy/future
    pub resources: Vec<ResourceDto>,
    pub storages: Vec<StorageDto>,
    pub converters: Vec<ConverterDto>,
    pub edges: Vec<FlowEdgeDto>,
}
```

**TypeScript DTOs (src/types/simulation.ts):**
*Must mirror the Rust types above precisely.*

### 2.2 System Architecture
- **Mines:** Standard `Converter` nodes with recipes that require `0` inputs.
- **Population Centers:** Standard `Converter` nodes with recipes that produce `0` outputs.
- **Factories:** Standard `Converter` nodes linking intermediate tiers.

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Expand DTO Contracts**
  - [x] Add `ResourceDto`, `StorageDto`, `ConverterDto`, and `FlowEdgeDto` to `synthetic-core/src/simulation.rs`.
  - [x] Update `SimulationStateDto` and the frontend `src/types/simulation.ts` to match.
- [x] **Step 2: Update World Serialization**
  - [x] Modify `SimulationStateDto::from_world` to query `Storage`, `Converter`, `FlowEdge`, and `FlowQueue` from the Bevy ECS world and map them to DTOs.
  - [x] Query a new `ResourceDirectory` resource that holds the names of the 40 resources to populate `resources` DTO.
- [x] **Step 3: Deterministic Generator in Rust**
  - [x] Create `generator.rs` or update `simulation.rs`.
  - [x] Generate 40 resources (10 Raw, 20 Intermediate, 10 Consumer).
  - [x] Generate standard recipes connecting these tiers.
  - [x] Instantiate 200 Converter entities (Mines, Factories, Populations).
  - [x] Instantiate `Storage` nodes for each converter's inputs and outputs.
  - [x] Algorithmically spawn `FlowEdge` components linking compatible storage nodes.
- [x] **Step 4: Frontend Visualization**
  - [x] Update `src/App.tsx` to handle the new full-graph DTO payload.
  - [x] Build a tabbed or filterable table view allowing users to inspect Storages, Converters, and Flows independently.
  - [x] Add real-time performance telemetry (TPS and backend Compute Latency in ms) to the metric cards.
  - [x] Ensure the existing "START CLOCK" / "PAUSE CLOCK" controls correctly hook into the new simulation tick cycle.
- [x] **Step 5: Full Regression & Verification**
  - [x] Execute `cargo check` and `npm run typecheck` (`strict: true`).
  - [x] Execute `cargo test`.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- `cargo check` and `npm run typecheck` pass with zero errors.
- Simulation initialization successfully spawns ~200 converters and valid edges.
- Front-end correctly renders tables parsing the 200-node graph without crashing.

### 4.2 Verification Commands
```bash
# Backend Check
cargo check --manifest-path crates/synthetic-core/Cargo.toml
cargo test --manifest-path crates/synthetic-core/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml

# Frontend Typecheck and Test
npm run typecheck
npm test
npm run lint
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)
- `SimulationSession` was adopted in `src-tauri/src/lib.rs` to maintain the Bevy ECS World continuously in memory across ticks rather than re-instantiating from DTO snapshots on every IPC invocation.
- Modularized frontend tables and telemetry cards into `MetricCards.tsx` and `NetworkTables.tsx` to maintain strict single-responsibility units and keep all source files well within the 250-line limit.
- Implemented a TypeScript mirror scenario generator and simulation stepper in `src/domain/generator.ts` to provide full functional parity for browser-only exploration (`npm run dev`) alongside native Tauri execution.
- Verified 100% test pass rate across all 24 unit tests, 5 network regression tests, and 3 spatial attachment tests.
