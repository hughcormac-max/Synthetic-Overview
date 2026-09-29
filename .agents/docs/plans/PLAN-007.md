---
id: PLAN-007
title: "Restructure and Standardize SSOT Library"
status: completed
author: "Antigravity"
created: 2026-09-13
updated: 2026-09-16
completed_at: 2026-09-16
branch: "main"
---

# PLAN-007: Restructure and Standardize SSOT Library

> **Status:** `completed` | **Created:** 2026-09-13 | **Completed:** 2026-09-16
> **Author:** Antigravity

---

## 🎯 1. Intent & Boundaries

### 1.1 Problem Statement
The current `.agents/docs/ssot/` library consists of 8 documents that function as high-level narrative game-design pitches rather than authoritative, deterministic Tier 0 specifications. AI coding agents attempting to implement systems from these documents are forced to hallucinate memory layouts (ECS), tick-loop integrations, and fixed-point mathematical formulas. Furthermore, the documents suffer from domain overlap (e.g., cyberwarfare mechanics split across SSOT-002 and SSOT-006), terminology collisions (meso-entity presence), and architectural misalignment regarding spatial grids (we will pivot away from H3 and standardize on Fibonacci spheres).

### 1.2 Core Objectives
- Refactor the flat `SSOT-00X` namespace into a Domain-Prefixed Taxonomy (e.g., `SSOT-SYS-*`, `SSOT-PHY-*`, `SSOT-CYB-*`) to prevent context fragmentation for specialized agents.
- Enforce the standard `.agents/skills/define-ssot/resources/template.md` across all documents, mandating explicit Invariants, Variable Dictionaries, State Machines, ECS Struct Layouts, and Golden Test Vectors.
- Resolve architectural contradictions (e.g., unify the `converter-node` taxonomy, standardize on Fibonacci spheres over H3 grids for surface node distribution, and consolidate overlapping cyber-warfare capabilities).
- Provide clean, ASCII-formatted, Zero-LaTeX mathematical formulas for all system dynamics (Utility AI scoring, Hohmann transfer latency, tracer traversal, fixed-point rationing).

### 1.3 Non-Goals & Exclusions
- This plan *does not* implement any Rust or TypeScript game code.
- This plan *does not* alter existing architecture code in `synthetic-core` or `synthetic-client`, it strictly aligns the documentation to reflect reality and serve as a hallucination-free roadmap for future features.

---

## 📐 2. Technical Contracts & Interfaces

### 2.1 Proposed New Taxonomy
The legacy `SSOT-001` through `SSOT-008` files will be superseded and replaced by the following modular, domain-prefixed documents:

**System & Core Engine (`SSOT-SYS-*`)**
- `SSOT-SYS-001-Tick-Kernel.md` (Formerly SSOT-001) - 4-Phase tick loop, fixed-point rationing algorithms, structural wear, ECS primitives.
- `SSOT-SYS-002-Glossary.md` (Formerly SSOT-007) - Root terminology, node/entity definitions, unifying taxonomy.

**Physical & Spatial (`SSOT-PHY-*`)**
- `SSOT-PHY-001-Astrodynamics.md` (Formerly SSOT-003a) - Coplanar 2D orbits, Fibonacci spheres for surface node distribution, coordinate transformations.
- `SSOT-PHY-002-Logistics.md` (Formerly SSOT-003b) - Transit queues, dynamic `tau` latency, gateway converter interfaces.
- `SSOT-PHY-003-Manufacturing.md` - Specific catalogs of converter node types, their names, processes, inputs, and outputs.

**Sociology & Entities (`SSOT-SOC-*`)**
- `SSOT-SOC-001-Utility-AI.md` (Formerly SSOT-005) - Micro/Meso/Macro entity hierarchy, drive equations, trait scoring, time-sliced evaluation pipelines.
- `SSOT-SOC-002-Progression.md` (Formerly SSOT-004a) - Narrative phases, discrete state machine gates, victory/defeat predicates.
- `SSOT-SOC-003-Tech-Tree.md` (Formerly SSOT-004b) - Directed discovery, R&D converter recipes, countermeasure emergence.

