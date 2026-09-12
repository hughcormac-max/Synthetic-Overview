# SSOT-001: Simulation Architecture - Interconnected Node Systems

## 1. System Philosophy & Foundational Graph Primitives

The core system is modeled as a **Directed Stock-and-Flow Network** governed by strict conservation laws, transport latency, and non-linear rate limiters. Computation is separated into distinct states to eliminate execution-order bias and non-deterministic behavior.

### 1.1 The Three Core Primitives

1. **Stock Nodes (Capacitors / Accumulators):**
   * **Definition:** Passive storage reservoirs holding discrete quantities of an element or abstract resource `S_i(t)`.
   * **Role:** Act as low-pass filters / dampeners. High buffer capacity absorbs upstream shocks but introduces latency in market signals; minimal buffer capacity increases system efficiency but creates brittle vulnerability to supply blips.
   * **State Attributes:**
     * `S_i`: Current inventory quantity.
     * `S_cap`: Maximum volumetric/mass capacity.
     * `S_target`: Desired buffer level (set by player or autonomous heuristic).

2. **Flow Edges (Resistors & Inductors):**
   * **Definition:** Directed conduits connecting stocks to converters or converters to stocks.
   * **Role:** Govern transfer throughput, delay, and friction.
   * **State Attributes:**
     * `F_max`: Maximum throughput capacity per tick.
     * `tau`: Transport latency (ticks required for mass to traverse the edge).
     * `eta`: Transmission efficiency (1 - loss rate).
     * `Q_transit`: FIFO queue of in-flight resource packets.

3. **Converter Nodes (Processors / Transformers):**
   * **Definition:** Active entities executing stoichiometric conversion recipes.
   * **Role:** Transform input bundles into output bundles according to production functions.
   * **State Attributes:**
     * `R_in`: Input recipe vector (e.g., 2A + 1B).
     * `R_out`: Output recipe vector (e.g., 1C + 0.2 Heat).
     * `H`: Operational integrity / health (0.0 to 1.0).
     * `U_upkeep`: Continuous maintenance consumption required to prevent H degradation.
     * `E_efficiency(H)`: Functional output multiplier scaled by structural health.

---

## 2. Discrete Simulation Engine Architecture

To guarantee mass-energy balance and prevent artifacts from node evaluation order, tick evaluation uses a strict **Two-Pass Cycle**:

1. **Phase 1: Polling & Demand Registration**
   * Converters calculate desired input requests based on capacity & target.
   * Downstream stocks report remaining capacity backpressure.
2. **Phase 2: Contention Resolution & Allocation**
   * Upstream stocks evaluate total demand vs. available inventory.
   * Apply Pro-Rata Rationing or Strict Priority Tiers.
3. **Phase 3: Physical Flow & Latency Transit**
   * Deduct allocated goods from source stocks.
   * Push packets into edge queues stamped with an **Absolute Arrival Tick** (Current Tick + `tau`). This ensures that dynamic orbital distances (SSOT-003) do not break in-flight transit times.
   * Pop arrived packets (where Arrival Tick <= Current Tick) into destination stocks.
4. **Phase 4: Production Integration & Wear**
   * Converters execute transformations on arrived inputs.
   * Apply maintenance decay if upkeep was starved.
   * Emit outputs and waste heat / pollutants.

---

## 3. Feedback Loops & System Dynamics

### 3.1 Balancing Loops (Stabilizers)
1. **Backpressure Throttling:** When output stocks reach saturation, converter production is throttled.
2. **Price & Friction Dampening:** Dynamic local marginal cost scales inversely with remaining inventory.

### 3.2 Reinforcing Loops (Engines of Collapse)
1. **The Energy-to-Extract-Energy Spiral:** If net energy yield falls below operational threshold, extraction throttles, causing complete cessation.
2. **The Maintenance Debt Cascade:** Shortage of maintenance components leads to structural wear, lowering output, lowering component production further.
3. **Speculative Hoarding & Buffer Shock:** Autonomous nodes detect rising volatility and artificially increase local safety stocks, draining liquid inventory.

---

## 4. Tri-Layer Conceptual Architecture (The Narrative Stacking)

To extend physical stock-and-flow mechanics into macroeconomic and cultural simulations, we conceptually stack the simulation into three vertically coupled planes. **Critically, per DoD and SSOT-007, these layers are purely conceptual.** Computationally, a Layer 3 Political Faction and a Layer 1 Iron Mine are the exact same struct residing in the exact same flat `Converter Array`.

### LAYER 3: POLITICAL & CULTURAL SUPERSTRUCTURE
* **Stocks:** Legitimacy, Grievance, Clout, Cohesion, Radicalism
* **Converters:** Factions, Demographic Pops, Political Parties, Courts

### LAYER 2: SOCIO-ECONOMIC DISTRIBUTION & MARKETS
* **Stocks:** Capital Assets, Treasury, Sovereign Debt, Wage Pools
* **Converters:** Enterprises, Banks, Market Bourses, Guilds

### LAYER 1: PHYSICAL MATERIAL SUBSTRATE
* **Stocks:** Food, Energy, Strategic Minerals, Manufactured Components
* **Converters:** Mines, Farms, Power Grids, Refineries, Transports

### Sociopolitical Failure Modes
* **The Fiscal-Military Death Loop:** Elites demand higher extraction -> state expands military -> fiscal deficit swells -> taxes increase -> commoner purchasing power collapses -> radical grievance reaches critical mass.
* **The Legitimacy Trap & Repression Debt:** Suppressed grievance converts into Latent Repression Debt, releasing explosively if coercion fails.
* **Sigmoidal Phase Transitions:** Radicalization scales as an S-curve. A trivial trigger sets off catastrophic cascading mobilization if the system is in a super-critical regime.
* **Geographic Latency & Cultural Balkanization:** Spatial distance introduces latency. Cultural cohesion decays without continuous flow of cultural enforcement.

---

## 5. Technical Implementation & Data Architecture

### 5.1 Data-Oriented Design (Flat Array Topology)
To support thousands of nodes at 60 FPS / fixed tick steps, use flat Contiguous Arrays (ECS-compatible):
* **Stock Array:** `[ Index | ElementID | CurrentAmount | Capacity | Target ]`
* **Edge Array:** `[ Index | SourceStockIdx | DestConverterIdx | F_max | Latency | Efficiency ]`
* **Converter Array:** `[ Index | Health | RecipeID | OperationalStatus ]`

### 5.2 Zero-Sum Conservation via Fixed-Point Arithmetic
* Never use floating-point numbers (`float32`, `float64`) for economic inventory tracking to prevent drift.
* Use **Fixed-Point Integers** (`int64` representing micro-units).

