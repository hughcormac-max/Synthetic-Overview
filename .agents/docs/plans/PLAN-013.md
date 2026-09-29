---
id: PLAN-013
title: "Switch Surface Topography to Spherical Fibonacci Lattice"
status: completed
author: "Antigravity"
created: 2026-09-20
updated: 2026-09-20
completed_at: 2026-09-20
branch: "main"
---

# PLAN-013: Switch Surface Topography to Spherical Fibonacci Lattice

> **Status:** `completed` | **Created:** 2026-09-20 | **Completed:** 2026-09-20
> **Author:** Antigravity | **Branch:** main

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The current Poisson sphere surface generation uses quadratic dart-throwing (`O(N^2)`), which causes unacceptable latency and scaling bottlenecks when generating thousands of surface nodes for large bodies like Earth or Jupiter. We need to transition planetary surface node generation to the closed-form Spherical Fibonacci spiral lattice (`O(N)`), providing instant, deterministic point generation with uniform equal-area distribution.

### 1.2 Core Objectives
- Update `docs/ssot/SSOT-PHY-004-AstroNode-Architecture.md` to define the Spherical Fibonacci lattice as the Tier 0 domain specification.
- Replace `generate_poisson_surface_nodes` with `generate_fibonacci_surface_nodes(radius_m: f64, node_count: usize) -> Vec<SurfaceNode>` in `synthetic-core`.
- Provide helper function for density-based generation `generate_fibonacci_nodes_by_density(radius_m: f64, k_density: f64) -> Vec<SurfaceNode>`.
- Port Golden Test Vectors (`VEC-TOP-01` through `VEC-TOP-04`) to `synthetic-core` unit tests.
- Update `synthetic-client` ECS initialization to generate surface nodes via the Fibonacci algorithm.

### 1.3 Non-Goals & Exclusions
- Do not implement KD-Tree spatial acceleration in this plan (scheduled for the surface landing resolution phase).
- Do not modify existing orbital mechanics or JSON data schemas.

### 1.4 Invariants & Grounding
- **Zero-LaTeX Compliance:** Pure plain text/ASCII math notation (no `$`, `$$`, `\dot`, etc.).
- **Strict Layer Separation:** `synthetic-core` has zero external I/O or UI dependencies.
- **Downward Reference Rule:** SSOT documents must contain zero upward citations to code file paths.

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Authoritative Domain References
- [SSOT-PHY-004: Surface Topography](../ssot/SSOT-PHY-004-AstroNode-Architecture.md) — Spherical Fibonacci lattice formulas and golden test vectors.
- [SSOT-SYS-000: SI Units](../ssot/SSOT-SYS-000-SI-Units.md) — Canonical SI units and f64 determinism.

### 2.2 Public API / Service Signatures
```rust
// synthetic-core/src/astronomy/topography.rs
pub const GOLDEN_ANGLE_RAD: f64 = 2.399_963_229_728_653;

/// Generates Spherical Fibonacci surface nodes for an AstroNode based on its radius and exact node count.
pub fn generate_fibonacci_surface_nodes(radius_m: f64, node_count: usize) -> Vec<SurfaceNode>;

/// Generates Spherical Fibonacci surface nodes for an AstroNode based on its radius and global density factor.
pub fn generate_fibonacci_nodes_by_density(radius_m: f64, k_density: f64) -> Vec<SurfaceNode>;
```

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Update SSOT-PHY-004**
  - [x] Update `docs/ssot/SSOT-PHY-004-AstroNode-Architecture.md` with Spherical Fibonacci lattice formulas and golden test vectors.
  - [x] Assert: Zero-LaTeX compliance and downward reference integrity.

- [x] **Step 2: Implement Spherical Fibonacci Generator in synthetic-core**
  - [x] Implement `generate_fibonacci_surface_nodes` and `generate_fibonacci_nodes_by_density` in `crates/synthetic-core/src/astronomy/topography.rs`.
  - [x] Re-export in `crates/synthetic-core/src/astronomy/mod.rs`.
  - [x] Author unit tests covering Golden Test Vectors `VEC-TOP-01` to `VEC-TOP-04` and edge cases.
  - [x] Assert: Run `cargo test -p synthetic-core`.

- [x] **Step 3: Update Client ECS Spawning in synthetic-client**
  - [x] Update `crates/synthetic-client/src/systems/astronomy.rs` to invoke `generate_fibonacci_nodes_by_density`.
  - [x] Assert: Run `cargo test -p synthetic-client`.

- [x] **Step 4: Full Regression & Verification**
  - [x] Run `cargo check --workspace`, `cargo test --workspace`, and `cargo clippy --workspace`.
  - [x] Verify zero LaTeX usage.
  - [x] Update plan checklist.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [x] Target 1: Golden vectors VEC-TOP-01 through VEC-TOP-04 pass within tolerance.
- [x] Target 2: O(N) generation completes for N = 100,000 in < 5 milliseconds.
- [x] Target 3: Workspace tests and clippy pass cleanly.

### 4.2 Verification Commands
```bash
cargo check
cargo test
cargo clippy
```

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- None. SSOT-PHY-004 was updated prior to implementation to establish the Spherical Fibonacci lattice as the Tier 0 domain specification, strictly adhering to downward referencing.

### 5.2 Lessons Learned & Follow-Up Tasks
- The Spherical Fibonacci lattice provides immediate O(N) evaluation with near-optimal spherical area packing. 100,000 points generate instantaneously without memory allocations beyond the output vector.
- In future landing resolution systems, local spatial indexing (such as a 3D KD-tree) can be built directly over these Fibonacci nodes for logarithmic nearest-neighbor landing searches.
