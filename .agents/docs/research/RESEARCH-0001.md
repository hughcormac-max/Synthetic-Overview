# Research Report: Kerbal Space Program Engine Architecture, Astrodynamics, and Technical Post-Mortem (KSP 1 vs KSP 2)

> **Date:** 2026-09-10
> **Objective:** Comprehensive technical investigation into the software architecture, physics pipeline, coordinate precision frameworks, and astrodynamical mechanics of Kerbal Space Program 1, contrasted against the design choices, engineering compromises, and eventual cancellation of Kerbal Space Program 2.

---

## 📑 Executive Summary

Kerbal Space Program (KSP 1) stands as a landmark technical achievement in commercial game physics, successfully bridging rigid-body computational mechanics with celestial orbital mechanics on top of an off-the-shelf game engine (Unity). Beginning in 2011 on Unity 4.6 LTS with 32-bit memory constraints and PhysX 2.8.3, Squad engineered a series of custom architectural workarounds—most notably the `FloatingOrigin` and `Krakensbane` coordinate translation systems—to circumvent the fundamental limits of single-precision IEEE 754 floating-point arithmetic across interplanetary distances. The core vessel architecture relies on a strict directed acyclic tree (arborescence) of GameObjects, each bound to a distinct PhysX `Rigidbody` connected via `ConfigurableJoint` constraints. While this enabled modular in-flight construction and dynamic staging, PhysX's monolithic `PxIslandManager` architectural design constrained craft physics to a single thread, generating systemic CPU bottlenecks and iterative solver flex ("noodle rockets") solved pragmatically through user-tweakable constraints (`Autostrut` and `Rigid Attachment`).

Orbital flight in KSP 1 bypasses expensive, chaotic numerical n-body integration in favor of an analytical patched conics formulation governed by Keplerian two-body mechanics and Laplace Spheres of Influence (SOI). This dual-mode simulation architecture segregates computation into two mutually exclusive regimes: an interactive, sub-stepped numerical PhysX simulation ("Physical Time-Warp", 1x to 4x) for atmospheric flight and powered maneuvers, and an analytical closed-form Keplerian propagator ("On-Rails Time-Warp", up to 100,000x) that marks vessels as kinematic and solves Kepler's equation in O(1) time complexity. Astrodynamics calculations are maintained in 64-bit double precision (`Vector3d`), projected dynamically onto a 32-bit single-precision Unity world space, and visually mapped through a dual-camera hierarchy using a 1:6000 scaled-space transform equipped with a logarithmic depth buffer.

Kerbal Space Program 2 (Intercept Games / Take-Two Interactive) attempted to modernize this foundation targeting massive multi-part colonies, interstellar distances, and multi-threaded simulation on Unity 2022.3.5f1 LTS. Although pre-release technical disclosures promised an engine rebuild based on Unity DOTS/ECS, the C# Job System, and the Burst Compiler, decompile post-mortems reveal that core vessel simulation directly replicated KSP 1's single-threaded GameObject, Rigidbody, and PhysX joint-tree paradigm. Burst and Jobs were relegated to secondary systems (such as PQS+ terrain generation and trajectory tessellation). Launched in Early Access without crucial joint stabilization mechanisms and crippled by severe UI draw-call bottlenecks and persistent numerical integration leaks in vacuum orbits, KSP 2 failed to achieve technical viability, culminating in the closure of Intercept Games in June 2024 and the complete abandonment of planned interstellar, colony, and multiplayer systems.

---

## 🔍 Key Findings

### 1. Engine Evolution, Runtime Pipeline, and Memory Management

