---
id: SSOT-SYS-001
title: "Discrete Stock-and-Flow Simulation Kernel"
domain: "Game Engine / Discrete Simulation"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-17
sources:
  - "Jay W. Forrester, 'Industrial Dynamics', MIT Press, 1961"
  - "Donella H. Meadows, 'Thinking in Systems: A Primer', Chelsea Green Publishing, 2008"
---

# SSOT-SYS-001: Discrete Stock-and-Flow Simulation Kernel

> **SSOT ID:** `SSOT-SYS-001` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-17
> **Authoritative Sources:** Industrial Dynamics (Forrester 1961), Thinking in Systems (Meadows 2008)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the deterministic, fixed-point computational substrate and chronological clock for the entire simulation engine. All physical, economic, and industrial activities in the solar system compile down to a directed graph composed strictly of three foundational primitives:
1. **Stock Nodes.** State accumulators that store discrete quantities of physical or abstract resources.
2. **Flow Edges.** Directed rate pipelines that move resources between Stocks and Converters with discrete transit latency.
3. **Converter Nodes.** Transformation transformers that consume fixed ratios of input stocks to produce output stocks according to deterministic recipes, subject to operational efficiency and upkeep wear.

Execution proceeds in discrete, sequential simulation ticks. The simulation clock increments in simulation time from a strict epoch start date, regulated by discrete speed control multipliers. All calculations enforce bitwise determinism and strict mass/energy conservation across closed subgraphs.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict Mass Conservation).** In any closed subgraph with 100% transmission efficiency, the sum of resources across all stocks, converters, and in-flight transit queues at tick `t + 1` must exactly equal the sum at tick `t`. Resources cannot appear or vanish due to rounding errors.
- **Invariant 2 (f64 Determinism).** The simulation kernel uses 64-bit IEEE-754 floating-point arithmetic (`f64`). To prevent cross-platform floating-point drift, execution must occur in a strict WebAssembly environment or utilize deterministic soft-float libraries. Hardware FMA and hardware transcendental instructions are strictly prohibited.
- **Invariant 3 (Atomic 4-Phase Tick Ordering).** Every simulation tick must execute the four discrete phases sequentially: Demand Registration -> Contention & Rationing -> Flow & Transit -> Converter Integration. Phases cannot be interleaved or reordered.
- **Invariant 4 (Order-Independent State Updates).** Within any phase, system evaluation order must not affect outcomes. State mutations are double-buffered or resolved via deterministic sorting keys.
- **Invariant 5 (Capacity & Non-Negativity Bounds).** Stock levels cannot be negative (`S >= 0`). Outflows from a stock cannot exceed its current quantity (`Outflow <= S`). Inflows cannot exceed available capacity (`Inflow <= S_max - S`).

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Formula Definitions

- **Pro-Rata Allocation.**
  When total requested demand `D_total = sum(d_i)` exceeds available stock `S_avail`, allocate continuously:
  `alloc[i] = d_i * (S_avail / D_total)`
- **Converter Recipe Execution.**
  For recipe requiring input ratios `req[j]` to produce output ratios `prod[k]`:
  1. Limiting batch count based on input availability:
     `batch_max = min_j(stock[j] / req[j])`
  2. Bounded by converter throughput rating `B_max`:
     `batches_executed = min(batch_max, B_max)`
  3. Input consumption:
     `consumed[j] = batches_executed * req[j]`
  4. Effective output generation scaled by operational health `H`:
     `effective_prod[k] = batches_executed * prod[k] * H`

- **Structural Wear & Upkeep Degradation.**
  Let `U_provided` be upkeep delivered and `U_required` be rated upkeep per tick:
  1. If `U_provided >= U_required`:
     `H_next = min(1.0, H + WEAR_REPAIR_RATE)`
  2. If `U_provided < U_required`:
     `starvation_ratio = (U_required - U_provided) / U_required`
     `decay_delta = WEAR_BASE_DECAY * starvation_ratio`
     `H_next = max(0.0, H - decay_delta)`

- **In-Flight Flow Latency.**
  When a resource packet of amount `Q` is dispatched along edge `e` at tick `t_current` with latency `tau`:
  `arrival_tick = t_current + tau`
  `received_amount = Q * edge_efficiency`
  `loss_amount = Q - received_amount`

