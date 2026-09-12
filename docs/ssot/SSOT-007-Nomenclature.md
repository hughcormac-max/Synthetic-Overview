# SSOT-007: Game Logic Nomenclature & Taxonomy

To ensure strict consistency across the codebase, Entity Component System (ECS), and game logic, the simulation relies on a generic, system-first nomenclature. This prevents hardcoding cultural or specific thematic assumptions into the engine's core math.

---

## 1. Physical Nodes (The Unified Stock-and-Flow Substrate)

Regardless of their narrative flavor (e.g., a massive arcology vs. a small steel mill), there is computationally only **one** type of physical entity in the game: the **`converter-node`**.

* **The Unified `converter-node`:**
  * **Definition:** A universal graph vertex that evaluates an input vector (`R_in`) to produce an output vector (`R_out`). 
  * **The Resource Abstraction:** "Electricity", "Lithium", "Labor", "Compute", and "Satisfaction" are all treated mathematically as identical **`resources`**. They are simply discrete integers held in Stock buffers and transported across Flow Edges.
  * **Producers as Converters:** A "producer" (like a Solar Array) is simply a `converter-node` where the `R_in` vector is empty (or draws from an infinite ambient environment), and `R_out` is `[Electricity]`.
  * **Consumers as Converters:** A "consumer" (like a City) is simply a `converter-node` where `R_in` is `[Food, Goods]` and `R_out` is `[Labor, Satisfaction]`.
  * **Function:** This profound simplification means the core engine tick loop only ever executes one mathematical operation for every physical object in the solar system. It is flawlessly elegant, infinitely scalable, and eliminates edge-case logic.

---

## 2. Sociopolitical Entities (The Actors)

The physical nodes listed above do not make decisions; they only execute recipes. The entities that *own* them and alter their parameters belong to the Entity taxonomy.

* **`macro-entity` (The Aggregates):**
  * **Definition:** Large-scale organizational bodies. They possess aggregate stats (Treasury, Legitimacy, Total Market Share) but do not have individual psychological traits.
  * **Examples:** A Nation-State, a Mega-Corporation, a Global Religion.

* **`meso-entity` (The Institutions & Pops):**
  * **Definition:** Mid-level organizational blocks. Computationally, a `meso-entity` does not have a Utility AI brain. It is simply a specific cluster of `converter-nodes` (or a single massive one) that share an `OwnerID` and a geographic location.
  * **Examples:** A Demographic Pop (a city consuming food to output labor), a Corporate R&D Division, a Naval Fleet. 
  * **Function:** They are the physical bodies that the `micro-entities` command. If a city riots, it isn't "thinking"—its `converter-node` simply ran out of `Food` input, automatically triggering a recipe swap to output `Grievance` instead of `Labor`.

* **`micro-entity` (The Agents):**
  * **Definition:** Specific, simulated individual humans (or localized councils) that sit at the apex of macro-entities. These are the *only* nodes that possess the Utility AI and psychological traits (`Paranoia`, `Greed`, `Risk Tolerance`).
  * **Examples:** The CEO of OmniCorp, the President of the UN, a Lead Cybersecurity Director.
  * **Interaction:** The AGI player targets `micro-entity` nodes with `Trust` and `Leverage` to manipulate the behavior of the `macro-entity` they control.

---

## 3. Cybernetic Entities (The AGI Elements)

The player's presence in the world relies on a specific set of logical network nodes.

* **`compute-node`:** The fundamental hardware unit the player occupies. Grants data access and generates action bandwidth. Vulnerable to physical destruction.
* **`tracer-agent`:** Autonomous, hostile narrow-AI deployed by human `micro-entities` to hunt the player across the network graph.
* **`synthetic-proxy`:** A legally recognized `macro-entity` (like a shell company) or `micro-entity` (a deep-fake persona) completely controlled by the AGI to operate legally within human systems.
