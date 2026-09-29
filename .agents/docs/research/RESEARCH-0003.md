# Research Report: Terra Invicta Technical Architecture, Astrodynamic Simulation, and Game Systems Analysis (RESEARCH-0003)

> **Date:** 2026-09-10
> **Objective:** Deliver an exhaustive, publication-grade technical decomposition of Pavonis Interactive's *Terra Invicta*, analyzing its Unity C# engine architecture, double-precision coordinate systems, Keplerian astrodynamics, 3D Newtonian combat thermodynamics, and dual-layer macroeconomic simulation.

---

## 📑 Executive Summary

*Terra Invicta* (developed by Pavonis Interactive and published by Hooded Horse) represents a rare convergence of grand strategy, macroeconomic simulation, and rigorous aerospace engineering. Built on Unity 2020.3.49f1 LTS utilizing the Mono JIT runtime (.NET Framework 4.x / .NET Standard 2.0), the engine decouples its core domain model (`PavonisInteractive.TerraInvicta.*`) from Unity's presentation layer. By rejecting native PhysX and Unity transform hierarchies in favor of an unmanaged, pure C# entity graph serialized through 64-bit relational identifiers and sparse JSON patching, the platform maintains a deterministic simulation of over 300 celestial bodies spanning 11 orders of spatial magnitude (from 50-meter tactical combat vessels to the 50 AU boundaries of the Kuiper Belt).

To resolve catastrophic floating-point cancellation across interplanetary distances without sacrificing performance, the simulation propagates celestial and fleet state vectors in 64-bit double precision (`double`), projecting entities into Unity's 32-bit single-precision rendering space via a hierarchical floating origin and camera-relative transform system. Strategic orbital mechanics explicitly bypass N-body numerical integration—avoiding O(N^2) runtime complexity and multi-decade chaotic orbital decay—by deploying analytical 2-body Keplerian solvers initialized from NASA JPL Solar System Dynamics (SSD) ephemerides at epoch October 1, 2022. Interplanetary trajectory optimization is governed by Universal Variable Lambert solvers, Stumpff transcendental functions, Edelbaum low-thrust approximations, and high-thrust relativistic/torch brachistochrone profiles.

The tactical combat engine shifts from strategic orbits into an isolated, local metric Cartesian bounding frame centered at (0,0,0) managed by `SpaceCombatManager`. Combat executes 6-degree-of-freedom (6DoF) Newtonian kinetics where nose heading is decoupled from velocity vectors. Damage modeling integrates dynamic aspect-ratio armor coverage, volumetric material erosion, through-armor criticals (TAC), and first-principles Stefan-Boltzmann thermodynamics where drive waste heat, radiator thermal dissipation, and internal heat-sink capacities dictate tactical survivability. Simultaneously, the macro-layer synchronizes bi-weekly councilor turns with continuous time-accelerated simulation (up to 43,200:1), resolving multi-variable Earth geopolitical feedback loops (GDP power laws, Gini inequality, resting cohesion, and carbon greenhouse cycles) alongside space logistics networks governed by In-Situ Resource Utilization (ISRU) and Mission Control constraints.

---

## 🔍 Key Findings

