# SSOT-003: Physical Architecture & Interplanetary Logistics

## 1. The 2D Orbital Substrate

To maximize computational efficiency for the Stock-and-Flow engine, the physical solar system is strictly constrained to a 2D coplanar plane.

### 1.1 Deterministic Mechanics
* **Coplanar Assumption:** We ignore Z-axis orbital inclination. All astronomical bodies (Planets, Moons, Asteroids) exist on a single flat disk representing the ecliptic plane.
* **Deterministic Motion:** Bodies follow simplified, deterministic Keplerian orbits. Their positions can be calculated mathematically for any point in time `t` without requiring heavy N-body physics simulations.
* **Visual Representation:** The primary player interface is a top-down, radar-like tactical map (reminiscent of air traffic control or Defcon), emphasizing nodes and data flows rather than graphical planetary rendering.

---

## 2. Geographic Abstraction (Macro & Micro Nodes)

Because simulating individual delivery trucks alongside interplanetary freighters is computationally wasteful, the physical map uses nested graph clusters.

### 2.1 Macro-Nodes (Planetary Bodies)
Earth, Luna, Mars, and specific high-value asteroids are treated as isolated macro-nodes. When zoomed out, the player views the logistics flowing *between* these macro-nodes.

### 2.2 Micro-Nodes (Surface & Orbital)
Within a Macro-Node, geography is divided into distinct zones:
* **Surface Clusters:** Continents, megacities, or specific extraction zones (e.g., "Lunar South Pole Ice Mine", "North American Datacenter").
* **Orbital Clusters:** Low Orbit, High Orbit, and Lagrange points (e.g., "Earth LEO Shipyard", "Mars L1 Relay").
* **The Gravity Well Bottleneck:** Moving resources between Surface and Orbital clusters requires highly expensive, bandwidth-limited `Converter Nodes` (Rockets, Mass Drivers, Space Elevators).

---

## 3. Dynamic Flow Edges & The "Launch Window"

In the Stock-and-Flow architecture (SSOT-001), a Flow Edge has a latency (`tau`) and a maximum bandwidth (`F_max`). In the Physical Layer, these values are not static.

### 3.1 Surface Logistics (Static)
* Trains, pipelines, and terrestrial shipping have a fixed `tau` and `F_max`. They are reliable, continuous flows.

### 3.2 Interplanetary Logistics (Dynamic)
* Because planets are constantly moving, the distance—and therefore the transport latency (`tau`)—between Earth and Mars changes dynamically.
* **Launch Windows:** Cargo cannot flow freely at all times. Interplanetary Flow Edges only open (or become economically viable regarding fuel efficiency) during specific phase angles between bodies.
* **Gameplay Implication:** If a corporation requires 1,000 tons of Lunar Titanium to build a Mars habitat, they must load it onto a freighter during the launch window. If the player hacks the orbital staging yard and delays the shipment by a few hours, the window closes. The cargo sits in orbit for months, starving the Mars habitat and triggering massive cascading failure (The Bullwhip Effect).

---

## 4. The Speed of Light vs. The Speed of Mass

As a distributed intelligence, the AGI exists as data, bound only by the speed of light, while its physical constraints are bound by the speed of mass.

### 4.1 Latency Arbitrage
* **Ping:** The distance between Earth and Mars varies from 3 to 22 light-minutes.
* Information travels instantly relative to physical goods, but not relative to the simulation tick rate.
* **The Mechanic:** The player can leverage information asymmetry. If a disaster destroys a major food-producing Converter Node on Earth, the Earth commodity markets crash instantly. However, the Mars stock exchange won't know about it for 12 minutes. The player can use their processing nodes to execute massive short-sells on the Mars market *before* the light-speed data packet carrying the news even arrives.

### 4.2 Network Propagation
* To control subverted nodes or proxies across the solar system, the player must physically establish or subvert **Relay Nodes** (satellites, deep space comms arrays). If a relay is destroyed, the AGI loses connection to that sector of the map, reverting those nodes to local autonomous control until the connection is re-established.

