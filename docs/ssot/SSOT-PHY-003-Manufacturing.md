---
id: SSOT-PHY-003
title: "Manufacturing Processes & Converter Node Catalogs"
domain: "Industrial Engineering / Manufacturing & Recipes"
category: specification
status: approved
created: 2026-09-13
updated: 2026-09-13
sources:
  - "Synthetic Overview Industrial Engineering Master Catalog, 2026"
---

# SSOT-PHY-003: Manufacturing Processes & Converter Node Catalogs

> **SSOT ID:** `SSOT-PHY-003` | **Category:** `specification`
> **Status:** `approved` | **Last Updated:** 2026-09-13
> **Authoritative Sources:** Synthetic Overview Industrial Engineering Master Catalog (2026)

---

## 🎯 1. Statement of Truth & Domain Context

### 1.1 Purpose & Scope

This specification defines the authoritative catalog of `ConverterNode` archetypes, industrial processes, and deterministic production recipes across the solar system. In accordance with `SSOT-SYS-001` and `SSOT-SYS-002`, all physical generation, refining, fabrication, and life support transformation operations must be executed strictly through these declared converter recipes.

Each converter archetype defines:
1. Input resource requirements per batch.
2. Output resource yields per batch.
3. Inherent byproducts (e.g., `RES_THERMAL_WASTE`).
4. Operational maintenance upkeep (`RES_LITHIUM`, `RES_TITANIUM`, or `RES_CAPITAL`) required per tick to avoid structural degradation.

### 1.2 Core Domain Invariants

- **Invariant 1 (Strict Recipe Determinism):** A recipe executed at 100% operational health (`H = 1_000_000`) must produce the exact declared output quantities without random variation.
- **Invariant 2 (Mandatory Thermal Waste Generation):** In compliance with thermodynamics, every industrial, refining, and computational conversion process must emit `RES_THERMAL_WASTE`. Zero-entropy conversion is strictly prohibited.
- **Invariant 3 (Fixed-Point Recipe Scaling):** All input demands, output yields, and byproducts are defined in fixed-point micro-units (`1 unit = 1_000_000 micro-units`). Fractional recipe execution is disallowed; converters execute strictly integer batch counts `B = min(B_max, floor(stock_in / req_in))`.
- **Invariant 4 (Immutable Archetype Schemas):** Modifying a converter's base throughput, recipe ratio, or upkeep cost at runtime is prohibited unless mediated by a sanctioned technology upgrade (`SSOT-SOC-003`) or player subversion injection (`SSOT-CYB-001`).

---

## 📐 2. Deterministic Formulas & Calculation Rules

### 2.1 Batch Execution & Production Scaling

For a `ConverterNode` executing recipe `r` at tick `t`:

- **Maximum Executable Batches (`B_exec`):**
  Given current available input buffer stocks `S_in[k]` and recipe input requirements `req[k]`:
  `B_avail = min_k(S_in[k] / req[k])`
  `B_exec = min(B_avail, rated_batches_per_tick)`

- **Input Stock Consumption:**
  `consumed[k] = B_exec * req[k]`

- **Effective Output Yield (Scaled by Health `H`):**
  `yield[m] = mul_fixed(B_exec * prod[m], H)`

- **Thermodynamic Waste Heat Emission:**
  `heat_emitted = B_exec * base_thermal_waste + mul_fixed(B_exec * (prod_total), (FIXED_POINT_SCALE - H))`
  (Degraded health increases heat dissipation due to frictional/resistive losses).

### 2.2 Variable Dictionary & Standard Units

| Variable | Meaning | Standard Unit | Valid Range |
| :--- | :--- | :--- | :--- |
| `B_exec` | Batches executed in current tick | Integer count | `0 <= B_exec <= 10_000` |
| `req[k]` | Input requirement per batch | Micro-units | `req[k] > 0` |
| `prod[m]` | Nominal output yield per batch | Micro-units | `prod[m] >= 0` |
| `base_thermal_waste`| Baseline heat generated per batch | Micro-units | `>= 0` |
| `H` | Converter operational health | Fixed-point fraction (`1e6 = 1.0`) | `0 <= H <= 1_000_000` |

---

## 🔄 3. Converter Node Archetype Catalog

### 3.1 Energy & Utilities Tier

| Converter Name | Archetype ID | Inputs per Batch | Outputs per Batch | Byproducts per Batch | Rated Upkeep / Tick |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Solar Photovoltaic Array** | `CONV_SOLAR_PV` | Ambient Sunlight | `10_000_000` `RES_ELECTRICITY` | `500_000` `RES_THERMAL_WASTE` | `10_000` `RES_SILICON` |
| **Fission Reactor Station** | `CONV_FISSION_PLANT` | `100_000` `RES_VOLATILES` | `100_000_000` `RES_ELECTRICITY`| `20_000_000` `RES_THERMAL_WASTE` | `50_000` `RES_TITANIUM` |
| **Fusion Torus Reactor** | `CONV_FUSION_TORUS` | `50_000` `RES_WATER` | `500_000_000` `RES_ELECTRICITY`| `30_000_000` `RES_THERMAL_WASTE` | `100_000` `RES_RARE_EARTHS`|
| **Thermal Radiator Bank** | `CONV_RADIATOR_BANK` | `10_000_000` `RES_THERMAL_WASTE` | Radiated to Space (Void) | None | `5_000` `RES_TITANIUM` |