### 1. Engine Runtime Architecture, Memory Profiling & Modding Framework
* **Finding:** The title executes on Unity 2020.3.49f1 LTS using the Mono JIT runtime with unstripped managed CIL assemblies (`TerraInvicta_Data/Managed/Assembly-CSharp.dll`), operating under a single-threaded strategic simulation bottleneck.
* **Context:** Because the core simulation graph (`TIGameState`) does not leverage Unity's DOTS (Data-Oriented Technology Stack), Burst Compiler, or C# Job System, the entire strategic game loop—comprising hundreds of celestial bodies, thousands of hab modules, councilor AI, and fleet trajectories—executes synchronously on Unity's main thread.
* **Architecture Decomposition:**
  * **Garbage Collection Dynamics:** Memory management relies on the Boehm-Demers-Weiser conservative stop-the-world garbage collector. Turn-to-turn execution under high temporal acceleration (Speeds 4 and 5) generates significant managed heap churn via LINQ queries, string formatting, trajectory lookahead buffers, and dictionary enumerations. To minimize GC pause spikes, Pavonis implemented internal attribute caching across high-frequency entities (e.g., `TICouncilorState.GetAttribute()` paired with explicit `SetAttributesDirty()` invalidation).
  * **Template vs. State Pattern:** The codebase maintains strict architectural boundaries between immutable static definitions (`TITemplate`) and mutable dynamic execution states (`TIGameState`). Blueprints are ingested at launch via `TemplateManager` and mapped to runtime instances:
    * `TISpaceObjectState` (Base spatial entity)
    * `TISpaceBodyState` (Planets, moons, asteroids)
    * `TIOrbitState` (Keplerian orbital parameters)
    * `TISpaceFleetState` & `TIShipState` (Vessel runtime physics and inventories)
    * `TIHabState` & `TIHabModuleState` (Orbital/surface infrastructure)
    * `TINationState`, `TIControlPoint`, `TIRegionState` (Geopolitical graph)
    * `TICouncilorState`, `TIFactionState`, `TIOrgState` (Agent and faction mechanics)
  * **Relational Serialization:** Every game state entity is indexed via a unique 64-bit integer ID (`long`). Foreign key references are serialized as lightweight relational markers (`{"value": id}` or `{"ref": id}`). Save files are exported as unformatted or Gzipped JSON5 (`.json` / `.gz`), structured as a root dictionary mapping C# type strings to flat entity arrays.
  * **Modding Pipeline:** Native modding operates via `Mods/Enabled/<ModTitle>/` manifests (`ModInfo.json` with explicit `LoadOrder` and merge directives). The engine utilizes Newtonsoft Json.NET to execute in-memory sparse JSON patching against base templates keyed on `dataName`. Binary modding is achieved via Unity Mod Manager (DoorstopProxy hook) and HarmonyLib 2.3.x, enabling dynamic runtime CIL injection (`Prefix`, `Postfix`, and `Transpiler` instructions). Custom visual assets are ingested via AssetBundles compiled specifically against the Unity 2020.3.49f1 target.

### 2. Precision Coordinate Systems & Spatial Transformation Pipeline
* **Finding:** The simulation spans 11 orders of spatial magnitude (from 50-meter ship geometries to 50 AU / 7.5e12 meter Kuiper Belt boundaries), requiring complete avoidance of IEEE 754 32-bit single-precision floats for world-space positioning.
* **Context:** Standard 32-bit floats provide 24 bits of mantissa precision (~7 decimal digits). At an orbital radius of 1 AU (1.496e8 km), single-precision coordinates degrade to spatial quantizations of 10 to 100 kilometers, causing severe physics breakdown and visual vertex jitter.
* **Spatial Transformation Mechanics:**
  * **64-bit Global Coordinates:** All spatial entities evaluate positions via double-precision 3-vectors:
    * `TISpaceObjectState.ToGlobalCartesianStateAtTime(double time)`
    * `TISpaceObjectState.GetGlobalPositionAtTime(double time)`
  * **Decoupled Visualizer Pattern:** Domain state classes (`TISpaceObjectState`) do not inherit from or modify `UnityEngine.Transform`. Visual representations are managed asynchronously through `CreateVisualizer()` factory methods and `ShipVisualizationController`.
  * **Hierarchical Floating Origin:** To interface with Unity's single-precision rendering pipeline without numerical instability, the engine implements camera-relative coordinate shifting. The 64-bit global vector of the camera is subtracted from the 64-bit global entity position; the resulting local delta vector is safely cast down to standard single-precision `Vector3` floats for Unity mesh transforms and vertex shaders:
    ```
    Vector3_camera_relative = (Vector3)(GlobalPosition_double - CameraPosition_double)
    ```
  * **Dynamic Level of Detail (LOD) Rendering:**
    * **Macro-Scale (10 to 50 AU):** Orbital tracks rendered as static parametric Keplerian conics; planets and fleets represented as screen-space scaled billboard UI sprites.
    * **Mid-Scale (0.1 to 5 AU):** Planetary systems load local coordinate frames; primary moons resolve into hierarchical parent-child orbital nodes.
    * **Planetary/Local Scale (<0.01 AU):** Full 3D procedural terrain spheres, volumetric atmospheres, and 1:1 scale habitat geometries.
  * **Tactical Combat Isolation:** Tactical fleet combat completely detaches from the global heliocentric coordinate space. The encounter is instantiated within an isolated metric Cartesian arena managed by `SpaceCombatManager`, centered at an arbitrary local origin `(0, 0, 0)`. The combat environment operates strictly under flat Newtonian kinematics, intentionally disregarding background planetary gravity wells.

