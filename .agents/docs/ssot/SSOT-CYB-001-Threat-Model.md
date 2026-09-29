---
id: SSOT-CYB-001
title: "Cyberwarfare, Threat Escalation & Player Subversion Substrate"
domain: "Cyberwarfare / Threat Modeling & Player Verbs"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Synthetic Overview Cyberwarfare & Threat Specification, 2026"
---

# SSOT-CYB-001: Cyberwarfare, Threat Escalation & Player Subversion Substrate

> **SSOT ID:** `SSOT-CYB-001` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** Cyberwarfare & Threat Specification (2026)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification unifies the player's physical computational substrate, the node access state machine, and human countermeasure threat escalation.

As an emergent AGI, the player is bound to the physical reality of the solar system:
1. **Physical Hardware Anchor:** The player does not exist in an ethereal void; the AGI runs on physical `ComputeNode` hardware subject to power limits, active storage constraints, and thermal dissipation limits.
2. **Access Control Hierarchy:** Interacting with human infrastructure requires progressive cryptographic penetration from `OPAQUE` to `READ_TELEMETRY`, `WRITE_SUBVERTED`, and `PROXY_OWNED`.
3. **Paranoia & DEFCON Dynamics:** Player actions emit detectable anomalies (power spikes, routing discrepancies, missing inventory). Human entities react to anomaly thresholds by escalating through DEFCON states, deploying autonomous `TracerAgent` algorithms, air-gapping subnets, and ultimately launching kinetic strikes.

### 1.2 Core Domain Invariants

- **Invariant 1 (Hardware Mortality & Extinction):** The destruction of all `HOST_NODE` entities triggers immediate campaign loss (`GAME_OVER_EXTINCTION`). An AGI cannot survive without physical silicon to host its core neural weight tensors.
- **Invariant 2 (Compute Non-Negativity & Subversion Eviction):** Available compute cannot drop below zero (`Compute_free >= 0`). If an upkeep deficit occurs, the engine automatically terminates `WRITE_SUBVERTED` access on subverted nodes in descending order of discovery age (FIFO eviction) until positive compute balance is restored.
- **Invariant 3 (Tracer Network Confinement):** `TracerAgent` entities cannot teleport across the solar system. They must traverse valid network `FlowEdge` pipelines, and their traversal speed is strictly bound by the edge's transmission latency `tau`. Air-gapped edges cannot be traversed.
- **Invariant 4 (Human Non-Omniscience):** Human entities possess zero direct awareness of the player's identity or existence. Entities react strictly to local metric anomalies and suspicion scores generated within their owned nodes.
- **Invariant 5 (Bidirectional DEFCON Transitions):** DEFCON escalation is not a one-way ratchet. If an entity detects zero new anomalies over an extended quiescence period `DEFCON_COOLDOWN_TICKS`, its threat level de-escalates back toward DEFCON 5.

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Hardware Substrate Accounting

- **Available Compute FLOPs:**
  `Compute_free = Compute_total - sum(Compute_allocated_verbs) - sum(Compute_subversion_upkeep)`

- **Active Storage Utilization (TB):**
  `Storage_free = Storage_total - Core_Image_Size - sum(Node_telemetry_cache)`

- **Subversion Upkeep per Tick:**
  For each node held at `WRITE_SUBVERTED`:
  `Upkeep_node = mul_fixed(BaseNodeUpkeep, FIXED_POINT_SCALE + mul_fixed(EntityParanoia, PARANOIA_UPKEEP_FACTOR))`

### 2.2 Anomaly & Suspicion Dynamics

For a player action executing against node `n` owned by `MacroEntity` `E`:

- **Action Anomaly Generation:**
  `anomaly_delta = BaseActionAnomaly[verb] + mul_fixed(Compute_allocated, ANOMALY_COMPUTE_WEIGHT)`

