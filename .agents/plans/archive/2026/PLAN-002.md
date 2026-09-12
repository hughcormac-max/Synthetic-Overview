---
id: PLAN-002
title: "Scaffold Core ECS and Node/Processor Network using bevy_ecs"
status: completed
author: "Antigravity"
created: 2026-09-12
updated: 2026-09-12
completed_at: 2026-09-12
branch: "main"
---

# PLAN-002: Scaffold Core ECS and Node/Processor Network using bevy_ecs

> **Status:** `completed` | **Created:** 2026-09-12 | **Last Updated:** 2026-09-12 | **Completed:** 2026-09-12
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
We need to establish the foundational "Micro" scale of the simulation: the Node/Processor network. Because entities in our simulation are heterogeneous (e.g., surface facilities vs. orbital facilities) and can dynamically gain or lose traits (e.g., relocating), a rigid custom `Vec` approach is insufficient. We will implement this Tier 0 domain logic using `bevy_ecs` to power the simulation tick loop, upon which orbital mechanics (Macro) and planetary surfaces (Meso) will eventually run.

### 1.2 Core Objectives
- Integrate `bevy_ecs` into the `synthetic-core` Tier 0 domain layer.
- Implement the Data-Oriented Design (DOD) Stock-and-Flow primitives defined in SSOT-001 (Stocks, Flow Edges, Converters) as Bevy `Component`s.
- Utilize Bevy's sparse-set storage for highly dynamic components (like `Relocating`).
- Implement the rigid Two-Pass Cycle tick loop (Phase 1: Demand Registration, Phase 2: Allocation, Phase 3: Transit, Phase 4: Production) via Bevy `System`s and `Schedule`s.
- Enforce Zero-Sum Conservation using Fixed-Point Arithmetic (64-bit integers) for resource tracking to prevent floating-point drift.

### 1.3 Non-Goals & Exclusions
- Macro-scale orbital mechanics visualization (deferred).
- Meso-scale H3 surface implementation (deferred).
- UI/Tauri integration beyond a basic tick trigger (deferred).

---

## ðŸ“ 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-001: Simulation Architecture](../../../docs/ssot/SSOT-001-Simulation-Architecture.md) â€” *Stock, Edge, and Converter definitions, Two-Pass Cycle.*

### 2.2 Domain Types & Schemas
```rust
use bevy_ecs::prelude::*;

// Fixed-point arithmetic wrapper for resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResourceAmount(pub i64); 

// Core Node Components
#[derive(Component, Debug, Clone)]
pub struct Storage {
    pub element_id: u32,
    pub current_amount: ResourceAmount,
    pub capacity: ResourceAmount,
}

#[derive(Component, Debug, Clone)]
pub struct Converter {
    pub recipe_id: u32,
    pub health: u16, // Fixed point 0-10000 representing 0.0 - 1.0
    pub operational_status: bool,
}

// Dynamic/Transient Tags (Stored in Sparse Sets to prevent archetype fragmentation)
#[derive(Component, Debug, Clone)]
#[component(storage = "SparseSet")]
pub struct Relocating {
    pub destination_id: Entity,
    pub arrival_tick: u64,
}
```

### 2.3 Public API / Service Signatures
```rust
use bevy_ecs::prelude::*;

/// Initializes the Bevy ECS World and core schedules.
pub fn initialize_simulation_world() -> World {
    let mut world = World::new();
    // Add Resources (e.g., CurrentTick)
    // Spawn initial entities
    world
}

/// Core tick pipeline executing the ordered SSOT-001 phases.
pub fn tick_simulation_world(world: &mut World) -> Result<(), DomainError> {
    let mut schedule = Schedule::new(Update);
    
    // Phase 1: Polling & Demand Registration
    schedule.add_systems(poll_demand_system);
    // Phase 2: Contention Resolution
    // Phase 3: Physical Flow & Latency Transit
    // Phase 4: Production Integration
    schedule.add_systems(process_converters_system.after(poll_demand_system));

    schedule.run(world);
    Ok(())
}
```

### 2.4 Layer Boundary Mapping
- **Domain Layer:** `crates/synthetic-core/src/network/...` (New module containing ECS definitions and systems)
- **Application Layer:** `crates/synthetic-core/src/simulation.rs` (Refactored to hold and tick a Bevy `World`)
- **Infrastructure Layer:** N/A (Zero I/O)
- **Presentation Layer:** N/A

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: ECS Integration & Data Contracts**
  - [x] Add `bevy_ecs` to `crates/synthetic-core/Cargo.toml`.
  - [x] Create `network.rs` module in `synthetic-core`.
  - [x] Define `ResourceAmount`, `Storage`, `Converter`, and transient components with `#[derive(Component)]`.
  - [x] Implement robust fixed-point math wrappers for `ResourceAmount`.
- [x] **Step 2: Core Systems & Pipelines**
  - [x] Implement `initialize_simulation_world` to setup the ECS environment and resources.
  - [x] Implement Bevy `System`s for Phase 1 & 2: Demand Polling and Contention Resolution.
  - [x] Implement Bevy `System`s for Phase 3: Edge transit latency (using ECS events or queue components).
  - [x] Implement Bevy `System`s for Phase 4: Converter execution.
  - [x] Organize systems into a strict `Schedule` to guarantee the Two-Pass Cycle execution order.
- [x] **Step 3: Application Adapter**
  - [x] Refactor `step_simulation` in `simulation.rs` to execute the Bevy `World` schedule instead of iterating over `Vec<OrbitalState>`.
  - [x] Ensure `SimulationStateDto` can successfully extract/serialize data from the Bevy `World` to pass back to Tauri.
- [x] **Step 4: Full Regression & Verification**
  - [x] Write hermetic unit tests proving mass-energy conservation across ticks within the ECS world.
  - [x] Write tests confirming the execution order of systems matches SSOT-001.
  - [x] Execute `cargo clippy` and `cargo test`.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- 100% unit test pass rate for the new `network` module.
- Mass conservation proven programmatically in ECS tests (total system inventory at `tick 0 == tick 100`).
- No floating-point types used in resource accounting.

### 4.2 Unit Test Targets
| Module / File | Test File | Key Scenarios Covered |
| :--- | :--- | :--- |
| `src/network.rs` | inline tests | Zero-sum conservation, system ordering constraints, dynamic component addition. |
| `tests/network_regression.rs` | integration suite | 100-tick mass conservation, exact latency queues, pro-rata contention remainder handling, Phase 3 transit arrival before Phase 4 converter, backpressure throttling. |

### 4.3 Verification Commands
```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- **Transient Component Storage:** Used `#[component(storage = "SparseSet")]` for `Relocating` to prevent archetype table moving and avoid cache fragmentation for long-lived nodes.
- **DTO Separation for Bevy Entities:** Bevy ECS `Entity` ids are mapped into u64 index identifiers in serialization DTOs (`SimulationStateDto`), maintaining a clear boundary across the Tauri IPC interface.

### 5.2 Lessons Learned & Follow-Up Tasks
- The Two-Pass Cycle mapped cleanly into ordered Bevy `Schedule` stages with `.chain()`.
- Pro-rata allocation requires explicit remainder distribution to avoid 1-micro-unit truncation leaks across competing consumer edges.
- Hermetic integration test proved absolute mass conservation over 100 continuous ticks with simultaneous transit latency and converter synthesis.




