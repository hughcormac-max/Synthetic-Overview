---
id: SSOT-PHY-004
title: "Spherical Fibonacci Surface Topography & Spatial Indexing"
domain: "Spatial Geometry / Planetary Topography"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Álvaro González, 'Measurement of Areas on a Sphere Using Fibonacci and Latitude-Longitude Lattices', Mathematical Geosciences, 2010"
  - "Richard Swinbank and James C. Purser, 'Fibonacci grids on the sphere', Quarterly Journal of the Royal Meteorological Society, 2006"
  - "SSOT-SYS-000: Universal SI Units & Dimensional Metrology"
---

# SSOT-PHY-004: Spherical Fibonacci Surface Topography & Spatial Indexing

> **SSOT ID:** `SSOT-PHY-004` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** González (2010), Swinbank & Purser (2006), SSOT-SYS-000

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the deterministic spatial topology of planetary surface micro space for all celestial bodies (`Astronodes`):
1. **Isolated Local 3D Spherical Coordinate Space:** Each major body possesses an independent 3D Cartesian reference frame centered on its barycenter `(0, 0, 0)`. Planetary surfaces are modeled as discrete spherical shells of radius `R_km` decoupled from heliocentric space.
2. **Spherical Fibonacci Lattice (Golden Ratio):** To guarantee strictly uniform spatial density without polar clustering, geometric distortion, or hex-grid seam stitching errors, all discrete surface interaction points (`SurfaceNodes`) are distributed across the sphere using a deterministic spherical Fibonacci spiral algorithm.
3. **Spatial Indexing & Boundary Transitions:** Point queries, proximity indexing, surface pathfinding, and landing descent site selections are resolved via a balanced 3D spatial KD-Tree. Atmospheric boundary crossing is governed by the body's **Karman Line**.

Interplanetary orbital trajectories, rocket equations, and SOI transfers are defined in `SSOT-PHY-001`.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict Spherical Fibonacci Distribution):** Surface nodes on any spherical celestial body must be generated strictly via the deterministic spherical Fibonacci spiral lattice. Random sampling, geodesic Voronoi meshes, and hierarchical hexagonal grids (such as Uber H3) are strictly prohibited.
- **Invariant 2 (Barycentric Coordinate Isolation):** Surface node positions are stored strictly in local 3D Cartesian coordinates `(X_local, Y_local, Z_local)` relative to the body barycenter in SI kilometers or meters. Combining surface coordinates with interplanetary heliocentric coordinates inside a single 3D world space is forbidden.
- **Invariant 3 (Fixed-Point Trigonometric Lookup):** All azimuthal stepping angles and elevation fractions must be computed via integer fixed-point arithmetic (`FIXED_POINT_SCALE = 1_000_000`) and precomputed integer sine/cosine lookup tables. Direct runtime calls to floating-point transcendental functions (`f32::sin`, `f64::cos`) are strictly prohibited.
- **Invariant 4 (Karman Line Boundary Isolation):** A vessel in orbital space cannot directly access or interact with a `SurfaceNode` without executing an explicit Karman boundary descent sequence or interfacing with an established surface-to-orbit gateway converter (e.g. space elevator or mass driver).

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Spherical Fibonacci Surface Lattice

For an `Astronode` with surface radius `R_km` allocated `N` discrete `SurfaceNodes`:
For point index `i` from `0` to `N - 1`:

- **Z Coordinate Fraction (Equal-Area Cylindrical Projection):**
  `z_fraction_i = 1_000_000 - ((2 * i + 1) * 1_000_000) / N`
  where `z_fraction_i` spans the closed interval `[-1_000_000, +1_000_000]`.

- **Azimuthal Angle (Golden Ratio Stepping in micro-radians):**
  `phi_i = (i * GOLDEN_ANGLE_MICRO) mod TWO_PI_MICRO`
  where `GOLDEN_ANGLE_MICRO = 2_399_963` micro-radians (approximately `137.507764` degrees).

- **Radial Horizontal Scale (`r_xy_fraction`):**
  `r_xy_sq = 1_000_000_000_000 - z_fraction_i * z_fraction_i`
  `r_xy_fraction = sqrt_int64(r_xy_sq) / 1_000`

- **Local Cartesian Coordinates `(X_local, Y_local, Z_local)` in km:**
  `X_local[i] = (R_km * mul_fixed(r_xy_fraction, cos_table(phi_i))) / FIXED_POINT_SCALE`
  `Y_local[i] = (R_km * mul_fixed(r_xy_fraction, sin_table(phi_i))) / FIXED_POINT_SCALE`
  `Z_local[i] = (R_km * z_fraction_i) / FIXED_POINT_SCALE`

### 2.2 Surface Great-Circle Distance

For two surface points `P1 = (X1, Y1, Z1)` and `P2 = (X2, Y2, Z2)` on a sphere of radius `R_km`:

- **Normalized Dot Product (Fixed-Point Cosine of Central Angle):**
  `dot_product = (X1 * X2 + Y1 * Y2 + Z1 * Z2) / (R_km * R_km)`
  `cos_sigma = clamp_fixed(dot_product, -1_000_000, 1_000_000)`

- **Central Angular Separation (micro-radians):**
  `sigma_urad = arccos_table(cos_sigma)`

- **Surface Great-Circle Distance (`d_surface` in km):**
  `d_surface = (R_km * sigma_urad) / FIXED_POINT_SCALE`

### 2.3 3D Spatial KD-Tree Indexing

To resolve proximity queries, line-of-sight targeting, and landing destination snaps in `O(log N)` time:
- **Node Data Structure:** Each tree node contains `(SurfaceNodeID, X_local, Y_local, Z_local, LeftChildIndex, RightChildIndex)`.
- **Partitioning Strategy:** Split dimension cycles cyclically based on tree depth:
  - `depth mod 3 == 0`: Split on `X_local`
  - `depth mod 3 == 1`: Split on `Y_local`
  - `depth mod 3 == 2`: Split on `Z_local`