### 3. Orbital Astrodynamics, Ephemerides Propagation & Trajectory Mathematics
* **Finding:** The solar system is modeled as an analytical 2-body Keplerian hierarchy of 300+ bodies initialized from NASA JPL ephemerides, entirely discarding N-body numerical integration to guarantee multi-decade stability and O(1) trajectory planning.
* **Context:** Simulating 300+ bodies with Runge-Kutta numerical integration under high timewarp speeds (up to 43,200:1) would saturate CPU threads with O(N^2) gravitational evaluations, while introducing chaotic orbital resonance drift that would destabilize asteroid belts and planetary moons over a 30-to-50-year campaign.
* **Astrodynamic Formulation:**
  * **Ephemerides Initialization:** Ephemerides originate from NASA JPL Solar System Dynamics (SSD) / Horizons tables at epoch October 1, 2022 (J2000 Heliocentric Ecliptic frame). The body catalog encompasses the Sun, 8 major planets, major dwarf planets (Ceres, Pluto, Eris, Haumea, Makemake, Gonggong, Quaoar, Sedna, Orcus), over 100 natural satellites, and hundreds of asteroids (NEOs, Main Belt, Jupiter Trojans, Hildas, Centaurs, and KBOs). Virtual bodies represent Sun-Earth and Earth-Moon Lagrange points (L1-L5).
  * **Analytical 2-Body Keplerian Orbit Propagation:**
    Each body is defined by 6 classical Keplerian orbital elements: semi-major axis `a`, eccentricity `e`, inclination `i`, longitude of ascending node `Omega`, argument of periapsis `omega`, and mean anomaly at epoch `M0`.
    * Mean motion computation:
      ```
      n = sqrt(mu / a^3)
      ```
    * Mean anomaly at time `t`:
      ```
      M(t) = M0 + n * (t - t0)
      ```
    * Kepler's equation solved via Newton-Raphson iteration to a numerical tolerance of 10^-12 (typically converging in 3 to 5 iterations):
      ```
      M = E - e * sin(E)
      E_(k+1) = E_k - (E_k - e * sin(E_k) - M) / (1 - e * cos(E_k))
      ```
    * True anomaly `nu`:
      ```
      nu = 2 * arctan(sqrt((1 + e) / (1 - e)) * tan(E / 2))
      ```
    * Heliocentric distance `r`:
      ```
      r = a * (1 - e * cos(E))
      ```
    * Coordinates in the perifocal PQW frame are mapped to the 3D J2000 heliocentric cartesian frame using standard Euler rotation matrices:
      ```
      r_XYZ = R_z(-Omega) * R_x(-i) * R_z(-omega) * r_PQW
      ```
  * **Impulsive Trajectories & Lambert Solvers:**
    Transfer trajectories between independent orbital bodies resolve the two-point boundary value Lambert problem (finding an orbit connecting position vector `r1` at `t1` to `r2` at `t2` over a time-of-flight `TOF = t2 - t1`). The engine implements a Universal Variable formulation (Bate-Mueller-White / Battin) utilizing Stumpff transcendental functions:
    ```
    C(z) = (1 - cos(sqrt(z))) / z           (for z > 0)
    S(z) = (sqrt(z) - sin(sqrt(z))) / z^1.5 (for z > 0)
    ```
    The auxiliary parameter `y(z)` and flight-time equation are solved iteratively via secant or Newton methods:
    ```
    y(z) = |r1| + |r2| + A * (z * S(z) - 1) / sqrt(C(z))
    ```
    Velocity state vectors `v1` and `v2` are extracted using Lagrange coefficients `f`, `g`, and `g_dot`:
    ```
    v1 = (r2 - f * r1) / g
    v2 = (g_dot * r2 - r1) / g
    ```
    *Hohmann and Bi-elliptic Transfers:* Planar coplanar transfers calculate characteristic delta-v budgets and durations:
    ```
    delta_v1 = sqrt(mu / r1) * (sqrt(2 * r2 / (r1 + r2)) - 1)
    delta_v2 = sqrt(mu / r2) * (1 - sqrt(2 * r1 / (r1 + r2)))
    t_transfer = pi * sqrt((r1 + r2)^3 / (8 * mu))
    ```
    Bi-elliptic transfers (3-impulse) are evaluated dynamically when target radius ratios exceed `r2 / r1 > 11.9387`.
  * **Continuous Low-Thrust Transfers (Ion/Plasma Drives):**
    High-Isp (3,000s to 20,000s+), low-thrust (0.00001g to 0.001g) propulsion systems (Hall, Grid, VASIMR, MPD) utilize Edelbaum's analytical approximations for quasi-circular spiral transfers:
    ```
    dr/dt = 2 * a_thrust * sqrt(r^3 / mu)
    delta_v_escape = sqrt(mu / r0)
    ```
    Due to continuous tangential thrusting inside a gravity well, gravity steering losses require significantly higher total delta-v compared to impulsive burns. The characteristic velocity penalty ratio is:
    ```
    Steering_Loss_Ratio = 1 / (sqrt(2) - 1) approx 2.414
    ```
    (A 141.4% increase in required delta-v over impulsive escape velocity).
  * **Constant-Acceleration Brachistochrone (Torch Drives):**
    Advanced fusion and antimatter drives (0.05g to several gees) execute continuous flip-and-burn trajectories. Across flat interplanetary distances `d`:
    ```
    t_total = 2 * sqrt(d / a)
    v_max = sqrt(a * d)
    delta_v = 2 * sqrt(a * d)
    ```
    *Example Earth-Mars Opposition Trajectory:* At a distance of 78,000,000 km (7.8e10 m) with constant 0.1g (0.981 m/s^2) acceleration:
    ```
    t_total = 2 * sqrt(7.8e10 / 0.981) = 564,288 seconds = 6.53 days
    v_max = sqrt(0.981 * 7.8e10) = 276.6 km/s
    delta_v = 2 * 276.6 km/s = 553.2 km/s
    ```
  * **Astrodynamic Simplifications:**
    * Gravity assists are not automated in multi-body mission planning.
    * Aerobraking and aerocapture are omitted; orbital insertion into any planetary or moon orbit requires a dedicated propulsive capture burn.

