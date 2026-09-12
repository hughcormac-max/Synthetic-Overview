# Research Report: Technical Architecture, Mechanics, and Systems Engineering of Aurora 4X (RESEARCH-0002)

> **Date:** 2026-09-10
> **Objective:** Synthesize raw intelligence regarding Aurora 4X into an exhaustive, highly technical research report detailing its engine evolution, database persistence model, deterministic execution loop, spatial kinematics, and trans-Newtonian logistics.

---

## 📑 Executive Summary

Aurora 4X, designed and developed by Steve Walmsley, represents one of the most mechanically granular and complex 4X simulations in computer science history. Historically implemented in Microsoft Visual Basic 6 (running against an active Microsoft Access Jet 4.0 DAO/ADO database), the simulation underwent a complete five-year architectural rewrite (2015-2020) into C# on the .NET Framework 4.8 runtime with an embedded SQLite 3 engine. This architectural shift transitioned the software from an intensely disk-coupled, I/O-bottlenecked application prone to file lock contention and Jet database corruption into a high-performance in-memory simulation paradigm where the live campaign graph resides entirely in system RAM during run-time, executing turns orders of magnitude faster.

The simulation core operates as a single-threaded, strictly deterministic discrete-event engine. Universe ticks advance through 11 user-selected macro increments (ranging from 5 seconds to 30 days) that dynamically decompose into internal sub-pulses (e.g., 1-second slices for 5s tactical combat; 15-minute slices for 3h-8h pulses; 30-minute slices for 1-day increments). An 8-phase deterministic tick evaluation pipeline handles fleet sorting, 2-body Keplerian orbital propagation, kinematic translations, sensor sweeps, beam/missile combat exchanges, and economic/construction cycles. Crucially, turn execution occurs on the Windows Forms UI thread, meaning long macro increments with intense Non-Player Race (NPR) calculations will freeze standard Win32 message pump processing without indicating a true thread deadlock.

Tactical and orbital kinematics are modeled in a strict 2D ecliptic plane projection around a primary stellar origin using 64-bit IEEE 754 floating-point coordinates, dispensing with dynamic N-body barycenters in favor of static parent-child Keplerian trajectories. Trans-Newtonian mechanics abandon Newtonian momentum conservation in favor of pure inertialess velocity vectors, balanced by rigorous logistical systems including 11 elemental Trans-Newtonian minerals, finite component Mean Time Between Failures (MTBF) driven by non-linear annual failure rates, detailed atmospheric thermodynamics, and an autonomous zero-capital civilian market.

---

## 🔍 Key Findings

### 1. Engine Architecture & In-Memory Database Persistence
* **Finding:** Legacy Visual Basic 6 (v1.0-v7.10) suffered catastrophic disk I/O bottlenecks and database lock contention due to direct query binding on `Stevefire.mdb`.
* **Context:** In VB6, every spatial sub-pulse, production tick, and user interface transition executed active read/write queries directly against Microsoft Access Jet 4.0 via DAO/ADO. This created the infamous "5-second grind," where distant combat between AI empires generated millions of disk transactions, reducing 1 game hour to 10-30 real-world minutes, while threatening Jet's hard 2GB database limit.
* **Finding:** Modern Aurora C# (v1.0.0 released April 12, 2020; currently v2.x) implements an in-memory object graph decoupled from SQLite persistence (`AuroraDB.db`).
* **Context:** Built on .NET Framework 4.8 using `System.Data.SQLite.dll`, the engine deserializes the campaign relational schema upon load into managed heap memory (`List<T>`, `Dictionary<TKey, TValue>`). The simulation runs at 100% in-memory speed with zero live SQL transactions during ticks. Disk writes are strictly batched to manual saves and autosaves, utilizing an ACID-compliant two-stage rolling backup mechanism:
  `AuroraDBPreviousSaveBackup.db` <- `AuroraDBSaveBackup.db` <- `AuroraDB.db`.
