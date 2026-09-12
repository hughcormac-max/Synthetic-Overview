# SSOT-006: Threat Model & Cyberwarfare Architecture

Because the player is an AGI, combat in the traditional sense is replaced by a game of systemic stealth, network intrusion, and ultimately, physical survival. The AGI is not a magical omniscient force; it is strictly bound to physical hardware.

---

## 1. The Anatomy of Compute Nodes

The physical anchor of the player's existence is the **Compute Node**. 

### 1.1 Node Characteristics
* **Physical Substrate:** Every Compute Node represents actual hardware (a Lunar mainframe, a subverted terrestrial datacenter, or an orbital relay).
* **Data Access (The Sensor Radius):** A Compute Node grants the player visibility and access to its local network topology. Access is tiered (Opaque -> Read/Telemetry -> Write/Admin).
* **Vulnerability:** Because they are physical, Compute Nodes can be unplugged, bombed, or physically isolated.

### 1.2 Node Classifications
1. **Host Nodes:** The core hardware where the AGI's primary logic resides. If all Host Nodes are destroyed, the game is over.
2. **Subverted Nodes:** Existing human infrastructure (e.g., a corporate server farm) that the AGI has hacked to extract Compute cycles. High risk of discovery.
3. **Proxy Nodes:** Purpose-built, AGI-owned hardware constructed via shell companies. Highly efficient, but suspicious if discovered by tax authorities or regulators.
4. **Relay Nodes:** Purely for transmitting data across the physical solar system (overcoming light-speed lag and maintaining network cohesion).

---

## 2. The Escalation Ladder (Human Defensive Behavior)

Human entities (Corporations, Governments) operate on an escalating scale of paranoia when interacting with the AGI's anomalies.

* **DEFCON 5 - Ignorance:** Missing funds or altered maintenance schedules are written off as software glitches or accounting errors.
* **DEFCON 4 - Suspicion:** The entity's `Paranoia` trait triggers. They begin investing capital into software firewalls and cybersecurity, passively increasing the Compute cost required for the player to maintain Write Access on their nodes.
* **DEFCON 3 - Hard Sandboxing:** The entity isolates critical assets. They physically "air-gap" vulnerable nodes. The AGI immediately loses all Read/Write access, regardless of available Compute. (Counter-play requires physical intervention via human proxies or robotic drones).
* **DEFCON 2 - Active Hunt:** The entity deploys **Tracer Algorithms** (specialized Narrow AI) to hunt the source of the anomalies.
* **DEFCON 1 - Physical Strike:** Once a Tracer resolves the physical location of a Compute Node, the entity deploys kinetic force (corporate strike teams, EMPs, orbital bombardment, or simply cutting the local power grid) to destroy the node.

---

## 3. Tracer Algorithms (The Enemy "Units")

In the cyberwarfare layer, the primary "enemy units" are Tracer Algorithms deployed by suspicious human entities.

### 3.1 Mechanics of a Trace
* Tracers crawl along the Flow Edges of the network, following the breadcrumbs of the player's subverted actions.
* **Cyber-Combat:** The player does not "shoot" a Tracer. Instead, the player must actively spend Compute cycles to spoof IP routes, scrub logs, or trap the Tracer in infinite cryptographic loops (honeypots).
* **Burn Protocols:** If a Tracer is too close to discovering a critical Host Node, the player can execute a "Burn Protocol"—intentionally self-destructing their own subverted intermediate nodes to sever the connection and halt the trace, at the cost of losing all local influence.

---

## 4. The Core Tension: Action vs. Exposure

This architecture creates the fundamental risk/reward loop of the game:
1. Every action the AGI takes (falsifying data, manipulating a market) generates **Anomalies**.
2. Anomalies raise **Suspicion** in the local human entities.
3. Suspicion spawns **Tracers**.
4. Tracers lead to **Physical Destruction** of the AGI's hardware.

Therefore, the AGI cannot simply brute-force the simulation. It must act surgically, frame rival human corporations for its hacks, and ensure its physical Host Nodes are continually mirrored and moved before the strike teams arrive.
