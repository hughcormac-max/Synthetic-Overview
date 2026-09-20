---
id: SSOT-SYS-000
title: "Universal SI Units & Dimensional Metrology"
domain: "Universal Standards / Dimensional Metrology"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "BIPM, 'The International System of Units (SI)', 9th Edition, Bureau International des Poids et Mesures, 2019"
  - "ISO/IEC 80000-1:2022, 'Quantities and units — Part 1: General', International Organization for Standardization, 2022"
  - "NIST Special Publication 330, 'The International System of Units (SI)', National Institute of Standards and Technology, 2019"
---

# SSOT-SYS-000: Universal SI Units & Dimensional Metrology

> **SSOT ID:** `SSOT-SYS-000` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** BIPM SI Brochure (9th ed. 2019), ISO 80000-1:2022, NIST SP 330

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification establishes the authoritative, immutable Tier 0 ground truth for all units of measurement, dimensional analysis, fixed-point integer scaling, and fundamental physical constants across the entire simulation codebase.

Every physical calculation, astrodynamic formula, manufacturing recipe, energy flow, network latency calculation, and structural property in all other SSOT documents and system modules must strictly ground its dimensions in the International System of Units (SI) defined herein. Non-SI systems (e.g. Imperial, nautical, or custom arbitrated units) are strictly prohibited.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict SI Grounding):** All quantities in code, state, memory, and data serialization must be expressed in canonical SI base or coherent derived units, or exact decimal integer multiples thereof (e.g. `km`, `t`) with documented fixed conversion factors.
- **Invariant 2 (Discrete Time Grounding):** The base unit of simulated time is the SI second (`s`). The discrete simulation kernel tick is strictly defined as `1 tick = 60 s` (1 minute). All per-tick rates are exact functions of this time step: `rate_per_tick = rate_per_second * 60`.
- **Invariant 3 (f64 Determinism):** To prevent cross-platform floating-point divergence, all fractional physical values in simulation state must use strictly deterministic `f64` arithmetic (IEEE-754 semantics). Using hardware transcendentals or non-deterministic math is strictly prohibited.
- **Invariant 4 (Zero-LaTeX Notation):** All formulas, units, variables, and constants in this and dependent specifications must be written in clean, human-readable plain text or ASCII math. Single dollar signs or double dollar signs and LaTeX macros are strictly prohibited.
- **Invariant 5 (Downward Dimensional Inheritance):** Any new physical quantity introduced in dependent SSOTs must declare its dimensional formula in terms of base dimensions `[L]`, `[M]`, `[T]`, `[I]`, `[Theta]`, `[N]`, `[J]` defined in Section 2.1.

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Base Dimensions & Primary Units

| Dimension Symbol | Physical Dimension | SI Base Unit | Unit Symbol | Definition / Grounding |
| :--- | :--- | :--- | :--- | :--- |
| `[L]` | Length | meter | `m` | Distance travelled by light in vacuum in `1 / 299_792_458 s` |
| `[M]` | Mass | kilogram | `kg` | Defined via Planck constant `h = 6.62607015e-34 J * s` |
| `[T]` | Time | second | `s` | Defined via Cesium 133 frequency `delta_nu_Cs = 9_192_631_770 Hz` |
| `[I]` | Electric current | ampere | `A` | Defined via elementary charge `e = 1.602176634e-19 C` |
| `[Theta]` | Thermodynamic temperature | kelvin | `K` | Defined via Boltzmann constant `k = 1.380649e-23 J / K` |
| `[N]` | Amount of substance | mole | `mol` | Defined via Avogadro constant `N_A = 6.02214076e23 mol^-1` |
| `[J]` | Luminous intensity | candela | `cd` | Defined via luminous efficacy `K_cd = 683 lm / W` |

### 2.2 Coherent Derived Units & Dimensional Formulas