* **Finding:** KSP 1 transitioned across four major Unity generations (Unity 4.6 to 2019.4.18f1 LTS), maintaining an exclusively unstripped Mono JIT runtime that prioritized modding accessibility over native IL2CPP performance.
* **Context:** The engine migration path was dictated by memory architecture and PhysX upgrades:
  * *KSP 0.x to 1.0.5:* Built on Unity 4.6 LTS (32-bit x86, PhysX 2.8.3). Hard-capped by the 32-bit virtual address space limit (~3.5 GB usable RAM), precipitating rampant Out-Of-Memory (OOM) crashes when high-resolution texture assets were loaded.
  * *KSP 1.1 "Turbo Charged" (April 2016):* Upgraded to Unity 5.2.4f1/5.4.0p4 and PhysX 3.3. Introduced native 64-bit binaries across Windows, Linux, and macOS, SIMD-accelerated vector operations, and migrated from immediate-mode GUI (`OnGUI`) to Unity's optimized canvas system (`uGUI`).
  * *KSP 1.8 (October 2019) to 1.12 LTS:* Shifted to Unity 2019.2.2f1 (later finalized on 2019.4.18f1 LTS), adopting the .NET Standard 2.0 / .NET 4.x runtime, DX11/OpenGL Core rendering, and Unity's Incremental Garbage Collector.
  * *Mono JIT Runtime Architecture:* Squad deliberately retained the Mono JIT scripting backend (`mono-2.0-bdwgc.dll` and `Assembly-CSharp.dll`) rather than compiling via IL2CPP. This kept CIL bytecode intact, allowing tools like Harmony, ModuleManager, and CKAN to inject runtime detours, rewrite part logic, and maintain a vibrant modding ecosystem.
  * *Garbage Collection Spikes:* KSP 1 suffered from regular 100-300 ms freeze frames occurring every 5-15 seconds. This was caused by the non-generational Boehm-Demers-Weiser mark-and-sweep GC scanning megabytes of heap allocations generated per frame by LINQ queries, hidden enumerator allocations (`GetEnumerator()`), and struct boxing in `Update()` and `FixedUpdate()` loops. The community initially mitigated this via pad-allocation mods (e.g., `MemGraph`), before Unity's Incremental GC in KSP 1.8 partitioned GC pauses across multiple frame intervals.

---

### 2. Structural Joint Dynamics and the Multi-Core PhysX Bottleneck

* **Finding:** Both KSP 1 and KSP 2 implement craft as directed acyclic trees of distinct `Rigidbody` primitives linked via iterative PhysX constraints, forming a monolithic "PhysX Island" that restricts single-vessel physics simulation to a single CPU thread.
* **Context:** The structural mechanics and performance bottlenecks stem from the underlying constraint solver:
  * *Part Joint Hierarchy:* Vessels are arborescences rooted at a single part (`rootPart`). Every attached part instantiates an independent `GameObject`, `Rigidbody`, `Collider`, and a custom `PartJoint` wrapper managing a Unity `ConfigurableJoint`.
  * *PhysX Projected Gauss-Seidel (PGS) Iterative Solver:* Unity's default constraint solver iterates a set number of times (typically `defaultSolverIterations = 6 to 8`). Because the solver enforces constraints iteratively rather than analytically, residual drift and angular slack remain after each physics tick. Across long chains of parts (e.g., a multi-stage rocket stack), this elasticity compounds, creating the infamous "noodle rocket" flex.
  * *Mass Ratio Divergence:* Iterative solvers exhibit severe numerical divergence when high mass ratios are joined in series (e.g., a 0.05-tonne interstage decoupler sandwiched between two 36-tonne propellant tanks). The lighter body oscillates violently between the corrective impulses of the heavier bodies, introducing mechanical resonance. When coupled with active SAS reaction wheels or gimbaling engines, a positive feedback loop occurs, leading to structural disintegration.
  * *Constraint Hardening (`Autostrut` & `Rigid Attachment`):* In KSP 1.2, Squad introduced internal constraint reinforcements:
    * `Autostrut`: Dynamically creates invisible, zero-mass `ConfigurableJoint` links connecting leaf parts directly to the craft's `Root`, `Heaviest`, or `Grandparent` part. While `Grandparent` autostrutting creates stable linear damping, `Heaviest` autostrutting can induce physics spikes during staging when the center of mass or heaviest part ID recalculates mid-frame.
    * `Rigid Attachment`: Disables joint spring compliance, activating `JointProjectionMode.PositionAndRotation`. While this stops rotational bending, it eliminates shock absorption, transferring all mechanical impulse undamped to joint breaking thresholds (`breakForce` and `breakTorque`) and resulting in brittle shearing failures.
  * *The PhysX Island Bottleneck:* PhysX optimizes simulation by grouping contacting and joint-connected rigidbodies into an "Island" (`PxIslandManager`). In KSP, because all constituent parts of a craft are connected through `PartJoint` links, an entire 500-part rocket is treated as a single island. **PhysX cannot solve a single island across multiple CPU cores; the entire constraint graph must be evaluated sequentially on one core.** Multi-threaded physics scaling only functions when separate vessels are outside contact range. Docking two vessels immediately collapses their separate islands into a single thread.
  * *KSP 2 Structural Failures:* While KSP 2 was marketed as a ground-up rebuild leveraging Unity DOTS/ECS for parallelized physics, the ongoing API churn of Unity DOTS during 2019-2022 led Intercept Games to scrap DOTS for core vessel simulation. KSP 2 shipped Early Access on Unity 2022.3.5f1 LTS with the same single-threaded GameObject/Rigidbody/PhysX joint architecture as KSP 1, but initially omitted Autostrut and misconfigured joint damping, producing extreme launchpad wobble. Patch 0.2.0 ("For Science!") mitigated this by deploying David Tregoning's Enhanced Joint System (multi-joint reinforcement and artificial inertia tensor boosting on low-mass nodes).

