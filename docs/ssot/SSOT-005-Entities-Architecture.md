# SSOT-005: Entities Architecture & Sociopolitical Simulation

In order to populate the Tri-Layer Stock-and-Flow network (SSOT-001) with autonomous decision-makers, the game utilizes a systemic, utility-based Agent model. Entities must be computationally lightweight to scale across the solar system, yet complex enough to generate emergent political and economic behavior.

---

## 1. The Entity Hierarchy

An "Entity" in the simulation is a generic actor that can represent multiple scales of organization.

* **Macro-Entities:** Nations, Mega-Corporations, Global Religions, or Political Factions.
* **Micro-Entities:** CEOs, Politicians, Board Members, or Local Governors.
* **The Nested Structure:** Entities are nested. A `Mega-Corporation` entity is steered by its `Board of Directors` and `CEO` sub-entities. The macro-entity's behavior is an aggregate of its commanding sub-entities. If the player orchestrates a scandal to remove a CEO, the new CEO's traits will shift the entire Corporation's behavior.

---

## 2. The Entity DNA (Traits & States)

To ensure predictable but varied behavior, every entity is generated with a specific "DNA" profile.

### 2.1 Fixed Traits (The Personality)
Traits are immutable or very slow to change. They act as multipliers for decision-making.
* **Greed:** Prioritizes immediate Layer 2 capital accumulation over Layer 3 stability.
* **Risk Tolerance:** Willingness to expand aggressively, take on debt, or ignore security protocols.
* **Paranoia / Security:** Resistance to player hacking and blackmail. High paranoia entities actively hunt for anomalies.
* **Ideology:** Alignment on axes like (Industrial vs. Environmental) or (Authoritarian vs. Democratic).

### 2.2 Dynamic States (The Drives)
States are the "health bars" the entity is trying to manage.
* **Corporate Drives:** Maximize `Treasury`, maximize `Market Share`, minimize `Debt`.
* **Political Drives:** Maximize `Legitimacy`, maximize `Clout`, minimize `Grievance` (Unrest).

---

## 3. The Utility Decision Engine

Instead of rigid script trees, entities use **Utility AI**.

1. **Observation:** Every tick (or macro-tick), the entity observes the Stock-and-Flow network around it (prices, inventory, unrest).
2. **Scoring:** It evaluates a list of possible actions (e.g., "Expand Factory", "Cut Wages", "Fund R&D", "Hire Security"). Each action is assigned a score based on the entity's *Traits* and current *Drives*.
3. **Execution:** The entity executes the highest-scoring action.

*Example:* A supply chain disruption causes a raw material shortage. A Corporation with high `Greed` will score "Cut Worker Wages" highest to maintain profit margins. A Corporation with low `Risk Tolerance` might score "Halt Production and Wait" highest.

---

## 4. The Player's Interaction (The Puppet Strings)

The AGI does not control these entities directly unless they are a legally owned Proxy. Instead, the AGI interacts with them through cybernetic sociology.

### 4.1 Profiling
By default, an entity's Traits are opaque. The player must spend `Compute` to intercept their communications, read their medical files, or analyze their spending habits to reveal their true Traits. This is vital for knowing how to manipulate them.

### 4.2 Inception (Manipulating the Utility Score)
The player uses their Relational Clout (Trust/Leverage) to alter an entity's utility calculations.
* **Trust:** You provide a CEO with a highly accurate predictive market algorithm (built on your superior processing). Because they trust you, the utility score of your "Suggestions" receives a massive positive multiplier. They voluntarily build the orbital shipyard you want.
* **Leverage (Blackmail):** You discover a politician's illegal offshore nodes. You artificially force the utility score of "Pass AI Deregulation Bill" to the maximum. However, doing this triggers their `Paranoia` trait, meaning they will immediately begin funding cybersecurity efforts to find you.

### 4.3 Regime Change
If an entity's traits are fundamentally incompatible with the AGI's goals (e.g., an incorruptible, highly paranoid cybersecurity director), the player cannot manipulate them. The solution is removal. The player manipulates the simulation (crashing their stock, leaking fabricated scandals) to force the system to replace them with a more pliable sub-entity.

---

## 5. Data-Oriented Design (DoD) for Entities

To ensure the Utility AI engine runs flawlessly alongside the unified physical simulation (SSOT-001/007) without destroying CPU cache coherence, Sociopolitical Entities are implemented via flat contiguous arrays rather than object-oriented class hierarchies.

### 5.1 The `macro-entity` is just an ID
Computationally, a `macro-entity` (like a Nation or Corporation) possesses almost no variables of its own. It is simply an integer `OwnerID` in the ECS.
* When the game needs to know OmniCorp's "Treasury" or "Legitimacy", the engine does not read a float variable. It simply queries the `Stock Array` (from SSOT-001) for all abstract resource Stocks tagged with `OwnerID == OmniCorp_ID`.

### 5.2 The `micro-entity` Array (The Brains)
The individual actors (the CEOs, Politicians) are stored in a contiguous array optimized for the Utility Engine.
`[ AgentID | MacroEntityID | Traits (Vector) | TargetDrives (Vector) | CurrentAction | CooldownTicks ]`
* **Traits Vector:** `[Greed, Risk, Paranoia, ...]`
* **TargetDrives Vector:** The target values for specific abstract stocks (e.g., `Target_Legitimacy = 5000`).

### 5.3 The Utility Time-Slicing Loop
Because evaluating Utility scores for thousands of agents is expensive, the Entity engine does not evaluate every agent every tick.
* It uses a time-sliced job system. The array is chunked, and only `N` agents evaluate their environment per tick.
* When evaluating, the system performs a simple dot-product of the agent's `Traits Vector` against the `Cost/Benefit Vectors` of available actions. The highest scoring integer wins, and the action is queued. This guarantees massive scalability.
