---
id: SSOT-SOC-001
title: "Autonomous Utility AI & Sociopolitical Decision Engine"
domain: "Sociopolitical Simulation / Agent AI"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Dave Mark, 'Behavioral Mathematics for Game AI', Course Technology PTR, 2009"
  - "Synthetic Overview Sociopolitical Behavioral Model, 2026"
---

# SSOT-SOC-001: Autonomous Utility AI & Sociopolitical Decision Engine

> **SSOT ID:** `SSOT-SOC-001` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** Behavioral Mathematics for Game AI (Mark 2009), Sociopolitical Behavioral Model (2026)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the deterministic behavioral engine governing autonomous `MicroEntity` agents (e.g., corporate executives, station governors, ministers, and fleet admirals). In accordance with `SSOT-SYS-002`, high-level entities make decisions through independent Utility AI evaluations rather than scripted behavior trees or black-box neural networks.

Each `MicroEntity` possesses:
1. Fixed **Personality Traits** (`Greed`, `RiskTolerance`, `Paranoia`, `Ideology`).
2. Dynamic **Tracked Drives** reflecting the health and deficits of their parent `MacroEntity` (Treasury, Market Share, Grievance, Sovereign Security).
3. Evaluated **Action Catalogs** whose utility scores are computed via deterministic fixed-point dot products.

To maintain 60 FPS / sub-millisecond tick budgets across tens of thousands of active agents, execution is scheduled via a **Time-Sliced DoD Ring Buffer**.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict Determinism & Zero Float):** Utility scoring must evaluate strictly using 64-bit integer fixed-point arithmetic (`int64`). Floating-point curves (`f32::exp`, `f64::pow`) are strictly prohibited; non-linear curves must use integer lookup tables.
- **Invariant 2 (Strict Tie-Breaking):** If two or more available actions evaluate to the exact same total utility score, the engine must break the tie deterministically by selecting the action with the lowest numeric `ActionID`. Random selection is strictly forbidden.
- **Invariant 3 (Phase 1 Integration):** Decisions chosen by `MicroEntity` agents do not mutate world stocks directly or instantly. They register demands, submit trade bids, or inject recipe adjustments strictly during **Phase 1 (Demand Registration)** of `SSOT-SYS-001`.
- **Invariant 4 (Time-Slicing Invariance):** The time-slicing partition must guarantee that every agent executes an evaluation cycle exactly once every `TOTAL_SLICES` ticks. An agent's action cooldown decrements every tick, regardless of slice assignment.

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Utility AI Scoring Pipeline

For a `MicroEntity` evaluating candidate action `a` at tick `t`:

- **Drive Urgency Calculation:**
  For each tracked abstract stock drive `d`:
  `deficit = TargetStock[d] - CurrentStock[d]`
  `Urgency[d] = clamp_int64((deficit * 1_000) / max(1, Capacity[d]), -1_000, 1_000)`

- **Drive Alignment Score:**
  `DriveScore[a] = sum_d(ActionDriveImpact[a, d] * Urgency[d])`

- **Trait Alignment Score:**
  For personality traits `t in [GREED, RISK, PARANOIA, IDEOLOGY]`:
  `TraitScore[a] = sum_t(AgentTraits[t] * ActionTraitVector[a, t]) / 1_000`

- **Clout & Subversion Modifiers:**
  Let `Trust` and `Leverage` be the player's directed clout vectors against this agent (from `SSOT-CYB-001`):
  `CloutModifier[a] = (Trust * ActionPlayerAlignment[a] + Leverage * ActionCoercionRating[a]) / 1_000`

- **Total Action Utility:**
  `TotalScore[a] = DriveScore[a] + TraitScore[a] + CloutModifier[a]`

### 2.2 Time-Sliced Evaluation Scheduling

Given total slices `TOTAL_SLICES = 100`:
- **Active Evaluation Predicate:**
  `is_active_this_tick = (agent_id % TOTAL_SLICES) == (current_tick % TOTAL_SLICES)`
- **Evaluation Rule:**
  If `is_active_this_tick == true` and `cooldown_ticks == 0`:
  1. Scan all actions where `preconditions_met(a) == true`.
  2. Evaluate `TotalScore[a]` for each.
  3. Select action `a_best = argmax(TotalScore[a])`.
  4. Submit action intent to Phase 1 buffer.
  5. Reset `cooldown_ticks = ActionBaseCooldown[a_best]`.

### 2.3 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit | Valid Range |
| :--- | :--- | :--- | :--- |
| `AgentTraits` | Personality trait scores | Fixed integer units | `-1_000 <= trait <= 1_000` |
| `Urgency` | Deficit pressure for drive `d` | Fixed integer units | `-1_000 <= Urgency <= 1_000` |
| `ActionImpact` | Predicted impact on drive `d` | Fixed integer units | `-1_000 <= Impact <= 1_000` |
| `TotalScore` | Final evaluated action score | Integer utility score | `-10_000_000 <= TotalScore <= 10_000_000` |
| `TOTAL_SLICES` | Time-slicing cadence | Ticks | `100 ticks` (~1.6 hours in-game) |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 MicroEntity Evaluation State Machine

