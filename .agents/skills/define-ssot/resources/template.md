---
id: SSOT-NNNN
title: "[Short, Descriptive Domain SSOT Title]"
domain: "[e.g. Astrodynamics / Financial Risk / Tax Logic / Cryptography]"
category: specification # specification | formula | decision-tree | constants-table
status: approved # draft | approved | deprecated
created: [YYYY-MM-DD]
updated: [YYYY-MM-DD]
sources:
  - "[Author / Standard / Regulatory Body], '[Title / RFC / Standard Number]', [Publisher / Link], [Year]"
---

# SSOT-NNNN: [Short, Descriptive Domain SSOT Title]

> **SSOT ID:** `SSOT-NNNN` | **Category:** `[specification / formula / decision-tree / constants-table]`
> **Status:** `approved` | **Last Updated:** [YYYY-MM-DD]
> **Authoritative Sources:** [List authoritative papers, standards, specifications, or internal domain directives]

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

[Describe the exact domain reality, algorithm, regulatory policy, or mathematical foundation defined by this SSOT.]

### 1.2 Core Domain Invariants

- **Invariant 1:** [Absolute ground truth that must never be violated.]
- **Invariant 2:** [Constraint on input ranges, state transitions, or physical limits.]

---

## 📐 2. Deterministic Formulas & Calculation Rules

*All formulations must use clean plain-text/ASCII notation (Zero-LaTeX).*

### 2.1 Formula Definitions

- **[Formula / Metric Name]:**
  `result = base_value * (1 + rate * time) - damping_factor`

### 2.2 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit (SI/UCUM/ISO) | Valid Range |
| :--- | :--- | :--- | :--- |
| `base_value` | Initial measured magnitude | `m` (meters) | `base_value >= 0` |
| `rate` | Linear growth / decay rate | `1/s` | `-1.0 <= rate <= 1.0` |
| `time` | Elapsed duration | `s` (seconds) | `time >= 0` |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

*Explicit algorithmic branching, state machines, and loop conditions.*

### 3.1 State Machine / Transition Matrix

```text
[State: IDLE] --- (Event: INITIALIZE) ---> [State: PROCESSING]
[State: PROCESSING] --- (Condition: success == true) ---> [State: RESOLVED]
[State: PROCESSING] --- (Condition: error_count >= 3) ---> [State: FAILED]
```

### 3.2 Decision Rules & Branching Truth Table

| Condition A | Condition B | Threshold Check | Resulting State / Action |
| :--- | :--- | :--- | :--- |
| `is_verified == true` | `balance >= amount` | `amount <= MAX_TRANSFER_LIMIT` | `EXECUTE_TRANSACTION` |
| `is_verified == true` | `balance < amount` | Any | `REJECT_INSUFFICIENT_FUNDS` |
| `is_verified == false` | Any | Any | `REJECT_UNAUTHORIZED` |

---

## 📊 4. Constants, Figures & Baseline Data Tables

*Authoritative numbers, conversion factors, and fixed parameters.*

| Parameter / Constant | Exact Value | Standard Unit | Source / Authority |
| :--- | :--- | :--- | :--- |
| `GRAVITATIONAL_CONSTANT_G` | `6.67430e-11` | `m^3 / (kg * s^2)` | CODATA 2018 |
| `STANDARD_SEA_LEVEL_PRESSURE` | `101325` | `Pa` | ISO 2533:1975 |

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

*Specific false assumptions, obsolete patterns, or subtle traps that AI coding agents commonly fall into in this domain.*

- [ ] **Trap 1 (Unit Confusion):** Common error is assuming angles are in degrees instead of radians. All trigonometric operations require radians.
- [ ] **Trap 2 (Sign Convention):** Damping forces must be subtracted, not added. Adding introduces artificial energy into the system.
- [ ] **Trap 3 (Boundary Misinterpretation):** At `value == 0.0`, the system must not divide by zero; apply epsilon threshold `eps = 1e-12`.
- [ ] **Trap 4 (Timezone / Date Bias):** Never assume local time; all epoch timestamps must parse as UTC.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

*Deterministic input-to-output test cases for hermetic unit testing.*

| Vector ID | Test Scenario | Input Parameters | Exact Expected Output | Tolerance / Epsilon |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-01` | Nominal calculation | `base_value = 100.0`, `rate = 0.05`, `time = 10.0` | `result = 150.0` | `+/- 1e-6` |
| `VEC-02` | Zero boundary check | `base_value = 0.0`, `rate = 0.05`, `time = 10.0` | `result = 0.0` | Exact `0.0` |
| `VEC-03` | Maximum rate clamp | `base_value = 100.0`, `rate = 2.0`, `time = 1.0` | Throws `DomainRangeError` | N/A |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "[Short title & domain scope]",
  "category": "[specification / formula / decision-tree / constants-table]",
  "key_invariants_formulas": "[Brief summary of key formulas or invariants]",
  "status": "approved"
}
```
