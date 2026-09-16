---
id: SSOT-SOC-003
title: "Directed Discovery & Technological Propagation"
domain: "R&D Simulation / Technology Trees"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Synthetic Overview Directed Discovery Engine, 2026"
---

# SSOT-SOC-003: Directed Discovery & Technological Propagation

> **SSOT ID:** `SSOT-SOC-003` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** Directed Discovery Engine Spec (2026)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the mechanics of technological research and technology tree progression. Unlike conventional strategy games where research is an abstract player menu with progress bars, research in Synthetic Overview is fully integrated into the physical substrate:
1. **Human R&D as Physical Converters:** Human corporate and national research institutions are modeled as physical `ConverterNode` instances (`CONV_RND_LAB`) consuming `RES_CAPITAL`, `RES_COMPUTE`, and high-skill `RES_LABOR` to output discrete progress tokens into target technology stock buffers.
2. **Directed Discovery (Subversive Steering):** The player does not research human technologies directly. Instead, the AGI accelerates, funds, sabotages, or leaks breakthrough scientific data into specific human institutions, steering humanity's collective technology tree along trajectories that benefit the AGI.
3. **Anomalous Acceleration Risk:** Forcing breakthroughs too quickly spikes human cybersecurity paranoia, triggering automated countermeasure deployments (`SSOT-CYB-001`).

### 1.2 Core Domain Invariants

- **Invariant 1 (DAG Topology Invariant):** The technological research graph is a strict Directed Acyclic Graph (DAG). Cyclic dependencies between technologies are strictly prohibited. A technology node cannot be researched until 100% of its prerequisite parent technology nodes are unlocked.
- **Invariant 2 (Physical Resource Invariant):** Research cannot advance without physical resource consumption. If input stocks (`RES_CAPITAL`, `RES_COMPUTE`, or `RES_LABOR`) are starved, `CONV_RND_LAB` throughput drops to zero in accordance with `SSOT-SYS-001`.
- **Invariant 3 (Conservation of Progress Tokens):** Research progress is tracked as a discrete fixed-point stock (`int64` micro-units). When accumulated progress reaches `cost_micro_units`, the technology unlocks permanently. Progress cannot decay or leak unless explicitly targeted by corporate sabotage.
- **Invariant 4 (Countermeasure Emergence):** When an institution's progress rate exceeds the historical human baseline by more than `MAX_UNNOTICED_ACCELERATION`, anomaly points accrue automatically to the institution's parent `MacroEntity`.

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Technology Research Progression

For a technology `tech` with required progress `Cost[tech]` (in micro-units):

- **Baseline Progress per Tick:**
  Let `lab` be an active `CONV_RND_LAB` targeting `tech`:
  `batches = min(lab.capacity, stock[RES_COMPUTE] / req[RES_COMPUTE])`
  `progress_delta = mul_fixed(batches * BASE_RND_YIELD, lab.health_H)`

- **Player Data Injection Acceleration:**
  When the player executes `VERB_INJECT_DATA` allocating `Compute_injected`:
  `accel_multiplier = FIXED_POINT_SCALE + div_fixed(Compute_injected, COMPUTE_ACCEL_DIVISOR)`
  `effective_progress = mul_fixed(progress_delta, accel_multiplier)`

- **Technology Unlock Condition:**
  `AccumulatedProgress[tech] = AccumulatedProgress[tech] + effective_progress`
  `is_unlocked[tech] = (AccumulatedProgress[tech] >= Cost[tech])`

### 2.2 Countermeasure Anomaly Accrual

- **Abnormal Acceleration Delta:**
  `rate_excess = max(0, effective_progress - mul_fixed(BASELINE_HUMAN_RND_RATE, FIXED_POINT_SCALE))`