```
[PhysX Island Simulation Bottleneck]

Single Vessel (500 Connected Parts via PartJoint)
+-----------------------------------------------------------------------+
| Core Vessel Physics Graph (Single PxIslandManager Island)             |
|                                                                       |
|  [Part 1] ===Joint=== [Part 2] ===Joint=== [Part 3] ... [Part 500]    |
+-----------------------------------------------------------------------+
                                  |
                                  v
              Locked to ONE Worker Thread (Single CPU Core)
        *Multi-core parallelization impossible on a single craft*
```

---

### 3. Floating-Point Precision, Krakensbane, and Scaled Space

* **Finding:** Standard IEEE 754 32-bit floating-point variables fail catastrophically across interplanetary distances, necessitating a hybrid double-precision simulation loop paired with the `FloatingOrigin` and `Krakensbane` coordinate translation algorithms.
* **Context:** The precision limits and mitigation algorithms are structured as follows:
  * *IEEE 754 Floating-Point Quantization:* A standard 32-bit float uses a 24-bit effective mantissa (~7.22 decimal digits of precision) with machine epsilon `eps = 2^(-23) ~= 1.192e-7`. At Kerbin's orbital distance from the Sun (1 AU, or approximately `1.496e11 m`), the minimum representable spatial increment is:
    ```
    Delta_x ~= 1.496e11 m * 1.192e-7 ~= 17,833.8 m (~17.8 km)
    ```
    Even at Low Kerbin Orbit (`r = 700,000 m`), spatial resolution degrades to `Delta_x ~= 8.34 cm`. This coarseness causes severe vertex tearing, z-fighting, collider penetration, and iterative solver breakdown.
  * *Taxonomy of the "Space Kraken":*
    * *Deep Space Kraken:* Triggered at high velocities (`|v| > 750 m/s`), where large position updates per physics frame cause constraint solvers to introduce massive corrective phantom forces, instantly ripping vessels apart.
    * *Hell Kraken:* Occurs when terrain collision detection or raycasting produces a divide-by-zero, injecting `NaN` into transform positions, blanking the camera view, and locking the altimeter at `666,666 m`.
    * *Roll/Gyro Kraken:* Numerical instability in non-diagonalized inertia tensors, amplifying vessel angular velocity uncontrollably.
    * *Water Kraken:* Discontinuous bounding box surface calculations in ocean volume integration applying near-infinite buoyant impulses.
  * *The FloatingOrigin Algorithm:* KSP 1 monitors the active vessel's distance from the Unity world origin `(0,0,0)` every frame. When `|activeVessel.position| > threshold` (typically 2,000 m):
    ```
    Vector3 offset = activeVessel.transform.position;
    foreach (Transform rootTransform in Scene.RootTransforms) {
        rootTransform.position -= offset;
    }
    foreach (Rigidbody rb in Scene.AllRigidbodies) {
        rb.position -= offset; // Directly repositions without applying PhysX velocity impulse
    }
    FloatingOrigin.SetOffset(offset); // Broadcasts to cameras, particles, and trajectory renderers
    ```
  * *The Krakensbane Frame Shift:* High-velocity physics solver breakdown is resolved by inverting the universe's velocity vector once the craft exceeds a critical velocity threshold (`MaxV = 750 m/s` relative to the parent celestial body). The vessel's PhysX velocity is subtracted:
    ```
    Vector3 frameVelocity = activeVessel.rigidbody.velocity;
    foreach (Rigidbody rb in Scene.AllRigidbodies) {
        rb.velocity -= frameVelocity;
    }
    ```
    The craft is brought essentially to rest (`v_PhysX ~= 0`) in the local Unity scene, while the celestial bodies, atmosphere, and coordinate space are translated backward by `-(frameVelocity * fixedDeltaTime)` each physics step. Aerodynamic algorithms compute lift and drag by referencing the stored `frameVelocity` rather than the PhysX body velocity.
  * *Dual-Precision Mathematical Architecture:* Orbital astrodynamics, ephemeris calculations, and celestial state vectors are evaluated in 64-bit double precision (`Vector3d`), where quantization at 1 AU is reduced to `Delta_x ~= 33.3 micrometers`. Single-precision 32-bit floats are reserved strictly for Unity scene transforms, mesh geometry, and the local PhysX simulation bubble:
    ```
    unityWorldPos = (Vector3)(bodyDoublePosition - universeOriginOffsetDouble);
    ```
  * *Scaled Space Dual-Camera Pipeline:* Rendering celestial bodies spanning billions of meters within Unity's depth buffer is handled using two distinct cameras:
    1. *Local Physics Camera (`depth = 1`, `ClearFlags = DepthOnly`):* Renders the immediate local environment (0 to 2.5 km/10 km physics bubble) at a 1:1 scale.
    2. *Scaled Space Camera (`depth = 0`, `ScaleFactor = 1.0 / 6000.0`):* Renders the solar system at a 1:6000 scale. Kerbin (radius 600,000 m) is rendered as a 100-meter sphere positioned at 1/6000th of its actual distance. A logarithmic depth buffer eliminates z-fighting:
       ```
       z_ndc = (log(C * w + 1) / log(C * Far + 1)) * w
       ```
  * *Navball Quaternions and Reference Frames:* To prevent gimbal lock, vessel orientation is computed using unit quaternions `q = [w, x, y, z]`. The relative navball rotation is calculated as:
    ```
    q_navball = Quaternion.Inverse(q_frame) * q_vessel
    ```
    Coordinate baselines are mapped across three principal flight modes:
    * *Orbit Frame:* Forward axis `v_fwd = (v_orbit).normalized`, Normal axis `v_up = (r cross v_orbit).normalized`.
    * *Surface Frame:* Accounts for planetary rotation: `v_surface = v_orbit - (omega cross r)`.
    * *Target Frame:* Dynamic line-of-sight and relative velocity vectors between craft docking ports.

