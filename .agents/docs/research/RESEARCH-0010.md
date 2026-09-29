# RESEARCH-0010: Orbital Mechanics Synthesis - Transfers, Patched Conics, GVE, and Precision

> **Date:** 2026-09-12
> **Context:** Deep synthesis report analyzing the legacy `Keplerian-Elements` simulation architecture alongside mathematical web research and existing project architecture (`SSOT-003`).

---

## 1. Executive Summary
This report establishes a unified theoretical foundation for 2D co-planar orbital mechanics, deterministic Patched Conic approximations, continuous-thrust Gauss Variational Equations (GVE), Hohmann transfer mathematics, Sphere of Influence (SOI) boundary management, and strategies to mitigate floating-point precision collapse in astrodynamics software.

## 2. Keplerian Elements & Orbital Geometry
*   **2D Planar Constraint:** Both the legacy engine and the current physical architecture (`SSOT-003`) explicitly restrict the universe to a 2D ecliptic plane. Inclination and the Longitude of the Ascending Node are permanently neutralized (`inclination = 0`, `longAscNode = 0`). This optimization eliminates out-of-plane maneuver costs and vastly simplifies phase angle calculations.
*   **Orbital State & Propagations:** Orbits are resolved as simplified `OscullatingElements`. The instantaneous orbital velocity relies on the Vis-Viva Orbital Energy Equation: `epsilon = v^2 / 2 - mu / r = - mu / (2 * a)`.
*   **Anomaly Resolution:** Resolving future position (`t`) requires determining True Anomaly via Mean Anomaly (`M = M0 + n * t`). The legacy engine iteratively solves Kepler's equation (`M = E - e * sin(E)`) using 64-bit Newton-Raphson methods with fixed tolerances (`1e-12`) and strict iteration bounds.
*   **Lambert Solvers:** For intercept calculations between two points in time, universal variable Lambert-Stumpff solvers limit complexity to `O(1)`, using 4th-order Taylor series near zero (`|z| < 1e-6`) to mitigate singularities.

## 3. Spheres of Influence (SOI) & Patched Conics
Multi-body gravitation relies completely on Patched Conic approximations, switching reference frames dynamically at calculated boundaries.
*   **Laplace SOI:** Used universally for calculating handoff radii in patched conics, defined as `r_SOI = a * (m / M)^(2/5)`. This determines where the secondary body's gravity effectively dominates the primary's perturbation.
*   **Hill Sphere:** Defined as `r_H = a * (m / (3 * M))^(1/3)`. Crucially, this determines long-term orbital stability rather than immediate patched conic handoffs, and is always larger than Laplace SOI.
*   **Mathematical Transition Egress/Ingress:** To exit an SOI, the legacy engine analytically solves the exit True Anomaly via `nu = acos((p / r_SOI - 1) / e)`. For entries (flybys), a golden-section search (`0.6180339`) iteratively detects intersection nodes where `minDistance <= r_SOI`, projecting relative state vectors into the new parent's frame. Dual-threshold hysteresis (`1.01` to `1.05 * r_SOI`) prevents frame-flickering at the boundary edge.

## 4. Maneuvers, Acceleration & Transfers
The simulation supports both discrete impulsive chemical burns and continuous low-thrust propulsion.
*   **Impulsive Burns & Hohmann Transfers:** Standard Hohmann transfers (the most efficient 2-impulse coplanar transfers) calculate Delta-V as:
    *   `Delta_v1 = sqrt(mu / r1) * (sqrt(2 * r2 / (r1 + r2)) - 1)`
    *   `Delta_v2 = sqrt(mu / r2) * (1 - sqrt(2 * r1 / (r1 + r2)))`
    *   Burn durations use the Tsiolkovsky equation: `t_burn = ((m_0 * I_sp * g_0) / F_thrust) * (1 - exp(-Delta-v / (I_sp * g_0)))`.
    *   Due to the strict 2D planar environment, classic 3D Porkchop Plots (which visualize characteristic energy `C3 = v_inf^2` mapped against launch/arrival dates) become mathematically trivial, resolving entirely into coplanar phase angles.
*   **Continuous Acceleration (GVE):** Ion/Plasma low-thrust maneuvers transition out of standard Kepler elements (which encounter singularities at `e=0`) and into 2D In-Plane Equinoctial variables (`a`, `p_x`, `p_y`, `lambda`). 
    *   For highly eccentric orbits, states are integrated using a 4th-Order Runge-Kutta scheme, natively conserving specific energy and angular momentum.
    *   For near-circular low-thrust spirals (`e < 0.05`), the simulation completely bypasses RK4, invoking closed-form Edelbaum analytical equations (`dr/dt = 2 * a_thrust * sqrt(r^3 / mu)`) yielding sub-millisecond execution times.

## 5. Floating Point Precision Mitigation
Simulating astronomical scales in game engines inevitably triggers IEEE 754 precision loss.
*   **Data Types:** All astrodynamics logic must enforce `Float64Array` usage. 32-bit float provides ~7 digits of precision (incapable of millimeter precision over millions of kilometers), while 64-bit double provides ~15-17.
*   **Hierarchical Reference Frames:** Position vectors are never calculated relative to the absolute center of the universe (e.g. the Sun). Coordinates are stored relative to the dominant local gravitational body (Local Barycentric Origin).
*   **Modulo Wrapping:** Continuous variables that scale infinitely (like Mean Anomaly `M`) are heavily clamped via `((M_start + n * dt) % (2 * PI) + 2 * PI) % (2 * PI)` to prevent mantissa explosion.
*   **Kahan Summation & Clamping:** When accumulating millions of tiny delta-vs (e.g. numerical integration), Kahan Compensated Summation is recommended to retain low-order bits. Furthermore, domain inputs to inverse trigonometric functions must strictly clamp to `max(-1, min(1, val))` to prevent NaN cascades due to sub-decimal drift.