### 2.2 Simulation Time & Speed Control

The kernel maintains a global simulation clock representing real-world dates and times.
- **Epoch Start Date.** The simulation initializes exactly at `SIM_EPOCH_UTC` (`2026-10-14T14:00:00Z`).
- **Clock Tracking.** Simulation time `sim_time` is tracked internally as a strict Unix timestamp (`u64` seconds since 1970-01-01) initialized to the epoch.
- **Speed Multiplier.** The user controls the passage of simulation time via a defined set of acceleration gears, representing simulation time elapsed per real-time second.

```text
enum SimSpeed {
    REALTIME       = 1,         // 1s/s
    MINUTE_PER_SEC = 60,        // 1min/s
    HOUR_PER_SEC   = 3600,      // 1hr/s
    DAY_PER_SEC    = 86400,     // 1d/s
    MONTH_PER_SEC  = 2592000    // 1mon/s
}
```

If the engine runs a fixed simulation tick of `dt = 1s`, faster speeds dispatch proportionally more ticks per real-time second. If `dt` is variable, the kernel integrates the time delta directly, scaling all rate-based constants (flow capacity, converter batches) by `dt`.

### 2.3 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit | Valid Range |
| :--- | :--- | :--- | :--- |
| `S` | Current stock quantity | Float unit (`f64`) | `0.0 <= S <= S_max` |
| `S_max` | Maximum storage capacity | Float unit (`f64`) | `0.0 <= S_max` |
| `d_i` | Demanded quantity by consumer `i` | Float unit (`f64`) | `d_i >= 0.0` |
| `F_max` | Flow edge maximum capacity per tick | Float unit per tick (`f64`) | `F_max >= 0.0` |
| `tau` | Flow edge transit latency | Integer Ticks | `tau >= 0` |
| `edge_efficiency`| Transmission efficiency | Float fraction (`f64`) | `0.0 <= edge_efficiency <= 1.0` |
| `H` | Converter structural health | Float fraction (`f64`) | `0.0 <= H <= 1.0` |
| `B_max` | Converter max batches per tick | Float count (`f64`) | `B_max >= 0.0` |
| `U_required` | Required maintenance upkeep | Float unit per tick (`f64`) | `U_required >= 0.0` |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 4-Phase Tick Execution Cycle

```text
[ TICK START: Tick t ]
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ PHASE 1: DEMAND REGISTRATION & POLLING                      │
│ - Consumers query target stocks and register demand d_i.    │
│ - Converters register input requirements for upcoming batch.│
└─────────────────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ PHASE 2: CONTENTION & RATIONING                             │
│ - Evaluate D_total vs S_avail for all contested stocks.     │
│ - If D_total <= S_avail: Allocate 100% of demand.           │
│ - If D_total > S_avail: Run Largest-Remainder Pro-Rata.     │
│ - Deduct allocated quantities from source stocks.           │
└─────────────────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ PHASE 3: FLOW & DYNAMIC TRANSIT                             │
│ - Enqueue allocated packets into edge transit queues.       │
│ - Scan in-flight packets: if arrival_tick == t:             │
│     * Deliver received_amount to destination stock.         │
│     * Route loss_amount to designated waste stock.          │
└─────────────────────────────────────────────────────────────┘
       │
       ▼
┌─────────────────────────────────────────────────────────────┐
│ PHASE 4: CONVERTER INTEGRATION & UPKEEP                     │
│ - Execute batches for converters with complete inputs.      │
│ - Calculate output production scaled by health H.           │
│ - Apply maintenance wear / repair based on upkeep delivery. │
└─────────────────────────────────────────────────────────────┘
       │
       ▼
[ TICK COMPLETE: Tick t -> t + 1 (Time += dt) ]
```

### 3.2 Decision Rules & Overflow Truth Table

| Stock Space Available | Packet Arriving | Destination Buffer Policy | Action Taken |
| :--- | :--- | :--- | :--- |
| `space >= received_amount` | `true` | Any | Normal delivery: `S = S + received_amount` |
| `0 < space < received_amount` | `true` | `TRUNCATE_TO_WASTE` | Accept `space`, route remaining to local waste heap |
| `space == 0` | `true` | `SPILL_HAZARD` | Emit Environmental Hazard event, dump 100% to waste |
| Any | `false` | Any | No state mutation |