---

### 4. Orbital Astrodynamics, Patched Conics, and Time-Warp Mechanics

* **Finding:** KSP rejects numerical n-body integration in favor of patched Keplerian two-body conics using the Laplace Sphere of Influence, maintaining deterministic O(1) state evaluations that enable 100,000x on-rails time-warp.
* **Context:** The astrodynamical engine operates on strict analytical mechanics:
  * *Keplerian Two-Body Equations:* Orbital motion is governed by the two-body differential equation:
    ```
    d^2 r / dt^2 = - (mu / r^3) * r
    ```
    where `mu = G * M` is the standard gravitational parameter. Key orbital invariants include:
    * Specific angular momentum: `h = r cross v`
    * Specific orbital energy: `epsilon = (v^2 / 2) - (mu / r)`
    * Eccentricity vector: `e_vec = (1 / mu) * (v cross h) - (r / |r|)`
    * Semi-major axis: `a = - mu / (2 * epsilon)`
    * Semi-latus rectum: `p = (h^2 / mu) = a * (1 - e^2)`
    * True anomaly polar equation: `r(nu) = p / (1 + e * cos(nu))`
  * *Solving Kepler's Equation:* Mean anomaly evolves linearly with time:
    ```
    M(t) = M_0 + n_mean * (t - t_0),  where n_mean = sqrt(mu / a^3)
    ```
    To find the Eccentric Anomaly `E` from Mean Anomaly `M` for elliptic orbits (`0 <= e < 1`), KSP solves transcendental Kepler's Equation `M = E - e * sin(E)` via Newton-Raphson iteration:
    ```
    E_{k+1} = E_k - (E_k - e * sin(E_k) - M) / (1 - e * cos(E_k))
    ```
    The True Anomaly `nu` is derived through the relation:
    ```
    tan(nu / 2) = sqrt((1 + e) / (1 - e)) * tan(E / 2)
    ```
    For hyperbolic trajectories (`e > 1`), Kepler's equation is formulated as `M_h = e * sinh(H) - H` and solved iteratively for hyperbolic anomaly `H`.
  * *Coordinate System Bridging:* Unity's native coordinate system is left-handed with Y-up, whereas standard astrodynamics uses right-handed Z-up frames. KSP reconciles this in `Orbit.UpdateFromStateVectors()` using a global quaternion transformation (`Planetarium.ZupRotation`).
  * *Patched Conics and Laplace SOI:* The boundary where a celestial body dominates orbital mechanics over its parent is governed by the Laplace Sphere of Influence:
    ```
    r_SOI = a * (m / M)^(2/5)
    ```
    *(Note: This differs mathematically from the gravitational Hill Sphere, `r_Hill = a * (m / (3 * M))^(1/3)`).*
    * *Boundary Exit:* Calculated analytically by determining the true anomaly `nu_exit` where the current orbit intersects the SOI radius:
      ```
      cos(nu_exit) = ((p / r_SOI) - 1) / e
      ```
    * *Boundary Entry:* When a craft leaves a parent SOI, entry into a target body's SOI is detected using coarse temporal interval stepping followed by Brent's root-finding method / Newton-Raphson on:
      ```
      f(t) = |r_vessel(t) - r_target(t)|^2 - (r_SOI_target)^2 = 0
      ```
    * *Coordinate Hand-off:* At the transition boundary, state vectors are converted between parent and target frames:
      ```
      r_vessel_in_target = r_vessel_in_parent(t_entry) - r_target_in_parent(t_entry)
      v_vessel_in_target = v_vessel_in_parent(t_entry) - v_target_in_parent(t_entry)
      ```
      Traversed trajectories are stored as a doubly-linked list of `Orbit` instances (`nextPatch`, `previousPatch`).
  * *The Rejection of N-Body Mechanics:*
    1. *Analytical Determinism:* Patched conics calculate an exact vessel position at any time `t` in O(1) mathematical complexity. N-body simulation requires sequential numerical integration, producing floating-point drift over time.
    2. *Fast Time-Warp Compatibility:* Simulating high-rate warp (100,000x to 1,000,000x) under n-body physics causes severe integration errors or CPU exhaustion.
    3. *Joolian System Resonance Collapse:* Under full n-body gravitation, the Laplace resonance of Jool's inner moons (Laythe, Vall, Tylo in a 1:2:4 resonance) is chaotic and unstable; numerical simulation ejects Vall and Bop from the Joolian system within a few game weeks. (The *Principia* mod mitigates this by rewriting celestial orbits into a modified stable configuration termed "Retrobop" and using Symplectic Runge-Kutta-Nyström [SRKN] integrators in native C++).
  * *Maneuver Planning and Burn Approximations:* Trajectory alterations are computed using the Tsiolkovsky Rocket Equation:
    ```
    Delta-v = I_sp * g_0 * ln(m_0 / m_f)
    ```
    where burn duration is calculated by integrating mass loss over time:
    ```
    t_burn = ((m_0 * I_sp * g_0) / F_thrust) * (1 - exp(-Delta-v / (I_sp * g_0)))
    ```
    KSP's maneuver node interface assumes an instantaneous, impulsive velocity change at node epoch `t_node`. During execution, players execute a finite burn split symmetrically (50% before `t_node`, 50% after), introducing cosine steering losses and degrading Oberth efficiency relative to the idealized node.
  * *Physical vs. On-Rails Time-Warp:*
    * *Physical Warp (1x to 4x):* Active during atmospheric flight, surface operations, and engine burns. PhysX remains fully active using sub-stepped semi-implicit Euler integration. Part joints flex and aerodynamic forces apply, consuming high CPU resources.
    * *On-Rails Warp (up to 100,000x):* Enabled in vacuum when engines are idle. PhysX rigidbodies are set to kinematic (`isKinematic = true`), joint physics calculations are suspended, and vessel positions advance purely analytically via Keplerian equations with near-zero CPU overhead.
  * *KSP 2 Innovations:* KSP 2 implemented "Persistent Thrust under Warp" using a numerical integrator on an aggregated rigid mass model to allow low-thrust ion and nuclear torch engines to burn during time-warp. It also introduced screen-space orbit tessellation (authored by Johannes Peter), which uses triangle-area screen heuristics to dynamically subdivide orbital splines, eliminating polyline visual artifacts at extreme zoom levels.

