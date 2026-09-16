---
id: SSOT-PHY-002
title: "Interplanetary Logistics, Transfer Windows & Relativistic Latency"
domain: "Orbital Mechanics / Logistics & Networks"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Howard D. Curtis, 'Orbital Mechanics for Engineering Students', Elsevier Aerospace Engineering Series, 2013"
  - "Walter Hohmann, 'The Attainability of Heavenly Bodies', Oldenbourg, 1925"
---

# SSOT-PHY-002: Interplanetary Logistics, Transfer Windows & Relativistic Latency

> **SSOT ID:** `SSOT-PHY-002` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** Orbital Mechanics for Engineering Students (Curtis 2013), Hohmann Transfer Formulations (1925)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the mechanics of mass and information transfer across the solar system. Interplanetary logistics are fundamentally constrained by celestial geometry:
1. **Dynamic Physical Logistics:** Interplanetary `FlowEdges` between celestial bodies do not have constant latency or continuous throughput. High-volume, fuel-efficient mass transit is gated by periodic **Hohmann Transfer Windows** determined by relative orbital phase angles. Transit latency `tau` is locked at packet departure.
2. **Relativistic Cybernetic Information Dynamics:** Communications and market intelligence travel at the speed of light `c`. Information asymmetry across astronomical distances creates causal latency horizons (`tau_c`), enabling latency arbitrage and localized information blackouts during **Solar Conjunction Occlusions**.
3. **Gravity Well Boundary Gateways:** Mass cannot transition directly between an astronode's surface and an interplanetary flow edge without passing through a specialized gateway `ConverterNode` (e.g., Space Elevator, Mass Driver, Launch Complex) that expends energy/propellant to overcome the gravity well.

### 1.2 Core Domain Invariants

- **Invariant 1 (Departure-Locked Physical Latency):** When a physical cargo packet is committed to an interplanetary `FlowEdge` at tick `t_dep`, its arrival tick is immutably set to `arrival_tick = t_dep + tau(t_dep)`. Moving bodies during transit do not alter the flight time of in-flight ballistic trajectories.
- **Invariant 2 (Phase Angle Window Gating):** An interplanetary bulk flow edge has non-zero throughput `F_max > 0` if and only if the current relative phase angle between source and destination bodies falls within the allowable launch window tolerance `|delta_theta - phi_opt| <= phi_window_tolerance`. Outside this window, ballistic `F_max = 0`.
- **Invariant 3 (Finite Speed of Light Information Horizon):** No information, price update, player command, or cybernetic tracer can propagate faster than `SPEED_OF_LIGHT_KM_TICK`. Information latency `tau_c = ceil(distance_km / SPEED_OF_LIGHT_KM_TICK)`.
- **Invariant 4 (Solar Conjunction Line-of-Sight Occlusion):** A direct communication edge between two bodies is blocked (`STATUS_OCCLUDED`) if the 2D line segment connecting them passes within the Sun exclusion radius `R_SUN_EXCLUSION_KM`. Packets must dynamically reroute through orbital relay nodes (e.g., L4/L5 probes) or be queued until conjunction clears.
- **Invariant 5 (Strict Gravity Well Decoupling):** Interplanetary flow edges connect exclusively between orbital staging stock buffers of different astronodes. Direct edges between a `SurfaceNode` on Body A and a `SurfaceNode` on Body B are physically impossible.

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Hohmann Transfer Windows & Latency

Let source body radius be `r_src` and destination body radius be `r_dest` (heliocentric semi-major axes in km), with Sun gravitational parameter `MU_SUN_KM3_TICK2`:

- **Relative Phase Angle:**
  `delta_theta(t) = (theta_dest(t) - theta_src(t)) mod (TWO_PI_MICRO)`

- **Optimal Hohmann Phase Angle (Zero-LaTeX):**
  Let ratio `k_r = div_fixed(r_src, r_dest)`:
  `phi_opt = mul_fixed(PI_MICRO, FIXED_POINT_SCALE - div_fixed(FIXED_POINT_SCALE, 2_828_427) * pow_1_5(FIXED_POINT_SCALE + k_r))`
  (For inner-to-outer transfers, optimal angle is positive; for outer-to-inner, it is negative).

