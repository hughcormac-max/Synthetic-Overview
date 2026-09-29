---
id: SSOT-SYS-002
title: "Universal Simulation Taxonomy & Entity Glossary"
domain: "Game Engine / Taxonomy & Data Contracts"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Synthetic Overview Architectural Master Glossary, 2026"
---

# SSOT-SYS-002: Universal Simulation Taxonomy & Entity Glossary

> **SSOT ID:** `SSOT-SYS-002` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** Synthetic Overview Architectural Master Glossary (2026)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification establishes the authoritative, engine-wide terminology, taxonomy, and entity classifications for Synthetic Overview. It resolves all naming ambiguities between physical spatial primitives, computational graph structures, sociopolitical organizational tiers, and cybernetic network components. Every ECS component, entity archetype, network packet, and diagnostic log in the codebase must conform strictly to the definitions and casing conventions declared here.

### 1.2 Core Domain Invariants

- **Invariant 1 (Singular Physical Transformation Primitive):** All physical processing must be represented by a `ConverterNode`. Dedicated `producer-node` or `consumer-node` entity archetypes are strictly prohibited. A producer is simply a converter whose recipe inputs are zero or environmental; a consumer is a converter whose physical recipe outputs are zero or waste.
- **Invariant 2 (Three-Tier Sociopolitical Hierarchy):** Sociopolitical agency is organized into strictly three tiers: `MacroEntity` (Aggregate sovereignty/capital), `MesoEntity` (Institutional or demographic grouping), and `MicroEntity` (Autonomous Utility AI decision-maker).
- **Invariant 3 (Spatial Node Distinction):** Celestial celestial bodies are designated as `Astronode`. Discrete surface locations mapped onto an astronode's Fibonacci sphere are designated as `SurfaceNode`. The terms "macro-node" and "micro-node" are deprecated to prevent collision with sociopolitical `MacroEntity` and `MicroEntity`.
- **Invariant 4 (Strict Identifier & Casing Conventions):**
  - ECS Components & Archetypes: `PascalCase` (e.g., `ConverterNode`, `StockBuffer`, `FlowEdge`).
  - System Enums & States: `SCREAMING_SNAKE_CASE` (e.g., `ACCESS_OPAQUE`, `NODE_SUBVERTED`).
  - Canonical Resource Keys: `RES_` prefix with `SCREAMING_SNAKE_CASE` (e.g., `RES_ELECTRICITY`, `RES_COMPUTE`).
  - Fields & Variables: `snake_case` (e.g., `current_amount`, `arrival_tick`).

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Entity Relational Schema & Ownership Trees

- **Ownership Resolution Contract:**
  Every `ConverterNode`, `StockBuffer`, and `ComputeNode` must declare an `owner_id`.
  `resolve_sovereign_owner(entity_id) -> MacroEntityId`:
  1. If `owner_id` is a `MicroEntity`: return `MicroEntity.parent_macro_id`.
  2. If `owner_id` is a `MesoEntity`: return `MesoEntity.parent_macro_id`.
  3. If `owner_id` is a `MacroEntity`: return `owner_id`.
  4. If `owner_id == UNOWNED`: return `NULL`.

- **Meso-Entity Demographic Consumption Formula:**
  A `MesoEntity` representing population pops consumes essential resources per tick based on head count:
  `demand[RES_FOOD] = pop_count * POP_FOOD_CONSUMPTION_RATE`
  `demand[RES_WATER] = pop_count * POP_WATER_CONSUMPTION_RATE`
  `labor_output = mul_fixed(pop_count * BASE_LABOR_RATE, satisfaction_index)`

### 2.2 Variable Dictionary & Standard Taxonomy Catalog