---

### 5. Vessel Topological Structures, Logistics, and KSP 2 Architectural Post-Mortem

* **Finding:** Craft assembly enforces a strict single-parent tree graph (arborescence) that prohibits closed kinematic loops, while KSP 2's inability to overcome these legacy structural limits led to its technical collapse.
* **Context:** Topological constraints, fuel algorithms, and the ultimate cancellation of KSP 2 trace back to core architectural dependencies:
  * *The Prohibition of Closed Kinematic Loops:* KSP 1 enforces a single-parent hierarchy (`in-degree <= 1`, root `parent = null`). Cyclic parenting throws immediate circular dependency exceptions. This constraint exists because:
    1. PhysX iterative constraint solvers become numerically unstable when resolving closed kinematic loops, producing explosive force multiplication ("Kraken strikes").
    2. Critical recursive operations (center-of-mass evaluation, staging sequences, resource crossfeed, and `.craft`/`.sfs` serialization) require acyclic graph traversals.
    3. Structural multi-connections are approximated using non-hierarchical workarounds: `EAS-4 Strut Connectors` and `Autostruts` instantiate secondary `ConfigurableJoint` links between rigidbodies without modifying the underlying transform tree. Multi-port docking allows only one primary logical parent-child connection; secondary ports form auxiliary `PartJoint` constraints without altering the hierarchy.
  * *Docking Logic and Subtree Path Inversion:* When two independent craft dock, the smaller craft's hierarchy is restructured via subtree path inversion:
    1. The game designates a master vessel and a subordinate vessel.
    2. In the subordinate vessel, parent-child pointers along the branch between the original root and the docking port are inverted, establishing the docking port as the new temporary root.
    3. The subordinate docking port is parented to the master docking port (`port_B.parent = port_A`).
    4. The subordinate vessel instance is destroyed, merging all rigidbodies into the master craft's single PhysX island.
  * *Resource Flow Progression:*
    * *Pre-1.2 Legacy System:* Used a depth-first search (DFS) stack drain that consumed fuel from the furthest leaf parts first, requiring external physical fuel pipe parts (`FuelDuct`) to bypass the hierarchy.
    * *KSP 1.2+ Priority Graph Search:* Introduced staging-based fuel priorities. Parts are categorized into priority buckets (`fuel_priority = stage_offset + user_tweak`). Engines draw propellant evenly from all tanks within the highest priority bucket before drawing from lower tiers.
    * *KSP 2 Resource Architecture:* Transitioned from per-frame polling to a cached graph query system managed by `ResourceFlowRequestBroker`, `ResourceFlowManager`, and `ResourceFlowPriorityQuerySolver`.
  * *The Demise of KSP 2:*
    * *Development Timeline:* Announced in 2019 under Star Theory Games; late 2019 contract termination by Take-Two Interactive; establishment of Intercept Games (Seattle); multiple multi-year release delays; Early Access launch in February 2023 at 49.99 USD; Patch 0.2.0 "For Science!" released in December 2023; studio closure in May/June 2024 via WARN notice laying off 70 employees.
    * *Root Architectural Debt:* While marketed as an all-new engine written from scratch, decompilation confirmed that KSP 2 carried over large portions of ported KSP 1 source code and maintained the same single-threaded PhysX joint-tree architecture. The promised Unity DOTS/ECS foundation was largely abandoned during pre-production.
    * *Critical Flaws:* Launch was plagued by severe UI rendering overhead driven by unbatched UI Toolkit draw calls, catastrophic joint flex, and an orbital decay bug in vacuum orbits caused by continuous numerical velocity leakage. With foundational architecture incapable of supporting massive colonies, interstellar distances, and multi-threaded simulation, Take-Two Interactive cancelled the project, leaving planned roadmap milestones permanently unfulfilled.