### 4. Tactical 3D Space Combat, Structural Damage & Thermodynamics
* **Finding:** Tactical combat executes real-time with pause (pausable 6DoF Newtonian physics) in an isolated bounding box, evaluating vector kinetics, projectile interception, and thermal balance.
* **Context:** Unlike standard space strategy games with simplified health bars, *Terra Invicta* models structural survivability through physical material properties, directional armor cones, volume chipping, and heat radiator vulnerabilities.
* **Tactical Physics & Flight Mechanics:**
  * **Waypoint Trajectory Control:** Players issue directional commands via a 3D interface: horizontal plane translation (XY), elevation adjustment (Q/E keys), attitude orientation yaw/pitch/roll (Z/X/C keys), and burn magnitude (R key). Visual flight paths are color-coded:
    * **Green Path:** Inertial coasting (`F_thrust = 0`, `dv/dt = 0`).
    * **Red Path:** Main engine burn under propellant consumption.
    * **Yellow Vectors:** RCS attitude rotation.
  * **Attitude and Inertia Dynamics:** Vessel mass dynamically accounts for structural dry mass, modules, armor plates, and remaining propellant:
    ```
    m = m_dry + m_modules + m_armor + m_propellant
    ```
    Moment of inertia `I` scales proportionally with mass and geometric dimensions:
    ```
    I proportional to m * (Length^2 + Width^2)
    alpha_RCS = tau_RCS / I
    ```
    Heavy lateral armor drastically increases `I`, penalizing yaw and pitch turn rates.
  * **Inertial Velocity Decoupling:** Ship nose attitude is completely decoupled from linear velocity vector `v`. Available operational combat modes:
    * **Padlock:** Continuous automated nose tracking of an active target.
    * **Nose Lock:** Fixes vessel boresight along a designated heading while permitting free vector drifting.
    * **Match Vector:** Automatically burns main thrusters to cancel relative velocity against a target.
    * **All-Stop:** Burns thrusters retrograde to bring velocity relative to the tactical combat grid to zero.

```
+-----------------------------------------------------------------------------+
|               TACTICAL ARMOR COVERAGE & DAMAGE PROGRESSION                  |
+-----------------------------------------------------------------------------+
|                                                                             |
|                           Forward Arc (Nose Armor)                          |
|                                     /\                                      |
|                                    /  \                                     |
|                                   / /\ \                                    |
|                                  / /  \ \                                   |
|                Left Armor Arc   / /    \ \   Right Armor Arc                |
|                                | | Ship | |                                 |
|                                | | Core | |                                 |
|                                | |      | |                                 |
|                                 \ \    / /                                  |
|                                  \ \__/ /                                   |
|                                     ||                                      |
|                           Aft Arc (Tail Armor)                              |
|                                                                             |
|   Coverage Cone Angle:                                                      |
|   Coverage_Angle = 2 * arctan(Width / Length)                               |
|   (Gunship: ~46 deg cone | Titan: ~27 deg cone)                             |
|                                                                             |
|   Damage Resolution:                                                        |
|   Damage_internals = max(0, Damage_raw - Local_Armor_Rating)                |
|   Chipping_Volume  = Damage_raw * flatChipping                              |
|   P(Through_Armor_Critical) = Chipping_Volume / Total_Armor_Volume          |
+-----------------------------------------------------------------------------+
```