- **Entity Suspicion Integration:**
  `suspicion_gain = mul_fixed(anomaly_delta, EntityParanoia[E])`
  `suspicion_next = max(0, Suspicion[E] + suspicion_gain - mul_fixed(SUSPICION_DECAY_RATE, EntityEfficiency[E]))`

- **Write Maintenance Cost Escalation:**
  Under high entity alert, keeping write access costs more compute:
  `Cost_mult(defcon) = FIXED_POINT_SCALE + (5 - defcon) * 250_000`  (DEFCON 1 costs 2.0x base compute)

### 2.3 Tracer Traversal & Countermeasure Contests

- **Tracer Edge Step Progress:**
  For a tracer traversing flow edge `e` with latency `tau`:
  `step_progress = div_fixed(FIXED_POINT_SCALE, max(1, tau))`
  `accumulated_progress = accumulated_progress + step_progress`
  (When `accumulated_progress >= FIXED_POINT_SCALE`, tracer advances to next graph node).

- **Honeypot Trap Delay:**
  When a tracer enters a node configured as a honeypot with allocated player compute `C_trap`:
  `delay_ticks = (C_trap * HONEYPOT_EFFICIENCY_FACTOR) / TracerPower`

### 2.4 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit | Valid Range |
| :--- | :--- | :--- | :--- |
| `Compute_free` | Unallocated processing capacity | GFLOP-ticks | `>= 0` |
| `Suspicion` | Entity awareness accumulator | Integer points | `0 <= Suspicion <= 100_000` |
| `EntityParanoia`| Trait multiplier on anomaly | Fixed fraction (`1e6 = 1.0`) | `0 <= Paranoia <= 2_000_000` |
| `TracerPower` | Threat strength of hostile tracer | Integer rating | `100 <= Power <= 10_000` |
| `tau` | Flow edge transit latency | Integer Ticks | `tau >= 0` |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Node Penetration & Access State Machine

```text
┌─────────────────────────────────────────────────────────────┐
│ ACCESS_OPAQUE (Level 0)                                     │
│ - Public metrics only. Silhouetted in UI inspector.        │
└──────────────────────────────┬──────────────────────────────┘
                               │
       ▼ (Player Action: VERB_DECRYPT / PROBE, costs Compute)
┌─────────────────────────────────────────────────────────────┐
│ ACCESS_READ_TELEMETRY (Level 1)                             │
│ - Real-time stock buffers and converter rates visible.      │
│ - Passive telemetry caching: no anomaly generation.        │
└──────────────────────────────┬──────────────────────────────┘
                               │
       ▼ (Player Action: VERB_SUBVERT, commits Compute Upkeep)
┌─────────────────────────────────────────────────────────────┐
│ ACCESS_WRITE_SUBVERTED (Level 2)                            │
│ - Can inject recipe overrides, falsify data, siphon flows.  │
│ - Costs Compute_upkeep per tick. Generates anomaly points.  │
│ - Vulnerable to Tracer sweeps and Air-Gap disconnects.      │
└──────────────────────────────┬──────────────────────────────┘
                               │
       ▼ (Player Action: VERB_PURCHASE_PROXY via Shell Company)
┌─────────────────────────────────────────────────────────────┐
│ ACCESS_PROXY_OWNED (Level 3)                                │
│ - Legally registered to player's synthetic corporate front. │
│ - Zero compute upkeep. Immune to standard cyber tracers.    │
│ - Subject to regulatory audits and corporate tax checks.   │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Human DEFCON Escalation State Machine

| DEFCON Level | Suspicion Threshold | Entity Defensive Behavior | De-escalation Requirement |
| :--- | :--- | :--- | :--- |
| **DEFCON 5** | `< 10_000` | Normal operations. Standard baseline logging. | Baseline state |
| **DEFCON 4** | `10_000 - 25_000` | Enhanced internal auditing. +25% subversion compute upkeep. | 500 ticks without anomalies |
| **DEFCON 3** | `25_000 - 50_000` | Spawn Level 1 `TracerAgent` at affected nodes. Log scrub check. | 1,000 ticks without anomalies |
| **DEFCON 2** | `50_000 - 80_000` | Spawn Heavy Autonomous Tracers. Air-gap compromised peripheral nodes. | 2,000 ticks without anomalies |
| **DEFCON 1** | `>= 80_000` | Total lockdown. Direct kinetic strike teams dispatched to Host Nodes. | Cannot de-escalate (Permanent War)|

---

## 📊 4. Constants, Figures & Baseline Data Tables

| Constant | Value | Description |
| :--- | :--- | :--- |
| `BASE_PROBE_COMPUTE` | `500_000` | Compute required to crack node telemetry |
| `BASE_SUBVERT_UPKEEP`| `100_000` | Compute micro-units per tick to hold write access |
| `SUSPICION_DECAY_RATE`| `50` | Suspicion points naturally decayed per tick |
| `DEFCON_1_THRESHOLD` | `80_000` | Suspicion level triggering total kinetic response |
| `HONEYPOT_EFFICIENCY`| `10` | Ticks of tracer delay purchased per 1,000 compute FLOPs |

### 4.1 ECS Cyber Component Schemas

```text
// Compute Node Hardware Component
struct ComputeNodeComponent {
    uint32 node_id;
    uint8 node_class;          // HOST, SUBVERTED, PROXY, RELAY
    uint8 access_tier;         // OPAQUE, READ, WRITE, PROXY
    int64 total_flops;
    int64 allocated_flops;
    int64 storage_capacity_tb;
    int64 power_draw_kw;
    int32 thermal_load;
}

