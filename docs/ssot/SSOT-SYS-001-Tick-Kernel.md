---
id: SSOT-SYS-001
title: "Discrete Stock-and-Flow Simulation Kernel"
domain: "Game Engine / Discrete Simulation"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Jay W. Forrester, 'Industrial Dynamics', MIT Press, 1961"
  - "Donella H. Meadows, 'Thinking in Systems: A Primer', Chelsea Green Publishing, 2008"
---

# SSOT-SYS-001: Discrete Stock-and-Flow Simulation Kernel

> **SSOT ID:** `SSOT-SYS-001` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** Industrial Dynamics (Forrester 1961), Thinking in Systems (Meadows 2008)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the deterministic, fixed-point computational substrate for the entire simulation engine. All physical, economic, and industrial activities in the solar system compile down to a directed graph composed strictly of three foundational primitives:
1. **Stock Nodes:** State accumulators that store discrete quantities of physical or abstract resources.
2. **Flow Edges:** Directed rate pipelines that move resources between Stocks and Converters with discrete transit latency.
3. **Converter Nodes:** Transformation transformers that consume fixed ratios of input stocks to produce output stocks according to deterministic recipes, subject to operational efficiency and upkeep wear.

Execution proceeds in discrete, sequential simulation ticks. All calculations enforce bitwise determinism and strict mass/energy conservation across closed subgraphs.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict Mass Conservation):** In any closed subgraph with 100% transmission efficiency, the sum of resources across all stocks, converters, and in-flight transit queues at tick `t + 1` must exactly equal the sum at tick `t`. Resources cannot appear or vanish due to rounding errors.
- **Invariant 2 (Bitwise Determinism & Zero Float):** Floating-point arithmetic (`f32`, `f64`) is strictly prohibited in the simulation kernel. All quantities, rates, efficiencies, and fractions are represented as 64-bit signed integers (`int64`) scaled by `FIXED_POINT_SCALE = 1_000_000` (1 resource unit = 1,000,000 micro-units).
- **Invariant 3 (Atomic 4-Phase Tick Ordering):** Every simulation tick must execute the four discrete phases sequentially: Demand Registration -> Contention & Rationing -> Flow & Transit -> Converter Integration. Phases cannot be interleaved or reordered.
- **Invariant 4 (Order-Independent State Updates):** Within any phase, system evaluation order must not affect outcomes. State mutations are double-buffered or resolved via deterministic sorting keys (e.g., lowest Entity ID first).
- **Invariant 5 (Capacity & Non-Negativity Bounds):** Stock levels cannot be negative (`S >= 0`). Outflows from a stock cannot exceed its current quantity (`Outflow <= S`). Inflows cannot exceed available capacity (`Inflow <= S_max - S`).

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Formula Definitions

- **Fixed-Point Multiplication:**
  `mul_fixed(a, b) = (a * b) / FIXED_POINT_SCALE`

- **Fixed-Point Division:**
  `div_fixed(a, b) = (a * FIXED_POINT_SCALE) / b`

- **Largest-Remainder (Hamilton-Hare) Pro-Rata Allocation:**
  When total requested demand `D_total = sum(d_i)` exceeds available stock `S_avail`:
  1. Base integer allocation for consumer `i`:
     `alloc_base[i] = (d_i * S_avail) / D_total`
  2. Remainder score for consumer `i`:
     `rem_score[i] = (d_i * S_avail) % D_total`
  3. Total unallocated remainder:
     `R = S_avail - sum(alloc_base[i])`
  4. Sort consumers descending by `rem_score[i]`, breaking ties deterministically by `ConsumerID` ascending.
  5. Add 1 micro-unit to the top `R` consumers:
     `alloc_final[i] = alloc_base[i] + (1 if rank[i] < R else 0)`

- **Converter Recipe Execution:**
  For recipe requiring input ratios `req[j]` to produce output ratios `prod[k]`:
  1. Limiting batch count based on input availability:
     `batch_max = min_j(stock[j] / req[j])`
  2. Bounded by converter throughput rating `B_max`:
     `batches_executed = min(batch_max, B_max)`
  3. Input consumption:
     `consumed[j] = batches_executed * req[j]`
  4. Effective output generation scaled by operational health `H`:
     `effective_prod[k] = mul_fixed(batches_executed * prod[k], H)`

- **Structural Wear & Upkeep Degradation:**
  Let `U_provided` be upkeep delivered and `U_required` be rated upkeep per tick:
  1. If `U_provided >= U_required`:
     `H_next = min(FIXED_POINT_SCALE, H + WEAR_REPAIR_RATE)`
  2. If `U_provided < U_required`:
     `starvation_ratio = div_fixed(U_required - U_provided, U_required)`
     `decay_delta = mul_fixed(WEAR_BASE_DECAY, starvation_ratio)`
     `H_next = max(0, H - decay_delta)`

- **In-Flight Flow Latency:**
  When a resource packet of amount `Q` is dispatched along edge `e` at tick `t_current` with latency `tau`:
  `arrival_tick = t_current + tau`
  `received_amount = mul_fixed(Q, edge_efficiency)`
  `loss_amount = Q - received_amount`