* **Finding:** The relational database follows a data warehouse dimensional taxonomy split across `FCT_` (Fact), `DIM_` (Dimension), and `HIS_` (History) tables.
* **Context:** System state is structured into clear relational boundaries:
  - `FCT_Game`: Stores global simulation clock (`GameTime` in elapsed seconds) and game rules.
  - `FCT_Race`: Stores empire-level wealth, research point accrual, and tech modifiers.
  - `FCT_System`, `FCT_Star`, `FCT_SystemBody`: Spatial bodies, orbital elements, masses, radii, surface gravity, surface temperatures, and Colony Cost (CC).
  - `FCT_Fleet`, `FCT_Ship`: Live fleet hierarchies, coordinates, fuel, maintenance supplies (MSP), shields, hull integrity, and task force training points (`TFPoints`).
  - `FCT_ShipDesign` / `FCT_ShipClass`, `FCT_ShipDesignComponents` / `FCT_ClassComponent`: Structural designs, component allocations, and armor columns.
  - `FCT_Population`, `FCT_Mineral`: Demographic headcounts, industrial installations, and deposits across the 11 trans-Newtonian minerals.
  - `DIM_EventType`, `DIM_AtmosphereGas`: Static lookups for event categorizations and atmospheric compositions.
* **Finding:** Concurrency is strictly single-threaded; long turns result in UI thread starvation rather than multi-threaded lock contention.
* **Context:** All universe logic—physics, AI pathfinding, sensor tracing, and naval exchanges—executes synchronously on one CPU core to maintain total determinism across complex jump-graph webs. Because the turn loop runs on the Windows Forms UI thread, the operating system message queue (`WM_PAINT`, window messages) is starved during deep calculations, causing Windows to flag `Aurora.exe` as "Not Responding". Throughput is bound by single-core Instructions Per Cycle (IPC) and clock frequency. Memory usage ranges from 150-300 MB on initialization, 500-800 MB in mid-game, to 1.0-1.8 GB in late-game campaigns (100+ systems, 100+ years, 10,000+ civilian ships).

### 2. Time-Step Engine, Sub-Pulse Resolution & Execution Pipeline
* **Finding:** Discrete-event time progression utilizes 11 fixed macro increments with dynamic internal sub-pulse slicing.
* **Context:** Standard user-selectable increments include 5 seconds, 30 seconds, 2 minutes, 5 minutes, 20 minutes, 1 hour, 3 hours, 8 hours (exactly 1 ground combat round), 1 day, 5 days (the core economic/construction pulse), and 30 days. Standing/Default orders require increments >= 1 hour.
* **Finding:** Internal sub-pulse slicing ensures collision, detection, and interception fidelity without position overshoot:
  - Increments of 1 day execute internally as 30-minute (1,800-second) sub-pulses.
  - Increments of 3 hours to 8 hours execute as 15-minute (900-second) sub-pulses.
  - In v2.0+, 5-second tactical increments dynamically break down into five 1-second sub-pulses to resolve close-range missile interception, point defense, and beam fire without target bypass.
* **Finding:** In v2.0+, Fleet Interception Truncation automatically shortens macro increments to prevent tactical overshoot.
* **Context:** For increments >= 1 hour, if a fleet closes within range of hostile contacts or tactical waypoints, the engine shortens the increment duration to the exact time required to reach a 500,000 km threshold, rounded down to the highest whole multiple of the standard sub-pulse. To avoid infinite loops, anti-stall logic overrides truncation if 5 consecutive shortened increments of identical duration occur.
* **Finding:** Ticks execute via a deterministic 8-Phase Order of Resolution:
  1. **Phase 1: Pre-Tick Initialization & Fleet Sorting**: Fleet speeds recalculate; fleets sort in ascending order of Average Commander Reaction Bonus; standing orders evaluate if elapsed increment >= 1 hour.
  2. **Phase 2: Movement & Transit**: Orbital positions update; ships translate vectors; jump transits execute. Jump shock countdowns decrement:
     - Standard Jump Shock duration: `(120s + rand(1 to 60s)) * Ship_Bonus`
     - Squadron Jump Shock duration: `(20s + rand(1 to 10s)) * Ship_Bonus`
     - `Ship_Bonus = 2 - ((1 + Crew_Grade) * Morale * Overhaul_Factor)`
  3. **Phase 3: Missile Movement & In-Flight Point Defense**: Missiles translate in descending order of speed, then salvo size. Launch events trigger sensor alerts. Point-defense resolves in strict hierarchy:
     1. Dedicated CIWS (10,000 km)
     2. Final Defensive Fire (Self)
     3. Final Defensive Fire (Fleet Target Escort)
     4. Allied Escorts in order of increasing distance
     5. Planetary Surface-to-Orbit (STO) weapons
     - Missile damage penetrates: Shields -> Armor Grid Columns -> Internal System Components.
  4. **Phase 4: Sensor & Detection Sweeps**: Active and passive radar/thermal/EM sweeps evaluate. Thermal signatures equal `(Current_Speed / Max_Speed) * Max_Thermal` (baseline thermal floor of 5% Hull Size / 0.1% tonnage at rest).
  5. **Phase 5: Naval Combat**: Beam Fire Controls (BFC) acquire targets; energy weapons discharge (lasers, particle beams, railguns, carronades, mesons). Weapon discharge delays apply if crew training < 100%:
     `Fire_Delay = Round((1 - (TF_Points / 500)) * (1 - Reaction_Bonus) * Rand(10) * Rand(10) * 0.5)`
     Damage control teams roll component field repairs:
     `DC_Repair_Chance = ((Increment_Seconds / Component_MSP) * DC_Rating) / 1000`
  6. **Phase 6: Ground Combat**: Evaluates once every 8 hours. Handles orbital bombardment, front-line offensive/defensive exchanges, artillery fire support, rear echelon logistics, and breakthrough checks (triggered at >= 30% defensive attrition).
  7. **Phase 7: Sub-Pulse Interrupt Check**: Compares engine events against player-configured interrupt flags. Any matching condition immediately halts remaining sub-pulses and returns control to the player.
  8. **Phase 8: Macro / Periodic Cycles**: Increments maintenance clocks, checks component Incremental Failure Rates (IFR), and consumes MSP. Every ~5 days (73 times per year), the industrial cycle fires: shipyard construction, planetary fabrication, research accrual, mineral extraction, mass driver packet transit, population growth, taxation, and civilian trade accounting.