```
[Vessel Tree Arborescence and In-Flight Docking Re-Rooting]

Initial State: Two Separate Trees
Craft A: [Root A] ---> [Core Tank] ---> [Docking Port A]
Craft B: [Root B] ---> [Truss] --------> [Docking Port B]

Inversion & Fusion Step (Port B re-rooted as subordinate tree head):
[Root A] ---> [Core Tank] ---> [Docking Port A] ===(Joint)===> [Docking Port B]
                                                                      |
                                                                   [Truss]
                                                                      |
                                                                   [Root B]
*Subordinate tree parent pointers inverted; merged into Craft A's hierarchy*
```

---

## ⚖️ Conflicting Information & Ambiguities

During synthesis of the research data, the following technical discrepancies and source conflicts were analyzed:

1. **Unity DOTS / ECS Utilization in KSP 2:**
   * *Claim A (Pre-Launch Marketing & Developer Diaries):* Early press releases asserted that KSP 2 was re-architected entirely around Unity's Data-Oriented Technology Stack (DOTS), utilizing Entity Component System (ECS), the C# Job System, and the Burst Compiler to achieve parallel multi-threaded joint physics across modern multi-core processors.
   * *Claim B (Decompile Analysis & Post-Mortem Technical Reviews):* Independent C# decompilation of public KSP 2 binaries revealed that the core vessel physics pipeline remained an object-oriented, single-threaded Unity `GameObject` / `Rigidbody` / `PhysX` joint structure identical to KSP 1. Unity Burst and the Job System were isolated to secondary auxiliary tasks: procedural terrain generation (PQS+), orbital trajectory curve tessellation, and water buoyancy calculations (introduced in Patch 0.1.3.0).
   * *Resolution / Credibility:* Claim B is confirmed by direct binary inspection. The ongoing breaking changes within Unity's DOTS/ECS API between 2018 and 2022 forced Intercept Games to abandon DOTS for the core vessel physics engine early in development.

