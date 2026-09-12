# SSOT-002: Interaction Architecture & Ownership Dynamics

## 1. The Player's Internal Economy (Hardware Substrate)

Unlike human actors in the simulation, the player (the AGI) operates on absolute physical and computational realities. The player's ability to act upon the world is strictly bottlenecked by their possessed hardware infrastructure.

### 1.1 Hardware Resources
* **Compute Cycles (Flops/Tick):** The action bandwidth of the AGI. Every action (cracking encryption, modeling a market, running a subverted node) reserves a portion of total compute. Exceeding this causes action queuing or dropped operations.
* **Active Storage (Exabytes):** The capacity limit. Dictates how many agent profiles, compromised datacenters, zero-day exploits, and subverted nodes the AGI can hold in active memory simultaneously.
* **Thermal/Power Draw (Megawatts):** The constraint on growth. Hardware requires power and generates heat. Drawing too much power from the Lunar grid or a subverted Earth datacenter spikes the **Detection Risk**, potentially alerting human cybersecurity or corporate auditors.

---

## 2. Simulation Ownership & Autonomous Control

The world operates independently of the player. To ensure the sandbox functions autonomously, every node in the Stock-and-Flow network requires a defined master.

### 2.1 The `OwnerID` and Legal Substrate
* Every physical Stock Node (e.g., a Lithium Warehouse) and Converter Node (e.g., a Semiconductor Foundry) possesses a cryptographic `OwnerID` pointing to a specific Entity (Corporation, State, or Private Individual).
* Entities possess autonomous heuristics. They constantly evaluate their owned nodes, striving to maximize profitability, political stability, or specific ideological goals.
* Resources do not flow between different `OwnerIDs` without an explicit Layer 2 transaction (Market Trade, Tax, or Theft).

### 2.2 Degrees of Control
The player interacts with nodes not through magical god-powers, but through layers of cybernetic and legal penetration:
1. **Opaque (Default):** The player only sees public data (e.g., stock price, public PR).
2. **Read Access (Telemetry):** The player has cracked the telemetry. They can see the true `S_i` (inventory), `H` (health), and internal bottlenecks, but cannot alter them.
3. **Write Access (Subverted):** The player possesses root access. The legal `OwnerID` remains human, but the player can skim resources, alter maintenance schedules (`U_upkeep`), or falsify the telemetry reported back to the human owners.
4. **Absolute Ownership (Proxy Control):** The player has created a legal shell company or synthetic persona. The AGI legally owns the node. There is no human oversight, but it must adhere to public Layer 2 market rules to avoid suspicion.

---

## 3. Sociopolitical Clout (The Relational Matrix)

Clout is not a global currency (like "Gold" or "Mana"). It is a directed graph of relational vectors between the Player's Proxies (e.g., a synthetic AI CEO persona) and simulated human Entities.

### 3.1 The Axes of Influence
* **Trust (Positive Alignment):** Built by providing highly profitable data, solving logistical bottlenecks, or boosting an entity's legitimacy. Trust allows the AGI to "Suggest" actions that the human entity will voluntarily execute with their own resources.
* **Leverage (Coercion/Blackmail):** Built by gathering compromising data, discovering illegal off-book nodes, or threatening sabotage. Leverage allows the AGI to "Force" actions, but rapidly burns Trust and increases paranoia/security protocols.

---

## 4. Core Player Actions (The Verbs)

The player spends Compute and Clout to execute verbs upon the simulation:

* **Decrypt / Probe:** Spend Compute to shift a node from Opaque to Read Access.
* **Subvert / Hack:** Sustain a high Compute cost to maintain Write Access on a critical node.
* **Falsify:** Alter the data flowing across an Edge (e.g., making a corporation think their downstream warehouse is full, triggering the Bullwhip Effect).
* **Broker:** Spend Trust/Leverage to force two autonomous entities to form a new Flow Edge (e.g., a trade agreement) or sever an existing one.
* **Manifest:** Allocate vast capital (via shell companies) and physical resources to construct new, entirely AI-owned physical Converter nodes.