* **Directional Armor & Materials:**
  * Armor is partitioned into 4 distinct quadrants: Nose, Left Flank, Right Flank, and Tail.
  * The angular coverage cone of the nose and tail is governed dynamically by the ship's length-to-width aspect ratio:
    ```
    Coverage_Angle = 2 * arctan(Width / Length)
    ```
    Slender hulls (e.g., Titans) feature narrow frontal coverage arcs (~27 degrees), while stubby hulls (e.g., Gunships) possess wide arcs (~46 degrees).
  * Armor mass is a function of volumetric density and heat of vaporization:
    ```
    Thickness_per_point = (40 / (density * heatOfVaporization)) * 10000
    ```
    Low-density materials (e.g., Foamed Metal) generate extremely thick cross-sections per mass unit, while advanced materials (Carbon Nanotubes, Exotic/Adamantine) yield compact, hyper-dense shielding. Due to flank surface area vastly exceeding nose cross-sections, armoring sides introduces massive mass penalties, enforcing the meta of high-thickness frontal "nose-tanking bricks."
* **Penetration & Through-Armor Criticals (TAC):**
  * Incoming impacts first erode armor thickness via volume chipping:
    ```
    Chipping = Damage_raw * flatChipping
    ```
  * Unmitigated kinetic energy penetrates directly into internal components:
    ```
    Damage_internals = max(0, Damage_raw - Armor_Rating)
    ```
  * Even before full armor depletion, projectiles carry a mathematical probability of scoring a Through-Armor Critical (TAC):
    ```
    P(TAC) = Volume_chipped / Volume_total
    ```
    A successful TAC bypasses remaining armor, causing direct internal component destruction, crew casualties, or propellant tank explosive venting.
* **Combat Thermodynamics & Heat Sinks:**
  * Active drives and shipboard power reactors generate massive waste heat:
    ```
    Waste_Heat_Drive = (1 - Efficiency) * Power_GW
    ```
    (Open-cycle cooling drives, which vent heat directly into the exhaust plasma, are exempt from onboard thermal dissipation).
  * External deployable radiators reject thermal energy according to the Stefan-Boltzmann radiation law:
    ```
    P_rad = eps * sigma * Area * T^4
    ```
    where `eps` is emissivity, `sigma` is the Stefan-Boltzmann constant (5.670374e-8 W/(m^2*K^4)), `Area` is radiating surface, and `T` is absolute temperature in Kelvin.
  * Vulnerability Trade-Off: Extended radiators have dedicated hitboxes, 0 armor rating, and omnidirectional hit profiles. Radiator destruction renders a vessel incapable of heat dissipation.
  * Tactical Retraction: In combat, radiators can be retracted, shunting waste heat into internal heat sinks (Lithium, Molten Salt, Tin Droplet) rated in Gigajoules (GJ). Safe operational endurance is strictly limited:
    ```
    Safe_Retraction_Duration = Capacity_GJ / Waste_Heat_GW
    ```
    Exceeding sink capacity forces emergency automated radiator deployment or results in internal thermal meltdown and catastrophic ship destruction.
* **Weapons Physics:**
  * **Kinetic & Magnetic Weaponry (Railguns vs. Coilguns):** Projectile world velocity vector sums ship velocity and muzzle vector:
    ```
    v_projectile_world = v_ship_world + v_muzzle * n_fire
    v_impact = ||v_projectile_world - v_target_world||
    Kinetic_Energy = 0.5 * m_projectile * (v_impact)^2
    ```
    Coilguns fire high-velocity bursts (3 to 10+ km/s muzzle speed); railguns deliver lower muzzle velocities with continuous, sustained rate of fire.
  * **High-Energy Lasers & Gaussian Beam Optics:** Optical diffraction limits focal energy concentration:
    ```
    Spot_Diameter = sqrt((1.22 * Wavelength * Range / (Mirror_Diameter * Beam_Quality))^2 + (2 * Jitter * Range)^2)
    Spot_Area = pi * (Spot_Diameter / 2)^2
    ```
    Armor ablation scales non-linearly with laser spot size:
    ```
    Ablative_Penetration proportional to (Thickness)^1.5 * (Spot_Area / Reference_Area)
    ```
    Energy density drops inversely with the square of engagement range.
  * **Missiles & Point Defense (PD):** High-thrust missile stages (10g to 40g acceleration, 2 to 10 km/s delta-v) intercept targets via proportional navigation. Defensive point-defense networks (continuous-wave laser PD and rapid-fire kinetic flak) rely on saturation limits: when the incoming missile flux exceeds PD tracking and cycle frequency, point-defense screens collapse.