- **Hohmann Transfer Flight Duration (`tau_transfer`):**
  Semi-major axis of transfer ellipse:
  `a_transfer = (r_src + r_dest) / 2`
  `tau_transfer = ceil_int64(PI_MICRO * sqrt_int64((a_transfer * a_transfer * a_transfer) / MU_SUN_KM3_TICK2) / 1_000_000)`

- **Launch Window Condition:**
  `window_open = abs_int64(delta_theta(t) - phi_opt) <= LAUNCH_WINDOW_TOLERANCE_MICRO`
  `F_max(t) = RATED_EDGE_BANDWIDTH if window_open else 0`

### 2.2 Relativistic Signal Propagation & Occlusion

- **Causal Light Latency (`tau_c`):**
  `tau_c(t) = ceil_int64(div_fixed(d_ij(t), SPEED_OF_LIGHT_KM_TICK))`

- **Solar Line-of-Sight Raycast Occlusion:**
  Let source position be `P_src = (x1, y1)` and destination be `P_dest = (x2, y2)` with Sun at `(0, 0)`:
  1. Ray direction vector: `dx = x2 - x1`, `dy = y2 - y1`
  2. Line segment length squared: `L_sq = dx * dx + dy * dy`
  3. Projection parameter `u = clamp((-x1 * dx - y1 * dy) / L_sq, 0.0, 1.0)`
  4. Closest distance from Sun to ray:
     `x_closest = x1 + u * dx`
     `y_closest = y1 + u * dy`
     `d_sun_closest = sqrt_int64(x_closest * x_closest + y_closest * y_closest)`
  5. Occlusion state:
     `is_occluded = (d_sun_closest <= R_SUN_EXCLUSION_KM)`

### 2.3 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit | Valid Range |
| :--- | :--- | :--- | :--- |
| `r_src`, `r_dest` | Heliocentric orbital radii | km | `> 0` |
| `delta_theta` | Relative angular separation | Micro-radians | `0 <= delta_theta < 6_283_185` |
| `phi_opt` | Optimal departure phase angle | Micro-radians | `0 <= phi_opt < 6_283_185` |
| `tau_transfer`| Mass transit flight time | Integer Ticks (minutes) | `tau_transfer > 0` |
| `tau_c` | Speed of light ping delay | Integer Ticks (minutes) | `tau_c >= 0` |
| `d_sun_closest`| Minimum ray distance to Sun | km | `d_sun_closest >= 0` |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Interplanetary Cargo Packet Lifecycle

```text
[ SOURCE SURFACE STOCK ]
       │
       ▼ (Gateway Converter: Launch / Mass Driver / Elevator)
[ SOURCE ORBITAL STAGING STOCK ]
       │
       ▼ (Check Launch Window: |delta_theta - phi_opt| <= tolerance)
┌─────────────────────────────────────────────────────────────┐
│ TRANSFER WINDOW EVALUATION                                  │
│ - Window Open == false: FlowEdge capacity F_max = 0 (Wait)  │
│ - Window Open == true:  Allocate up to F_max micro-units    │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│ IN-FLIGHT BALLISTIC PACKET                                  │
│ - Freeze absolute arrival: arrival_tick = CurrentTick + tau │
│ - Packet sits in Edge Transit FIFO Buffer                   │
└──────────────────────────────┬──────────────────────────────┘
                               │
               (Condition: CurrentTick == arrival_tick)
                               ▼
┌─────────────────────────────────────────────────────────────┐
│ DESTINATION ORBITAL STAGING STOCK                           │
│ - Deduct transmission loss: received = mul_fixed(Q, eta)    │
│ - Deposit received quantity into orbital staging stock      │
└──────────────────────────────┬──────────────────────────────┘
                               │
       ▼ (Gateway Converter: Aerobraking / De-orbit Shuttle)
[ DESTINATION SURFACE STOCK ]
```

### 3.2 Signal Routing & Occlusion Decision Truth Table