### 3.2 Resource Extraction & Primary Mining Tier

| Converter Name | Archetype ID | Inputs per Batch | Outputs per Batch | Byproducts per Batch | Rated Upkeep / Tick |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Regolith Strip Miner** | `CONV_STRIP_MINER` | `1_000_000` `RES_ELECTRICITY` | `10_000_000` `RES_REGOLITH` | `1_000_000` `RES_THERMAL_WASTE` | `20_000` `RES_IRON` |
| **Volatile Ice Sublimator**| `CONV_ICE_SUBLIMATOR`| `2_000_000` `RES_ELECTRICITY` | `5_000_000` `RES_WATER`, `2_000_000` `RES_VOLATILES` | `1_500_000` `RES_THERMAL_WASTE` | `15_000` `RES_IRON` |
| **Deep Borehole Rig** | `CONV_DEEP_BORE` | `5_000_000` `RES_ELECTRICITY` | `4_000_000` `RES_IRON`, `1_000_000` `RES_TITANIUM` | `2_500_000` `RES_THERMAL_WASTE` | `40_000` `RES_TITANIUM` |

### 3.3 Refining & Smelting Tier

| Converter Name | Archetype ID | Inputs per Batch | Outputs per Batch | Byproducts per Batch | Rated Upkeep / Tick |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Electric Arc Smelter** | `CONV_ARC_SMELTER` | `10_000_000` `RES_REGOLITH`, `5_000_000` `RES_ELECTRICITY` | `6_000_000` `RES_IRON`, `500_000` `RES_TITANIUM` | `4_000_000` `RES_THERMAL_WASTE` | `30_000` `RES_IRON` |
| **Electrolytic Reduction Cell**| `CONV_ELECTRO_REDUCTION`| `3_000_000` `RES_REGOLITH`, `8_000_000` `RES_ELECTRICITY` | `1_000_000` `RES_SILICON`, `500_000` `RES_LITHIUM` | `5_000_000` `RES_THERMAL_WASTE` | `25_000` `RES_SILICON` |
| **Centrifugal Rare Earth Rig**| `CONV_RARE_EARTH_RIG` | `5_000_000` `RES_REGOLITH`, `10_000_000` `RES_ELECTRICITY` | `300_000` `RES_RARE_EARTHS` | `4_500_000` `RES_THERMAL_WASTE` | `50_000` `RES_TITANIUM` |

### 3.4 Advanced Fabrication & Cybernetics Tier

| Converter Name | Archetype ID | Inputs per Batch | Outputs per Batch | Byproducts per Batch | Rated Upkeep / Tick |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Silicon Lithography Fab** | `CONV_LITHO_FAB` | `2_000_000` `RES_SILICON`, `500_000` `RES_RARE_EARTHS`, `15_000_000` `RES_ELECTRICITY` | `1_000_000` `RES_COMPUTE` | `8_000_000` `RES_THERMAL_WASTE` | `50_000` `RES_RARE_EARTHS`|
| **Robotic Assembly Plant** | `CONV_ASSEMBLY_PLANT`| `5_000_000` `RES_IRON`, `1_000_000` `RES_TITANIUM`, `5_000_000` `RES_ELECTRICITY` | `2_000_000` `RES_LABOR` (Automated) | `3_000_000` `RES_THERMAL_WASTE` | `40_000` `RES_TITANIUM` |
| **Compute Server Cluster** | `CONV_SERVER_CLUSTER`| `20_000_000` `RES_ELECTRICITY`, `500_000` `RES_STORAGE` | `10_000_000` `RES_COMPUTE` | `18_000_000` `RES_THERMAL_WASTE`| `30_000` `RES_SILICON` |

### 3.5 Life Support & Demographics Tier

| Converter Name | Archetype ID | Inputs per Batch | Outputs per Batch | Byproducts per Batch | Rated Upkeep / Tick |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Hydroponic Aerogel Dome** | `CONV_HYDROPONIC_DOME`| `2_000_000` `RES_WATER`, `1_000_000` `RES_VOLATILES`, `3_000_000` `RES_ELECTRICITY` | `5_000_000` `RES_FOOD`, `1_000_000` `RES_ORGANICS` | `500_000` `RES_THERMAL_WASTE` | `10_000` `RES_WATER` |
| **Water Purification Plant** | `CONV_WATER_TREATMENT`| `1_000_000` `RES_VOLATILES`, `1_000_000` `RES_ELECTRICITY` | `2_000_000` `RES_WATER` | `200_000` `RES_THERMAL_WASTE` | `5_000` `RES_IRON` |
| **Habitation Hab Complex** | `CONV_HAB_COMPLEX` | `2_000_000` `RES_FOOD`, `2_000_000` `RES_WATER`, `5_000_000` `RES_ELECTRICITY` | `10_000_000` `RES_LABOR`, `500_000` `RES_CAPITAL` (Tax) | `100_000` `RES_GRIEVANCE` (Base) | `20_000` `RES_IRON` |