```text
[ TICK EVALUATION ]
       │
       ▼ (Check Time-Slice: agent_id % TOTAL_SLICES == tick % TOTAL_SLICES)
┌─────────────────────────────────────────────────────────────┐
│ TIME-SLICE CHECK                                            │
│ - Mismatch: Decrement cooldown if > 0. Exit tick.           │
│ - Match: Proceed to Cooldown Check.                         │
└──────────────────────────────┬──────────────────────────────┘
                               │
       ▼ (Check: cooldown_ticks == 0)
┌─────────────────────────────────────────────────────────────┐
│ PRECONDITION FILTERING                                      │
│ - Filter action catalog against treasury, inventory, rights.│
│ - If no actions available: emit IDLE event, set cooldown 10.│
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│ UTILITY SCORING LOOP                                        │
│ - For each valid action:                                    │
│     Compute DriveScore + TraitScore + CloutModifier         │
│ - Find maximum TotalScore. Break ties by lowest ActionID.   │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│ ACTION DISPATCH & STATE COMMIT                              │
│ - Register intent in Phase 1 Demand Buffer.                 │
│ - Set cooldown_ticks = a_best.base_cooldown.                │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Canonical Action Catalog Sample Truth Table

| Action Name | Action ID | Target Drive | Trait Alignment | Base Cooldown |
| :--- | :--- | :--- | :--- | :--- |
| `ACTION_EXPAND_MINING` | `101` | Treasury (+), Market Share (+) | Greed (+), Risk (+) | `500 ticks` |
| `ACTION_HOARD_CAPITAL` | `102` | Treasury (+), Grievance (+) | Paranoia (+), Risk (-) | `300 ticks` |
| `ACTION_DEPLOY_TRACER` | `103` | Security (+), Compute (-) | Paranoia (+), Greed (-) | `100 ticks` |
| `ACTION_SUBSIDIZE_POPS` | `104` | Grievance (-), Treasury (-) | Ideology (+), Greed (-) | `600 ticks` |

---

## 📊 4. Constants, Figures & Baseline Data Tables

| Constant | Value | Description |
| :--- | :--- | :--- |
| `TOTAL_SLICES` | `100` | Agent evaluation cadence partitions |
| `TRAIT_SCALE` | `1_000` | Fixed-point scale for personality traits (`1.0 = 1000`) |
| `MAX_ACTIONS_PER_AGENT` | `32` | Maximum actions scored per agent evaluation cycle |

### 4.1 Data-Oriented Component Memory Layout (DoD)

```text
// MicroEntity Component (Contiguous flat array)
struct MicroEntityComponent {
    uint32 agent_id;
    uint32 parent_macro_id;
    int16 traits[4];           // Greed, Risk, Paranoia, Ideology
    uint32 tracked_stock_ids[4];
    int64 target_levels[4];
    uint16 current_action_id;
    uint16 cooldown_ticks;
}

// Clout Component (Directed Player -> Agent vector)
struct CloutComponent {
    uint32 agent_id;
    int16 trust;               // -1000 to +1000
    int16 leverage;            // 0 to +1000
    uint64 last_subverted_tick;
}
```

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Instantaneous Action Mutation):** An agent deciding to buy ore cannot immediately alter the stock buffer. It must submit an order packet into Phase 1 of `SSOT-SYS-001`, subject to market contention and pricing.
- **Trap 2 (Non-Deterministic Random Tie-Breaking):** Never use pseudo-random selection when action utility scores match. Tie-breaks must strictly favor the lowest `ActionID` for save/replay consistency.
- **Trap 3 (Direct Meso-Entity Utility Processing):** Meso-entities (demographic groups, worker shifts) have no agency. Only `MicroEntity` agents evaluate Utility AI.
- **Trap 4 (Evaluating All Agents Every Tick):** Attempting to score 50,000 agents every 60-second tick will exhaust CPU budgets. Always adhere to the 100-slice time-slicing schedule.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Traits & Drives | Actions Available | Expected Winner | Total Score |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `VEC-SOC-01` | Greedy Exec with Treasury Deficit | Greed: `+800`, Paranoia: `0`, Deficit: `+500` | `101 (Expand)` vs `104 (Subsidize)` | `Action 101` | `+850_000` |
| `VEC-SOC-02` | Paranoid Minister under Anomaly | Paranoia: `+900`, Greed: `-200`, Suspicion: `High` | `102 (Hoard)` vs `103 (Deploy Tracer)` | `Action 103` | `+920_000` |
| `VEC-SOC-03` | Exact Utility Score Tie-Break | Both Actions evaluate to exactly `500_000` | `Action 102` vs `Action 105` | `Action 102` (Lowest ID) | Exact tie-break |
| `VEC-SOC-04` | Time-Slice Evaluation Skip | `agent_id = 42`, `current_tick = 43`, `TOTAL_SLICES = 100` | Any | Skipped (`is_active = false`) | No evaluation |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Autonomous Utility AI & Sociopolitical Decision Engine (SOC-001)",
  "category": "specification",
  "key_invariants_formulas": "Utility scoring dot-product, time-sliced DoD execution, deterministic lowest-ID tie-breaking",
  "status": "approved"
}
```

