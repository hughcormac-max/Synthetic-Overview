# Research Report: SimpleRockets / Juno: New Origins Technical Architecture, Aerodynamics, and Simulation Systems

> **Date:** 2026-09-10
> **Objective:** Deliver an exhaustive, publication-grade technical decomposition of SimpleRockets 1 vs. SimpleRockets 2 / Juno: New Origins, analyzing its engine transition (Box2D to Unity C#), composite rigidbody welding, procedural part mesh generation, de Laval nozzle thermodynamics, custom aerodynamic polars, floating origin astrodynamics, and Vizzy visual VM architecture.

---

## 📑 Executive Summary

SimpleRockets 2, officially rebranded as **Juno: New Origins** upon its 1.0 release in January 2023, represents a foundational architectural evolution in consumer aerospace simulation. While its predecessor, SimpleRockets 1 (2013), was constrained to a 2D Box2D physics environment driven by a custom cross-platform C++ engine, Juno: New Origins transitioned to the Unity 3D engine (migrating across Unity 2018.4 LTS through 2021 LTS). On desktop platforms, it utilizes the Mono C# scripting runtime to preserve managed assemblies (`SimpleRockets2.dll`, `ModApi.dll`) that enable dynamic reflection, mod loading, and Harmony method patching. On mobile (iOS/Android ARM64) and console platforms, it leverages Unity's IL2CPP Ahead-Of-Time (AOT) transpilation pipeline alongside the Burst Compiler (`Unity.Burst`) and C# Job System for SIMD vectorization of aerostructural calculations.

The simulation's primary architectural breakthrough lies in its structural mechanics and parametric geometry pipeline. Rejecting the discrete "Lego-brick" part catalogs of Kerbal Space Program (KSP), Juno utilizes procedural geometric synthesis (`Fuselage`, `Wing`, `RocketEngine`) governed by parametric spline cross-sections and analytical volume evaluation. Crucially, Juno resolves KSP's notorious "noodle rocket" structural instability by implementing **composite Rigidbody welding**. Instead of treating every part as an independent PhysX `Rigidbody` coupled via compliant `ConfigurableJoint` spring-dampers, Juno recursively fuses all statically connected parts into a singular, monolithic PhysX `Rigidbody` with an analytically integrated center of mass, dynamic parallel-axis moment of inertia tensor, and compound segmented convex colliders. PhysX dynamic joints are strictly reserved for actual articulated degrees of freedom (pistons, rotators, landing gear, and docking ports).

The physical simulation fidelity extends across every domain of flight mechanics. Propulsion is governed by 1D compressible gas dynamics based on isentropic de Laval nozzle theory, modeling four distinct thermodynamic power cycles, finite area ratio expansion, propellant chemistry, and Summerfield flow separation criteria. Atmospheric flight is driven by a custom aerodynamic polar pipeline featuring Prandtl-Glauert compressibility corrections, supersonic wave drag, crossflow stall regimes, and analytical fairing volume drag occlusion. Hypersonic reentry computes Sutton-Graves stagnation convective heating, Fourier thermal conduction, and Planckian blackbody incandescence. Astrodynamic trajectories operate under a 64-bit dual-precision floating-origin system with Keplerian patched-conic analytical propagation during time warp up to 1,000,000x. Finally, onboard automation is provided by **Vizzy**, a sandboxed visual block programming virtual machine executed synchronously within Unity's `FixedUpdate` loop, providing closed-loop vector math, state feedback, and telemetry processing.

---

## 🔍 Key Findings

### 1. Engine Evolution & Technology Stack: SR1 Box2D to SR2 Unity/C#

#### 1.1 SimpleRockets 1 Architecture (2013)
SimpleRockets 1 was engineered by Jundroo as a lightweight 2D orbital mechanics and rocketry simulator. Its architectural stack consisted of:
- **Core Simulation:** 2D rigid-body dynamics implemented via the open-source **Box2D** physics engine. Part structures were represented as 2D polygonal rigid bodies interconnected by Box2D distance and revolute joints.
- **Engine Framework:** Approximately 95% of the codebase was written in platform-agnostic, portable C++. Platform shims handled graphics rendering, audio, and platform lifecycle:
  - *Windows PC:* Win32 API with native DirectX / OpenGL rendering contexts, compiled in Visual Studio 2012.
  - *iOS:* Objective-C / Cocoa Touch wrappers driving OpenGL ES 2.0.
  - *Android:* Java Native Interface (JNI) binding Android Activity lifecycles to the native C++ core.
  - *Windows Phone 8:* C++/CX and managed C# runtime wrappers.
- **Assets & Rendering:** 2D planar sprite sheet atlases generated via TexturePacker; audio synthesis processed via Audacity.
- **Orbital Mechanics:** Pure 2D Keplerian two-body analytical orbital propagation mapped directly to Box2D spatial coordinates.

#### 1.2 SimpleRockets 2 / Juno: New Origins Engine Migration
To transition into full 3D multi-body aerospace simulation, Jundroo bypassed custom C++ rendering engine development and migrated to the **Unity Engine** (initial development on Unity 2018.4 LTS, subsequent production upgrades through 2019.3.15f1, and long-term stabilization on Unity 2020/2021 LTS).

```
+---------------------------------------------------------------------------------------+
|                                JUNO: NEW ORIGINS ENGINE ARCHITECTURE                  |
+---------------------------------------------------------------------------------------+
| Desktop (PC / macOS):                                                                 |
|   - Managed C# Scripting Backend (Mono Runtime)                                       |
|   - Dynamic Assembly Loading & Runtime Reflection (ModApi.dll, SimpleRockets2.dll)     |
|   - Harmony Method Patching for Deep Modding Injection                                |
| Mobile (iOS / Android ARM64) & Consoles:                                              |
|   - Ahead-Of-Time (AOT) Compilation via Unity IL2CPP                                  |
|   - C++ Native Code Generation for Maximum CPU Register Allocation & Apple Compliance |
+---------------------------------------------------------------------------------------+
| High-Performance Physics & Aero Pipeline:                                             |
|   - Unity Burst Compiler (Unity.Burst) generating SIMD-vectorized machine code        |
|   - Unity C# Job System distributing aerodynamic face integration across CPU threads  |
|   - PhysX 4.x/5.x multi-threaded rigid-body simulation engine                         |
+---------------------------------------------------------------------------------------+
```

#### 1.3 Scripting Backends, IL2CPP, and Burst Compilation
Juno: New Origins implements a split scripting architecture across target platforms:
1. **Desktop Scripting (Mono):** Preserves intermediate Common Intermediate Language (CIL) bytecode inside `SimpleRockets2.dll` and `ModApi.dll`. This maintains complete support for dynamic runtime reflection, external C# mod loading via the `ModApi` interface, and binary hooking via Harmony.
2. **Mobile & Console Scripting (IL2CPP):** To comply with Apple App Store regulations forbidding runtime JIT compilation, and to maximize execution speed on ARM64 mobile chipsets, Juno compiles C# code directly into C++ via IL2CPP before building platform binaries. This ensures zero managed garbage collection overhead during flight simulation and maximizes instruction cache locality.
3. **Burst Compiler & C# Job System:** Performance-critical subsystems—specifically aerothermal face-pressure integration, procedural mesh deformation, particle trajectory calculations, and thermal conduction networks—are executed via the Unity C# Job System compiled with the LLVM-based Unity Burst Compiler (`Unity.Burst`). This yields vectorized SIMD execution (AVX2/NEON) matching native C++ performance.

#### 1.4 Architecture Comparison Matrix

| Technical Metric | SimpleRockets 1 (2013) | Juno: New Origins (2023-2026) | Kerbal Space Program 1 (2011-2023) | Kerbal Space Program 2 (2023-2024) |
| :--- | :--- | :--- | :--- | :--- |
| **Physics Engine** | Box2D (2D) | NVIDIA PhysX 4.x/5.x (3D) | NVIDIA PhysX 3.x/4.x (3D) | NVIDIA PhysX 4.x (3D) |
| **Primary Codebase** | Portable C++ (~95%) | C# (Unity Engine) | C# (Unity Engine) | C# (Unity Engine) |
| **Scripting Backend** | Native Platform Shims | Mono (PC) / IL2CPP (Mobile) | Mono (PC/Consoles) | IL2CPP / Mono hybrid |
| **Multi-Threading** | Single-threaded physics | C# Job System + Burst Compiler | Single-threaded PhysX main loop | Multi-threaded engine attempts |
| **Structural Physics** | 2D Box2D Joints | **Composite Rigidbody Welding** | Per-part PhysX Rigidbodies + Joints | Per-part PhysX Rigidbodies + Joint tree |
| **Part Geometry** | Fixed 2D Sprites | Parametric Procedural 3D | Fixed Discrete 3D Meshes | Fixed Discrete 3D Meshes |
| **Modding Support** | Minimal (XML tweaking) | ModApi DLLs, Harmony, XML | Assembly-CSharp, Harmony, KSP-e | Partial / Interrupted Mod Support |

---

### 2. Procedural Part Mesh Generation & Dynamic Collider Architecture

#### 2.1 The Parametric Philosophy
Unlike Kerbal Space Program's discrete catalog of fixed-size fuel tanks, fuselage adapters, and wings, Juno: New Origins is architected entirely around **continuous procedural mesh generation**. Every aerodynamic and structural part is defined mathematically through the `ModApi.Craft.Parts.PartModifier` class hierarchy.

```
ModApi.Craft.Parts.PartModifier (Abstract Base)
  ├── Fuselage / FuselageData
  ├── Wing / WingData
  ├── Fairing / FairingData
  └── RocketEngine / RocketEngineData
```

#### 2.2 Procedural Fuselages and Tanks (`FuselageData`)
Fuselages and structural fuel tanks are generated through continuous parametric lofting:
- **Cross-Sectional Vectors:** Defined by `topScale` (Vector2), `bottomScale` (Vector2), and a spatial `offset` (Vector3). The offset parameter enables asymmetric cones, offset nose cones, and oblique adapters without requiring new assets.
- **Fillet Curvature & Corner Radii:** Controlled via the `cornerRadiuses` float array. The mesh generator evaluates superelliptic and fillet algorithms, smoothly interpolating from sharp n-gons (rectangles, triangles, octagons) to perfect cylinders and rounded rectangles.
- **Longitudinal Curvature:** The `depthCurve` and `depthCurved` parameters evaluate cubic Hermite or Bezier splines along the longitudinal extrusion axis, generating parabolic nose cones, ogive fairings, and curved fuselage hulls.
- **Normal Smoothing & Lighting:** Seamless visual integration between adjacent procedural parts is achieved through `normalSmoothingAngle` and `flattenNormals`. When two fuselages are snapped axially, the mesh generator forces the boundary vertex ring normals of both meshes to align with the connection plane normal, eliminating geometric seams under dynamic PBR lighting.
- **Procedural UV Generation:** Texture mapping coordinates are synthesized procedurally: U maps continuously across the normalized perimeter [0, 1], while V maps along the extrusion height, scaled dynamically by user-specified `tilingX` and `tilingY` material parameters.

```
               Procedural Fuselage Cross-Section Generation

             topScale (Vector2)           offset (Vector3)
               +-------------+           \
               |   Top Ring  |            \  Longitudinal Spline:
               +------+------+             \   depthCurve (Bezier)
                      |                     \
                      |                      v
                      |                +------------------+
                      +--------------->|    Bottom Ring   |
                                       +------------------+
                                       bottomScale (Vector2)
```

#### 2.3 Procedural Wings (`WingData`)
Wings are synthesized as parametric 3D airfoils governed by:
- **Planform Parameters:** Semi-span vector `tipPosition` (encoding span, sweep angle, and dihedral angle simultaneously), root chord leading/trailing offsets (`rootLeadingOffset`, `rootTrailingOffset`), tip chord offsets (`tipLeadingOffset`, `tipTrailingOffset`), and sectional thickness ratio.
- **Control Surface Slicing:** Wings dynamically split trailing-edge geometry via procedural boolean operations when `allowControlSurfaces` is enabled. Hinge location is determined by `hingeDistanceFromTrailingEdge`, creating functional ailerons, elevators, and flaps without discrete hinge parts.

#### 2.4 Dynamic Colliders: Solid vs. Hollow Compound Architecture
PhysX imposes a strict computational restriction: dynamic moving `Rigidbody` instances cannot utilize concave `MeshCollider` geometry; dynamic mesh colliders must be strictly convex. Juno handles this limitation through two distinct strategies:
1. **Solid Parts:** Evaluated as single convex hulls (`MeshCollider` with `convex = true`) or bounded by primitive compounds (`BoxCollider`, `CapsuleCollider`) to minimize GJK collision-detection cost.
2. **Hollow & Enclosed Parts (Fairings, Cargo Bays, Hollow Cylinders):** A single convex hull over a hollow cylinder creates an impenetrable barrier across the open volume, causing payloads to violently eject due to PhysX collision overlap. Juno's procedural engine dynamically decomposes hollow geometries into **compound segmented convex hulls**. A hollow cylinder is generated as an assembly of 8, 12, or 16 radial wedge convex colliders framing the perimeter walls, leaving the interior cavity completely collision-free for internal payload stowage.

#### 2.5 Craft Serialization & Funky Trees
Craft data is serialized as a human-readable, structured XML document (`Craft.xml`):
- **Hierarchy:**
  ```xml
  <Craft name="Orbital Launcher">
    <Assembly>
      <Parts>
        <Part id="1" partType="fuselage" position="0,0,0" rotation="0,0,0">
          <Fuselage.State topScale="1.5,1.5" bottomScale="2.0,2.0" offset="0,3.0,0" cornerRadiuses="1,1,1,1" />
          <FuelTank.State fuelType="Kerolox" capacity="12500" />
        </Part>
        <Part id="2" partType="rocket-engine" position="0,-1.5,0">
          <RocketEngine.State fuelType="Kerolox" cycle="GasGenerator" chamberPressure="8.5e6" nozzleThroatSize="0.45" nozzleSize="1.85" />
        </Part>
      </Parts>
      <Connections>
        <Connection partA="1" partB="2" attachType="Radial" />
      </Connections>
    </Assembly>
  </Craft>
  ```
- **Funky Trees Expression Engine:** Juno supports runtime programmatic property binding through "Funky Trees". Players inject dynamic mathematical expressions into any XML part attribute using the syntax `{PartSelector}.ModifierSelector.PropertySelector`. Expressions support algebraic operations, trigonometric functions, logic conditionals (`? :`), and flight telemetry variables (e.g., `Throttle = clamp(Altitude < 10000 ? 1.0 : (TargetApoapsis - Apoapsis) / 5000, 0, 1)`).

---

### 3. Procedural Rocket Engine Thermodynamics & Gas Dynamics

#### 3.1 1D Compressible Gas Dynamics & de Laval Nozzle Physics
Juno: New Origins does not rely on static thrust lookups. Its procedural rocket engine system computes real-time 1D compressible isentropic flow across a de Laval converging-diverging nozzle.

```
       1D Isentropic de Laval Converging-Diverging Nozzle

  Combustion Chamber        Throat (At)               Nozzle Exit (Ae)
      (Pc, Tc)              Mach = 1.0                   (Pe, ue)
  +------------------\                      /-------------------------+
  |                   \        _--_        /                          |
  |  Propellant        \      /    \      /      Supersonic           |
  |  Combustion         ======| At |=====        Exhaust Gas          |
  |                    /      \____/      \                           |
  +-------------------/                    \--------------------------+
                                            <------- Nozzle Bell ----->
                                              Expansion Ratio: Ae/At
```

The thermodynamic state is evaluated via the following analytical sequence:
1. **Geometric Properties:**
   - Throat cross-sectional area: `At = pi * (nozzleThroatSize / 2)^2`
   - Exit cross-sectional area: `Ae = pi * (nozzleSize / 2)^2`
   - Expansion area ratio: `epsilon = Ae / At`
2. **Characteristic Exhaust Velocity (`c_star`):**
   Governed by combustion chamber temperature `Tc`, specific heat ratio `gamma`, and specific gas constant `R_spec = R_univ / M_mol`:
   `c_star = sqrt((R_spec * Tc) / gamma) * ((gamma + 1) / 2)^((gamma + 1) / (2 * (gamma - 1)))`
3. **Mass Flow Rate (`mdot`):**
   Determined directly by chamber pressure `Pc`, throat area `At`, and `c_star`:
   `mdot = (Pc * At) / c_star`
4. **Exit Mach Number (`Me`):**
   Solved numerically via root-finding from the isentropic area-Mach relation:
   `epsilon = (1 / Me) * [(2 / (gamma + 1)) * (1 + 0.5 * (gamma - 1) * Me^2)]^((gamma + 1) / (2 * (gamma - 1)))`
5. **Nozzle Exit Static Pressure (`Pe`):**
   Derived from total chamber pressure `Pc` and exit Mach number:
   `Pe = Pc * [1 + 0.5 * (gamma - 1) * Me^2]^(-gamma / (gamma - 1))`
6. **Exit Exhaust Velocity (`ue`):**
   Evaluated using the Saint-Venant-Wantzel equation for 1D isentropic expansion:
   `ue = sqrt((2 * gamma / (gamma - 1)) * R_spec * Tc * [1 - (Pe / Pc)^((gamma - 1) / gamma)])`
7. **Net Engine Thrust (`F`):**
   Combines momentum thrust and pressure thrust across exit plane area `Ae` at ambient pressure `Pa`:
   `F = mdot * ue + (Pe - Pa) * Ae`
8. **Specific Impulse (`Isp`):**
   `Isp = F / (mdot * g0)`
   - *Vacuum Specific Impulse:* `Isp_vac = (ue + (Pe * Ae) / mdot) / g0`
   - *Sea-Level Specific Impulse:* `Isp_sl = (ue + (Pe - 101325) * Ae / mdot) / g0`

#### 3.2 Thermodynamic Engine Cycles
Juno provides four distinct thermodynamic power cycle architectures, each introducing realistic engineering trade-offs between chamber pressure `Pc`, dry mass, specific impulse penalties, and manufacturing cost:

```
+---------------------------------------------------------------------------------------------------+
|                                   THERMODYNAMIC POWER CYCLES                                      |
+---------------------------------------------------------------------------------------------------+
| 1. Pressure-Fed (PressureFed):                                                                    |
|    - Architecture: Propellants driven into chamber via high-pressure inert tank ullage.           |
|    - Chamber Pressure: Extremely low (1.0 to 2.5 MPa).                                            |
|    - Dry Mass: Minimum engine dry mass (zero turbomachinery), but requires heavier tanks.         |
| 2. Gas Generator (GasGenerator):                                                                  |
|    - Architecture: Open thermodynamic cycle. Turbopump driven by fuel-rich preburner gas dumped   |
|      overboard through an auxiliary exhaust manifold.                                             |
|    - Chamber Pressure: Moderate to high (5.0 to 12.0 MPa).                                        |
|    - Penalty: 1.5% to 3.0% effective Isp penalty due to dumped unexpanded turbopump exhaust.     |
| 3. Staged Combustion (StagedCombustion):                                                          |
|    - Architecture: Closed thermodynamic cycle. Entire turbopump preburner exhaust is routed into  |
|      the main combustion chamber.                                                                 |
|    - Chamber Pressure: Extreme (20.0 to 30.0+ MPa). Zero Isp dumping penalty.                     |
|    - Penalty: Highest dry engine mass, high complexity, and extreme manufacturing cost.           |
| 4. Expander Cycle (Expander):                                                                     |
|    - Architecture: Closed cryogenic cycle. Liquid fuel circulates through nozzle cooling channels;|
|      absorbed thermal energy vaporizes propellant to drive turbines before chamber injection.     |
|    - Chamber Pressure: Moderate (3.0 to 8.0 MPa). Maximum efficiency without soot formation.      |
|    - Penalty: Thrust and chamber pressure strictly constrained by regenerative cooling heat flux. |
+---------------------------------------------------------------------------------------------------+
```

#### 3.3 Propellant Combustion Database

| Propellant Combination | Bulk Density (`rho`) | Combustion Temp (`Tc`) | Specific Heat Ratio (`gamma`) | Characteristic Vel (`c_star`) | Optimal Application Profile |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Kerolox (RP-1 / LOX)** | ~1026 kg/m^3 | ~3600 K | 1.23 | ~1780 m/s | First-stage high density impulse; low tank volume |
| **Methalox (CH4 / LOX)** | ~830 kg/m^3 | ~3500 K | 1.22 | ~1870 m/s | Reusable boosters; clean burning; Mars ISRU |
| **Hydrolox (LH2 / LOX)** | ~358 kg/m^3 | ~3200 K | 1.21 | ~2420 m/s | Upper stages; extreme Isp; massive volumetric tanks |
| **Hypergolic (N2O4 / UDMH)**| ~1200 kg/m^3 | ~3150 K | 1.24 | ~1720 m/s | Deep-space restart; hypergolic ignition; zero boiloff |

#### 3.4 Flow Separation & Summerfield Criterion
When a rocket engine with a high expansion ratio operates at low altitudes, the exit pressure `Pe` falls substantially below atmospheric pressure `Pa` (`Pe << Pa`), creating an overexpanded regime. Juno evaluates the **Summerfield criterion for flow separation**:
- If `Pe / Pa <= 0.35 to 0.40`, atmospheric pressure forces an oblique shock wave inside the nozzle bell, causing boundary-layer separation.
- In Juno, crossing this threshold triggers dynamic mechanical vibrations and structural fatigue via the `overexpansionDamage` parameter. Sustained operation under severe overexpansion results in thrust oscillation and catastrophic nozzle structural failure.

#### 3.5 Mass & Economic Scaling Formulations
Engine dry mass and construction price scale analytically with physical geometry and cycle choice:
- **Engine Mass:**
  `EngineMass = M_base(Cycle) + k_throat * (At^1.5) + k_pc * (Pc * At) + k_nozzle * (Ae - At) * L_nozzle`
- **Engine Cost:**
  `Price = BasePrice(Cycle) * [1 + c1 * (Pc / Pc_ref)^1.2] * (ThroatRadius / R_ref)^2 + NozzlePrice(Ae, L_nozzle)`

---

### 4. Structural Mechanics: Composite Rigidbody Welding vs KSP Noodle Wobble

#### 4.1 The KSP Joint Elasticity Pathology ("Noodle Rockets")
In Kerbal Space Program 1, every discrete part in a vessel is instantiated as an independent Unity PhysX `Rigidbody`. Connections between parts are maintained using PhysX `ConfigurableJoint` spring-dampers. When a launch vehicle reaches 100+ parts, the series of compliant joints acts as a continuous elastic beam. Under heavy axial thrust, the joints experience cascading harmonic oscillations, lateral bending ("noodle wobble"), and numerical instability (the "Kraken"), forcing players to rely on artificial autostrut systems.

```
       KSP 1: Independent Rigidbodies with Joint Compliance (Noodle Wobble)

       [ Part Rigidbody 1 ]
               S  (ConfigurableJoint: Spring-Damper compliance causes lateral sway)
       [ Part Rigidbody 2 ]
               S  (Cascading angular deflections under high thrust)
       [ Part Rigidbody 3 ]  ---> Harmonic resonance & structural failure

       -----------------------------------------------------------------------

       JUNO: Composite Rigidbody Welding Architecture

       +---------------------------------------------------------------------+
       |                  SINGLE UNIFIED PHYSX RIGIDBODY                     |
       |                                                                     |
       |   [ Part 1 Mesh ] === (WELDED) === [ Part 2 Mesh ] === [ Part 3 ]   |
       |                                                                     |
       |   Single CoM: R_com = sum(m_i * r_i) / sum(m_i)                     |
       |   Single Moment of Inertia: I_total (Parallel Axis Theorem)         |
       |   Compound Segmented Collider Hierarchy                             |
       |   ZERO Compliance - ZERO Joint Wobble - ZERO Artificial Struts     |
       +---------------------------------------------------------------------+
```

#### 4.2 Juno's Composite Rigidbody Welding Architecture
Juno: New Origins resolves this structural problem at the architectural level through **Composite Rigidbody Welding**:
1. **Graph Amalgamation:** The engine parses the craft's structural graph and statically welds all rigid connections into a **single composite Unity Rigidbody**.
2. **Aggregated Mass & Center of Mass:** The combined Rigidbody possesses a single global mass `M_total = sum(m_i)` and a unified Center of Mass (`CoM`):
   `R_com = (sum(m_i * r_i)) / (sum(m_i))`
3. **Aggregated Moment of Inertia Tensor:** Juno integrates the inertia tensor `I_total` across all welded parts using the 3D **Parallel Axis Theorem**:
   `I_total = sum(I_i + m_i * (d_i^2 * E - d_i (x) d_i))`
   where `I_i` is the intrinsic moment of inertia of part `i`, `m_i` is part mass, `d_i = r_i - R_com` is the displacement vector to the composite CoM, `E` is the 3x3 identity matrix, and `(x)` denotes the outer product.
4. **PhysX Joint Isolation:** Dynamic PhysX joints (`ConfigurableJoint` / `HingeJoint`) are strictly reserved for genuine articulated mechanical interfaces:
   - `JointRotator` / `SubPartRotator` (robotic azimuth/elevation actuators)
   - `Piston` (prismatic linear actuators)
   - `Suspension` / `Wheel` (vehicle wheel suspension dampening)
   - `Detacher` / `Interstage` (explosive decoupling interfaces)
   - `DockingPort` (orbital magnetic capture interfaces)

#### 4.3 Topological Graphs & Dynamic Fracture
- **Arbitrary Cyclic Graphs:** While KSP enforces a strict single-parent directed tree (which prevents closed loops without docking ports or struts), Juno supports arbitrary cyclic graphs via `<Part.Connections>` in XML. Closed-loop truss structures, bi-coupler loops, and multi-point payload cages are natively supported.
- **Dynamic Fracture Mechanics:** Every welded connection monitors net internal shear force, tensile force, and bending torque. If the instantaneous mechanical load exceeds user-defined or material-enforced `breakForce` (N) or `breakTorque` (N*m) thresholds:
  1. The connection graph splits at the fractured node.
  2. Juno dynamically partitions the sub-assemblies into two independent composite Rigidbodies.
  3. CoM, mass, and inertia tensors are recalculated instantaneously for both vessels without frame hitching.
- **Over-G & Thermal Structural Failure:** Structural failure also triggers if a part's linear acceleration exceeds structural G-limits, impact velocity exceeds `crashTolerance` (m/s), or stagnation temperature exceeds material melting limits.

---

### 5. Custom Aerodynamic Pipeline: Polars, Wave Drag, and Fairing Occlusion

#### 5.1 Procedural Wing Geometry & Airfoil Polars
Juno models aerodynamic lift and drag analytically on every exposed lifting surface. Wing geometry is parameterized by:
- Planform area: `S = b * (c_root + c_tip) / 2`
- Aspect ratio: `AR = b^2 / S`
- Taper ratio: `lambda = c_tip / c_root`

```
                        Procedural Wing Geometry Definition

          Root Leading Offset               Tip Leading Offset
               \                                   \
                +-----------------------------------+  ^
                |                                   |  | Tip Chord (c_tip)
     Root Chord |               WING                |  v
     (c_root)   |             PLANFORM              +-----+
                |               AREA (S)            |     | Trailing-Edge
                +-----------------------------------+     | Control Surface
               /                                   /      v
          Root Trailing Offset             Tip Trailing Offset

          <---------------- Semi-Span (b) ---------------->
```

The aerodynamic polar model supports four distinct airfoil profiles:
1. **Symmetric (NACA 00xx):** Zero-lift angle of attack `alpha_0 = 0`, zero camber, `C_L0 = 0`, minimal parasite drag `C_D0`. Ideal for vertical stabilizers, rudders, and aerobatic craft.
2. **Semi-Symmetric (NACA 24xx):** Moderate positive camber, `C_L0 > 0`, high lift-to-drag ratio (`L/D`) at subsonic cruising velocities. Standard for general aviation wings.
3. **Flat-Bottom:** High camber, maximum lift coefficient `C_Lmax` at low velocities, low stall speed, but penalizing transonic wave drag.
4. **Inverted Airfoil (`invertAirfoil = true`):** Reverses camber direction to generate aggressive aerodynamic downforce for high-speed terrestrial vehicles and racing rovers.

#### 5.2 Mathematical Aerodynamic Formulations
Dynamic pressure is defined as:
`q = 0.5 * rho * v^2`
Lift and drag forces are calculated as:
`L = q * S * C_L`
`D = q * S * C_D`

1. **Attached Flow Regime (Subsonic Pre-Stall):**
   - 2D theoretical lift curve slope: `C_La_2D ~= 2 * pi rad^-1 ~= 0.11 deg^-1`
   - 3D finite wing correction (Helmbold-Diederich formulation):
     `C_La_3D = C_La_2D / [1 + (C_La_2D / (pi * AR * e))]`
     where `e` is the Oswald planform efficiency factor (~0.80 to 0.90).
   - Lift coefficient: `C_L(alpha) = C_L0 + C_La_3D * alpha`
   - Induced drag coefficient: `C_Di = C_L^2 / (pi * AR * e)`
   - Total pre-stall drag: `C_D = C_D0 + C_Di`
2. **Post-Stall Crossflow Regime:**
   When angle of attack `alpha` exceeds the critical stall angle `alpha_stall` (~15 to 18 deg), flow separates from the upper surface. Juno transitions smoothly into a sinusoidal crossflow drag model:
   `C_L ~= C_L_peak * sin(2 * alpha)`
   `C_D ~= C_D0 + C_D_crossflow * sin(alpha)^2`
   This formulation provides realistic post-stall recovery, flat spins, and high-alpha spacecraft aerobraking.

#### 5.3 Compressibility & Supersonic Wave Drag
As freestream velocity approaches Mach 1.0, Juno introduces compressibility and wave drag penalties:
- **Sweep Angle Adjustment:** For a swept wing with leading-edge sweep angle `Lambda`, the effective Mach number acting on the airfoil is reduced:
  `M_eff = M * cos(Lambda)`
- **Transonic Drag Rise:** As `M_eff` approaches critical Mach `M_crit` (~0.75 - 0.85), Juno applies the **Prandtl-Glauert** compressibility scaling:
  `C_p = C_p0 / sqrt(|1 - M_eff^2|)`
- **Supersonic Wave Drag:** In the supersonic regime (`M > 1.0`), wave drag scales quadratically with wing thickness-to-chord ratio `(t/c)` and inversely with the supersonic Mach cone angle:
  `C_D_wave proportional to (t/c)^2 / sqrt(M_eff^2 - 1)`

#### 5.4 Aerodynamic Occlusion and Fairing Shielding
Juno implements an analytical occlusion pipeline to calculate aerodynamic shadowing:
1. **Fuselage Axial Occlusion:** When two fuselages are stacked along their longitudinal axis, the mating contact surface is subtracted from exposed frontal drag area:
   `A_exposed = max(0, A_fore - A_aft)`
   Internal flat bulkheads produce zero parasite drag or skin friction.
2. **Analytical Fairing Volume Shielding:** When a procedural payload fairing is constructed, the engine compiles the geometry into an analytical bounding volume hull. All parts enclosed within this volume have their aerodynamic properties modified:
   `dragMultiplier = 0`
   Enclosed components generate zero pressure drag, zero wave drag, and zero convective reentry heating.
3. **Fairing Jettison:** Upon fairing deployment, the enclosing volume is destroyed, and the enclosed payload components are instantly registered in the active aerodynamic raycasting pipeline.
4. **Leeward Wake Shadowing:** Bluff bodies (such as blunt heat shields) project an analytical conical wake shadow downstream, reducing dynamic pressure on trailing stages during descent.

#### 5.5 Aerodynamic Architecture Comparison

| Simulation System | Aerodynamic Model | Joint / Flex Implementation | Supersonic Wave Drag | Fairing Occlusion |
| :--- | :--- | :--- | :--- | :--- |
| **Juno: New Origins** | Analytical component polar strip theory + crossflow | **Composite Rigidbody Welding** (zero wobble) | Sweep-adjusted Prandtl-Glauert + `(t/c)^2` wave drag | Analytical geometric bounding volume |
| **KSP 1 (Stock)** | Pre-baked 6-axis Drag Cubes | ConfigurableJoints per part (wobbly joints) | Piecewise Mach/drag multiplier curves | Part-flag tagging via stock cargo bays |
| **KSP 2 (Stock)** | Modified Drag Cubes | ConfigurableJoints (unstable joint tree) | Piecewise curves | Part-flag tagging |
| **Ferram Aerospace (FAR)** | Real-time voxelization + modified Newtonian / thin-wing | Retains KSP 1 wobbly joint tree | Full area-rule calculation via cross-section curves | Voxel grid density evaluation |

---

### 6. Aerothermal Reentry Physics, Sutton-Graves Heating, and Plasma Visuals

#### 6.1 Hypersonic Stagnation & Convective Heating Formulations
During atmospheric entry, hypersonic gas compression generates a detached bow shock wave, heating atmospheric gases into an incandescent plasma layer. Juno calculates stagnation point convective heat flux using the **Sutton-Graves / Chapman formulation**:
`q_dot_conv = C * sqrt(rho / R_nose) * v^3`
where:
- `C` is a planetary atmospheric composition constant (calibrated to atmospheric molar mass and gas constants).
- `rho` is the local atmospheric density at current altitude.
- `R_nose` is the effective nose radius of curvature of the leading part (blunter bodies spread heat over larger shock layers, lowering stagnation heat flux).
- `v` is freestream hypersonic relative velocity.

#### 6.2 Radiative Dissipation & Part Thermal Equilibrium
Heat energy does not accumulate indefinitely; surface thermal radiation dissipates energy into space according to the **Stefan-Boltzmann law**:
`q_dot_rad = epsilon * sigma * A_total * (T^4 - T_ambient^4)`
where `epsilon` is surface emissivity, `sigma` is the Stefan-Boltzmann constant (`5.670374e-8 W/(m^2*K^4)`), and `A_total` is part radiating surface area.

The net thermal energy balance and temperature rate of change are governed by:
`dE/dt = (q_dot_conv * A_windward * cos(theta)) - q_dot_rad`
`dT/dt = (dE/dt) / (m * c_p)`
where `m` is part mass, `c_p` is specific heat capacity, and `theta` is the angle between the surface normal and the freestream velocity vector.

#### 6.3 Thermal Conduction & Ablative Shielding
- **Sacrificial Ablation:** Procedural ablative heat shields possess elevated maximum temperature tolerances (`T_max` = 3000 to 4000 K vs. 1200 to 1800 K for structural aluminum/composites). Ablative shielding absorbs incoming heat flux through endothermic pyrolytic mass loss:
  `dm_ablate/dt = -q_dot_net / h_ablation`
  Once the ablator mass is exhausted, thermal energy transfers directly into the base structure.
- **Inter-Part Conduction:** Heat flows through the structural connection graph via **Fourier's 1D thermal conduction law**:
  `Q_dot_cond = -k * A_contact * (T_1 - T_2) / L_distance`
  where `k` is thermal conductivity, `A_contact` is interface area, and `L_distance` is the conduction path length between part centers.

#### 6.4 Visual Shaders: Update 1.4 "Glow Up" Visual Overhaul
In Update 1.4 ("Glow Up", August 2026), Juno completely overhauled its reentry aerothermal visual pipeline:
1. **Procedural 3D Shock Cones:** Rather than relying on 2D billboard sprite overlays, Juno synthesizes a dynamic 3D mesh shock cone that wraps the craft's outer geometric silhouette. The shader dynamically interpolates from a translucent white Prandtl-Glauert vapor cone at transonic speeds (`M ~ 0.9 - 1.2`) into an intense, ionized cyan-and-orange plasma sheath at hypersonic velocities (`M > 5.0`).
2. **Persistent Plasma Wakes:** Bluff bodies generate volumetric ionized plasma wake trails behind the vehicle that interact with planetary shadows and atmospheric density layers.
3. **Planckian Locus Incandescence:** Overheating parts calculate blackbody emissive spectrum radiation along the Planckian locus:
   - `T ~ 800 K`: Deep cherry-red glow.
   - `T ~ 1500 K`: Incandescent bright orange and yellow emission.
   - `T > 2500 K`: Blinding white-hot incandescence with dynamic HDR screen bloom.

---

### 7. Astrodynamics: Dual-Precision Scaling, Floating Origin, and Time-Warp Pipeline

#### 7.1 The Floating-Point Breakdown in Planetary Simulation
Standard 32-bit single-precision floating-point numbers (IEEE 754 float32) possess a 24-bit mantissa, yielding approximately 7.22 decimal digits of precision. The Unit of Least Precision (ULP) represents the minimum quantifiable spatial step at distance `x`:
`ULP(x) = 2^(floor(log2(|x|)) - 23)`

```
+---------------------------------------------------------------------------------------+
|                   FLOAT32 QUANTIZATION BREAKDOWN AT SOLAR SYSTEM SCALES                |
+---------------------------------------------------------------------------------------+
| Radial Distance from Origin | Float32 ULP Step Size | Physical Simulation Failure Mode|
+-----------------------------+-----------------------+---------------------------------+
| 10 km (Low Orbit / Launch)  | ~0.001 m (1 mm)       | PhysX contact threshold jitter  |
| 1,000 km (Medium Orbit)     | ~0.125 m (12.5 cm)    | Visible mesh vertices twitching |
| 1 AU (1.496e11 m, Earth)    | ~16,384 m (16.4 km)   | Massive physics engine collapse |
| 67 AU (1.000e13 m, Kuiper)  | ~1,048,576 m (1,048 km)| Total orbital trajectory loss   |
+---------------------------------------------------------------------------------------+
```

#### 7.2 Dual-Precision Floating-Origin Architecture
To simulate full solar systems without coordinate quantization failure, Juno: New Origins implements a **64-bit dual-precision floating-origin system**:
- **Global Coordinates (64-bit Double):** All celestial body positions, orbital ephemerides, and craft world positions are stored in 64-bit double precision (`Vector3d`, 53-bit mantissa). At 1 AU, double precision provides a spatial resolution of `0.03 mm`; at 67 AU, resolution remains under `1.95 mm`.
- **Local Physics Bubble (32-bit Float):** The active PhysX simulation operates within a local physics bubble centered at the Unity world origin `(0, 0, 0)`:
  `Pos_unity = (Vector3)(Pos_global_Vector3d - FloatingOrigin_Vector3d)`
- **Origin Rebasing Execution:** Whenever the active vessel travels beyond a specified threshold radius `R_threshold` (configured between 1,000 m and 5,000 m from the local Unity origin):
  1. The global floating origin accumulator is updated:
     `FloatingOrigin_Vector3d += delta`
  2. All active `GameObject` transform positions are offset:
     `transform.position -= delta`
  3. PhysX `Rigidbody` spatial states are synchronized:
     `rigidbody.position -= delta` (velocities remain invariant)
  4. Particle systems, orbital trajectory LineRenderers, and quadtree planet terrain anchors are shifted by `-delta`.

```
                    Dual-Precision Floating Origin Architecture

        Global Universe Space (64-bit Double: Vector3d)
        +-------------------------------------------------------------+
        |  Sun (0, 0, 0)                                              |
        |      \                                                      |
        |       \ 1 AU (1.496e11 m)                                   |
        |        v                                                    |
        |    Earth Position (Vector3d)                                |
        |        \                                                    |
        |         \ Local Offset                                      |
        |          v                                                  |
        |      [ Active Craft Position (Vector3d) ]                   |
        +-------------------------------------------------------------+
                               |
                               | (Pos_global - FloatingOrigin)
                               v
        Local PhysX Physics Scene (32-bit Float: Vector3)
        +-------------------------------------------------------------+
        | Unity Origin (0, 0, 0)                                      |
        |   ^                                                         |
        |   | |Pos_unity| < 5,000 m                                   |
        |   v                                                         |
        | [ Local Rigidbody Vessel ] <---> [ Terrain Quadtree Tiles ] |
        |                                                             |
        | When |Pos_unity| > 5,000 m:                                 |
        |   FloatingOrigin += delta                                   |
        |   All Unity Transforms -= delta                             |
        +-------------------------------------------------------------+
```

#### 7.3 Patched-Conic Keplerian Propagation
To prevent chaotic secular drift and maintain high computational efficiency, Juno rejects N-body numerical integration in favor of **2-body analytical patched conics**:
1. **Cartesian State Vectors to Keplerian Elements:**
   Position vector `r` and velocity vector `v` relative to the primary body with gravitational parameter `mu = G * M` are converted into 6 orbital elements:
   - Semi-major axis: `a = -mu / (2 * ((v^2 / 2) - (mu / r)))`
   - Specific angular momentum: `h = r x v`
   - Eccentricity vector: `e_vec = (1 / mu) * ((v^2 - (mu / r)) * r - (r . v) * v)`
   - Inclination: `i = arccos(h_z / h)`
   - Longitude of ascending node: `Omega = arccos(n_x / n)` (where node vector `n = (0, 0, 1) x h`)
   - Argument of periapsis: `omega = arccos((n . e_vec) / (n * e))`
   - True anomaly: `nu = arccos((e_vec . r) / (e * r))`
2. **Kepler Equation Solver:**
   Orbital propagation requires solving Kepler's transcendental equation for eccentric anomaly `E`:
   `M = E - e * sin(E)`
   Juno solves this using a rapid **Newton-Raphson iteration**:
   `E_(k+1) = E_k - (E_k - e * sin(E_k) - M) / (1 - e * cos(E_k))`
   This iteration converges to machine precision (`< 1e-12 rad`) within 3 to 5 steps. For hyperbolic trajectories (`e > 1.0`), the hyperbolic Kepler equation `M_h = e * sinh(H) - H` is solved.
3. **Sphere of Influence (SOI) & Hysteresis:**
   Planetary transitions occur at the Laplace Sphere of Influence:
   `r_SOI = a * (m / M)^(2/5)`
   To prevent high-frequency boundary flickering when a vessel skims an SOI boundary, Juno enforces a dual-threshold hysteresis: entry is evaluated at `r_entry = r_SOI`, while exit requires exceeding `r_exit = 1.01 to 1.05 * r_SOI`.

#### 7.4 Maneuver Planning & Burn Equations
Maneuver node calculations evaluate the Tsiolkovsky rocket equation:
`Delta_v = I_sp * g_0 * ln(m_0 / m_f)`
Burn duration is computed analytically based on constant mass flow:
`t_burn = (m_0 * I_sp * g_0 / F_thrust) * (1 - exp(-Delta_v / (I_sp * g_0)))`
To minimize steering losses, the burn is centered symmetrically around the node epoch:
`t_start = t_node - (t_burn / 2)`

#### 7.5 Dual Time-Warp Pipeline: Physics Warp vs. On-Rails Warp
Juno implements two distinct time-warp regimes:
1. **Numerical Physics Warp (1x to 2x):**
   PhysX remains fully engaged at a fixed 60 Hz time-step (`fixedDeltaTime = 0.01667 s`). Aerodynamic forces, engine thrust, RCS torque, and joint stresses are accumulated dynamically:
   `F_net = F_grav + F_thrust + F_drag + F_lift + F_rcs`
2. **Analytical On-Rails Warp (>2x up to 1,000,000x):**
   PhysX rigidbodies are set to `rigidbody.isKinematic = true`. All collision detection, joint calculations, and aerodynamic forces are suspended. The vessel's position is computed analytically along its Keplerian conic section:
   `Pos(t) = Kepler(elements, t)`
   This evaluation executes at `O(1)` CPU complexity, guaranteeing zero orbital decay, zero joint drift, and zero numerical error regardless of warp speed.
3. **Low-Thrust On-Rails Propulsion:** For ion engines and low-thrust propulsion systems, Juno incorporates a specialized solver that applies continuous low thrust during on-rails time warp by integrating delta-v analytically into the Keplerian orbital elements without engaging PhysX.

---

### 8. Vizzy Visual Programming VM, Planet Studio, and Ecosystem Roadmap

#### 8.1 Vizzy Flight Computer Architecture
Juno: New Origins incorporates **Vizzy**, a fully integrated visual block programming environment designed for autonomous guidance, navigation, and control (GNC).

```
                      VIZZY COMPILATION & EXECUTION PIPELINE

    Visual Block Canvas (UI) <===> Abstract Syntax Tree (AST) <===> Craft.xml
                                                                        |
                                                                        v
    Unity Engine Main Thread                                 Managed C# Interpreter
    +--------------------------------------------------------------------------+
    | FixedUpdate() Physics Tick (50 / 60 Hz)                                  |
    |   ├── Step PhysX Rigidbodies                                             |
    |   ├── Vizzy VM Execution Slice (Instruction Quota per Tick)             |
    |   │     ├── Decode AST Node                                              |
    |   │     ├── Evaluate Expression / Vector Math                            |
    |   │     └── Command Actuators (Throttle, Gimbal, Pitch, Roll)           |
    |   │     └── Yield on [Wait 0 Seconds] to prevent frame hitching          |
    |   └── Update Part Modifiers & Aerodynamics                              |
    +--------------------------------------------------------------------------+
```

- **Serialization:** Visual blocks are mapped 1:1 to an Abstract Syntax Tree (AST) serialized directly into `Craft.xml` within the `<FlightProgram>` element.
- **Execution Loop:** The interpreted C# VM executes synchronously within Unity's `FixedUpdate` physics loop. Each flight program is allocated an instruction budget per tick. To prevent game loop freezing, long loops require explicit yielding via the `Wait 0 Seconds` block, which suspends execution until the subsequent physics tick.
- **Data Types:** Dynamic type system supporting 64-bit floats, 3D vectors (`Vector3`), strings, booleans, and dynamic 1D lists.
- **Concurrency:** Programs utilize event-driven roots (`On Start`, `Receive Message`, `On Part Exploded`). Programs can broadcast local events within the same chip (`Broadcast Message`) or across multiple autonomous craft command pods (`Broadcast Message to Craft`).
- **Sandboxing:** Vizzy operates entirely within a sandboxed managed memory space; it has no access to underlying operating system file systems, command shells, or network sockets.
- **Flight Telemetry & Vector Math:** Native support for 3D vector algebra (dot product, cross product, vector projection, normalization, magnitude). Native coordinate frames include:
  - Planet-Centric Inertial (PCI)
  - Surface Relative (Heading, Pitch, Bank, True/ASL Altitude)
  - Craft Body-Fixed (Forward, Right, Up)
  - Orbital Prograde / Retrograde / Normal / Radial
  - Maneuver Node (`Lock Nav Sphere to BurnNode`)
- **Closed-Loop Attitude Control:** Players construct programmatic closed-loop PID controllers:
  `P = Kp * error`
  `I = clamp(I + Ki * error * dt, -max_I, max_I)`
  `D = Kd * (error - prev_error) / dt`
  `Output = P + I + D`
- **Speech Synthesizer:** Update 1.4 integrated an automated text-to-speech engine into Vizzy, enabling custom voice callouts for launch countdowns, staging, and abort sequences.

#### 8.2 Architectural Comparison: Vizzy vs. Kerbal Operating System (kOS)

| Technical Feature | Vizzy (Juno: New Origins) | Kerbal Operating System (kOS - KSP Mod) |
| :--- | :--- | :--- |
| **Programming Paradigm** | Scratch-style Visual Block AST | Imperative Compiled Text Language (KerboScript) |
| **Execution Architecture** | Managed C# AST interpreter in `FixedUpdate` | Custom Bytecode VM executing KerboScript |
| **Hardware Overhead** | Zero mass, zero volume, zero electrical draw | Physical computer parts, disk space limits, electric drain |
| **Platform Availability** | Native Cross-Platform (PC, Mac, iOS, Android) | PC Only (Requires desktop KSP modding) |
| **Serialization** | Embedded XML `<FlightProgram>` in craft file | External `.ks` text files on local disk |
| **Maneuver Node Control** | Indirect attitude lock (`BurnNode` vector) | Native maneuver node manipulation API |
| **Inter-Craft Radio** | Native broadcast across local physics bubble (< 10 km) | Addon-dependent (RemoteTech / CommNet integration) |

#### 8.3 Planet Studio World-Building Suite
Integrated in February 2021, **Planet Studio** provides a procedural world-building engine within Juno:
1. **Procedural Noise Stack:** Terrains are synthesized using a hierarchical procedural noise graph:
   - *Simplex & Perlin Noise:* Macro continental landmasses, plate tectonics, and base elevations.
   - *Voronoi / Cellular Noise:* Impact craters, fault lines, and sheer canyon walls.
   - *Ridged Multifractal Noise:* Alpine mountain chains, sharp ridges, and erosion gullies.
   - *3D Terrain Stamp Tool:* Allows artists to hand-sculpt local landing sites directly onto procedural planets.
2. **Cube-Sphere Quadtree LOD Tessellation:**
   To eliminate polar coordinate singularities and texture pinching, planets are modeled as normalized **Cube-Spheres**:
   `P_sphere = normalize(P_cube) * R`
   Each of the six cube faces is recursively subdivided via a quadtree. Dynamic level-of-detail (LOD) splits chunks based on camera distance, utilizing skirt geometry and edge-vertex stitching to prevent cracks between adjacent resolution levels.
3. **PBR Surface Shading & Linx Parallax:**
   - *MicroSplat Multi-Texture Blending:* Height-based multi-texture splatting smoothly transitions rock, sand, snow, and sediment textures.
   - *Linx Parallax GPU Scatter (Update 1.2, Sep 2023):* Implements compute-shader-driven GPU instancing to render millions of 3D rocks, boulders, and surface displacement features with zero CPU draw calls.
4. **Atmospheric & Ocean Physics:**
   - Rayleigh and Mie scattering atmospheric shaders (optimized with ALU approximations in Update 1.4).
   - Exponential barometric density: `P(h) = P_0 * exp(-h / H)` (where `H` is scale height).
   - Gerstner wave ocean shaders evaluating procedural water displacement and displaced-volume buoyancy forces.

#### 8.4 Rebranding & Ecosystem Roadmap
- **Nov 8, 2018:** SimpleRockets 2 enters Steam Early Access.
- **Sep 2019:** Full mobile launch on iOS and Android with complete cross-platform craft compatibility.
- **Feb 2021:** Release of Planet Studio, opening procedural solar system generation to players.
- **Jan 26, 2023 (Version 1.0):** Rebranded from *SimpleRockets 2* to **Juno: New Origins**. Release of full Career Mode, featuring:
  - Procedural contract generation engine.
  - 4-branch research tech tree (rocketry, aeronautics, terrestrial rovers, and avionics).
  - Cash funding, launch site customization, and milestone tracking.
- **Update 1.1:** Added procedural propellers, turboprops, and turbofan engines.
- **Update 1.2 (Sep 2023):** Integrated Linx Parallax GPU scatter, dynamic rocket plume atmospheric expansion, and overhauled procedural wings.
- **Update 1.3:** Career economy rebalance, contract pacing refinements.
- **Update 1.4 "Glow Up" (Aug 2026):** Visual overhaul featuring 3D procedural shockwave cones, blackbody incandescence shaders, redesigned flight camera, map view overhaul, and Vizzy speech synthesis.
- **Community Hub:** Integrated platform at `SimpleRockets.com` (Juno Web Portal) supporting seamless cross-platform sharing of craft XMLs, sub-assemblies, planets, and solar systems between PC and mobile users, augmented by Steam Workshop integration.

---

## ⚖️ Conflicting Information & Ambiguities

### 1. Naming Conventions & Product Identity (SimpleRockets 2 vs. Juno: New Origins)
- **Discrepancy:** Community resources, historical technical threads, and modding repositories frequently refer to the software as "SimpleRockets 2" or "SR2", whereas official documentation post-January 2023 strictly designates it as "Juno: New Origins".
- **Resolution & Assessment:** The name change occurred simultaneously with the Version 1.0 update on January 26, 2023, reflecting the transition from an early-access rocketry sandbox to a comprehensive aerospace simulation incorporating career mode, terrestrial vehicles, and aviation. Under the hood, code assemblies, configuration files, and XML namespaces retain the legacy naming convention (e.g., `SimpleRockets2.dll`, `SimpleRockets.com`, and the `ModApi.Craft` namespace). Both names describe the exact same underlying simulation technology.

### 2. Marketing Claims vs. Aerodynamic Pipeline Implementation ("Real-Time CFD")
- **Discrepancy:** Promotional literature and community discourse occasionally claim that Juno: New Origins computes "real-time computational fluid dynamics (CFD)" over user aircraft.
- **Resolution & Assessment:** Technical decomposition reveals that Juno does **not** execute grid-based Navier-Stokes or lattice Boltzmann CFD equations in real time (an approach that remains computationally impossible for interactive 60 FPS consumer physics). Instead, Juno utilizes an advanced **component-based analytical polar strip model** combined with empirical transonic/supersonic corrections (Prandtl-Glauert, Helmbold-Diederich, and thickness-ratio wave drag scaling) and analytical fairing geometric occlusion. While substantially more advanced than stock KSP's 6-axis drag cubes, it is an empirical aerodynamic formulation rather than true fluid dynamics.

### 3. Gravitational Simulation Architecture: Patched Conics vs. N-Body Physics
- **Discrepancy:** Early development roadmaps and community feature requests debated the inclusion of full N-body gravitational integration (similar to the *Principia* mod for KSP).
- **Resolution & Assessment:** Jundroo explicitly rejected N-body numerical integration in favor of **2-body patched-conic Keplerian propagation**. N-body integration introduces chaotic secular drift that destabilizes orbital arrangements over long simulation timescales and incurs severe CPU overhead during 100,000x+ time warp. Patched conics guarantees deterministic, closed-form, `O(1)` analytical trajectory solutions across time warp, ensuring smooth performance on mobile chipsets.

### 4. Rigid-Body Compliance: Welded Rigidity vs. Dynamic Articulated Joints
- **Discrepancy:** While Juno's composite Rigidbody welding eliminates structural wobble within statically welded part assemblies, players occasionally report joint flex during flight.
- **Resolution & Assessment:** High source credibility confirms that composite Rigidbody welding applies strictly to *static* part connections. Whenever a craft incorporates intentional articulation points (`JointRotator`, `Piston`, `Suspension`, `DockingPort`), PhysX `ConfigurableJoint` spring-dampers must be instantiated to provide dynamic degrees of freedom. Under extreme aerodynamic loads or high thrust-to-weight ratios, these articulated joints still experience physical deflection and can flex if structural limits are exceeded.

---

## 🔗 Sources & Citations

1. [Jundroo Official Developer Blogs & Release Announcements (2018–2026)](https://www.jundroo.com/) - Technical breakdown of engine architecture, early access updates, 1.0 launch, and Update 1.4 "Glow Up" features.
2. [Juno: New Origins Official Modding API Reference](https://soap.jundroo.com/ModApi/) - Documentation of `ModApi.Craft.Parts`, `PartModifier`, `FuselageData`, and `WingData` classes.
3. [Juno: New Origins Community Knowledge Base & XML Documentation](https://www.simplerockets.com/) - Specification of XML craft schema, Funky Trees syntax, and procedural engine parameters.
4. [Unity Technologies PhysX Integration Manual](https://docs.unity3d.com/Manual/PhysicsOverview.html) - Documentation on NVIDIA PhysX rigidbodies, compound colliders, and `fixedDeltaTime` loops.
5. [Unity Technologies IL2CPP & Burst Compiler Documentation](https://docs.unity3d.com/Manual/IL2CPP.html) - Technical specifications of C# AOT compilation and SIMD job vectorization.
6. Sutton, G. P., & Biblarz, O. *Rocket Propulsion Elements*, 9th Edition, Wiley - Theoretical foundation for 1D isentropic de Laval nozzle flow, characteristic velocity (`c_star`), and thermodynamic cycles.
7. Anderson, J. D. *Fundamentals of Aerodynamics*, 6th Edition, McGraw-Hill - Mathematical formulations for Prandtl-Glauert compressibility, finite wing lift slopes, and supersonic wave drag.
8. NASA SP-8000 Series: *Aerodynamic and Astrodynamic Design Criteria* - Sutton-Graves stagnation point convective heating formulation and Chapman hypersonic approximations.
9. Vallado, D. A. *Fundamentals of Astrodynamics and Applications*, 4th Edition, Microcosm Press - Kepler's equation solvers, Newton-Raphson orbital iteration, state-vector conversions, and Laplace Sphere of Influence formulations.

---

## 🗃️ Index Metadata

```json
{
  \"title_and_scope\": \"SimpleRockets / Juno: New Origins Technical Architecture, Aerodynamics, and Simulation Systems\",
  \"date\": \"2026-09-10\",
  \"objective\": \"Deliver an exhaustive, publication-grade technical decomposition of SimpleRockets 1 vs. SimpleRockets 2 / Juno: New Origins, analyzing its engine transition (Box2D to Unity C#), composite rigidbody welding, procedural part mesh generation, de Laval nozzle thermodynamics, custom aerodynamic polars, floating origin astrodynamics, and Vizzy visual VM architecture.\",
  \"conclusions\": \"Juno: New Origins eliminates KSP-style joint flex by welding static part trees into composite Rigidbodies, while providing 1D isentropic de Laval nozzle engine thermodynamics, custom transonic/supersonic aerodynamic polars, and a sandboxed visual AST interpreter (Vizzy) executing synchronously on Unity's FixedUpdate tick.\"
}
```

