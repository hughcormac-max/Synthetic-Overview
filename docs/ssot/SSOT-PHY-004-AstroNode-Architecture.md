---
id: SSOT-PHY-004
title: "AstroNode Architecture & Spherical Fibonacci Surface Topography"
domain: "Spatial Geometry / Planetary Topography"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-20
sources:
  - "Álvaro González, 'Measurement of Areas on a Sphere Using Fibonacci and Latitude-Longitude Lattices', Mathematical Geosciences, 2010"
  - "Richard Swinbank and James C. Purser, 'Fibonacci grids on the sphere', Quarterly Journal of the Royal Meteorological Society, 2006"
  - "SSOT-SYS-000: Universal SI Units & Dimensional Metrology"
  - "SSOT-PHY-001: 2D Keplerian Astrodynamics"
---

# SSOT-PHY-004: AstroNode Architecture & Spherical Fibonacci Surface Topography

> **SSOT ID:** `SSOT-PHY-004` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-20
> **Authoritative Sources:** González (2010), Swinbank & Purser (2006), SSOT-SYS-000, SSOT-PHY-001

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the authoritative architecture of celestial bodies (`AstroNodes`) and their discrete, interactable surface locations (`SurfaceNodes`):
1. **AstroNode Definition.** Every astronomical body (star, planet, moon, asteroid, comet) is defined as an `AstroNode`.
2. **2D Keplerian Orbits.** The solar system simulation is strictly 3D, but all planetary orbits are coplanar. Thus, `AstroNode` positions are derived using 2D Keplerian elements relative to their parent body.
3. **Spherical Fibonacci Lattice.** Surface interaction points (`SurfaceNodes`) are distributed across the sphere using a deterministic spherical Fibonacci spiral algorithm based on the golden ratio angle, guaranteeing uniform equal-area coverage without polar clustering.
4. **Viable Entity Locations.** `SurfaceNodes` are the only valid locations for entities, structures, or landed vessels on a celestial body.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict Coplanar Orbit).** All `AstroNode` orbits are strictly 2D and coplanar. The `z` or inclination coordinate for any macroscopic orbit is exactly `0.0`.
- **Invariant 2 (Radius-Proportional Allocation).** The total number of `SurfaceNodes` on an `AstroNode` may be allocated via explicit count or derived proportionally from physical radius `R_m` and density factor `K_density`.
- **Invariant 3 (Spherical Fibonacci Distribution).** `SurfaceNodes` on any spherical body must be generated strictly via the deterministic spherical Fibonacci spiral lattice. Random sampling, geodesic Voronoi meshes, and hierarchical hexagonal grids (such as Uber H3) are prohibited.
- **Invariant 4 (Exclusive Occupancy).** Macroscopic surface entities and landed vessels may only exist precisely at the discrete `(X_local, Y_local, Z_local)` coordinates of a generated `SurfaceNode`. Arbitrary surface coordinate parking is forbidden.

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 AstroNode Definition & Variables

An `AstroNode` requires the following defining parameters:

| Variable | Meaning | Standard Unit (SI) |
| :--- | :--- | :--- |
| `name` | Canonical string identifier | String |
| `parent_id` | Identifier of parent `AstroNode` | UUID / String (null for Root) |
| `mass` | Total physical mass | Kilograms (`kg`) |
| `radius` | Physical planetary radius `R_m` | Meters (`m`) |
| `a` | Semi-major axis | Meters (`m`) |
| `e` | Eccentricity | Dimensionless (`0.0 <= e < 1.0`) |
| `nu_0` | True anomaly at epoch | Radians (`rad`) |
| `varpi` | Longitude of periapsis | Radians (`rad`) |
| `n` | Mean motion | Radians/second (`rad/s`) |

### 2.2 2D Coplanar Keplerian Position

To find the global Cartesian coordinates `(X_global, Y_global, Z_global)` of an `AstroNode` at time `T` (seconds since epoch):

- **Mean Anomaly (`M`).**
  `M = (n * T) mod TWO_PI`

- **Eccentric Anomaly (`E`) Approximation (Newton-Raphson).**
  Solve `E - e * sin(E) = M` iteratively.

- **True Anomaly (`nu`).**
  `nu = 2.0 * atan2(sqrt(1.0 + e) * sin(E / 2.0), sqrt(1.0 - e) * cos(E / 2.0))`

- **Orbital Radius (`r`).**
  `r = a * (1.0 - e * cos(E))`

- **Parent-Relative 2D Position.**
  `X_rel = r * cos(nu + varpi)`
  `Y_rel = r * sin(nu + varpi)`
  `Z_rel = 0.0`

- **Global 3D Position.**
  `X_global = X_parent + X_rel`
  `Y_global = Y_parent + Y_rel`
  `Z_global = Z_parent + 0.0`

