---
id: SSOT-PHY-001
title: "2D Keplerian Astrodynamics, Rocket Mechanics & Orbital Transfers"
domain: "Astrodynamics / Orbital & Propulsion Mechanics"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Archibald E. Roy, 'Orbital Motion', 4th Edition, Institute of Physics Publishing, 2005"
  - "Howard D. Curtis, 'Orbital Mechanics for Engineering Students', 4th Edition, Butterworth-Heinemann, 2020"
  - "Konstantin E. Tsiolkovsky, 'The Exploration of Cosmic Space by Means of Reaction Devices', 1903"
  - "SSOT-SYS-000: Universal SI Units & Dimensional Metrology"
---

# SSOT-PHY-001: 2D Keplerian Astrodynamics, Rocket Mechanics & Orbital Transfers

> **SSOT ID:** `SSOT-PHY-001` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** Orbital Motion (Roy 2005), Orbital Mechanics for Engineering Students (Curtis 2020), Tsiolkovsky (1903), SSOT-SYS-000

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the deterministic spatial topology and orbital mechanics of interplanetary space, celestial trajectories, and vessel propulsion dynamics:
1. **2D Coplanar Heliocentric Space:** Major celestial bodies (`Astronodes`) and interplanetary vessels move strictly within the 2D ecliptic plane `(x, y)`. Body positions are computed directly as closed-form functions of the integer simulation tick `t` without numerical N-body integration drift.
2. **Propulsion Dynamics & Delta-V:** Vessel velocity changes, propellant consumption, thrust generation, and burn durations follow the exact Tsiolkovsky rocket equation grounded in SI units defined in `SSOT-SYS-000`.
3. **Orbital Transfers & Patched Conics:** Interplanetary navigation uses patched conics based on Laplace Spheres of Influence (SOI). Inside an Astronode's SOI, celestial gravity dominates; outside, the Sun's heliocentric gravity dominates.

Local planetary surface coordinates, Fibonacci lattices, and surface node structures are strictly decoupled from this document and defined separately in `SSOT-PHY-004`.

### 1.2 Core Domain Invariants

- **Invariant 1 (2D Coplanar Interplanetary Motion):** All orbital trajectories, velocity vectors, maneuvers, and body barycenters reside strictly in the 2D ecliptic plane `(x, y)`. Out-of-plane inclination (`z`) is zero for all primary celestial bodies and vessels.
- **Invariant 2 (Closed-Form Ephemeris):** Celestial body positions at tick `t` are calculated via deterministic closed-form trigonometric formulas. Numerical Runge-Kutta or Euler integrators for celestial bodies are strictly forbidden.
- **Invariant 3 (Fixed-Point Integer Determinism):** All orbital angles must be represented in integer micro-radians (`urad`), where `2 * pi = 6_283_185 urad`. Runtime floating-point trigonometry (`f64::sin`, `f64::cos`) is forbidden; all angle queries use precomputed integer sine/cosine tables.
- **Invariant 4 (Strict Mass & Delta-V Conservation):** Maneuvers strictly deplete propellant mass according to the Tsiolkovsky rocket equation. Non-propulsive delta-V creation is forbidden. Vessels cannot burn without consuming propellant from onboard tanks.
- **Invariant 5 (Patched Conics Boundary Invariant):** When a vessel's Euclidean distance to an Astronode center is `<= r_soi`, gravitational calculations and state vectors immediately patch into the local body-centric frame. When distance `> r_soi`, the state vector transitions to the heliocentric frame.

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 2D Coplanar Heliocentric Kinematics

For each `Astronode` `k` orbiting a parent with standard gravitational parameter `mu` (in `m^3 / s^2`), semi-major axis `a_k` (in `km`), and orbital period `P_k` (in simulation ticks):

- **Mean Angular Motion (micro-radians per tick):**
  `omega_k = TWO_PI_MICRO / P_k`

- **Mean Longitude / True Anomaly at Tick `t`:**
  `theta_k(t) = (theta_0_k + omega_k * t) mod TWO_PI_MICRO`

- **Heliocentric Position `(x, y)` in km:**
  `x_k(t) = mul_fixed(a_k, cos_table(theta_k(t)))`
  `y_k(t) = mul_fixed(a_k, sin_table(theta_k(t)))`

- **Inter-Body Euclidean Distance in km:**
  `dx = x_j(t) - x_i(t)`
  `dy = y_j(t) - y_i(t)`
  `d_ij(t) = sqrt_int64(dx * dx + dy * dy)`