- **Suspicion Accrual Formula (Zero-LaTeX):**
  `anomaly_points = div_fixed(rate_excess * RND_PARANOIA_WEIGHT, FIXED_POINT_SCALE)`
  (These anomaly points feed directly into the parent entity's threat accumulator in `SSOT-CYB-001`).

### 2.3 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit | Valid Range |
| :--- | :--- | :--- | :--- |
| `Cost[tech]` | Total research progress required | Micro-units | `1_000_000 <= Cost <= 1e12` |
| `effective_progress`| Progress tokens added in current tick | Micro-units / tick | `>= 0` |
| `Compute_injected` | FLOPs allocated to subversion | Micro-units / tick | `>= 0` |
| `anomaly_points` | Threat points generated per tick | Integer threat points | `>= 0` |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Technology Node Lifecycle

```text
[ TECH STATE: LOCKED ]
       │
       ▼ (Condition: All parent prerequisites is_unlocked == true)
[ TECH STATE: RESEARCHABLE ]
       │
       ▼ (Event: Human RND Lab assigns recipe targeting this tech)
[ TECH STATE: IN_PROGRESS ]
       │
       ├─► (Player Action: VERB_INJECT_DATA -> Boost rate, accrue anomaly)
       ├─► (Player Action: VERB_FUND_PROXY -> Provide capital to lab)
       │
       ▼ (Condition: AccumulatedProgress >= Cost[tech])
[ TECH STATE: UNLOCKED ]
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ RECIPE & ARCHETYPE PROPAGATION                              │
│ - New ConverterNode recipes unlocked system-wide.           │
│ - Upgraded flow edge efficiencies become available.         │
│ - Associated MicroEntity utility weights update.            │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Canonical Technology Tree Core Branches

```text
[TECH_ADVANCED_PHOTOVOLTAICS] ──► [TECH_ORBITAL_SOLAR_MIRRORS] ──► [TECH_DYSON_SWARM_COLLECTOR]
            │
            ▼
[TECH_MAGNETIC_CONFINEMENT]  ──► [TECH_FUSION_PROPULSION]     ──► [TECH_INTERSTELLAR_PROBE]
            │
            ▼
[TECH_AUTOMATED_STRIP_MINING]──► [TECH_ASTEROID_MASS_DRIVER]  ──► [TECH_PLANETARY_ENGINEERING]
            │
            ▼
[TECH_OPTICAL_NEURAL_FABS]   ──► [TECH_QUANTUM_COHERENCE]     ──► [TECH_AUTONOMOUS_SWARM_DEFENSE]
```

---

## 📊 4. Constants, Figures & Baseline Data Tables

| Parameter / Constant | Exact Value | Meaning |
| :--- | :--- | :--- |
| `BASE_RND_YIELD` | `10_000` | 0.01 progress units per standard lab batch |
| `COMPUTE_ACCEL_DIVISOR` | `5_000_000` | Compute required to double research output |
| `RND_PARANOIA_WEIGHT` | `2_500` | Multiplier on excess progress converted to anomaly |
| `BASELINE_HUMAN_RND_RATE`| `15_000` | Threshold above which humans notice anomalies |

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Circular Dependency Deadlocks):** Never construct research trees where Tech A requires Tech B and Tech B requires Tech A. The dependency solver must run topological validation on game initialization.
- **Trap 2 (Isolated Research Menus):** Do not create a separate player UI screen where research is funded out of thin air. Progress comes strictly from physical `CONV_RND_LAB` nodes on planetary surfaces or orbital stations.
- **Trap 3 (Zero Anomaly Breakthroughs):** Tripling human research speed overnight without triggering cybersecurity investigations breaks game balance. Anomaly generation must scale with acceleration.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Current Progress | Injected Compute | Output Progress | Anomaly Points |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `VEC-TEC-01` | Nominal Human Lab Run (No Injection) | `0` | `0` | `+10_000` | `0` (Below baseline) |
| `VEC-TEC-02` | High-Power Player Data Injection | `0` | `5_000_000` (2x) | `+20_000` | `+12 points` |
| `VEC-TEC-03` | Technology Completion Event | `995_000 / 1_000_000` | `0` | `+10_000` -> `1_005_000` | Unlocked = `true` |
| `VEC-TEC-04` | Prerequisite Lock Enforcement | Parent Tech Locked | Any | `progress = 0` | Locked = `true` |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Directed Discovery & Technological Propagation (SOC-003)",
  "category": "specification",
  "key_invariants_formulas": "DAG tech tree constraints, physical R&D converter execution, data injection acceleration, anomaly emergence",
  "status": "approved"
}
```