---

## 📊 4. Constants, Figures & Baseline Data Tables

| Parameter / Constant | Exact Value | Meaning |
| :--- | :--- | :--- |
| `SIM_EPOCH_UTC` | `"2026-10-14T14:00:00Z"` | Global initialization timestamp |
| `WEAR_BASE_DECAY` | `0.005` | 0.5% health loss per unmaintained tick |
| `WEAR_REPAIR_RATE` | `0.010` | 1.0% health restored per fully maintained tick |
| `MAX_TRANSIT_QUEUE_DEPTH` | `16_384` | Max active in-flight packets per flow edge buffer |

### 4.1 Data-Oriented Component Memory Layout

```text
// Stock Component (Aligned contiguous array)
struct StockComponent {
    uint32 resource_id;
    f64 current_amount;
    f64 capacity;
    f64 reserved_amount;
}

// Flow Edge Component
struct FlowEdgeComponent {
    uint32 edge_id;
    uint32 source_stock_id;
    uint32 dest_target_id;      // StockId or ConverterId
    f64 max_flow_per_tick;      // F_max
    uint32 latency_ticks;       // tau
    f64 efficiency;             // 0.0 to 1.0
}

// In-Flight Packet Entry
struct InFlightPacket {
    uint32 resource_id;
    f64 amount;
    uint64 arrival_tick;        // Absolute delivery tick
}

// Converter Component
struct ConverterComponent {
    uint32 recipe_id;
    f64 health;                 // H: 0.0 to 1.0
    f64 max_batches_per_tick;
    f64 upkeep_required;
    f64 upkeep_received;
}
```

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Hardware Floating-Point Drift).** While the state uses `f64`, you must never use non-deterministic hardware math (like `f64::cos`, `f64::exp`) or rely on platform-dependent FMA instructions.
- **Trap 2 (Fractional Allocation Loss in Rationing).** Always sum demands deterministically sorted by ConsumerID before rationing. Order of summation matters for float parity.
- **Trap 3 (Off-by-One Packet Delivery).** A packet dispatched at tick `10` with latency `tau = 3` arrives at tick `13`. It must be processed during Phase 3 of tick `13`.
- **Trap 4 (In-Flight Destination Saturation).** The overflow policy must be deterministic if the destination fills before in-flight packets arrive.
- **Trap 5 (Converter Order Bias).** Converters executing in loop order `0, 1, 2...` will starve later converters. Demand must be registered globally in Phase 1 and rationed in Phase 2 before any converter executes in Phase 4.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Inputs | Exact Expected Output | Tolerance |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-SYS-01` | Largest-Remainder Rationing | `S_avail = 100`, `Demands = [40, 40, 40]`, `ConsumerIDs = [1, 2, 3]` | `alloc = [34, 33, 33]`, `sum = 100` | Exact `0` error |
| `VEC-SYS-02` | Fixed-Point Transmission Loss | `Q = 10_000_000`, `efficiency = 950_000` (95%) | `received = 9_500_000`, `loss = 500_000` | Exact `0` error |
| `VEC-SYS-03` | Starvation Health Decay | `H = 1_000_000`, `U_required = 100_000`, `U_provided = 0`, `ticks = 10` | `H = 950_000` (`5_000` decay/tick) | Exact `0` error |
| `VEC-SYS-04` | Converter Health Throttling | `H = 750_000`, `B_max = 10`, `prod_ratio = 1_000_000` per batch | `batches = 10`, `prod = 7_500_000` | Exact `0` error |
| `VEC-SYS-05` | Simulation Speed Multiplier | `dt = 1s`, `speed = MONTH_PER_SEC`, `elapsed_real = 1s` | `sim_time` advances `2592000s` | Exact `0` error |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Discrete Stock-and-Flow Simulation Kernel (SYS-001)",
  "category": "specification",
  "key_invariants_formulas": "Mass conservation, zero float, 4-phase cycle, UTC ISO clock, variable sim speed multipliers",
  "status": "approved"
}
```