- **Median Selection & Tie-Breaking:** Subarrays at each depth are partitioned by median coordinate. Ties are strictly broken by choosing the lowest `SurfaceNodeID`.

### 2.4 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit (SI) | Valid Range |
| :--- | :--- | :--- | :--- |
| `R_km` | Celestial body radius | Kilometers (`km`) | `R_km > 0` |
| `N` | Total Fibonacci surface nodes | Integer count | `100 <= N <= 100_000` |
| `i` | Discrete surface node index | Zero-based integer | `0 <= i < N` |
| `z_fraction_i` | Normalized vertical elevation fraction | Micro-units (`1e-6`) | `-1_000_000 <= z <= +1_000_000` |
| `phi_i` | Azimuthal Golden angle | Micro-radians (`urad`) | `0 <= phi_i < 6_283_185` |
| `X, Y, Z` | Local barycentric coordinates | Kilometers (`km`) | `-R_km <= X, Y, Z <= +R_km` |
| `h_alt` | Altitude above surface | Kilometers (`km`) | `0 <= h_alt <= SOI_RADIUS` |
| `d_surface` | Great-circle distance along surface | Kilometers (`km`) | `0 <= d_surface <= pi * R_km` |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Spatial Boundary & Landing Transition State Machine

```text
┌─────────────────────────────────────────────────────────────┐
│                    LOCAL ORBIT / PARKING                    │
│   - Frame: 2D/3D Body-Centric                               │
│   - Altitude: h_alt = d_body - R_km                         │
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
| `<= HIT_TOLERANCE_KM` | `<= MAX_NEIGHBOR_RADIUS` | Friendly / Neutral / Empty | `PERMITTED_LANDING` | Berthing at `SurfaceNodeID` |
| `<= HIT_TOLERANCE_KM` | `<= MAX_NEIGHBOR_RADIUS` | Hostile Air Defense Active | `DENIED_INTERCEPTED` | Kinetic surface battery intercept |
| `<= HIT_TOLERANCE_KM` | `> MAX_NEIGHBOR_RADIUS` | Any | `SNAP_TO_CLOSEST_NODE` | Clamps to nearest valid node |
| `> HIT_TOLERANCE_KM` | Any | Any | `MISS_CONTINUE_ORBIT` | Skips atmosphere; maintains orbit |

---

## 📊 4. Constants, Figures & Baseline Data Tables

### 4.1 Planetary Surface & Topography Constants

| Constant | Exact Value | Standard Unit | Notes |
| :--- | :--- | :--- | :--- |
| `GOLDEN_ANGLE_MICRO` | `2_399_963` | `urad` | Golden Angle `pi * (3 - sqrt(5)) * 1e6` (~137.508 deg) |
| `PI_MICRO` | `3_141_593` | `urad` | Half-circle angle |
| `TWO_PI_MICRO` | `6_283_185` | `urad` | Full-circle angle |
| `FIXED_POINT_SCALE` | `1_000_000` | dimensionless | Standard `1e6` integer divisor |

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
- **Trap 2 (Universal 3D World Space):** Storing surface nodes in heliocentric coordinates (e.g. Earth surface nodes at `149_597_870 km + 6_371 km`). This causes massive 32-bit floating-point precision jitter. Surface nodes must strictly reside in body-local coordinates `(X_local, Y_local, Z_local)`.
- **Trap 3 (Runtime Transcendental Calls):** Calling `f32::sin()`, `f64::cos()`, or `f64::atan2()` at runtime for surface points. All angle and coordinate generation must query the deterministic integer fixed-point lookup tables.
- **Trap 4 (Zero-LaTeX Notation):** Writing equations with single or double dollar delimiters or LaTeX macros. Adhere strictly to the project-wide ASCII plain text standard.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Input Parameters | Expected Output | Tolerance |
| :--- | :--- | :--- | :--- | :--- |
| `VEC-TOP-01` | Fibonacci Sphere North Pole Apex | `i = 0`, `N = 10_000`, `R = 6_371 km` | `Z_local[0] = 6_370 km`, `X_local[0] = 0 km`, `Y_local[0] = 0 km` | `+/- 1 km` |
| `VEC-TOP-02` | Fibonacci Sphere South Pole Apex | `i = 9_999`, `N = 10_000`, `R = 6_371 km` | `Z_local[9999] = -6_370 km`, `X_local[9999] = 0 km`, `Y_local[9999] = 0 km` | `+/- 1 km` |
| `VEC-TOP-03` | Equatorial Node Radius Consistency | `i = 5_000`, `N = 10_000`, `R = 6_371 km` | `sqrt(X^2 + Y^2 + Z^2) = 6_371 km`, `Z_local approx 0 km` | `+/- 1 km` |
| `VEC-TOP-04` | Golden Angle First Step | `i = 1`, `GOLDEN_ANGLE_MICRO = 2_399_963` | `phi_1 = 2_399_963 urad` (~137.51 deg) | Exact integer |
| `VEC-TOP-05` | Antipodal Great-Circle Distance | `P1 = (0, 0, 6371)`, `P2 = (0, 0, -6371)`, `R = 6_371` | `d_surface = 20_015 km` (`pi * R`) | `+/- 2 km` |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Spherical Fibonacci Surface Topography & Spatial Indexing (PHY-004)",
  "category": "specification",
  "key_invariants_formulas": "Spherical Fibonacci spiral lattice, Golden angle stepping, local 3D barycentric coordinate isolation, KD-Tree spatial queries, Karman boundary transitions",
  "status": "approved"
}
```