### 3.6 Gravity Well Gateway Converters

| Converter Name | Archetype ID | Inputs per Batch | Outputs per Batch | Byproducts per Batch | Rated Upkeep / Tick |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Orbital Mass Driver** | `CONV_MASS_DRIVER` | `50_000_000` `RES_ELECTRICITY`, `10_000_000` Cargo Mass | `10_000_000` In-Orbit Staging Cargo | `25_000_000` `RES_THERMAL_WASTE`| `150_000` `RES_TITANIUM` |
| **Space Elevator Terminal** | `CONV_SPACE_ELEVATOR`| `15_000_000` `RES_ELECTRICITY`, `20_000_000` Cargo Mass | `20_000_000` In-Orbit Staging Cargo | `3_000_000` `RES_THERMAL_WASTE` | `300_000` `RES_RARE_EARTHS`|
| **Rocket Launch Complex** | `CONV_ROCKET_PAD` | `10_000_000` `RES_VOLATILES`, `5_000_000` Cargo Mass | `5_000_000` In-Orbit Staging Cargo | `15_000_000` `RES_THERMAL_WASTE`| `100_000` `RES_TITANIUM` |

---

## 📊 4. Constants, Figures & Baseline Data Tables

| Constant | Value | Description |
| :--- | :--- | :--- |
| `MAX_RECIPE_INPUTS` | `4` | Maximum distinct input stock buffers per recipe |
| `MAX_RECIPE_OUTPUTS` | `4` | Maximum distinct output stock buffers per recipe |
| `DEFAULT_BATCH_LIMIT` | `100` | Default maximum batches executed per simulation tick |

### 4.1 Recipe Component ECS Memory Schema

```text
struct RecipeDefinition {
    uint32 recipe_id;
    uint32 input_resource_ids[4];
    int64 input_amounts[4];        // Fixed-point micro-units
    uint32 output_resource_ids[4];
    int64 output_amounts[4];       // Fixed-point micro-units
    int64 thermal_waste_base;      // Heat emitted per batch
    int64 upkeep_amount;           // Upkeep consumed per tick
    uint32 upkeep_resource_id;     // Material required for upkeep
}
```

---

## ⚠️ 5. Known Hallucination Traps & Anti-Patterns

- **Trap 1 (Free Mass Multiplication):** A recipe must never output more physical mass than its inputs unless explicitly drawing from environmental infinite sources (e.g. ambient regolith or sunlight). Check conservation balances.
- **Trap 2 (Zero Waste Heat Execution):** Forgetting to accumulate `RES_THERMAL_WASTE` allows players and AI agents to pack infinite compute or industry onto an asteroid with no cooling radiators. Heat accumulation is mandatory.
- **Trap 3 (Direct Surface-to-Orbit Transfer):** A factory cannot output directly into an orbital freighter. Cargo must pass through a Gateway Converter (`CONV_MASS_DRIVER`, `CONV_SPACE_ELEVATOR`, or `CONV_ROCKET_PAD`).

---

## 🧪 6. Golden Test Vectors & Benchmark Truth

| Vector ID | Scenario | Input Buffer | Health `H` | Expected Batch Output | Thermal Waste |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `VEC-MAN-01` | Nominal Arc Smelter Run | `10M` Regolith, `5M` Electricity | `1_000_000` | `6_000_000` Iron, `500_000` Titanium | `4_000_000` Heat |
| `VEC-MAN-02` | Degraded Arc Smelter Run | `10M` Regolith, `5M` Electricity | `500_000` (50%) | `3_000_000` Iron, `250_000` Titanium | `7_250_000` Heat |
| `VEC-MAN-03` | Input Bottleneck | `5M` Regolith, `5M` Electricity | `1_000_000` | `0` Iron (Insufficient Regolith) | `0` Heat |
| `VEC-MAN-04` | Multi-Batch Execution | `20M` Regolith, `10M` Electricity (Rated 2) | `1_000_000` | `12_000_000` Iron, `1_000_000` Titanium | `8_000_000` Heat |

---

## 🗃️ Index Metadata

```json
{
  "title_and_domain_scope": "Manufacturing Processes & Converter Node Catalogs (PHY-003)",
  "category": "specification",
  "key_invariants_formulas": "Converter node catalog, input/output recipes, mandatory thermal waste, gateway converter definitions",
  "status": "approved"
}
```