- **Circular Orbital Speed (m/s):**
  `v_circ = sqrt_int64(mu / (r_km * 1_000))`

- **Vis-Viva Equation (m/s):**
  `v = sqrt_int64(mu * (2 / (r_km * 1_000) - 1 / (a_km * 1_000)))`

### 2.2 Rocket Propulsion & Delta-V Mechanics

All propulsion calculations are grounded in SI base units (`kg`, `s`, `m`, `N`):

- **Effective Exhaust Velocity (`v_e` in m/s):**
  `v_e = (I_sp * G_0_MICRO) / 1_000_000`
  where `I_sp` is specific impulse in seconds, and `G_0_MICRO = 9_806_650` (`9.80665 m/s^2 * 1e6`).

- **Thrust Force (`F_thrust` in N):**
  `F_thrust = m_dot * v_e`
  where `m_dot` is fuel mass flow rate in `kg/s`.

- **Tsiolkovsky Rocket Equation (Delta-V in m/s):**
  `delta_v = v_e * ln_fixed(m_0, m_f)`
  where `m_0` is initial wet mass (kg), `m_f` is final dry mass (kg), and `ln_fixed(m_0, m_f)` evaluates the natural logarithm via fixed-point Series:
  `ln(m_0 / m_f) = ln(1 + (m_0 - m_f) / m_f)`

- **Propellant Mass Required for Maneuver (`m_prop` in kg):**
  `m_prop = m_0 * (1_000_000 - exp_neg_fixed(delta_v, v_e)) / 1_000_000`
  where `exp_neg_fixed(delta_v, v_e)` computes `exp(-delta_v / v_e) * 1_000_000`.

- **Burn Duration in Discrete Ticks:**
  `burn_seconds = ceil(m_prop / m_dot)`
  `burn_ticks = ceil(burn_seconds / 60)`

- **Tick Acceleration (`a_tick` in m/s^2):**
  `m_avg = (m_0 + m_f) / 2`
  `a_tick = F_thrust / m_avg`

### 2.3 Sphere of Influence (SOI) & Boundary Transitions

- **Laplace Sphere of Influence Radius (`r_soi` in km):**
  `r_soi = a_k * (m_body / m_parent)^(2/5)`
  For integer computation:
  `ratio_scaled = (m_body * 1_000_000) / m_parent`
  `r_soi = (a_k * int_pow_fifth(ratio_scaled * ratio_scaled)) / 1_000_000`

- **Patched Conic Coordinate Transition:**
  - Let `(x_v, y_v)` be vessel heliocentric position, and `(x_b, y_b)` be body position.
  - Radial distance to body:
    `d_b = sqrt_int64((x_v - x_b)^2 + (y_v - y_b)^2)`
  - **SOI Entry Condition:** If `d_b <= r_soi`, transform vessel state vector to body-centric frame:
    `x_rel = x_v - x_b`
    `y_rel = y_v - y_b`
    `vx_rel = vx_v - vx_b`
    `vy_rel = vy_v - vy_b`
  - **SOI Exit Condition:** If `d_b > r_soi`, transform vessel state vector back to heliocentric frame:
    `x_v = x_b + x_rel`
    `y_v = y_b + y_rel`
    `vx_v = vx_b + vx_rel`
    `vy_v = vy_b + vy_rel`

- **Hyperbolic Excess Velocity (`v_inf` in m/s):**
  `v_inf_sq = vx_rel * vx_rel + vy_rel * vy_rel`
  `v_periapsis = sqrt_int64(v_inf_sq + (2 * mu_body) / r_periapsis_m)`

### 2.4 Hohmann Transfer Formulation

For coplanar circular orbits with initial orbital radius `r_1` (m) and target orbital radius `r_2` (m) around central body `mu`:

- **Transfer Semi-Major Axis (m):**
  `a_trans = (r_1 + r_2) / 2`

- **Departure Burn Delta-V (`delta_v_1` in m/s):**
  `v_circ_1 = sqrt_int64(mu / r_1)`
  `v_trans_1 = sqrt_int64(mu * (2 / r_1 - 1 / a_trans))`
  `delta_v_1 = abs(v_trans_1 - v_circ_1)`

- **Arrival Injection Delta-V (`delta_v_2` in m/s):**
  `v_circ_2 = sqrt_int64(mu / r_2)`
  `v_trans_2 = sqrt_int64(mu * (2 / r_2 - 1 / a_trans))`
  `delta_v_2 = abs(v_circ_2 - v_trans_2)`