2. **Orbital Mechanics Under Vacuum: Analytical Propagation vs. Numerical Leakage in KSP 2:**
   * *Claim A:* Celestial mechanics in both KSP 1 and KSP 2 are advertised as analytical, closed-form Keplerian trajectories while in vacuum orbit, guaranteeing that craft in stable orbits experience zero orbital decay over time.
   * *Claim B:* Early Access KSP 2 players documented continuous, measurable periapsis and apoapsis degradation while coasting in hard vacuum with engines shut down.
   * *Resolution / Credibility:* Confirmed software defect. In KSP 2's early releases, the simulation pipeline failed to switch vessels to a pure kinematic on-rails Keplerian state during coast phases. Instead, craft remained exposed to an active numerical velocity integration loop, causing floating-point rounding errors to bleed orbital energy frame-by-frame.

3. **Gravitational Boundary Formulations (Laplace SOI vs. Hill Sphere):**
   * *Ambiguity:* Informal community discussions frequently conflate the Laplace Sphere of Influence with the Hill Sphere.
   * *Resolution:* The research data clarifies that KSP explicitly employs the Laplace Sphere of Influence (`r_SOI = a * (m / M)^(2/5)`), which calculates the boundary where the two-body acceleration ratio of the secondary body exceeds that of the primary body. This is distinct from the Hill Sphere (`r_Hill = a * (m / (3 * M))^(1/3)`), which defines the boundary of true gravitational orbital stability under three-body mechanics.