| Direct Line Distance | Solar Ray Distance `d_sun` | Available Relay Nodes | Network Edge Routing State |
| :--- | :--- | :--- | :--- |
| Any | `> R_SUN_EXCLUSION_KM` | Any | `DIRECT_LINE_OF_SIGHT` (Latency = `tau_c(direct)`) |
| Any | `<= R_SUN_EXCLUSION_KM` | Valid L4/L5 Relay Path | `REROUTED_VIA_RELAY` (Latency = `tau_c(hop1) + tau_c(hop2)`) |
| Any | `<= R_SUN_EXCLUSION_KM` | None / Destroyed | `CONJUNCTION_BLACKOUT` (Buffer packets at source) |

---

## 📊 4. Constants, Figures & Baseline Data Tables

| Parameter / Constant | Exact Value | Standard Unit | Notes |
| :--- | :--- | :--- | :--- |
| `MU_SUN_KM3_TICK2` | `4_786_915_000_000` | `km^3 / tick^2` | `1.3271244e11 km^3/s^2 * 3600` |
| `R_SUN_EXCLUSION_KM` | `5_000_000` | km | Solar interference and physical plasma boundary |
| `LAUNCH_WINDOW_TOLERANCE_MICRO` | `87_266` | micro-radians | +/- 5.0 degrees allowable departure spread |
| `SPEED_OF_LIGHT_KM_TICK` | `17_987_547` | km / tick | `299_792.458 km/s * 60` |

### 4.1 Nominal Transfer Latencies (Ballistic Hohmann)

| Origin | Destination | Optimal Departure Angle `phi_opt` | Hohmann Flight Duration `tau_transfer` | Synodic Period |
| :--- | :--- | :--- | :--- | :--- |
| `EARTH` | `MARS` | `+774_926` micro-rad (+44.4 deg) | `373_000 ticks` (~259 days) | `1_123_200 ticks` (~780 days) |
| `MARS` | `EARTH` | `-1_312_000` micro-rad (-75.2 deg)| `373_000 ticks` (~259 days) | `1_123_200 ticks` (~780 days) |
| `EARTH` | `LUNA` | Non-Hohmann direct injection | `4_320 ticks` (~3 days) | Continuous (No synodic gate) |
| `EARTH` | `CERES` | `+1_152_000` micro-rad (+66.0 deg) | `678_000 ticks` (~471 days) | `672_000 ticks` (~467 days) |

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Continuous Interplanetary Shipping):** Never permit continuous bulk mass transfer between planets without transfer windows. Simulating continuous daily shipments without massive delta-V penalties violates celestial mechanics.
- **Trap 2 (Dynamic Re-computation of In-Flight Arrival):** Once a cargo packet departs, its arrival tick is fixed. Do not recalculate `arrival_tick` as the destination moves, because ballistic trajectories are solved for future intercept.
- **Trap 3 (Instantaneous Market Intelligence):** An AGI located on Mars cannot know Earth stock prices instantly. All market and sociopolitical telemetry from Earth must arrive via the light-speed latency buffer `tau_c`.
- **Trap 4 (Conjunction Oversight):** When Mars and Earth are in solar conjunction (on opposite sides of the Sun), direct radio transmission is physically impossible due to solar plasma noise. Omitting solar exclusion causes impossible communications through the Sun.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Input | Expected Output | Tolerance |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-LOG-01` | Earth-to-Mars Hohmann Flight Duration | `r1 = 149_597_870`, `r2 = 227_939_200` | `tau_transfer = 373_000 ticks` | `+/- 2_000 ticks` |
| `VEC-LOG-02` | Earth-to-Mars Launch Window Check | `delta_theta = 775_000`, `phi_opt = 774_926` | `window_open = true` (within tolerance) | Exact boolean |
| `VEC-LOG-03` | Solar Conjunction Raycast Occlusion | `P_src = (-150M, 0)`, `P_dest = (228M, 0)` | `is_occluded = true` (`d_sun = 0 <= 5M`) | Exact boolean |
| `VEC-LOG-04` | Non-Occluded Signal Line | `P_src = (0, 150M)`, `P_dest = (228M, 0)` | `is_occluded = false` (`d_sun = 124.7M`) | Exact boolean |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Interplanetary Logistics, Transfer Windows & Relativistic Latency (PHY-002)",
  "category": "specification",
  "key_invariants_formulas": "Hohmann transfer duration, launch window gating, speed of light causal horizons, solar conjunction occlusion",
  "status": "approved"
}
```