| Quantity | Unit Name | Symbol | Dimensional Expression | SI Base Equivalent |
| :--- | :--- | :--- | :--- | :--- |
| Plane Angle | radian | `rad` | `m / m` (dimensionless) | `1` |
| Frequency | hertz | `Hz` | `1 / [T]` | `s^-1` |
| Force | newton | `N` | `[M] * [L] / [T]^2` | `kg * m / s^2` |
| Pressure / Stress | pascal | `Pa` | `[M] / ([L] * [T]^2)` | `N / m^2 = kg / (m * s^2)` |
| Energy / Work / Heat | joule | `J` | `[M] * [L]^2 / [T]^2` | `N * m = kg * m^2 / s^2` |
| Power / Heat Flow | watt | `W` | `[M] * [L]^2 / [T]^3` | `J / s = kg * m^2 / s^3` |
| Electric Charge | coulomb | `C` | `[I] * [T]` | `A * s` |
| Electric Potential | volt | `V` | `[M] * [L]^2 / ([I] * [T]^3)` | `W / A = kg * m^2 / (A * s^3)` |
| Velocity | speed | `m/s` | `[L] / [T]` | `m / s` |
| Acceleration | acceleration | `m/s^2` | `[L] / [T]^2` | `m / s^2` |
| Density | volumetric mass | `kg/m^3` | `[M] / [L]^3` | `kg / m^3` |
| Specific Impulse (Effective) | exhaust velocity | `m/s` | `[L] / [T]` | `m / s` |
| Specific Impulse (Weight) | seconds | `s` | `[T]` | `s` (where `v_e = I_sp * g_0`) |

### 2.3 Unit Multiples, Prefixes & Permitted Scales

| Prefix | Symbol | Factor | Applied Unit | Canonical Simulation Role |
| :--- | :--- | :--- | :--- | :--- |
| `micro-` | `u` | `1e-6` | `um`, `urad` | High-precision angles and state vectors |
| `milli-` | `m` | `1e-3` | `mm`, `ms` | Fine structural clearances, telemetry delays |
| `kilo-` | `k` | `1e3` | `km`, `kg`, `kW`, `kN` | Celestial distances (`km`), mechanical forces (`kN`) |
| `mega-` | `M` | `1e6` | `MJ`, `MW`, `MPa` | Industrial reactor outputs, bulk material stress |
| `giga-` | `G` | `1e9` | `GJ`, `GW` | Macro grid energy, planetary kinetic weapons |

Permitted non-SI decimal multiples:
- **Metric Ton (`t`):** `1 t = 1_000 kg` (used for vessel, cargo, and bulk resource payload mass).
- **Kilometer (`km`):** `1 km = 1_000 m` (used for macro planetary orbital semi-major axes and radii).
- **Simulation Tick (`tick`):** `1 tick = 60 s` (fundamental discrete time quantum).

### 2.4 Standard f64 Conversion & Scaling Rules

All physical simulation quantities must be stored and manipulated as `f64` floats.

- **Float Representation:**
  `value = raw_value * factor`

- **SI Velocity to Tick Distance (m):**
  `d_m_per_tick = v_m_s * 60.0`

- **SI Acceleration to Tick Velocity Change (m/s):**
  `delta_v_per_tick = a_m_s2 * 60.0`

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Unit Normalization Pipeline