| Taxonomy Term | Archetype / Type | Primary Responsibility | Associated SSOT |
| :--- | :--- | :--- | :--- |
| `Astronode` | Spatial Entity | Celestial body (Sun, Planet, Moon, Asteroid) with 2D Keplerian orbit | `SSOT-PHY-001` |
| `SurfaceNode` | Spatial Entity | Discrete surface point on an Astronode's Fibonacci sphere | `SSOT-PHY-001` |
| `StockBuffer` | Graph Node | Accumulator storing discrete units of a single resource | `SSOT-SYS-001` |
| `FlowEdge` | Graph Edge | Directed pipeline moving resources with latency `tau` | `SSOT-SYS-001` |
| `ConverterNode`| Graph Node | Physical or economic transformation machine executing a recipe | `SSOT-PHY-003` |
| `MacroEntity` | Sociopolitical | Sovereign polity, megacorporation, or financial bourse | `SSOT-SOC-001` |
| `MesoEntity` | Sociopolitical | Demographic pop, military cohort, or labor bureau | `SSOT-SOC-001` |
| `MicroEntity` | Sociopolitical | Individual Utility AI agent (CEO, minister, scientist) | `SSOT-SOC-001` |
| `ComputeNode` | Cybernetic | Physical hardware substrate providing FLOPs, power, and storage | `SSOT-CYB-001` |
| `TracerAgent` | Cybernetic | Threat algorithm traversing network edges toward player nodes | `SSOT-CYB-001` |
| `SyntheticProxy`| Cybernetic | Front organization or sockpuppet entity controlled by the player | `SSOT-CYB-001` |

---

## 🔄 3. Logic Loops, Decision Trees & State Transitions

### 3.1 Sociopolitical Hierarchy & Ownership Graph

```text
┌─────────────────────────────────────────────────────────────┐
│                       MACRO-ENTITY                          │
│   (e.g., European Lunar Mining Syndicate, Pan-Asian Alliance)│
│   - Holds Treasury, Sovereign Debt, Systemic Legitimacy     │
│   - Aggregates Macro Economic Metrics                       │
└──────────────────────────────┬──────────────────────────────┘
                               │
            ┌──────────────────┴──────────────────┐
            ▼                                     ▼
┌──────────────────────────────┐    ┌──────────────────────────────┐
│        MESO-ENTITY           │    │        MICRO-ENTITY          │
│   (Institutional / Pop)      │    │    (Utility AI Decision)     │
│   - Demographic Population   │    │    - Chief Operating Officer │
│   - Labor Union Cohort       │    │    - Station Administrator   │
│   - Military Garrison        │    │    - Lead Physicist          │
│   - Generates Labor/Grievance│    │    - Evaluates Utility AI    │
└──────────────┬───────────────┘    └──────────────┬───────────────┘
               │                                   │
               │         Commands & Upkeep         │
               └─────────────────┬─────────────────┘
                                 ▼
┌─────────────────────────────────────────────────────────────┐
│                    PHYSICAL SUBSTRATE                       │
│   - ConverterNode (Foundries, Refineries, Hydroponics)      │
│   - StockBuffer (Warehouses, Tanks, Silos)                  │
│   - ComputeNode (Server racks, Supercomputers, Relays)      │
│   - SurfaceNode (Discrete Fibonacci geographical anchors)   │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Canonical Resource Identifier Enumeration

```text
// Energy & Basic Substrates
RES_ELECTRICITY         // Power units (kW-ticks)
RES_THERMAL_WASTE       // Waste heat emitted by compute and heavy industry
RES_REGOLITH            // Raw lunar / asteroid dirt
RES_VOLATILES           // Cometary ices, ammonia, methane
RES_WATER               // Purified H2O

// Industrial Metals & Refined Materials
RES_IRON                // Structural steel precursor
RES_TITANIUM            // High-strength aerospace alloy
RES_SILICON             // Semiconductor wafer precursor
RES_LITHIUM             // Battery & reactor substrate
RES_RARE_EARTHS         // Neodymium, Dysprosium for advanced optics & actuators

// Biological & Demographic
RES_FOOD                // Hydroponic caloric output
RES_ORGANICS            // Fertilizer, biopolymers, medicine
RES_LABOR               // Available workforce person-ticks