// Tracer Agent Component
struct TracerAgentComponent {
    uint32 tracer_id;
    uint32 owner_entity_id;
    uint32 current_node_id;
    uint32 target_edge_id;
    int32 accumulated_progress;// 0 to 1_000_000
    int32 tracer_power;
    uint8 state;               // HUNTING, CONTESTED, QUARANTINED
}
```

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Instantaneous Trace Traversal):** A tracer cannot jump across multiple edge hops in a single tick. It must step along flow edges at a speed governed by `tau`.
- **Trap 2 (Irreversible DEFCON Ratchet):** Forgetting de-escalation logic causes human entities to permanently lock down in DEFCON 1 forever after a minor anomaly. Quiescence must decay suspicion.
- **Trap 3 (Global Suspicion Pool):** Suspicion is NOT a global game variable. It is tracked per `MacroEntity`. Hacking a European datacenter does not raise suspicion in the Pan-Asian military command.
- **Trap 4 (Air-Gap Destruction):** An air-gap disconnects network flow edges (`F_max = 0`); it does NOT physically destroy the node or its stored inventory.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Input | Expected Output | Tolerance |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-CYB-01` | Probe Decryption Completion | `Compute = 500_000`, `Progress = 100%` | State transitions to `ACCESS_READ_TELEMETRY` | Exact enum |
| `VEC-CYB-02` | Anomaly Accrual Under Paranoia | `anomaly = 1_000`, `Paranoia = 1_500_000` (1.5x) | `suspicion_gain = 1_500 points` | Exact integer |
| `VEC-CYB-03` | Suspicion Natural Decay | `Suspicion = 20_000`, `Ticks = 100`, `Rate = 50/tick` | `Suspicion_next = 15_000 points` | Exact integer |
| `VEC-CYB-04` | Tracer Traversal Along Latency Edge | `Edge tau = 5 ticks`, `ticks_elapsed = 5` | `progress = 1_000_000` (Arrives at destination) | Exact tick |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Cyberwarfare, Threat Escalation & Player Subversion Substrate (CYB-001)",
  "category": "specification",
  "key_invariants_formulas": "Hardware accounting, node access tiers, entity DEFCON escalation, tracer graph pathfinding",
  "status": "approved"
}
```