**Cyberwarfare & Player Interactions (`SSOT-CYB-*`)**
- `SSOT-CYB-001-Threat-Model.md` (Merges SSOT-002 & SSOT-006) - Suspicion decay formulas, node access tiers (Opaque -> Proxy), tracer traversal, anomaly accumulation.

**Presentation & UI (`SSOT-UIX-*`)**
- `SSOT-UIX-001-Presentation.md` (Formerly SSOT-008) - Bevy 4-pane docked UI layout, semantic zoom aggregation thresholds, command terminal EBNF grammar.

### 2.2 Template Enforcement Requirements
Every single document *must* contain:
1. `yaml` Frontmatter & Metadata
2. Explicit State Transitions (Truth tables, enum definitions)
3. Data-Oriented Memory Layout (ECS structs, AoS vs SoA)
4. Anti-Hallucination Traps
5. Golden Test Vectors

---

## 🛠️ 3. Implementation Steps

- [x] **Step 1: Scaffold New Directory & Index**
  - [x] Initialize the new SSOT files using the `define-ssot` skill templates.
  - [x] Update `docs/ssot/INDEX.md` to reflect the domain-prefixed taxonomy and dependency graph.
- [x] **Step 2: Rewrite Core Engine (SYS & PHY)**
  - [x] Write `SSOT-SYS-001` (Tick Kernel) ensuring explicit formulas for largest-remainder pro-rata allocation.
  - [x] Write `SSOT-SYS-002` (Glossary) reconciling entity nomenclature.
  - [x] Write `SSOT-PHY-001` and `002`, embedding fixed-point math and confirming Fibonacci sphere indexing formulas.
  - [x] Write `SSOT-PHY-003` (Manufacturing) detailing explicit converter node types, processes, inputs, and outputs.
- [x] **Step 3: Rewrite Socio-Economic (SOC)**
  - [x] Write `SSOT-SOC-001` (Utility AI) with explicit dot-product/utility formulas and time-slicing logic.
  - [x] Write `SSOT-SOC-002` and `003`, converting narrative text into boolean state gates.
- [x] **Step 4: Rewrite Cyber & UI (CYB & UIX)**
  - [x] Write `SSOT-CYB-001`, merging hardware upkeep costs with anomaly escalation formulas.
  - [x] Write `SSOT-UIX-001`, providing semantic zoom integer thresholds and terminal grammar.
- [x] **Step 5: Deprecation & Archival**
  - [x] Remove legacy `SSOT-001` through `SSOT-008`.
  - [x] Ensure `AGENTS.md` and any referencing plans are updated to point to the new namespace.

---

## 🧪 4. Verification & Criteria

### 4.1 Measurable Benchmarks & Targets
- [x] Target 1: Zero LaTeX macros across all generated files.
- [x] Target 2: All 10 new SSOT documents successfully pass a schema/template audit (contains Invariants, Data Structs, Formulas, Test Vectors).
- [x] Target 3: `INDEX.md` correctly maps the hierarchy without dead links.

---

## 📝 5. Deviations & Retrospective (Post-Implementation)

### 5.1 Architectural Deviations
- **Fibonacci Sphere Pivot:** Standardized strictly on spherical Fibonacci spiral lattices for surface node distribution and KD-Tree query indexing across `SSOT-PHY-001` and `SSOT-UIX-001`, removing all legacy references to H3 hexagonal grids per user directive.
- **Dedicated Manufacturing Specification:** Added `SSOT-PHY-003-Manufacturing.md` to cleanly separate concrete converter recipes and thermodynamic waste heat emission from the low-level discrete tick engine kernel in `SSOT-SYS-001`.
- **Subversion & Threat Unification:** Merged player hardware budgeting (formerly SSOT-002) and cyber threat escalation (formerly SSOT-006) into `SSOT-CYB-001-Threat-Model.md` to prevent duplication of node penetration states and compute allocation rules.

### 5.2 Lessons Learned & Follow-Up Tasks
- The domain-prefixed taxonomy (`SYS`, `PHY`, `SOC`, `CYB`, `UIX`) provides immediate context bounding for autonomous coding agents, allowing an agent to ingest only its relevant subsystem spec.
- Future code implementation plans should cite these new Tier 0 SSOT specifications and assert against the golden test vectors provided in each document.