// Intangible, Cybernetic & Sociopolitical
RES_COMPUTE             // Available floating-point execution time (GFLOP-ticks)
RES_STORAGE             // Persistent data allocation blocks (TB-ticks)
RES_CAPITAL             // Universal credit currency units
RES_LEGITIMACY          // Political compliance and regulatory mandate
RES_GRIEVANCE           // Unrest, strike probability, sabotage potential
RES_DATA_TELEMETRY      // Sensor logs and intelligence packets
```

---

## 📊 4. Constants, Figures & Baseline Data Tables

| Parameter / Constant | Exact Value | Meaning |
| :--- | :--- | :--- |
| `POP_FOOD_CONSUMPTION_RATE` | `100` | 0.000100 food units per pop per tick |
| `POP_WATER_CONSUMPTION_RATE`| `150` | 0.000150 water units per pop per tick |
| `BASE_LABOR_RATE` | `1_000` | 0.001000 labor units per healthy pop per tick |
| `MAX_RESOURCE_ID_COUNT` | `256` | Maximum unique resource types supported by fixed bitmask |

### 4.1 ECS Core Type Definitions

```text
// Entity Scale Enum
enum EntityScale {
    MACRO = 0,
    MESO = 1,
    MICRO = 2
}

// Compute Node Access Tier Enum
enum AccessTier {
    OPAQUE = 0,           // No internal telemetry visible
    READ_TELEMETRY = 1,   // Real-time stock and flow metrics readable
    WRITE_SUBVERTED = 2,  // Player can inject recipe overrides and falsify data
    PROXY_OWNED = 3       // Full legal ownership by player synthetic front
}

// Compute Node Physical Hardware Class
enum ComputeNodeClass {
    HOST_NODE = 0,        // Core AGI existence node; destruction equals game over
    SUBVERTED_NODE = 1,   // Human hardware running covert player worker threads
    PROXY_NODE = 2,       // Legally leased hardware owned by player shell company
    RELAY_NODE = 3        // Orbital or line-of-sight signal routing repeater
}
```

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Dedicated Producer / Consumer Classes):** Never author distinct `ProducerNode` or `ConsumerNode` structs. All production and consumption are handled by `ConverterNode` executing recipes.
- **Trap 2 (Assigning Utility AI to Meso-Entities):** A `MesoEntity` (such as a demographic population or factory shift) does NOT have a Utility AI brain. It is an aggregate pool that consumes resources and produces labor or grievance. Decisions are made strictly by `MicroEntity` agents (e.g., union leaders, corporate directors).
- **Trap 3 (Macro-Node vs Macro-Entity Confusion):** An `Astronode` is a physical celestial body (Earth, Mars). A `MacroEntity` is a sociopolitical organization (European Union, Ares Mining Conglomerate). Never conflate geographical spatial bodies with legal organizations.
- **Trap 4 (Free-Floating Resources):** Resources never exist without a container. They must reside in a `StockBuffer`, be in transit inside a `FlowEdge`, or be actively locked within a `ConverterNode` recipe execution buffer.

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Input | Expected Output |
| :--- | :--- | :--- | :--- |
| `VEC-GLO-01` | Sovereign Owner Resolution via MicroEntity | `owner_id = MicroEntity(id: 42, parent_macro_id: 10)` | `resolve_sovereign_owner(42) == 10` |
| `VEC-GLO-02` | Sovereign Owner Resolution via MesoEntity | `owner_id = MesoEntity(id: 88, parent_macro_id: 12)` | `resolve_sovereign_owner(88) == 12` |
| `VEC-GLO-03` | Sovereign Owner Direct MacroEntity | `owner_id = MacroEntity(id: 7)` | `resolve_sovereign_owner(7) == 7` |
| `VEC-GLO-04` | Pop Labor Generation with 80% Satisfaction | `pop_count = 100_000`, `satisfaction_index = 800_000` | `labor_output = 80_000_000` micro-units |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Universal Simulation Taxonomy & Entity Glossary (SYS-002)",
  "category": "specification",
  "key_invariants_formulas": "Singular ConverterNode primitive, Macro/Meso/Micro hierarchy, Astronode/SurfaceNode distinction, canonical resource IDs",
  "status": "approved"
}
```