---

## 🔗 Sources & Citations

* [Squad KSP 1.1 "Turbo Charged" Release Notes & Unity 5 Upgrade Documentation](https://www.kerbalspaceprogram.com/) — Technical documentation on the transition from PhysX 2.8.3 to PhysX 3.3, 64-bit platform stability, and the deprecation of `OnGUI` in favor of `uGUI`.
* [Squad Developer Blogs: Floating Origin, Krakensbane, and Scale Space Pipelines (Felipe Falanghe / HarvesteR)](https://forum.kerbalspaceprogram.com/) — Foundational architectural post-mortems detailing IEEE 754 precision mitigation algorithms and dual-camera rendering systems.
* [Intercept Games Developer Insights #9: Screen-Space Orbit Tessellation (Johannes Peter)](https://forum.kerbalspaceprogram.com/) — Detailed technical overview of adaptive screen-space triangle area subdivision for Keplerian orbit visualization.
* [Intercept Games Developer Insights: Enhanced Joint System (David Tregoning)](https://forum.kerbalspaceprogram.com/) — Architectural breakdown of Patch 0.2.0 joint damping, secondary structural links, and artificial inertia tensor boosting.
* [The Principia Development Team (Mockingbird Technical Papers & GitHub Repository)](https://github.com/mockingbirdnest/Principia) — In-depth analysis of n-body numerical integration, Symplectic Runge-Kutta-Nyström (SRKN) methods, and the gravitational instability of the stock Joolian system.
* [State of Washington WARN Act Notice & Take-Two Interactive Q4 Financial Disclosures (June 2024)](https://esd.wa.gov/about-employees/WARN) — Public filing confirming the closure of Intercept Games' Seattle studio, laying off 70 developers and formalizing the cancellation of KSP 2.

---

## 🗃️ Index Metadata

```json
{
  "title_and_scope": "Kerbal Space Program Engine Architecture, Astrodynamics, and Technical Post-Mortem (KSP 1 vs KSP 2)",
  "date": "2026-09-10",
  "objective": "Investigate KSP 1's physics pipeline, coordinate precision systems, and astrodynamical propagation, contrasting these against KSP 2's engine architecture and cancellation.",
  "conclusions": "KSP 1 overcame 32-bit floating-point limits via custom FloatingOrigin and Krakensbane systems, but was permanently constrained by single-threaded PhysX joint trees. KSP 2 failed to transition core physics to Unity DOTS/ECS, leading to unmitigated performance bottlenecks and its ultimate studio closure."
}
```

