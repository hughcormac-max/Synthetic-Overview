---
id: SSOT-PHY-004
title: "Spherical Fibonacci Surface Topography & Spatial Indexing"
domain: "Spatial Geometry / Planetary Topography"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-17
sources:
  - "Álvaro González, 'Measurement of Areas on a Sphere Using Fibonacci and Latitude-Longitude Lattices', Mathematical Geosciences, 2010"
  - "Richard Swinbank and James C. Purser, 'Fibonacci grids on the sphere', Quarterly Journal of the Royal Meteorological Society, 2006"
  - "SSOT-SYS-000: Universal SI Units & Dimensional Metrology"
---

# SSOT-PHY-004: Spherical Fibonacci Surface Topography & Spatial Indexing

> **SSOT ID:** `SSOT-PHY-004` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-17
> **Authoritative Sources:** González (2010), Swinbank & Purser (2006), SSOT-SYS-000

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the deterministic spatial topology of planetary surface micro space for all celestial bodies (`Astronodes`):
1. **Unified 3D Coordinate Space:** Planetary surfaces are integrated directly into the global `f64` 3D Cartesian reference frame. Surface nodes and orbital paths reside in the same global space, utilizing the sub-millimeter precision of 64-bit floats to eliminate boundary jitter.
2. **Spherical Fibonacci Lattice (Golden Ratio):** To guarantee strictly uniform spatial density without polar clustering, geometric distortion, or hex-grid seam stitching errors, all discrete surface interaction points (`SurfaceNodes`) are distributed across the sphere using a deterministic spherical Fibonacci spiral algorithm.
3. **Spatial Indexing & Boundary Transitions:** Point queries, proximity indexing, surface pathfinding, and landing descent site selections are resolved via a balanced 3D spatial KD-Tree. Atmospheric boundary crossing is governed by the body's **Karman Line**.

Interplanetary orbital trajectories, rocket equations, and SOI transfers are defined in `SSOT-PHY-001`.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict Spherical Fibonacci Distribution):** Surface nodes on any spherical celestial body must be generated strictly via the deterministic spherical Fibonacci spiral lattice. Random sampling, geodesic Voronoi meshes, and hierarchical hexagonal grids (such as Uber H3) are strictly prohibited.
- **Invariant 2 (Unified f64 Scale):** Surface node positions are generated in local barycentric coordinates and immediately transformed into global heliocentric `f64` 3D Cartesian coordinates. The use of 32-bit floats (`f32`) is forbidden to prevent spatial jitter.
- **Invariant 3 (f64 Determinism):** All azimuthal stepping angles and elevation fractions must be computed via 64-bit IEEE-754 floating point arithmetic. Hardware transcendental instructions (e.g., `sin`, `cos`) are strictly prohibited; calculations must use deterministic software math libraries under strict WebAssembly/IEEE-754 semantics.
- **Invariant 4 (Karman Line Boundary):** A vessel in orbital space cannot directly access or interact with a `SurfaceNode` without executing an explicit Karman boundary descent sequence or interfacing with an established surface-to-orbit gateway converter (e.g. space elevator or mass driver).

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Spherical Fibonacci Surface Lattice

For an `Astronode` with surface radius `R_m` (in meters) allocated `N` discrete `SurfaceNodes`:
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
  `X_global[i] = X_body + X_local[i]`
  `Y_global[i] = Y_body + Y_local[i]`
  `Z_global[i] = Z_body + Z_local[i]`

### 2.2 Surface Great-Circle Distance

For two local surface points `P1 = (X1, Y1, Z1)` and `P2 = (X2, Y2, Z2)` on a sphere of radius `R_m`:

- **Normalized Dot Product (Cosine of Central Angle):**
  `dot_product = (X1 * X2 + Y1 * Y2 + Z1 * Z2) / (R_m * R_m)`
  `cos_sigma = clamp(dot_product, -1.0, 1.0)`

- **Central Angular Separation (radians):**
  `sigma_rad = acos(cos_sigma)`

- **Surface Great-Circle Distance (`d_surface` in meters):**
  `d_surface = R_m * sigma_rad`

### 2.3 3D Spatial KD-Tree Indexing

To resolve proximity queries, line-of-sight targeting, and landing destination snaps in `O(log N)` time:
- **Node Data Structure:** Each tree node contains `(SurfaceNodeID, X_local, Y_local, Z_local, LeftChildIndex, RightChildIndex)`. Local coordinates are used for the tree to maintain fixed spatial relationships regardless of orbital translation.
- **Partitioning Strategy:** Split dimension cycles cyclically based on tree depth:
  - `depth mod 3 == 0`: Split on `X_local`
  - `depth mod 3 == 1`: Split on `Y_local`
  - `depth mod 3 == 2`: Split on `Z_local`
- **Median Selection & Tie-Breaking:** Subarrays at each depth are partitioned by median coordinate. Ties are strictly broken by choosing the lowest `SurfaceNodeID`.

### 2.4 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit (SI) | Valid Range |
| :--- | :--- | :--- | :--- |
| `R_m` | Celestial body radius | Meters (`m`) | `R_m > 0.0` |
| `N` | Total Fibonacci surface nodes | Integer count | `100 <= N <= 100_000` |
| `i` | Discrete surface node index | Zero-based integer | `0 <= i < N` |
| `z_fraction_i` | Normalized vertical elevation fraction | Dimensionless | `-1.0 < z < 1.0` |
| `phi_i` | Azimuthal Golden angle | Radians (`rad`) | `0.0 <= phi_i < 6.2831853` |
| `X, Y, Z` | Local barycentric coordinates | Meters (`m`) | `-R_m <= X, Y, Z <= +R_m` |
| `h_alt` | Altitude above surface | Meters (`m`) | `0.0 <= h_alt <= SOI_RADIUS` |
| `d_surface` | Great-circle distance along surface | Meters (`m`) | `0.0 <= d_surface <= pi * R_m` |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Spatial Boundary & Landing Transition State Machine

