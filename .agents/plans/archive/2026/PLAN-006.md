---
id: PLAN-006
title: "Pivot UI/Rendering Stack to Bevy"
status: completed
author: "Antigravity"
created: 2026-09-13
updated: 2026-09-13
completed_at: 2026-09-13
branch: "main"
---

# PLAN-006: Pivot UI/Rendering Stack to Bevy

> **Status:** `completed` | **Created:** 2026-09-13 | **Last Updated:** 2026-09-13
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The current UI/Rendering stack relies on Tauri, React, and Deck.gl. While functional, it incurs IPC serialization overhead and introduces an impedance mismatch between the pure Rust/ECS backend and the functional/reactive JavaScript frontend. To achieve the performance required for a solar-system scale, massive-entity simulation (millions of nodes) defined in SSOT-008, the stack needs to pivot entirely to a pure-Rust ECS environment using the **Bevy** game engine.

### 1.2 Core Objectives
- Complete removal of Tauri, React, Vite, Node, and Deck.gl dependencies (disregarding backwards compatibility).
- Re-architect the client as a pure Bevy application.
- Implement the baseline UI 'frame' as described by the user's mockup:
  - **Window 1:** Left sidebar
  - **Window 2:** Main View (2D top-down or 3D astro surface)
  - **Window 3:** Bottom bar for data/views
  - **Window 4:** Right sidebar
- Spawn these components as docked UI viewports within a single native OS window, establishing the absolute minimal structural frame.

### 1.3 Non-Goals & Exclusions
- We are *not* implementing the complex 3D globe math or H3 hex grid picking in this plan. This plan only establishes the minimal architectural frame and removes legacy code.
- We are *not* porting existing React component logic. This is a clean slate UI teardown.

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-008: UI & Visualization Architecture](../../../docs/ssot/SSOT-008-UI-Architecture.md) — *Defines the "Cyber-Tactical OS" widget/workspace interface and decoupled spatial instances.*

### 2.2 Workspace & Cargo Changes
The project will move from a mixed Node/Cargo workspace to a pure Cargo workspace.

```toml
# Expected root Cargo.toml workspace members
[workspace]
resolver = "2"
members = [
    "crates/synthetic-core",
    "crates/synthetic-client", # NEW: The Bevy Application
]
```

### 2.3 Layer Boundary Mapping
- **Simulation Layer:** `crates/synthetic-core/` (Existing ECS logic/simulation).
- **Presentation/Client Layer:** `crates/synthetic-client/` (New Bevy App, depends on core).

---

## 🛠️ 3. Implementation Steps

*Ordered checkbox checklist broken down into atomic, testable steps.*

- [x] **Step 1: Nuke Legacy Dependencies**
  - [x] Delete `package.json`, `package-lock.json`, `node_modules`, `vite.config.ts`, `tsconfig.*`, `eslint.config.js`.
  - [x] Delete the `src-tauri` directory.
  - [x] Delete the `src` and `public` directories (React code).
- [x] **Step 2: Initialize Bevy Client Workspace**
  - [x] Update root `Cargo.toml` to remove `src-tauri` and add `crates/synthetic-client`.
  - [x] Create `crates/synthetic-client/Cargo.toml` with dependencies on `bevy` (latest stable) and `synthetic-core`.
  - [x] Set up basic `crates/synthetic-client/src/main.rs`.
- [x] **Step 3: Implement the UI Frame (Docked Viewports)**
  - [x] Define the Bevy App setup system to spawn the 4 target zones within a single primary window.
  - [x] Set up Bevy UI nodes (`NodeBundle` / `Node`) with Flexbox layout to dock the Sidebars (1, 4) and Bottom bar (3) around the Main View (2).
  - [x] Assign distinct background colors to each UI node panel to visually verify the frame layout is rendering correctly.
- [x] **Step 4: Cleanup & Verification**
  - [x] Ensure `cargo build` and `cargo run --bin synthetic-client` compile successfully.
  - [x] Verify the 4 docked viewports appear correctly on screen upon launch.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- Zero JavaScript/TypeScript files remaining in the build pipeline.
- `cargo run` successfully launches a native Bevy application.
- The 4 specified docked viewports (Left, Main, Bottom, Right) render correctly within a single primary window.

### 4.2 Verification Commands
```bash
# Clean previous build artifacts
cargo clean

# Build and verify the entire workspace
cargo check --workspace

# Run the new client
cargo run -p synthetic-client
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- **Bevy 0.15 UI Components:** In Bevy 0.15, `NodeBundle` is replaced by the required components architecture. The implementation directly leverages `Node`, `BackgroundColor`, `BorderColor`, `Text`, `TextFont`, and `TextColor`.
- **Modular Spawn Hierarchy:** UI setup was decomposed into focused helper functions (`spawn_label`, `spawn_sidebar`, `spawn_view_panel`, `spawn_center_column`) keeping all functions under 30 lines and adhering strictly to AGENTS.md rules.

### 5.2 Lessons Learned & Follow-Up Tasks
- The pure Cargo workspace is drastically cleaner and faster to check/build than the dual Node/Tauri setup.
- Next follow-up (PLAN-007): Integrate Bevy 3D/2D camera rendering and spatial picking onto Panel 2 (Main View).