### 2.3 Spherical Fibonacci Surface Lattice

For an `AstroNode` with surface radius `R_m` (in meters) allocated `N` discrete `SurfaceNodes`:
For point index `i` from `0` to `N - 1`:

- **Z Coordinate Fraction (Equal-Area Cylindrical Projection):**
  `z_fraction_i = 1.0 - (2.0 * i + 1.0) / N`
  where `z_fraction_i` spans the open interval `(-1.0, 1.0)`.

- **Azimuthal Angle (Golden Ratio Stepping in radians):**
  `phi_i = (i * GOLDEN_ANGLE_RAD) mod TWO_PI`
  where `GOLDEN_ANGLE_RAD = 2.399963229728653` radians.

- **Radial Horizontal Scale (`r_xy_fraction`):**
  `r_xy_fraction = sqrt(1.0 - z_fraction_i * z_fraction_i)`

- **Local Cartesian Coordinates `(X_local, Y_local, Z_local)` in meters:**
  `X_local[i] = R_m * r_xy_fraction * cos(phi_i)`
  `Y_local[i] = R_m * r_xy_fraction * sin(phi_i)`
  `Z_local[i] = R_m * z_fraction_i`

- **Global Cartesian Coordinates:**
  `X_global[i] = X_parent + X_local[i]`
  `Y_global[i] = Y_parent + Y_local[i]`
  `Z_global[i] = Z_parent + Z_local[i]`

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Surface Node Resolution

When a descending vessel crosses the `AstroNode` Karman line, it must target a valid `SurfaceNode`.

```text
┌─────────────────────────────────────────────────────────────┐
│                 ATMOSPHERIC / KARMAN DESCENT                │
│   - Vessel orbital path crosses Karman altitude             │
│   - Target selection mode activated                         │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                   NEAREST NODE RESOLUTION                   │
│   - Query spatial index for nearest SurfaceNode using       │
│     projected surface intersection coordinate               │
└──────────────┬──────────────────────────────┬───────────────┘
               │                              │
     (Node is Occupied)                       │ (Node is Vacant)
               ▼                              ▼
┌──────────────────────────────┐    ┌──────────────────────────────┐
│       LANDING ABORTED        │    │       LANDING COMPLETE       │
│  - Vessel enters holding     │    │  - Vessel anchors at node    │
│  - or selects next nearest   │    │  - Inventory / State merges  │
└──────────────────────────────┘    └──────────────────────────────┘
```

---

## ⚠️ 4. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (3D Orbital Inclination).** Hallucinating full 3D Keplerian mechanics (inclination `i`, right ascension of ascending node `Omega`). This simulation strictly enforces a 2D coplanar macroscopic orbital space (`Z_rel = 0.0`).
- **Trap 2 (Hierarchical Hex Grids / H3 / Voronoi).** Attempting to generate planetary grids using Uber H3 or random Voronoi diagrams. Planetary surface nodes must strictly use the **Spherical Fibonacci Lattice**, which guarantees uniform node spacing with zero pentagonal defects.
- **Trap 3 (Continuous Surface Parking).** Allowing a vessel to land at any arbitrary latitude/longitude. Entities can only exist at defined `SurfaceNodes`.
- **Trap 4 (Zero-LaTeX Notation).** Writing equations with LaTeX tags (`$`, `$$`, `\dot`). Adhere strictly to ASCII plain text.

---

## 🧪 5. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Input Parameters | Expected Output | Tolerance |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-TOP-01` | Fibonacci Sphere North Pole Apex | `i = 0`, `N = 10_000`, `R = 6371000.0 m` | `Z_local[0] = 6370362.9 m`, `X_local[0] = 0.0 m`, `Y_local[0] = 0.0 m` | `+/- 1.0 m` |
| `VEC-TOP-02` | Fibonacci Sphere South Pole Apex | `i = 9_999`, `N = 10_000`, `R = 6371000.0 m` | `Z_local[9999] = -6370362.9 m`, `X_local[9999] = 0.0 m`, `Y_local[9999] = 0.0 m` | `+/- 1.0 m` |
| `VEC-TOP-03` | Equatorial Node Radius Consistency | `i = 5_000`, `N = 10_000`, `R = 6371000.0 m` | `sqrt(X^2 + Y^2 + Z^2) = 6371000.0 m` | `+/- 1.0 m` |
| `VEC-TOP-04` | Golden Angle First Step | `i = 1`, `GOLDEN_ANGLE = 2.399963` | `phi_1 = 2.399963 rad` (~137.51 deg) | `+/- 0.0001` |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "AstroNode Architecture & Spherical Fibonacci Surface Topography (PHY-004)",
  "category": "specification",
  "key_invariants_formulas": "2D coplanar Keplerian orbits, Spherical Fibonacci surface nodes, discrete entity locations",
  "status": "approved"
}
```