```text
┌─────────────────────────────────────────────────────────────┐
│                 EXTERNAL INPUT / CONFIG VALUE               │
│     Raw magnitude + Declared Unit Identifier String         │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 DIMENSIONAL VALIDATION GATE                 │
│   - Check declared unit against Authorized Unit Registry    │
│   - Reject if non-SI (e.g. feet, pounds, PSI, knots, d)    │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│               SI BASE CONVERSION & CANONICALIZATION          │
│   - Apply exact integer multiplier: value_si = raw * factor │
│   - Standardize to: m, kg, s, J, W, N, rad                  │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│               DISCRETE FIXED-POINT ENCODING                 │
│   - Scale by FIXED_POINT_SCALE (1e6) or canonical int type   │
│   - Store in deterministic ECS component / data struct      │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Unit Validation & Dimension Truth Table

| Input Quantity | Declared Unit | Valid Dimension | Action / Conversion | Resulting Canonical Storage |
| :--- | :--- | :--- | :--- | :--- |
| Orbital Radius | `km` | `[L]` | `r_m = r_km * 1_000.0` | `f64` meters |
| Delta-V | `m/s` | `[L] / [T]` | Identity | `f64` m/s |
| Vessel Mass | `t` (ton) | `[M]` | `m_kg = m_t * 1_000.0` | `f64` kilograms |
| Reactor Output | `MW` | `[M]*[L]^2 / [T]^3` | `p_w = p_mw * 1_000_000.0` | `f64` watts |
| Thrust Force | `kN` | `[M]*[L] / [T]^2` | `f_n = f_kn * 1_000.0` | `f64` newtons |
| Time Duration | `min` | `[T]` | `t_s = t_min * 60.0` | `f64` seconds / ticks |
| Non-SI Unit | `lb`, `mi`, `ft` | Any | REJECT & PANIC | Invalid Domain Specification |

---

## 📊 4. Constants, Figures & Baseline Data Tables

### 4.1 Fundamental Physical & Astronomical Constants

| Constant | Symbol | Exact / Standard Value | SI Unit | Dimensional Formula | Source Authority |
| :--- | :--- | :--- | :--- | :--- | :--- |
| Speed of Light in Vacuum | `c` | `299_792_458` | `m / s` | `[L] / [T]` | CODATA 2018 / BIPM |
| Newtonian Gravitational Constant | `G` | `6.67430e-11` | `m^3 / (kg * s^2)` | `[L]^3 / ([M] * [T]^2)` | CODATA 2018 |
| Standard Earth Gravity | `g_0` | `9.80665` | `m / s^2` | `[L] / [T]^2` | ISO 80000-3 |
| Astronomical Unit | `AU` | `149_597_870_700` | `m` | `[L]` | IAU 2012 Resolution B2 |
| Astronomical Unit (in km) | `AU_KM` | `149_597_870` | `km` | `[L]` | Truncated Integer km |
| Speed of Light per Tick (km) | `c_tick_km` | `17_987_547` | `km / tick` | `[L] / [T_tick]` | `(299_792_458 * 60) / 1000` |
| Speed of Light per Tick (m) | `c_tick_m` | `17_987_547_480` | `m / tick` | `[L] / [T_tick]` | `299_792_458 * 60` |
| Standard Simulation Tick | `T_TICK` | `60` | `s` | `[T]` | System Core Invariant |
| Circle Full Angle (Micro-rad) | `TWO_PI` | `6_283_185` | `urad` | Dimensionless (`1e-6`) | `2 * pi * 1e6` |
| Half Circle Angle (Micro-rad) | `PI` | `3_141_593` | `urad` | Dimensionless (`1e-6`) | `pi * 1e6` |
| Golden Angle (Micro-rad) | `GOLDEN` | `2_399_963` | `urad` | Dimensionless (`1e-6`) | `pi * (3 - sqrt(5)) * 1e6` |

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Imperial Unit Intrusion):** Using feet (`ft`), miles (`mi`), pounds (`lb`), or atmospheres (`atm`). All mechanics, thrust, and structures must strictly use meters (`m`), kilograms (`kg`), and pascals (`Pa`).
- **Trap 2 (Tick vs Second Confusion):** Confusing time parameters in seconds with time parameters in ticks. Standard acceleration is in `m/s^2`, but velocity change over a tick is `delta_v = a * 60 s`. Never multiply `a * 1` assuming 1 tick is 1 second.
- **Trap 3 (Specific Impulse Unit Ambiguity):** Specific impulse can be quoted in seconds (`I_sp_s`) or effective exhaust velocity (`v_e` in `m/s`). In this project, all rocket equations must explicitly distinguish `I_sp` (in `s`) from `v_e` (in `m/s`) via `v_e = I_sp * g_0` where `g_0 = 9.80665 m/s^2`.
- **Trap 4 (Precision Loss in Extremes):** Using `f64` prevents overflow during scaling operations, but precision loss (catastrophic cancellation) can occur when summing very small differences (like sub-millimeter delta-v) into very large quantities (like AU-scale orbital radii). Always group additions of similar magnitude before adding to a massive accumulator.
- **Trap 5 (LaTeX Formula Formatting):** Writing math using LaTeX math formatting or LaTeX macros. This violates repository zero-LaTeX rules. Always use ASCII text: `v = sqrt(mu / r)`.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Test Scenario | Input Parameters | Exact Expected Output | Tolerance |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-UNT-01` | Light travel distance per tick | `c = 299_792_458 m/s`, `t = 60 s` | `17_987_547_480 m` (`17_987_547.48 km`) | Exact integer truncated `17_987_547 km` |
| `VEC-UNT-02` | Exhaust velocity from Isp | `I_sp = 300 s`, `g_0 = 9.80665 m/s^2` | `v_e = 2_941.995 m/s` | `+/- 0.001 m/s` |
| `VEC-UNT-03` | Fixed-point angle full wrap | `theta = 6_283_185 + 1_000` | `theta mod TWO_PI = 1_000 urad` | Exact integer |
| `VEC-UNT-04` | Mass metric ton to kg | `m_payload = 45 t` | `45_000 kg` | Exact integer |
| `VEC-UNT-05` | Force from mass and acceleration | `m = 10_000 kg`, `a = 2.5 m/s^2` | `F = 25_000 N` (`25 kN`) | Exact integer |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Universal SI Units & Dimensional Metrology (SYS-000)",
  "category": "specification",
  "key_invariants_formulas": "Base SI units (m, kg, s, A, K, mol, cd), 1 tick = 60s, fixed-point scale 1e6, universal constants c, G, g_0, AU",
  "status": "approved"
}
```