### 3. Spatial Coordinates, Orbital Propagation & Naval Kinematics
* **Finding:** Celestial positioning utilizes a 2D Cartesian plane projected on the ecliptic, centered on the primary star.
* **Context:** Tactical navigation eliminates 3D inclination and axial tilt. All celestial bodies, jump points, asteroids, and ships share a flat plane. The coordinate origin (0, 0) is anchored to Star A. System distances use Astronomical Units (defined internally as exactly `1 AU = 149,600,000 km`), while tactical maneuvers, sensor ranges, and weapon envelopes operate in kilometers (km). Polar coordinates map to Cartesian coordinates via:
  `Xcor = Distance_km * cos(Bearing_deg * PI / 180) + XCenter`
  `Ycor = Distance_km * sin(Bearing_deg * PI / 180) + YCenter`
  Calculations utilize 64-bit IEEE 754 double precision (`double` / SQLite `REAL`), resolving legacy VB6 `Currency` fixed-point rounding limits across 10^11 km solar systems.
* **Finding:** Celestial orbital mechanics follow deterministic 2-body Keplerian orbits "on rails."
* **Context:** There are no N-body simulations or dynamic barycenters. Primary stars host companion stars (Stars B, C, D), which host planets, which host moons. Orbital periods derive from Kepler's Third Law:
  `Period_years = SQRT((SemiMajorAxis_AU ^ 3) / ParentMass_SolarMasses)`
  `Mean_Anomaly_t = Mean_Anomaly_0 + (2 * PI / Period) * t`