- **Total Mission Transfer Delta-V (m/s):**
  `delta_v_total = delta_v_1 + delta_v_2`

- **One-Way Flight Time (seconds & ticks):**
  `t_flight_s = PI_MICRO * sqrt_int64((a_trans^3) / mu) / 1_000_000`
  `t_flight_ticks = ceil(t_flight_s / 60)`

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Vessel Astrodynamic Flight State Machine

```text
┌─────────────────────────────────────────────────────────────┐
│                    ORBITING_HELIOCENTRIC                    │
│   - Tracked relative to Sun barycenter (0, 0)               │
│   - State: (x, y, vx, vy) in Heliocentric km & m/s          │
└──────────────────────────────┬──────────────────────────────┘
                               │
               (Event: INITIATE_MANEUVER_BURN)
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                     PROPULSIVE_BURN                         │
│   - Consumes propellant: m_fuel -= m_dot * dt               │
│   - Applies delta_v vector along burn heading               │
│   - State transition when: burn_ticks_elapsed >= burn_ticks │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                     KEPLERIAN_COAST                         │
│   - Ballistic transit along conic section                   │
│   - Continuous distance check to all Astronodes             │
└──────────────┬──────────────────────────────┬───────────────┘
               │                              │
     (d_body <= r_soi)                        │ (d_current_body > r_soi)
               ▼                              ▼
┌──────────────────────────────┐    ┌──────────────────────────────┐
│          SOI_ENTRY           │    │           SOI_EXIT           │
│  - Patch to local frame      │    │  - Patch to Sun frame        │
│  - Compute hyperbolic v_inf  │    │  - Add body velocity vector  │
└──────────────┬───────────────┘    └──────────────┬───────────────┘
               │                                   │
               ▼                                   ▼
┌──────────────────────────────┐    ┌──────────────────────────────┐
│        ORBITING_LOCAL        │    │    ORBITING_HELIOCENTRIC     │
│  - Tracked relative to body  │    │                              │
│  - Stable circular/elliptic  │    │                              │
└──────────────────────────────┘    └──────────────────────────────┘
```

### 3.2 Maneuver Feasibility Decision Table

| Condition | Check / Predicate | Result / Action |
| :--- | :--- | :--- |
| `m_prop_available < m_prop_required` | Insufficient onboard propellant | `ABORT_INSUFFICIENT_FUEL` (No burn) |
| `F_thrust == 0` or Engine Offline | Zero propulsion capability | `ABORT_ENGINE_FAILURE` |
| `burn_ticks <= 0` | Maneuver magnitude below threshold | `NO_OP_MAINTAIN_TRAJECTORY` |
| Nominal propellant & engine online | All criteria satisfied | `EXECUTE_PROPULSIVE_BURN` |

---

## 📊 4. Constants, Figures & Baseline Data Tables

### 4.1 Fundamental Astrodynamic Constants

| Constant | Exact / Standard Value | SI Unit | Notes |
| :--- | :--- | :--- | :--- |
| `SIM_TICK_SECONDS` | `60` | `s` | 1 simulation tick = 1 minute |
| `PI_MICRO` | `3_141_593` | `urad` | Fixed-point integer Pi |
| `TWO_PI_MICRO` | `6_283_185` | `urad` | Fixed-point integer 2 * Pi |
| `AU_KM` | `149_597_870` | `km` | 1 Astronomical Unit |
| `SPEED_OF_LIGHT_KM_TICK`| `17_987_547` | `km/tick` | `299_792.458 km/s * 60` |
| `MU_SUN` | `1.32712440018e20` | `m^3 / s^2` | Standard gravitational parameter Sun |
| `G_0` | `9.80665` | `m / s^2` | Standard Earth surface gravity |

### 4.2 Primary Celestial Bodies Table (Epoch t = 0)

| Body Name | Parent | Semi-Major Axis `a` (km) | Orbital Period `P` (ticks) | Mass `M` (kg) | Standard Grav. Parameter `mu` (m^3 / s^2) | SOI Radius `r_soi` (km) |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| `SUN` | None | `0` | `0` | `1.989e30` | `1.32712e20` | Infinity |
| `EARTH` | Sun | `149_597_870` | `525_949` (~365.24 d) | `5.972e24` | `3.98600e14` | `925_000` |
| `LUNA` | Earth | `384_400` | `39_343` (~27.32 d) | `7.342e22` | `4.90487e12` | `66_100` |
| `MARS` | Sun | `227_939_200` | `989_478` (~687.14 d) | `6.417e23` | `4.28284e13` | `577_000` |
| `CERES` | Sun | `413_700_000` | `2_422_000` (~1682 d) | `9.393e20` | `6.26325e10` | `100_000` |