```text
┌─────────────────────────────────────────────────────────────┐
│                    LOCAL ORBIT / PARKING                    │
│   - Frame: 3D Body-Centric or Heliocentric                  │
│   - Altitude: h_alt = d_body - R_m                          │
│   - Boundary check: karman_alt < h_alt <= r_soi             │
└──────────────────────────────┬──────────────────────────────┘
                               │
                (Event: INITIATE_DE-ORBIT_BURN)
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 ATMOSPHERIC / KARMAN DESCENT                │
│   - Trajectory intersects Karman altitude: h_alt <= karman  │
│   - Raycast to spherical surface determines landing coords  │
└──────────────────────────────┬──────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                   KD-TREE SITE RESOLUTION                   │
│   - Query KD-Tree for nearest SurfaceNodeID                 │
│   - Evaluate surface air defense & site access constraints  │
└──────────────┬──────────────────────────────┬───────────────┘
               │                              │
     (Site Access Granted)                    │ (Air Defense Active / Hostile)
               ▼                              ▼
┌──────────────────────────────┐    ┌──────────────────────────────┐
│       LANDING_COMPLETE       │    │      DESCENT_INTERCEPTED     │
│  - Vessel berthed at node    │    │  - Vessel destroyed/damaged  │
│  - SurfaceNode inventory add │    │  - Kinetic impact event      │
└──────────────────────────────┘    └──────────────────────────────┘
```

### 3.2 KD-Tree Query & Landing Site Resolution Truth Table

| Raycast Intersection Distance | Nearest Node Distance | Node Ownership Status | Landing Sequence State | Action / Transition |
| :--- | :--- | :--- | :--- | :--- |
| `<= HIT_TOLERANCE_M` | `<= MAX_NEIGHBOR_RADIUS` | Friendly / Neutral / Empty | `PERMITTED_LANDING` | Berthing at `SurfaceNodeID` |
| `<= HIT_TOLERANCE_M` | `<= MAX_NEIGHBOR_RADIUS` | Hostile Air Defense Active | `DENIED_INTERCEPTED` | Kinetic surface battery intercept |
| `<= HIT_TOLERANCE_M` | `> MAX_NEIGHBOR_RADIUS` | Any | `SNAP_TO_CLOSEST_NODE` | Clamps to nearest valid node |
| `> HIT_TOLERANCE_M` | Any | Any | `MISS_CONTINUE_ORBIT` | Skips atmosphere; maintains orbit |

---

## 📊 4. Constants, Figures & Baseline Data Tables

### 4.1 Planetary Surface & Topography Constants

| Constant | Exact Value | Standard Unit | Notes |
| :--- | :--- | :--- | :--- |
| `GOLDEN_ANGLE_RAD` | `2.399963229728653` | `rad` | Golden Angle `pi * (3 - sqrt(5))` (~137.508 deg) |
| `PI` | `3.141592653589793` | `rad` | Half-circle angle |
| `TWO_PI` | `6.283185307179586` | `rad` | Full-circle angle |

### 4.2 Celestial Body Surface Node Allocations & Atmospheric Limits

| Astronode Name | Mean Radius `R` (km) | Surface Nodes `N` | Karman Altitude (km) | Surface Gravity (m/s^2) | Primary Biome Type |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `SUN` | `696_340` | `0` | N/A | `274.0` | Solar Plasma (Uninhabitable) |
| `EARTH` | `6_371` | `10_000` | `100` | `9.807` | Terrestrial / Oceanic / Urban |
| `LUNA` | `1_737` | `4_000` | `0` (Vacuum) | `1.622` | Regolith / Basaltic Mare |
| `MARS` | `3_390` | `8_000` | `80` | `3.721` | Iron Oxide Desert / Polar Ice |
| `CERES` | `473` | `1_000` | `0` (Vacuum) | `0.270` | Carbonaceous / Cryovolcanic |

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Hierarchical Hex Grids / H3 / Voronoi):** Attempting to generate planetary grids using Uber H3 or random Voronoi diagrams. Planetary surface nodes must strictly use the **Spherical Fibonacci Lattice**, which guarantees uniform node spacing with zero pentagonal defects.
- **Trap 2 (32-bit Floating Point Jitter):** Generating the global unified 3D space using 32-bit floats (`f32`). This causes massive jitter at solar system distances. The simulation must strictly adhere to 64-bit floats (`f64`), which provide sub-millimeter accuracy out to 50 AU, making the unified 3D space viable.
- **Trap 3 (Hardware Transcendental Drift):** Using standard hardware math instructions for transcendentals (e.g., `f64::cos`). Since determinism is critical for the ECS, hardware `cos`/`sin` is strictly forbidden. A deterministic software approximation library must be used.
- **Trap 4 (Zero-LaTeX Notation):** Writing equations with single or double dollar delimiters or LaTeX macros. Adhere strictly to the project-wide ASCII plain text standard.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

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
  "title_and_domain_scope": "Spherical Fibonacci Surface Topography & Spatial Indexing (PHY-004)",
  "category": "specification",
  "key_invariants_formulas": "Spherical Fibonacci spiral lattice, Golden angle stepping, global unified 3D f64 coordinate space, KD-Tree spatial queries",
  "status": "approved"
}
```
