---
id: SSOT-NNNN
title: "[Short, Descriptive Domain SSOT Title]"
domain: "[Domain / Subsystem Name]"
type: context # context (flat reference lookup) | specification (formulas, state machines, golden vectors)
status: approved # draft | approved | deprecated
created: YYYY-MM-DD
updated: YYYY-MM-DD
sources:
  - "[Authoritative Source, Standard, RFC, or Internal Domain Directive]"
---

# SSOT-NNNN: [Short, Descriptive Domain SSOT Title]

> **SSOT ID:** `SSOT-NNNN` | **Type:** `[context | specification]` | **Status:** `approved` | **Updated:** YYYY-MM-DD

---

## 1. Scope & Core Truths

[1-3 sentences defining the exact domain scope, policy, or algorithm governed by this SSOT.]

- **[Fact / Rule / Invariant 1]:** [Exact definition, rule, or ground truth.]
- **[Fact / Rule / Invariant 2]:** [Exact definition, rule, or ground truth.]
- **[Fact / Rule / Invariant 3]:** [Exact definition, rule, or ground truth.]

---

## 2. Reference Data & Rules

*Use flat bullet lists or lookup tables tailored to the domain (e.g., Glossary, Constants, Permissions, Conventions, or Schemas). Omit if Section 1 covers all facts.*

| Key / Parameter | Value / Definition | Unit / Constraint / Source |
| :--- | :--- | :--- |
| `[KEY_OR_CONSTANT]` | `[Exact value or rule]` | `[Unit, valid range, or authority]` |

---

## 3. Agent Constraints & Known Gotchas

*Explicit anti-patterns, edge cases, or false assumptions that AI coding agents must avoid.*

- **Trap / Constraint 1:** [Specific mistake to avoid and the exact rule to follow instead.]
- **Trap / Constraint 2:** [Boundary condition, unit convention, or edge case behavior.]

---

<!-- ===================================================================== -->
<!-- OPTIONAL SPECIFICATION MODULES                                        -->
<!-- Include ONLY the sections below that apply when type: specification.  -->
<!-- Delete any unused optional sections completely (never leave "N/A").   -->
<!-- ===================================================================== -->

## 4. [Optional] Deterministic Formulas & Variables

*All formulations must use clean plain-text/ASCII notation (Zero-LaTeX).*

- **[Formula Name]:** `result = base_value * (1 + rate * time) - damping_factor`

| Variable | Meaning | Standard Unit | Valid Range |
| :--- | :--- | :--- | :--- |
| `base_value` | Initial magnitude | `m` | `base_value >= 0` |
| `rate` | Growth / decay rate | `1/s` | `-1.0 <= rate <= 1.0` |
| `time` | Elapsed duration | `s` | `time >= 0` |

---

## 5. [Optional] State Transitions & Decision Tables

```text
[State: IDLE] --- (Event: INITIALIZE) ---> [State: PROCESSING]
[State: PROCESSING] --- (Condition: success == true) ---> [State: RESOLVED]
```

| Condition A | Condition B | Resulting State / Action |
| :--- | :--- | :--- |
| `is_verified == true` | `balance >= amount` | `EXECUTE_TRANSACTION` |
| `is_verified == false` | Any | `REJECT_UNAUTHORIZED` |

---

## 6. [Optional] Golden Test Vectors

| Vector ID | Test Scenario | Input Parameters | Exact Expected Output | Tolerance / Epsilon |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-01` | Nominal path | `base_value = 100.0`, `rate = 0.05`, `time = 10.0` | `result = 150.0` | `+/- 1e-6` |
| `VEC-02` | Boundary check | `base_value = 0.0`, `rate = 0.05`, `time = 10.0` | `result = 0.0` | Exact `0.0` |