### 2.2 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit | Valid Range |
| :--- | :--- | :--- | :--- |
| `S` | Current stock quantity | Micro-units (`1e-6` units) | `0 <= S <= S_max` |
| `S_max` | Maximum storage capacity | Micro-units (`1e-6` units) | `0 <= S_max <= 1e15` |
| `d_i` | Demanded quantity by consumer `i` | Micro-units (`1e-6` units) | `d_i >= 0` |
| `F_max` | Flow edge maximum capacity per tick | Micro-units per tick | `F_max >= 0` |
| `tau` | Flow edge transit latency | Integer Ticks | `tau >= 0` |
| `edge_efficiency`| Transmission efficiency | Fixed-point fraction (`1e6 = 1.0`) | `0 <= edge_efficiency <= 1_000_000` |
| `H` | Converter structural health | Fixed-point fraction (`1e6 = 1.0`) | `0 <= H <= 1_000_000` |
| `B_max` | Converter max batches per tick | Integer count | `B_max >= 0` |
| `U_required` | Required maintenance upkeep | Micro-units per tick | `U_required >= 0` |

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
[ TICK COMPLETE: Tick t -> t + 1 ]
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

| Parameter / Constant | Exact Integer Value | Meaning |
| :--- | :--- | :--- |
| `FIXED_POINT_SCALE` | `1_000_000` | 1 full unit = 1,000,000 micro-units |
| `WEAR_BASE_DECAY` | `5_000` | 0.005 (0.5% health loss per unmaintained tick) |
| `WEAR_REPAIR_RATE` | `10_000` | 0.010 (1.0% health restored per fully maintained tick) |
| `MAX_TRANSIT_QUEUE_DEPTH` | `16_384` | Max active in-flight packets per flow edge buffer |

### 4.1 Data-Oriented Component Memory Layout

```text
// Stock Component (Aligned contiguous array)
struct StockComponent {
    uint32 resource_id;
    int64 current_amount;       // Fixed-point micro-units
    int64 capacity;             // Fixed-point micro-units
    int64 reserved_amount;      // Reserved for pending outgoing flows
}

// Flow Edge Component
struct FlowEdgeComponent {
    uint32 edge_id;
    uint32 source_stock_id;
    uint32 dest_target_id;      // StockId or ConverterId
    int64 max_flow_per_tick;    // F_max micro-units
    uint32 latency_ticks;       // tau
    int32 efficiency_fixed;     // 0 to 1_000_000
}

// In-Flight Packet Entry
struct InFlightPacket {
    uint32 resource_id;
    int64 amount;               // Fixed-point micro-units
    uint64 arrival_tick;        // Absolute delivery tick
}

// Converter Component
struct ConverterComponent {
    uint32 recipe_id;
    int32 health_fixed;         // H: 0 to 1_000_000
    uint32 max_batches_per_tick;
    int64 upkeep_required;
    int64 upkeep_received;
}
```

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Floating-Point Drift):** Never cast amounts to `f32` or `f64`. A calculation like `100 / 3.0` will introduce floating point variance across different CPU architectures, breaking multiplayer lockstep and save determinism.
- **Trap 2 (Integer Truncation Loss in Rationing):** In integer division `(d_i * S) / D`, the sum of integer quotients is strictly less than or equal to `S`. Dropping the remainder destroys mass. Always use the Largest-Remainder method to distribute remaining units.
- **Trap 3 (Off-by-One Packet Delivery):** A packet dispatched at tick `10` with latency `tau = 3` arrives at tick `13`. It must be processed during Phase 3 of tick `13`, not tick `12` or `14`.
- **Trap 4 (In-Flight Destination Saturation):** Never check destination capacity only at departure. If multiple edges feed the same stock, or if consumption stalls, the destination may fill before in-flight packets arrive. The overflow policy must be deterministic.
- **Trap 5 (Converter Order Bias):** Converters executing in loop order `0, 1, 2...` will starve later converters of shared input stocks. Demand must be registered globally in Phase 1 and rationed in Phase 2 before any converter executes in Phase 4.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Inputs | Exact Expected Output | Tolerance |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-SYS-01` | Largest-Remainder Rationing | `S_avail = 100`, `Demands = [40, 40, 40]`, `ConsumerIDs = [1, 2, 3]` | `alloc = [34, 33, 33]`, `sum = 100` | Exact `0` error |
| `VEC-SYS-02` | Fixed-Point Transmission Loss | `Q = 10_000_000`, `efficiency = 950_000` (95%) | `received = 9_500_000`, `loss = 500_000` | Exact `0` error |
| `VEC-SYS-03` | Starvation Health Decay | `H = 1_000_000`, `U_required = 100_000`, `U_provided = 0`, `ticks = 10` | `H = 950_000` (`5_000` decay/tick) | Exact `0` error |
| `VEC-SYS-04` | Converter Health Throttling | `H = 750_000`, `B_max = 10`, `prod_ratio = 1_000_000` per batch | `batches = 10`, `prod = 7_500_000` | Exact `0` error |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Discrete Stock-and-Flow Simulation Kernel (SYS-001)",
  "category": "specification",
  "key_invariants_formulas": "Mass conservation, zero float, 4-phase cycle, Largest-Remainder pro-rata allocation",
  "status": "approved"
}
```

