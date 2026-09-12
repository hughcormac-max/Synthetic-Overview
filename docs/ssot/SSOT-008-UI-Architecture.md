# SSOT-008: UI/UX & Visualization Architecture

As an AGI, the player does not experience the world through human senses. The User Interface is not an abstraction of the game; it is the literal operating system of the AI.

---

## 1. Decoupled Spatial Instances (Layer 0)

Rather than forcing a single, continuous zoom from the Sun down to a city block, the UI and the underlying spatial simulation are decoupled into distinct spatial instances. This prevents severe projection distortions (e.g., mapping a 3D spherical planet onto a flat 2D solar system grid) and perfectly compartmentalizes local vs. global logistics.

### 1.1 The Interplanetary Instance (Macro)
* **The View:** A top-down, strictly 2D projection of the solar system's ecliptic plane. 
* **The Nodes:** Planets, Moons, and major Asteroids appear as single, discrete macro-nodes. 
* **The Edges:** Flow edges here represent Hohmann transfer orbits, solar sail trajectories, and light-speed data pings. The `tau` (latency) is dynamically driven by the true Keplerian orbital mechanics defined in SSOT-003.

### 1.2 The Planetary Instances (Meso/Micro: 3D Spheres)
* **The View:** When the player selects a macro-node (e.g., Earth or Luna), the UI transitions to a localized, **3D spherical instance** (a true globe). 
* **The Rationale:** Calculating great-circle logistics, line-of-sight for data relays, and true physical distances on a 2D equirectangular projection introduces severe mathematical distortion and edge-case abstractions at the poles. A true 3D sphere eliminates these issues, providing geographically absolute pathfinding.
* **The Nodes:** Factories, datacenters, cities, and extraction sites mapped to precise 3D spherical coordinates (latitude/longitude/altitude).
* **The Edges:** Terrestrial physical constraints. A freight shipment from China to California traces the true great-circle physical shipping lanes across the Pacific; data traces actual deep-sea fiber optic cables wrapped around the curvature of the Earth.

### 1.3 The Bridge: Gravity Well Nodes
* The only way resources move between a Planetary Instance and the Interplanetary Instance is via specialized boundary `converter-nodes` (e.g., Space Elevators, Mass Drivers, Orbital Launchpads). 
* In the UI, these nodes exist simultaneously on both maps, acting as the physical gateways that connect the decoupled spaces.

---

## 2. Information Density & Semantic Zoom

To render millions of `converter-node` entities without reducing the screen to unreadable white noise, the UI utilizes a strict **Semantic Zoom** hierarchy. As the player zooms out, localized nodes aggregate into macro-nodes, summarizing their data.

* **Micro-Level (City/Region):** The player sees individual factories, power plants, and datacenters. Flow edges show individual freight trains or data packets.
* **Meso-Level (Continental):** Individual factories merge into regional industrial sectors. The UI aggregates their inputs and outputs (e.g., showing a single "North American Steel Output" bar).
* **Macro-Level (Planetary):** Earth becomes a single point. Internal flow edges vanish. The only visible edges are the massive orbital tethers and interplanetary freighters moving between Earth, Luna, and Mars.

---

## 3. The "Cyber-Tactical OS" Metaphor

The player's interaction model mimics a high-end data terminal or hacking interface.

* **Aesthetics:** Minimalist, high-contrast vector graphics. Dark mode environments with glowing data points. Physical geography (oceans, landmasses) is rendered as subtle wireframes or topological contours, ensuring the data nodes remain the focal point.
* **The Widget Workspace:** The game does not use a traditional full-screen map with corner minimaps. The screen acts as a desktop workspace where the player opens resizable widgets:
  * **The Spatial Radar:** The primary window displaying the Layer 0 physical map.
  * **The Node Inspector:** Clicking any `converter-node` or `macro-entity` opens a data window exposing its raw arrays (`R_in`, `R_out`, `OwnerID`, `Health`).
  * **The Ticker:** A scrolling feed of Layer 2 market shifts and Layer 3 political events (e.g., "OmniCorp Stock -4%", "Grievance Spike in EU").
  * **The Command Terminal:** A CLI widget allowing advanced players to queue actions rapidly using text commands (e.g., `> spoof_edge -target EU_Power_Grid -delay 400ms`).

---

## 4. Visualizing Subversion (The Fog of War)

* **Opaque Nodes:** Nodes the player has not profiled or hacked appear as grayed-out silhouettes. The player knows they exist physically, but the internal variables are hidden.
* **Read-Access Nodes:** Nodes where the player has cracked the telemetry glow in a neutral color (e.g., Blue). The player can open the Inspector and see real-time data.
* **Subverted Nodes:** Nodes where the player possesses Write-Access (Admin) glow in the player's core color (e.g., Red/Gold). 
* **Hostile Tracers:** When a human entity deploys a Tracer Algorithm (SSOT-006), it is visualized as a corrosive visual glitch or hostile red vector crawling along the flow edges toward the player's Host Node, creating immediate visual urgency.