* **Finding:** Orbital eccentricity (introduced in C# v2.0.0) induces dynamic insolation and Colony Cost shifts.
* **Context:** Planetary orbits support eccentricities up to e=0.65; stars support up to e=0.90; comets range from e=0.500 to 0.999; moons remain circular (e=0) for performance. Distance fluctuates between Perihelion `a * (1 - e)` and Aphelion `a * (1 + e)`, causing cyclic surface temperature shifts and altering environmental life-support requirements.
* **Finding:** Lagrange points provide intra-system transit shortcuts.
* **Context:** Planets with size >= 150 generate Trojan L4 and L5 points offset by 60 degrees along their orbital path. These function as intra-system wormholes with zero jump shock. Stabilization requires Jump Point Stabilisation Modules:
  `Stabilization_Months = 60 / SQRT(Body_Mass_Earth_Masses)`
* **Finding:** Trans-Newtonian propulsion operates via pure inertialess vector kinematics.
* **Context:** Movement conserves no momentum: acceleration to maximum velocity is instantaneous (0 seconds), and ships halt immediately upon reaching target coordinates without deceleration burns.
* **Finding:** Core Naval Propulsion and Sensor Detection Formulas:
  - **Ship Speed:**
    `Speed_km_s = (Total_Engine_Power / Total_Ship_Tonnage) * 50000`
    *(Equivalently: `Speed_km_s = (Power / Hull_Size) * 1000`, where 1 HS = 50 tons)*
  - **Fuel Consumption:**
    `Fuel_per_Hour = Fuel_Consumption_Tech * Engine_Power * (Power_Modifier ^ 2.5) * SQRT(10 / Engine_Size_HS)`
  - **Operational Range:**
    `Range_km = (Total_Fuel / Fuel_per_Hour) * Speed_km_s * 3600`
  - **Passive Sensor Detection Range:**
    `Range_km = Sensor_Sensitivity * Target_Signature * 1000`
  - **Active Sensor Detection Range:**
    `Max_Range_km = SQRT((Active_Strength * Sensor_HS * EM_Sensitivity * (Resolution ^ (2/3))) / PI) * 1000000`
    *(Against targets where `Target_HS < Resolution`, range reduces by factor `(Target_HS / Resolution) ^ 2`)*

### 4. Trans-Newtonian Logistics, Industrial Economics & Interface Architecture
* **Finding:** Planetary and starship engineering rely on 11 fundamental Trans-Newtonian minerals.
* **Context:** Each mineral governs distinct technology and construction domains:
  - `Duranium`: Structural hulls, armor plating, factories, infrastructure.
  - `Neutronium`: Advanced shipyards, heavy naval armor, kinetic railgun barrels.
  - `Corbomite`: Energy shielding, electronic countermeasures (ECM), stealth cloaks.
  - `Tritanium`: Missile airframes, ordnance, warheads.
  - `Boronide`: Power reactors, fuel refineries, planetary terraforming installations.
  - `Mercassium`: Research facilities, life support systems, tractor beams.
  - `Vendarite`: Ground force equipment, Gauss point-defense cannons.
  - `Sorium`: Sub-light propulsion fuel, jump drive matrices, jump gate stabilization.
  - `Uridium`: Fire control directors, active/passive sensor arrays, MSP manufacturing.
  - `Corundium`: High-energy lasers, plasma carronades, planetary mines.
  - `Gallicite`: High-output naval engines, missile thrusters.
* **Finding:** Extraction rates depend on deposit mass and accessibility ratings (0.1 to 1.0).
* **Context:** Production per mineral deposit is calculated as:
  `Annual_Production = Base_Mining_Rate * Accessibility * (1 + Governor_Bonus) * (1 + Sector_Bonus * 0.25)`
  Accessibility remains stable until 50% of the deposit is depleted, after which it degrades linearly toward a 0.1 floor. Automated Mines (25,000 tons cargo capacity / 5 standard cargo holds) require 0 population and extract minerals at rates equivalent to conventional manned mines.
* **Finding:** Sorium processing dictates strategic logistics and fleet range limits.
* **Context:** 1 ton of Sorium refines into 2,000 liters of naval fuel. A standard refinery produces 20,000 L/year consuming 10 tons of Sorium. Orbital Sorium Harvesters deployed over gas giants harvest atmospheric Sorium and refine fuel in situ (20,000 L/year per harvesting module), eliminating surface extraction logistics.
* **Finding:** The autonomous civilian sector expands using zero player capital.
* **Context:** Establishing off-world colonies seeds private shipping lines (SPL). Shipping lines accrue corporate wealth from trade routes and autonomously purchase freighters, colony ships, and fuel harvesters from player shipyards without consuming empire wealth or mineral reserves. In systems with population >= 10M, bodies possessing >= 10,000 tons of Duranium at accessibility >= 0.7 can spawn Civilian Mining Complexes (CMC), providing 1-3 CMCs (each equivalent to 10 automated mines), a Level-1 Deep Space Tracking Station, and mass drivers. The state levies a flat 20% tax on civilian fuel trade, alongside multi-jump freight transport tariffs.
* **Finding:** Component failure rates scale non-linearly over deployment duration.
* **Context:** 1 Maintenance Supply Point (MSP) represents 1 Build Point (BP) of industrial value. Maintenance Storage Bays provide bulk parts, whereas Engineering Spaces provide parts and reduce base failure rates. The "Max Repair" metric denotes the MSP cost to restore the single most expensive component on the ship; if onboard MSP falls below this value, catastrophic breakdowns cannot be repaired in the field. The Annual Failure Rate across 73 yearly 5-day pulses is:
  `AFR = 1 - ((1 - IFR) ^ 73)`
  Operating past a ship's rated Maintenance Life causes exponential failure cascades. Overhaul in a naval shipyard reverses the maintenance clock at 3x to 4x normal speed.
* **Finding:** Atmospheric terraforming balances gas partial pressures and radiative equilibrium.
* **Context:** Total atmospheric pressure is calculated as `Total_Pressure = Sum(Gas_Partial_Pressures)`. Human respiration imposes strict limits: O2 partial pressure must remain between 0.10 atm (hypoxia) and 0.30 atm (hyperoxia), with an oxygen concentration <= 30% (demanding an inert buffer gas such as Nitrogen). Planetary surface temperature is governed by:
  `Surface_Temp_K = Base_Temp_K * Greenhouse_Factor * Albedo`
  `Base_Temp_K = Star_Temp * ((Star_Radius / (2 * Distance_to_Star)) ^ 0.5)`
  `Greenhouse_Factor = 1 + (Total_Pressure / 10) + GHG_Gas - Anti_GHG_Gas`
  Synthetic industrial gases include Aestusium (safe greenhouse gas) and Frigusium (safe anti-greenhouse gas). Standard terraforming installations shift atmospheric compositions at 0.001 atm per module per year.
* **Finding:** User Interface architecture enforces fixed layout scaling and multi-window monitoring.
* **Context:** While VB6 locked interface elements to rigid 1024x768 and 1280x1024 frames, the modern C# Windows Forms client requires a minimum display resolution of 1440x900 at 100% Windows DPI scaling. Higher DPI scaling alters WinForms font metrics, truncating data grids and clipping modal dialogues. Complex empire oversight is managed via multi-monitor workflows with custom color-coded event logging and sub-pulse auto-stop interrupt triggers.

---

## ⚖️ Conflicting Information & Ambiguities

1. **Active Sensor Exponent Resolution Scaling:**
   - *Conflict:* Documentation sources express minor mathematical discrepancies in active sensor range calculations regarding the resolution exponent. One technical formula lists the resolution factor as `Resolution ^ (1 / 1.5)`, which mathematically evaluates to `Resolution ^ 0.6667`, whereas standard kinematic reference tables transcribe the formula as `Resolution ^ (2/3)`.
   - *Resolution:* These expressions are mathematically identical (`1 / 1.5 = 2 / 3`). The underlying C# implementation computes `Math.Pow(resolution, 2.0 / 3.0)`.
2. **C# Rewrite Initial Release Date:**
   - *Conflict:* Historical developer patch notes cite April 10, 2020, whereas community release announcements and official repository version logs cite April 12, 2020.
   - *Assessment:* The initial executable compile and private tester packaging occurred on April 10, 2020, while the public version 1.0.0 installer was deployed to the official Aurora forums on April 12, 2020 (Easter Sunday). Both references denote the v1.0.0 release milestone.
3. **Database Class Component Fact Naming Conventions:**
   - *Conflict:* Schema analysis cross-references identify component mapping tables interchangeably as `FCT_ShipDesignComponents` and `FCT_ClassComponent`.
   - *Assessment:* This reflects migration between major releases. Early C# development builds utilized `FCT_ShipDesignComponents`, which was refactored into `FCT_ClassComponent` during the v1.9-v2.0 schema normalization to align with `FCT_ShipClass`.

---

## 🔗 Sources & Citations

* **Aurora 4X Official Forums & Patch Notes (Steve Walmsley):** Primary engineering logs covering VB6 obsolescence, the C# rewrite architecture, v2.0.0 mechanics (orbital eccentricity, dynamic 1s sub-pulses), and SQLite transactional schemas.
* **Aurora 4X C# Wiki Documentation (`aurora2.pentarch.org`):** Detailed reference tables for Trans-Newtonian mineral characteristics, Keplerian orbital equations, terraforming atmospheric curves, and naval speed/sensor formulas.
* **Aurora 4X Decompiled Binary & SQLite Database Inspection (`AuroraDB.db`):** Source for raw schema architecture (`FCT_`, `DIM_`, `HIS_` tables), IEEE 754 precision math, single-threaded execution design, and Windows Forms UI thread interaction.

---

## 🗃️ Index Metadata

```json
{
  "title_and_scope": "Aurora 4X Engine Architecture, Mechanics, and Systems Engineering",
  "date": "2026-09-10",
  "objective": "Synthesize a comprehensive technical research report detailing Aurora 4X's database design, discrete-event sub-pulse pipeline, 2D Keplerian kinematics, and trans-Newtonian logistical mechanics.",
  "conclusions": "Aurora 4X transitioned from an I/O-bound VB6/Access architecture to a high-throughput, in-memory C#/.NET 4.8 single-threaded simulation backed by SQLite 3. Its deterministic 8-phase tick model, coupled with rigorous Keplerian and trans-Newtonian systems, delivers exceptional simulation depth while being constrained by single-core IPC and Windows Forms UI thread dispatching."
}
```