### 4.3 Engine Propulsion Archetypes

| Engine Archetype | Specific Impulse `I_sp` (s) | Exhaust Velocity `v_e` (m/s) | Nominal Thrust `F` (kN) | Primary Propellant |
| :--- | :--- | :--- | :--- | :--- |
| Chemical Bipropellant | `320` | `3_138` | `450` | `LOX_CH4` / `HYDROCARBONS` |
| Hydrolox Vacuum | `450` | `4_413` | `250` | `LH2_LOX` |
| Nuclear Thermal Rocket (NTR) | `900` | `8_826` | `150` | `LH2` |
| Magnetoplasmadynamic (MPD) | `5_000` | `49_033` | `15` | `ARGON` / `XENON` |
| Relativistic Particle Beam | `20_000` | `196_133` | `2` | `HE3` / `DEUTERIUM` |

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Numerical N-Body Drift):** Attempting to compute planetary orbits using numerical integration (e.g. Verlet or Runge-Kutta). Orbits of major bodies must be calculated deterministically from tick `t` using analytical 2D Kepler formulas.
- **Trap 2 (Free Delta-V Creation):** Executing orbital maneuvers without subtracting propellant mass via the Tsiolkovsky rocket equation. Maneuvers must enforce `m_final = m_0 - m_prop`.
- **Trap 3 (Single Universal 3D Coordinate Space):** Blending interplanetary coordinates with planetary surface meshes in one floating-point 3D space causes severe jitter. Interplanetary astrodynamics must remain in 2D ecliptic plane `(x, y)`, and planetary surfaces must remain in separate 3D local spheres (see `SSOT-PHY-004`).
- **Trap 4 (Exhaust Velocity / Isp Confusion):** Forgetting that `v_e = I_sp * g_0`. An `I_sp` of 300 s gives `v_e approx 2942 m/s`, not `300 m/s`.
- **Trap 5 (LaTeX Formula Formatting):** Writing math with single or double dollar delimiters or LaTeX macros. This violates the zero-LaTeX invariant. Always use ASCII text.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Input | Expected Output | Tolerance |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-AST-01` | Earth Heliocentric Position at `t = 0` | `a = 149_597_870 km`, `theta_0 = 0` | `x = 149_597_870 km`, `y = 0 km` | Exact `0` |
| `VEC-AST-02` | Earth Heliocentric Position at Quarter Orbit | `t = 131_487 ticks` (`P / 4`) | `x = 0 km`, `y = 149_597_870 km` | `+/- 100 km` |
| `VEC-AST-03` | Tsiolkovsky Delta-V Calculation | `m_0 = 100_000 kg`, `m_f = 20_000 kg`, `v_e = 3_000 m/s` | `delta_v = 3000 * ln(5) = 4_828.31 m/s` | `+/- 1.0 m/s` |
| `VEC-AST-04` | Propellant Mass for Burn | `m_0 = 50_000 kg`, `delta_v = 1_000 m/s`, `v_e = 4_413 m/s` | `m_prop = 50_000 * (1 - exp(-1000/4413)) = 10_134 kg` | `+/- 5 kg` |
| `VEC-AST-05` | Earth Circular Orbital Speed around Sun | `mu = 1.32712e20 m^3/s^2`, `r = 149_597_870_000 m` | `v_circ = 29_784 m/s` (~29.78 km/s) | `+/- 5 m/s` |
| `VEC-AST-06` | Earth Laplace SOI Radius | `a = 149_597_870 km`, `M_earth = 5.972e24`, `M_sun = 1.989e30` | `r_soi = 925_000 km` | `+/- 2_000 km` |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "2D Keplerian Astrodynamics, Rocket Mechanics & Orbital Transfers (PHY-001)",
  "category": "specification",
  "key_invariants_formulas": "2D coplanar Keplerian kinematics, Tsiolkovsky rocket equation, thrust/Isp conversion, Laplace SOI boundaries, Hohmann transfers",
  "status": "approved"
}
```