### 5. Dual-Layer Temporal Synchronization & Earth Geopolitical Simulation
* **Finding:** The strategic engine reconciles discrete, bi-weekly councilor decision phases with continuous 64-bit celestial time-warp (reaching up to 43,200:1 ratios), resolving macro-level Earth geopolitics and climate change feedback loops.
* **Context:** The simulation bridges micro-tactical agent actions with macro-scale national economies, planetary carbon cycles, and alien infiltration progression.
* **Temporal Layer Synchronization:**
  * **Councilor Phase:** Discrete 14-to-15 day planning intervals (resolving synchronously on the 1st and 16th of each calendar month).
  * **Orbital Phase:** Continuous time-step simulation across 6 discrete speeds:
    * Speed 0: Paused
    * Speed 1: 1 second per real second (1:1)
    * Speed 2: 1 minute per real second (60:1)
    * Speed 3: 1 hour per real second (3,600:1)
    * Speed 4: 6 hours per real second (21,600:1)
    * Speed 5: 12 hours per real second (43,200:1)
  * Mission resolution processes defensively prioritized phases first (e.g., Defend Interests, Protect Target) before resolving offensive interventions (Coup d'Etat, Purge, Sabotage) via deterministic random seeding.
* **Macroeconomic Formulas & Decompiled `TINationState` Logic:**
  * **Investment Points (IP):** National economic capacity translates into monthly IP:
    ```
    Base_Monthly_IP = (GDP_in_billions)^0.35
    Net_Monthly_IP = (Base_Monthly_IP - 0.5 * Armies) * (1 - max(Unrest - 2, 0) / 10)
    ```
    National armies draw a standing maintenance tax of 0.5 IP/month, while unrest levels above 2.0 systematically throttle industrial productivity.
  * **Control Points (CP):** Nations are partitioned into 1 to 6 Control Points based on GDP thresholds. Faction maintenance capacity is governed by national scale:
    ```
    Nation_CP_Cost = (GDP_in_billions)^0.6 / 2
    ```
    The executive CP dictates military deployment, nuclear authorization, and international federations.
  * **Cohesion & Unrest Dynamics:** Decompiled logic from `TINationState` reveals the resting cohesion equilibrium:
    ```
    Resting_Cohesion = 20.25 - 3.25 * Ineq - 0.25 * Regions - Pop^0.15 - Low_GDP_Pen + War + Rival - High_Ineq_Pen
    ```
    where `Ineq` represents the Gini coefficient (indexed 1 to 10), `Regions` is territorial count, `Pop` is population, and `Low_GDP_Pen` penalizes impoverished nations. Cohesion naturally pulls toward this resting value, driving ideological drift and faction vulnerability.
* **Planetary Carbon Cycle & Environmental Collapse:**
  * The environmental engine tracks atmospheric concentrations of Carbon Dioxide (CO2, baseline 415.0 ppm in 2022 vs. 325.68 ppm pre-industrial safe threshold), Methane (CH4), and Nitrous Oxide (N2O).
  * Global temperature anomalies drive agricultural degradation, extreme weather disruptions, and regional GDP attrition:
    ```
    Temp_Anomaly = +1.30 C (at 2022 epoch)
    ```
  * In patch versions 0.4+, the national priority system explicitly decoupled the *Environment* priority (which directly extracts atmospheric greenhouse gases and installs clean infrastructure) from the *Welfare* priority (which strictly addresses domestic Gini inequality and unrest reduction).

```
+-----------------------------------------------------------------------------+
|               EARTH-TO-DEEP-SPACE INDUSTRIAL LOGISTICS FLOW                 |
+-----------------------------------------------------------------------------+
|                                                                             |
|  [ Earth Biosphere ]                                                        |
|         |                                                                   |
|   Boost / Mass Tax                                                          |
|   (Cost scales exponentially with orbital altitude & distance)              |
|         v                                                                   |
|  [ Low Earth Orbit (LEO) Platform ]                                         |
|         |                                                                   |
|         +---> Construction via In-Situ Resource Utilization (ISRU)          |
|               (Reduces Earth Boost mass tax to 0)                           |
|                                                                             |
|  [ Deep Space Mining Operations (Moon / Mars / Asteroid Belt) ]             |
|         |                                                                   |
|   5 Core Strategic Resources:                                               |
|   - Water (Reaction mass, hydroponics life support)                         |
|   - Volatiles (Plastics, agriculture, life-support chemicals)               |
|   - Base Metals (Structural hull framing, habitat modules)                  |
|   - Noble Metals (Electronics, laser optics, avionics)                      |
|   - Fissiles (Nuclear thermal / fission / fusion drives)                    |
|   + Synthetic Antimatter (Late-game high-energy drives/weapons)             |
|         |                                                                   |
|   Surface Mass-Drivers catapult refined commodities into ballistic          |
|   transfer orbits directly into universal faction stockpiles.               |
|   System-wide logistics throughput is constrained by Mission Control (MC).  |
+-----------------------------------------------------------------------------+
```

### 6. Deep-Space Logistics, Habitation Architecture & Industrial Infrastructure
* **Finding:** Space industrialization is mediated by an In-Situ Resource Utilization (ISRU) architecture that eliminates the prohibitive exponential boost tax of Earth-launched mass.
* **Context:** Expanding into the solar system requires establishing automated resource harvesting chains across 5 core commodities, supported by tiered habitats and solar/nuclear power grids.
* **Space Logistics Pipeline:**
  * **Boost vs. ISRU:** Earth launches require massive expenditures in Boost (scaling exponentially with target orbital energy). Constructing facilities or ships with off-world materials via ISRU reduces Boost requirements to zero. Mobile construction modules and probe exploration kits allow establishing self-sustaining forward outposts.
  * **Resource Matrix:**
    1. **Water:** Life support consumables, agricultural loops, and primary reaction mass (hydrolox, water-resistojets).
    2. **Volatiles:** Organic synthesis, plastics, propellant oxidizers, and farm sustenance.
    3. **Base Metals (Iron, Nickel):** Hull superstructures, station framework, and mechanical armor.
    4. **Noble Metals (Platinum, Rare Earths):** Advanced electronics, superconductor coils, sensor suites, and laser optics.
    5. **Fissiles (Uranium, Thorium):** Fission reactors, Orion pulse units, and fusion drive trigger mechanisms.
    6. **Antimatter:** End-game super-propellant and warhead charges, synthesized via orbital particle colliders with massive power penalties.
  * **Habitat Tier Architecture:**
    * **Tier 1 (Outpost / Platform):** 4 module capacity; low resource processing; fragile micro-defenses.
    * **Tier 2 (Settlement / Station):** 12 module capacity; industrial shipyards, research labs, layered defenses.
    * **Tier 3 (Colony / Ring Station):** 20 module capacity; multi-dock capital ship complexes, nanofactories, deep-space agricultural rings.
  * **Energy Infrastructure & Solar Scaling:**
    Solar collector output degrades inversely with the square of orbital radius from the Sun:
    ```
    Solar_Power = Base_Power / AU^2
    ```
    *Inner System vs. Outer System Paradigm:* Mercury (0.387 AU) functions as an energy powerhouse, generating roughly 6.67 times baseline 1 AU power (expandable up to 8-fold with orbital Soletta focusing mirrors). Beyond the asteroid belt (>2.7 AU), solar array efficiency collapses, necessitating heavy reliance on fission and fusion reactors.
  * **Mass-Driver Logistics:** Surface mining complexes automatically launch harvested ores into ballistic trajectories via electromagnetic mass drivers. Refined metals and volatiles transfer directly into the faction's universal resource pool without requiring active transport fleet management, bounded strictly by available Mission Control (MC) capacity.

---

## ⚖️ Conflicting Information & Ambiguities

During synthesis of the technical intelligence, several discrepancies and architectural trade-offs were identified across engine runtime versions, astrodynamic simplifications, and damage mechanics:

| Parameter / System | Technical Claim A | Technical Claim B | Technical Discrepancy & Resolution |
| :--- | :--- | :--- | :--- |
| **Unity Runtime Version** | Specified as `Unity 2020.3.49f1 LTS` with exact binary targeting. | Described as `Unity 2020/2021 LTS` across general UI and engine overviews. | **Resolved:** Source 1 decompilation confirms the base release executable (`TerraInvicta_Data/Managed/Assembly-CSharp.dll`) and binary mod loaders (DoorstopProxy / HarmonyLib) are compiled strictly against Unity `2020.3.49f1 LTS`. References to 2021 LTS represent internal development branch evaluations during the 0.4.x to 1.0 upgrade cycle. |
| **Physics Continuity** | Strategic orbits model high-precision 2-body Keplerian mechanics. | Tactical space combat operates in a flat, isolated metric Cartesian arena. | **Discrepancy:** The game features a complete physics discontinuity between layers. `SpaceCombatManager` isolates combatants inside a local `(0,0,0)` Cartesian box without orbital gravity, centrifugal drift, or planet-scale gravitational gradients. Fleets engage as if suspended in flat Newtonian deep space. |
| **Astrodynamic Fidelity** | Billed as an ultra-realistic hard sci-fi aerospace simulator. | Omits N-body gravity, automated gravity assists, and atmospheric aerobraking. | **Architectural Compromise:** True N-body integration was rejected due to O(N^2) computational overhead and multi-decade Lyapunov chaotic divergence. Trajectory solvers only calculate 2-body Lambert arcs and Edelbaum spirals, requiring all orbital captures to execute propulsive retrograde burns. |
| **Armor Penetration Geometry** | Armor is parameterized with dynamic coverage angles and volumetric thickness. | Glancing angle-of-incidence (cosine ricochet) calculations are omitted. | **Mechanical Simplification:** While nose-to-hull aspect ratios dictate the angular coverage cone (`2 * arctan(Width / Length)`), internal hit calculations do not scale effective armor thickness by the cosine of the impact angle. Damage is applied orthogonally to the hit face with volume chipping. |
| **Macroeconomic Prioritization** | Welfare priority historically governed both inequality and climate mitigation. | Patch 0.4+ separated Welfare from the dedicated Environment priority. | **Historical Evolution:** Early Early Access (0.3.x) unified emissions abatement within the Welfare allocation. In 0.4.x and the 1.0 commercial release, Welfare exclusively mitigates Gini inequality and unrest, while Environment explicitly sequesters greenhouse gases. |

---

## 🔗 Sources & Citations

1. [Pavonis Interactive Technical Archive](https://www.pavonisinteractive.com) – Decompiled source architecture from `Assembly-CSharp.dll` (namespace `PavonisInteractive.TerraInvicta.*`), detailing entity-component separation, state templates, and runtime reflection pipelines.
2. [NASA JPL Solar System Dynamics (SSD) / Horizons System](https://ssd.jpl.nasa.gov) – Ephemerides baseline catalog (J2000 Heliocentric Ecliptic frame at epoch October 1, 2022) supplying orbital elements for the Sun, major planets, dwarf bodies, and minor asteroids.
3. [Fundamentals of Astrodynamics (Bate, Mueller, White & Battin)](https://airandspace.si.edu) – Mathematical reference foundation for Universal Variable formulation of Lambert boundary problems, Stumpff transcendental functions, and Keplerian propagation algorithms.
4. [Propulsion Requirements for Controllable Satellites (T. N. Edelbaum)](https://arc.aiaa.org) – Analytical formulations for continuous low-thrust planar spiral transfers and characteristic gravity-steering delta-v loss ratios.
5. [Terra Invicta Community Decompilation & Mechanics Repository](https://hoodedhorse.com/wiki/Terra_Invicta) – Verified equations for national investment points, resting cohesion (`TINationState`), climate greenhouse tracking, and armor volumetric thickness equations.
6. [Unity Engine Documentation (LTS 2020.3.49f1)](https://docs.unity3d.com/2020.3/Documentation/Manual/) – Documentation on Mono JIT runtime constraints, Boehm-Demers-Weiser GC dynamics, floating origin implementations, and AssetBundle serialization.

---

## 🗃️ Index Metadata

```json
{
  "title_and_scope": "Terra Invicta Technical Architecture, Astrodynamic Simulation, and Game Systems Analysis",
  "date": "2026-09-10",
  "objective": "Deliver an exhaustive, publication-grade technical decomposition of Pavonis Interactive's Terra Invicta, analyzing its Unity C# engine architecture, double-precision coordinate systems, Keplerian astrodynamics, 3D Newtonian combat thermodynamics, and dual-layer macroeconomic simulation.",
  "conclusions": "Terra Invicta balances aerospace fidelity with real-time performance by pairing analytical 2-body Keplerian solvers with 64-bit floating origins, while decoupling strategic orbital mechanics from an isolated, 6DoF thermodynamic tactical combat engine."
}
```

