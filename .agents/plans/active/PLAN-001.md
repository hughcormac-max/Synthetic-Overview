---
id: PLAN-001
title: "Define Tech Stack, Engine, and System Architecture"
status: in-progress
author: "Antigravity"
created: 2026-09-12
updated: 2026-09-12
branch: "main"
---

# PLAN-001: Define Tech Stack, Engine, and System Architecture

> **Status:** `in-progress` | **Created:** 2026-09-12 | **Last Updated:** 2026-09-12
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The project requires a concrete decision on the core programming language, game engine (or rendering framework), and system architecture to fulfill the requirements of a solar-system scale, AGI-driven grand strategy game. The simulation must handle millions of autonomous nodes asynchronously with strict Entity Component System (ECS) and Data-Oriented Design (DOD) principles, avoiding floating-point drift.

### 1.2 Core Objectives
- Select the primary programming language: **Rust** for the backend simulation brain, maximizing 12-core multithreading and utilizing the full 64GB of RAM without browser constraints.
- Select the architecture framework: **Tauri**, providing a native desktop OS footprint with a lightweight webview frontend.
- Select the rendering/UI engine: **WebGPU** via **Deck.gl and CesiumJS** running inside the Tauri webview, paired with React for TUI-style Cyber-OS widgets.
- Implement a **Discrete Global Grid System (DGGS)** for planetary surfaces using Uber H3 (a Goldberg polyhedron/subdivided icosahedron consisting of hexagons and exactly 12 pentagons), pushing spatial computations to the GPU via compute shaders.
- Establish the hierarchical coordinate systems (Local Barycentric Origins) and strict deterministic integers/fixed-point math in Rust to prevent floating-point drift.
- Define the project scaffolding, CI/CD, and build tooling.

### 1.3 Non-Goals & Exclusions
- Implementation of game mechanics or SSOT logic (deferred to subsequent plans).
- Authoring of gameplay content.

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-001: Simulation Architecture](../../../docs/ssot/SSOT-001-Simulation-Architecture.md)
- [SSOT-008: UI & Visualization Architecture](../../../docs/ssot/SSOT-008-UI-Architecture.md)

### 2.2 Domain Types & Schemas
```rust
// Core deterministic state models managed in Rust
#[derive(Debug, Clone, Copy)]
pub struct OrbitalState {
    pub entity_id: u64,
    pub barycenter_id: u64,
    // Fixed-point or 64-bit precision parameters
    pub true_anomaly: f64, 
    pub semi_major_axis: f64,
}
```

### 2.3 Public API / Service Signatures
```rust
// Tauri IPC Command Signature bridging Rust brain and React/WebGPU Eyes
#[tauri::command]
fn fetch_simulation_tick() -> Result<SimulationStateDto, DomainError> {
    // Returns serialized state to the webview
}
```

### 2.4 Layer Boundary Mapping
- **Domain Layer (Tier 0):** Pure Rust ECS simulation utilizing DOD. Absolute zero I/O or UI logic.
- **Application Layer:** Rust systems coordinating the game loops, time warp, and event resolution.
- **Infrastructure Layer:** Tauri native file I/O, SQLite/local database persistence.
- **Presentation Layer (Tauri Webview):** React, Deck.gl (WebGPU), and CesiumJS executing H3 DGGS rendering.

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Stack Decision & Validation**
  - [x] Finalize Language (Rust), Framework (Tauri), and ECS approach.
  - [x] Finalize UI/Rendering approach (WebGPU + Deck.gl + React).
- [ ] **Step 2: Project Initialization**
  - [ ] Scaffold the Tauri workspace (`create-tauri-app`).
  - [ ] Configure `rust-toolchain.toml`, `clippy` linting rules, and strict TypeScript configs for the frontend.
- [ ] **Step 3: CI/CD & Testing Setup**
  - [ ] Setup `cargo test` framework and Vitest for frontend regression.
  - [ ] Implement a basic Rust fixed-point/deterministic math test to prove no floating-point drift.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- A successfully compiling "Hello World" simulation project.
- CI pipelines passing type checks, linting, and basic tests.
- Mathematical determinism proven through a basic test case.

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- *[None logged during drafting.]*

### 5.2 Lessons Learned & Follow-Up Tasks
- *[None logged during drafting.]*
